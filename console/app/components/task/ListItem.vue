<script setup lang="ts">
import { useTaskStore } from "~/stores/tasks";
import { useWorkspacesStore } from "~/stores/workspaces";
import type { Task } from "~/stores/tasks";

const { task } = defineProps<{
  task: Task;
}>();

const emit = defineEmits<{
  toggle: [identifier: string, done: boolean];
  edit: [identifier: string];
  delete: [identifier: string];
}>();

function formatDueDate(dateStr: string | null) {
  if (!dateStr) return null;
  const d = new Date(dateStr);
  const datePart = d.toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
  });
  const timePart = d.toLocaleTimeString("en-US", {
    hour: "numeric",
    minute: "2-digit",
  });
  return `${datePart} · ${timePart}`;
}

function isToday(dateStr: string | null) {
  if (!dateStr) return false;
  const d = new Date(dateStr);
  const now = new Date();
  return (
    d.getFullYear() === now.getFullYear() &&
    d.getMonth() === now.getMonth() &&
    d.getDate() === now.getDate()
  );
}
const workspaceStore = useWorkspacesStore();
const currentWorkspaceId = computed(() => workspaceStore.activeWorkspaceId);
const taskStore = useTaskStore();
const handleDuplicate = async (targetWorkspaceId: string) => {
  await taskStore.duplicateTask(
    task.identifier,
    currentWorkspaceId.value,
    targetWorkspaceId,
  );
};

const handleTransfer = async (targetWorkspaceId: string) => {
  await taskStore.transferTask(
    task.identifier,
    currentWorkspaceId.value,
    targetWorkspaceId,
  );
};
</script>

<template>
  <RyderCard
    :ui="{
      root: 'flex items-center gap-4 hover:shadow-sm transition-shadow group',
    }"
  >
    <button @click="emit('toggle', task.identifier, !task.done)">
      <UIcon
        :name="task.done ? 'heroicons:check-circle-solid' : 'lucide:circle'"
        class="size-5 transition-colors"
        :class="
          task.done
            ? 'text-primary-500'
            : 'text-gray-300 dark:text-gray-600 hover:text-gray-400'
        "
      />
    </button>

    <div class="flex-1 min-w-0">
      <p
        class="text-sm transition-colors"
        :class="
          task.done
            ? 'line-through text-gray-400'
            : 'text-gray-700 dark:text-gray-200'
        "
      >
        {{ task.title }}
      </p>
      <p v-if="task.description" class="text-xs text-gray-400 truncate mt-0.5">
        {{ task.description }}
      </p>
    </div>

    <span
      v-if="task.dueDate"
      class="text-xs shrink-0 px-1.5 py-0.5 rounded-md"
      :class="
        isToday(task.dueDate)
          ? 'bg-primary-100 dark:bg-primary-950 text-primary-600 dark:text-primary-300 font-medium'
          : 'text-gray-400'
      "
    >
      {{ formatDueDate(task.dueDate) }}
    </span>

    <div
      class="flex items-center gap-1 opacity-100 md:opacity-0 md:group-hover:opacity-100 transition-opacity"
    >
      <button
        v-if="!task.done"
        class="flex items-center gap-1 px-2 py-1 rounded-md text-xs font-medium text-emerald-600 dark:text-emerald-400 hover:bg-emerald-50 dark:hover:bg-emerald-950 transition-colors"
        @click="emit('toggle', task.identifier, true)"
      >
        <UIcon name="heroicons:check" class="size-3.5" />
        Done
      </button>
      <MetaControls
        item-name="task"
        @edit-record="emit('edit', task.identifier)"
        @delete-record="emit('delete', task.identifier)"
        @duplicate-record="
          (targetWorkspaceId) => handleDuplicate(targetWorkspaceId)
        "
        @transfer-record="
          (targetWorkspaceId) => handleTransfer(targetWorkspaceId)
        "
      />
    </div>
  </RyderCard>
</template>
