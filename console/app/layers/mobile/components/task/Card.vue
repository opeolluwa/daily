<script setup lang="ts">
import { useTaskStore } from "@shared/stores/tasks";
import { useWorkspacesStore } from "@shared/stores/workspaces";
import type { Task } from "@shared/stores/tasks";
import MetaControls from "../meta/meta-controls.vue";

const { task } = defineProps<{
  task: Task;
}>();

const emit = defineEmits<{
  toggle: [identifier: string, done: boolean];
  edit: [identifier: string];
  delete: [identifier: string];
}>();

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
      root: 'flex items-center gap-4 transition-shadow hover:shadow-sm group !rounded-lg !border-gray-100 dark:!border-gray-700 dark:!bg-gray-800',
    }"
  >
    <UCheckbox
      :label="task.title"
      :description="String(task.description)"
      :v-model="task.done"
      :ui="{
        label: task.done ? 'line-through text-gray-400' : '',
        description: task.done ? 'line-through text-gray-400' : '',
        root: 'w-full',
      }"
      @update:model-value="(e) => emit('toggle', task.identifier, e)"
    />

    <div
      class="flex items-center gap-1 opacity-100 md:opacity-0 md:group-hover:opacity-100 transition-opacity"
    >
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
