<script setup lang="ts">
import { DnDProvider } from "@vue-dnd-kit/core";
import TaskDialog from "~/components/task/Dialog.vue";
import { useTaskStore, type Task } from "~/stores/tasks";
import { ryderPageHeaderOverride } from "~/assets/ryder.css";
definePageMeta({ layout: false });

const taskStore = useTaskStore();

const { setSearch, clearSearch } = useAppSearch();
const dialogOpen = ref(false);
const editingTask = ref<Task | null>(null);
const isEmptyList = computed(() => !taskStore.tasks.length);

const isMobile = useIsMobile();
const router = useRouter();

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

async function handleToggle(identifier: string, done: boolean) {
  await taskStore.toggleDone(identifier, done);
}

async function handleDelete(identifier: string) {
  await taskStore.deleteTask(identifier);
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
      <RyderFab v-if="!isEmptyList" aria-label="New Task" @click="openCreate" />

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

      <DnDProvider v-if="taskStore.tasks.length">
        <TaskBoard
          @edit="handleEdit"
          @delete="handleDelete"
          @toggle="handleToggle"
        />
      </DnDProvider>
    </template>

    <template #modal_content>
      <TaskDialog v-model:open="dialogOpen" :task="editingTask" />
    </template>
  </NuxtLayout>
</template>
