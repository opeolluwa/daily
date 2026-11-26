<script setup lang="ts">
import NotesCard from "~/components/notes/notes-card.vue";
import { NOTE_SORT_OPTIONS, sortNotes, type NoteSort } from "~/utils/sorting";
import { useNoteStore } from "~/stores/notes";
import { formatDate } from "~/utils/date";
import { ryderPageHeaderOverride } from "~/assets/ryder.css";

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
    <template #main_content>
      <RyderPageHeader
        v-if="noteStore.notes.length > 0"
        :ui="ryderPageHeaderOverride"
        title="Notes"
        cta-text="New Note"
        cta-icon="heroicons:plus"
        @cta="router.push('/notes/create-notes')"
      />
      <RyderFab
        v-if="noteStore.notes.length > 0 && filteredNotes.length > 0"
        aria-label="New Note"
        @click="router.push('/notes/create-notes')"
      />
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
  </NuxtLayout>
</template>
