<script setup lang="ts">
import { useSidebarStore } from "~/stores/sidebar";
import { useWorkspacesStore } from "~/stores/workspaces";

const sidebarStore = useSidebarStore();
const workspaceStore = useWorkspacesStore();

const route = useRoute();
const { mobileNavOpen } = useMobileNav();
const noPaddingPatterns = [/^\/whiteboard\/[^/]+$/];
const isNoPaddingNeededView = computed(() =>
  noPaddingPatterns.some((pattern) => pattern.test(route.path)),
);
</script>

<template>
  <UDashboardGroup
    id="wild_almonds_app"
    as="div"
    unit="rem"
    :persistent="false"
  >
    <!-- Sidebar: icons-only strip when collapsed -->
     <Navigation class="bg-white dark:bg-gray-900 mb-4 mx-3 rounded-xl px-2" />

    <!-- Right column: header + main content -->
    <div class="flex flex-col flex-1 min-w-0 overflow-hidden">
      <!-- Page content + inline aside (fullscreen mode) -->
      <div class="flex flex-1 overflow-hidden">
        <main
          :class="isNoPaddingNeededView ? 'p-0' : 'p-6'"
          class="flex flex-1 flex-col overflow-y-auto scrollbar-config dark:bg-app-dark-800"
        >
          <slot
            v-if="
              !workspaceStore.isCurrentWorkspaceLocked ||
              route.path.startsWith('/settings')
            "
            name="modal_content"
          />
          <div class="mt-5 flex min-h-0 flex-1 flex-col">
            <div
              v-if="
                workspaceStore.isCurrentWorkspaceLocked &&
                !route.path.startsWith('/settings')
              "
              class="flex flex-col items-center justify-center gap-3 py-24 text-center"
            >
              <UIcon
                name="heroicons:lock-closed"
                class="size-10 text-amber-400 dark:text-amber-500"
              />
              <p class="text-sm font-medium text-gray-700 dark:text-gray-300">
                This workspace is password protected
              </p>
              <p class="text-xs text-gray-400 dark:text-gray-500">
                Select it from the workspace switcher to unlock.
              </p>
            </div>
            <slot
              v-else-if="
                !workspaceStore.isCurrentWorkspaceLocked ||
                route.path.startsWith('/settings')
              "
              name="main_content"
            />
          </div>
        </main>
      </div>
    </div>

    <!-- Mobile nav drawer -->
    <USlideover
      v-model:open="mobileNavOpen"
      side="left"
      :ui="{ content: sidebarStore.collapsed ? 'max-w-20' : 'max-w-64' }"
    >
      <template #content>
        <div class="flex flex-col h-full bg-white dark:bg-gray-900">
          <!-- Safe-area spacer -->
          <div class="shrink-0" style="height: env(safe-area-inset-top)" />
          <!-- Header -->
          <div
            class="flex items-center justify-between px-3 py-4 border-b border-gray-200 dark:border-gray-800 shrink-0"
          >
            <UButton
              size="sm"
              color="neutral"
              variant="ghost"
              :icon="sidebarStore.collapsed ? 'heroicons:chevron-double-right' : 'heroicons:chevron-double-left'"
              aria-label="Toggle sidebar"
              @click="sidebarStore.collapsed = !sidebarStore.collapsed"
            />
            <UButton
              size="sm"
              color="neutral"
              variant="ghost"
              icon="heroicons:x-mark"
              aria-label="Close menu"
              @click="mobileNavOpen = false"
            />
          </div>

          <!-- Primary nav -->
          <NavigationSideNavContent
            :collapsed="sidebarStore.collapsed"
            @navigate="mobileNavOpen = false"
          />
        </div>
      </template>
    </USlideover>
  </UDashboardGroup>
</template>
