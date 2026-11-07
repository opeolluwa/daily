<script setup lang="ts">
import ReminderForm from "@mobile/components/reminder/form.vue";
import { useReminderStore } from "@shared/stores/reminder";

definePageMeta({ name: "Edit reminder" });

const route = useRoute();
const router = useRouter();
const reminderStore = useReminderStore();
const { notify } = useAppNotification();

const id = computed(() =>
  typeof route.query.id === "string" ? route.query.id : undefined,
);

const reminder = computed(
  () =>
    reminderStore.reminders.find((entry) => entry.identifier === id.value) ??
    null,
);

function handleSaved() {
  notify({ message: "Reminder updated", type: "success" });
  router.replace("/reminders");
}

onMounted(async () => {
  if (reminderStore.reminders.length === 0) {
    await reminderStore.fetchReminders();
  }
});
</script>

<template>
  <div>
    <div v-if="reminderStore.loading && !reminder" class="flex flex-col gap-4">
      <USkeleton class="h-16 rounded-lg" />
      <USkeleton class="h-24 rounded-lg" />
      <USkeleton class="h-10 rounded-lg" />
    </div>

    <div
      v-else-if="!reminder"
      class="flex flex-col items-center justify-center py-20 text-center"
    >
      <div class="mb-4 p-3 rounded-full bg-gray-100 dark:bg-gray-800">
        <UIcon name="heroicons:clock" class="size-7 text-gray-400" />
      </div>
      <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
        Reminder not found
      </h3>
      <button
        class="text-xs text-primary-500 hover:text-primary-600 font-medium mt-2"
        @click="router.push('/reminders')"
      >
        Back to reminders
      </button>
    </div>

    <ReminderForm v-else :reminder="reminder" @saved="handleSaved" />
  </div>
</template>
