<script setup lang="ts">
import { useTaskStore } from "@shared/stores/tasks";

definePageMeta({ name: "Tasks" });

const taskStore = useTaskStore();
const router = useRouter();

function openCreate() {
  router.push("/tasks/create");
}

function openEdit(identifier: string) {
  router.push({ path: "/tasks/edit", query: { id: identifier } });
}

onMounted(() => {
  taskStore.fetchTasks();
});
</script>

<template>
  <RyderPullToRefresh @refresh="() => taskStore.fetchTasks()">
    <div>
      <!-- Create task FAB -->
      <RyderFab
        v-if="taskStore.tasks.length !== 0"
        class="!bottom-24"
        @click="openCreate"
      />
      <!-- Loading -->
      <div v-if="taskStore.loading" class="flex flex-col gap-2">
        <USkeleton v-for="i in 4" :key="i" class="h-16 rounded-lg" />
      </div>

      <!-- Empty state: no tasks at all -->
      <div v-else-if="taskStore.tasks.length === 0">
        <RyderEmptyState
          title="No task yet"
          description="Create your first task to get started."
          icon="ri:calendar-todo-line"
          action-label="create task"
          @action="openCreate"
        />
      </div>

      <!-- Task list -->
      <div v-else class="flex flex-col gap-2">
        <TaskCard
          v-for="task in taskStore.tasks"
          :key="task.identifier"
          :task="task"
          @toggle="(id, done) => taskStore.toggleDone(id, done)"
          @edit="openEdit"
          @delete="(id) => taskStore.deleteTask(id)"
        />
      </div>
    </div>
  </RyderPullToRefresh>
</template>
