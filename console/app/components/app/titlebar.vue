<script setup lang="ts">
import { useAuthStore } from "~/stores/auth";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { platform } from "@tauri-apps/plugin-os";
import { computed, useTemplateRef } from "vue";
import { IS_WEB } from "~/env";
import { useUserPreferenceStore } from "~/stores/workspace-profile";
import { syncNow, syncStatus } from "~/utils/sync/engine";

const iconSize = "md";
const authStore = useAuthStore();
const router = useRouter();

const syncLabel = computed(() => {
  if (syncStatus.error) return `Sync failed: ${syncStatus.error}`;
  if (syncStatus.syncing) return "Syncing…";
  if (syncStatus.pending > 0) return `${syncStatus.pending} change(s) pending`;
  if (syncStatus.lastSyncedAt)
    return `Synced ${new Date(syncStatus.lastSyncedAt).toLocaleTimeString()}`;
  return "Sync";
});

const { searchQuery, isOpen } = useAppSearch();
const appWindow = getCurrentWindow();
const searchContainerRef = useTemplateRef<HTMLElement>("searchContainerRef");
const globalSearchRef = useTemplateRef<{
  handleKeydown: (e: KeyboardEvent) => void;
}>("globalSearchRef");
const currentPlatform = platform();
const { toggleMobileNav } = useMobileNav();
const mobileSearchOpen = ref(false);

const colorMode = useColorMode();
const themeLabel = computed(() =>
  colorMode.value === "dark" ? "Light mode" : "Dark mode",
);
const isMacOS = currentPlatform.toLowerCase() === "macos";

const preferenceStore = useUserPreferenceStore();
const route = useRoute();
const isMobile = useIsMobile();
const authPagesPrefix = "/auth";
const isAuthPage = computed(() => route.path.startsWith(authPagesPrefix));
const hideTitlebar = computed(() => IS_WEB && isAuthPage.value && isMobile.value);
const parentPath = computed(() => {
  const segments = route.path.split("/").filter(Boolean);
  return segments.length > 1 ? `/${segments[0]}` : null;
});
function handleMobileNavClick() {
  if (parentPath.value) {
    navigateTo(parentPath.value);
    return;
  }
  toggleMobileNav();
}
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

const search = computed<string | number  | undefined>({
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

async function openMobileSearch() {
  mobileSearchOpen.value = true;
  await nextTick();
  focusSearch();
}

function closeMobileSearch() {
  mobileSearchOpen.value = false;
  clearSearch();
  searchContainerRef.value?.querySelector("input")?.blur();
}

function handleSearchClose() {
  mobileSearchOpen.value = false;
  clearSearch();
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
    if (mobileSearchOpen.value) closeMobileSearch();
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
    v-if="!hideTitlebar"
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
        :icon="parentPath ? 'heroicons:chevron-left' : 'heroicons:bars-3'"
        :aria-label="parentPath ? 'Go back' : 'Open menu'"
        @click="handleMobileNavClick"
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
    <UButton
      v-if="!isAuthPage && !mobileSearchOpen"
      :size="iconSize"
      color="neutral"
      variant="ghost"
      class="md:hidden shrink-0"
      icon="heroicons:magnifying-glass"
      aria-label="Search"
      @click="openMobileSearch"
    />

    <div
      v-if="!isAuthPage"
      ref="searchContainerRef"
      data-tour="search"
      class="relative min-w-0 items-center md:mx-auto md:max-w-sm"
      :class="
        mobileSearchOpen ? 'flex flex-1' : 'hidden md:flex md:flex-1'
      "
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
      />
      <UButton
        v-if="mobileSearchOpen"
        :size="iconSize"
        color="neutral"
        variant="ghost"
        class="md:hidden absolute end-0 top-1/2 -translate-y-1/2"
        icon="heroicons:x-mark"
        aria-label="Close search"
        @click="closeMobileSearch"
      />
      <AppGlobalSearch
        v-if="isOpen"
        ref="globalSearchRef"
        @close="handleSearchClose"
      />
    </div>

    <!-- Right actions -->
    <div
      class="items-center gap-0.5 ml-auto"
      :class="mobileSearchOpen ? 'hidden md:flex' : 'flex'"
    >
      <UTooltip v-if="!isAuthPage && !authStore.isGuest" :text="syncLabel">
        <UButton
          :size="iconSize"
          color="neutral"
          variant="ghost"
          :icon="
            syncStatus.error
              ? 'heroicons:exclamation-triangle'
              : 'heroicons:arrow-path'
          "
          :loading="syncStatus.syncing"
          aria-label="Sync now"
          @click="syncNow"
        />
      </UTooltip>

      <span data-tour="theme-toggle">
        <UTooltip :text="themeLabel">
          <UColorModeButton :size="iconSize" />
        </UTooltip>
      </span>

      <span v-if="!isAuthPage" data-tour="notifications">
        <UTooltip text="Notifications">
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
      </span>

      <div
        v-if="!isAuthPage && !authStore.isGuest"
        class="items-center gap-0.5 flex"
      >
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
                {{ preferenceStore.fullName || "Account" }}
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
