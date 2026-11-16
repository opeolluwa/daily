<script setup lang="ts">
import _ from "lodash";
import type { BookmarkTag, CreateBookmarkPayload } from "~/stores/bookmarks";

definePageMeta({ layout: false, name: "New Bookmark" });

const router = useRouter();
const bookmarkStore = useBookmarkStore();
const toast = useToast();

const tags: { label: string; value: BookmarkTag }[] = [
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
const submitting = ref(false);

async function handleSubmit() {
  if (!form.title.trim() || !form.url.trim()) return;
  submitting.value = true;
  try {
    const payload: CreateBookmarkPayload = {
      title: form.title.trim(),
      url: form.url.trim(),
      tag: form.tag,
    };
    await bookmarkStore.createBookmark(payload);
    toast.add({ color: "success", icon: "heroicons:check-circle", title: "Bookmark Saved", description: "Your bookmark has been added" });
    await router.push("/bookmarks");
  } catch {
    toast.add({ color: "error", icon: "heroicons:x-circle", title: "Something Went Wrong", description: "Failed to save bookmark" });
  } finally {
    submitting.value = false;
  }
}
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <div class="max-w-3xl">
        <div class="mb-6 flex items-center gap-3">
          <h1 class="text-lg font-semibold text-gray-800 dark:text-gray-100">Add Bookmark</h1>
        </div>

        <form class="flex flex-col gap-4" @submit.prevent="handleSubmit">
          <RyderInput v-model="form.title" placeholder="Bookmark title" label="Title" name="bookmark title" :disabled="submitting" />
          <RyderInput v-model="form.url" label="URL" name="bookmark url" placeholder="https://example.com" :disabled="submitting" />
          <RyderSelect
            v-model="form.tag"
            :items="tags.map((t) => ({ label: _.capitalize(t.label), value: t.value }))"
            value-key="value"
            label="Tag"
            name="bookmark tag"
            :disabled="submitting"
          />
          <div class="flex justify-end gap-2 pt-2">
            <UButton type="button" color="neutral" variant="ghost" :disabled="submitting" @click="router.back()">Cancel</UButton>
            <UButton type="submit" :loading="submitting" :disabled="!form.title.trim() || !form.url.trim()">Save</UButton>
          </div>
        </form>
      </div>
    </template>
  </NuxtLayout>
</template>
