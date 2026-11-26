<script setup lang="ts">
import { useSidebarStore } from "~/stores/sidebar";
const sidebarStore = useSidebarStore();

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
    <!-- Sidebar -->
    <Navigation
      class="bg-white border dark:border-gray-700 dark:bg-gray-800 dark:shadow my-5 mx-4 rounded-2xl"
    />

    <!-- Right column: header + main content -->
    <div class="flex flex-col flex-1 min-w-0 overflow-hidden">
      <!-- Page content + inline aside (fullscreen mode) -->
      <div class="flex flex-1 overflow-hidden">
        <main
          :class="isNoPaddingNeededView ? 'p-0' : 'p-6'"
          class="flex flex-1 flex-col overflow-y-auto scrollbar-config dark:bg-app-dark-800"
        >
          <slot name="modal_content" />
          <div class="flex min-h-0 flex-1 flex-col">
            <slot name="main_content" />
          </div>

          <!-- Workspace lock modal overlay -->
          <WorkspaceLockModal />
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
              :icon="
                sidebarStore.collapsed
                  ? 'heroicons:chevron-double-right'
                  : 'heroicons:chevron-double-left'
              "
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
          <NavigationContent
            :collapsed="sidebarStore.collapsed"
            @navigate="mobileNavOpen = false"
          />
        </div>
      </template>
    </USlideover>
  </UDashboardGroup>
</template>
