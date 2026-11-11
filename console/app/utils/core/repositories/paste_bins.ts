import type { CreatePasteBin, PasteBins, UpdatePasteBin } from "@daily/core";
import { BaseRepository, type RequestMeta } from "../base";

export type { CreatePasteBin, UpdatePasteBin };

const COLUMNS = [
  "title",
  "content",
  "is_pinned",
  "workspace_identifier",
  "created_at",
  "updated_at",
];

export class PasteBinRepository extends BaseRepository {
  async create(
    payload: CreatePasteBin,
    meta?: RequestMeta,
  ): Promise<PasteBins> {
    const m = this.requireMeta(meta);
    const now = this.now();
    return this.mustRow<PasteBins>(
      `INSERT INTO paste_bins
         (identifier, title, content, is_pinned, created_at, updated_at, workspace_identifier)
       VALUES ($1, $2, $3, $4, $5, $6, $7)
       RETURNING *`,
      [
        this.newUuid(),
        payload.title ?? null,
        payload.content,
        payload.isPinned ?? false,
        payload.createdAt ?? now,
        payload.updatedAt ?? now,
        m.workspaceIdentifier,
      ],
    );
  }

  async find_by_id(
    identifier: string,
    meta?: RequestMeta,
  ): Promise<PasteBins | null> {
    const m = this.requireMeta(meta);
    return this.row<PasteBins>(
      `SELECT * FROM paste_bins WHERE identifier = $1 AND workspace_identifier = $2`,
      [identifier, m.workspaceIdentifier],
    );
  }

  async find_all(meta?: RequestMeta): Promise<PasteBins[]> {
    const m = this.requireMeta(meta);
    return this.rows<PasteBins>(
      `SELECT * FROM paste_bins WHERE workspace_identifier = $1 ORDER BY updated_at DESC`,
      [m.workspaceIdentifier],
    );
  }

  async recently_added(meta?: RequestMeta): Promise<PasteBins[]> {
    const m = this.requireMeta(meta);
    return this.rows<PasteBins>(
      `SELECT * FROM paste_bins WHERE workspace_identifier = $1
       ORDER BY created_at DESC LIMIT 10`,
      [m.workspaceIdentifier],
    );
  }

  async update(
    identifier: string,
    payload: UpdatePasteBin,
    meta?: RequestMeta,
  ): Promise<PasteBins> {
    const m = this.requireMeta(meta);
    const sets: string[] = ["updated_at = $2"];
    const params: unknown[] = [identifier, this.now()];
    let idx = 3;

    if (payload.title !== undefined) {
      sets.push(`title = $${idx++}`);
      params.push(payload.title);
    }
    if (payload.content !== undefined) {
      sets.push(`content = $${idx++}`);
      params.push(payload.content);
    }
    if (payload.isPinned !== undefined) {
      sets.push(`is_pinned = $${idx++}`);
      params.push(payload.isPinned);
    }

    const row = await this.row<PasteBins>(
      `UPDATE paste_bins SET ${sets.join(", ")} WHERE identifier = $1 AND workspace_identifier = $${idx}
       RETURNING *`,
      [...params, m.workspaceIdentifier],
    );
    if (!row) throw new Error("paste bin not found");
    return row;
  }

  async delete(identifier: string, meta?: RequestMeta): Promise<void> {
    const m = this.requireMeta(meta);
    const model = await this.find_by_id(identifier, m);
    if (!model) throw new Error("paste bin not found");

    await this.run(
      `INSERT INTO recycle_bin
         (identifier, item_id, item_type, payload, deleted_at, workspace_identifier)
       VALUES ($1, $2, 'paste_bin', $3, $4, $5)`,
      [
        this.newUuid(),
        identifier,
        JSON.stringify(model),
        this.now(),
        m.workspaceIdentifier,
      ],
    );

    await this.run(
      `DELETE FROM paste_bins WHERE identifier = $1 AND workspace_identifier = $2`,
      [identifier, m.workspaceIdentifier],
    );
  }

  async transfer_record(
    record_identifier: string,
    previous_workspace_identifier: string,
    target_workspace_identifier: string,
  ): Promise<void> {
    await this.transferWorkspaceRecord(
      "paste_bins",
      record_identifier,
      previous_workspace_identifier,
      target_workspace_identifier,
    );
  }

  async duplicate_record(
    record_identifier: string,
    previous_workspace_identifier: string,
    target_workspace_identifier: string,
  ): Promise<void> {
    await this.duplicateWorkspaceRecord(
      "paste_bins",
      record_identifier,
      previous_workspace_identifier,
      target_workspace_identifier,
      COLUMNS,
    );
  }

  async record_exists_in_workspace(
    record_identifier: string,
    workspace_identifier: string,
  ): Promise<boolean> {
    return this.recordExistsInWorkspace(
      "paste_bins",
      record_identifier,
      workspace_identifier,
    );
  }
}
