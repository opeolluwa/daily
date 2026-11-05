<script setup lang="ts">
import PasteBinCard from "@desktop/components/paste-bin/paste-bin-card.vue";
import { usePasteBinStore, type PasteBin } from "@shared/stores/paste-bin";
import {
  PASTE_BIN_SORT_OPTIONS,
  sortPasteBins,
  type PasteBinSort,
} from "@shared/utils/sorting";

definePageMeta({ layout: false });

const pasteBinStore = usePasteBinStore();
const { notify } = useAppNotification();
const { searchQuery, setSearch, clearSearch } = useAppSearch();
const sortBy = ref<PasteBinSort>("date-newest");

const selectedId = ref<string | null>(null);
const title = ref("");
const content = ref("");
const isPinned = ref(false);
const titleDirty = ref(false);
const hydrating = ref(false);
const saveStatus = ref<"idle" | "saving" | "saved">("idle");

let saveTimer: ReturnType<typeof setTimeout> | null = null;
let saveChain: Promise<void> = Promise.resolve();

const selectedPaste = computed(() =>
  pasteBinStore.pasteBins.find(
    (paste) => paste.identifier === selectedId.value,
  ),
);

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

const displayTitle = computed(() => title.value.trim() || "Untitled paste");
const canSave = computed(() => !!content.value.trim() || !!selectedId.value);

function deriveTitle(text: string) {
  const firstLine = text
    .split("\n")
    .map((line) => line.trim())
    .find(Boolean);
  if (!firstLine) return "";
  return firstLine.replace(/^#+\s*/, "").slice(0, 60);
}

function hydrate(paste: PasteBin) {
  hydrating.value = true;
  selectedId.value = paste.identifier;
  title.value = paste.title ?? "";
  content.value = paste.content;
  isPinned.value = paste.isPinned;
  titleDirty.value =
    !!paste.title && paste.title !== deriveTitle(paste.content);
  saveStatus.value = "saved";
  nextTick(() => {
    hydrating.value = false;
  });
}

function clearComposer() {
  selectedId.value = null;
  title.value = "";
  content.value = "";
  isPinned.value = false;
  titleDirty.value = false;
  saveStatus.value = "idle";
}

async function saveNow() {
  if (hydrating.value || !canSave.value) return;

  saveStatus.value = "saving";
  try {
    if (!selectedId.value) {
      const created = await pasteBinStore.createPasteBin({
        title: title.value.trim() || null,
        content: content.value,
        isPinned: isPinned.value,
      });
      selectedId.value = created.identifier;
    } else {
      const updated = await pasteBinStore.updatePasteBin(selectedId.value, {
        title: title.value.trim(),
        content: content.value,
        isPinned: isPinned.value,
      });
      if (title.value === updated.title && !titleDirty.value) {
        title.value = updated.title ?? "";
      }
    }
    saveStatus.value = "saved";
  } catch (error) {
    console.error("[paste-bin] failed to save", error);
    saveStatus.value = "idle";
    notify({ type: "error", message: "Failed to save paste" });
  }
}

function flush() {
  if (saveTimer) {
    clearTimeout(saveTimer);
    saveTimer = null;
  }
  saveChain = saveChain.then(() => saveNow()).catch(() => undefined);
  return saveChain;
}

function scheduleSave() {
  if (hydrating.value || !canSave.value) return;
  saveStatus.value = "saving";
  if (saveTimer) clearTimeout(saveTimer);
  saveTimer = setTimeout(() => {
    saveTimer = null;
    saveChain = saveChain.then(() => saveNow()).catch(() => undefined);
  }, 900);
}

watch(content, (value) => {
  if (!titleDirty.value) title.value = deriveTitle(value);
  scheduleSave();
});

watch([title, isPinned], scheduleSave);

watch(
  () => pasteBinStore.pasteBins.map((paste) => paste.identifier),
  (identifiers) => {
    if (selectedId.value && !identifiers.includes(selectedId.value)) {
      clearComposer();
    }
  },
);

async function selectPaste(identifier: string) {
  if (selectedId.value === identifier) return;
  await flush();
  const paste = pasteBinStore.pasteBins.find(
    (entry) => entry.identifier === identifier,
  );
  if (paste) hydrate(paste);
}

async function createNewPaste() {
  await flush();
  clearComposer();
}

async function pasteFromClipboard() {
  try {
    const text = await navigator.clipboard.readText();
    if (!text) return;
    await flush();
    selectedId.value = null;
    titleDirty.value = false;
    content.value = text;
    title.value = deriveTitle(text);
    isPinned.value = false;
    await flush();
    notify({ message: "Pasted from clipboard", type: "success" });
  } catch (error) {
    console.error("[paste-bin] clipboard read failed", error);
    notify({ type: "error", message: "Could not read the clipboard" });
  }
}

async function copyCurrent() {
  if (!content.value) return;
  await navigator.clipboard.writeText(content.value);
  notify({ message: "Copied to clipboard", type: "success" });
}

function handleTitleInput() {
  titleDirty.value = true;
  scheduleSave();
}

onMounted(async () => {
  setSearch({ placeholder: "Search paste bin..." });
  await pasteBinStore.fetchPasteBins();
});

onBeforeUnmount(() => {
  void flush();
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

    <template #primary_cta>
      <div
        v-if="pasteBinStore.pasteBins.length !== 0"
        class="hidden items-center justify-end md:flex"
      >
        <button
          class="flex items-center gap-2 rounded-lg bg-primary-500 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-primary-600"
          @click="createNewPaste"
        >
          <UIcon name="heroicons:plus" class="size-4" />
          New Paste
        </button>
      </div>
      <RyderFab aria-label="New Paste" @click="createNewPaste" />
    </template>

    <template #main_content>
      <div
        class="mb-5 overflow-hidden rounded-lg border border-gray-100 bg-white dark:border-gray-700 dark:bg-gray-800"
      >
        <div
          class="flex items-center gap-2 border-b border-gray-50 px-4 py-2.5 dark:border-gray-700"
        >
          <input
            v-model="title"
            type="text"
            :placeholder="displayTitle"
            class="min-w-0 flex-1 bg-transparent text-sm font-medium text-gray-800 outline-none placeholder:text-gray-400 dark:text-gray-100 dark:placeholder:text-gray-500"
            @input="handleTitleInput"
            @blur="flush"
          />
          <button
            class="rounded-lg p-1.5 text-gray-400 transition-colors hover:bg-gray-100 hover:text-gray-600 dark:hover:bg-gray-700 dark:hover:text-gray-300"
            :class="isPinned ? 'text-primary-500 dark:text-primary-400' : ''"
            :aria-label="isPinned ? 'Unpin paste' : 'Pin paste'"
            @click="isPinned = !isPinned"
          >
            <UIcon
              :name="isPinned ? 'ri:pushpin-2-fill' : 'ri:pushpin-2-line'"
              class="size-4"
            />
          </button>
        </div>

        <textarea
          v-model="content"
          class="h-64 w-full resize-none bg-transparent p-4 font-mono text-sm leading-relaxed text-gray-700 outline-none placeholder:text-gray-400 dark:text-gray-200 dark:placeholder:text-gray-500"
          placeholder="Paste or type anything here. It saves automatically."
          @blur="flush"
        />

        <div
          class="flex items-center justify-between border-t border-gray-50 px-4 py-2 dark:border-gray-700"
        >
          <span class="text-xs text-gray-400">
            {{
              saveStatus === "saving"
                ? "Saving..."
                : saveStatus === "saved"
                  ? "Saved"
                  : "Auto-saves as you type"
            }}
          </span>
          <div class="flex items-center gap-3">
            <button
              v-if="content"
              class="text-xs font-medium text-gray-400 transition-colors hover:text-gray-600 dark:hover:text-gray-300"
              @click="copyCurrent"
            >
              Copy
            </button>
            <span class="text-xs text-gray-400">
              {{ content.length }} characters
            </span>
          </div>
        </div>
      </div>

      <div class="mb-3 flex items-center justify-between">
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
        description="Paste something above and it will be saved automatically."
        icon="ri:clipboard-line"
        action-label="Paste from clipboard"
        @action="pasteFromClipboard"
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
          :active="paste.identifier === selectedId"
          @select="selectPaste"
        />
      </div>
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
          class="flex items-center gap-2 rounded-lg px-3 py-2 text-left text-sm transition-colors"
          :class="
            paste.identifier === selectedId
              ? 'bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-200'
              : 'text-gray-600 hover:bg-gray-50 dark:text-gray-400 dark:hover:bg-gray-800'
          "
          @click="selectPaste(paste.identifier)"
        >
          <UIcon name="ri:clipboard-fill" class="size-4 shrink-0" />
          <span class="truncate">{{ paste.title || "Untitled paste" }}</span>
        </button>
      </div>

      <USeparator class="my-4" />

      <h2 class="mb-3 text-sm font-medium text-gray-500 dark:text-gray-400">
        Quick Actions
      </h2>
      <div class="flex flex-col gap-2">
        <button
          class="flex w-full items-center gap-2 rounded-lg px-3 py-2 text-left text-sm text-gray-600 transition-colors hover:bg-gray-50 dark:text-gray-400 dark:hover:bg-gray-800"
          @click="pasteFromClipboard"
        >
          <UIcon name="ri:clipboard-line" class="size-4" />
          Paste from clipboard
        </button>
        <button
          class="flex w-full items-center gap-2 rounded-lg px-3 py-2 text-left text-sm text-gray-600 transition-colors hover:bg-gray-50 disabled:opacity-50 dark:text-gray-400 dark:hover:bg-gray-800"
          :disabled="!content"
          @click="copyCurrent"
        >
          <UIcon name="ri:file-copy-line" class="size-4" />
          Copy current paste
        </button>
        <button
          class="flex w-full items-center gap-2 rounded-lg px-3 py-2 text-left text-sm text-gray-600 transition-colors hover:bg-gray-50 dark:text-gray-400 dark:hover:bg-gray-800"
          @click="createNewPaste"
        >
          <UIcon name="heroicons:plus" class="size-4" />
          New paste
        </button>
      </div>
    </template>
  </NuxtLayout>
</template>
