<script setup lang="ts">
import { useAuthStore } from "~/stores/auth";

definePageMeta({ layout: "settings" });

const authStore = useAuthStore();
watchEffect(() => {
  if (authStore.isGuest) {
    navigateTo("/settings/notifications", { replace: true });
  }
});
</script>

<template>
  <SettingsProfileSettings v-if="!authStore.isGuest" />
</template>
