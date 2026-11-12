<script setup lang="ts">
import { useAuthStore } from "~/stores/auth";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { platform } from "@tauri-apps/plugin-os";
import { computed, useTemplateRef } from "vue";
import { IS_WEB } from "~/env";
import { useUserPreferenceStore } from "~/stores/workspace-profile";

const iconSize = "md";
const authStore = useAuthStore();
const router = useRouter();

const { searchQuery, isOpen } = useAppSearch();
const appWindow = getCurrentWindow();
const searchContainerRef = useTemplateRef<HTMLElement>("searchContainerRef");
const globalSearchRef = useTemplateRef<{
  handleKeydown: (e: KeyboardEvent) => void;
}>("globalSearchRef");
const currentPlatform = platform();
const { toggleMobileNav } = useMobileNav();

const colorMode = useColorMode();
const themeLabel = computed(() =>
  colorMode.value === "dark" ? "Light mode" : "Dark mode",
);
const isMacOS = currentPlatform.toLowerCase() === "macos";

const preferenceStore = useUserPreferenceStore();
const route = useRoute();
const authPagesPrefix = "/auth";
const isAuthPage = computed(() => route.path.startsWith(authPagesPrefix));
const userMenuItems = computed(() => [
  [
    {
      label: "Profile",
      icon: "i-lucide-user",
      kbds: ["g", "p"],
      onSelect: () => navigateTo("/settings?section=profile"),
    },
    {
      label: "Workspaces",
      icon: "heroicons:briefcase",
      kbds: ["g", "w"],
      onSelect: () => navigateTo("/settings?section=workspaces"),
    },
  ],
  [
    {
      label: authStore.isGuest ? "Exit" : "Logout",
      icon: "heroicons:arrow-right-start-on-rectangle",
      class: "text-red-500 dark:text-red-400",
      kbds: ["g", "l"],
      onSelect: () => {
        authStore.clearSession();
        authStore.exitGuestMode();
        navigateTo("/auth/login");
      },
    },
  ],
]);

const search = computed<string | number | null | undefined>({
  get: () => searchQuery.value,
  set: (value) => {
    const next = value == null ? "" : String(value);
    searchQuery.value = next;
    isOpen.value = next.trim().length > 0;
  },
});

function focusSearch() {
  isOpen.value = searchQuery.value.trim().length > 0;
  const input = searchContainerRef.value?.querySelector("input");
  input?.focus();
  input?.select();
}

function clearSearch() {
  search.value = "";
}

function resetSearch() {
  clearSearch();
  focusSearch();
}

function onSearchKeydown(event: KeyboardEvent) {
  if (event.key === "Escape") {
    event.preventDefault();
    isOpen.value = false;
    searchContainerRef.value?.querySelector("input")?.blur();
    return;
  }
  globalSearchRef.value?.handleKeydown(event);
}

const shortcuts = computed(() => ({
  meta_k: { handler: focusSearch, usingInput: true },
  meta_f: { handler: focusSearch, usingInput: true },
  "meta_[": { handler: () => router.back(), usingInput: true },
  "meta_]": { handler: () => router.forward(), usingInput: true },
  ...extractShortcuts(userMenuItems.value, "-"),
}));

defineShortcuts(shortcuts);
</script>

<template>
  <div
    class="titlebar flex items-center gap-1.5 px-2 h-10"
    data-tauri-drag-region
    :class="{ 'rounded-t-2xl': !IS_WEB }"
  >
    <!-- mobile nav toggle -->
    <div v-if="!isAuthPage">
      <UButton
        :size="iconSize"
        color="neutral"
        variant="ghost"
        class="md:hidden"
        icon="heroicons:bars-3"
        aria-label="Open menu"
        @click="toggleMobileNav"
      />
    </div>

    <!-- mac os controls-->
    <div v-if="isMacOS && !IS_WEB" class="traffic-lights shrink-0">
      <span class="btn close" @click="appWindow.close()" />
      <span class="btn minimize" @click="appWindow.minimize()" />
      <span class="btn maximize" @click="appWindow.toggleMaximize()" />
    </div>

    <!-- Windows controls -->
    <div v-else-if="!isMacOS && IS_WEB" class="flex items-center shrink-0">
      <UTooltip text="Minimize">
        <UButton
          :size="iconSize"
          color="neutral"
          variant="ghost"
          icon="heroicons:minus"
          aria-label="Minimize"
          @click="appWindow.minimize()"
        />
      </UTooltip>

      <!-- web control/mobile -->

      <UTooltip text="Maximize">
        <UButton
          :size="iconSize"
          color="neutral"
          variant="ghost"
          icon="lucide:maximize"
          aria-label="Maximize"
          @click="appWindow.maximize()"
        />
      </UTooltip>

      <UTooltip text="Close">
        <UButton
          :size="iconSize"
          color="neutral"
          variant="ghost"
          icon="heroicons:x-mark"
          aria-label="Close"
          @click="appWindow.close()"
        />
      </UTooltip>
    </div>

    <div class="inline-flex items-center shrink-0">
      <UButton
        :size="iconSize"
        color="neutral"
        variant="ghost"
        class="cursor-pointer hidden md:block"
        icon="heroicons:chevron-left"
        aria-label="Go back"
        @click="router.back()"
      />
      <UButton
        :size="iconSize"
        color="neutral"
        variant="ghost"
        class="cursor-pointer hidden md:block"
        icon="heroicons:chevron-right"
        aria-label="Go forward"
        @click="router.forward()"
      />
    </div>

    <WorkspaceSelect
      v-if="!isAuthPage"
      class="cursor-pointer hidden md:block"
    />

    <!-- Search -->
    <div
      v-if="!isAuthPage"
      ref="searchContainerRef"
      class="relative flex flex-1 min-w-0 items-center md:mx-auto md:max-w-sm"
    >
      <UInput
        v-model="search"
        size="sm"
        icon="heroicons:magnifying-glass"
        placeholder="Search..."
        autocapitalize="off"
        autocorrect="off"
        spellcheck="false"
        :ui="{ base: 'ps-8 pe-10 py-3 bg-gray-50/30 dark:bg-transparent' }"
        class="w-full cursor-text"
        @keydown="onSearchKeydown"
      >
        <template #trailing>
          <UButton
            v-if="searchQuery"
            size="xs"
            color="neutral"
            variant="ghost"
            icon="heroicons:x-mark"
            class="cursor-pointer"
            aria-label="Clear search"
            @click="resetSearch"
          />
          <div v-else class="hidden items-center gap-0.5 sm:flex">
            <UKbd value="meta" size="xs" />
            <UKbd value="k" size="xs" />
          </div>
        </template>
      </UInput>

      <AppGlobalSearch
        v-if="isOpen"
        ref="globalSearchRef"
        @close="clearSearch"
      />
    </div>

    <!-- Right actions -->
    <div class="flex items-center gap-0.5 ml-auto">
      <UTooltip :text="themeLabel">
        <UColorModeButton :size="iconSize" />
      </UTooltip>

      <UTooltip v-if="!isAuthPage" text="Notifications">
        <UButton
          :size="iconSize"
          color="neutral"
          class="cursor-pointer"
          variant="ghost"
          icon="heroicons:bell"
          aria-label="Notifications"
          @click="() => navigateTo('/notifications')"
        />
      </UTooltip>

      <div v-if="!isAuthPage" class="items-center gap-0.5 flex">
        <UDropdownMenu
          :items="userMenuItems"
          :ui="{
            content:
              'min-w-48 rounded-lg shadow-lg ring-1 ring-gray-200 dark:ring-gray-700 bg-white dark:bg-gray-900 py-1',
            item: 'rounded-md mx-0.5 px-2.5 py-1.5 gap-2 text-[13px] transition-colors duration-100',
            separator: 'my-0.5 mx-1.5',
          }"
        >
          <UAvatar icon="i-lucide-user" size="sm" class="cursor-pointer" />

          <template #header>
            <div
              class="px-2.5 py-2 border-b border-gray-100 dark:border-gray-700"
            >
              <p
                class="text-[13px] font-medium text-gray-700 dark:text-gray-200 truncate"
              >
                {{
                  authStore.isGuest
                    ? "Guest mode"
                    : preferenceStore.fullName || "Account"
                }}
              </p>
            </div>
          </template>
        </UDropdownMenu>
      </div>
    </div>
  </div>
</template>

<style scoped>
.titlebar > * {
  cursor: pointer;
  border: none;
  outline: none;
  box-shadow: none;
}

.traffic-lights {
  display: flex;
  gap: 8px;
  padding: 8px;
}

.btn {
  width: 11px;
  height: 12px;
  border-radius: 50%;
  display: inline-block;
  cursor: pointer;
}

/* Colors */
.close {
  background: #ff5f57;
}

.minimize {
  background: #ffbd2e;
}

.maximize {
  background: #28c840;
}

/* Optional hover icons */
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
