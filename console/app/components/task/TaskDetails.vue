<script setup lang="ts">
import type { CalendarDate } from "@internationalized/date";
import type { TaskCategory } from "~/stores/task-categories";
import type { TaskSubtaskDraft } from "~/stores/tasks";

const props = withDefaults(
  defineProps<{
    title: string;
    description: string;
    dueDate?: CalendarDate | null;
    categoryIdentifier?: string | null;
    categories: TaskCategory[];
    done: boolean;
    subtasks: TaskSubtaskDraft[];
    busy?: boolean;
    showDone?: boolean;
    showManageCategories?: boolean;
  }>(),
  {
    dueDate: null,
    categoryIdentifier: null,
    busy: false,
    showDone: true,
    showManageCategories: true,
  },
);

const emit = defineEmits<{
  "update:title": [value: string];
  "update:description": [value: string];
  "update:dueDate": [value: CalendarDate | undefined];
  "update:categoryIdentifier": [value: string | null];
  "update:done": [value: boolean];
  "update:subtasks": [value: TaskSubtaskDraft[]];
  manageCategories: [];
}>();

function onTitleInput(event: Event) {
  emit("update:title", (event.target as HTMLInputElement).value);
}

function onDescriptionInput(event: Event) {
  emit("update:description", (event.target as HTMLTextAreaElement).value);
}

function onDoneChange(event: Event) {
  emit("update:done", (event.target as HTMLInputElement).checked);
}

function onSubtaskTitleInput(event: Event, index: number) {
  const title = (event.target as HTMLInputElement).value;
  emit(
    "update:subtasks",
    props.subtasks.map((subtask, i) =>
      i === index ? { ...subtask, title } : subtask,
    ),
  );
}

function onSubtaskDoneChange(event: Event, index: number) {
  const done = (event.target as HTMLInputElement).checked;
  emit(
    "update:subtasks",
    props.subtasks.map((subtask, i) =>
      i === index ? { ...subtask, done } : subtask,
    ),
  );
}

function addSubtask() {
  emit("update:subtasks", [
    ...props.subtasks,
    {
      identifier: null,
      title: "",
      done: false,
      sortOrder: props.subtasks.length,
    },
  ]);
}

function removeSubtask(index: number) {
  emit(
    "update:subtasks",
    props.subtasks
      .filter((_, i) => i !== index)
      .map((subtask, i) => ({ ...subtask, sortOrder: i })),
  );
}

function selectCategory(identifier: string | null) {
  emit("update:categoryIdentifier", identifier);
}
</script>

<template>
  <div class="bg-white text-gray-800 dark:bg-gray-900 dark:text-gray-100">
    <div class="px-7 pt-7">
      <div class="flex items-start gap-3">
        <input
          v-if="props.showDone"
          type="checkbox"
          :checked="props.done"
          :disabled="props.busy"
          class="mt-1.5 size-5 shrink-0 rounded-md border-gray-300 text-[#42b883] focus:ring-[#42b883] dark:border-gray-600"
          @change="onDoneChange"
        >
        <input
          :value="props.title"
          :disabled="props.busy"
          placeholder="Task title"
          class="min-w-0 flex-1 bg-transparent text-xl font-medium tracking-tight outline-none placeholder:text-gray-300 dark:placeholder:text-gray-600"
          @input="onTitleInput"
        >
      </div>

      <div
        class="mt-6 rounded-2xl bg-gray-50/90 px-5 py-4 dark:bg-gray-800/60"
      >
        <div class="flex items-center justify-between gap-4">
          <span class="text-xs font-medium text-gray-500 dark:text-gray-400">
            Category
          </span>
          <div class="flex min-w-0 flex-wrap items-center justify-end gap-1.5">
            <button
              type="button"
              class="inline-flex items-center gap-1 rounded-full border px-2.5 py-1 text-xs transition-colors"
              :class="
                !props.categoryIdentifier
                  ? 'border-primary-300 bg-primary-50 text-primary-600 dark:border-primary-700 dark:bg-primary-950 dark:text-primary-400'
                  : 'border-transparent text-gray-500 hover:bg-white dark:text-gray-400 dark:hover:bg-gray-700'
              "
              :disabled="props.busy"
              @click="selectCategory(null)"
            >
              None
            </button>
            <button
              v-for="category in props.categories"
              :key="category.identifier"
              type="button"
              class="inline-flex items-center gap-1 rounded-full border px-2.5 py-1 text-xs transition-colors"
              :class="
                props.categoryIdentifier === category.identifier
                  ? 'border-primary-300 bg-primary-50 text-primary-600 dark:border-primary-700 dark:bg-primary-950 dark:text-primary-400'
                  : 'border-transparent text-gray-500 hover:bg-white dark:text-gray-400 dark:hover:bg-gray-700'
              "
              :disabled="props.busy"
              @click="selectCategory(category.identifier)"
            >
              <span
                class="size-2 rounded-full"
                :style="{ backgroundColor: category.color }"
              />
              {{ category.name }}
            </button>
            <button
              v-if="props.showManageCategories"
              type="button"
              class="text-[10px] text-primary-500 hover:text-primary-600 dark:text-primary-400"
              @click="emit('manageCategories')"
            >
              Manage
            </button>
          </div>
        </div>

        <div class="mt-3 flex items-center justify-between gap-4">
          <span class="text-xs font-medium text-gray-500 dark:text-gray-400">
            Due date
          </span>
          <RyderDateInput
            :model-value="props.dueDate ?? undefined"
            placeholder="Pick a date"
            size="sm"
            @update:model-value="emit('update:dueDate', $event)"
          />
        </div>

        <div v-if="$slots.extras" class="mt-4">
          <slot name="extras" />
        </div>
      </div>

      <textarea
        :value="props.description"
        :disabled="props.busy"
        rows="4"
        placeholder="Write a note…"
        class="mt-6 w-full resize-none rounded-2xl bg-[#fffaf0] px-5 py-4 text-sm leading-relaxed text-gray-700 outline-none ring-1 ring-[#f3e5c7] placeholder:text-[#c8b48b] focus:ring-[#e0c994] disabled:opacity-60 dark:text-gray-800"
        @input="onDescriptionInput"
      />

      <div class="mt-7">
        <div class="mb-3 flex items-center justify-between">
          <span class="text-xs font-semibold tracking-wide text-gray-500 uppercase dark:text-gray-400">
            Subtasks
          </span>
          <span
            class="rounded-full bg-gray-100 px-2 py-0.5 font-mono text-[10px] text-gray-500 dark:bg-gray-800 dark:text-gray-400"
          >
            {{ props.subtasks.length }}
          </span>
        </div>

        <div class="flex flex-col gap-2">
          <div
            v-for="(subtask, index) in props.subtasks"
            :key="subtask.identifier ?? `new-${index}`"
            class="group flex items-center gap-3 rounded-lg px-1.5 py-1 hover:bg-gray-50 dark:hover:bg-gray-800/60"
          >
            <input
              type="checkbox"
              :checked="subtask.done"
              :disabled="props.busy"
              class="size-4 shrink-0 rounded border-gray-300 text-[#42b883] focus:ring-[#42b883] dark:border-gray-600"
              @change="onSubtaskDoneChange($event, index)"
            >
            <input
              :value="subtask.title"
              :disabled="props.busy"
              placeholder="Subtask title"
              class="min-w-0 flex-1 bg-transparent text-sm outline-none placeholder:text-gray-300 dark:placeholder:text-gray-600"
              :class="
                subtask.done
                  ? 'text-gray-400 line-through dark:text-gray-500'
                  : 'text-gray-700 dark:text-gray-200'
              "
              @input="onSubtaskTitleInput($event, index)"
            >
            <button
              type="button"
              class="opacity-0 transition-opacity group-hover:opacity-100 text-gray-400 hover:text-rose-500 disabled:cursor-not-allowed"
              :disabled="props.busy"
              @click="removeSubtask(index)"
            >
              <UIcon name="heroicons:x-mark" class="size-4" />
            </button>
          </div>
        </div>

        <button
          type="button"
          class="mt-3 flex items-center gap-1.5 rounded-lg py-2 text-xs font-medium text-gray-500 transition-colors hover:text-primary-600 disabled:cursor-not-allowed dark:text-gray-400"
          :disabled="props.busy"
          @click="addSubtask"
        >
          <UIcon name="heroicons:plus" class="size-3.5" />
          Add a subtask
        </button>
      </div>
    </div>
  </div>
</template>
