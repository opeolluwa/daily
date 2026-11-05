<script setup lang="ts">
import AppFab from "@mobile/components/app/fab.vue";
import PasteBinCard from "@mobile/components/paste-bin/card.vue";
import PasteBinCreatePopup from "@mobile/components/paste-bin/create-popup.vue";
import { usePasteBinStore, type PasteBin } from "@shared/stores/paste-bin";
import { sortPasteBins } from "@shared/utils/sorting";

definePageMeta({ name: "Paste Bin" });

const pasteBinStore = usePasteBinStore();
const { notify } = useAppNotification();
const { searchQuery, clearSearch } = useAppSearch();

const showPopup = ref(false);
const editingPaste = ref<PasteBin | null>(null);

const filteredPastes = computed(() => {
  const query = searchQuery.value.trim().toLowerCase();
  const list = query
    ? pasteBinStore.pasteBins.filter(
        (paste) =>
          paste.title?.toLowerCase().includes(query) ||
          paste.content.toLowerCase().includes(query),
      )
    : pasteBinStore.pasteBins;

  const sorted = sortPasteBins(list, "date-newest");
  return [...sorted].sort((a, b) => Number(b.isPinned) - Number(a.isPinned));
});

function openCreate() {
  editingPaste.value = null;
  showPopup.value = true;
}

function openEdit(identifier: string) {
  const paste = pasteBinStore.pasteBins.find(
    (entry) => entry.identifier === identifier,
  );
  if (!paste) return;
  editingPaste.value = paste;
  showPopup.value = true;
}

function handleSaved() {
  notify({ message: "Paste saved", type: "success" });
}

onMounted(() => {
  pasteBinStore.fetchPasteBins();
});

onUnmounted(() => clearSearch());
</script>

<template>
  <AppPullToRefresh @refresh="() => pasteBinStore.fetchPasteBins()">
    <div>
      <AppFab
        v-if="!pasteBinStore.loading && filteredPastes.length > 0"
        @click="openCreate"
      />

      <div v-if="pasteBinStore.loading" class="flex flex-col gap-3">
        <USkeleton v-for="i in 4" :key="i" class="h-24 rounded-lg" />
      </div>

      <template v-else>
        <div v-if="pasteBinStore.pasteBins.length === 0">
          <RyderEmptyState
            title="No pastes yet"
            description="Save text, links, or anything you want to keep handy."
            icon="ri:clipboard-line"
            action-label="create paste"
            @action="openCreate"
          />
        </div>

        <template v-else>
          <RyderEmptyState
            v-if="filteredPastes.length === 0"
            title="No results found"
            description="Try a different search term."
            icon="heroicons:magnifying-glass"
            action-label="clear search"
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
              @edit="openEdit"
            />
          </div>
        </template>
      </template>

      <PasteBinCreatePopup
        v-model:open="showPopup"
        :paste="editingPaste"
        @saved="handleSaved"
      />
    </div>
  </AppPullToRefresh>
</template>
