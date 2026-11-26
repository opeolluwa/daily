<script setup lang="ts">
import { CalendarDate, Time } from "@internationalized/date";

definePageMeta({ layout: false, name: "Edit Task" });

const route = useRoute();
const router = useRouter();
const taskStore = useTaskStore();
const toast = useToast();

const id = computed(() => route.query.id as string | undefined);
const task = computed(
  () => taskStore.tasks.find((t) => t.identifier === id.value) ?? null,
);

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

function hydrate() {
  if (!task.value) return;
  form.title = task.value.title;
  form.description = task.value.description ?? "";
  form.dueDate = fromIsoDate(task.value.dueDate);
  form.priority = task.value.priority;
  form.done = task.value.done;
  if (task.value.dueTime) {
    const [h, m] = task.value.dueTime.split(":").map(Number);
    selectedTime.value = new Time(h, m);
  } else {
    selectedTime.value = undefined;
  }
}

watch(task, hydrate, { immediate: true });

onMounted(async () => {
  if (taskStore.tasks.length === 0) await taskStore.fetchTasks();
  hydrate();
});

async function handleSubmit() {
  if (!task.value || !form.title.trim()) return;
  submitting.value = true;
  try {
    await taskStore.updateTask(task.value.identifier, {
      title: form.title.trim(),
      description: form.description.trim() || undefined,
    });
    const current = taskStore.tasks.find(
      (t) => t.identifier === task.value!.identifier,
    );
    if (current) {
      if (current.done !== form.done)
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

          <button
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
              :name="
                form.done ? 'heroicons:check-circle-solid' : 'lucide:circle'
              "
              class="size-5"
              :class="
                form.done
                  ? 'text-emerald-500'
                  : 'text-gray-300 dark:text-gray-600'
              "
            />
            <span
              class="text-sm font-medium"
              :class="
                form.done
                  ? 'text-emerald-700 dark:text-emerald-300'
                  : 'text-gray-600 dark:text-gray-400'
              "
              >{{ form.done ? "Marked as done" : "Mark as done" }}</span
            >
          </button>

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
