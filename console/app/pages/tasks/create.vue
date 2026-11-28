<script setup lang="ts">
import { CalendarDate, type Time } from "@internationalized/date";
import type { TaskSubtaskDraft } from "~/stores/tasks";
import TaskDetails from "~/components/task/TaskDetails.vue";

definePageMeta({ layout: false, name: "New Task" });

const route = useRoute();
const router = useRouter();
const taskStore = useTaskStore();
const toast = useToast();

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
});

const selectedTime = shallowRef<Time | undefined>();
const subtasks = ref<TaskSubtaskDraft[]>([]);
const submitting = ref(false);

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

onMounted(() => {
  const initialDate = route.query.date as string | undefined;
  if (initialDate) form.dueDate = fromIsoDate(initialDate);
});

async function handleSubmit() {
  if (!form.title.trim()) return;
  submitting.value = true;
  try {
    const created = await taskStore.createTask({
      title: form.title.trim(),
      description: form.description.trim() || null,
      dueDate: toIsoDate(form.dueDate) ?? undefined,
      time: selectedTime.value ? formatTime(selectedTime.value) : undefined,
      priority: form.priority,
    });
    await taskStore.saveSubtasks(created.identifier, subtasks.value);
    toast.add({
      color: "success",
      icon: "heroicons:check-circle",
      title: "Task Created",
      description: "Your task has been saved",
    });
    await router.push("/tasks");
  } catch {
    toast.add({
      color: "error",
      icon: "heroicons:x-circle",
      title: "Something Went Wrong",
      description: "Failed to create task",
    });
  } finally {
    submitting.value = false;
  }
}
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <div class="max-w-3xl">
        <div class="mb-6 flex items-center gap-3">
          <h1 class="text-lg font-semibold text-gray-800 dark:text-gray-100">
            New Task
          </h1>
        </div>

        <form class="flex flex-col gap-4" @submit.prevent="handleSubmit">
          <TaskDetails
            v-model:title="form.title"
            v-model:description="form.description"
            v-model:due-date="form.dueDate"
            v-model:subtasks="subtasks"
            :busy="submitting"
            :show-done="false"
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
              >Create</UButton
            >
          </div>
        </form>
      </div>
    </template>
  </NuxtLayout>
</template>
