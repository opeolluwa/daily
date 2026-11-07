<script setup lang="ts">
import type { Reminder } from "@shared/stores/reminder";
import { useReminderStore } from "@shared/stores/reminder";

const props = defineProps<{
  reminder?: Reminder | null;
}>();

const emit = defineEmits<{
  saved: [];
}>();

const reminderStore = useReminderStore();

const editing = computed(() => !!props.reminder);

const form = reactive({
  title: "",
  description: "",
  recurring: false,
  recurrenceRule: "",
});
const remindDate = ref<Date | null>(null);
const remindTime = ref<{ hour: number; minute: number } | null>(null);

const errors = reactive({ title: "", remindAt: "" });
const loading = ref(false);
const submitError = ref("");

function formatDisplayDate(date: Date | null): string {
  if (!date) return "";
  return date.toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  });
}

function buildRemindAt(): string | null {
  if (!remindDate.value || !remindTime.value) return null;
  const date = new Date(remindDate.value);
  date.setHours(remindTime.value.hour, remindTime.value.minute, 0, 0);
  return date.toISOString();
}

function resetForm() {
  form.title = "";
  form.description = "";
  form.recurring = false;
  form.recurrenceRule = "";
  remindDate.value = null;
  remindTime.value = null;
  errors.title = "";
  errors.remindAt = "";
  submitError.value = "";
}

watch(
  () => props.reminder,
  (reminder) => {
    if (!reminder) {
      resetForm();
      return;
    }

    const remindAt = new Date(reminder.remindAt);
    form.title = reminder.title;
    form.description = reminder.description ?? "";
    form.recurring = reminder.recurring;
    form.recurrenceRule = reminder.recurrenceRule ?? "";
    remindDate.value = remindAt;
    remindTime.value = {
      hour: remindAt.getHours(),
      minute: remindAt.getMinutes(),
    };
    errors.title = "";
    errors.remindAt = "";
    submitError.value = "";
  },
  { immediate: true },
);

async function handleSubmit() {
  errors.title = form.title.trim() ? "" : "Title is required";

  const remindAt = buildRemindAt();
  errors.remindAt = remindAt ? "" : "Pick a date and time";

  if (errors.title || errors.remindAt || !remindAt) return;

  loading.value = true;
  submitError.value = "";
  try {
    const payload = {
      title: form.title.trim(),
      description: form.description.trim() || null,
      remindAt,
      recurring: form.recurring,
      recurrenceRule: form.recurring
        ? form.recurrenceRule.trim() || null
        : null,
    };

    if (editing.value && props.reminder) {
      await reminderStore.updateReminder(props.reminder.identifier, payload);
    } else {
      await reminderStore.createReminder(payload);
    }

    emit("saved");
  } catch (e) {
    console.error(e);
    submitError.value = editing.value
      ? "Failed to update reminder. Please try again."
      : "Failed to create reminder. Please try again.";
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
          ? "Update the details of this reminder."
          : "Set a reminder so you don't forget."
      }}
    </p>

    <div class="flex flex-col gap-4">
      <div>
        <RyderInput
          v-model="form.title"
          label="Title"
          type="text"
          name="reminder-title"
          placeholder="What should I remind you about?"
          :disabled="loading"
        />
        <p v-if="errors.title" class="mt-1 text-xs text-red-500">
          {{ errors.title }}
        </p>
      </div>

      <RyderTextarea
        v-model="form.description"
        label="Description"
        name="reminder-description"
        placeholder="Add more details..."
        :rows="3"
        :disabled="loading"
      />

      <div class="flex flex-col gap-1.5">
        <label class="text-xs font-medium text-gray-600 dark:text-gray-400">
          Remind at
        </label>
        <div class="flex items-center gap-2">
          <UPopover class="flex-1">
            <button
              type="button"
              class="w-full flex items-center gap-2 bg-white dark:bg-gray-800 rounded-lg px-4 py-2.5 text-sm border border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600 transition-colors text-left disabled:opacity-50"
              :class="
                remindDate
                  ? 'text-gray-700 dark:text-gray-200'
                  : 'text-gray-400 dark:text-gray-500'
              "
              :disabled="loading"
            >
              <UIcon
                name="heroicons:calendar"
                class="size-4 shrink-0 text-gray-400"
              />
              {{ remindDate ? formatDisplayDate(remindDate) : "Pick a date" }}
            </button>
            <template #content="{ close }">
              <AppDatePicker v-model="remindDate" @update:model-value="close" />
            </template>
          </UPopover>
          <button
            v-if="remindDate"
            type="button"
            class="p-2.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 transition-colors"
            aria-label="Clear date"
            @click="remindDate = null"
          >
            <UIcon name="heroicons:x-mark" class="size-4" />
          </button>
        </div>

        <div class="flex items-center gap-2">
          <AppTimePicker
            v-model="remindTime"
            class="flex-1"
            :disabled="loading"
          />
          <button
            v-if="remindTime"
            type="button"
            class="p-2.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 transition-colors"
            aria-label="Clear time"
            @click="remindTime = null"
          >
            <UIcon name="heroicons:x-mark" class="size-4" />
          </button>
        </div>
        <p v-if="errors.remindAt" class="text-xs text-red-500">
          {{ errors.remindAt }}
        </p>
      </div>

      <label class="flex items-center justify-between py-1">
        <span class="text-sm text-gray-700 dark:text-gray-300">
          Recurring reminder
        </span>
        <USwitch v-model="form.recurring" :disabled="loading" />
      </label>

      <RyderInput
        v-if="form.recurring"
        v-model="form.recurrenceRule"
        label="Recurrence rule"
        name="recurrence-rule"
        placeholder="e.g. FREQ=DAILY;INTERVAL=1"
        :disabled="loading"
      />

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
        {{ editing ? "Save changes" : "Create reminder" }}
      </RyderButton>
  </form>
</template>
