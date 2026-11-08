import type { CreateTodo, Todo, TodoStatus, UpdateTodo } from "lunar";
import { defineStore } from "pinia";
import { invoke } from "../utils/invoke";

export type { Todo };

export type CreateTodoPayload = Partial<CreateTodo> &
  Pick<CreateTodo, "title" | "priority"> & { time?: string };

export type UpdateTodoPayload = Partial<UpdateTodo>;

interface State {
  todos: Todo[];
  loading: boolean;
}
export const useTodoStore = defineStore("todo_store", {
  state: (): State => ({
    todos: [] as Todo[],
    loading: false,
  }),

  getters: {
    activeTodos: (state: State) => state.todos.filter((t) => !t.done),
    completedTodos: (state: State) => state.todos.filter((t) => t.done),
    highPriorityCount: (state: State) =>
      state.todos.filter((t) => t.priority === "high").length,
    mediumPriorityCount: (state: State) =>
      state.todos.filter((t) => t.priority === "medium").length,
    lowPriorityCount: (state: State) =>
      state.todos.filter((t) => t.priority === "low").length,
    plannedTasks: (state: State) =>
      state.todos.filter((t) => t.status == "planned"),
    completedTasks: (state: State) =>
      state.todos.filter((t) => t.status == "done"),
    activeTasks: (state: State) =>
      state.todos.filter((t) => t.status == "in_progress"),
  },

  actions: {
    async fetchTodos() {
      this.loading = true;
      try {
        this.todos = await invoke<Todo[]>("get_all_todos", {
          meta: await getWorkspaceMeta(),
        });
      } catch (error) {
        console.error("[todos] failed to fetch", error);
      } finally {
        this.loading = false;
      }
    },

    async createTodo(payload: CreateTodoPayload): Promise<Todo> {
      const created = await invoke<Todo>("create_todo", {
        todo: payload,
        meta: await getWorkspaceMeta(),
      });

      this.todos.unshift(created);
      return created;
    },

    async updateTodo(
      identifier: string,
      payload: UpdateTodoPayload,
    ): Promise<Todo> {
      const updated = await invoke<Todo>("update_todo", {
        identifier,
        todo: payload,
        meta: await getWorkspaceMeta(),
      });

      const idx = this.todos.findIndex((t) => t.identifier === identifier);
      if (idx !== -1) this.todos[idx] = updated;

      return updated;
    },

    async toggleDone(identifier: string, done: boolean): Promise<Todo> {
      let updated = await invoke<Todo>("mark_todo_done", {
        identifier,
        done,
        meta: await getWorkspaceMeta(),
      });

      const nextStatus: TodoStatus = done
        ? "done"
        : updated.status === "done"
          ? "planned"
          : updated.status;

      if (nextStatus !== updated.status) {
        updated = await invoke<Todo>("update_todo", {
          identifier,
          todo: { status: nextStatus },
          meta: await getWorkspaceMeta(),
        });
      }

      const idx = this.todos.findIndex((t) => t.identifier === identifier);
      if (idx !== -1) this.todos[idx] = updated;

      return updated;
    },

    async updateTodoStatus(
      identifier: string,
      status: TodoStatus,
    ): Promise<Todo> {
      let updated = await invoke<Todo>("update_todo", {
        identifier,
        todo: { status },
        meta: await getWorkspaceMeta(),
      });

      const shouldBeDone = status === "done";

      if (updated.done !== shouldBeDone) {
        updated = await invoke<Todo>("mark_todo_done", {
          identifier,
          done: shouldBeDone,
          meta: await getWorkspaceMeta(),
        });
      }

      const idx = this.todos.findIndex((t) => t.identifier === identifier);
      if (idx !== -1) this.todos[idx] = updated;

      return updated;
    },

    async changePriority(
      identifier: string,
      priority: "high" | "medium" | "low",
    ): Promise<Todo> {
      const updated = await invoke<Todo>("change_todo_priority", {
        identifier,
        priority,
        meta: await getWorkspaceMeta(),
      });

      const idx = this.todos.findIndex((t) => t.identifier === identifier);
      if (idx !== -1) this.todos[idx] = updated;

      return updated;
    },

    async updateDueDate(
      identifier: string,
      dueDate: string | null,
    ): Promise<Todo> {
      const updated = await invoke<Todo>("update_todo_due_date", {
        identifier,
        dueDate,
        meta: await getWorkspaceMeta(),
      });

      const idx = this.todos.findIndex((t) => t.identifier === identifier);
      if (idx !== -1) this.todos[idx] = updated;

      return updated;
    },

    async deleteTodo(identifier: string) {
      await invoke("delete_todo", {
        identifier,
        meta: await getWorkspaceMeta(),
      });

      this.todos = this.todos.filter((t) => t.identifier !== identifier);
    },

    async duplicateTodo(
      identifier: string,
      sourceWorkspaceId: string,
      targetWorkspaceId: string,
    ): Promise<Todo> {
      const updated = await invoke<Todo>("duplicate_todo", {
        identifier,
        sourceWorkspaceId,
        targetWorkspaceId,
        meta: await getWorkspaceMeta(),
      });

      this.todos.push(updated);
      return updated;
    },

    async transferTodo(
      identifier: string,
      sourceWorkspaceId: string,
      targetWorkspaceId: string,
    ): Promise<Todo> {
      const updated = await invoke<Todo>("transfer_todo", {
        identifier,
        sourceWorkspaceId,
        targetWorkspaceId,
        meta: await getWorkspaceMeta(),
      });

      const idx = this.todos.findIndex((t) => t.identifier === identifier);
      if (idx !== -1) this.todos[idx] = updated;
      return updated;
    },

    async fetchUnsynced() {
      try {
        const todos = await invoke<Todo[]>("get_unsynced_todos");
        return todos;
      } catch (error) {
        console.error("Error fetching unsynced todos:", error);
        return [];
      }
    },
  },
});
