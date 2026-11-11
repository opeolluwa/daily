<script setup lang="ts">
import PasteBinCard from "~/components/paste-bin/paste-bin-card.vue";
import PasteBinDialog from "~/components/paste-bin/PasteBinDialog.vue";
import { usePasteBinStore, type PasteBin } from "~/stores/paste-bin";
import {
  PASTE_BIN_SORT_OPTIONS,
  sortPasteBins,
  type PasteBinSort,
} from "~/utils/sorting";

definePageMeta({ layout: false });

const pasteBinStore = usePasteBinStore();
const toast = useToast();
const { searchQuery, setSearch, clearSearch } = useAppSearch();
const sortBy = ref<PasteBinSort>("date-newest");

const editingPaste = ref<PasteBin | null>(null);
const showEditDialog = ref(false);

const filteredPastes = computed(() => {
  const query = searchQuery.value.trim().toLowerCase();
  const list = query
    ? pasteBinStore.pasteBins.filter(
        (paste) =>
          paste.title?.toLowerCase().includes(query) ||
          paste.content.toLowerCase().includes(query),
      )
    : pasteBinStore.pasteBins;

  const sorted = sortPasteBins(list, sortBy.value);
  return [...sorted].sort((a, b) => Number(b.isPinned) - Number(a.isPinned));
});

const pinnedPastes = computed(() =>
  pasteBinStore.pasteBins.filter((paste) => paste.isPinned),
);

function deriveTitle(text: string) {
  const firstLine = text
    .split("\n")
    .map((line) => line.trim())
    .find(Boolean);
  if (!firstLine) return "";
  return firstLine.replace(/^#+\s*/, "").slice(0, 60);
}

function openEdit(paste: PasteBin) {
  editingPaste.value = paste;
  showEditDialog.value = true;
}

function selectPaste(identifier: string) {
  const paste = pasteBinStore.pasteBins.find(
    (entry) => entry.identifier === identifier,
  );
  if (paste) openEdit(paste);
}

async function handlePasteEvent(event: Event) {
  if (!(event instanceof ClipboardEvent)) return;
  const target = event.target as HTMLElement;
  if (
    target.tagName === "TEXTAREA" ||
    target.tagName === "INPUT" ||
    target.isContentEditable
  )
    return;
  const text = event.clipboardData?.getData("text/plain");
  if (!text) return;
  event.preventDefault();
  try {
    await pasteBinStore.createPasteBin({
      title: deriveTitle(text),
      content: text,
      isPinned: false,
    });
    toast.add({
      color: "success",
      icon: "heroicons:check-circle",
      title: "Pasted",
      description: "Content pasted from clipboard",
    });
  } catch {
    toast.add({
      color: "error",
      icon: "heroicons:x-circle",
      title: "Something Went Wrong",
      description: "Failed to save paste",
    });
  }
}

onMounted(async () => {
  setSearch({ placeholder: "Search paste bin..." });
  document.addEventListener("paste", handlePasteEvent);
  await pasteBinStore.fetchPasteBins();
});

onBeforeUnmount(() => {
  document.removeEventListener("paste", handlePasteEvent);
  clearSearch();
});
</script>

<template>
  <NuxtLayout name="default">
    <template #page_title>
      <h1
        v-if="pasteBinStore.pasteBins.length > 0"
        class="text-2xl font-semibold text-gray-800 dark:text-gray-100"
      >
        Paste Bin
      </h1>
    </template>

    <template #main_content>
      <div
        v-if="pasteBinStore.pasteBins.length > 0"
        class="mb-3 flex items-center justify-between"
      >
        <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400">
          Pastes
        </h2>
        <AppSortMenu v-model="sortBy" :options="PASTE_BIN_SORT_OPTIONS" />
      </div>

      <div v-if="pasteBinStore.loading" class="flex flex-col gap-3">
        <USkeleton v-for="i in 3" :key="i" class="h-28 rounded-lg" />
      </div>

      <RyderEmptyState
        v-else-if="pasteBinStore.pasteBins.length === 0"
        title="No pastes yet"
        description="Press ⌘V / Ctrl+V to paste from clipboard."
        icon="ri:clipboard-line"
      />

      <RyderEmptyState
        v-else-if="filteredPastes.length === 0"
        title="No results found"
        description="Try a different search term."
        icon="heroicons:funnel"
        action-label="Clear search"
        @action="searchQuery = ''"
      />

      <div v-else class="flex flex-col gap-3">
        <PasteBinCard
          v-for="paste in filteredPastes"
          :key="paste.identifier"
          :identifier="paste.identifier"
          :title="paste.title"
          :content="paste.content"
          :is-pinned="paste.isPinned"
          :updated-at="paste.updatedAt"
          @select="selectPaste"
          @edit="selectPaste"
        />
      </div>

      <PasteBinDialog v-model:open="showEditDialog" :paste="editingPaste" />
    </template>

    <template #side_content>
      <h2 class="mb-3 text-sm font-medium text-gray-500 dark:text-gray-400">
        Pinned
      </h2>

      <RyderEmptyState
        v-if="pinnedPastes.length === 0"
        title="Nothing pinned"
        description="Pin a paste to keep it close."
        icon="ri:pushpin-2-line"
        compact
      />

      <div v-else class="flex flex-col gap-1">
        <button
          v-for="paste in pinnedPastes"
          :key="paste.identifier"
          class="flex items-center gap-2 rounded-lg px-3 py-2 text-left text-sm transition-colors text-gray-600 hover:bg-gray-50 dark:text-gray-400 dark:hover:bg-gray-800"
          @click="selectPaste(paste.identifier)"
        >
          <UIcon name="ri:clipboard-fill" class="size-4 shrink-0" />
          <span class="truncate">{{ paste.title || "Untitled paste" }}</span>
        </button>
      </div>

      <USeparator class="my-4" />

      <div
        class="flex items-center gap-2 rounded-lg px-3 py-2 text-sm text-gray-400 dark:text-gray-500"
      >
        <UIcon name="ri:clipboard-line" class="size-4" />
        <span>Press</span>
        <kbd
          class="rounded bg-gray-100 px-1.5 py-0.5 text-[10px] font-medium text-gray-500 dark:bg-gray-700 dark:text-gray-400"
          >⌘V</kbd
        >
        <span>to paste</span>
      </div>
    </template>
  </NuxtLayout>
</template>
