<script setup lang="ts">
import BookmarkForm from "@mobile/components/bookmark/form.vue";

definePageMeta({ name: "New bookmark" });

const router = useRouter();
const { notify } = useAppNotification();

function handleSaved() {
  notify({ message: "Bookmark created", type: "success" });
  router.replace("/bookmarks");
}
</script>

<template>
  <BookmarkForm @saved="handleSaved" />
</template>
