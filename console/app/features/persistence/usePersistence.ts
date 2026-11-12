import { computed, onScopeDispose, ref, shallowRef, toValue, watch } from "vue";
import type { MaybeRefOrGetter } from "vue";
import { useDebounceFn, useEventListener, useIntervalFn } from "@vueuse/core";
import {
  useDrawVue,
  hashElementsVersion,
  getNonDeletedElements,
  tryCatch,
  serializeFiles,
  restoreImageCache,
  toFileId,
} from "@drawvue/core";
import type { BinaryFiles, ExcalidrawElement } from "@drawvue/core";
import { useWhiteboardStore } from "~/stores/whiteboards";
import type { UpdateWhiteboardPayload } from "~/stores/whiteboards";
import {
  isValidPersistedScene,
  migrateScene,
  normalizeElements,
  serializeElements,
  buildPersistedScene,
} from "./sceneStorage";
import type {
  PersistenceEvent,
  PersistenceEventType,
  SaveStatus,
  StoreMetadata,
  UsePersistenceReturn,
} from "./types";
import { CURRENT_SCHEMA_VERSION } from "./types";

export function usePersistence(
  whiteboardId: MaybeRefOrGetter<string>,
): UsePersistenceReturn {
  const { elements, dirty, imageCache } = useDrawVue();
  const whiteboardStore = useWhiteboardStore();

  const getWhiteboardId = (): string => toValue(whiteboardId);

  // ── Reactive state ──────────────────────────────────────────────────
  const isRestored = ref(false);
  const saveStatus = ref<SaveStatus>("idle");
  const error = shallowRef<Error | null>(null);

  // ── Diagnostic state (was non-reactive, now refs for inspector) ─────
  const lastSavedHash = ref(0);
  const forwardVersionMode = ref(false);
  let watchPaused = false;

  // File identifiers known to be persisted for this whiteboard — used to
  // diff new files (upload once) and garbage-collect unreferenced ones.
  const persistedFileIds = new Set<string>();

  // ── Live scene hash ─────────────────────────────────────────────────
  const sceneHash = computed(() =>
    hashElementsVersion(getNonDeletedElements(elements.elements.value)),
  );

  // ── Event log ───────────────────────────────────────────────────────
  const events = shallowRef<readonly PersistenceEvent[]>([]);
  let eventIdCounter = 0;

  function logEvent(type: PersistenceEventType, message: string): void {
    const event: PersistenceEvent = {
      id: ++eventIdCounter,
      type,
      timestamp: Date.now(),
      message,
    };
    const next = [...events.value, event];
    events.value = next.length > 50 ? next.slice(-50) : next;
    console.warn(`[persistence] ${type}: ${message}`);
  }

  // ── Save logic ──────────────────────────────────────────────────────

  async function persistFiles(
    nonDeleted: readonly ExcalidrawElement[],
  ): Promise<void> {
    const files = serializeFiles(imageCache.cache.value, nonDeleted);
    const referencedIds = new Set(Object.keys(files));

    // Write files not yet persisted (each file is written once)
    for (const [fileId, file] of Object.entries(files)) {
      if (persistedFileIds.has(fileId)) continue;
      await whiteboardStore.saveWhiteboardFile(
        getWhiteboardId(),
        fileId,
        file.mimeType,
        file.dataURL,
      );
      persistedFileIds.add(fileId);
    }

    // Garbage-collect persisted files no longer referenced by any element
    for (const fileId of [...persistedFileIds]) {
      if (!referencedIds.has(fileId)) {
        await whiteboardStore.deleteWhiteboardFile(fileId);
        persistedFileIds.delete(fileId);
      }
    }
  }

  async function executeSave(): Promise<void> {
    if (forwardVersionMode.value) {
      return;
    }

    if (!isRestored.value) {
      return;
    }

    if (saveStatus.value === "error") {
      return;
    }

    const nonDeleted = getNonDeletedElements(elements.elements.value);
    const currentHash = hashElementsVersion(nonDeleted);
    if (currentHash === lastSavedHash.value) {
      logEvent("save-skip", `hash unchanged (${currentHash})`);
      saveStatus.value = "idle";
      return;
    }

    saveStatus.value = "saving";
    logEvent(
      "debounce",
      `fired — version changed (${lastSavedHash.value} → ${currentHash})`,
    );

    const [serializeError, serialized] = serializeElements(
      elements.elements.value,
    );

    if (serializeError) {
      saveStatus.value = "error";
      error.value = new Error("Failed to serialize drawing");
      logEvent("error", error.value.message);
      return;
    }

    const scene = buildPersistedScene(serialized);

    const [saveError] = await tryCatch(
      (async () => {
        await Promise.all([
          whiteboardStore.updateWhiteboard(getWhiteboardId(), {
            payload: scene as unknown as UpdateWhiteboardPayload["payload"],
          }),
          persistFiles(serialized),
        ]);
      })(),
    );

    if (saveError) {
      saveStatus.value = "error";
      error.value = new Error("Failed to save drawing to database");
      logEvent("error", `${saveError.message}`);
      return;
    }

    lastSavedHash.value = currentHash;
    error.value = null;
    saveStatus.value = "idle";
    logEvent(
      "save",
      `completed — hash ${currentHash} — ${serialized.length} elements`,
    );
  }

  const scheduleSave = useDebounceFn(executeSave, 300, { maxWait: 1000 });

  function requestSave(): void {
    if (forwardVersionMode.value) {
      return;
    }

    saveStatus.value = "pending";
    logEvent("change", "detected — element change");
    void scheduleSave();
  }

  function flushSave(): void {
    // executeSave() is idempotent via hash comparison — safe to call even if
    // the debounced timer hasn't fired yet (double call is a no-op).
    void executeSave();
  }

  // ── Clear storage (keeps the whiteboard row, empties its content) ───

  async function clearStorage(): Promise<void> {
    const id = getWhiteboardId();

    const files = await whiteboardStore.getWhiteboardFiles(id);
    for (const file of files) {
      await whiteboardStore.deleteWhiteboardFile(file.identifier);
    }
    persistedFileIds.clear();

    await whiteboardStore.updateWhiteboard(id, {
      payload: buildPersistedScene([]),
    });

    lastSavedHash.value = 0;
    logEvent("clear", "persisted scene and files cleared from database");
  }

  // ── Watch elements for changes ──────────────────────────────────────
  watch(
    () => elements.elements.value,
    () => {
      if (!watchPaused) {
        requestSave();
      }
    },
  );

  // ── Watch image cache for additions (new uploads, bg removal results) ──
  watch(imageCache.cache, () => {
    if (!watchPaused) {
      requestSave();
    }
  });

  // ── Poll for in-place mutations (drag, resize, rotate, style) ──────
  // The watcher only fires when the ShallowRef array reference changes.
  // mutateElement() edits in place, so poll versionNonce hashes every 2s.
  useIntervalFn(() => {
    if (watchPaused || forwardVersionMode.value || !isRestored.value) return;
    const currentHash = hashElementsVersion(
      getNonDeletedElements(elements.elements.value),
    );
    if (currentHash !== lastSavedHash.value && saveStatus.value !== "saving") {
      requestSave();
    }
  }, 2000);

  // ── File restore helper ────────────────────────────────────────────
  async function restoreFiles(): Promise<void> {
    const [filesError, files] = await tryCatch(
      whiteboardStore.getWhiteboardFiles(getWhiteboardId()),
    );
    if (filesError) {
      logEvent("error", `Failed to load files: ${filesError.message}`);
      return;
    }

    for (const file of files) {
      persistedFileIds.add(file.identifier);
    }

    if (files.length === 0) return;

    const binaryFiles: BinaryFiles = {};
    for (const file of files) {
      binaryFiles[file.identifier] = {
        id: toFileId(file.identifier),
        mimeType: file.mimeType,
        dataURL: file.data,
        created: Date.parse(file.createdAt) || Date.now(),
      };
    }

    const [restoreError] = await tryCatch(
      restoreImageCache(binaryFiles, imageCache.addImage),
    );
    if (restoreError) {
      logEvent(
        "error",
        `Failed to restore image cache: ${restoreError.message}`,
      );
      return;
    }
    logEvent("restore", `restored ${files.length} binary files`);
  }

  // ── Restore (runs immediately in setup, not onMounted) ─────────────
  void (async () => {
    watchPaused = true;

    const [loadError, whiteboard] = await tryCatch(
      whiteboardStore.getWhiteboard(getWhiteboardId()),
    );

    if (loadError) {
      logEvent("error", `Failed to load whiteboard: ${loadError.message}`);
      saveStatus.value = "error";
      error.value = loadError;
      isRestored.value = true;
      watchPaused = false;
      return;
    }

    if (!whiteboard) {
      logEvent("error", "Whiteboard not found — saves are disabled");
      forwardVersionMode.value = true;
      saveStatus.value = "error";
      error.value = new Error("Whiteboard not found");
      isRestored.value = true;
      watchPaused = false;
      return;
    }

    let restoredCount: number;

    const payload = whiteboard.payload;
    if (payload !== null && isValidPersistedScene(payload)) {
      if (payload.schemaVersion > CURRENT_SCHEMA_VERSION) {
        forwardVersionMode.value = true;
        logEvent(
          "restore",
          "Data has a newer schema version. Saves are disabled — please refresh this tab.",
        );
        elements.replaceElements(normalizeElements(payload.elements));
      } else {
        const migrated = migrateScene(payload);
        restoredCount = normalizeElements(migrated.elements).length;
        if (restoredCount > 0) {
          elements.replaceElements(normalizeElements(migrated.elements));
        }
        logEvent("restore", `from database — ${restoredCount} elements loaded`);
      }
    } else if (payload !== null) {
      logEvent("error", "Invalid scene payload in database — starting empty");
    }

    await restoreFiles();
    dirty.value?.markStaticDirty();

    lastSavedHash.value = hashElementsVersion(
      getNonDeletedElements(elements.elements.value),
    );
    isRestored.value = true;
    watchPaused = false;
  })();

  // ── Page unload handlers ────────────────────────────────────────────
  // flushSave() is fire-and-forget: the async database write may not
  // complete during unload. The debounced autosave (300ms) plus the
  // visibilitychange/pagehide flushes cover normal navigation.

  useEventListener(document, "visibilitychange", () => {
    if (document.hidden) {
      flushSave();
    }
  });

  useEventListener(globalThis, "pagehide", () => flushSave());

  // ── Cleanup: flush pending save on scope disposal ───────────────────

  onScopeDispose(() => flushSave());

  // ── Diagnostics: read metadata back from the database ────────────────

  async function readStoreMetadata(): Promise<StoreMetadata> {
    const [wbError, whiteboard] = await tryCatch(
      whiteboardStore.getWhiteboard(getWhiteboardId()),
    );

    const whiteboardMeta: StoreMetadata["whiteboard"] =
      wbError || !whiteboard
        ? null
        : {
            identifier: whiteboard.identifier,
            title: whiteboard.title,
            schemaVersion: isValidPersistedScene(whiteboard.payload)
              ? whiteboard.payload.schemaVersion
              : CURRENT_SCHEMA_VERSION,
            elementCount: isValidPersistedScene(whiteboard.payload)
              ? whiteboard.payload.elementCount
              : 0,
            savedAt: isValidPersistedScene(whiteboard.payload)
              ? whiteboard.payload.savedAt
              : 0,
            dataSize: whiteboard.payload
              ? new Blob([JSON.stringify(whiteboard.payload)]).size
              : 0,
            updatedAt: whiteboard.updatedAt,
          };

    const [filesError, files] = await tryCatch(
      whiteboardStore.getWhiteboardFiles(getWhiteboardId()),
    );

    const filesMeta: StoreMetadata["files"] = filesError
      ? null
      : {
          count: files.length,
          totalSize: files.reduce((sum, f) => sum + f.data.length, 0),
        };

    return { whiteboard: whiteboardMeta, files: filesMeta };
  }

  // ── Return ──────────────────────────────────────────────────────────

  return {
    isRestored,
    saveStatus,
    error,
    diagnostics: {
      lastSavedHash,
      sceneHash,
      forwardVersionMode,
      events,
      flushSave,
      clearStorage,
      readStoreMetadata,
    },
  };
}
