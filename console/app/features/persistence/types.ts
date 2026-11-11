import type { ExcalidrawElement } from "@drawvue/core";
import type { ComputedRef, InjectionKey, Ref, ShallowRef } from "vue";

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

export const ELEMENT_COUNT_WARN_THRESHOLD = 3000;
export const CURRENT_SCHEMA_VERSION = 1 as const;

// ---------------------------------------------------------------------------
// Persisted scene — discriminated union by schemaVersion
// ---------------------------------------------------------------------------

export type PersistedSceneV1 = {
  schemaVersion: 1;
  elements: ExcalidrawElement[];
  savedAt: number;
  elementCount: number;
};

export type PersistedScene = PersistedSceneV1;

export type CurrentPersistedScene = PersistedSceneV1;

// ---------------------------------------------------------------------------
// Migration map — compiler-enforced entry for every version except current
// ---------------------------------------------------------------------------

export type MigrationMap = {
  [V in PersistedScene["schemaVersion"] as V extends typeof CURRENT_SCHEMA_VERSION ? never : V]: (
    data: Extract<PersistedScene, { schemaVersion: V }>,
  ) => CurrentPersistedScene;
};

// ---------------------------------------------------------------------------
// Save status state machine
// ---------------------------------------------------------------------------

export type SaveStatus = "idle" | "pending" | "saving" | "unavailable" | "error";

// ---------------------------------------------------------------------------
// Persistence event log
// ---------------------------------------------------------------------------

export type PersistenceEventType =
  | "save"
  | "save-skip"
  | "debounce"
  | "change"
  | "restore"
  | "error"
  | "clear"
  | "export";

export type PersistenceEvent = {
  id: number;
  type: PersistenceEventType;
  timestamp: number;
  message: string;
};

// ---------------------------------------------------------------------------
// Store metadata (for inspector display) — read back from the database
// ---------------------------------------------------------------------------

export type StoreMetadata = {
  whiteboard: {
    identifier: string;
    title: string | null;
    schemaVersion: number;
    elementCount: number;
    savedAt: number;
    dataSize: number;
    updatedAt: string;
  } | null;
  files: {
    count: number;
    totalSize: number;
  } | null;
};

// ---------------------------------------------------------------------------
// Diagnostics (exposed from usePersistence for inspector)
// ---------------------------------------------------------------------------

export type PersistenceDiagnostics = {
  lastSavedHash: Readonly<Ref<number>>;
  sceneHash: Readonly<ComputedRef<number>>;
  forwardVersionMode: Readonly<Ref<boolean>>;
  events: Readonly<ShallowRef<readonly PersistenceEvent[]>>;
  flushSave: () => void;
  clearStorage: () => Promise<void>;
  readStoreMetadata: () => Promise<StoreMetadata>;
};

// ---------------------------------------------------------------------------
// Inspector context (provided by PersistenceProvider)
// ---------------------------------------------------------------------------

export type PersistenceInspectorContext = {
  saveStatus: Readonly<Ref<SaveStatus>>;
  isRestored: Readonly<Ref<boolean>>;
  error: Readonly<ShallowRef<Error | null>>;
  diagnostics: PersistenceDiagnostics;
};

export const PERSISTENCE_INSPECTOR_KEY: InjectionKey<PersistenceInspectorContext> =
  Symbol("persistence-inspector");

// ---------------------------------------------------------------------------
// Composable return type
// ---------------------------------------------------------------------------

export type UsePersistenceReturn = {
  isRestored: Readonly<Ref<boolean>>;
  saveStatus: Readonly<Ref<SaveStatus>>;
  error: Readonly<ShallowRef<Error | null>>;
  diagnostics: PersistenceDiagnostics;
};
