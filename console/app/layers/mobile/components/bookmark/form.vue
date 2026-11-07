<script setup lang="ts">
import {
  useBookmarkStore,
  type BookmarkTag,
} from "@shared/stores/bookmarks";

const emit = defineEmits<{
  saved: [];
}>();

const bookmarkStore = useBookmarkStore();

const TAGS: { label: string; value: BookmarkTag }[] = [
  { label: "Development", value: "development" },
  { label: "Design", value: "design" },
  { label: "Research", value: "research" },
  { label: "Inspiration", value: "inspiration" },
];

const form = reactive({
  title: "",
  url: "",
  tag: "development" as BookmarkTag,
});
const errors = reactive({ title: "", url: "" });
const loading = ref(false);
const submitError = ref("");

async function handleSubmit() {
  errors.title = form.title.trim() ? "" : "Title is required";
  errors.url = form.url.trim() ? "" : "URL is required";
  if (errors.title || errors.url) return;
  loading.value = true;
  submitError.value = "";
  try {
    await bookmarkStore.createBookmark({
      title: form.title.trim(),
      url: form.url.trim(),
      tag: form.tag,
    });
    emit("saved");
  } catch (e) {
    console.error(e);
    submitError.value = "Failed to save bookmark. Please try again.";
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <form class="flex flex-col gap-4" @submit.prevent="handleSubmit">
    <p class="text-sm text-gray-500 dark:text-gray-400">
      Save links you want to keep track of.
    </p>

    <div class="flex flex-col gap-4">
      <div>
        <RyderInput
          v-model="form.title"
          label="Title"
          type="text"
          name="bookmark-title"
          placeholder="Bookmark title"
          :disabled="loading"
        />
        <p v-if="errors.title" class="mt-1 text-xs text-red-500">
          {{ errors.title }}
        </p>
      </div>

      <div>
        <RyderInput
          v-model="form.url"
          label="URL"
          type="text"
          name="bookmark-url"
          placeholder="https://example.com"
          :disabled="loading"
        />
        <p v-if="errors.url" class="mt-1 text-xs text-red-500">
          {{ errors.url }}
        </p>
      </div>

      <div class="flex flex-col gap-1.5">
        <label class="text-xs font-medium text-gray-600 dark:text-gray-400">
          Tag
        </label>
        <URadioGroup
          v-model="form.tag"
          :items="TAGS"
          variant="card"
          orientation="horizontal"
          size="sm"
          :disabled="loading"
        />
      </div>

      <p v-if="submitError" class="text-sm text-red-500">
        {{ submitError }}
      </p>
    </div>

    <div
      class="sticky bottom-0 -mx-6 border-t border-gray-100 bg-white/95 px-6 py-3 backdrop-blur dark:border-gray-700 dark:bg-app-dark-800/95"
    >
      <RyderButton
        type="submit"
        size="md"
        block
        :loading="loading"
        :disabled="loading"
      >
        Create bookmark
      </RyderButton>
    </div>
  </form>
</template>
