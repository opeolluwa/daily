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
  task: "todo",
  bookmark: "bookmark",
  snippets: "snippet",
  reminder: "reminder",
  paste_bins: "paste_bin",
};

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
      if (item.operation !== "DELETE") {
        item.payload = await this.payload(
          item.tableName,
          item.recordIdentifier,
        );
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

    const deviceId = this.newUuid();
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

  private async applyChange(
    workspaceIdentifier: string,
    change: RemoteSyncChange,
  ): Promise<void> {
    if (change.operation === "DELETE") {
      const itemType = RECYCLE_BIN_TABLES[change.tableName];
      if (itemType) {
        await this.run(
          "DELETE FROM recycle_bin WHERE item_id = $1 AND item_type = $2",
          [change.recordIdentifier, itemType],
        );
      }
      await this.run(
        `DELETE FROM "${change.tableName}" WHERE identifier = $1`,
        [change.recordIdentifier],
      );
      return;
    }

    if (!change.payload || typeof change.payload !== "object") {
      throw new Error("sync change is missing a payload");
    }

    const allowed = await this.columns(change.tableName);
    if (allowed.size === 0)
      throw new Error(`unknown sync table: ${change.tableName}`);

    const payload = change.payload as Record<string, unknown>;
    const columns = Object.keys(payload).filter((key) => allowed.has(key));
    if (!columns.includes("identifier")) columns.unshift("identifier");
    const values = columns.map((column) =>
      column === "identifier"
        ? (payload[column] ?? change.recordIdentifier)
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
        ? `INSERT INTO "${change.tableName}" (${quoted}) VALUES (${placeholders})
           ON CONFLICT (identifier) DO NOTHING`
        : `INSERT INTO "${change.tableName}" (${quoted}) VALUES (${placeholders})
           ON CONFLICT (identifier) DO UPDATE SET ${updates}
           WHERE excluded."updated_at" >= "${change.tableName}"."updated_at"`;

    await this.run(sql, values);
    await this.stageRecycleBin(workspaceIdentifier, change.tableName, payload);
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

    await this.run(
      `INSERT INTO recycle_bin
         (identifier, item_id, item_type, payload, deleted_at, workspace_identifier)
       SELECT $1, $2, $3, $4, $5, $6
       WHERE NOT EXISTS (
         SELECT 1 FROM recycle_bin WHERE item_id = $2 AND item_type = $3
       )`,
      [
        this.newUuid(),
        identifier,
        itemType,
        JSON.stringify(payload),
        this.now(),
        workspaceIdentifier,
      ],
    );
  }
}
