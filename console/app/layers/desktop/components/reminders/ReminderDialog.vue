<script setup lang="ts">
import { useReminderStore } from "@shared/stores/reminder";

const props = defineProps<{
  reminder?: {
    identifier: string;
    title: string;
    description?: string | null;
    remindAt: string;
    recurring: boolean;
    recurrenceRule?: string | null;
    alarmSound?: string | null;
  } | null;
}>();

const open = defineModel<boolean>("open", { required: true });

const reminderStore = useReminderStore();
const { notify } = useAppNotification();

const editing = computed(() => !!props.reminder);
const submitting = ref(false);
const deleting = ref(false);

const form = reactive({
  title: "",
  description: "",
  remindAt: "",
  recurring: false,
  recurrenceRule: "",
  alarmSound: "",
});

function resetForm() {
  Object.assign(form, {
    title: "",
    description: "",
    remindAt: "",
    recurring: false,
    recurrenceRule: "",
    alarmSound: "",
  });
}

watch(
  () => props.reminder,
  (r) => {
    if (r) {
      form.title = r.title;
      form.description = r.description ?? "";
      form.remindAt = new Date(r.remindAt).toISOString().slice(0, 16);
      form.recurring = r.recurring;
      form.recurrenceRule = r.recurrenceRule ?? "";
      form.alarmSound = r.alarmSound ?? "";
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
  if (!form.title.trim() || !form.remindAt) return;
  submitting.value = true;
  try {
    const payload = {
      title: form.title.trim(),
      description: form.description.trim() || undefined,
      remindAt: new Date(form.remindAt).toISOString(),
      recurring: form.recurring || undefined,
      recurrenceRule: form.recurrenceRule.trim() || undefined,
      alarmSound: form.alarmSound.trim() || undefined,
    };

    if (editing.value && props.reminder) {
      await reminderStore.updateReminder(props.reminder.identifier, payload);
      notify({ type: "success", message: "Reminder updated" });
    } else {
      await reminderStore.createReminder(payload);
      notify({ type: "success", message: "Reminder created" });
    }
    open.value = false;
  } catch {
    notify({
      type: "error",
      message: editing.value
        ? "Failed to update reminder"
        : "Failed to create reminder",
    });
  } finally {
    submitting.value = false;
  }
}

async function handleDelete() {
  if (!props.reminder) return;
  deleting.value = true;
  try {
    await reminderStore.deleteReminder(props.reminder.identifier);
    notify({ type: "success", message: "Reminder deleted" });
    open.value = false;
  } catch {
    notify({ type: "error", message: "Failed to delete reminder" });
  } finally {
    deleting.value = false;
  }
}
</script>

<template>
  <RyderCreateDialog
    v-model:open="open"
    :title="editing ? 'Edit Reminder' : 'New Reminder'"
    :loading="submitting"
    :submit-label="editing ? 'Save' : 'Create'"
    :submit-disabled="!form.title.trim() || !form.remindAt"
    @submit="handleSubmit"
    @cancel="resetForm"
  >
    <div class="flex flex-col gap-4">
      <div class="flex flex-col gap-1.5">
        <label class="text-xs font-medium text-gray-500 dark:text-gray-400">
          Title <span class="text-rose-500">*</span>
        </label>
        <input
          v-model="form.title"
          type="text"
          placeholder="What should I remind you about?"
          class="almond_input_box"
        >
      </div>

      <div class="flex flex-col gap-1.5">
        <label class="text-xs font-medium text-gray-500 dark:text-gray-400">
          Description
        </label>
        <textarea
          v-model="form.description"
          placeholder="Add more details..."
          rows="3"
          class="w-full bg-white dark:bg-gray-800 rounded-lg px-4 py-2.5 text-sm text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-700 outline-none focus:ring-2 focus:ring-primary-300 dark:focus:ring-primary-600 focus:border-transparent placeholder-gray-400 dark:placeholder-gray-500 resize-none"
        />
      </div>

      <div class="flex flex-col gap-1.5">
        <label class="text-xs font-medium text-gray-500 dark:text-gray-400">
          Remind at <span class="text-rose-500">*</span>
        </label>
        <div class="relative">
          <UIcon
            name="heroicons:clock"
            class="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-gray-400 pointer-events-none"
          />
          <input
            v-model="form.remindAt"
            type="datetime-local"
            class="w-full bg-white dark:bg-gray-800 rounded-lg pl-9 pr-4 py-2.5 text-sm text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-700 outline-none focus:ring-2 focus:ring-primary-300 dark:focus:ring-primary-600 focus:border-transparent"
          >
        </div>
      </div>

      <div class="flex items-center gap-3">
        <button
          type="button"
          class="relative inline-flex h-5 w-9 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 focus:outline-none"
          :class="
            form.recurring
              ? 'bg-primary-500'
              : 'bg-gray-200 dark:bg-gray-700'
          "
          @click="form.recurring = !form.recurring"
        >
          <span
            class="pointer-events-none inline-block h-4 w-4 rounded-full bg-white shadow transform transition duration-200"
            :class="form.recurring ? 'translate-x-4' : 'translate-x-0'"
          />
        </button>
        <label
          class="text-sm text-gray-600 dark:text-gray-400 cursor-pointer select-none"
          @click="form.recurring = !form.recurring"
        >
          Recurring reminder
        </label>
      </div>

      <div v-if="form.recurring" class="flex flex-col gap-1.5">
        <label class="text-xs font-medium text-gray-500 dark:text-gray-400">
          Recurrence rule
        </label>
        <input
          v-model="form.recurrenceRule"
          type="text"
          placeholder="e.g. FREQ=DAILY;INTERVAL=1"
          class="almond_input_box"
        >
      </div>

      <button
        v-if="editing"
        type="button"
        :disabled="deleting"
        class="flex items-center gap-2 text-sm text-rose-500 hover:text-rose-600 disabled:opacity-50 disabled:cursor-not-allowed transition-colors mt-2"
        @click="handleDelete"
      >
        <UIcon name="heroicons:trash" class="size-4" />
        {{ deleting ? "Deleting..." : "Delete reminder" }}
      </button>
    </div>
  </RyderCreateDialog>
</template>
