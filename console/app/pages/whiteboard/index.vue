<script setup lang="ts">
import { useWhiteboardStore } from "~/stores/whiteboards";
import WhiteboardCard from "~/components/whiteboards/whiteboard-card.vue";

definePageMeta({ layout: false });

const router = useRouter();
const whiteboardStore = useWhiteboardStore();
const { searchQuery, setSearch, clearSearch } = useAppSearch();

const filteredWhiteboards = computed(() => {
  const q = searchQuery.value.trim().toLowerCase();
  if (!q) return whiteboardStore.whiteboards;
  return whiteboardStore.whiteboards.filter((w) =>
    (w.title ?? "Untitled").toLowerCase().includes(q),
  );
});

async function createWhiteboard() {
  const created = await whiteboardStore.createWhiteboard({ title: null });
  await router.push(`/whiteboard/${created.identifier}`);
}

onMounted(async () => {
  setSearch({ placeholder: "Search whiteboards..." });
  await whiteboardStore.fetchWhiteboards();
});

onUnmounted(() => clearSearch());
</script>

<template>
  <NuxtLayout name="default">
    <template #page_title>
      <h1
        v-if="whiteboardStore.whiteboards.length > 0"
        class="text-2xl font-semibold text-gray-800 dark:text-gray-100"
      >
        Whiteboards
      </h1>
    </template>

    <template #primary_cta>
      <div
        v-if="whiteboardStore.whiteboards.length !== 0"
        class="hidden md:flex items-center justify-end"
        @click="createWhiteboard"
      >
        <button
          class="flex items-center gap-2 py-2 px-4 bg-primary-500 text-white rounded-lg text-sm font-medium hover:bg-primary-600 transition-colors"
        >
          <UIcon name="heroicons:plus" class="size-4" />
          New Whiteboard
        </button>
      </div>
      <RyderFab aria-label="New Whiteboard" @click="createWhiteboard" />
    </template>

    <template #main_content>
      <!-- Loading skeletons -->
      <div v-if="whiteboardStore.loading" class="flex flex-col gap-3">
        <USkeleton v-for="i in 3" :key="i" class="h-24 rounded-lg" />
      </div>

      <RyderEmptyState
        v-else-if="whiteboardStore.whiteboards.length === 0"
        title="No whiteboards yet"
        description="Create your first whiteboard to start drawing."
        icon="i-lucide-pen-tool"
        action-label="New whiteboard"
        @action="createWhiteboard"
      />

      <RyderEmptyState
        v-else-if="filteredWhiteboards.length === 0"
        title="No results found"
        description="Try a different search."
        icon="heroicons:funnel"
        action-label="Clear search"
        @action="searchQuery = ''"
      />

      <!-- Whiteboard list -->
      <div v-else class="flex flex-col gap-3">
        <WhiteboardCard
          v-for="whiteboard in filteredWhiteboards"
          :key="whiteboard.identifier"
          :whiteboard="whiteboard"
          :search-query="searchQuery"
        />
      </div>
    </template>
  </NuxtLayout>
</template>
