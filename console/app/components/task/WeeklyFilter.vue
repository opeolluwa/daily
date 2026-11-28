<script setup lang="ts">
import type { TaskCategory } from "~/stores/task-categories";

const props = defineProps<{
  categories: TaskCategory[];
  activeFilters: {
    categories: Set<string>;
    priorities: Set<string>;
  };
}>();

const emit = defineEmits<{
  "update:filters": [
    filters: { categories: Set<string>; priorities: Set<string> },
  ];
}>();

const priorities = ["high", "medium", "low"] as const;
const priorityColors: Record<string, string> = {
  high: "bg-rose-500",
  medium: "bg-amber-500",
  low: "bg-emerald-500",
};

function toggleCategory(id: string) {
  const next = new Set(props.activeFilters.categories);
  if (next.has(id)) next.delete(id);
  else next.add(id);
  emit("update:filters", { ...props.activeFilters, categories: next });
}

function togglePriority(p: string) {
  const next = new Set(props.activeFilters.priorities);
  if (next.has(p)) next.delete(p);
  else next.add(p);
  emit("update:filters", { ...props.activeFilters, priorities: next });
}

function clearAll() {
  emit("update:filters", {
    categories: new Set(),
    priorities: new Set(),
  });
}

const hasActive = computed(
  () =>
    props.activeFilters.categories.size > 0 ||
    props.activeFilters.priorities.size > 0,
);
</script>

<template>
  <UPopover>
    <UButton
      icon="heroicons:funnel"
      color="neutral"
      :variant="hasActive ? 'soft' : 'ghost'"
      size="sm"
    />
    <template #content>
      <div class="w-56 p-3 flex flex-col gap-3">
        <div v-if="categories.length > 0" class="flex flex-col gap-1.5">
          <span
            class="text-[10px] font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500"
          >
            Category
          </span>
          <button
            v-for="cat in categories"
            :key="cat.identifier"
            type="button"
            class="flex items-center gap-2 rounded-md px-2 py-1.5 text-xs transition-colors w-full text-left"
            :class="
              activeFilters.categories.has(cat.identifier)
                ? 'bg-gray-100 dark:bg-gray-800 font-medium text-gray-800 dark:text-gray-100'
                : 'text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800/60'
            "
            @click="toggleCategory(cat.identifier)"
          >
            <span
              class="size-2 shrink-0 rounded-full"
              :style="{ backgroundColor: cat.color }"
            />
            {{ cat.name }}
          </button>
        </div>

        <div class="flex flex-col gap-1.5">
          <span
            class="text-[10px] font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500"
          >
            Priority
          </span>
          <button
            v-for="p in priorities"
            :key="p"
            type="button"
            class="flex items-center gap-2 rounded-md px-2 py-1.5 text-xs capitalize transition-colors w-full text-left"
            :class="
              activeFilters.priorities.has(p)
                ? 'bg-gray-100 dark:bg-gray-800 font-medium text-gray-800 dark:text-gray-100'
                : 'text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800/60'
            "
            @click="togglePriority(p)"
          >
            <span
              class="size-2 shrink-0 rounded-full"
              :class="priorityColors[p]"
            />
            {{ p }}
          </button>
        </div>

        <button
          v-if="hasActive"
          type="button"
          class="text-xs text-primary-500 hover:text-primary-600 dark:text-primary-400 dark:hover:text-primary-300 mt-1"
          @click="clearAll"
        >
          Clear filters
        </button>
      </div>
    </template>
  </UPopover>
</template>
