import type { CreateTask, Priority, Task, UpdateTask } from "@daily/core";
import { BaseRepository, type RequestMeta } from "../base";

export type { CreateTask, UpdateTask };

const COLUMNS = [
  "title",
  "description",
  "due_date",
  "priority",
  "done",
  "status",
  "workspace_identifier",
  "created_at",
  "updated_at",
];

export class TaskRepository extends BaseRepository {
  async create_task(payload: CreateTask, meta?: RequestMeta): Promise<Task> {
    const m = this.requireMeta(meta);
    const now = this.now();
    return this.mustRow<Task>(
      `INSERT INTO task
         (identifier, title, description, due_date, priority, done, created_at, updated_at, workspace_identifier)
       VALUES ($1, $2, $3, $4, $5, FALSE, $6, $7, $8)
       RETURNING *`,
      [
        this.newUuid(),
        payload.title,
        payload.description ?? null,
        payload.dueDate ?? null,
        payload.priority,
        now,
        now,
        m.workspaceIdentifier,
      ],
    );
  }

  async find_by_id(
    identifier: string,
    meta?: RequestMeta,
  ): Promise<Task | null> {
    const m = this.requireMeta(meta);
    return this.row<Task>(
      `SELECT * FROM task WHERE identifier = $1 AND workspace_identifier = $2`,
      [identifier, m.workspaceIdentifier],
    );
  }

  async find_all(meta?: RequestMeta): Promise<Task[]> {
    const m = this.requireMeta(meta);
    return this.rows<Task>(
      `SELECT * FROM task WHERE workspace_identifier = $1 ORDER BY created_at DESC`,
      [m.workspaceIdentifier],
    );
  }

  async change_priority(
    identifier: string,
    priority: Priority,
    meta?: RequestMeta,
  ): Promise<Task> {
    const m = this.requireMeta(meta);
    const row = await this.row<Task>(
      `UPDATE task SET priority = $2, updated_at = $3
       WHERE identifier = $1 AND workspace_identifier = $4
       RETURNING *`,
      [identifier, priority, this.now(), m.workspaceIdentifier],
    );
    if (!row) throw new Error("task not found");
    return row;
  }

  async mark_done(
    identifier: string,
    done: boolean,
    meta?: RequestMeta,
  ): Promise<Task> {
    const m = this.requireMeta(meta);
    const row = await this.row<Task>(
      `UPDATE task SET done = $2, updated_at = $3
       WHERE identifier = $1 AND workspace_identifier = $4
       RETURNING *`,
      [identifier, done, this.now(), m.workspaceIdentifier],
    );
    if (!row) throw new Error("task not found");
    return row;
  }

  async update_due_date(
    identifier: string,
    due_date: string | null,
    meta?: RequestMeta,
  ): Promise<Task> {
    const m = this.requireMeta(meta);
    const row = await this.row<Task>(
      `UPDATE task SET due_date = $2, updated_at = $3
       WHERE identifier = $1 AND workspace_identifier = $4
       RETURNING *`,
      [identifier, due_date, this.now(), m.workspaceIdentifier],
    );
    if (!row) throw new Error("task not found");
    return row;
  }

  async update(
    identifier: string,
    payload: UpdateTask,
    meta?: RequestMeta,
  ): Promise<Task> {
    const m = this.requireMeta(meta);
    const sets: string[] = ["updated_at = $2"];
    const params: unknown[] = [identifier, this.now()];
    let idx = 3;

    if (payload.title !== undefined) {
      sets.push(`title = $${idx++}`);
      params.push(payload.title);
    }
    if (payload.description !== undefined) {
      sets.push(`description = $${idx++}`);
      params.push(payload.description);
    }
    if (payload.status !== undefined && payload.status !== null) {
      sets.push(`status = $${idx++}::task_status`);
      params.push(payload.status);
    }

    const row = await this.row<Task>(
      `UPDATE task SET ${sets.join(", ")} WHERE identifier = $1 AND workspace_identifier = $${idx}
       RETURNING *`,
      [...params, m.workspaceIdentifier],
    );
    if (!row) throw new Error("task not found");
    return row;
  }

  async delete(identifier: string, meta?: RequestMeta): Promise<void> {
    const m = this.requireMeta(meta);
    const model = await this.find_by_id(identifier, m);
    if (!model) throw new Error("task not found");

    await this.run(
      `INSERT INTO recycle_bin
         (identifier, item_id, item_type, payload, deleted_at, workspace_identifier)
       VALUES ($1, $2, 'task', $3, $4, $5)`,
      [
        this.newUuid(),
        identifier,
        JSON.stringify(model),
        this.now(),
        m.workspaceIdentifier,
      ],
    );

    await this.run(
      `DELETE FROM task WHERE identifier = $1 AND workspace_identifier = $2`,
      [identifier, m.workspaceIdentifier],
    );
  }

  async transfer_record(
    record_identifier: string,
    previous_workspace_identifier: string,
    target_workspace_identifier: string,
  ): Promise<void> {
    await this.transferWorkspaceRecord(
      "task",
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
      "task",
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
      "task",
      record_identifier,
      workspace_identifier,
    );
  }
}
