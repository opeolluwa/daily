<script setup lang="ts">
import { useTaskCategoryStore, type TaskCategory } from "~/stores/task-categories";

const open = defineModel<boolean>("open", { required: true });
const categoryStore = useTaskCategoryStore();
const toast = useToast();

const newName = ref("");
const newColor = ref("#3b82f6");
const creating = ref(false);

const editingId = ref<string | null>(null);
const editName = ref("");
const editColor = ref("");

const PRESET_COLORS = [
  "#3b82f6",
  "#f43f5e",
  "#f97316",
  "#ec4899",
  "#a855f7",
  "#10b981",
  "#06b6d4",
  "#eab308",
  "#6366f1",
  "#64748b",
];

async function handleCreate() {
  if (!newName.value.trim()) return;
  creating.value = true;
  try {
    await categoryStore.createCategory({
      name: newName.value.trim(),
      color: newColor.value,
    });
    newName.value = "";
    newColor.value = "#3b82f6";
  } catch {
    toast.add({
      color: "error",
      icon: "heroicons:x-circle",
      title: "Failed to create category",
    });
  } finally {
    creating.value = false;
  }
}

function startEdit(cat: TaskCategory) {
  editingId.value = cat.identifier;
  editName.value = cat.name;
  editColor.value = cat.color;
}

async function saveEdit() {
  if (!editingId.value || !editName.value.trim()) return;
  try {
    await categoryStore.updateCategory(editingId.value, {
      name: editName.value.trim(),
      color: editColor.value,
    });
    editingId.value = null;
  } catch {
    toast.add({
      color: "error",
      icon: "heroicons:x-circle",
      title: "Failed to update category",
    });
  }
}

async function handleDelete(id: string) {
  try {
    await categoryStore.deleteCategory(id);
  } catch {
    toast.add({
      color: "error",
      icon: "heroicons:x-circle",
      title: "Failed to delete category",
    });
  }
}
</script>

<template>
  <USlideover v-model:open="open">
    <template #header>
      <h3 class="text-base font-semibold text-gray-800 dark:text-gray-100">
        Manage Categories
      </h3>
    </template>
    <template #body>
      <div class="flex flex-col gap-4 p-4">
        <!-- Add new -->
        <div class="flex items-end gap-2">
          <div class="flex-1">
            <label class="text-xs font-medium text-gray-500 dark:text-gray-400 mb-1 block">
              New category
            </label>
            <input
              v-model="newName"
              type="text"
              placeholder="Category name"
              class="w-full rounded-lg border border-gray-200 bg-white px-3 py-2 text-sm text-gray-800 placeholder-gray-400 focus:border-primary-500 focus:outline-none dark:border-gray-700 dark:bg-gray-800 dark:text-gray-100 dark:placeholder-gray-500"
              @keydown.enter="handleCreate"
            />
          </div>
          <div class="flex items-center gap-1">
            <button
              v-for="color in PRESET_COLORS"
              :key="color"
              type="button"
              class="size-5 rounded-full border-2 transition-transform hover:scale-110"
              :class="
                newColor === color
                  ? 'border-gray-800 dark:border-white scale-110'
                  : 'border-transparent'
              "
              :style="{ backgroundColor: color }"
              @click="newColor = color"
            />
          </div>
          <UButton
            size="sm"
            :loading="creating"
            :disabled="!newName.trim()"
            @click="handleCreate"
          >
            Add
          </UButton>
        </div>

        <hr class="border-gray-200 dark:border-gray-700" />

        <!-- Existing categories -->
        <div class="flex flex-col gap-2">
          <div
            v-for="cat in categoryStore.categories"
            :key="cat.identifier"
            class="flex items-center gap-2 rounded-lg border border-gray-200 px-3 py-2 dark:border-gray-700"
          >
            <template v-if="editingId === cat.identifier">
              <div class="flex items-center gap-1 shrink-0">
                <button
                  v-for="color in PRESET_COLORS"
                  :key="color"
                  type="button"
                  class="size-4 rounded-full border-2 transition-transform hover:scale-110"
                  :class="
                    editColor === color
                      ? 'border-gray-800 dark:border-white scale-110'
                      : 'border-transparent'
                  "
                  :style="{ backgroundColor: color }"
                  @click="editColor = color"
                />
              </div>
              <input
                v-model="editName"
                type="text"
                class="flex-1 rounded border-0 bg-transparent px-1 py-0.5 text-sm text-gray-800 focus:outline-none dark:text-gray-100"
                @keydown.enter="saveEdit"
                @keydown.escape="editingId = null"
              />
              <UButton size="xs" variant="soft" @click="saveEdit">Save</UButton>
              <UButton
                size="xs"
                color="neutral"
                variant="ghost"
                @click="editingId = null"
              >
                Cancel
              </UButton>
            </template>
            <template v-else>
              <span
                class="size-3 shrink-0 rounded-full"
                :style="{ backgroundColor: cat.color }"
              />
              <span class="flex-1 text-sm text-gray-800 dark:text-gray-100">
                {{ cat.name }}
              </span>
              <button
                type="button"
                class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
                @click="startEdit(cat)"
              >
                <UIcon name="heroicons:pencil-square" class="size-4" />
              </button>
              <button
                type="button"
                class="text-gray-400 hover:text-rose-500"
                @click="handleDelete(cat.identifier)"
              >
                <UIcon name="heroicons:trash" class="size-4" />
              </button>
            </template>
          </div>

          <p
            v-if="categoryStore.categories.length === 0"
            class="text-center text-sm text-gray-400 dark:text-gray-500 py-4"
          >
            No categories yet
          </p>
        </div>
      </div>
    </template>
  </USlideover>
</template>
