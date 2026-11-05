<script setup lang="ts">
import { kSheet } from "konsta/vue";
import { usePasteBinStore, type PasteBin } from "@shared/stores/paste-bin";

const props = defineProps<{
  paste?: PasteBin | null;
}>();

const open = defineModel<boolean>("open", { required: true });

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

function resetForm() {
  title.value = "";
  content.value = "";
  isPinned.value = false;
  titleDirty.value = false;
  submitError.value = "";
}

watch(
  [() => props.paste, open],
  ([paste, isOpen]) => {
    if (!isOpen) return;
    if (paste) {
      title.value = paste.title ?? "";
      content.value = paste.content;
      isPinned.value = paste.isPinned;
      titleDirty.value =
        !!paste.title && paste.title !== deriveTitle(paste.content);
    } else {
      resetForm();
    }
  },
  { immediate: true },
);

watch(content, (value) => {
  if (!titleDirty.value) title.value = deriveTitle(value);
});

function requestClose() {
  if (loading.value) return;
  open.value = false;
  resetForm();
}

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
    open.value = false;
    resetForm();
  } catch (error) {
    console.error("[paste-bin] failed to save", error);
    submitError.value = "Failed to save paste. Please try again.";
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <kSheet :opened="open" @backdropclick="requestClose">
    <div
      class="flex max-h-[85dvh] flex-col overflow-hidden rounded-t-[20px] bg-white pb-1 dark:bg-app-dark-800"
      style="padding-bottom: env(safe-area-inset-bottom, 0px)"
    >
      <div
        class="mx-auto my-3 h-1 w-9 shrink-0 rounded-full bg-gray-300 dark:bg-gray-600"
      />

      <div class="shrink-0 px-4 pb-5">
        <div class="flex items-center justify-between">
          <h2
            class="text-lg font-semibold leading-6 text-gray-900 dark:text-white"
          >
            {{ editing ? "Edit Paste" : "New Paste" }}
          </h2>
          <UButton
            size="md"
            color="neutral"
            variant="ghost"
            icon="heroicons:x-mark"
            aria-label="Close"
            :disabled="loading"
            class="-mr-2"
            @click="requestClose"
          />
        </div>
        <p class="pt-1 text-sm text-gray-500 dark:text-gray-400">
          Save text, links, or anything you want to keep handy.
        </p>
      </div>

      <form class="flex min-h-0 flex-1 flex-col" @submit.prevent="handleSubmit">
        <div
          class="flex flex-1 flex-col gap-4 overflow-y-auto px-4 pb-4"
          style="-webkit-overflow-scrolling: touch"
        >
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

        <div
          class="shrink-0 border-t border-gray-100 px-4 py-3 dark:border-gray-700"
        >
          <RyderButton
            type="submit"
            size="md"
            block
            :loading="loading"
            :disabled="!content.trim() || loading"
          >
            {{ editing ? "Save changes" : "Create paste" }}
          </RyderButton>
        </div>
      </form>
    </div>
  </kSheet>
</template>
