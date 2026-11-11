<script setup lang="ts">
import TaskForm from "@mobile/components/task/Form.vue";
import { useTaskStore } from "@shared/stores/tasks";

definePageMeta({ name: "Edit task" });

const route = useRoute();
const router = useRouter();
const taskStore = useTaskStore();
const { notify } = useAppNotification();

const id = computed(() =>
  typeof route.query.id === "string" ? route.query.id : undefined,
);

const task = computed(
  () => taskStore.tasks.find((entry) => entry.identifier === id.value) ?? null,
);

function handleSaved() {
  notify({ message: "Task updated", type: "success" });
  router.replace("/tasks");
}

onMounted(async () => {
  if (taskStore.tasks.length === 0) {
    await taskStore.fetchTasks();
  }
});
</script>

<template>
  <div>
    <div v-if="taskStore.loading && !task" class="flex flex-col gap-4">
      <USkeleton class="h-16 rounded-lg" />
      <USkeleton class="h-24 rounded-lg" />
      <USkeleton class="h-10 rounded-lg" />
    </div>

    <div
      v-else-if="!task"
      class="flex flex-col items-center justify-center py-20 text-center"
    >
      <div class="mb-4 p-3 rounded-full bg-gray-100 dark:bg-gray-800">
        <UIcon name="heroicons:calendar" class="size-7 text-gray-400" />
      </div>
      <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
        Task not found
      </h3>
      <button
        class="text-xs text-primary-500 hover:text-primary-600 font-medium mt-2"
        @click="router.push('/tasks')"
      >
        Back to tasks
      </button>
    </div>

    <TaskForm v-else :task="task" @saved="handleSaved" />
  </div>
</template>
