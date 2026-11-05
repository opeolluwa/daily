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
const toast = useToast();

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
      toast.add({
        color: "success",
        icon: "heroicons:check-circle",
        title: "Reminder Updated",
        description: "Your changes have been saved",
      });
    } else {
      await reminderStore.createReminder(payload);
      toast.add({
        color: "success",
        icon: "heroicons:check-circle",
        title: "Reminder Created",
        description: "Your reminder has been saved",
      });
    }
    open.value = false;
  } catch {
    toast.add({
      color: "error",
      icon: "heroicons:x-circle",
      title: "Something Went Wrong",
      description: editing.value
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
    toast.add({
      color: "success",
      icon: "heroicons:check-circle",
      title: "Reminder Deleted",
      description: "The reminder has been removed",
    });
    open.value = false;
  } catch {
    toast.add({
      color: "error",
      icon: "heroicons:x-circle",
      title: "Something Went Wrong",
      description: "Failed to delete reminder",
    });
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
      <RyderInput
        v-model="form.title"
        label="Title"
        :required="true"
        placeholder="What should I remind you about?"
        name="reminder-title"
      />

      <RyderTextarea
        v-model="form.description"
        label="Description"
        name="reminder-description"
        placeholder="Add more details..."
        :rows="3"
      />

      <RyderInput
        v-model="form.remindAt"
        label="Remind at"
        :required="true"
        type="datetime-local"
        icon="heroicons:clock"
        name="remind-at"
      />

      <div class="flex items-center gap-3">
        <button
          type="button"
          class="relative inline-flex h-5 w-9 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 focus:outline-none"
          :class="
            form.recurring ? 'bg-primary-500' : 'bg-gray-200 dark:bg-gray-700'
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

      <RyderInput
        v-if="form.recurring"
        v-model="form.recurrenceRule"
        label="Recurrence rule"
        placeholder="e.g. FREQ=DAILY;INTERVAL=1"
        name="recurrence-rule"
      />

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
