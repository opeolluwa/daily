<script setup lang="ts">
definePageMeta({ layout: "default" });

const query = ref("");
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <div class="flex flex-1 flex-col items-center justify-center">
        <div class="w-full max-w-xl flex flex-col items-center gap-6">
          <UIcon
            name="heroicons:magnifying-glass"
            class="size-10 text-gray-400 dark:text-gray-500"
          />
          <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
            Search
          </h1>
          <RyderInput
            v-model="query"
            type="text"
            name="search"
            placeholder="Search everything..."
            class="w-full"
          />
          <p class="text-sm text-gray-500 dark:text-gray-400">
            Search across notes, tasks, bookmarks, and more.
          </p>
        </div>
      </div>
    </template>
  </NuxtLayout>
</template>
