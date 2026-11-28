import type {
  CreateTask,
  Task,
  TaskStatus,
  TaskSubtaskInterface,
  TaskWithSubtasks,
  UpdateTask,
} from "@daily/core";
import { defineStore } from "pinia";
import { invoke } from "../utils/invoke";

export type { Task };

export type CreateTaskPayload = Partial<CreateTask> &
  Pick<CreateTask, "title" | "priority"> & {
    time?: string;
    categoryIdentifier?: string;
  };

export type UpdateTaskPayload = Partial<UpdateTask>;

export interface TaskSubtaskDraft {
  identifier: string | null;
  title: string;
  done: boolean;
  sortOrder: number;
}

interface State {
  tasks: Task[];
  subtasksByTask: Record<string, TaskSubtaskDraft[]>;
  progressByTask: Record<string, number>;
  loading: boolean;
}
export const useTaskStore = defineStore("task_store", {
  state: (): State => ({
    tasks: [] as Task[],
    subtasksByTask: {},
    progressByTask: {},
    loading: false,
  }),

  getters: {
    pendingTasks: (state: State) => state.tasks.filter((t) => !t.done),
    doneTasks: (state: State) => state.tasks.filter((t) => t.done),
    highPriorityCount: (state: State) =>
      state.tasks.filter((t) => t.priority === "high").length,
    mediumPriorityCount: (state: State) =>
      state.tasks.filter((t) => t.priority === "medium").length,
    lowPriorityCount: (state: State) =>
      state.tasks.filter((t) => t.priority === "low").length,
    plannedTasks: (state: State) =>
      state.tasks.filter((t) => t.status == "planned"),
    completedTasks: (state: State) =>
      state.tasks.filter((t) => t.status == "done"),
    activeTasks: (state: State) =>
      state.tasks.filter((t) => t.status == "in_progress"),

    subtaskCountByTask: (state: State) =>
      Object.fromEntries(
        Object.entries(state.subtasksByTask).map(([id, subtasks]) => [
          id,
          subtasks.length,
        ]),
      ),

    tasksByDate: (state: State) => {
      const map = new Map<string, Task[]>();
      for (const task of state.tasks) {
        const key = task.dueDate ?? "__unscheduled__";
        const list = map.get(key);
        if (list) list.push(task);
        else map.set(key, [task]);
      }
      return map;
    },
  },

  actions: {
    async fetchTasks() {
      this.loading = true;
      try {
        const details = await invoke<TaskWithSubtasks[]>(
          "get_tasks_with_subtasks",
          { meta: await getWorkspaceMeta() },
        );
        this.tasks = details.map((detail) => detail.task);
        this.subtasksByTask = Object.fromEntries(
          details.map((detail) => [
            detail.task.identifier,
            detail.subtasks.map((subtask) => ({
              identifier: subtask.identifier,
              title: subtask.title,
              done: subtask.done,
              sortOrder: subtask.sortOrder,
            })),
          ]),
        );
        this.progressByTask = Object.fromEntries(
          details.map((detail) => [detail.task.identifier, detail.progress]),
        );
      } catch (error) {
        console.error("[tasks] failed to fetch", error);
      } finally {
        this.loading = false;
      }
    },

    async createTask(payload: CreateTaskPayload): Promise<Task> {
      const created = await invoke<Task>("create_task", {
        task: payload,
        meta: await getWorkspaceMeta(),
      });

      this.tasks.unshift(created);
      this.subtasksByTask[created.identifier] = [];
      this.progressByTask[created.identifier] = 0;
      return created;
    },

    async updateTask(
      identifier: string,
      payload: UpdateTaskPayload,
    ): Promise<Task> {
      const updated = await invoke<Task>("update_task", {
        identifier,
        task: payload,
        meta: await getWorkspaceMeta(),
      });

      const idx = this.tasks.findIndex((t) => t.identifier === identifier);
      if (idx !== -1) this.tasks[idx] = updated;

      return updated;
    },

    async toggleDone(identifier: string, done: boolean): Promise<Task> {
      let updated = await invoke<Task>("mark_task_done", {
        identifier,
        done,
        meta: await getWorkspaceMeta(),
      });

      const nextStatus: TaskStatus = done
        ? "done"
        : updated.status === "done"
          ? "planned"
          : updated.status;

      if (nextStatus !== updated.status) {
        updated = await invoke<Task>("update_task", {
          identifier,
          task: { status: nextStatus },
          meta: await getWorkspaceMeta(),
        });
      }

      const idx = this.tasks.findIndex((t) => t.identifier === identifier);
      if (idx !== -1) this.tasks[idx] = updated;

      return updated;
    },

    async updateTaskStatus(
      identifier: string,
      status: TaskStatus,
    ): Promise<Task> {
      let updated = await invoke<Task>("update_task", {
        identifier,
        task: { status },
        meta: await getWorkspaceMeta(),
      });

      const shouldBeDone = status === "done";

      if (updated.done !== shouldBeDone) {
        updated = await invoke<Task>("mark_task_done", {
          identifier,
          done: shouldBeDone,
          meta: await getWorkspaceMeta(),
        });
      }

      const idx = this.tasks.findIndex((t) => t.identifier === identifier);
      if (idx !== -1) this.tasks[idx] = updated;

      return updated;
    },

    async changePriority(
      identifier: string,
      priority: "high" | "medium" | "low",
    ): Promise<Task> {
      const updated = await invoke<Task>("change_task_priority", {
        identifier,
        priority,
        meta: await getWorkspaceMeta(),
      });

      const idx = this.tasks.findIndex((t) => t.identifier === identifier);
      if (idx !== -1) this.tasks[idx] = updated;

      return updated;
    },

    async updateDueDate(
      identifier: string,
      dueDate: string | null,
    ): Promise<Task> {
      const updated = await invoke<Task>("update_task_due_date", {
        identifier,
        dueDate,
        meta: await getWorkspaceMeta(),
      });

      const idx = this.tasks.findIndex((t) => t.identifier === identifier);
      if (idx !== -1) this.tasks[idx] = updated;

      return updated;
    },

    async deleteTask(identifier: string) {
      await invoke("delete_task", {
        identifier,
        meta: await getWorkspaceMeta(),
      });

      this.tasks = this.tasks.filter((t) => t.identifier !== identifier);
      this.subtasksByTask = Object.fromEntries(
        Object.entries(this.subtasksByTask).filter(([id]) => id !== identifier),
      );
      this.progressByTask = Object.fromEntries(
        Object.entries(this.progressByTask).filter(([id]) => id !== identifier),
      );
    },

    async duplicateTask(
      identifier: string,
      sourceWorkspaceId: string,
      targetWorkspaceId: string,
    ): Promise<Task> {
      const updated = await invoke<Task>("duplicate_task", {
        identifier,
        sourceWorkspaceId,
        targetWorkspaceId,
        meta: await getWorkspaceMeta(),
      });

      this.tasks.push(updated);
      this.subtasksByTask[updated.identifier] = [];
      this.progressByTask[updated.identifier] = 0;
      return updated;
    },

    async transferTask(
      identifier: string,
      sourceWorkspaceId: string,
      targetWorkspaceId: string,
    ): Promise<Task> {
      const updated = await invoke<Task>("transfer_task", {
        identifier,
        sourceWorkspaceId,
        targetWorkspaceId,
        meta: await getWorkspaceMeta(),
      });

      const idx = this.tasks.findIndex((t) => t.identifier === identifier);
      if (idx !== -1) this.tasks[idx] = updated;
      return updated;
    },

    async loadSubtasks(taskIdentifier: string): Promise<TaskSubtaskDraft[]> {
      const subtasks = await invoke<TaskSubtaskInterface[]>(
        "get_task_subtasks",
        {
          taskIdentifier,
          meta: await getWorkspaceMeta(),
        },
      );
      const drafts = subtasks.map((subtask) => ({
        identifier: subtask.identifier,
        title: subtask.title,
        done: subtask.done,
        sortOrder: subtask.sortOrder,
      }));
      this.subtasksByTask[taskIdentifier] = drafts;
      return drafts;
    },

    async saveSubtasks(
      taskIdentifier: string,
      subtasks: TaskSubtaskDraft[],
    ): Promise<TaskSubtaskDraft[]> {
      const saved = await invoke<TaskSubtaskInterface[]>("save_task_subtasks", {
        taskIdentifier,
        subtasks: subtasks.map((subtask) => ({
          identifier: subtask.identifier,
          title: subtask.title,
          done: subtask.done,
          sortOrder: subtask.sortOrder,
        })),
        meta: await getWorkspaceMeta(),
      });
      const drafts = saved.map((subtask) => ({
        identifier: subtask.identifier,
        title: subtask.title,
        done: subtask.done,
        sortOrder: subtask.sortOrder,
      }));
      this.subtasksByTask[taskIdentifier] = drafts;
      await this.fetchTasks();
      return drafts;
    },

    async fetchUnsynced() {
      try {
        const tasks = await invoke<Task[]>("get_unsynced_tasks");
        return tasks;
      } catch (error) {
        console.error("Error fetching unsynced tasks:", error);
        return [];
      }
    },
  },
});
