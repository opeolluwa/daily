import { BaseRepository } from "../base";

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
  operation: "INSERT" | "UPDATE" | "DELETE";
  payload?: unknown;
  seq?: string;
  updatedAt?: string;
}

export interface ApplyChangesResult {
  applied: number;
  errors: string[];
}

const RECYCLE_BIN_TABLES: Record<string, string> = {
  notes: "note",
  task: "task",
  bookmark: "bookmark",
  snippets: "snippet",
  reminder: "reminder",
  paste_bins: "paste_bin",
};

const SYNCABLE_TABLES = new Set([
  ...Object.keys(RECYCLE_BIN_TABLES),
  "whiteboards",
  "workspaces",
]);

export class SyncRepository extends BaseRepository {
  async queue(workspaceIdentifier: string): Promise<SyncQueueItem[]> {
    const items = await this.rows<SyncQueueItem>(
      `SELECT identifier, table_name, record_identifier, operation,
              workspace_identifier, created_at, attempts, last_error, device_id
       FROM sync_queue
       WHERE workspace_identifier = $1
       ORDER BY created_at ASC, identifier ASC`,
      [workspaceIdentifier],
    );
    for (const item of items) {
      const payload = await this.payload(item.tableName, item.recordIdentifier);
      if (payload) {
        item.payload = payload;
      } else if (item.operation === "DELETE") {
        const timestamp = item.createdAt || this.now();
        item.payload = {
          identifier: item.recordIdentifier,
          ...(item.tableName === "workspaces"
            ? {}
            : { workspace_identifier: item.workspaceIdentifier }),
          deleted_at: timestamp,
          updated_at: timestamp,
        };
      }
    }
    return items;
  }

  private async payload(
    table: string,
    recordIdentifier: string,
  ): Promise<Record<string, unknown> | null> {
    const db = await this.db();
    const result = await db.query<Record<string, unknown>>(
      `SELECT * FROM "${table}" WHERE identifier = $1`,
      [recordIdentifier],
    );
    const row = result.rows?.[0];
    if (!row) return null;
    const payload: Record<string, unknown> = {};
    for (const [key, value] of Object.entries(row)) {
      payload[key] = value instanceof Date ? value.toISOString() : value;
    }
    return payload;
  }

  async ack(identifiers: string[]): Promise<void> {
    if (identifiers.length === 0) return;
    const placeholders = identifiers
      .map((_, index) => `$${index + 1}`)
      .join(", ");
    await this.run(
      `DELETE FROM sync_queue WHERE identifier IN (${placeholders})`,
      identifiers,
    );
  }

  async fail(identifier: string, error: string): Promise<void> {
    await this.run(
      `UPDATE sync_queue SET attempts = attempts + 1, last_error = $2
       WHERE identifier = $1`,
      [identifier, error],
    );
  }

  async state(workspaceIdentifier: string): Promise<SyncState> {
    const row = await this.row<SyncState>(
      `SELECT workspace_identifier, last_cursor::text AS last_cursor,
              last_synced_at, last_error
       FROM sync_state WHERE workspace_identifier = $1`,
      [workspaceIdentifier],
    );
    return (
      row ?? {
        workspaceIdentifier,
        lastCursor: "0",
        lastSyncedAt: null,
        lastError: null,
      }
    );
  }

  async setState(
    workspaceIdentifier: string,
    cursor: string,
    syncedAt: string | null,
    error: string | null,
  ): Promise<void> {
    await this.run(
      `INSERT INTO sync_state
         (workspace_identifier, last_cursor, last_synced_at, last_error)
       VALUES ($1, $2, $3, $4)
       ON CONFLICT (workspace_identifier) DO UPDATE SET
         last_cursor = excluded.last_cursor,
         last_synced_at = excluded.last_synced_at,
         last_error = excluded.last_error`,
      [workspaceIdentifier, cursor, syncedAt, error],
    );
  }

  async setApplying(applying: boolean): Promise<void> {
    if (!applying) {
      await this.run("DELETE FROM sync_meta WHERE key = 'applying'");
      return;
    }
    await this.run(
      `INSERT INTO sync_meta (key, value) VALUES ('applying', '1')
       ON CONFLICT (key) DO UPDATE SET value = excluded.value`,
    );
  }

  async clearApplying(): Promise<void> {
    await this.setApplying(false);
  }

  async deviceId(): Promise<string> {
    const row = await this.row<{ value: string }>(
      `SELECT value FROM sync_meta WHERE key = 'device_id'`,
    );
    if (row?.value) return row.value;

    const deviceId = this.newId();
    await this.run(
      `INSERT INTO sync_meta (key, value) VALUES ('device_id', $1)
       ON CONFLICT (key) DO UPDATE SET value = excluded.value`,
      [deviceId],
    );
    return deviceId;
  }

  private async columns(table: string): Promise<Map<string, string>> {
    const rows = await this.rows<{
      columnName: string;
      dataType: string;
      udtName: string;
    }>(
      `SELECT column_name, data_type, udt_name
       FROM information_schema.columns WHERE table_name = $1`,
      [table],
    );
    return new Map(
      rows.map((row) => [
        row.columnName,
        row.udtName === "jsonb" || row.udtName === "json"
          ? "json"
          : row.dataType,
      ]),
    );
  }

  private value(value: unknown, type: string | undefined): unknown {
    if (value === null || value === undefined) return null;
    if (type === "json") return JSON.stringify(value);
    if (typeof value === "object") return JSON.stringify(value);
    return value;
  }

  async applyChanges(
    workspaceIdentifier: string,
    changes: RemoteSyncChange[],
  ): Promise<ApplyChangesResult> {
    if (changes.length === 0) return { applied: 0, errors: [] };

    await this.setApplying(true);
    const result: ApplyChangesResult = { applied: 0, errors: [] };
    const ordered = [...changes].sort((left, right) =>
      left.tableName === "workspaces"
        ? -1
        : right.tableName === "workspaces"
          ? 1
          : 0,
    );

    for (const change of ordered) {
      try {
        await this.applyChange(workspaceIdentifier, change);
        result.applied += 1;
      } catch (error) {
        result.errors.push(
          `${change.tableName} ${change.recordIdentifier}: ${(error as Error).message}`,
        );
      }
    }

    await this.setApplying(false);
    return result;
  }

  private tombstoneTimestamp(change: RemoteSyncChange): string {
    const payload = (change.payload ?? {}) as Record<string, unknown>;
    const value = payload.deleted_at ?? payload.updated_at ?? change.updatedAt;
    return typeof value === "string" && value ? value : this.now();
  }

  private async applyChange(
    workspaceIdentifier: string,
    change: RemoteSyncChange,
  ): Promise<void> {
    if (change.operation === "DELETE") {
      if (!SYNCABLE_TABLES.has(change.tableName)) {
        throw new Error(`unknown sync table: ${change.tableName}`);
      }

      const timestamp = this.tombstoneTimestamp(change);
      const isWorkspace = change.tableName === "workspaces";
      const hadContent =
        !!change.payload &&
        typeof change.payload === "object" &&
        Object.keys(change.payload as Record<string, unknown>).length > 1;
      const payload: Record<string, unknown> = {
        ...((change.payload as Record<string, unknown> | undefined) ?? {}),
        identifier: change.recordIdentifier,
        deleted_at: timestamp,
        updated_at:
          (change.payload as Record<string, unknown> | undefined)?.updated_at ??
          timestamp,
      };
      if (!isWorkspace) payload.workspace_identifier = workspaceIdentifier;

      const sql = isWorkspace
        ? `UPDATE "workspaces" SET deleted_at = $2, updated_at = $2, is_hidden = TRUE
           WHERE identifier = $1
             AND (updated_at IS NULL OR updated_at <= $2::timestamptz)
           RETURNING identifier`
        : `UPDATE "${change.tableName}" SET deleted_at = $2, updated_at = $2
           WHERE identifier = $1
             AND (updated_at IS NULL OR updated_at <= $2::timestamptz)
           RETURNING identifier`;
      const updated = await this.rows<{ identifier: string }>(sql, [
        change.recordIdentifier,
        timestamp,
      ]);

      if (updated.length === 0) {
        const exists = await this.bool(
          `SELECT EXISTS (SELECT 1 FROM "${change.tableName}" WHERE identifier = $1) AS ok`,
          [change.recordIdentifier],
        );
        if (exists) return;
        if (!hadContent) return;
        await this.upsert(change.tableName, change.recordIdentifier, payload);
        if (!isWorkspace) {
          await this.stageRecycleBin(
            workspaceIdentifier,
            change.tableName,
            payload,
          );
        }
      } else if (!isWorkspace) {
        await this.stageRecycleBin(
          workspaceIdentifier,
          change.tableName,
          payload,
        );
      }

      if (isWorkspace) {
        await this.run(
          `UPDATE "workspaces" SET is_default = TRUE
           WHERE identifier = (
             SELECT identifier FROM "workspaces"
             WHERE deleted_at IS NULL ORDER BY created_at ASC LIMIT 1
           )
           AND NOT EXISTS (
             SELECT 1 FROM "workspaces"
             WHERE deleted_at IS NULL AND is_default = TRUE
           )`,
        );
      }
      return;
    }

    if (!change.payload || typeof change.payload !== "object") {
      throw new Error("sync change is missing a payload");
    }

    const payload: Record<string, unknown> = {
      ...(change.payload as Record<string, unknown>),
    };
    if (change.tableName !== "workspaces") {
      payload.workspace_identifier = workspaceIdentifier;
    }

    await this.upsert(change.tableName, change.recordIdentifier, payload);
    await this.stageRecycleBin(workspaceIdentifier, change.tableName, payload);
  }

  private async upsert(
    tableName: string,
    recordIdentifier: string,
    payload: Record<string, unknown>,
  ): Promise<void> {
    const allowed = await this.columns(tableName);
    if (allowed.size === 0) throw new Error(`unknown sync table: ${tableName}`);

    const columns = Object.keys(payload).filter((key) => allowed.has(key));
    if (!columns.includes("identifier")) columns.unshift("identifier");
    const values = columns.map((column) =>
      column === "identifier"
        ? (payload[column] ?? recordIdentifier)
        : this.value(payload[column], allowed.get(column)),
    );

    const placeholders = columns.map((_, index) => `$${index + 1}`).join(", ");
    const quoted = columns.map((column) => `"${column}"`).join(", ");
    const updates = columns
      .filter((column) => column !== "identifier")
      .map((column) => `"${column}" = excluded."${column}"`)
      .join(", ");

    const sql =
      updates.length === 0
        ? `INSERT INTO "${tableName}" (${quoted}) VALUES (${placeholders})
           ON CONFLICT (identifier) DO NOTHING`
        : `INSERT INTO "${tableName}" (${quoted}) VALUES (${placeholders})
           ON CONFLICT (identifier) DO UPDATE SET ${updates}
           WHERE excluded."updated_at" >= "${tableName}"."updated_at"`;

    await this.run(sql, values);
  }

  private async stageRecycleBin(
    workspaceIdentifier: string,
    tableName: string,
    payload: Record<string, unknown>,
  ): Promise<void> {
    const itemType = RECYCLE_BIN_TABLES[tableName];
    if (!itemType) return;
    const identifier = String(payload.identifier ?? "");
    if (!identifier) return;

    if (payload.deleted_at === null || payload.deleted_at === undefined) {
      await this.run(
        "DELETE FROM recycle_bin WHERE item_id = $1 AND item_type = $2",
        [identifier, itemType],
      );
      return;
    }

    const deletedAt = String(payload.deleted_at);
    const owner = String(payload.workspace_identifier ?? workspaceIdentifier);
    const updated = await this.rows<{ identifier: string }>(
      `UPDATE recycle_bin
       SET payload = $1, deleted_at = $2, workspace_identifier = $3
       WHERE item_id = $4 AND item_type = $5
       RETURNING identifier`,
      [JSON.stringify(payload), deletedAt, owner, identifier, itemType],
    );
    if (updated.length > 0) return;

    await this.run(
      `INSERT INTO recycle_bin
         (identifier, item_id, item_type, payload, deleted_at, workspace_identifier)
       VALUES ($1, $2, $3, $4, $5, $6)`,
      [
        this.newId(),
        identifier,
        itemType,
        JSON.stringify(payload),
        deletedAt,
        owner,
      ],
    );
  }
}
