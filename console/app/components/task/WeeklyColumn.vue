<script setup lang="ts">
import type { Task } from "~/stores/tasks";
import type { TaskCategory } from "~/stores/task-categories";

const props = defineProps<{
  date: string;
  tasks: Task[];
  categories: TaskCategory[];
  isToday: boolean;
}>();

const emit = defineEmits<{
  edit: [identifier: string];
  toggle: [identifier: string, done: boolean];
  addTask: [date: string];
}>();

const dayLabel = computed(() => {
  const d = new Date(props.date + "T00:00:00");
  const day = d.getDate();
  const weekday = d.toLocaleDateString("en-US", { weekday: "short" });
  const suffix = props.isToday ? " · Today" : "";
  return `${day} ${weekday}${suffix}`;
});

const pendingCount = computed(
  () => props.tasks.filter((t) => !t.done).length,
);
</script>

<template>
  <section
    class="flex min-h-0 flex-col rounded-xl border p-3 transition-colors"
    :class="
      'border-gray-200 bg-gray-50/70 dark:border-gray-800 dark:bg-gray-900/40'
    "
  >
    <header class="mb-3 flex items-center gap-2 px-1">
      <h3
        class="text-sm font-semibold"
        :class="
          'text-gray-700 dark:text-gray-200'
        "
      >
        {{ dayLabel }}
      </h3>
      <span
        v-if="pendingCount > 0"
        class="ml-auto rounded-full bg-emerald-100 px-2 py-0.5 font-mono text-[11px] leading-none font-medium text-emerald-700 dark:bg-emerald-900/60 dark:text-emerald-300"
      >
        {{ pendingCount }}
      </span>
    </header>

    <div class="flex flex-1 flex-col gap-2 overflow-y-auto">
      <TaskWeeklyCard
        v-for="task in tasks"
        :key="task.identifier"
        :task="task"
        :categories="categories"
        @toggle="(id, done) => emit('toggle', id, done)"
        @edit="(id) => emit('edit', id)"
      />

      <div
        v-if="tasks.length === 0"
        class="flex min-h-20 flex-1 items-center justify-center rounded-lg border border-dashed border-gray-200 text-xs text-gray-400 dark:border-gray-700 dark:text-gray-500"
      >
        No tasks
      </div>
    </div>

    <button
      type="button"
      class="mt-3 flex items-center justify-center gap-1.5 rounded-lg border border-dashed border-gray-300 py-2 text-xs font-medium text-gray-500 transition-colors hover:border-gray-400 hover:text-gray-600 dark:border-gray-700 dark:text-gray-400 dark:hover:border-gray-600 dark:hover:text-gray-300"
      @click="emit('addTask', date)"
    >
      <UIcon name="heroicons:plus" class="size-3.5" />
      Add New Task
    </button>
  </section>
</template>
