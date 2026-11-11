<script setup lang="ts">
import { computed } from "vue";
import type { TaskStatus } from "@daily/core";
import Column from "./Column.vue";
import { useTaskStore, type Task } from "~/stores/tasks";

const taskStore = useTaskStore();

const emit = defineEmits<{
  edit: [identifier: string];
  delete: [identifier: string];
  toggle: [identifier: string, done: boolean];
}>();

function forwardEdit(identifier: string) {
  emit("edit", identifier);
}

function forwardDelete(identifier: string) {
  emit("delete", identifier);
}

function forwardToggle(identifier: string, done: boolean) {
  emit("toggle", identifier, done);
}

const columns = computed<
  {
    status: TaskStatus;
    title: string;
    color: string;
    cards: Task[];
  }[]
>(() => [
  {
    status: "planned",
    title: "Planned",
    color: "#a1a1aa",
    cards: taskStore.plannedTasks,
  },
  {
    status: "in_progress",
    title: "In Progress",
    color: "#f59e0b",
    cards: taskStore.activeTasks,
  },
  {
    status: "done",
    title: "Done",
    color: "#42b883",
    cards: taskStore.completedTasks,
  },
]);

async function handleMove({
  items,
  status,
}: {
  items: Task[];
  status: TaskStatus;
}) {
  await Promise.all(
    items.map((task) => taskStore.updateTaskStatus(task.identifier, status)),
  );
}
</script>

<template>
  <div class="grid h-full min-h-0 grid-cols-3 gap-4">
    <Column
      v-for="column in columns"
      :key="column.status"
      :status="column.status"
      :title="column.title"
      :color="column.color"
      :cards="column.cards"
      class="min-w-0"
      @move="handleMove"
      @edit="forwardEdit"
      @delete="forwardDelete"
      @toggle="forwardToggle"
    />
  </div>
</template>
