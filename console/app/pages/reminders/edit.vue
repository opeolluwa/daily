<script setup lang="ts">
import { CalendarDate, Time } from "@internationalized/date";

definePageMeta({ layout: false, name: "Edit Reminder" });

const route = useRoute();
const router = useRouter();
const reminderStore = useReminderStore();
const toast = useToast();

const id = computed(() => route.query.id as string | undefined);
const reminder = computed(() => reminderStore.reminders.find((r) => r.identifier === id.value) ?? null);

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
const submitting = ref(false);
const deleting = ref(false);

function toIsoDate(date: CalendarDate | undefined): string | null {
  if (!date) return null;
  return [date.year, String(date.month).padStart(2, "0"), String(date.day).padStart(2, "0")].join("-");
}
function fromIsoDate(value: string | null | undefined): CalendarDate | undefined {
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
  const timeStr = selectedTime.value ? formatTime(selectedTime.value) : "09:00";
  return new Date(`${dateStr}T${timeStr}`).toISOString();
}

function hydrate() {
  if (!reminder.value) return;
  form.title = reminder.value.title;
  form.description = reminder.value.description ?? "";
  const dt = new Date(reminder.value.remindAt);
  form.date = new CalendarDate(dt.getFullYear(), dt.getMonth() + 1, dt.getDate());
  selectedTime.value = new Time(dt.getHours(), dt.getMinutes());
  form.endDate = fromIsoDate(reminder.value.endDate);
  form.recurring = reminder.value.recurring;
  form.recurrenceType = reminder.value.recurrenceRule ?? "";
  form.alarmSound = reminder.value.alarmSound ?? "";
}

watch(reminder, hydrate, { immediate: true });
onMounted(async () => {
  if (reminderStore.reminders.length === 0) await reminderStore.fetchReminders();
  hydrate();
});

async function handleSubmit() {
  if (!reminder.value || !form.title.trim() || !form.date) return;
  submitting.value = true;
  try {
    await reminderStore.updateReminder(reminder.value.identifier, {
      title: form.title.trim(),
      description: form.description.trim() || undefined,
      remindAt: buildRemindAt(),
      endDate: toIsoDate(form.endDate) ?? null,
      recurring: form.recurring,
      recurrenceRule: form.recurring ? form.recurrenceType || null : null,
      alarmSound: form.alarmSound.trim() || undefined,
    });
    toast.add({ color: "success", icon: "heroicons:check-circle", title: "Reminder Updated", description: "Your changes have been saved" });
    await router.push("/reminders");
  } catch {
    toast.add({ color: "error", icon: "heroicons:x-circle", title: "Something Went Wrong", description: "Failed to update reminder" });
  } finally {
    submitting.value = false;
  }
}

async function handleDelete() {
  if (!reminder.value) return;
  deleting.value = true;
  try {
    await reminderStore.deleteReminder(reminder.value.identifier);
    toast.add({ color: "success", icon: "heroicons:check-circle", title: "Reminder Deleted", description: "The reminder has been removed" });
    await router.push("/reminders");
  } catch {
    toast.add({ color: "error", icon: "heroicons:x-circle", title: "Something Went Wrong", description: "Failed to delete reminder" });
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
          <h1 class="text-lg font-semibold text-gray-800 dark:text-gray-100">Edit Reminder</h1>
        </div>

        <div v-if="!reminder" class="flex flex-col items-center justify-center py-20 text-center">
          <UIcon name="heroicons:clock" class="size-7 text-gray-400 mb-3" />
          <p class="text-sm text-gray-600 dark:text-gray-400 mb-3">Reminder not found</p>
          <UButton size="sm" variant="outline" @click="router.push('/reminders')">Back to reminders</UButton>
        </div>

        <form v-else class="flex flex-col gap-4" @submit.prevent="handleSubmit">
          <RyderInput v-model="form.title" label="Title" :required="true" placeholder="What should I remind you about?" name="reminder-title" />
          <RyderTextarea v-model="form.description" label="Description" name="reminder-description" placeholder="Add more details..." :rows="3" />
          <div class="grid grid-cols-2 gap-3">
            <RyderDateInput v-model="form.date" label="Start Date" :required="true" name="reminder-start-date" placeholder="Start" />
            <RyderDateInput v-model="form.endDate" label="End Date" name="reminder-end-date" placeholder="Optional" />
          </div>
          <RyderTimeInput v-model="selectedTime" label="Time" name="reminder-time" size="sm" />
          <div class="flex items-center gap-3">
            <button type="button" class="relative inline-flex h-5 w-9 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200" :class="form.recurring ? 'bg-primary-500' : 'bg-gray-200 dark:bg-gray-700'" @click="form.recurring = !form.recurring">
              <span class="pointer-events-none inline-block h-4 w-4 rounded-full bg-white shadow transform transition duration-200" :class="form.recurring ? 'translate-x-4' : 'translate-x-0'" />
            </button>
            <span class="text-sm text-gray-600 dark:text-gray-400">Recurring</span>
          </div>
          <RyderSelect v-if="form.recurring" v-model="form.recurrenceType" :items="recurrenceOptions" label="Recurrence" name="reminder-recurrence" placeholder="Select frequency" />
          <RyderInput v-model="form.alarmSound" label="Alarm sound (optional)" name="reminder-alarm-sound" placeholder="Default" />

          <button type="button" :disabled="deleting" class="flex items-center justify-center gap-2 py-2 px-4 rounded-lg text-sm text-rose-500 hover:bg-rose-50 dark:hover:bg-rose-950 disabled:opacity-50" @click="handleDelete">
            <UIcon name="heroicons:trash" class="size-4" />
            {{ deleting ? 'Deleting...' : 'Delete reminder' }}
          </button>

          <div class="flex justify-end gap-2 pt-2">
            <UButton type="button" color="neutral" variant="ghost" :disabled="submitting" @click="router.back()">Cancel</UButton>
            <UButton type="submit" :loading="submitting" :disabled="!form.title.trim() || !form.date">Save</UButton>
          </div>
        </form>
      </div>
    </template>
  </NuxtLayout>
</template>
