<script setup lang="ts">
import { useNoteStore } from "~/stores/notes";
import { onBeforeRouteLeave } from "vue-router";
import NoteTitleInput from "~/components/notes/note-title-input.vue";
definePageMeta({ layout: false, name: "New note", keepalive: true });

const router = useRouter();
const noteStore = useNoteStore();

const title = ref("");
const content = ref("");
const submitting = ref(false);
const saved = ref(false);
const error = ref<string | null>(null);
const notesEditor = ref<InstanceType<typeof NotesEditor> | null>(null);

onActivated(() => {
  title.value = "";
  content.value = "";
  error.value = null;
  submitting.value = false;
  saved.value = false;
  notesEditor.value?.editor?.clearContent(false);
});

// ── word count ────────────────────────────────────────────────────────────────
const wordCount = computed(() => {
  const text = content.value.replace(/<[^>]*>/g, " ").trim();
  if (!text) return 0;
  return text.split(/\s+/).filter(Boolean).length;
});

const _readTime = computed(() => Math.max(1, Math.ceil(wordCount.value / 200)));

const charCount = computed(() => {
  return content.value.replace(/<[^>]*>/g, "").replace(/\s/g, "").length;
});

const lastSaved = ref<Date | null>(null);

const plainText = computed(() => content.value.replace(/<[^>]*>/g, "").trim());

const hasContent = computed(() => !!title.value.trim() || !!plainText.value);

// ── save ──────────────────────────────────────────────────────────────────────
async function handleSave() {
  if (!hasContent.value) return;
  submitting.value = true;
  error.value = null;
  try {
    await noteStore.createNote({
      title: title.value.trim() || "Untitled",
      content: content.value,
    });
    saved.value = true;
    lastSaved.value = new Date();
    router.push("/notes");
  } catch (e) {
    error.value = String(e);
    submitting.value = false;
  }
}

useEventListener("keydown", (e: KeyboardEvent) => {
  if ((e.metaKey || e.ctrlKey) && e.key === "s") {
    e.preventDefault();
    if (hasContent.value && !submitting.value) handleSave();
  }
});

onBeforeRouteLeave(async () => {
  if (submitting.value || saved.value) return;
  if (!hasContent.value) return;
  try {
    await noteStore.createNote({
      title: title.value.trim() || "Untitled",
      content: content.value,
    });
  } catch (e) {
    console.error(e);
  }
});
</script>

<template>
  <NuxtLayout name="default">
    <template #page_title>
      <NoteTitleInput v-model="title" :disabled="submitting" />
    </template>
    <template #main_content>
      <div class="notes-editor-desktop min-h-[calc(100dvh-12rem)]">
        <NotesEditor ref="notesEditor" v-model="content">
          <template #toolbar>
            <NotesDesktopToolBarWrapper>
              <NotesEditorToolBar />
            </NotesDesktopToolBarWrapper>
          </template>
        </NotesEditor>
      </div>
      <p v-if="error" class="text-xs text-red-500 mt-6">
        {{ error }}
      </p>
    </template>

    <template #side_content>
      <!-- Save -->
      <RyderButton
        block
        :loading="submitting"
        :disabled="!hasContent"
        class="mb-2"
        @click="handleSave"
      >
        Save note
      </RyderButton>
      <RyderButton
        block
        variant="outline"
        :disabled="submitting"
        @click="router.push('/notes')"
      >
        Discard
      </RyderButton>

      <USeparator class="my-5" />

      <!-- Stats -->
      <p
        class="text-[10px] font-semibold text-gray-400 dark:text-gray-500 uppercase tracking-widest mb-3"
      >
        Document
      </p>
      <div class="flex flex-col gap-0.5">
        <div
          v-for="stat in [
            { label: 'Words', value: wordCount },
            { label: 'Characters', value: charCount },
          ]"
          :key="stat.label"
          class="flex items-center justify-between py-2 border-b border-gray-50 dark:border-gray-800/60 text-xs"
        >
          <span class="text-gray-400">{{ stat.label }}</span>
          <span
            class="tabular-nums font-semibold text-gray-700 dark:text-gray-200"
          >
            {{ stat.value }}
          </span>
        </div>
      </div>
    </template>
  </NuxtLayout>
</template>

<style></style>
