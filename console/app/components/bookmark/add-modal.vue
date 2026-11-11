<script setup lang="ts">
import type {
  BookmarkTag,
  CreateBookmarkPayload,
} from "~/stores/bookmarks";
import _ from "lodash";

defineProps<{
  tags: { label: string; value: BookmarkTag }[];
}>();

const open = defineModel<boolean>("open", { required: true });

const emit = defineEmits<{ create: [payload: CreateBookmarkPayload] }>();

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
    emit("create", {
      title: form.title.trim(),
      url: form.url.trim(),
      tag: form.tag,
    });
    form.title = "";
    form.url = "";
    form.tag = "development";
    open.value = false;
  } finally {
    submitting.value = false;
  }
}
</script>

<template>
  <RyderCreateDialog
    v-model:open="open"
    title="Add Bookmark"
    :loading="submitting"
    submit-label="Save"
    :submit-disabled="!form.title.trim() || !form.url.trim()"
    @submit="handleSubmit"
    @cancel="open = false"
  >
    <div class="flex flex-col gap-4">
      <RyderInput
        v-model="form.title"
        placeholder="Bookmark title"
        label="Title"
        name="bookmark title"
        :disabled="submitting"
      />

      <RyderInput
        v-model="form.url"
        label="URL"
        name="bookmark url"
        placeholder="https://example.com"
        :disabled="submitting"
      />

      <RyderSelect
        v-model="form.tag"
        :items="
          tags.map((t) => ({ label: _.capitalize(t.label), value: t.value }))
        "
        value-key="value"
        label="Tag"
        name="bookmark tag"
        :disabled="submitting"
      />
    </div>
  </RyderCreateDialog>
</template>
