<script setup lang="ts">
import { useTaskStore, type Task } from "~/stores/tasks";
import type { TaskCategory } from "~/stores/task-categories";
import { useWorkspacesStore } from "~/stores/workspaces";
import TaskProgressRing from "./TaskProgressRing.vue";

const props = defineProps<{
  task: Task;
  categories: TaskCategory[];
}>();

const emit = defineEmits<{
  toggle: [identifier: string, done: boolean];
  edit: [identifier: string];
  delete: [identifier: string];
}>();

const taskStore = useTaskStore();
const workspaceStore = useWorkspacesStore();
const currentWorkspaceId = computed(() => workspaceStore.activeWorkspaceId);
const subtaskCount = computed(
  () => taskStore.subtaskCountByTask[props.task.identifier] ?? 0,
);
const subtaskProgress = computed(
  () => taskStore.progressByTask[props.task.identifier] ?? 0,
);

const category = computed(() => {
  const catId = (props.task as Record<string, unknown>).categoryIdentifier as
    | string
    | null
    | undefined;
  if (!catId) return null;
  return props.categories.find((c) => c.identifier === catId) ?? null;
});

function formatTime(timeStr: string | null | undefined): string | null {
  if (!timeStr) return null;
  const [h, m] = timeStr.split(":").map(Number);
  const hour12 = h % 12 || 12;
  const ampm = h < 12 ? "AM" : "PM";
  return `${hour12}:${String(m).padStart(2, "0")} ${ampm}`;
}

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
  <div
    class="group flex items-start gap-3 rounded-lg border border-gray-200 bg-white p-3 transition-all hover:shadow-sm hover:border-gray-300 dark:bg-gray-800 dark:border-gray-600 dark:hover:border-gray-500 cursor-pointer"
    @click="emit('edit', task.identifier)"
  >
    <button
      type="button"
      class="mt-0.5 shrink-0"
      @click.stop="emit('toggle', task.identifier, !task.done)"
    >
      <UIcon
        :name="task.done ? 'heroicons:check-circle-solid' : 'lucide:circle'"
        class="size-5 transition-colors"
        :class="
          task.done
            ? 'text-primary-500'
            : 'text-gray-300 dark:text-gray-500 hover:text-gray-400 dark:hover:text-gray-400'
        "
      />
    </button>

    <div class="min-w-0 flex-1">
      <div class="flex items-start justify-between gap-2">
        <h4
          class="min-w-0 truncate text-sm font-medium transition-colors"
          :class="
            task.done
              ? 'text-gray-400 line-through dark:text-gray-500'
              : 'text-gray-800 dark:text-gray-100'
          "
        >
          {{ task.title }}
        </h4>
        <div
          class="shrink-0 opacity-0 transition-opacity group-hover:opacity-100"
          @click.stop
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
      <p
        v-if="task.description"
        class="mt-0.5 truncate text-xs text-gray-400 dark:text-gray-400"
      >
        {{ task.description }}
      </p>

      <div class="mt-2 flex items-center gap-2">
        <TaskProgressRing
          v-if="subtaskCount > 0"
          :progress="subtaskProgress"
          :size="26"
        />

        <span
          v-if="category"
          class="inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-[10px] font-medium"
          :style="{
            backgroundColor: category.color + '30',
            color: category.color,
          }"
        >
          <span
            class="size-1.5 rounded-full"
            :style="{ backgroundColor: category.color }"
          />
          {{ category.name }}
        </span>

        <span
          v-if="task.dueTime"
          class="ml-auto inline-flex items-center gap-1 text-[10px] text-gray-400 dark:text-gray-400"
        >
          <UIcon name="heroicons:clock" class="size-3" />
          {{ formatTime(task.dueTime) }}
        </span>
      </div>
    </div>
  </div>
</template>
