<script setup lang="ts">
import { computed } from "vue";
import Column from "./Column.vue";
import { useTodoStore } from "@shared/stores/todo";

const todoStore = useTodoStore();

const columns = computed(() => [
  {
    id: "planned",
    title: "Planned",
    color: "#a1a1aa",
    cards: todoStore.plannedTasks,
  },
  {
    id: "active",
    title: "In Progress",
    color: "#f59e0b",
    cards: todoStore.activeTasks,
  },
  {
    id: "completed",
    title: "Completed",
    color: "#42b883",
    cards: todoStore.completedTasks,
  },
]);
</script>

<template>
  <div class="grid h-full min-h-0 grid-cols-3 gap-4">
    <Column
      v-for="column in columns"
      :key="column.id"
      :column="column"
      class="min-w-0"
    />
  </div>
</template>
