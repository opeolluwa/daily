<script setup lang="ts">
import { computed } from "vue";
import type { TodoStatus } from "lunar";
import Column from "./Column.vue";
import { useTodoStore, type Todo } from "@shared/stores/todo";

const todoStore = useTodoStore();

const columns = computed<
  {
    status: TodoStatus;
    title: string;
    color: string;
    cards: Todo[];
  }[]
>(() => [
  {
    status: "planned",
    title: "Planned",
    color: "#a1a1aa",
    cards: todoStore.plannedTasks,
  },
  {
    status: "in_progress",
    title: "In Progress",
    color: "#f59e0b",
    cards: todoStore.activeTasks,
  },
  {
    status: "done",
    title: "Done",
    color: "#42b883",
    cards: todoStore.completedTasks,
  },
]);

async function handleMove({
  items,
  status,
}: {
  items: Todo[];
  status: TodoStatus;
}) {
  await Promise.all(
    items.map((todo) => todoStore.updateTodoStatus(todo.identifier, status)),
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
    />
  </div>
</template>
