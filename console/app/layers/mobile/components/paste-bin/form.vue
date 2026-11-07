<script setup lang="ts">
import { usePasteBinStore, type PasteBin } from "@shared/stores/paste-bin";

const props = defineProps<{
  paste?: PasteBin | null;
}>();

const emit = defineEmits<{
  saved: [];
}>();

const pasteBinStore = usePasteBinStore();

const title = ref("");
const content = ref("");
const isPinned = ref(false);
const titleDirty = ref(false);
const loading = ref(false);
const submitError = ref("");

const editing = computed(() => !!props.paste);

function deriveTitle(text: string) {
  const firstLine = text
    .split("\n")
    .map((line) => line.trim())
    .find(Boolean);
  if (!firstLine) return "";
  return firstLine.replace(/^#+\s*/, "").slice(0, 60);
}

watch(
  () => props.paste,
  (paste) => {
    if (!paste) return;
    titleDirty.value =
      !!paste.title && paste.title !== deriveTitle(paste.content);
    title.value = paste.title ?? "";
    content.value = paste.content;
    isPinned.value = paste.isPinned;
    submitError.value = "";
  },
  { immediate: true },
);

watch(content, (value) => {
  if (!titleDirty.value) title.value = deriveTitle(value);
});

function handleTitleInput() {
  titleDirty.value = true;
}

async function handleSubmit() {
  if (!content.value.trim()) {
    submitError.value = "Add some content before saving.";
    return;
  }

  loading.value = true;
  submitError.value = "";
  try {
    if (editing.value && props.paste) {
      await pasteBinStore.updatePasteBin(props.paste.identifier, {
        title: title.value.trim(),
        content: content.value,
        isPinned: isPinned.value,
      });
    } else {
      await pasteBinStore.createPasteBin({
        title: title.value.trim() || null,
        content: content.value,
        isPinned: isPinned.value,
      });
    }
    emit("saved");
  } catch (error) {
    console.error("[paste-bin] failed to save", error);
    submitError.value = "Failed to save paste. Please try again.";
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <form class="flex flex-col gap-4" @submit.prevent="handleSubmit">
    <p class="text-sm text-gray-500 dark:text-gray-400">
      Save text, links, or anything you want to keep handy.
    </p>

    <div class="flex flex-col gap-4">
      <RyderInput
        v-model="title"
        label="Title"
        type="text"
        name="paste-title"
        placeholder="Untitled paste"
        :disabled="loading"
        @input="handleTitleInput"
      />

      <div class="font-mono">
        <RyderTextarea
          v-model="content"
          label="Content"
          name="paste-content"
          placeholder="Paste or type anything..."
          :rows="10"
          :disabled="loading"
        />
      </div>

      <label class="flex items-center justify-between py-1">
        <span class="text-sm text-gray-700 dark:text-gray-300">
          Pin this paste
        </span>
        <USwitch v-model="isPinned" :disabled="loading" />
      </label>

      <p v-if="submitError" class="text-xs text-red-500">
        {{ submitError }}
      </p>
    </div>


      <RyderButton
        type="submit"
        size="md"
        block
        :loading="loading"
        :disabled="!content.trim() || loading"
      >
        {{ editing ? "Save changes" : "Create paste" }}
      </RyderButton>
  </form>
</template>
