import { BaseRepository, type RequestMeta } from "../base";

export interface TaskCategory {
  identifier: string;
  name: string;
  color: string;
  workspaceIdentifier: string | null;
  createdAt: string;
  updatedAt: string;
}

export interface CreateTaskCategoryPayload {
  name: string;
  color: string;
}

export interface UpdateTaskCategoryPayload {
  name?: string;
  color?: string;
}

export class TaskCategoryRepository extends BaseRepository {
  async create(
    payload: CreateTaskCategoryPayload,
    meta?: RequestMeta,
  ): Promise<TaskCategory> {
    const m = this.requireMeta(meta);
    const now = this.now();
    return this.mustRow<TaskCategory>(
      `INSERT INTO task_category
         (identifier, name, color, workspace_identifier, created_at, updated_at)
       VALUES ($1, $2, $3, $4, $5, $6)
       RETURNING *`,
      [this.newId(), payload.name, payload.color, m.workspaceIdentifier, now, now],
    );
  }

  async find_all(meta?: RequestMeta): Promise<TaskCategory[]> {
    const m = this.requireMeta(meta);
    return this.rows<TaskCategory>(
      `SELECT * FROM task_category WHERE workspace_identifier = $1 ORDER BY created_at ASC`,
      [m.workspaceIdentifier],
    );
  }

  async update(
    identifier: string,
    payload: UpdateTaskCategoryPayload,
    meta?: RequestMeta,
  ): Promise<TaskCategory> {
    const m = this.requireMeta(meta);
    const sets: string[] = ["updated_at = $2"];
    const params: unknown[] = [identifier, this.now()];
    let idx = 3;

    if (payload.name !== undefined) {
      sets.push(`name = $${idx++}`);
      params.push(payload.name);
    }
    if (payload.color !== undefined) {
      sets.push(`color = $${idx++}`);
      params.push(payload.color);
    }

    const row = await this.row<TaskCategory>(
      `UPDATE task_category SET ${sets.join(", ")} WHERE identifier = $1 AND workspace_identifier = $${idx}
       RETURNING *`,
      [...params, m.workspaceIdentifier],
    );
    if (!row) throw new Error("task category not found");
    return row;
  }

  async delete(identifier: string, meta?: RequestMeta): Promise<void> {
    const m = this.requireMeta(meta);
    await this.run(
      `DELETE FROM task_category WHERE identifier = $1 AND workspace_identifier = $2`,
      [identifier, m.workspaceIdentifier],
    );
  }
}
