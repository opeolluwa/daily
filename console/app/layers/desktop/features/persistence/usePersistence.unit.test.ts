import { nextTick } from "vue";
import { withDrawVue, createTestElement } from "@drawvue/core/test-utils";
import { usePersistence } from "./usePersistence";
import { CURRENT_SCHEMA_VERSION } from "./types";
import type { SaveStatus } from "./types";
import type { Whiteboard, WhiteboardFiles } from "@shared/stores/whiteboards";

// ---------------------------------------------------------------------------
// Mocks
// ---------------------------------------------------------------------------

const whiteboardStoreMock = vi.hoisted(() => ({
  getWhiteboard: vi.fn(),
  updateWhiteboard: vi.fn(),
  saveWhiteboardFile: vi.fn(),
  getWhiteboardFiles: vi.fn(),
  deleteWhiteboardFile: vi.fn(),
}));

vi.mock("@shared/stores/whiteboards", () => ({
  useWhiteboardStore: () => whiteboardStoreMock,
}));

// Capture debounce and event listener callbacks
let debouncedFn!: (...args: unknown[]) => unknown;
const eventListenerHandlers: Record<string, () => void> = {};

vi.mock("@vueuse/core", async (importOriginal) => {
  const actual = await importOriginal<typeof import("@vueuse/core")>();
  return {
    ...actual,
    useDebounceFn: vi.fn((fn: (...args: unknown[]) => unknown) => {
      debouncedFn = fn;
      return fn;
    }),
    useEventListener: vi.fn((target: unknown, event: string, handler: () => void) => {
      const key =
        typeof target === "object" && target !== null && "hidden" in target
          ? `document:${event}`
          : event;
      eventListenerHandlers[key] = handler;
    }),
    useIntervalFn: vi.fn(),
  };
});

// ---------------------------------------------------------------------------
// Fixtures
// ---------------------------------------------------------------------------

const WHITEBOARD_ID = "wb-1";

function makeWhiteboard(overrides: Partial<Whiteboard> = {}): Whiteboard {
  return {
    identifier: WHITEBOARD_ID,
    title: null,
    payload: null,
    createdAt: "2026-01-01T00:00:00Z",
    updatedAt: "2026-01-01T00:00:00Z",
    workspaceIdentifier: "ws-1",
    ...overrides,
  };
}

function makeFiles(count: number, dataLength = 10): WhiteboardFiles[] {
  return Array.from({ length: count }, (_, i) => ({
    identifier: `file-${i}`,
    whiteboardIdentifier: WHITEBOARD_ID,
    mimeType: "image/png",
    data: "x".repeat(dataLength),
    createdAt: "2026-01-01T00:00:00Z",
    updatedAt: "2026-01-01T00:00:00Z",
  }));
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function setupDefaults(): void {
  whiteboardStoreMock.getWhiteboard.mockResolvedValue(makeWhiteboard());
  whiteboardStoreMock.getWhiteboardFiles.mockResolvedValue([]);
  whiteboardStoreMock.updateWhiteboard.mockResolvedValue(makeWhiteboard());
  whiteboardStoreMock.saveWhiteboardFile.mockResolvedValue(undefined);
  whiteboardStoreMock.deleteWhiteboardFile.mockResolvedValue(undefined);
}

async function waitForRestore(ctx: { isRestored: { value: boolean } }): Promise<void> {
  await vi.waitFor(
    () => {
      expect(ctx.isRestored.value).toBe(true);
    },
    { timeout: 100 },
  );
}

async function triggerSave(): Promise<void> {
  await debouncedFn();
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

describe("usePersistence", () => {
  // eslint-disable-next-line vitest/no-hooks -- clearing mocks & stubs is necessary for isolated tests
  beforeEach(async () => {
    // Drain floating promises from previous test's onScopeDispose → flushSave → executeSave
    await new Promise((r) => setTimeout(r, 0));
    vi.clearAllMocks();
    // Stub `document` for node env — useEventListener(document, ...) evaluates the argument eagerly
    vi.stubGlobal("document", { hidden: false });
    // Reset captured handlers
    for (const key of Object.keys(eventListenerHandlers)) {
      delete eventListenerHandlers[key];
    }
  });

  // eslint-disable-next-line vitest/no-hooks -- cleanup global stubs
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  describe("restore flow", () => {
    it("sets error and disables saves when the whiteboard is not found", async () => {
      whiteboardStoreMock.getWhiteboard.mockResolvedValue(null);

      using ctx = withDrawVue(() => usePersistence(WHITEBOARD_ID));
      await waitForRestore(ctx);

      expect(ctx.isRestored.value).toBe(true);
      expect(ctx.saveStatus.value).toBe("error" satisfies SaveStatus);
      expect(ctx.error.value).toBeInstanceOf(Error);
      expect(ctx.error.value!.message).toBe("Whiteboard not found");

      ctx.drawVue.elements.addElement(createTestElement({ id: "blocked" }));
      await nextTick();
      await triggerSave();
      expect(whiteboardStoreMock.updateWhiteboard).not.toHaveBeenCalled();
    });

    it("enters forward version mode when the stored schema is ahead", async () => {
      const futurePayload = {
        schemaVersion: CURRENT_SCHEMA_VERSION + 1,
        elements: [createTestElement({ id: "future" })],
        savedAt: 2000,
        elementCount: 1,
      };
      whiteboardStoreMock.getWhiteboard.mockResolvedValue(
        makeWhiteboard({ payload: futurePayload }),
      );

      using ctx = withDrawVue(() => usePersistence(WHITEBOARD_ID));
      await waitForRestore(ctx);

      expect(ctx.diagnostics.forwardVersionMode.value).toBe(true);
      // Elements are still displayed read-only
      expect(ctx.drawVue.elements.elements.value).toHaveLength(1);

      ctx.drawVue.elements.addElement(createTestElement({ id: "blocked" }));
      await nextTick();
      await triggerSave();
      expect(whiteboardStoreMock.updateWhiteboard).not.toHaveBeenCalled();
    });

    it("restores elements from the whiteboard payload into the canvas", async () => {
      const payload = {
        schemaVersion: CURRENT_SCHEMA_VERSION,
        elements: [
          createTestElement({ id: "a" }),
          createTestElement({ id: "b" }),
          null as unknown as never,
        ],
        savedAt: 1000,
        elementCount: 3,
      };
      whiteboardStoreMock.getWhiteboard.mockResolvedValue(makeWhiteboard({ payload }));

      using ctx = withDrawVue(() => usePersistence(WHITEBOARD_ID));
      await waitForRestore(ctx);

      expect(ctx.drawVue.elements.elements.value).toHaveLength(2);
      expect(whiteboardStoreMock.getWhiteboard).toHaveBeenCalledWith(WHITEBOARD_ID);
      expect(whiteboardStoreMock.getWhiteboardFiles).toHaveBeenCalledWith(WHITEBOARD_ID);
    });

    it("starts empty when the payload is not a valid scene", async () => {
      whiteboardStoreMock.getWhiteboard.mockResolvedValue(
        makeWhiteboard({ payload: { nonsense: true } }),
      );

      using ctx = withDrawVue(() => usePersistence(WHITEBOARD_ID));
      await waitForRestore(ctx);

      expect(ctx.drawVue.elements.elements.value).toHaveLength(0);
      expect(ctx.saveStatus.value).toBe("idle" satisfies SaveStatus);
    });

    it("initializes lastSavedHash from restored elements and skips unchanged saves", async () => {
      const payload = {
        schemaVersion: CURRENT_SCHEMA_VERSION,
        elements: [createTestElement({ id: "hash-el" })],
        savedAt: 1000,
        elementCount: 1,
      };
      whiteboardStoreMock.getWhiteboard.mockResolvedValue(makeWhiteboard({ payload }));

      using ctx = withDrawVue(() => usePersistence(WHITEBOARD_ID));
      await waitForRestore(ctx);

      expect(ctx.diagnostics.lastSavedHash.value).toBeGreaterThan(0);

      await triggerSave();
      expect(whiteboardStoreMock.updateWhiteboard).not.toHaveBeenCalled();
      expect(ctx.saveStatus.value).toBe("idle" satisfies SaveStatus);
    });

    it("sets error status when loading the whiteboard fails", async () => {
      whiteboardStoreMock.getWhiteboard.mockRejectedValue(new Error("db offline"));

      using ctx = withDrawVue(() => usePersistence(WHITEBOARD_ID));
      await waitForRestore(ctx);

      expect(ctx.saveStatus.value).toBe("error" satisfies SaveStatus);
      expect(ctx.error.value).toBeInstanceOf(Error);
    });
  });

  describe("save scheduling", () => {
    it("does not save before restore completes", async () => {
      whiteboardStoreMock.getWhiteboard.mockReturnValue(new Promise(() => {}));

      using ctx = withDrawVue(() => usePersistence(WHITEBOARD_ID));
      await nextTick();
      expect(ctx.isRestored.value).toBe(false);

      await triggerSave();
      expect(whiteboardStoreMock.updateWhiteboard).not.toHaveBeenCalled();
    });

    it("does not save in forward version mode", async () => {
      const futurePayload = {
        schemaVersion: CURRENT_SCHEMA_VERSION + 1,
        elements: [],
        savedAt: 2000,
        elementCount: 0,
      };
      whiteboardStoreMock.getWhiteboard.mockResolvedValue(
        makeWhiteboard({ payload: futurePayload }),
      );

      using ctx = withDrawVue(() => usePersistence(WHITEBOARD_ID));
      await waitForRestore(ctx);

      ctx.drawVue.elements.addElement(createTestElement({ id: "blocked" }));
      await nextTick();
      await triggerSave();

      expect(whiteboardStoreMock.updateWhiteboard).not.toHaveBeenCalled();
    });
  });

  describe("save execution", () => {
    it("persists the scene payload through the store on change", async () => {
      setupDefaults();

      using ctx = withDrawVue(() => usePersistence(WHITEBOARD_ID));
      await waitForRestore(ctx);

      ctx.drawVue.elements.addElement(createTestElement({ id: "saved" }));
      await nextTick();
      await triggerSave();

      expect(whiteboardStoreMock.updateWhiteboard).toHaveBeenCalledWith(
        WHITEBOARD_ID,
        expect.objectContaining({
          payload: expect.objectContaining({
            schemaVersion: CURRENT_SCHEMA_VERSION,
            elementCount: 1,
          }),
        }),
      );
      expect(ctx.saveStatus.value).toBe("idle" satisfies SaveStatus);
      expect(ctx.error.value).toBeNull();
    });

    it("sets error status on save failure", async () => {
      setupDefaults();
      whiteboardStoreMock.updateWhiteboard.mockRejectedValue(new Error("write failed"));

      using ctx = withDrawVue(() => usePersistence(WHITEBOARD_ID));
      await waitForRestore(ctx);

      ctx.drawVue.elements.addElement(createTestElement({ id: "fail" }));
      await nextTick();
      await triggerSave();

      expect(ctx.saveStatus.value).toBe("error" satisfies SaveStatus);
      expect(ctx.error.value).toBeInstanceOf(Error);
      expect(ctx.error.value!.message).toBe("Failed to save drawing to database");
    });

    it("does not retry while in error state", async () => {
      setupDefaults();
      whiteboardStoreMock.updateWhiteboard.mockRejectedValue(new Error("write failed"));

      using ctx = withDrawVue(() => usePersistence(WHITEBOARD_ID));
      await waitForRestore(ctx);

      ctx.drawVue.elements.addElement(createTestElement({ id: "fail" }));
      await nextTick();
      await triggerSave();
      expect(whiteboardStoreMock.updateWhiteboard).toHaveBeenCalledTimes(1);

      // Second save attempt should bail because status is "error"
      ctx.drawVue.elements.addElement(createTestElement({ id: "fail-2" }));
      await nextTick();
      await triggerSave();
      expect(whiteboardStoreMock.updateWhiteboard).toHaveBeenCalledTimes(1);
    });
  });

  describe("page unload handlers", () => {
    it("visibilitychange while hidden flushes a pending save", async () => {
      setupDefaults();

      using ctx = withDrawVue(() => usePersistence(WHITEBOARD_ID));
      await waitForRestore(ctx);

      ctx.drawVue.elements.addElement(createTestElement({ id: "flushed" }));
      await nextTick();
      // Hash changed but debounced save has not run yet

      vi.stubGlobal("document", { hidden: true });
      const handler = eventListenerHandlers["document:visibilitychange"];
      expect(handler).toBeDefined();
      handler!();

      await vi.waitFor(() => {
        expect(whiteboardStoreMock.updateWhiteboard).toHaveBeenCalled();
      });
    });

    it("pagehide flushes a pending save", async () => {
      setupDefaults();

      using ctx = withDrawVue(() => usePersistence(WHITEBOARD_ID));
      await waitForRestore(ctx);

      ctx.drawVue.elements.addElement(createTestElement({ id: "pagehide" }));
      await nextTick();

      const handler = eventListenerHandlers["pagehide"];
      expect(handler).toBeDefined();
      handler!();

      await vi.waitFor(() => {
        expect(whiteboardStoreMock.updateWhiteboard).toHaveBeenCalled();
      });
    });
  });

  describe("diagnostics", () => {
    it("exposes lastSavedHash ref that updates after save", async () => {
      setupDefaults();

      using ctx = withDrawVue(() => usePersistence(WHITEBOARD_ID));
      await waitForRestore(ctx);

      const hashAfterRestore = ctx.diagnostics.lastSavedHash.value;
      expect(hashAfterRestore).toBeGreaterThanOrEqual(0);

      ctx.drawVue.elements.addElement(createTestElement({ id: "hash-test" }));
      await nextTick();
      await triggerSave();

      expect(ctx.diagnostics.lastSavedHash.value).toBe(ctx.diagnostics.sceneHash.value);
      expect(ctx.diagnostics.lastSavedHash.value).not.toBe(hashAfterRestore);
    });

    it("exposes sceneHash computed that changes when elements change", async () => {
      setupDefaults();

      using ctx = withDrawVue(() => usePersistence(WHITEBOARD_ID));
      await waitForRestore(ctx);

      const initialHash = ctx.diagnostics.sceneHash.value;
      ctx.drawVue.elements.addElement(createTestElement({ id: "scene-hash-test" }));
      await nextTick();

      expect(ctx.diagnostics.sceneHash.value).not.toBe(initialHash);
    });

    it("caps event log at 50 entries", async () => {
      setupDefaults();

      const warnSpy = vi.spyOn(console, "warn").mockImplementation(() => {});

      using ctx = withDrawVue(() => usePersistence(WHITEBOARD_ID));
      await waitForRestore(ctx);

      for (let i = 0; i < 60; i++) {
        ctx.drawVue.elements.addElement(createTestElement({ id: `cap-${i}` }));
        await nextTick();
        await triggerSave();
      }

      expect(ctx.diagnostics.events.value.length).toBeLessThanOrEqual(50);
      warnSpy.mockRestore();
    });

    it("clearStorage deletes files and empties the payload", async () => {
      setupDefaults();
      whiteboardStoreMock.getWhiteboardFiles.mockResolvedValue(makeFiles(2));

      using ctx = withDrawVue(() => usePersistence(WHITEBOARD_ID));
      await waitForRestore(ctx);

      await ctx.diagnostics.clearStorage();

      expect(whiteboardStoreMock.deleteWhiteboardFile).toHaveBeenCalledWith("file-0");
      expect(whiteboardStoreMock.deleteWhiteboardFile).toHaveBeenCalledWith("file-1");
      expect(whiteboardStoreMock.updateWhiteboard).toHaveBeenCalledWith(
        WHITEBOARD_ID,
        expect.objectContaining({
          payload: expect.objectContaining({ elementCount: 0 }),
        }),
      );
      expect(ctx.diagnostics.lastSavedHash.value).toBe(0);
    });

    it("readStoreMetadata returns whiteboard and file metadata from the store", async () => {
      setupDefaults();
      const payload = {
        schemaVersion: CURRENT_SCHEMA_VERSION,
        elements: [createTestElement({ id: "a" }), createTestElement({ id: "b" })],
        savedAt: 1_700_000_000_000,
        elementCount: 2,
      };
      whiteboardStoreMock.getWhiteboard.mockResolvedValue(
        makeWhiteboard({ title: "My Board", payload }),
      );
      whiteboardStoreMock.getWhiteboardFiles.mockResolvedValue(makeFiles(2, 25));

      using ctx = withDrawVue(() => usePersistence(WHITEBOARD_ID));
      await waitForRestore(ctx);

      const metadata = await ctx.diagnostics.readStoreMetadata();

      expect(metadata.whiteboard).not.toBeNull();
      expect(metadata.whiteboard!.identifier).toBe(WHITEBOARD_ID);
      expect(metadata.whiteboard!.title).toBe("My Board");
      expect(metadata.whiteboard!.schemaVersion).toBe(CURRENT_SCHEMA_VERSION);
      expect(metadata.whiteboard!.elementCount).toBe(2);
      expect(metadata.whiteboard!.savedAt).toBe(1_700_000_000_000);
      expect(metadata.whiteboard!.dataSize).toBeGreaterThan(0);

      expect(metadata.files).not.toBeNull();
      expect(metadata.files!.count).toBe(2);
      expect(metadata.files!.totalSize).toBe(50);
    });

    it("readStoreMetadata returns nulls when the whiteboard is gone", async () => {
      setupDefaults();
      whiteboardStoreMock.getWhiteboard.mockResolvedValue(null);
      whiteboardStoreMock.getWhiteboardFiles.mockRejectedValue(new Error("db offline"));

      using ctx = withDrawVue(() => usePersistence(WHITEBOARD_ID));
      await waitForRestore(ctx);

      const metadata = await ctx.diagnostics.readStoreMetadata();

      expect(metadata.whiteboard).toBeNull();
      expect(metadata.files).toBeNull();
    });
  });
});
