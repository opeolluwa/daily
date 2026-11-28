<script setup lang="ts">
import {
  isPermissionGranted,
  requestPermission,
} from "@tauri-apps/plugin-notification";
import { useAlarmScheduler } from "~/composables/useAlarmScheduler";
import { useWorkspaceSetup } from "~/composables/useWorkspaceSetup";
import "@domternal/theme";
const { init: initFontSize } = useFontSize();
const { init: initDarkTheme } = useDarkTheme();
const { setupRequired, checkSetup, initializing } = useUserSetup();
const {
  setupRequired: workspaceSetupRequired,
  checkSetup: checkWorkspaceSetup,
  initializing: workspaceInitializing,
} = useWorkspaceSetup();

useAlarmScheduler();

onMounted(async () => {
  try {
    document.body.style.backgroundColor = "transparent";

    initFontSize();
    initDarkTheme();
    await checkSetup();
    await checkWorkspaceSetup();

    let permissionGranted = await isPermissionGranted();

    if (!permissionGranted) {
      const permission = await requestPermission();
      permissionGranted = permission === "granted";
    }
  } catch (error) {
    console.error("[app] initialization failed", error);
  }
});
</script>

<template>
  <UApp :toaster="{ progress: false, duration: 1500, position: 'top-right' }">
    <!-- Background fill for the transparent desktop window -->
    <div class="fixed inset-0 rounded-2xl bg-gray-50 dark:bg-app-dark-800" />

    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
    <WorkspaceUserSetupModal v-if="setupRequired" />
    <WorkspaceSetupModal v-if="workspaceSetupRequired" />

    <Transition
      enter-active-class="transition-opacity duration-200"
      leave-active-class="transition-opacity duration-300"
      enter-from-class="opacity-0"
      leave-to-class="opacity-0"
    >
      <AppSplashScreen v-if="initializing || workspaceInitializing" />
    </Transition>

    <!-- Window controls (traffic lights / minimize-maximize-close) -->
    <AppWindowControls />

    <!-- <AppTitlebar :authenticated="authenticated" /> -->

    <!-- Masks the dark macOS window-edge hairline; keeps the OS shadow. -->
    <div
      aria-hidden="true"
      class="pointer-events-none fixed inset-0 z-9998 rounded-2xl border border-gray-200 dark:border-white/15"
    />
  </UApp>
</template>

<style>
html,
body {
  background-color: transparent;
}
</style>
