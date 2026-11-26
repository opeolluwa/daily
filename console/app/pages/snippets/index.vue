<script setup lang="ts">
import { useSnippetStore, type Snippet } from "~/stores/snippets";
import SnippetCard from "~/components/snippets/snippet-card.vue";
import SnippetDialog from "~/components/snippets/SnippetDialog.vue";
import {
  SNIPPET_SORT_OPTIONS,
  sortSnippets,
  type SnippetSort,
} from "~/utils/sorting";
definePageMeta({ layout: false });

const snippetStore = useSnippetStore();
const router = useRouter();
const isMobile = useIsMobile();
const activeLanguage = ref("All");
const { searchQuery, setSearch, clearSearch } = useAppSearch();
const sortBy = ref<SnippetSort>("date-newest");
const dialogOpen = ref(false);
const editingSnippet = ref<Snippet | null>(null);

const allLanguages = computed(() => ["All", ...snippetStore.languages]);

const filteredSnippets = computed(() => {
  let list = snippetStore.snippets;

  if (activeLanguage.value !== "All") {
    list = list.filter((s) => s.language === activeLanguage.value);
  }

  const q = searchQuery.value.trim().toLowerCase();
  if (q) {
    list = list.filter(
      (s) =>
        s.title?.toLowerCase().includes(q) ||
        s.language?.toLowerCase().includes(q) ||
        s.description?.toLowerCase().includes(q) ||
        s.code.toLowerCase().includes(q),
    );
  }

  return sortSnippets(list, sortBy.value);
});

function formatDate(dateStr: string) {
  return new Date(dateStr).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  });
}

function lineCount(code: string) {
  return code.split("\n").length;
}

function openCreate() {
  if (isMobile.value) {
    router.push("/snippets/create");
    return;
  }
  editingSnippet.value = null;
  dialogOpen.value = true;
}

onMounted(async () => {
  setSearch({ placeholder: "Search snippets..." });
  await Promise.all([
    snippetStore.fetchSnippets(),
    snippetStore.fetchRecentSnippets(),
  ]);
});

onUnmounted(() => clearSearch());
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <RyderPageHeader
        v-if="snippetStore.snippets.length > 0"
        title="Snippets"
        cta-text="New Snippet"
        cta-icon="heroicons:plus"
        @cta="openCreate"
      />
      <RyderFab
        v-if="snippetStore.snippets.length > 0 && filteredSnippets.length > 0"
        aria-label="New Snippet"
        @click="openCreate"
      />
      <!-- Language filter tabs + sort -->
      <div
        v-if="!snippetStore.loading && allLanguages.length > 1"
        class="flex items-center gap-2 mb-4 flex-wrap"
      >
        <div class="flex gap-2 flex-wrap flex-1">
          <button
            v-for="lang in allLanguages"
            :key="lang"
            class="px-3 py-1 rounded-full text-xs font-medium transition-colors"
            :class="
              activeLanguage === lang
                ? 'bg-primary-500 text-white'
                : 'bg-gray-100 dark:bg-gray-800 text-gray-600 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-700'
            "
            @click="activeLanguage = lang"
          >
            {{ lang }}
          </button>
        </div>
        <AppSortMenu v-model="sortBy" :options="SNIPPET_SORT_OPTIONS" />
      </div>

      <!-- Loading skeletons -->
      <div v-if="snippetStore.loading" class="flex flex-col gap-3">
        <USkeleton v-for="i in 3" :key="i" class="h-32 rounded-lg" />
      </div>

      <RyderEmptyState
        v-else-if="snippetStore.snippets.length === 0"
        title="No snippets yet"
        description="Save your first code snippet to get started."
        icon="heroicons:code-bracket"
        action-label="Create snippet"
        @action="openCreate"
      />

      <RyderEmptyState
        v-else-if="filteredSnippets.length === 0"
        title="No results found"
        description="Try a different search or language filter."
        icon="heroicons:funnel"
        action-label="Clear filters"
        @action="
          searchQuery = '';
          activeLanguage = 'All';
        "
      />

      <!-- Snippet list -->
      <div v-else class="flex flex-col gap-3">
        <SnippetCard
          v-for="snippet in filteredSnippets"
          :key="snippet.identifier"
          :identifier="snippet.identifier"
          :title="snippet.title ?? 'Untitled'"
          :language="snippet.language ?? 'Unknown'"
          :lines="lineCount(snippet.code)"
          :date="formatDate(snippet.createdAt)"
          :preview="snippet.code"
          :search-query="searchQuery"
        />
      </div>
    </template>

    <template #modal_content>
      <SnippetDialog v-model:open="dialogOpen" :snippet="editingSnippet" />
    </template>
  </NuxtLayout>
</template>
