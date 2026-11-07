<script setup lang="ts">
import { useSnippetStore } from "@shared/stores/snippets";
import {
  SNIPPET_SORT_OPTIONS,
  sortSnippets,
  type SnippetSort,
} from "@shared/utils/sorting";
import SnippetCard from "@mobile/components/snippet/snippet-card.vue";

definePageMeta({ name: "Snippets" });

const snippetStore = useSnippetStore();
const router = useRouter();
const { searchQuery, clearSearch } = useAppSearch();
const { notify } = useAppNotification();

const activeLanguage = ref("All");
const sortBy = ref<SnippetSort>("date-newest");

const allLanguages = computed(() => ["All", ...snippetStore.languages]);

const filteredSnippets = computed(() => {
  let list = snippetStore.snippets;

  if (activeLanguage.value !== "All") {
    list = list.filter((snippet) => snippet.language === activeLanguage.value);
  }

  const query = searchQuery.value.trim().toLowerCase();
  if (query) {
    list = list.filter(
      (snippet) =>
        snippet.title?.toLowerCase().includes(query) ||
        snippet.language?.toLowerCase().includes(query) ||
        snippet.description?.toLowerCase().includes(query) ||
        snippet.code.toLowerCase().includes(query),
    );
  }

  return sortSnippets(list, sortBy.value);
});

function openCreate() {
  router.push("/snippets/create-snippet");
}

async function handleDelete(identifier: string) {
  await snippetStore.deleteSnippet(identifier);
  notify({ message: "Snippet moved to recycle bin", type: "success" });
}

function clearFilters() {
  searchQuery.value = "";
  activeLanguage.value = "All";
}

onMounted(() => {
  snippetStore.fetchSnippets();
});

onUnmounted(() => clearSearch());
</script>

<template>
  <RyderPullToRefresh @refresh="() => snippetStore.fetchSnippets()">
    <div>
      <!-- Create snippet FAB -->
      <RyderFab
        v-if="!snippetStore.loading && snippetStore.snippets.length > 0"
        class="!bottom-24"
        @click="openCreate"
      />

      <!-- Loading -->
      <div v-if="snippetStore.loading" class="flex flex-col gap-3">
        <USkeleton v-for="i in 3" :key="i" class="h-40 rounded-lg" />
      </div>

      <template v-else>
        <!-- Empty state: no snippets at all -->
        <div v-if="snippetStore.snippets.length === 0">
          <RyderEmptyState
            title="No snippets yet"
            description="Save your first code snippet to get started."
            icon="ri:code-s-slash-line"
            action-label="create snippet"
            @action="openCreate"
          />
        </div>

        <template v-else>
          <!-- Language filters + sort -->
          <div class="mb-4 flex items-start justify-between gap-2">
            <div class="flex flex-1 flex-wrap gap-2">
              <button
                v-for="language in allLanguages"
                :key="language"
                class="rounded-full px-3 py-1.5 text-xs font-medium transition-colors"
                :class="
                  activeLanguage === language
                    ? 'bg-primary-500 text-white'
                    : 'bg-gray-100 text-gray-600 hover:bg-gray-200 dark:bg-gray-800 dark:text-gray-400 dark:hover:bg-gray-700'
                "
                @click="activeLanguage = language"
              >
                {{ language }}
              </button>
            </div>
            <AppSortMenu v-model="sortBy" :options="SNIPPET_SORT_OPTIONS" />
          </div>

          <!-- Empty state: filter/search has no results -->
          <RyderEmptyState
            v-if="filteredSnippets.length === 0"
            title="No results found"
            description="Try a different search term or language filter."
            icon="heroicons:magnifying-glass"
            action-label="clear filters"
            @action="clearFilters"
          />

          <!-- Snippet list -->
          <div v-else class="flex flex-col gap-3">
            <SnippetCard
              v-for="snippet in filteredSnippets"
              :key="snippet.identifier"
              :snippet="snippet"
              @delete="handleDelete"
            />
          </div>
        </template>
      </template>
    </div>
  </RyderPullToRefresh>
</template>
