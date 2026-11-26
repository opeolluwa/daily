<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import { platform } from "@tauri-apps/plugin-os";
import { useAuthStore } from "~/stores/auth";
import { useUserPreferenceStore } from "~/stores/workspace-profile";

const appWindow = getCurrentWindow();
const isMacOS = platform().toLowerCase() === "macos";
const router = useRouter();
const authStore = useAuthStore();
const preferenceStore = useUserPreferenceStore();
const route = useRoute();
const isAuthPage = computed(() => route.path.startsWith("/auth"));

const userMenuItems = computed(() => [
  [
    {
      label: "Profile",
      icon: "i-lucide-user",
      onSelect: () => navigateTo("/settings?section=profile"),
    },
    {
      label: "Workspaces",
      icon: "heroicons:briefcase",
      onSelect: () => navigateTo("/settings?section=workspaces"),
    },
  ],
  [
    {
      label: authStore.isGuest ? "Exit" : "Logout",
      icon: "heroicons:arrow-right-start-on-rectangle",
      class: "text-red-500 dark:text-red-400",
      onSelect: () => {
        authStore.clearSession();
        authStore.exitGuestMode();
        navigateTo("/auth/login");
      },
    },
  ],
]);
</script>

<template>
  <div
    class="fixed top-0 left-0 right-0 h-10 z-[9999] flex items-center px-4 rounded-t-2xl"
    data-tauri-drag-region
  >
    <!-- Left: traffic lights + nav -->
    <div v-if="isMacOS" class="traffic-lights shrink-0">
      <span class="btn close" @click="appWindow.close()" />
      <span class="btn minimize" @click="appWindow.minimize()" />
      <span class="btn maximize" @click="appWindow.toggleMaximize()" />
    </div>

    <div class="inline-flex items-center shrink-0 ml-1">
      <UButton
        size="sm"
        color="neutral"
        variant="ghost"
        icon="heroicons:chevron-left"
        aria-label="Go back"
        @click="router.back()"
      />
      <UButton
        size="sm"
        color="neutral"
        variant="ghost"
        icon="heroicons:chevron-right"
        aria-label="Go forward"
        @click="router.forward()"
      />
    </div>

    <WorkspaceSelect v-if="!isAuthPage" class="ml-1" />

    <!-- Right: notifications + avatar -->
    <div v-if="!isAuthPage" class="flex items-center gap-0.5 ml-auto">
      <UTooltip text="Notifications">
        <UButton
          size="sm"
          color="neutral"
          variant="ghost"
          icon="heroicons:bell"
          aria-label="Notifications"
          @click="() => navigateTo('/notifications')"
        />
      </UTooltip>

      <UDropdownMenu
        :items="userMenuItems"
        :ui="{
          content:
            'min-w-48 rounded-lg shadow-lg ring-1 ring-gray-200 dark:ring-gray-700 bg-white dark:bg-gray-900 py-1',
          item: 'rounded-md mx-0.5 px-2.5 py-1.5 gap-2 text-[13px] transition-colors duration-100',
          separator: 'my-0.5 mx-1.5',
        }"
      >
        <UAvatar
          :src="preferenceStore.preference?.profilePicture || undefined"
          icon="i-lucide-user"
          size="sm"
          class="cursor-pointer"
        />

        <template #header>
          <div
            class="px-2.5 py-2 border-b border-gray-100 dark:border-gray-700"
          >
            <p
              class="text-[13px] font-medium text-gray-700 dark:text-gray-200 truncate"
            >
              {{ preferenceStore.fullName || "Account" }}
            </p>
          </div>
        </template>
      </UDropdownMenu>
    </div>

    <!-- Windows controls (non-macOS) -->
    <div v-if="!isMacOS" class="flex items-center shrink-0" :class="{ 'ml-auto': isAuthPage }">
      <button
        class="p-1 hover:bg-gray-200 dark:hover:bg-gray-700 rounded"
        aria-label="Minimize"
        @click="appWindow.minimize()"
      >
        <UIcon name="heroicons:minus" class="size-4" />
      </button>
      <button
        class="p-1 hover:bg-gray-200 dark:hover:bg-gray-700 rounded"
        aria-label="Maximize"
        @click="appWindow.toggleMaximize()"
      >
        <UIcon name="lucide:maximize" class="size-4" />
      </button>
      <button
        class="p-1 hover:bg-red-500 hover:text-white rounded"
        aria-label="Close"
        @click="appWindow.close()"
      >
        <UIcon name="heroicons:x-mark" class="size-4" />
      </button>
    </div>
  </div>
</template>

<style scoped>
.traffic-lights {
  display: flex;
  gap: 8px;
}

.btn {
  width: 11px;
  height: 12px;
  border-radius: 50%;
  display: inline-block;
  cursor: pointer;
}

.close {
  background: #ff5f57;
}

.minimize {
  background: #ffbd2e;
}

.maximize {
  background: #28c840;
}

.traffic-lights:hover .btn::after {
  content: "";
  display: block;
  width: 100%;
  height: 100%;
  background-size: 8px;
  background-repeat: no-repeat;
  background-position: center;
}

.close:hover::after {
  content: "✕";
  font-size: 8px;
  color: #4d0000;
  text-align: center;
}

.minimize:hover::after {
  content: "–";
  font-size: 10px;
  color: #664d00;
  text-align: center;
}

.maximize:hover::after {
  content: "+";
  font-size: 10px;
  color: #003300;
  text-align: center;
}
</style>
