<script setup lang="ts">
import type { NotesInterface } from "@daily/core";

defineProps<{
  notes: NotesInterface[];
  loading: boolean;
}>();

function stripHtml(html: string) {
  return html
    .replace(/<[^>]*>/g, " ")
    .replace(/\s+/g, " ")
    .trim();
}

function formatDate(iso: string) {
  return new Date(iso).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
  });
}
</script>

<template>
  <div
    class="bg-white dark:bg-gray-800/60 rounded-2xl border border-gray-200 dark:border-white/15 overflow-hidden flex flex-col h-full"
  >
    <div
      class="flex items-center justify-between px-4 py-3.5 border-b border-gray-100 dark:border-white/10"
    >
      <h2
        class="text-sm font-semibold text-gray-700 dark:text-gray-300/70 flex items-center gap-1.5"
      >
        <UIcon name="heroicons:document-text" class="size-4 text-violet-400" />
        Recent notes
      </h2>
    </div>

    <div class="flex-1">
      <div
        v-if="loading"
        class="flex items-center gap-2 p-4 text-gray-400 text-xs"
      >
        <UIcon name="heroicons:arrow-path" class="size-3.5 animate-spin" />
        Loading…
      </div>

      <RyderEmptyState
        v-else-if="notes.length === 0"
        title="No notes yet"
        description="Your recent notes will appear here."
        icon="heroicons:document-text"
        compact
      />

      <div v-else class="divide-y divide-gray-100 dark:divide-gray-700/60">
        <NuxtLink
          v-for="note in notes"
          :key="note.identifier"
          to="/notes"
          class="group flex items-start gap-2.5 px-4 py-2.5 hover:bg-gray-50 dark:hover:bg-gray-800/40 transition-colors"
        >
          <div
            class="size-6 rounded-md bg-violet-50 dark:bg-violet-950/60 flex items-center justify-center shrink-0 mt-0.5"
          >
            <UIcon
              name="heroicons:document-text"
              class="size-3 text-violet-400"
            />
          </div>
          <div class="flex-1 min-w-0">
            <p
              class="text-xs font-medium text-gray-800 dark:text-gray-200 truncate group-hover:text-violet-600 dark:group-hover:text-violet-400 transition-colors"
            >
              {{ note.title }}
            </p>
            <p class="text-xs text-gray-400 truncate mt-0.5">
              {{ stripHtml(note.content) || "No content" }}
            </p>
          </div>
          <span class="text-xs text-gray-300 dark:text-gray-600 shrink-0">
            {{ formatDate(note.updatedAt) }}
          </span>
        </NuxtLink>
      </div>
    </div>
  </div>
</template>
