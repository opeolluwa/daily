<script setup lang="ts">
import { useTemplateRef } from "vue";
import { makeSortableItem } from "@vue-dnd-kit/core";
import { useTaskStore, type Task } from "~/stores/tasks";
import { useWorkspacesStore } from "~/stores/workspaces";

const props = defineProps<{
  task: Task;
}>();

const emit = defineEmits<{
  toggle: [identifier: string, done: boolean];
  edit: [identifier: string];
  delete: [identifier: string];
}>();

const el = useTemplateRef<HTMLElement>("el");
const grip = useTemplateRef<HTMLElement>("grip");

makeSortableItem(el, {
  id: () => props.task.identifier,
  item: () => props.task,
  handle: grip,
});

const priorityBorderClass: Record<string, string> = {
  high: "border-rose-500",
  medium: "border-amber-500",
  low: "border-emerald-500",
};

const priorityDotClass: Record<string, string> = {
  high: "bg-rose-500",
  medium: "bg-amber-500",
  low: "bg-emerald-500",
};

function formatDueDate(dateStr: string | null) {
  if (!dateStr) return null;

  const date = new Date(dateStr);
  const now = new Date();
  const tomorrow = new Date(now);
  tomorrow.setDate(tomorrow.getDate() + 1);

  if (
    date.getFullYear() === now.getFullYear() &&
    date.getMonth() === now.getMonth() &&
    date.getDate() === now.getDate()
  )
    return "Today";

  if (
    date.getFullYear() === tomorrow.getFullYear() &&
    date.getMonth() === tomorrow.getMonth() &&
    date.getDate() === tomorrow.getDate()
  )
    return "Tomorrow";

  return date.toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
  });
}

function isToday(dateStr: string | null) {
  if (!dateStr) return false;
  const date = new Date(dateStr);
  const now = new Date();
  return (
    date.getFullYear() === now.getFullYear() &&
    date.getMonth() === now.getMonth() &&
    date.getDate() === now.getDate()
  );
}

const workspaceStore = useWorkspacesStore();
const taskStore = useTaskStore();

const currentWorkspaceId = computed(() => workspaceStore.activeWorkspaceId);

async function handleDuplicate(targetWorkspaceId: string) {
  await taskStore.duplicateTask(
    props.task.identifier,
    currentWorkspaceId.value,
    targetWorkspaceId,
  );
}

async function handleTransfer(targetWorkspaceId: string) {
  await taskStore.transferTask(
    props.task.identifier,
    currentWorkspaceId.value,
    targetWorkspaceId,
  );
}
</script>

<template>
  <li ref="el" class="list-none">
    <div
      ref="grip"
      class="group relative cursor-grab rounded-lg border border-gray-200 bg-white p-3 transition-all hover:shadow-sm hover:border-gray-300 dark:bg-gray-800/60 dark:border-gray-700/50 dark:hover:border-gray-600"
    >
      <!-- Header: title + menu -->
      <div class="flex items-start justify-between gap-2">
        <h3
          class="min-w-0 truncate text-sm font-medium text-gray-800 dark:text-gray-100"
        >
          {{ task.title }}
        </h3>
        <div
          class="shrink-0 opacity-0 group-hover:opacity-100 transition-opacity"
        >
          <MetaControls
            horizontal-controls
            item-name="task"
            @edit-record="emit('edit', task.identifier)"
            @delete-record="emit('delete', task.identifier)"
            @duplicate-record="handleDuplicate"
            @transfer-record="handleTransfer"
          />
        </div>
      </div>

      <!-- Description -->
      <p
        v-if="task.description"
        class="mt-1 truncate text-xs text-gray-400 dark:text-gray-500"
      >
        {{ task.description }}
      </p>

      <!-- Footer: due date + priority -->
      <div class="mt-3 flex items-center gap-2 text-xs">
        <span
          v-if="task.dueDate"
          class="shrink-0 rounded-md px-1.5 py-0.5"
          :class="
            isToday(task.dueDate)
              ? 'bg-primary-100 font-medium text-primary-600 dark:bg-primary-950 dark:text-primary-300'
              : 'text-gray-400 dark:text-gray-500'
          "
        >
          {{ formatDueDate(task.dueDate) }}
        </span>

        <span class="ml-auto flex items-center gap-1.5 shrink-0">
          <span
            class="size-1.5 rounded-full"
            :class="priorityDotClass[task.priority]"
          />
          <span
            class="text-xs font-medium capitalize text-gray-400 dark:text-gray-500"
          >
            {{ task.priority }}
          </span>
        </span>
      </div>
    </div>
  </li>
</template>
