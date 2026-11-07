<script setup lang="ts">
import PasteBinCard from "@mobile/components/paste-bin/card.vue";
import { usePasteBinStore } from "@shared/stores/paste-bin";
import { sortPasteBins } from "@shared/utils/sorting";

definePageMeta({ name: "Paste Bin" });

const pasteBinStore = usePasteBinStore();
const router = useRouter();
const { searchQuery, clearSearch } = useAppSearch();

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
  router.push("/paste-bin/create-paste");
}

function openEdit(identifier: string) {
  router.push({ path: "/paste-bin/edit-paste", query: { id: identifier } });
}

onMounted(() => {
  pasteBinStore.fetchPasteBins();
});

onUnmounted(() => clearSearch());
</script>

<template>
  <RyderPullToRefresh @refresh="() => pasteBinStore.fetchPasteBins()">
    <div>
      <RyderFab
        v-if="!pasteBinStore.loading && filteredPastes.length > 0"
        class="!bottom-24"
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
    </div>
  </RyderPullToRefresh>
</template>
