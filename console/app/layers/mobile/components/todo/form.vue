<script setup lang="ts">
import { useTodoStore, type Todo } from "@shared/stores/todo";

const props = defineProps<{
  todo?: Todo | null;
}>();

const emit = defineEmits<{
  saved: [];
}>();

const todoStore = useTodoStore();

const priorityOptions = [
  { label: "Low", value: "low" },
  { label: "Medium", value: "medium" },
  { label: "High", value: "high" },
];

const editing = computed(() => !!props.todo);

const form = reactive({
  title: "",
  description: "",
  dueDate: null as Date | null,
  priority: "medium" as "high" | "medium" | "low",
});
const selectedTime = shallowRef<{ hour: number; minute: number }>();
const errors = reactive({ title: "" });
const loading = ref(false);
const submitError = ref("");

function toIsoDate(date: Date | null): string | null {
  if (!date) return null;
  return [
    date.getFullYear(),
    String(date.getMonth() + 1).padStart(2, "0"),
    String(date.getDate()).padStart(2, "0"),
  ].join("-");
}

function fromIsoDate(value: string | null): Date | null {
  if (!value) return null;
  const [year, month, day] = value.split("-").map(Number);
  if (!year || !month || !day) return null;
  return new Date(year, month - 1, day);
}

function formatDisplayDate(date: Date | null): string {
  if (!date) return "";
  return date.toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  });
}

function formatTime(value: { hour: number; minute: number }): string {
  return `${String(value.hour).padStart(2, "0")}:${String(value.minute).padStart(2, "0")}`;
}

watch(
  () => props.todo,
  (todo) => {
    if (!todo) return;
    form.title = todo.title;
    form.description = todo.description ?? "";
    form.dueDate = fromIsoDate(todo.dueDate);
    form.priority = todo.priority;
    errors.title = "";
    submitError.value = "";
  },
  { immediate: true },
);

async function handleSubmit() {
  errors.title = form.title.trim() ? "" : "Title is required";
  if (errors.title) return;

  loading.value = true;
  submitError.value = "";
  try {
    if (editing.value && props.todo) {
      const current = props.todo;
      await todoStore.updateTodo(current.identifier, {
        title: form.title.trim(),
        description: form.description.trim(),
      });

      if (form.priority !== current.priority) {
        await todoStore.changePriority(current.identifier, form.priority);
      }

      const dueDate = toIsoDate(form.dueDate);
      if (dueDate !== current.dueDate) {
        await todoStore.updateDueDate(current.identifier, dueDate);
      }
    } else {
      await todoStore.createTodo({
        title: form.title.trim(),
        description: form.description.trim() || undefined,
        dueDate: toIsoDate(form.dueDate) ?? undefined,
        time: selectedTime.value ? formatTime(selectedTime.value) : undefined,
        priority: form.priority,
      });
    }

    emit("saved");
  } catch (e) {
    console.error(e);
    submitError.value = editing.value
      ? "Failed to update task. Please try again."
      : "Failed to create task. Please try again.";
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <form class="flex flex-col gap-4" @submit.prevent="handleSubmit">
    <p class="text-sm text-gray-500 dark:text-gray-400">
      {{
        editing
          ? "Update the details of this task."
          : "Tasks help you keep track of what needs to get done."
      }}
    </p>

    <div class="flex flex-col gap-4">
      <div>
        <RyderInput
          v-model="form.title"
          label="Title"
          type="text"
          name="todo-title"
          placeholder="What needs to be done?"
          :disabled="loading"
        />
        <p v-if="errors.title" class="mt-1 text-xs text-red-500">
          {{ errors.title }}
        </p>
      </div>

      <RyderTextarea
        v-model="form.description"
        label="Description"
        name="todo-description"
        placeholder="Add more details..."
        :rows="3"
        :disabled="loading"
      />

      <div class="flex flex-col gap-1.5">
        <label class="text-xs font-medium text-gray-600 dark:text-gray-400">
          Due date
        </label>
        <div class="flex items-center gap-2">
          <UPopover class="flex-1">
            <button
              type="button"
              class="w-full flex items-center gap-2 bg-white dark:bg-gray-800 rounded-lg px-4 py-2.5 text-sm border border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600 transition-colors text-left disabled:opacity-50"
              :class="
                form.dueDate
                  ? 'text-gray-700 dark:text-gray-200'
                  : 'text-gray-400 dark:text-gray-500'
              "
              :disabled="loading"
            >
              <UIcon
                name="heroicons:calendar"
                class="size-4 shrink-0 text-gray-400"
              />
              {{
                form.dueDate ? formatDisplayDate(form.dueDate) : "Pick a date"
              }}
            </button>
            <template #content="{ close }">
              <AppDatePicker
                v-model="form.dueDate"
                @update:model-value="close"
              />
            </template>
          </UPopover>
          <button
            v-if="form.dueDate"
            type="button"
            class="p-2.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 transition-colors"
            aria-label="Clear due date"
            @click="form.dueDate = null"
          >
            <UIcon name="heroicons:x-mark" class="size-4" />
          </button>
        </div>
      </div>

      <div v-if="!editing" class="flex flex-col gap-1.5">
        <label class="text-xs font-medium text-gray-600 dark:text-gray-400">
          Time
        </label>
        <div class="flex items-center gap-2">
          <AppTimePicker
            v-model="selectedTime"
            class="flex-1"
            :disabled="loading"
          />
          <button
            v-if="selectedTime"
            type="button"
            class="p-2.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 transition-colors"
            aria-label="Clear time"
            @click="selectedTime = undefined"
          >
            <UIcon name="heroicons:x-mark" class="size-4" />
          </button>
        </div>
      </div>

      <div class="flex flex-col gap-1.5">
        <label class="text-xs font-medium text-gray-600 dark:text-gray-400">
          Priority
        </label>
        <URadioGroup
          v-model="form.priority"
          :items="priorityOptions"
          variant="card"
          orientation="horizontal"
          size="sm"
          :disabled="loading"
        />
      </div>

      <p v-if="submitError" class="text-sm text-red-500">
        {{ submitError }}
      </p>
    </div>

    <RyderButton
      type="submit"
      size="md"
      block
      :loading="loading"
      :disabled="loading"
    >
      {{ editing ? "Save changes" : "Create task" }}
    </RyderButton>
  </form>
</template>
