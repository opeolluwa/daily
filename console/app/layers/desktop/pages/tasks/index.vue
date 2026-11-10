<script setup lang="ts">
definePageMeta({ layout: false });
import { DnDProvider } from "@vue-dnd-kit/core";
import TaskListItem from "@desktop/components/task/ListItem.vue";
import TaskDialog from "@desktop/components/task/Dialog.vue";
import { useTaskStore, type Task } from "@shared/stores/tasks";
import {
  TASK_SORT_OPTIONS,
  sortTasks,
  type TaskSort,
} from "@shared/utils/sorting";

const taskStore = useTaskStore();

const { searchQuery, setSearch, clearSearch } = useAppSearch();
const filter = ref<"all" | "active" | "completed">("all");
const sortBy = ref<TaskSort>("priority-high");
const dialogOpen = ref(false);
const editingTask = ref<any>(null);
const activeTab = ref("1");
const isListViewMode = computed(() => activeTab.value === "0");
const isEmptyList = computed(() => !taskStore.tasks.length);
import type { TabsItem } from "@nuxt/ui";

const items = [
  {
    label: "List",
    slot: "list" as const,
  },
  {
    label: "Board",
    slot: "board" as const,
  },
] satisfies TabsItem[];

const filteredTasks = computed(() => {
  let list = taskStore.tasks;

  if (filter.value === "active") list = list.filter((t: Task) => !t.done);
  if (filter.value === "completed") list = list.filter((t: Task) => t.done);

  const q = searchQuery.value.trim().toLowerCase();
  if (q) {
    list = list.filter(
      (t) =>
        t.title.toLowerCase().includes(q) ||
        t.description?.toLowerCase().includes(q),
    );
  }

  return sortTasks(list, sortBy.value);
});

async function handleToggle(identifier: string, done: boolean) {
  await taskStore.toggleDone(identifier, done);
}

async function handleDelete(identifier: string) {
  await taskStore.deleteTask(identifier);
}

async function deleteCompleted() {
  await Promise.all(
    taskStore.doneTasks.map((t) => taskStore.deleteTask(t.identifier)),
  );
}

function handleEdit(identifier: string) {
  editingTask.value =
    taskStore.tasks.find((t: Task) => t.identifier === identifier) ?? null;
  dialogOpen.value = true;
}

onMounted(async () => {
  setSearch({ placeholder: "Search tasks..." });
  await taskStore.fetchTasks();
});

onUnmounted(() => clearSearch());
</script>

<template>
  <NuxtLayout name="default" :show-side-content="isListViewMode ^isEmptyList">
    <template #page_title>
      <RyderPageHeader
        title="Task"
        v-if="taskStore.tasks.length"
        subtitle="Plan your tasks"
        cta-text="New Task"
        cta-icon="heroicons:plus"
        @cta="
          editingTask = null;
          dialogOpen = true;
        "
      />
    </template>

    <template #main_content>
      <div
        v-if="
          !taskStore.loading && taskStore.tasks.length > 0 && isListViewMode
        "
        class="flex items-center gap-2 mb-4"
      >
        <div class="flex gap-1 bg-gray-100 dark:bg-gray-800 rounded-lg p-0.5">
          <button
            v-for="f in ['all', 'active', 'completed'] as const"
            :key="f"
            class="px-3 py-1.5 rounded-md text-xs font-medium transition-colors capitalize"
            :class="
              filter === f
                ? 'bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-100 shadow-sm'
                : 'text-gray-500 dark:text-gray-400'
            "
            @click="filter = f"
          >
            {{ f }}
          </button>
        </div>
        <AppSortMenu v-model="sortBy" :options="TASK_SORT_OPTIONS" />
        <button
          v-if="taskStore.doneTasks.length > 0"
          class="ml-auto flex items-center gap-1.5 text-xs text-red-400 hover:text-red-500 transition-colors"
          @click="deleteCompleted"
        >
          <UIcon name="heroicons:trash" class="size-3.5" />
          Clear done ({{ taskStore.doneTasks.length }})
        </button>
      </div>

      <!-- Loading -->
      <div v-if="taskStore.loading" class="flex flex-col gap-2">
        <USkeleton v-for="i in 4" :key="i" class="h-16 rounded-lg" />
      </div>

      <RyderEmptyState
        v-else-if="taskStore.tasks.length === 0"
        title="No tasks yet"
        description="Create your first task to get started."
        icon="heroicons:check-circle"
        action-label="Create task"
        @action="
          editingTask = null;
          dialogOpen = true;
        "
      />

      <!-- Task list -->
      <UTabs
        v-if="taskStore.tasks.length"
        color="neutral"
        variant="link"
        :items="items"
        :content="true"
        class="w-full"
        v-model:model-value="activeTab"
      >
        <template #list>
          <div class="flex flex-col gap-2">
            <TaskListItem
              v-for="task in filteredTasks"
              :key="task.identifier"
              :task="task"
              @toggle="handleToggle"
              @edit="handleEdit"
              @delete="handleDelete"
            /></div
        ></template>
        <template #board>
          <DnDProvider>
            <TaskBoard
              @edit="handleEdit"
              @delete="handleDelete"
              @toggle="handleToggle"
            />
          </DnDProvider>
        </template>
      </UTabs>
    </template>

    <template #side_content>

        <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
          Summary
        </h2>
        <div class="flex flex-col gap-3 mb-6">
          <div class="bg-primary-50 dark:bg-primary-950 rounded-lg p-3">
            <p
              class="text-2xl font-semibold text-primary-700 dark:text-primary-300"
            >
              {{ taskStore.pendingTasks.length }}
            </p>
            <p class="text-xs text-primary-500 dark:text-primary-400">
              Active tasks
            </p>
          </div>
          <div class="bg-emerald-50 dark:bg-emerald-950 rounded-lg p-3">
            <p
              class="text-2xl font-semibold text-emerald-700 dark:text-emerald-300"
            >
              {{ taskStore.doneTasks.length }}
            </p>
            <p class="text-xs text-emerald-500 dark:text-emerald-400">
              Completed
            </p>
          </div>
        </div>

        <USeparator class="my-4" />

        <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
          Priority
        </h2>
        <div class="flex flex-col gap-2">
          <div class="flex items-center justify-between text-sm">
            <div class="flex items-center gap-2">
              <UIcon name="heroicons:flag" class="size-4 text-rose-500" />
              <span class="text-gray-600 dark:text-gray-400">High</span>
            </div>
            <span class="text-xs text-gray-400">{{
              taskStore.highPriorityCount
            }}</span>
          </div>
          <div class="flex items-center justify-between text-sm">
            <div class="flex items-center gap-2">
              <UIcon name="heroicons:flag" class="size-4 text-amber-500" />
              <span class="text-gray-600 dark:text-gray-400">Medium</span>
            </div>
            <span class="text-xs text-gray-400">{{
              taskStore.mediumPriorityCount
            }}</span>
          </div>
          <div class="flex items-center justify-between text-sm">
            <div class="flex items-center gap-2">
              <UIcon name="heroicons:flag" class="size-4 text-emerald-500" />
              <span class="text-gray-600 dark:text-gray-400">Low</span>
            </div>
            <span class="text-xs text-gray-400">{{
              taskStore.lowPriorityCount
            }}</span>
          </div>
        </div>
      </template>


    <template #modal_content>
      <TaskDialog v-model:open="dialogOpen" :task="editingTask" />
    </template>
  </NuxtLayout>
</template>
