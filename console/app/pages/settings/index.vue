<script setup lang="ts">
import { useAuthStore } from "~/stores/auth";

definePageMeta({ layout: "settings" });

const authStore = useAuthStore();
const defaultPath = computed(() =>
  authStore.isGuest ? "/settings/notifications" : "/settings/profile",
);

watchEffect(() => {
  if (authStore.isGuest) {
    navigateTo("/settings/notifications", { replace: true });
  }
});
</script>

<template>
  <div>
    <SettingsNavList class="lg:hidden" />

    <div class="hidden lg:block">
      <SettingsProfileSettings v-if="!authStore.isGuest" />
      <div v-else>
        <NuxtLink :to="defaultPath" class="text-sm text-primary-500">
          Go to settings
        </NuxtLink>
      </div>
    </div>
  </div>
</template>
