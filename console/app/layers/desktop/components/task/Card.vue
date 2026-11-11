<script setup lang="ts">
import { useTemplateRef } from "vue";
import { makeSortableItem } from "@vue-dnd-kit/core";
import { useTaskStore, type Task } from "@shared/stores/tasks";
import { useWorkspacesStore } from "@shared/stores/workspaces";

const props = defineProps<{
  task: Task;
}>();

const emit = defineEmits<{
  toggle: [identifier: string, done: boolean];
  edit: [identifier: string];
  delete: [identifier: string];
}>();

// Drag and drop
const el = useTemplateRef<HTMLElement>("el");
const grip = useTemplateRef<HTMLElement>("grip");

makeSortableItem(el, {
  id: () => props.task.identifier,
  item: () => props.task,
  handle: grip,
});

// Priority styling
const priorityBorderClass: Record<string, string> = {
  high: "border-l-rose-500/50",
  medium: "border-l-amber-500/50",
  low: "border-l-emerald-500/50",
};

const priorityBgClass: Record<string, string> = {
  high: "bg-rose-500/50",
  medium: "bg-amber-500/50",
  low: "bg-emerald-500/50",
};
// Due date formatting
function formatDueDate(dateStr: string | null) {
  if (!dateStr) return null;

  const date = new Date(dateStr);

  const datePart = date.toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
  });

  const timePart = date.toLocaleTimeString("en-US", {
    hour: "numeric",
    minute: "2-digit",
  });

  return `${datePart} · ${timePart}`;
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

// Workspace actions
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
    <RyderCard
      :title="task.title"
      class="rounded-sm! border-l-2 cursor-grab"
      :class="priorityBorderClass[task.priority]"
      :ui="{ title: 'mb-1', header: 'gap-y-1' }"
      ref="grip"
    >
      <template #trailing>
        <MetaControls
          horizontal-controls
          item-name="task"
          @edit-record="emit('edit', task.identifier)"
          @delete-record="emit('delete', task.identifier)"
          @duplicate-record="handleDuplicate"
          @transfer-record="handleTransfer"
        />
      </template>

      <template #default>
        <!-- Task information -->
        <div class="min-w-0 flex flex-col">
          <p
            v-if="task.description"
            class="mt-0.5 truncate text-xs text-gray-400"
          >
            {{ task.description }}
          </p>

          <div class="flex justify-between items-center mt-3 text-xs">
            <span
              v-if="task.dueDate"
              class="shrink-0 rounded-md px-1.5 py-0.5"
              :class="
                isToday(task.dueDate)
                  ? 'bg-primary-100 font-medium text-primary-600 dark:bg-primary-950 dark:text-primary-300'
                  : 'text-gray-400'
              "
            >
              {{ formatDueDate(task.dueDate) }}
            </span>

            <span
              class="uppercase shrink-0 rounded-md ml-auto px-1.5 py-0.5"
              :class="priorityBgClass[task.priority]"
              >{{ task.priority }}</span
            >
          </div>
        </div>
      </template>
      <!-- Due date -->
    </RyderCard>
  </li>
</template>
