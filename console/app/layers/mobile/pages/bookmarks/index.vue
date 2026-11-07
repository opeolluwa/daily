<script setup lang="ts">
import { useBookmarkStore } from "@shared/stores/bookmarks";
import { safeOpenUrl as openUrl } from "@shared/utils/safe-open-url";
import BookmarkCard from "@mobile/components/bookmark/bookmark-card.vue";

definePageMeta({ name: "Bookmarks" });

const bookmarkStore = useBookmarkStore();
const router = useRouter();

function openCreate() {
  router.push("/bookmarks/create-bookmark");
}

onMounted(() => {
  bookmarkStore.fetchBookmarks();
});
</script>

<template>
  <RyderPullToRefresh @refresh="() => bookmarkStore.fetchBookmarks()">
    <div>
      <!-- Create bookmark FAB -->
      <RyderFab
        v-if="bookmarkStore.bookmarks.length !== 0"
        class="!bottom-24"
        @click="openCreate"
      />

      <!-- Loading -->
      <div v-if="bookmarkStore.loading" class="flex flex-col gap-3">
        <USkeleton v-for="i in 4" :key="i" class="h-24 rounded-lg" />
      </div>

      <!-- Empty state: no bookmarks at all -->
      <div v-else-if="bookmarkStore.bookmarks.length === 0">
        <RyderEmptyState
          title="No bookmarks yet"
          description="Create your first bookmark to get started."
          icon="ri:bookmark-line"
          action-label="create bookmark"
          @action="openCreate"
        />
      </div>

      <!-- Bookmark list -->
      <div v-else class="flex flex-col gap-3">
        <BookmarkCard
          v-for="bookmark in bookmarkStore.bookmarks"
          :key="bookmark.identifier"
          :bookmark="bookmark"
          @delete="(id) => bookmarkStore.deleteBookmark(id)"
          @preview="(bm) => openUrl(bm.url)"
        />
      </div>
    </div>
  </RyderPullToRefresh>
</template>
