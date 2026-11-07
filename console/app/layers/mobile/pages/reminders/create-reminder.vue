<script setup lang="ts">
import ReminderForm from "@mobile/components/reminder/form.vue";

definePageMeta({ name: "New reminder" });

const router = useRouter();
const { notify } = useAppNotification();

function handleSaved() {
  notify({ message: "Reminder created", type: "success" });
  router.replace("/reminders");
}
</script>

<template>
  <ReminderForm @saved="handleSaved" />
</template>
