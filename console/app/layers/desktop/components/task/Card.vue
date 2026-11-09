<script setup lang="ts">
import { useTemplateRef } from "vue";
import { makeSortableItem } from "@vue-dnd-kit/core";
import { useTodoStore, type Todo } from "@shared/stores/todo";
import { useWorkspacesStore } from "@shared/stores/workspaces";

const props = defineProps<{
  todo: Todo;
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
  id: () => props.todo.identifier,
  item: () => props.todo,
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
const todoStore = useTodoStore();

const currentWorkspaceId = computed(() => workspaceStore.activeWorkspaceId);

async function handleDuplicate(targetWorkspaceId: string) {
  await todoStore.duplicateTodo(
    props.todo.identifier,
    currentWorkspaceId.value,
    targetWorkspaceId,
  );
}

async function handleTransfer(targetWorkspaceId: string) {
  await todoStore.transferTodo(
    props.todo.identifier,
    currentWorkspaceId.value,
    targetWorkspaceId,
  );
}
</script>

<template>
  <li ref="el" class="list-none">
    <RyderCard
      :title="todo.title"
      class="rounded-sm! border-l-2 cursor-grab"
      :class="priorityBorderClass[todo.priority]"
      :ui="{ title: 'mb-1', header: 'gap-y-1' }"
      ref="grip"
    >
      <template #trailing>
        <MetaControls
          horizontal-controls
          item-name="todo"
          @edit-record="emit('edit', todo.identifier)"
          @delete-record="emit('delete', todo.identifier)"
          @duplicate-record="handleDuplicate"
          @transfer-record="handleTransfer"
        />
      </template>

      <template #default>
        <!-- Todo information -->
        <div class="min-w-0 flex flex-col">
          <p
            v-if="todo.description"
            class="mt-0.5 truncate text-xs text-gray-400"
          >
            {{ todo.description }}
          </p>

          <div class="flex justify-between items-center mt-3 text-xs">
            <span
              v-if="todo.dueDate"
              class="shrink-0 rounded-md px-1.5 py-0.5"
              :class="
                isToday(todo.dueDate)
                  ? 'bg-primary-100 font-medium text-primary-600 dark:bg-primary-950 dark:text-primary-300'
                  : 'text-gray-400'
              "
            >
              {{ formatDueDate(todo.dueDate) }}
            </span>

            <span
              class="uppercase shrink-0 rounded-md ml-auto px-1.5 py-0.5"
              :class="priorityBgClass[todo.priority]"
              >{{ todo.priority }}</span
            >
          </div>
        </div>
      </template>
      <!-- Due date -->
    </RyderCard>
  </li>
</template>
