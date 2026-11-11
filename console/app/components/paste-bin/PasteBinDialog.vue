<script setup lang="ts">
import { usePasteBinStore, type PasteBin } from "~/stores/paste-bin";
import { deriveTitle } from "~/utils/markdown";

const props = defineProps<{
  paste?: PasteBin | null;
}>();

const open = defineModel<boolean>("open", { required: true });

const pasteBinStore = usePasteBinStore();
const toast = useToast();

const form = reactive({
  title: "",
  content: "",
  isPinned: false,
});

const titleDirty = ref(false);
const submitting = ref(false);

function resetForm() {
  form.title = "";
  form.content = "";
  form.isPinned = false;
  titleDirty.value = false;
}

function populateFromPaste(paste: PasteBin) {
  const derived = deriveTitle(paste.content);
  titleDirty.value = !!paste.title && paste.title !== derived;
  form.title = paste.title ?? derived;
  form.content = paste.content;
  form.isPinned = paste.isPinned;
}

watch(
  [open, () => props.paste],
  ([isOpen, paste]) => {
    if (isOpen && paste) populateFromPaste(paste);
    else if (!isOpen) resetForm();
  },
  { immediate: true },
);

watch(
  () => form.content,
  (value) => {
    if (!titleDirty.value) form.title = deriveTitle(value);
  },
);

function handleTitleInput() {
  titleDirty.value = true;
}

async function handleSubmit() {
  if (!form.content.trim()) return;
  submitting.value = true;
  try {
    if (props.paste) {
      await pasteBinStore.updatePasteBin(props.paste.identifier, {
        title: form.title.trim() || null,
        content: form.content,
        isPinned: form.isPinned,
      });
      toast.add({
        color: "success",
        icon: "heroicons:check-circle",
        title: "Paste Updated",
        description: "Your changes have been saved",
      });
    }
    open.value = false;
    resetForm();
  } catch {
    toast.add({
      color: "error",
      icon: "heroicons:x-circle",
      title: "Something Went Wrong",
      description: "Failed to update paste",
    });
  } finally {
    submitting.value = false;
  }
}
</script>

<template>
  <RyderCreateDialog
    v-model:open="open"
    title="Edit Paste"
    :loading="submitting"
    submit-label="Save"
    :submit-disabled="!form.content.trim()"
    @submit="handleSubmit"
    @cancel="resetForm"
  >
    <div class="flex flex-col gap-4">
      <RyderInput
        v-model="form.title"
        label="Title"
        type="text"
        name="paste-title"
        placeholder="Untitled paste"
        :disabled="submitting"
        @input="handleTitleInput"
      />

      <div class="rounded-md border border-gray-200 dark:border-gray-700">
        <UEditor
          v-model="form.content"
          content-type="markdown"
          :placeholder="{
            placeholder: 'Paste or type anything...',
            mode: 'firstLine',
          }"
          :editable="!submitting"
          :ui="{ content: 'min-h-64 max-h-[50vh] overflow-y-auto' }"
        />
      </div>

      <div class="flex items-center justify-between">
        <label
          for="paste-pin"
          class="text-sm font-medium text-gray-700 dark:text-gray-300"
        >
          Pin this paste
        </label>
        <USwitch
          id="paste-pin"
          v-model="form.isPinned"
          :disabled="submitting"
        />
      </div>
    </div>
  </RyderCreateDialog>
</template>
