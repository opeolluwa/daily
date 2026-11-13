<script setup lang="ts">
import { CalendarDate, Time } from "@internationalized/date";
import { useReminderStore } from "~/stores/reminder";

const props = defineProps<{
  reminder?: {
    identifier: string;
    title: string;
    description?: string | null;
    remindAt: string;
    endDate?: string | null;
    recurring: boolean;
    recurrenceRule?: string | null;
    alarmSound?: string | null;
  } | null;
  initialDate?: string | null;
}>();

const open = defineModel<boolean>("open", { required: true });

const reminderStore = useReminderStore();
const toast = useToast();

const editing = computed(() => !!props.reminder);
const submitting = ref(false);
const deleting = ref(false);

const recurrenceOptions = [
  { label: "Daily", value: "daily" },
  { label: "Weekly", value: "weekly" },
  { label: "Biweekly", value: "biweekly" },
  { label: "Monthly", value: "monthly" },
  { label: "Yearly", value: "yearly" },
  { label: "Weekdays (Mon–Fri)", value: "weekdays" },
];

const form = reactive({
  title: "",
  description: "",
  date: undefined as CalendarDate | undefined,
  endDate: undefined as CalendarDate | undefined,
  recurring: false,
  recurrenceType: "",
  alarmSound: "",
});

const selectedTime = shallowRef<Time | undefined>();

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

function buildRemindAt(): string {
  const dateStr = toIsoDate(form.date);
  if (!dateStr) return "";
  const timeStr = selectedTime.value
    ? formatTime(selectedTime.value)
    : "09:00";
  return new Date(`${dateStr}T${timeStr}`).toISOString();
}

function resetForm() {
  Object.assign(form, {
    title: "",
    description: "",
    date: undefined,
    endDate: undefined,
    recurring: false,
    recurrenceType: "",
    alarmSound: "",
  });
  selectedTime.value = undefined;
}

watch(
  () => props.reminder,
  (r) => {
    if (r) {
      form.title = r.title;
      form.description = r.description ?? "";
      const dt = new Date(r.remindAt);
      form.date = new CalendarDate(
        dt.getFullYear(),
        dt.getMonth() + 1,
        dt.getDate(),
      );
      selectedTime.value = new Time(dt.getHours(), dt.getMinutes());
      form.endDate = fromIsoDate(r.endDate);
      form.recurring = r.recurring;
      form.recurrenceType = r.recurrenceRule ?? "";
      form.alarmSound = r.alarmSound ?? "";
    } else {
      resetForm();
      if (props.initialDate) {
        form.date = fromIsoDate(props.initialDate);
      }
    }
  },
  { immediate: true },
);

watch(open, (val) => {
  if (!val) resetForm();
});

async function handleSubmit() {
  if (!form.title.trim() || !form.date) return;
  submitting.value = true;
  try {
    const payload = {
      title: form.title.trim(),
      description: form.description.trim() || undefined,
      remindAt: buildRemindAt(),
      endDate: toIsoDate(form.endDate) ?? null,
      recurring: form.recurring,
      recurrenceRule: form.recurring ? form.recurrenceType || null : null,
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
    :submit-disabled="!form.title.trim() || !form.date"
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

      <div class="grid grid-cols-2 gap-3">
        <RyderDateInput
          v-model="form.date"
          label="Start Date"
          :required="true"
          name="reminder-start-date"
          placeholder="Start"
        />
        <RyderDateInput
          v-model="form.endDate"
          label="End Date"
          name="reminder-end-date"
          placeholder="Optional"
        />
      </div>

      <RyderTimeInput
        v-model="selectedTime"
        label="Time"
        name="reminder-time"
        size="sm"
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

      <div v-if="form.recurring" class="flex flex-col gap-1.5">
        <label class="text-xs font-medium text-gray-500 dark:text-gray-400">
          Repeat
        </label>
        <div class="grid grid-cols-3 gap-2">
          <button
            v-for="opt in recurrenceOptions"
            :key="opt.value"
            type="button"
            class="py-2 rounded-lg text-xs font-medium border transition-colors"
            :class="
              form.recurrenceType === opt.value
                ? 'bg-primary-50 dark:bg-primary-950 border-primary-300 dark:border-primary-700 text-primary-600 dark:text-primary-400'
                : 'bg-white dark:bg-gray-800 border-gray-200 dark:border-gray-700 text-gray-500 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-700'
            "
            @click="form.recurrenceType = opt.value"
          >
            {{ opt.label }}
          </button>
        </div>
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
