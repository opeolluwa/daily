<script setup lang="ts">
import { useTodoStore } from "@shared/stores/todo";

const props = defineProps<{
  todo?: {
    identifier: string;
    title: string;
    description?: string | null;
    dueDate?: string | null;
    time?: string | null;
    priority: "high" | "medium" | "low";
    done: boolean;
  } | null;
}>();

const open = defineModel<boolean>("open", { required: true });

const todoStore = useTodoStore();
const toast = useToast();

const editing = computed(() => !!props.todo);

const form = reactive({
  title: "",
  description: "",
  dueDate: null as Date | null,
  priority: "medium" as "high" | "medium" | "low",
  done: false,
});

const selectedTime = shallowRef<{ hour: number; minute: number } | undefined>();
const submitting = ref(false);
const deleting = ref(false);

function toIsoDate(date: Date | null): string | null {
  if (!date) return null;
  return [
    date.getFullYear(),
    String(date.getMonth() + 1).padStart(2, "0"),
    String(date.getDate()).padStart(2, "0"),
  ].join("-");
}

function formatDisplayDate(date: Date | null): string {
  if (!date) return "";
  return date.toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  });
}

function resetForm() {
  form.title = "";
  form.description = "";
  form.dueDate = null;
  form.priority = "medium";
  form.done = false;
  selectedTime.value = undefined;
}

watch(
  () => props.todo,
  (todo) => {
    if (todo) {
      form.title = todo.title;
      form.description = todo.description ?? "";
      form.dueDate = todo.dueDate ? new Date(todo.dueDate) : null;
      form.priority = todo.priority;
      form.done = todo.done;
      if (todo.time) {
        const [h, m] = todo.time.split(":").map(Number);
        selectedTime.value = { hour: h, minute: m };
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
    if (editing.value && props.todo) {
      await todoStore.updateTodo(props.todo.identifier, {
        title: form.title.trim(),
        description: form.description.trim() || undefined,
      });

      const current = todoStore.todos.find(
        (t) => t.identifier === props.todo!.identifier,
      );
      if (current) {
        if (current.done !== form.done) {
          await todoStore.toggleDone(props.todo.identifier, form.done);
        }
        if (current.priority !== form.priority) {
          await todoStore.changePriority(props.todo.identifier, form.priority);
        }
        const newDate = toIsoDate(form.dueDate);
        const existingDate = current.dueDate
          ? toIsoDate(new Date(current.dueDate))
          : null;
        if (newDate !== existingDate) {
          await todoStore.updateDueDate(props.todo.identifier, newDate);
        }
      }

      toast.add({
        color: "success",
        icon: "heroicons:check-circle",
        title: "Todo Updated",
        description: "Your changes have been saved",
      });
    } else {
      await todoStore.createTodo({
        title: form.title.trim(),
        description: form.description.trim() || undefined,
        dueDate: toIsoDate(form.dueDate) ?? undefined,
        time: selectedTime.value
          ? `${String(selectedTime.value.hour).padStart(2, "0")}:${String(selectedTime.value.minute).padStart(2, "0")}`
          : undefined,
        priority: form.priority,
      });
      toast.add({
        color: "success",
        icon: "heroicons:check-circle",
        title: "Todo Created",
        description: "Your todo has been saved",
      });
    }
    open.value = false;
  } catch {
    toast.add({
      color: "error",
      icon: "heroicons:x-circle",
      title: "Something Went Wrong",
      description: editing.value
        ? "Failed to update todo"
        : "Failed to create todo",
    });
  } finally {
    submitting.value = false;
  }
}

async function handleDelete() {
  if (!props.todo) return;
  deleting.value = true;
  try {
    await todoStore.deleteTodo(props.todo.identifier);
    toast.add({
      color: "success",
      icon: "heroicons:check-circle",
      title: "Todo Deleted",
      description: "The todo has been removed",
    });
    open.value = false;
  } catch {
    toast.add({
      color: "error",
      icon: "heroicons:x-circle",
      title: "Something Went Wrong",
      description: "Failed to delete todo",
    });
  } finally {
    deleting.value = false;
  }
}
</script>

<template>
  <RyderCreateDialog
    v-model:open="open"
    :title="editing ? 'Edit Todo' : 'New Todo'"
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
        name="todo-title"
      />

      <RyderTextarea
        v-model="form.description"
        label="Description"
        name="todo-description"
        placeholder="Add more details..."
        :rows="3"
      />

      <div class="flex flex-col gap-1.5">
        <label class="text-xs font-medium text-gray-500 dark:text-gray-400">
          Due date
        </label>
        <div class="flex items-center gap-2">
          <UPopover class="flex-1">
            <button
              type="button"
              class="w-full flex items-center gap-2 bg-white dark:bg-gray-800 rounded-lg px-4 py-2.5 text-sm border border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600 transition-colors text-left"
              :class="
                form.dueDate
                  ? 'text-gray-700 dark:text-gray-200'
                  : 'text-gray-400 dark:text-gray-500'
              "
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
            @click="form.dueDate = null"
          >
            <UIcon name="heroicons:x-mark" class="size-4" />
          </button>
        </div>
      </div>

      <div class="flex flex-col gap-1.5">
        <label class="text-xs font-medium text-gray-500 dark:text-gray-400">
          Time
        </label>
        <div class="flex items-center gap-2">
          <AppTimePicker v-model="selectedTime" class="flex-1" />
          <button
            v-if="selectedTime"
            type="button"
            class="p-2.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 transition-colors"
            @click="selectedTime = undefined"
          >
            <UIcon name="heroicons:x-mark" class="size-4" />
          </button>
        </div>
      </div>

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
        {{ deleting ? "Deleting..." : "Delete todo" }}
      </button>
    </div>
  </RyderCreateDialog>
</template>
