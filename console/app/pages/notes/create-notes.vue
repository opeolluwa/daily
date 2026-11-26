<script setup lang="ts">
import { useNoteStore } from "~/stores/notes";
import { useNoteEditor } from "~/composables/useNoteEditor";
import { stripHtml } from "~/utils/text";
definePageMeta({ layout: false, name: "New note", keepalive: true });

const noteStore = useNoteStore();

const { content, submitting, saved, error, notesEditor, title, handleSave } =
  useNoteEditor({
    saveFn: () =>
      noteStore.createNote({ title: title.value, content: content.value }),
    canSave: () => !!stripHtml(content.value).trim(),
  });

onActivated(() => {
  content.value = "";
  error.value = null;
  submitting.value = false;
  saved.value = false;
  notesEditor.value?.editor?.clearContent(false);
});
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <div class="notes-editor-desktop min-h-[calc(100dvh-12rem)]">
        <NotesEditor ref="notesEditor" v-model="content" />
        <p v-if="error" class="text-xs text-red-500 mt-4">{{ error }}</p>
      </div>
    </template>
  </NuxtLayout>
</template>
