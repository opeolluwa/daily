import { BaseRepository } from "../base";

export type BackupMode = "local_only" | "cloud_only" | "local_first" | "cloud_first";

export interface BackupSettings {
  identifier: string;
  userIdentifier: string | null;
  serverUrl: string;
  lastSync: string | null;
  backupMode: BackupMode;
  createdAt: string;
  updatedAt: string;
}

export interface UpdateBackupSettings {
  serverUrl?: string;
  backupMode?: BackupMode;
}

export class BackupSettingsRepository extends BaseRepository {
  async get_or_create(): Promise<BackupSettings> {
    const existing = await this.row<BackupSettings>(
      `SELECT * FROM backup_settings LIMIT 1`,
    );
    if (existing) return existing;

    const now = this.now();
    return this.mustRow<BackupSettings>(
      `INSERT INTO backup_settings
         (identifier, user_identifier, server_url, last_sync, backup_mode,
          created_at, updated_at)
       VALUES ($1, $2, $3, $4, $5, $6, $7)
       RETURNING *`,
      [this.newId(), null, "", null, "local_only", now, now],
    );
  }

  async update(
    identifier: string,
    payload: UpdateBackupSettings,
  ): Promise<BackupSettings> {
    const sets: string[] = ["updated_at = $2"];
    const params: unknown[] = [identifier, this.now()];
    let idx = 3;

    if (payload.serverUrl !== undefined) {
      sets.push(`server_url = $${idx++}`);
      params.push(payload.serverUrl);
    }
    if (payload.backupMode !== undefined) {
      sets.push(`backup_mode = $${idx++}`);
      params.push(payload.backupMode);
    }

    const row = await this.row<BackupSettings>(
      `UPDATE backup_settings SET ${sets.join(", ")}
       WHERE identifier = $1
       RETURNING *`,
      params,
    );
    if (!row) throw new Error("backup settings not found");
    return row;
  }
}
