<script setup lang="ts">
import { useNoteStore } from "~/stores/notes";
import { useNoteEditor } from "~/composables/useNoteEditor";
definePageMeta({ layout: false, keepalive: true, name: "Edit notes" });

const route = useRoute();
const noteStore = useNoteStore();

const id = computed(() => route.query.id as string | undefined);
const original = computed(
  () => noteStore.notes.find((n) => n.identifier === id.value) ?? null,
);

const { content, error, notesEditor, title, handleSave } =
  useNoteEditor({
    saveFn: () =>
      noteStore.updateNote(original.value!.identifier, {
        title: title.value,
        content: content.value,
      }),
    canSave: () => {
      if (!original.value) return false;
      return (
        title.value !== original.value.title ||
        content.value !== original.value.content
      );
    },
    source: original,
  });

onMounted(async () => {
  if (noteStore.notes.length === 0) {
    await noteStore.fetchNotes();
  }
});
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <!-- Not found -->
      <div
        v-if="!original && !noteStore.loading"
        class="flex flex-col items-center justify-center py-20 text-center"
      >
        <div class="mb-4 p-3 rounded-full bg-gray-100 dark:bg-gray-800">
          <UIcon name="heroicons:document-text" class="size-7 text-gray-400" />
        </div>
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
          Note not found
        </h3>
        <button
          class="text-xs text-primary-500 hover:text-primary-600 font-medium mt-2"
          @click="$router.push('/notes')"
        >
          Back to notes
        </button>
      </div>

      <!-- Loading -->
      <div
        v-else-if="noteStore.loading && !original"
        class="max-w-2xl mx-auto flex flex-col gap-4"
      >
        <USkeleton class="h-10 rounded-lg w-64" />
        <USkeleton class="h-4 rounded-lg w-32" />
        <USkeleton class="h-96 rounded-lg" />
      </div>

      <div v-else-if="original">
        <div class="notes-editor-desktop min-h-[calc(100dvh-12rem)]">
          <NotesEditor :key="id" ref="notesEditor" v-model="content" />
          <p v-if="error" class="text-xs text-red-500 mt-4">{{ error }}</p>
        </div>
      </div>
    </template>
  </NuxtLayout>
</template>
