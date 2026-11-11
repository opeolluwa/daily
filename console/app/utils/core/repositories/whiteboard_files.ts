import type { WhiteboardFiles } from "@daily/core";
import { BaseRepository, type RequestMeta } from "../base";

export type { WhiteboardFiles };

/**
 * Browser facade for the embedded image/audio assets of a whiteboard scene.
 * The rows live in the `whiteboard_files` table (created by the kernel
 * migration) and are keyed by the same file ids the scene payload references.
 * The native (Tauri) backend stores these as files on disk instead — both
 * sides exchange the same wire format: a data URL string.
 */
export class WhiteboardFileRepository extends BaseRepository {
  private async requireWhiteboardInWorkspace(
    whiteboardIdentifier: string,
    meta?: RequestMeta,
  ): Promise<void> {
    const m = this.requireMeta(meta);
    const ok = await this.bool(
      `SELECT EXISTS (
         SELECT 1 FROM whiteboards
         WHERE identifier = $1 AND workspace_identifier = $2
       ) AS ok`,
      [whiteboardIdentifier, m.workspaceIdentifier],
    );
    if (!ok) throw new Error("whiteboard not found");
  }

  async save(
    whiteboardIdentifier: string,
    fileId: string,
    mimeType: string,
    data: string,
    meta?: RequestMeta,
  ): Promise<void> {
    await this.requireWhiteboardInWorkspace(whiteboardIdentifier, meta);
    const now = this.now();
    await this.run(
      `INSERT INTO whiteboard_files
         (identifier, whiteboard_identifier, mime_type, data, created_at, updated_at)
       VALUES ($1, $2, $3, $4, $5, $6)
       ON CONFLICT (identifier) DO UPDATE
         SET mime_type = $3, data = $4, updated_at = $6`,
      [fileId, whiteboardIdentifier, mimeType, data, now, now],
    );
  }

  async find_all_for(
    whiteboardIdentifier: string,
    meta?: RequestMeta,
  ): Promise<WhiteboardFiles[]> {
    await this.requireWhiteboardInWorkspace(whiteboardIdentifier, meta);
    return this.rows<WhiteboardFiles>(
      `SELECT * FROM whiteboard_files WHERE whiteboard_identifier = $1
       ORDER BY created_at ASC`,
      [whiteboardIdentifier],
    );
  }

  async delete(fileId: string, meta?: RequestMeta): Promise<void> {
    await this.requireMeta(meta);
    await this.run(`DELETE FROM whiteboard_files WHERE identifier = $1`, [
      fileId,
    ]);
  }
}
