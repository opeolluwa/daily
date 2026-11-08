
<script setup lang="ts">
import { useTemplateRef } from "vue";
import { makeSortableItem } from "@vue-dnd-kit/core";
import {
  useTodoStore,
  type Todo,
} from "@shared/stores/todo";
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

makeSortableItem(el, {
  item: () => props.todo,
});

// Priority styling
const priorityColor: Record<string, string> = {
  high: "text-rose-500",
  medium: "text-amber-500",
  low: "text-emerald-500",
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

const currentWorkspaceId = computed(
  () => workspaceStore.activeWorkspaceId,
);

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
  <li
    ref="el"
    class="list-none"
  >
    <RyderCard
      :ui="{
        root: 'flex items-center gap-4 transition-shadow hover:shadow-sm group cursor-grab active:cursor-grabbing',
      }"
    >
      <!-- Toggle -->
      <button
        type="button"
        @click="emit('toggle', todo.identifier, !todo.done)"
      >
        <UIcon
          :name="
            todo.done
              ? 'heroicons:check-circle-solid'
              : 'lucide:circle'
          "
          class="size-5 transition-colors"
          :class="
            todo.done
              ? 'text-primary-500'
              : 'text-gray-300 hover:text-gray-400 dark:text-gray-600'
          "
        />
      </button>

      <!-- Todo information -->
      <div class="min-w-0 flex-1">
        <p
          class="text-sm transition-colors"
          :class="
            todo.done
              ? 'text-gray-400 line-through'
              : 'text-gray-700 dark:text-gray-200'
          "
        >
          {{ todo.title }}
        </p>

        <p
          v-if="todo.description"
          class="mt-0.5 truncate text-xs text-gray-400"
        >
          {{ todo.description }}
        </p>
      </div>

      <!-- Due date -->
      <span
        v-if="todo.dueDate"
        class="shrink-0 rounded-md px-1.5 py-0.5 text-xs"
        :class="
          isToday(todo.dueDate)
            ? 'bg-primary-100 font-medium text-primary-600 dark:bg-primary-950 dark:text-primary-300'
            : 'text-gray-400'
        "
      >
        {{ formatDueDate(todo.dueDate) }}
      </span>

      <!-- Actions -->
      <div
        class="flex items-center gap-1 opacity-100 transition-opacity md:opacity-0 md:group-hover:opacity-100"
      >
        <button
          v-if="!todo.done"
          type="button"
          class="flex items-center gap-1 rounded-md px-2 py-1 text-xs font-medium text-emerald-600 transition-colors hover:bg-emerald-50 dark:text-emerald-400 dark:hover:bg-emerald-950"
          @click="emit('toggle', todo.identifier, true)"
        >
          <UIcon name="heroicons:check" class="size-3.5" />
          Done
        </button>

        <MetaControls
          item-name="todo"
          @edit-record="emit('edit', todo.identifier)"
          @delete-record="emit('delete', todo.identifier)"
          @duplicate-record="handleDuplicate"
          @transfer-record="handleTransfer"
        />
      </div>
    </RyderCard>
  </li>
</template>
