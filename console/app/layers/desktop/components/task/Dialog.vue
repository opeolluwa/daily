<script setup lang="ts">
import { CalendarDate, Time } from "@internationalized/date";
import { useTaskStore } from "@shared/stores/tasks";

const props = defineProps<{
  task?: {
    identifier: string;
    title: string;
    description?: string | null;
    dueDate?: string | null;
    dueTime?: string | null;
    priority: "high" | "medium" | "low";
    done: boolean;
  } | null;
}>();

const open = defineModel<boolean>("open", { required: true });

const taskStore = useTaskStore();
const toast = useToast();

const editing = computed(() => !!props.task);

const form = reactive({
  title: "",
  description: "",
  dueDate: undefined as CalendarDate | undefined,
  priority: "medium" as "high" | "medium" | "low",
  done: false,
});

const selectedTime = shallowRef<Time | undefined>();
const submitting = ref(false);
const deleting = ref(false);

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
  selectedTime.value = undefined;
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
      if (task.dueTime) {
        const [h, m] = task.dueTime.split(":").map(Number);
        selectedTime.value = new Time(h, m);
      } else {
        selectedTime.value = undefined;
      }
    } else {
      resetForm();
    }
  },
  { immediate: true },
);

watch(open, (val) => {
  if (!val) resetForm();
});

async function handleSubmit() {
  if (!form.title.trim()) return;
  submitting.value = true;
  try {
    if (editing.value && props.task) {
      await taskStore.updateTask(props.task.identifier, {
        title: form.title.trim(),
        description: form.description.trim() || undefined,
      });

      const current = taskStore.tasks.find(
        (t) => t.identifier === props.task!.identifier,
      );
      if (current) {
        if (current.done !== form.done) {
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
      await taskStore.createTask({
        title: form.title.trim(),
        description: form.description.trim() || undefined,
        dueDate: toIsoDate(form.dueDate) ?? undefined,
        time: selectedTime.value ? formatTime(selectedTime.value) : undefined,
        priority: form.priority,
      });
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
      <RyderInput
        v-model="form.title"
        label="Title"
        :required="true"
        placeholder="What needs to be done?"
        name="task-title"
      />

      <RyderTextarea
        v-model="form.description"
        label="Description"
        name="task-description"
        placeholder="Add more details..."
        :rows="3"
      />

      <RyderDateInput
        v-model="form.dueDate"
        label="Date"
        name="task-date"
        placeholder="Pick a date"
      />

      <RyderTimeInput
        v-model="selectedTime"
        label="Time"
        name="task-time"
        size="sm"
      />

      <div class="flex flex-col gap-1.5">
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

      <button
        v-if="editing"
        type="button"
        class="flex items-center gap-3 py-3 px-4 rounded-lg border transition-colors w-full"
        :class="
          form.done
            ? 'bg-emerald-50 dark:bg-emerald-950 border-emerald-300 dark:border-emerald-700'
            : 'bg-white dark:bg-gray-800 border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700'
        "
        @click="form.done = !form.done"
      >
        <UIcon
          :name="form.done ? 'heroicons:check-circle-solid' : 'lucide:circle'"
          class="size-5 transition-colors"
          :class="
            form.done ? 'text-emerald-500' : 'text-gray-300 dark:text-gray-600'
          "
        />
        <span
          class="text-sm font-medium transition-colors"
          :class="
            form.done
              ? 'text-emerald-700 dark:text-emerald-300'
              : 'text-gray-600 dark:text-gray-400'
          "
        >
          {{ form.done ? "Marked as done" : "Mark as done" }}
        </span>
      </button>

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
