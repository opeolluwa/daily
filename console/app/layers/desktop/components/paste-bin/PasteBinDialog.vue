<script setup lang="ts">
import type { PasteBin } from "@shared/stores/paste-bin";

const props = defineProps<{
  paste?: PasteBin | null;
}>();

const open = defineModel<boolean>("open", { required: true });

const pasteBinStore = usePasteBinStore();
const toast = useToast();

const form = reactive({
  content: "",
  isPinned: false,
});

const submitting = ref(false);

function deriveTitle(text: string) {
  const firstLine = text
    .split("\n")
    .map((line) => line.trim())
    .find(Boolean);
  if (!firstLine) return "";
  return firstLine.replace(/^#+\s*/, "").slice(0, 60);
}

function resetForm() {
  form.content = "";
  form.isPinned = false;
}

watch(
  () => props.paste,
  (p) => {
    if (p) {
      form.content = p.content;
      form.isPinned = p.isPinned;
    } else {
      resetForm();
    }
  },
  { immediate: true },
);

watch(open, (val) => {
  if (!val) resetForm();
});

async function handleSubmit() {
  if (!form.content.trim()) return;
  submitting.value = true;
  try {
    if (props.paste) {
      await pasteBinStore.updatePasteBin(props.paste.identifier, {
        title: deriveTitle(form.content),
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
      <div class="font-mono">
        <RyderTextarea
          v-model="form.content"
          label="Content"
          name="paste-content"
          placeholder="Paste content..."
          :rows="12"
          :disabled="submitting"
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
