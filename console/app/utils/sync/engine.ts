import { reactive } from "vue";
import { $fetch } from "ofetch";
import { invoke } from "../invoke";
import { useAuthStore } from "../../stores/auth";
import { useWorkspacesStore } from "../../stores/workspaces";
import { useNoteStore } from "../../stores/notes";
import { useTaskStore } from "../../stores/tasks";
import { useBookmarkStore } from "../../stores/bookmarks";
import { useSnippetStore } from "../../stores/snippets";
import { useReminderStore } from "../../stores/reminder";
import { usePasteBinStore } from "../../stores/paste-bin";
import { useWhiteboardStore } from "../../stores/whiteboards";
import { useUserPreferenceStore } from "../../stores/workspace-profile";
import { useRecycleBinStore } from "../../stores/recycle-bin";

export interface SyncQueueItem {
  identifier: string;
  tableName: string;
  recordIdentifier: string;
  operation: string;
  workspaceIdentifier: string | null;
  createdAt: string;
  attempts: number;
  lastError: string | null;
  deviceId: string | null;
  payload?: unknown;
}

export interface SyncState {
  workspaceIdentifier: string;
  lastCursor: string;
  lastSyncedAt: string | null;
  lastError: string | null;
}

export interface RemoteSyncChange {
  tableName: string;
  recordIdentifier: string;
  operation: string;
  payload?: unknown;
  seq?: string;
  updatedAt?: string;
}

interface PushResult {
  applied: string[];
  errors: string[];
}

interface RawSyncChange {
  seq: string;
  table_name: string;
  record_identifier: string | null;
  operation: string;
  payload?: unknown;
  updated_at: string;
}

interface RawPullResult {
  changes: RawSyncChange[];
  next_cursor: string;
  has_more: boolean;
}

export const syncStatus = reactive({
  syncing: false,
  pending: 0,
  lastSyncedAt: null as string | null,
  error: null as string | null,
});

let baseUrl = "";
let graphqlEndpoint = "";
let timer: ReturnType<typeof setInterval> | null = null;
let debounce: ReturnType<typeof setTimeout> | null = null;
let running = false;

const PUSH = `
  mutation SyncPush($workspace_identifier: String!, $changes: [SyncChangeInput!]!) {
    sync_push(workspace_identifier: $workspace_identifier, changes: $changes) {
      applied
      errors
    }
  }
`;

const PULL = `
  query SyncPull($workspace_identifier: String!, $since: String!, $limit: Int) {
    sync_pull(workspace_identifier: $workspace_identifier, since: $since, limit: $limit) {
      changes { seq table_name record_identifier operation payload updated_at }
      next_cursor
      has_more
    }
  }
`;

function graphqlBase(): string {
  return baseUrl
    .replace(/\/+$/, "")
    .replace(/\/orchard\/graphql$/, "")
    .replace(/\/orchard$/, "");
}

async function request<T>(
  query: string,
  variables: Record<string, unknown>,
  token: string,
): Promise<T> {
  const candidates = graphqlEndpoint
    ? [graphqlEndpoint]
    : ["/orchard/graphql", "/orchard"];
  let lastError: unknown;

  for (const endpoint of candidates) {
    try {
      const response = await $fetch<{
        data?: T;
        errors?: { message: string }[];
      }>(`${graphqlBase()}${endpoint}`, {
        method: "POST",
        headers: {
          Authorization: `Bearer ${token}`,
          "Content-Type": "application/json",
        },
        body: { query, variables },
      });
      graphqlEndpoint = endpoint;
      if (response.errors?.length) {
        throw new Error(
          response.errors.map((error) => error.message).join("; "),
        );
      }
      if (!response.data) throw new Error("empty sync response");
      return response.data;
    } catch (error) {
      lastError = error;
      const status =
        (error as { statusCode?: number; status?: number }).statusCode ??
        (error as { status?: number }).status;
      if (status !== 404) throw error;
    }
  }

  throw lastError instanceof Error
    ? lastError
    : new Error("sync endpoint not found");
}

function normalizeUuid(value: string): string {
  return value.replace(/-/g, "").toLowerCase();
}

function dashedUuid(value: string): string {
  const hex = normalizeUuid(value);
  if (hex.length !== 32) return value;
  return `${hex.slice(0, 8)}-${hex.slice(8, 12)}-${hex.slice(12, 16)}-${hex.slice(16, 20)}-${hex.slice(20)}`;
}

async function coalesce(
  workspaceIdentifier: string,
): Promise<
  { change: RemoteSyncChange; identifiers: string[]; latest: string }[]
> {
  const items = await invoke<SyncQueueItem[]>("sync_queue_list", {
    workspaceIdentifier,
  });
  const groups = new Map<
    string,
    { change: RemoteSyncChange; identifiers: string[]; latest: string }
  >();

  for (const item of items) {
    const key = `${item.tableName}:${normalizeUuid(item.recordIdentifier)}`;
    const existing = groups.get(key);
    const operation =
      item.operation === "DELETE" || !item.payload ? "DELETE" : item.operation;
    groups.set(key, {
      change: {
        tableName: item.tableName,
        recordIdentifier: dashedUuid(item.recordIdentifier),
        operation,
        payload: operation === "DELETE" ? {} : item.payload,
      },
      identifiers: [...(existing?.identifiers ?? []), item.identifier],
      latest: item.identifier,
    });
  }

  return [...groups.values()];
}

async function push(workspaceIdentifier: string, token: string) {
  const groups = await coalesce(workspaceIdentifier);
  if (groups.length === 0) return;

  const data = await request<{ sync_push: PushResult }>(
    PUSH,
    {
      workspace_identifier: workspaceIdentifier,
      changes: groups.map((group) => ({
        table_name: group.change.tableName,
        record_identifier: group.change.recordIdentifier,
        operation: group.change.operation,
        payload: group.change.payload ?? {},
      })),
    },
    token,
  );
  const applied = new Set(data.sync_push.applied.map(normalizeUuid));
  const succeeded: string[] = [];
  const failed: typeof groups = [];

  for (const group of groups) {
    if (applied.has(normalizeUuid(group.change.recordIdentifier))) {
      succeeded.push(...group.identifiers);
    } else {
      failed.push(group);
    }
  }

  if (succeeded.length > 0) {
    await invoke("sync_queue_ack", { identifiers: succeeded });
  }

  if (failed.length === 0) return;

  const errors = data.sync_push.errors.join("; ") || "change was not applied";
  syncStatus.error = errors;
  for (const group of failed) {
    await invoke("sync_queue_fail", {
      identifier: group.latest,
      error: errors,
    });
  }
}

async function pull(
  workspaceIdentifier: string,
  token: string,
): Promise<number> {
  const state = await invoke<SyncState>("sync_state_get", {
    workspaceIdentifier,
  });
  let cursor = state.lastCursor || "0";
  let appliedCount = 0;

  for (;;) {
    const data = await request<{ sync_pull: RawPullResult }>(
      PULL,
      { workspace_identifier: workspaceIdentifier, since: cursor, limit: 500 },
      token,
    );
    const result = data.sync_pull;
    const changes: RemoteSyncChange[] = result.changes.map((change) => ({
      tableName: change.table_name,
      recordIdentifier: change.record_identifier ?? "",
      operation: change.operation,
      payload: change.payload,
      seq: change.seq,
      updatedAt: change.updated_at,
    }));
    if (changes.length > 0) {
      const applied = await invoke<{ applied: number; errors: string[] }>(
        "sync_apply_changes",
        { workspaceIdentifier, changes },
      );
      appliedCount += applied.applied;
      if (applied.errors.length > 0) {
        throw new Error(applied.errors.join("; "));
      }
    }
    cursor = result.next_cursor || cursor;
    await invoke("sync_state_set", {
      workspaceIdentifier,
      cursor,
      syncedAt: new Date().toISOString(),
      error: null,
    });
    syncStatus.lastSyncedAt = new Date().toISOString();
    if (!result.has_more) break;
  }

  const latest = await invoke<SyncState>("sync_state_get", {
    workspaceIdentifier,
  });
  syncStatus.pending = 0;
  syncStatus.lastSyncedAt = latest.lastSyncedAt ?? syncStatus.lastSyncedAt;
  return appliedCount;
}

async function refreshStores() {
  await Promise.allSettled([
    useWorkspacesStore().fetchWorkspaces(),
    useNoteStore().fetchNotes(),
    useTaskStore().fetchTasks(),
    useBookmarkStore().fetchBookmarks(),
    useSnippetStore().fetchSnippets(),
    useReminderStore().fetchReminders(),
    usePasteBinStore().fetchPasteBins(),
    useWhiteboardStore().fetchWhiteboards(),
    useUserPreferenceStore().fetchPreference(),
    useRecycleBinStore().fetchEntries(),
  ]);
}

export async function syncNow(): Promise<void> {
  if (running) return;
  const auth = useAuthStore();
  const workspaces = useWorkspacesStore();
  const workspaceIdentifier = workspaces.activeWorkspaceId;
  if (!auth.accessToken || auth.isGuest || !workspaceIdentifier) return;
  if (typeof navigator !== "undefined" && navigator.onLine === false) return;

  running = true;
  syncStatus.syncing = true;
  syncStatus.error = null;
  try {
    await invoke("sync_clear_applying");
    await push(workspaceIdentifier, auth.accessToken);
    const applied = await pull(workspaceIdentifier, auth.accessToken);
    const remaining = await invoke<SyncQueueItem[]>("sync_queue_list", {
      workspaceIdentifier,
    });
    syncStatus.pending = remaining.length;
    if (applied > 0 || remaining.length === 0) await refreshStores();
  } catch (error) {
    syncStatus.error = (error as Error).message;
  } finally {
    syncStatus.syncing = false;
    running = false;
  }
}

export function scheduleSync(delay = 500): void {
  if (debounce) clearTimeout(debounce);
  debounce = setTimeout(() => {
    debounce = null;
    void syncNow();
  }, delay);
}

export function startSync(apiBaseUrl: string): void {
  baseUrl = apiBaseUrl;
  if (timer) clearInterval(timer);
  timer = setInterval(() => void syncNow(), 60_000);

  if (typeof window !== "undefined") {
    window.addEventListener("online", () => void syncNow());
    window.addEventListener("focus", () => void syncNow());
  }

  void syncNow();
}

export function stopSync(): void {
  if (timer) clearInterval(timer);
  timer = null;
  if (debounce) clearTimeout(debounce);
  debounce = null;
}
