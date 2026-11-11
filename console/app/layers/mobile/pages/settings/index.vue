<script setup lang="ts">
definePageMeta({ name: "Settings" });

const routes = [
  {
    label: "Profile",
    description: "Name, email & avatar",
    icon: "heroicons:user",
    to: "/settings/profile",
  },
  {
    label: "Workspaces",
    description: "Create, secure & manage workspaces",
    icon: "heroicons:briefcase",
    to: "/settings/workspaces",
  },
  {
    label: "Backup & Sync",
    description: "Local or cloud backup",
    icon: "heroicons:cloud-arrow-up",
    to: "/settings/backup",
  },
  {
    label: "About",
    description: "Version, build & platform",
    icon: "heroicons:information-circle",
    to: "/settings/about",
  },
];
</script>

<template>
  <UNavigationMenu
    #item="{ item }"
    orientation="vertical"
    :items="routes"
    highlight
    highlight-color="neutral"
    class="w-full"
    :ui="{
      link: 'px-2 py-2 text-sm rounded-lg transition-colors hover:bg-white/5 data-active:font-medium',
      linkLeadingIcon: 'size-5  ',
      label:
        'mt-4 pt-3 border-t border-white/10 px-2 pb-1.5 text-[11px] font-medium uppercase tracking-[0.14em]',
    }"
  >
    <NuxtLink :to="item.to" class="flex items-center gap-y-8 gap-x-3 mb-3">
      <div class="min-w-0 flex-1">
        <h3 class="text-sm font-medium leading-5 text-gray-900 dark:text-white">
          {{ item.label }}
        </h3>
        <p class="text-sm text-gray-400 dark:text-gray-500 mt-0.5">
          {{ item.description }}
        </p>
      </div>
    </NuxtLink>
  </UNavigationMenu>
  
  
</template>
