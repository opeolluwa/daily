<script setup lang="ts">
import { CalendarDate, Time } from "@internationalized/date";
import { useTaskCategoryStore } from "~/stores/task-categories";
import type { TaskSubtaskDraft } from "~/stores/tasks";
import TaskDetails from "~/components/task/TaskDetails.vue";

definePageMeta({ layout: false, name: "Edit Task" });

const route = useRoute();
const router = useRouter();
const taskStore = useTaskStore();
const categoryStore = useTaskCategoryStore();
const toast = useToast();

const id = computed(() => route.query.id as string | undefined);
const task = computed(
  () => taskStore.tasks.find((t) => t.identifier === id.value) ?? null,
);

const priorityOptions = [
  { label: "Low", value: "low" },
  { label: "Medium", value: "medium" },
  { label: "High", value: "high" },
];

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

function hydrate() {
  if (!task.value) return;
  form.title = task.value.title;
  form.description = task.value.description ?? "";
  form.dueDate = fromIsoDate(task.value.dueDate);
  form.priority = task.value.priority;
  form.done = task.value.done;
  form.categoryIdentifier =
    ((task.value as Record<string, unknown>).categoryIdentifier as
      string | null) ?? null;
  if (task.value.dueTime) {
    const [h, m] = task.value.dueTime.split(":").map(Number);
    selectedTime.value = new Time(h, m);
  } else {
    selectedTime.value = undefined;
  }
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

async function loadDraftSubtasks(currentTask: { identifier: string }) {
  const cached = taskStore.subtasksByTask[currentTask.identifier];
  if (cached) {
    subtasks.value = cached.map((subtask) => ({ ...subtask }));
    if (subtasks.value.length > 0) {
      form.done = subtasks.value.every((subtask) => subtask.done);
    }
    return;
  }
  try {
    subtasks.value = await taskStore.loadSubtasks(currentTask.identifier);
    if (subtasks.value.length > 0) {
      form.done = subtasks.value.every((subtask) => subtask.done);
    }
  } catch {
    subtasks.value = [];
  }
}

watch(
  task,
  (currentTask) => {
    if (!currentTask) return;
    hydrate();
    doneTouched.value = false;
    void loadDraftSubtasks(currentTask);
  },
  { immediate: true },
);

onMounted(async () => {
  if (taskStore.tasks.length === 0) await taskStore.fetchTasks();
  if (categoryStore.categories.length === 0)
    await categoryStore.fetchCategories();
  hydrate();
});

async function handleSubmit() {
  if (!task.value || !form.title.trim()) return;
  submitting.value = true;
  try {
    await taskStore.updateTask(task.value.identifier, {
      title: form.title.trim(),
      description: form.description.trim() || null,
      categoryIdentifier: form.categoryIdentifier,
    } as Record<string, unknown>);
    await taskStore.saveSubtasks(task.value.identifier, subtasks.value);
    const current = taskStore.tasks.find(
      (t) => t.identifier === task.value!.identifier,
    );
    if (current) {
      if (doneTouched.value && current.done !== form.done)
        await taskStore.toggleDone(task.value.identifier, form.done);
      if (current.priority !== form.priority)
        await taskStore.changePriority(task.value.identifier, form.priority);
      const newDate = toIsoDate(form.dueDate);
      if ((current.dueDate ?? null) !== newDate)
        await taskStore.updateDueDate(task.value.identifier, newDate);
    }
    toast.add({
      color: "success",
      icon: "heroicons:check-circle",
      title: "Task Updated",
      description: "Your changes have been saved",
    });
    await router.push("/tasks");
  } catch {
    toast.add({
      color: "error",
      icon: "heroicons:x-circle",
      title: "Something Went Wrong",
      description: "Failed to update task",
    });
  } finally {
    submitting.value = false;
  }
}

async function handleDelete() {
  if (!task.value) return;
  deleting.value = true;
  try {
    await taskStore.deleteTask(task.value.identifier);
    toast.add({
      color: "success",
      icon: "heroicons:check-circle",
      title: "Task Deleted",
      description: "The task has been removed",
    });
    await router.push("/tasks");
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
  <NuxtLayout name="default">
    <template #main_content>
      <div class="max-w-3xl">
        <div class="mb-6 flex items-center gap-3">
          <h1 class="text-lg font-semibold text-gray-800 dark:text-gray-100">
            Edit Task
          </h1>
        </div>

        <div
          v-if="!task"
          class="flex flex-col items-center justify-center py-20 text-center"
        >
          <UIcon
            name="heroicons:check-circle"
            class="size-7 text-gray-400 mb-3"
          />
          <p class="text-sm text-gray-600 dark:text-gray-400 mb-3">
            Task not found
          </p>
          <UButton size="sm" variant="outline" @click="router.push('/tasks')"
            >Back to tasks</UButton
          >
        </div>

        <form v-else class="flex flex-col gap-4" @submit.prevent="handleSubmit">
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
          >
            <template #extras>
              <div class="flex items-start gap-4">
                <div class="min-w-0 flex-1">
                  <RyderTimeInput
                    v-model="selectedTime"
                    label="Time"
                    label-class="text-xs font-medium text-gray-500 dark:text-gray-400"
                    name="task-time"
                    size="sm"
                  />
                </div>
                <div class="min-w-0 flex-1">
                  <RyderSelect
                    v-model="form.priority"
                    :items="priorityOptions"
                    label="Priority"
                    label-class="text-xs font-medium text-gray-500 dark:text-gray-400"
                    name="task-priority"
                    size="sm"
                  />
                </div>
              </div>
            </template>
          </TaskDetails>

          <button
            type="button"
            :disabled="deleting"
            class="flex items-center justify-center gap-2 py-2 px-4 rounded-lg text-sm text-rose-500 hover:bg-rose-50 dark:hover:bg-rose-950 disabled:opacity-50"
            @click="handleDelete"
          >
            <UIcon name="heroicons:trash" class="size-4" />
            {{ deleting ? "Deleting..." : "Delete task" }}
          </button>

          <div class="flex justify-end gap-2 pt-2">
            <UButton
              type="button"
              color="neutral"
              variant="ghost"
              :disabled="submitting"
              @click="router.back()"
              >Cancel</UButton
            >
            <UButton
              type="submit"
              :loading="submitting"
              :disabled="!form.title.trim()"
              >Save</UButton
            >
          </div>
        </form>
      </div>
    </template>
  </NuxtLayout>
</template>
