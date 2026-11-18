import { BaseRepository } from "../base";

export interface NotificationSettings {
  identifier: string;
  tasksDue: boolean;
  reminders: boolean;
  syncComplete: boolean;
  appUpdates: boolean;
  userIdentifier: string | null;
  createdAt: string;
  updatedAt: string;
}

export interface UpdateNotificationSettings {
  tasksDue?: boolean;
  reminders?: boolean;
  syncComplete?: boolean;
  appUpdates?: boolean;
}

export class NotificationSettingsRepository extends BaseRepository {
  async get_or_create(): Promise<NotificationSettings> {
    const existing = await this.row<NotificationSettings>(
      `SELECT * FROM notification_settings LIMIT 1`,
    );
    if (existing) return existing;

    const now = this.now();
    return this.mustRow<NotificationSettings>(
      `INSERT INTO notification_settings
         (identifier, tasks_due, reminders, sync_complete, app_updates,
          user_identifier, created_at, updated_at)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
       RETURNING *`,
      [this.newId(), true, true, false, true, null, now, now],
    );
  }

  async update(
    identifier: string,
    payload: UpdateNotificationSettings,
  ): Promise<NotificationSettings> {
    const sets: string[] = ["updated_at = $2"];
    const params: unknown[] = [identifier, this.now()];
    let idx = 3;

    if (payload.tasksDue !== undefined) {
      sets.push(`tasks_due = $${idx++}`);
      params.push(payload.tasksDue);
    }
    if (payload.reminders !== undefined) {
      sets.push(`reminders = $${idx++}`);
      params.push(payload.reminders);
    }
    if (payload.syncComplete !== undefined) {
      sets.push(`sync_complete = $${idx++}`);
      params.push(payload.syncComplete);
    }
    if (payload.appUpdates !== undefined) {
      sets.push(`app_updates = $${idx++}`);
      params.push(payload.appUpdates);
    }

    const row = await this.row<NotificationSettings>(
      `UPDATE notification_settings SET ${sets.join(", ")}
       WHERE identifier = $1
       RETURNING *`,
      params,
    );
    if (!row) throw new Error("notification settings not found");
    return row;
  }
}
