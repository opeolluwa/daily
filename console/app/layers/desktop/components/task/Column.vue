<script setup lang="ts">
import { useTemplateRef } from "vue";
import { makeSortableList } from "@vue-dnd-kit/core";
import type { TodoStatus } from "lunar";
import Card from "./Card.vue";
import type { Todo } from "@shared/stores/todo";

const props = defineProps<{
  status: TodoStatus;
  title: string;
  color: string;
  cards: Todo[];
}>();

const emit = defineEmits<{
  move: [payload: { items: Todo[]; status: TodoStatus }];
}>();

const zone = useTemplateRef<HTMLElement>("zone");
const list = useTemplateRef<HTMLElement>("list");

const { items, isOver } = makeSortableList<Todo>(list, {
  id: props.status,
  items: () => props.cards,
  getKey: (todo) => todo.identifier,
  group: "board",
  dropzone: zone,
  onDrop(event) {
    event.preventDefault();

    if (event.to.list !== props.status || event.to.index === -1) return;
    if (event.sameList) return;

    emit("move", { items: event.items, status: props.status });
  },
});
</script>

<template>
  <section
    ref="zone"
    class="flex min-h-0 flex-col rounded-xl border border-gray-200 bg-gray-50/70 p-3 transition-colors dark:border-gray-800 dark:bg-gray-900/40"
    :class="{
      'border-primary-500 bg-primary-500/5': isOver,
    }"
  >
    <!-- Column header -->
    <header class="mb-3 flex items-center gap-2 px-1">
      <span
        class="size-2 shrink-0 rounded-full"
        :style="{ backgroundColor: color }"
      />

      <h3 class="text-sm font-semibold text-gray-700 dark:text-gray-200">
        {{ title }}
      </h3>

      <span
        class="ml-auto rounded-full bg-gray-200/70 px-2 py-1 font-mono text-[11px] leading-none text-gray-500 dark:bg-gray-800 dark:text-gray-400"
      >
        {{ items.length }}
      </span>
    </header>

    <!-- Sortable cards -->
    <ul
      ref="list"
      class="flex min-h-40 flex-1 flex-col gap-2 overflow-y-auto p-0"
    >
      <Card v-for="card in items" :key="card.identifier" :todo="card" />
    </ul>

    <!-- Empty state -->
    <div
      v-if="items.length === 0"
      class="pointer-events-none flex min-h-24 flex-1 items-center justify-center rounded-lg border border-dashed border-gray-200 text-xs text-gray-400 dark:border-gray-800 dark:text-gray-500"
    />
  </section>
</template>
