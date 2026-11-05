<script setup lang="ts">
import NotesCard from "@desktop/components/notes/notes-card.vue";
import {
  NOTE_SORT_OPTIONS,
  sortNotes,
  type NoteSort,
} from "@shared/utils/sorting";
import { useNoteStore } from "@shared/stores/notes";
import { formatDate } from "@shared/utils/date";

definePageMeta({ layout: false });

const noteStore = useNoteStore();
const router = useRouter();
const { searchQuery, setSearch, clearSearch } = useAppSearch();
const sortBy = ref<NoteSort>("date-newest");

const filteredNotes = computed(() => {
  const q = searchQuery.value.trim().toLowerCase();
  const list = q
    ? noteStore.notes.filter(
        (n) =>
          n.title.toLowerCase().includes(q) ||
          n.content.toLowerCase().includes(q),
      )
    : noteStore.notes;

  return sortNotes(list, sortBy.value);
});

onMounted(async () => {
  setSearch({ placeholder: "Search notes..." });
  await Promise.all([noteStore.fetchNotes(), noteStore.fetchRecentNotes()]);
});

onUnmounted(() => clearSearch());
</script>

<template>
  <NuxtLayout name="default">
    <template #page_title>
      <h1 v-if="noteStore.notes.length > 0" class="text-2xl font-semibold text-gray-800 dark:text-gray-100">
        Notes
      </h1>
    </template>

    <template #primary_cta>
      <AppPrimaryCta
        v-if="noteStore.notes.length !== 0"
        label="New Note"
        icon="heroicons:plus"
        to="/notes/create-notes"
      />
    </template>

    <template #main_content>
      <!-- Sort control -->
      <div
        v-if="!noteStore.loading && noteStore.notes.length > 0"
        class="flex justify-end mb-3"
      >
        <AppSortMenu v-model="sortBy" :options="NOTE_SORT_OPTIONS" />
      </div>

      <!-- Loading -->
      <div v-if="noteStore.loading" class="flex flex-col gap-3">
        <USkeleton v-for="i in 4" :key="i" class="h-24 rounded-lg" />
      </div>

      <RyderEmptyState
        v-else-if="noteStore.notes.length === 0"
        title="No notes yet"
        description="Create your first note to get started."
        icon="heroicons:document-text"
        action-label="Create note"
        @action="navigateTo('/notes/create-notes')"
      />

      <RyderEmptyState
        v-else-if="filteredNotes.length === 0"
        title="No results found"
        description="Try a different search term."
        icon="heroicons:magnifying-glass"
        action-label="Clear search"
        @action="searchQuery = ''"
      />

      <!-- Notes list -->
      <div v-else class="flex flex-col gap-3">
        <NotesCard
          v-for="note in filteredNotes"
          :key="note.identifier"
          :identifier="note.identifier"
          :title="note.title"
          :content="note.content"
          :updated-at="note.updatedAt"
        />
      </div>
    </template>

    <template #side_content>
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Recently modified
      </h2>

      <div v-if="noteStore.recentLoading" class="flex flex-col gap-1">
        <USkeleton v-for="i in 3" :key="i" class="h-10 rounded-lg" />
      </div>

      <RyderEmptyState
        v-else-if="noteStore.recent.length === 0"
        title="No recent notes"
        description="Your recently modified notes will appear here."
        icon="heroicons:clock"
        compact
      />

      <div v-else class="flex flex-col gap-1">
        <div
          v-for="note in noteStore.recent"
          :key="note.identifier"
          class="flex items-center gap-3 py-2.5 px-3 rounded-lg text-sm text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800 cursor-pointer transition-colors"
          @click="router.push(`/notes/edit-notes?id=${note.identifier}`)"
        >
          <UIcon
            name="heroicons:document-text"
            class="size-4 text-gray-400 shrink-0"
          />
          <div class="flex-1 min-w-0">
            <p
              class="font-medium text-gray-700 dark:text-gray-300 truncate text-xs"
            >
              {{ note.title || "Untitled" }}
            </p>
            <p class="text-xs text-gray-400">
              {{ formatDate(note.updatedAt) }}
            </p>
          </div>
        </div>
      </div>
    </template>
  </NuxtLayout>
</template>
