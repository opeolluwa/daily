import { defineStore } from "pinia";
import { invoke } from "../utils/invoke";
import type { TaskCategory } from "../utils/core/repositories/task-categories";
import { DEFAULT_TASK_CATEGORIES } from "../data/default-task-categories";

export type { TaskCategory };

interface State {
  categories: TaskCategory[];
  loading: boolean;
}

export const useTaskCategoryStore = defineStore("task_category_store", {
  state: (): State => ({
    categories: [],
    loading: false,
  }),

  actions: {
    async fetchCategories() {
      this.loading = true;
      try {
        this.categories = await invoke<TaskCategory[]>(
          "get_all_task_categories",
          { meta: await getWorkspaceMeta() },
        );
      } catch (error) {
        console.error("[task-categories] failed to fetch", error);
      } finally {
        this.loading = false;
      }
    },

    async seedDefaults() {
      if (this.categories.length > 0) return;
      for (const cat of DEFAULT_TASK_CATEGORIES) {
        await this.createCategory({ name: cat.name, color: cat.color });
      }
    },

    async createCategory(payload: {
      name: string;
      color: string;
    }): Promise<TaskCategory> {
      const created = await invoke<TaskCategory>("create_task_category", {
        category: payload,
        meta: await getWorkspaceMeta(),
      });
      this.categories.push(created);
      return created;
    },

    async updateCategory(
      identifier: string,
      payload: { name?: string; color?: string },
    ): Promise<TaskCategory> {
      const updated = await invoke<TaskCategory>("update_task_category", {
        identifier,
        category: payload,
        meta: await getWorkspaceMeta(),
      });
      const idx = this.categories.findIndex(
        (c) => c.identifier === identifier,
      );
      if (idx !== -1) this.categories[idx] = updated;
      return updated;
    },

    async deleteCategory(identifier: string) {
      await invoke("delete_task_category", {
        identifier,
        meta: await getWorkspaceMeta(),
      });
      this.categories = this.categories.filter(
        (c) => c.identifier !== identifier,
      );
    },
  },
});
