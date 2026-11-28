<script setup lang="ts">
import { useAuthStore } from "~/stores/auth";
import { computed, useTemplateRef } from "vue";
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
const searchContainerRef = useTemplateRef<HTMLElement>("searchContainerRef");
const globalSearchRef = useTemplateRef<{
  handleKeydown: (e: KeyboardEvent) => void;
}>("globalSearchRef");
const { toggleMobileNav } = useMobileNav();
const mobileSearchOpen = ref(false);

const colorMode = useColorMode();
const themeLabel = computed(() =>
  colorMode.value === "dark" ? "Light mode" : "Dark mode",
);

const preferenceStore = useUserPreferenceStore();
const route = useRoute();
const authPagesPrefix = "/auth";
const isAuthPage = computed(() => route.path.startsWith(authPagesPrefix));
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

const search = computed<string | number | undefined>({
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
  <div class="titlebar flex items-center gap-1.5 px-2 h-10" data-tauri-drag-region>
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
      :class="mobileSearchOpen ? 'flex flex-1' : 'hidden md:flex md:flex-1'"
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
</style>
