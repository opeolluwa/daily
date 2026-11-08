<script setup lang="ts">
import tabs from "@shared/data/settings-tabs";
</script>

<template>
  <nav class="space-y-2">
    <NuxtLink
      v-for="tab in tabs"
      :key="tab.key"
      :to="tab.to"
      class="flex min-h-11 w-full items-center gap-3 rounded-xl px-3 py-3 transition-colors active:bg-gray-50 dark:active:bg-gray-700/50"
    >
      <UIcon
        :name="tab.icon"
        class="size-5 shrink-0 text-gray-400 dark:text-gray-500"
      />

      <div class="min-w-0 flex-1">
        <p class="truncate text-sm font-medium text-gray-900 dark:text-white">
          {{ tab.label }}
        </p>

        <p class="truncate text-xs text-gray-400 dark:text-gray-500">
          {{ tab.desc }}
        </p>
      </div>

      <UIcon
        name="heroicons:chevron-right"
        class="size-4 shrink-0 text-gray-300 dark:text-gray-600"
      />
    </NuxtLink>
  </nav>
</template>
