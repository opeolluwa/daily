<script setup lang="ts">
import { CalendarDate, Time } from "@internationalized/date";
import { useTaskStore, type TaskSubtaskDraft } from "~/stores/tasks";
import { useTaskCategoryStore } from "~/stores/task-categories";
import TaskDetails from "./TaskDetails.vue";

const props = defineProps<{
  task?: {
    identifier: string;
    title: string;
    description?: string | null;
    dueDate?: string | null;
    dueTime?: string | null;
    priority: "high" | "medium" | "low";
    done: boolean;
    categoryIdentifier?: string | null;
  } | null;
  initialDate?: string | null;
}>();

const open = defineModel<boolean>("open", { required: true });

const taskStore = useTaskStore();
const categoryStore = useTaskCategoryStore();
const toast = useToast();

const editing = computed(() => !!props.task);
const categoryManagerOpen = ref(false);

const form = reactive({
  title: "",
  description: "",
  dueDate: undefined as CalendarDate | undefined,
  priority: "medium" as "high" | "medium" | "low",
  done: false,
  categoryIdentifier: null as string | null,
});

const selectedTime = shallowRef<Time | undefined>();
const subtasks = ref<TaskSubtaskDraft[]>([]);
const doneTouched = ref(false);
const submitting = ref(false);
const deleting = ref(false);

onMounted(() => {
  if (categoryStore.categories.length === 0) categoryStore.fetchCategories();
});

function toIsoDate(date: CalendarDate | undefined): string | null {
  if (!date) return null;
  return [
    date.year,
    String(date.month).padStart(2, "0"),
    String(date.day).padStart(2, "0"),
  ].join("-");
}

function fromIsoDate(
  value: string | null | undefined,
): CalendarDate | undefined {
  if (!value) return undefined;
  const [year, month, day] = value.split("-").map(Number);
  if (!year || !month || !day) return undefined;
  return new CalendarDate(year, month, day);
}

function formatTime(time: Time): string {
  return `${String(time.hour).padStart(2, "0")}:${String(time.minute).padStart(2, "0")}`;
}

function resetForm() {
  form.title = "";
  form.description = "";
  form.dueDate = undefined;
  form.priority = "medium";
  form.done = false;
  form.categoryIdentifier = null;
  selectedTime.value = undefined;
  subtasks.value = [];
  doneTouched.value = false;
}

function onDoneChange(value: boolean) {
  form.done = value;
  doneTouched.value = true;
}

function onSubtasksChange(value: TaskSubtaskDraft[]) {
  subtasks.value = value;
  if (value.length > 0) {
    form.done = value.every((subtask) => subtask.done);
    doneTouched.value = false;
  }
}

async function loadDraftSubtasks(task: { identifier: string }) {
  const cached = taskStore.subtasksByTask[task.identifier];
  if (cached) {
    subtasks.value = cached.map((subtask) => ({ ...subtask }));
    if (subtasks.value.length > 0) {
      form.done = subtasks.value.every((subtask) => subtask.done);
    }
    return;
  }
  try {
    subtasks.value = await taskStore.loadSubtasks(task.identifier);
    if (subtasks.value.length > 0) {
      form.done = subtasks.value.every((subtask) => subtask.done);
    }
  } catch {
    subtasks.value = [];
  }
}

watch(
  () => props.task,
  (task) => {
    if (task) {
      form.title = task.title;
      form.description = task.description ?? "";
      form.dueDate = fromIsoDate(task.dueDate);
      form.priority = task.priority;
      form.done = task.done;
      form.categoryIdentifier = task.categoryIdentifier ?? null;
      doneTouched.value = false;
      void loadDraftSubtasks(task);
      if (task.dueTime) {
        const [h, m] = task.dueTime.split(":").map(Number);
        selectedTime.value = new Time(h, m);
      } else {
        selectedTime.value = undefined;
      }
    } else {
      resetForm();
      if (props.initialDate) {
        form.dueDate = fromIsoDate(props.initialDate);
      }
    }
  },
  { immediate: true },
);

watch(open, (val) => {
  if (val) {
    if (props.task) {
      void loadDraftSubtasks(props.task);
    } else {
      resetForm();
      if (props.initialDate) {
        form.dueDate = fromIsoDate(props.initialDate);
      }
    }
  } else {
    resetForm();
  }
});

async function handleSubmit() {
  if (!form.title.trim()) return;
  submitting.value = true;
  try {
    if (editing.value && props.task) {
      await taskStore.updateTask(props.task.identifier, {
        title: form.title.trim(),
        description: form.description.trim() || null,
        categoryIdentifier: form.categoryIdentifier,
      } as Record<string, unknown>);

      await taskStore.saveSubtasks(props.task.identifier, subtasks.value);

      const current = taskStore.tasks.find(
        (t) => t.identifier === props.task!.identifier,
      );
      if (current) {
        if (doneTouched.value && current.done !== form.done) {
          await taskStore.toggleDone(props.task.identifier, form.done);
        }
        if (current.priority !== form.priority) {
          await taskStore.changePriority(props.task.identifier, form.priority);
        }
        const newDate = toIsoDate(form.dueDate);
        const existingDate = current.dueDate ?? null;
        if (newDate !== existingDate) {
          await taskStore.updateDueDate(props.task.identifier, newDate);
        }
      }

      toast.add({
        color: "success",
        icon: "heroicons:check-circle",
        title: "Task Updated",
        description: "Your changes have been saved",
      });
    } else {
      const created = await taskStore.createTask({
        title: form.title.trim(),
        description: form.description.trim() || null,
        dueDate: toIsoDate(form.dueDate) ?? undefined,
        time: selectedTime.value ? formatTime(selectedTime.value) : undefined,
        priority: form.priority,
        categoryIdentifier: form.categoryIdentifier ?? undefined,
      });
      await taskStore.saveSubtasks(created.identifier, subtasks.value);
      toast.add({
        color: "success",
        icon: "heroicons:check-circle",
        title: "Task Created",
        description: "Your task has been saved",
      });
    }
    open.value = false;
  } catch {
    toast.add({
      color: "error",
      icon: "heroicons:x-circle",
      title: "Something Went Wrong",
      description: editing.value
        ? "Failed to update task"
        : "Failed to create task",
    });
  } finally {
    submitting.value = false;
  }
}

async function handleDelete() {
  if (!props.task) return;
  deleting.value = true;
  try {
    await taskStore.deleteTask(props.task.identifier);
    toast.add({
      color: "success",
      icon: "heroicons:check-circle",
      title: "Task Deleted",
      description: "The task has been removed",
    });
    open.value = false;
  } catch {
    toast.add({
      color: "error",
      icon: "heroicons:x-circle",
      title: "Something Went Wrong",
      description: "Failed to delete task",
    });
  } finally {
    deleting.value = false;
  }
}
</script>

<template>
  <RyderCreateDialog
    v-model:open="open"
    :title="editing ? 'Edit Task' : 'New Task'"
    :loading="submitting"
    :submit-label="editing ? 'Save' : 'Create'"
    :submit-disabled="!form.title.trim()"
    @submit="handleSubmit"
    @cancel="resetForm"
  >
    <div class="flex flex-col gap-4">
      <TaskDetails
        v-model:title="form.title"
        v-model:description="form.description"
        v-model:due-date="form.dueDate"
        v-model:category-identifier="form.categoryIdentifier"
        :done="form.done"
        :subtasks="subtasks"
        :categories="categoryStore.categories"
        :busy="submitting"
        @update:done="onDoneChange"
        @update:subtasks="onSubtasksChange"
        @manage-categories="categoryManagerOpen = true"
      >
        <template #extras>
          <RyderTimeInput
            v-model="selectedTime"
            label="Time"
            name="task-time"
            size="sm"
          />

          <div class="mt-4 flex flex-col gap-1.5">
            <label class="text-xs font-medium text-gray-500 dark:text-gray-400">
              Priority
            </label>
            <div class="flex gap-2">
              <button
                v-for="p in ['low', 'medium', 'high'] as const"
                :key="p"
                type="button"
                class="flex-1 py-2 rounded-lg text-xs font-medium capitalize border transition-colors"
                :class="
                  form.priority === p
                    ? p === 'high'
                      ? 'bg-rose-50 dark:bg-rose-950 border-rose-300 dark:border-rose-700 text-rose-600 dark:text-rose-400'
                      : p === 'medium'
                        ? 'bg-amber-50 dark:bg-amber-950 border-amber-300 dark:border-amber-700 text-amber-600 dark:text-amber-400'
                        : 'bg-emerald-50 dark:bg-emerald-950 border-emerald-300 dark:border-emerald-700 text-emerald-600 dark:text-emerald-400'
                    : 'bg-white dark:bg-gray-800 border-gray-200 dark:border-gray-700 text-gray-500 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-700'
                "
                @click="form.priority = p"
              >
                {{ p }}
              </button>
            </div>
          </div>
        </template>
      </TaskDetails>

      <TaskCategoryManager v-model:open="categoryManagerOpen" />

      <button
        v-if="editing"
        type="button"
        :disabled="deleting"
        class="flex items-center justify-center gap-2 py-2 px-4 rounded-lg text-sm text-rose-500 hover:bg-rose-50 dark:hover:bg-rose-950 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
        @click="handleDelete"
      >
        <UIcon name="heroicons:trash" class="size-4" />
        {{ deleting ? "Deleting..." : "Delete task" }}
      </button>
    </div>
  </RyderCreateDialog>
</template>
