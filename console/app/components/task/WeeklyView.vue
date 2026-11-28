<script setup lang="ts">
import { useTaskStore, type Task } from "~/stores/tasks";
import { useTaskCategoryStore } from "~/stores/task-categories";

const emit = defineEmits<{
  edit: [identifier: string];
  toggle: [identifier: string, done: boolean];
  delete: [identifier: string];
  addTask: [date: string];
}>();

const taskStore = useTaskStore();
const categoryStore = useTaskCategoryStore();

const DAYS_TO_SHOW = 4;
const startDate = ref(new Date());

function toIso(d: Date): string {
  return [
    d.getFullYear(),
    String(d.getMonth() + 1).padStart(2, "0"),
    String(d.getDate()).padStart(2, "0"),
  ].join("-");
}

function todayIso(): string {
  return toIso(new Date());
}

const dates = computed(() => {
  const result: string[] = [];
  for (let i = 0; i < DAYS_TO_SHOW; i++) {
    const d = new Date(startDate.value);
    d.setDate(d.getDate() + i);
    result.push(toIso(d));
  }
  return result;
});

const dateRangeLabel = computed(() => {
  const first = new Date(dates.value[0] + "T00:00:00");
  const last = new Date(dates.value[dates.value.length - 1] + "T00:00:00");
  const opts: Intl.DateTimeFormatOptions = { day: "numeric", month: "short" };
  return `${first.toLocaleDateString("en-US", opts)} - ${last.toLocaleDateString("en-US", opts)}`;
});

function navigate(dir: number) {
  const d = new Date(startDate.value);
  d.setDate(d.getDate() + dir * DAYS_TO_SHOW);
  startDate.value = d;
}

function goToToday() {
  startDate.value = new Date();
}

const activeFilters = ref<{
  categories: Set<string>;
  priorities: Set<string>;
}>({ categories: new Set(), priorities: new Set() });

function tasksForDate(dateStr: string): Task[] {
  const all = taskStore.tasksByDate.get(dateStr) ?? [];
  const { categories, priorities } = activeFilters.value;
  if (categories.size === 0 && priorities.size === 0) return all;
  return all.filter((t) => {
    if (priorities.size > 0 && !priorities.has(t.priority)) return false;
    if (categories.size > 0) {
      const catId = (t as Record<string, unknown>).categoryIdentifier as
        | string
        | null;
      if (!catId || !categories.has(catId)) return false;
    }
    return true;
  });
}

const unscheduledCount = computed(() => {
  return (taskStore.tasksByDate.get("__unscheduled__") ?? []).length;
});

const commentOpen = ref(false);
const shareOpen = ref(false);

function handleShare() {
  const url = new URL(window.location.href);
  url.searchParams.set("from", dates.value[0]);
  url.searchParams.set("to", dates.value[dates.value.length - 1]);
  navigator.clipboard.writeText(url.toString());
}

function exportAsText() {
  const lines: string[] = [`Tasks: ${dateRangeLabel.value}`, ""];
  for (const dateStr of dates.value) {
    const d = new Date(dateStr + "T00:00:00");
    lines.push(
      d.toLocaleDateString("en-US", {
        weekday: "long",
        month: "short",
        day: "numeric",
      }),
    );
    const tasks = tasksForDate(dateStr);
    if (tasks.length === 0) {
      lines.push("  (no tasks)");
    } else {
      for (const t of tasks) {
        lines.push(`  ${t.done ? "[x]" : "[ ]"} ${t.title}`);
      }
    }
    lines.push("");
  }
  navigator.clipboard.writeText(lines.join("\n"));
}

onMounted(() => {
  const route = useRoute();
  if (route.query.from) {
    startDate.value = new Date(route.query.from as string + "T00:00:00");
  }
});
</script>

<template>
  <div class="flex h-full flex-col">
    <!-- Header -->
    <header class="mb-4 flex items-center justify-between gap-4">
      <div class="flex items-center gap-2">
        <button
          type="button"
          class="rounded-lg p-1.5 text-gray-500 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800"
          @click="navigate(-1)"
        >
          <UIcon name="heroicons:chevron-left" class="size-5" />
        </button>

        <h2
          class="min-w-[160px] text-center text-sm font-semibold text-gray-800 dark:text-gray-100 cursor-pointer hover:text-primary-600 dark:hover:text-primary-400 transition-colors"
          @click="goToToday"
        >
          {{ dateRangeLabel }}
        </h2>

        <button
          type="button"
          class="rounded-lg p-1.5 text-gray-500 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800"
          @click="navigate(1)"
        >
          <UIcon name="heroicons:chevron-right" class="size-5" />
        </button>
      </div>

      <div class="flex items-center gap-1">
        <TaskWeeklyFilter
          :categories="categoryStore.categories"
          :active-filters="activeFilters"
          @update:filters="activeFilters = $event"
        />

        <UButton
          icon="heroicons:chat-bubble-left-right"
          color="neutral"
          variant="ghost"
          size="sm"
          @click="commentOpen = true"
        />

        <UPopover>
          <UButton
            icon="heroicons:arrow-up-tray"
            color="neutral"
            variant="ghost"
            size="sm"
          />
          <template #content>
            <div class="flex flex-col gap-1 p-2 w-48">
              <button
                type="button"
                class="flex items-center gap-2 rounded-md px-3 py-2 text-sm text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-800 w-full text-left"
                @click="handleShare"
              >
                <UIcon name="heroicons:link" class="size-4" />
                Copy link
              </button>
              <button
                type="button"
                class="flex items-center gap-2 rounded-md px-3 py-2 text-sm text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-800 w-full text-left"
                @click="exportAsText"
              >
                <UIcon name="heroicons:document-text" class="size-4" />
                Export as text
              </button>
            </div>
          </template>
        </UPopover>
      </div>
    </header>

    <!-- Unscheduled banner -->
    <div
      v-if="unscheduledCount > 0"
      class="mb-3 flex items-center gap-2 rounded-lg bg-amber-50 px-3 py-2 text-xs text-amber-700 dark:bg-amber-950/40 dark:text-amber-400"
    >
      <UIcon name="heroicons:exclamation-triangle" class="size-4 shrink-0" />
      {{ unscheduledCount }} unscheduled task{{
        unscheduledCount > 1 ? "s" : ""
      }}
    </div>

    <!-- Day columns -->
    <div class="grid min-h-0 flex-1 grid-cols-4 gap-4">
      <TaskWeeklyColumn
        v-for="dateStr in dates"
        :key="dateStr"
        :date="dateStr"
        :tasks="tasksForDate(dateStr)"
        :categories="categoryStore.categories"
        :is-today="dateStr === todayIso()"
        class="min-w-0"
        @edit="(id) => emit('edit', id)"
        @delete="(id) => emit('delete', id)"
        @toggle="(id, done) => emit('toggle', id, done)"
        @add-task="(date) => emit('addTask', date)"
      />
    </div>

    <!-- Comment slideover -->
    <USlideover v-model:open="commentOpen">
      <template #header>
        <h3 class="text-base font-semibold text-gray-800 dark:text-gray-100">
          Task Notes
        </h3>
      </template>
      <template #body>
        <div class="flex flex-col gap-4 p-4">
          <template v-for="dateStr in dates" :key="dateStr">
            <div
              v-for="task in tasksForDate(dateStr).filter((t) => t.description)"
              :key="task.identifier"
              class="rounded-lg border border-gray-200 p-3 dark:border-gray-700"
            >
              <h4
                class="text-sm font-medium text-gray-800 dark:text-gray-100 mb-1"
              >
                {{ task.title }}
              </h4>
              <p class="text-xs text-gray-500 dark:text-gray-400 whitespace-pre-wrap">
                {{ task.description }}
              </p>
            </div>
          </template>
          <p
            v-if="
              dates.every(
                (d) => tasksForDate(d).filter((t) => t.description).length === 0,
              )
            "
            class="text-center text-sm text-gray-400 dark:text-gray-500 py-8"
          >
            No task notes for this week
          </p>
        </div>
      </template>
    </USlideover>
  </div>
</template>
