<script setup lang="ts">
import { DnDProvider } from "@vue-dnd-kit/core";
import TaskListItem from "~/components/task/ListItem.vue";
import TaskDialog from "~/components/task/Dialog.vue";
import { useTaskStore, type Task } from "~/stores/tasks";
import { TASK_SORT_OPTIONS, sortTasks, type TaskSort } from "~/utils/sorting";
import type { TabsItem } from "@nuxt/ui";
import { ryderPageHeaderOverride } from "~/assets/ryder.css";
definePageMeta({ layout: false });

const taskStore = useTaskStore();

const { searchQuery, setSearch, clearSearch } = useAppSearch();
const filter = ref<"all" | "active" | "completed">("all");
const sortBy = ref<TaskSort>("priority-high");
const dialogOpen = ref(false);
const editingTask = ref<Task | null>(null);
const activeTab = ref("1");
const isListViewMode = computed(() => activeTab.value === "0");
const isEmptyList = computed(() => !taskStore.tasks.length);

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

const isMobile = useIsMobile();
const isPhone = useMediaQuery("(max-width: 767px)");
const router = useRouter();

watch(
  isPhone,
  (phone) => {
    if (phone) activeTab.value = "0";
  },
  { immediate: true },
);

const visibleTabs = computed(() =>
  isPhone.value ? items.filter((item) => item.slot === "list") : items,
);

function openCreate() {
  if (isMobile.value) {
    router.push("/tasks/create");
    return;
  }
  editingTask.value = null;
  dialogOpen.value = true;
}

function handleEdit(identifier: string) {
  if (isMobile.value) {
    router.push({ path: "/tasks/edit", query: { id: identifier } });
    return;
  }
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
  <NuxtLayout name="default">
    <template #main_content>
      <RyderPageHeader
        v-if="taskStore.tasks.length"
        :ui="ryderPageHeaderOverride"
        title="Task"
        subtitle="Plan your tasks"
        cta-text="New Task"
        cta-icon="heroicons:plus"
        @cta="openCreate"
      />
      <RyderFab
        v-if="!isEmptyList"
        aria-label="New Task"
        @click="openCreate"
      />
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
        @action="openCreate"
      />

      <!-- Task list -->
      <UTabs
        v-if="taskStore.tasks.length"
        v-model:model-value="activeTab"
        color="neutral"
        variant="link"
        :items="visibleTabs"
        :content="true"
        :ui="isPhone ? { list: 'hidden' } : undefined"
        class="w-full"
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


    <template #modal_content>
      <TaskDialog v-model:open="dialogOpen" :task="editingTask" />
    </template>
  </NuxtLayout>
</template>
