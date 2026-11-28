<script setup lang="ts">
import { CalendarDate, type Time } from "@internationalized/date";
import { useTaskCategoryStore } from "~/stores/task-categories";
import type { TaskSubtaskDraft } from "~/stores/tasks";
import TaskDetails from "~/components/task/TaskDetails.vue";

definePageMeta({ layout: false, name: "New Task" });

const route = useRoute();
const router = useRouter();
const taskStore = useTaskStore();
const categoryStore = useTaskCategoryStore();
const toast = useToast();

const categoryManagerOpen = ref(false);

const form = reactive({
  title: "",
  description: "",
  dueDate: undefined as CalendarDate | undefined,
  priority: "medium" as "high" | "medium" | "low",
  categoryIdentifier: null as string | null,
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
  if (categoryStore.categories.length === 0) categoryStore.fetchCategories();
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
      categoryIdentifier: form.categoryIdentifier ?? undefined,
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
            v-model:category-identifier="form.categoryIdentifier"
            v-model:subtasks="subtasks"
            :categories="categoryStore.categories"
            :busy="submitting"
            :show-done="false"
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
                <label class="text-xs font-medium text-gray-500 dark:text-gray-400"
                  >Priority</label
                >
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
