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

const createOpen = ref(false);
const createName = ref("");
const creating = ref(false);

function openCreateModal() {
  createName.value = "";
  createOpen.value = true;
}

async function createWhiteboard() {
  if (creating.value) return;
  creating.value = true;
  try {
    const created = await whiteboardStore.createWhiteboard({
      title: createName.value.trim() || null,
    });
    createOpen.value = false;
    await router.push(`/whiteboard/${created.identifier}`);
  } finally {
    creating.value = false;
  }
}

onMounted(async () => {
  setSearch({ placeholder: "Search whiteboards..." });
  await whiteboardStore.fetchWhiteboards();
});

onUnmounted(() => clearSearch());
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <RyderPageHeader
        v-if="whiteboardStore.whiteboards.length > 0"
        title="Whiteboards"
        cta-text="New Whiteboard"
        cta-icon="heroicons:plus"
        @cta="openCreateModal"
      />
      <RyderFab aria-label="New Whiteboard" @click="openCreateModal" />
      <!-- Loading skeletons -->
      <div v-if="whiteboardStore.loading" class="flex flex-col gap-3">
        <USkeleton v-for="i in 3" :key="i" class="h-24 rounded-lg" />
      </div>

      <div
        v-else-if="whiteboardStore.whiteboards.length === 0"
        class="-mt-5 flex min-h-0 flex-1 items-center justify-center"
      >
        <RyderEmptyState
          title="No whiteboards yet"
          description="Create your first whiteboard to start drawing."
          icon="i-lucide-pen-tool"
          action-label="New whiteboard"
          :ui="{ root: '!h-auto' }"
          @action="openCreateModal"
        />
      </div>

      <div
        v-else-if="filteredWhiteboards.length === 0"
        class="-mt-5 flex min-h-0 flex-1 items-center justify-center"
      >
        <RyderEmptyState
          title="No results found"
          description="Try a different search."
          icon="heroicons:funnel"
          action-label="Clear search"
          :ui="{ root: '!h-auto' }"
          @action="searchQuery = ''"
        />
      </div>

      <!-- Whiteboard list -->
      <div v-else class="flex flex-wrap gap-3">
        <WhiteboardCard
          v-for="whiteboard in filteredWhiteboards"
          :key="whiteboard.identifier"
          :whiteboard="whiteboard"
          :search-query="searchQuery"
        />
      </div>
    </template>

    <template #modal_content>
      <UModal v-model:open="createOpen">
        <template #content>
          <div class="px-6 pt-6 pb-4 flex flex-col gap-4">
            <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
              New Whiteboard
            </h2>
            <RyderInput
              v-model="createName"
              label="Name"
              placeholder="Untitled"
              name="whiteboard-name"
              @keydown.enter="createWhiteboard"
            />
            <div class="flex justify-end gap-2">
              <UButton size="sm" variant="ghost" @click="createOpen = false">
                Cancel
              </UButton>
              <UButton size="sm" :loading="creating" @click="createWhiteboard">
                Create
              </UButton>
            </div>
          </div>
        </template>
      </UModal>
    </template>
  </NuxtLayout>
</template>
