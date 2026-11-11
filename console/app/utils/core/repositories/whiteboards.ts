import type {
  CreateWhiteboard,
  UpdateWhiteboard,
  Whiteboards,
} from "@daily/core";
import { BaseRepository, type RequestMeta } from "../base";

export type { CreateWhiteboard, UpdateWhiteboard };

const COLUMNS = [
  "title",
  "payload",
  "workspace_identifier",
  "created_at",
  "updated_at",
];

export class WhiteboardRepository extends BaseRepository {
  async create(
    payload: CreateWhiteboard,
    meta?: RequestMeta,
  ): Promise<Whiteboards> {
    const m = this.requireMeta(meta);
    const now = this.now();
    return this.mustRow<Whiteboards>(
      `INSERT INTO whiteboards
         (identifier, title, payload, created_at, updated_at, workspace_identifier)
       VALUES ($1, $2, $3, $4, $5, $6)
       RETURNING *`,
      [
        this.newUuid(),
        payload.title ?? null,
        payload.payload != null ? JSON.stringify(payload.payload) : null,
        now,
        now,
        m.workspaceIdentifier,
      ],
    );
  }

  async find_by_id(
    identifier: string,
    meta?: RequestMeta,
  ): Promise<Whiteboards | null> {
    const m = this.requireMeta(meta);
    return this.row<Whiteboards>(
      `SELECT * FROM whiteboards WHERE identifier = $1 AND workspace_identifier = $2`,
      [identifier, m.workspaceIdentifier],
    );
  }

  async find_all(meta?: RequestMeta): Promise<Whiteboards[]> {
    const m = this.requireMeta(meta);
    return this.rows<Whiteboards>(
      `SELECT * FROM whiteboards WHERE workspace_identifier = $1 ORDER BY updated_at DESC`,
      [m.workspaceIdentifier],
    );
  }

  async recently_added(meta?: RequestMeta): Promise<Whiteboards[]> {
    const m = this.requireMeta(meta);
    return this.rows<Whiteboards>(
      `SELECT * FROM whiteboards WHERE workspace_identifier = $1
       ORDER BY created_at DESC LIMIT 10`,
      [m.workspaceIdentifier],
    );
  }

  async update(
    identifier: string,
    payload: UpdateWhiteboard,
    meta?: RequestMeta,
  ): Promise<Whiteboards> {
    const m = this.requireMeta(meta);
    const sets: string[] = ["updated_at = $2"];
    const params: unknown[] = [identifier, this.now()];
    let idx = 3;

    if (payload.title !== undefined) {
      sets.push(`title = $${idx++}`);
      params.push(payload.title);
    }
    if (payload.payload !== undefined) {
      sets.push(`payload = $${idx++}`);
      params.push(payload.payload != null ? JSON.stringify(payload.payload) : null);
    }

    const row = await this.row<Whiteboards>(
      `UPDATE whiteboards SET ${sets.join(", ")} WHERE identifier = $1 AND workspace_identifier = $${idx}
       RETURNING *`,
      [...params, m.workspaceIdentifier],
    );
    if (!row) throw new Error("whiteboard not found");
    return row;
  }

  async delete(identifier: string, meta?: RequestMeta): Promise<Whiteboards> {
    const m = this.requireMeta(meta);
    return this.mustRow<Whiteboards>(
      `DELETE FROM whiteboards WHERE identifier = $1 AND workspace_identifier = $2
       RETURNING *`,
      [identifier, m.workspaceIdentifier],
    );
  }

  async transfer_record(
    record_identifier: string,
    previous_workspace_identifier: string,
    target_workspace_identifier: string,
  ): Promise<void> {
    await this.transferWorkspaceRecord(
      "whiteboards",
      record_identifier,
      previous_workspace_identifier,
      target_workspace_identifier,
    );
  }

  async record_exists_in_workspace(
    record_identifier: string,
    workspace_identifier: string,
  ): Promise<boolean> {
    return this.recordExistsInWorkspace(
      "whiteboards",
      record_identifier,
      workspace_identifier,
    );
  }
}
