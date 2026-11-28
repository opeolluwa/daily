<script setup lang="ts">
import TaskDialog from "~/components/task/Dialog.vue";
import { useTaskStore, type Task } from "~/stores/tasks";

definePageMeta({ layout: false });

const taskStore = useTaskStore();

const { setSearch, clearSearch } = useAppSearch();
const dialogOpen = ref(false);
const editingTask = ref<Task | null>(null);
const initialDate = ref<string | null>(null);

const isMobile = useIsMobile();
const router = useRouter();

function openCreate(date?: string) {
  if (isMobile.value) {
    router.push({ path: "/tasks/create", query: date ? { date } : {} });
    return;
  }
  editingTask.value = null;
  initialDate.value = date ?? null;
  dialogOpen.value = true;
}

function handleEdit(identifier: string) {
  if (isMobile.value) {
    router.push({ path: "/tasks/edit", query: { id: identifier } });
    return;
  }
  const task = taskStore.tasks.find((t: Task) => t.identifier === identifier);
  if (task) {
    editingTask.value = task;
    dialogOpen.value = true;
  }
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
      <!-- Loading -->
      <div v-if="taskStore.loading" class="flex flex-col gap-2">
        <USkeleton v-for="i in 4" :key="i" class="h-16 rounded-lg" />
      </div>

      <TaskWeeklyView
        v-else
        @edit="handleEdit"
        @toggle="handleToggle"
        @delete="handleDelete"
        @add-task="openCreate"
      />
    </template>

    <template #modal_content>
      <TaskDialog
        v-model:open="dialogOpen"
        :task="editingTask"
        :initial-date="initialDate"
      />
    </template>
  </NuxtLayout>
</template>
