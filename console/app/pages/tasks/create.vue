<script setup lang="ts">
import { CalendarDate, type Time } from "@internationalized/date";
import { useTaskCategoryStore } from "~/stores/task-categories";

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
    await taskStore.createTask({
      title: form.title.trim(),
      description: form.description.trim() || undefined,
      dueDate: toIsoDate(form.dueDate) ?? undefined,
      time: selectedTime.value ? formatTime(selectedTime.value) : undefined,
      priority: form.priority,
      categoryIdentifier: form.categoryIdentifier ?? undefined,
    });
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

          <div class="flex flex-col gap-1.5">
            <div class="flex items-center justify-between">
              <label class="text-xs font-medium text-gray-500 dark:text-gray-400">Category</label>
              <button type="button" class="text-[10px] text-primary-500 hover:text-primary-600 dark:text-primary-400" @click="categoryManagerOpen = true">Manage</button>
            </div>
            <div class="flex flex-wrap gap-1.5">
              <button type="button" class="inline-flex items-center gap-1 rounded-full px-2.5 py-1 text-xs border transition-colors" :class="!form.categoryIdentifier ? 'border-primary-300 bg-primary-50 text-primary-600 dark:border-primary-700 dark:bg-primary-950 dark:text-primary-400' : 'border-gray-200 text-gray-500 hover:bg-gray-50 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-800'" @click="form.categoryIdentifier = null">None</button>
              <button v-for="cat in categoryStore.categories" :key="cat.identifier" type="button" class="inline-flex items-center gap-1 rounded-full px-2.5 py-1 text-xs border transition-colors" :class="form.categoryIdentifier === cat.identifier ? 'border-primary-300 bg-primary-50 text-primary-600 dark:border-primary-700 dark:bg-primary-950 dark:text-primary-400' : 'border-gray-200 text-gray-500 hover:bg-gray-50 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-800'" @click="form.categoryIdentifier = cat.identifier">
                <span class="size-2 rounded-full" :style="{ backgroundColor: cat.color }" />
                {{ cat.name }}
              </button>
            </div>
          </div>

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
