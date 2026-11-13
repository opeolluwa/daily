<script setup lang="ts">
import { safeOpenUrl as openUrl } from "~/utils/safe-open-url";
import { useBookmarkStore, type BookmarkTag } from "~/stores/bookmarks";
import {
  BOOKMARK_SORT_OPTIONS,
  sortBookmarks,
  type BookmarkSort,
} from "~/utils/sorting";

definePageMeta({ layout: false });

const bookmarkStore = useBookmarkStore();
const { searchQuery, setSearch, clearSearch } = useAppSearch();

const TAGS: { label: string; value: BookmarkTag | "all" }[] = [
  { label: "All", value: "all" },
  { label: "Development", value: "development" },
  { label: "Design", value: "design" },
  { label: "Research", value: "research" },
  { label: "Inspiration", value: "inspiration" },
];

const TAG_ICONS: Record<BookmarkTag, string> = {
  development: "heroicons:code-bracket",
  design: "heroicons:paint-brush",
  research: "heroicons:magnifying-glass",
  inspiration: "heroicons:light-bulb",
};

const activeTag = ref<BookmarkTag | "all">("all");
const showAddModal = ref(false);
const sortBy = ref<BookmarkSort>("date-newest");

const filtered = computed(() => {
  let list =
    activeTag.value === "all"
      ? bookmarkStore.bookmarks
      : bookmarkStore.byTag(activeTag.value);

  const q = searchQuery.value.trim().toLowerCase();
  if (q) {
    list = list.filter(
      (b) =>
        b.title.toLowerCase().includes(q) ||
        b.url.toLowerCase().includes(q) ||
        b.tag.toLowerCase().includes(q),
    );
  }

  return sortBookmarks(list, sortBy.value);
});

onMounted(async () => {
  await bookmarkStore.fetchBookmarks();

  setSearch({ placeholder: "Search bookmarks..." });
});

onUnmounted(() => clearSearch());

async function handleCreate(payload: {
  title: string;
  url: string;
  tag: BookmarkTag;
}) {
  await bookmarkStore.createBookmark(payload);
}
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <RyderPageHeader
        v-if="bookmarkStore.bookmarks.length > 0"
        title="Bookmarks"
        cta-text="Add Bookmark"
        cta-icon="heroicons:plus"
        @cta="showAddModal = true"
      />
      <RyderFab aria-label="Add Bookmark" @click="showAddModal = true" />
      <div class="flex items-center gap-2 mb-1">
        <BookmarkTagFilters
          v-if="bookmarkStore.bookmarks.length !== 0"
          v-model="activeTag"
          :tags="TAGS"
          class="flex-1"
        />
        <AppSortMenu
          v-if="bookmarkStore.bookmarks.length !== 0"
          v-model="sortBy"
          :options="BOOKMARK_SORT_OPTIONS"
        />
      </div>

      <!-- Loading -->
      <div
        v-if="bookmarkStore.loading"
        class="flex items-center justify-center py-16 text-gray-400"
      >
        <UIcon name="heroicons:arrow-path" class="size-5 animate-spin mr-2" />
        <span class="text-sm">Loading bookmarks…</span>
      </div>

      <RyderEmptyState
        v-else-if="bookmarkStore.bookmarks.length === 0"
        title="No bookmarks yet"
        description="Save links you want to revisit."
        icon="heroicons:bookmark"
        action-label="Create bookmark"
        @action="showAddModal = true"
      />

      <RyderEmptyState
        v-else-if="filtered.length === 0"
        title="No results found"
        description="Try a different search term or tag."
        icon="heroicons:magnifying-glass"
        action-label="Clear filters"
        @action="
          searchQuery = '';
          activeTag = 'all';
        "
      />

      <!-- Bookmark list -->
      <div v-else class="flex flex-col gap-3">
        <BookmarkCard
          v-for="bookmark in filtered"
          :key="bookmark.identifier"
          :bookmark="bookmark"
          @delete="bookmarkStore.deleteBookmark"
          @preview="openUrl(bookmark.url)"
        />
      </div>
    </template>

    <template #side_content>
      <BookmarkCollections
        v-model="activeTag"
        :tags="TAGS"
        :tag-icons="TAG_ICONS"
        :total-count="bookmarkStore.bookmarks.length"
        :tag-counts="bookmarkStore.tagCounts"
      />
    </template>

    <template #modal_content>
      <BookmarkAddModal
        v-model:open="showAddModal"
        :tags="TAGS.slice(1) as { label: string; value: BookmarkTag }[]"
        @create="handleCreate"
      />
    </template>
  </NuxtLayout>
</template>
