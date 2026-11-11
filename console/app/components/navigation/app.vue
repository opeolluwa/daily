<script setup lang="ts">
import { useSidebarStore } from "~/stores/sidebar";

const sidebarStore = useSidebarStore();
const route = useRoute();

watch(
  () => route.path,
  (path) => {
    sidebarStore.collapsed = path.startsWith("/whiteboard/");
  },
  { immediate: true },
);
</script>

<template>
  <UDashboardSidebar
    id="app"
    v-model:collapsed="sidebarStore.collapsed"
    class="hidden md:flex"
    :collapsible="true"
    :toggle="false"
    :collapsed-size="4"
    :default-size="18"
    :min-size="4"
    :max-size="42"
    :ui="{
      root: [
        'bg-white dark:bg-gray-950',
        'border-e border-gray-200 dark:border-gray-800',
        'overflow-hidden',
        'min-h-0',
      ],
      header: 'shrink-0 h-auto p-0',
      body: 'flex flex-col flex-1 min-h-0 p-0 overflow-hidden',
      footer: 'shrink-0 p-0',
      handle: 'cursor-ew-resize',
    }"
  >
    <NavigationSideNavContent :collapsed="sidebarStore.collapsed" />
  </UDashboardSidebar>
</template>
