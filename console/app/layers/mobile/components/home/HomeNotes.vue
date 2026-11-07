<script setup lang="ts">
import { useNoteStore } from "@shared/stores/notes";

const noteStore = useNoteStore();

const recentNotes = computed(() => noteStore.notes.slice(0, 3));

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
  <USkeleton v-if="noteStore.loading" class="mt-4 h-40 rounded-2xl" />

  <RyderCard
    v-else
    :ui="{
      root: 'mt-4 overflow-hidden !rounded-2xl !border-gray-200 !bg-white dark:!border-white/15 dark:!bg-gray-800/60 !p-0',
    }"
  >
    <div
      v-if="recentNotes.length > 0"
      class="flex items-center justify-between border-b border-gray-100 px-4 py-3 dark:border-white/10"
    >
      <h2
        class="flex items-center gap-1.5 text-sm font-semibold text-gray-700 dark:text-gray-300/70"
      >
        <UIcon name="heroicons:document-text" class="size-4 text-primary-400" />
        Recent notes
      </h2>
      <NuxtLink
        to="/notes"
        class="text-xs font-medium text-primary-500 transition-colors hover:text-primary-600"
      >
        See all
      </NuxtLink>
    </div>

    <RyderEmptyState
      v-if="recentNotes.length === 0"
      title="No notes yet"
      description="Your recent notes will appear here."
      icon="heroicons:document-text"
      compact
    />

    <div v-else class="divide-y divide-gray-100 dark:divide-gray-700/60">
      <NuxtLink
        v-for="note in recentNotes"
        :key="note.identifier"
        to="/notes"
        class="flex items-start gap-2.5 px-4 py-3 transition-colors active:bg-gray-50 dark:active:bg-white/5"
      >
        <div
          class="mt-0.5 flex size-6 shrink-0 items-center justify-center rounded-md bg-primary-50 dark:bg-primary-950/60"
        >
          <UIcon
            name="heroicons:document-text"
            class="size-3 text-primary-400"
          />
        </div>
        <div class="min-w-0 flex-1">
          <p
            class="truncate text-xs font-medium text-gray-800 dark:text-gray-200"
          >
            {{ note.title }}
          </p>
          <p class="mt-0.5 truncate text-[11px] text-gray-400">
            {{ stripHtml(note.content) || "No content" }}
          </p>
        </div>
        <span class="shrink-0 text-[11px] text-gray-300 dark:text-gray-600">
          {{ formatDate(note.updatedAt) }}
        </span>
      </NuxtLink>
    </div>
  </RyderCard>
</template>
