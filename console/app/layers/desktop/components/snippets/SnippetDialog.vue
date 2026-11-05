<script setup lang="ts">
import { SNIPPET_LANGUAGES } from "@shared/data/languages";

const props = defineProps<{
  snippet?: {
    identifier: string;
    title: string | null;
    language: string | null;
    code: string;
    description: string | null;
    createdAt: string;
  } | null;
}>();

const open = defineModel<boolean>("open", { required: true });

const snippetStore = useSnippetStore();
const { notify } = useAppNotification();

const editing = computed(() => !!props.snippet);

const form = reactive({
  title: "",
  language: "",
  code: "",
  description: "",
});

const submitting = ref(false);

function resetForm() {
  form.title = "";
  form.language = "";
  form.code = "";
  form.description = "";
}

watch(
  () => props.snippet,
  (s) => {
    if (s) {
      form.title = s.title ?? "";
      form.language = s.language ?? "";
      form.code = s.code;
      form.description = s.description ?? "";
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
  if (!form.code.trim()) return;
  submitting.value = true;
  try {
    const payload = {
      title: form.title.trim() || null,
      language: form.language.trim() || null,
      code: form.code,
      description: form.description.trim() || null,
    };

    if (editing.value && props.snippet) {
      await snippetStore.updateSnippet(props.snippet.identifier, payload);
      notify({ type: "success", message: "Snippet updated" });
    } else {
      await snippetStore.createSnippet(payload);
      notify({ type: "success", message: "Snippet created" });
    }
    open.value = false;
    resetForm();
  } catch {
    notify({
      type: "error",
      message: editing.value
        ? "Failed to update snippet"
        : "Failed to create snippet",
    });
  } finally {
    submitting.value = false;
  }
}
</script>

<template>
  <RyderCreateDialog
    v-model:open="open"
    :title="editing ? 'Edit Snippet' : 'New Snippet'"
    :loading="submitting"
    :submit-label="editing ? 'Save' : 'Create'"
    :submit-disabled="!form.code.trim()"
    @submit="handleSubmit"
    @cancel="resetForm"
  >
    <div class="flex flex-col gap-4">
      <RyderInput
        v-model="form.title"
        hint="(optional)"
        placeholder="My snippet"
        label="Title"
        name="snippet title"
        :disabled="submitting"
      />

      <RyderSelect
        v-model="form.language"
        name="language"
        label="Language"
        hint="optional"
        placeholder="e.g. TypeScript"
        :items="SNIPPET_LANGUAGES"
        :disabled="submitting"
      />

      <div class="font-mono">
        <RyderTextarea
          v-model="form.code"
          label="Code"
          name="snippet-code"
          placeholder="Paste your code here..."
          :rows="12"
          :disabled="submitting"
        />
      </div>

      <RyderTextarea
        v-model="form.description"
        label="Description"
        hint="optional"
        name="snippet-description"
        placeholder="What does this snippet do?"
        :rows="3"
        :disabled="submitting"
      />
    </div>
  </RyderCreateDialog>
</template>
