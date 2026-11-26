<script setup lang="ts">
import { useSidebarStore } from "~/stores/sidebar";

const sidebarStore = useSidebarStore();
const collapsed = ref(false);
</script>

<template>
  <UDashboardSidebar
    id="app"
    v-model:collapsed="collapsed"
    data-tour="sidebar"
    class="hidden md:flex"
    :collapsible="true"
    :resizable="true"
    toggle-side="right"
    :collapsed-size="4"
    :default-size="16"
    :min-size="4"
    :max-size="42"
    :ui="{
      root: [
        'bg-transparent',
        'border border-gray-200 dark:border-gray-800',
        'overflow-hidden',
        'min-h-0',
      ],
      header: 'shrink-0 h-auto p-0',
      body: 'flex flex-col flex-1 min-h-0 p-0 overflow-hidden',
      footer: 'shrink-0 p-0',
      handle: 'cursor-ew-resize',
    }"
  >
    <NavigationContent
      :collapsed="sidebarStore.collapsed"
      :show-user-card="false"
    />
  </UDashboardSidebar>
</template>
