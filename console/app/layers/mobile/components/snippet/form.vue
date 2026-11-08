<script setup lang="ts">
import type { Snippet } from "@shared/stores/snippets";
import { useSnippetStore } from "@shared/stores/snippets";
import { SNIPPET_LANGUAGES } from "@shared/data/languages";

const props = defineProps<{
  snippet?: Snippet | null;
}>();

const emit = defineEmits<{
  saved: [];
}>();

const snippetStore = useSnippetStore();

const editing = computed(() => !!props.snippet);

const form = reactive({
  title: "",
  language: "",
  code: "",
  description: "",
});

const errors = reactive({ code: "" });
const loading = ref(false);
const submitError = ref("");

function resetForm() {
  form.title = "";
  form.language = "";
  form.code = "";
  form.description = "";
  errors.code = "";
  submitError.value = "";
}

watch(
  () => props.snippet,
  (snippet) => {
    if (!snippet) {
      resetForm();
      return;
    }

    form.title = snippet.title ?? "";
    form.language = snippet.language ?? "";
    form.code = snippet.code;
    form.description = snippet.description ?? "";
    errors.code = "";
    submitError.value = "";
  },
  { immediate: true },
);

async function handleSubmit() {
  errors.code = form.code.trim() ? "" : "Code is required";
  if (errors.code) return;

  loading.value = true;
  submitError.value = "";
  try {
    const payload = {
      title: form.title.trim() || null,
      language: form.language.trim() || null,
      code: form.code,
      description: form.description.trim() || null,
    };

    if (editing.value && props.snippet) {
      await snippetStore.updateSnippet(props.snippet.identifier, payload);
    } else {
      await snippetStore.createSnippet(payload);
    }

    emit("saved");
  } catch (e) {
    console.error(e);
    submitError.value = editing.value
      ? "Failed to update snippet. Please try again."
      : "Failed to create snippet. Please try again.";
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <form class="flex flex-col gap-4" @submit.prevent="handleSubmit">
    <p class="text-sm text-gray-500 dark:text-gray-400">
      {{
        editing
          ? "Update the details of this snippet."
          : "Save code you want to reuse later."
      }}
    </p>

    <div class="flex flex-col gap-4">
      <RyderInput
        v-model="form.title"
        label="Title"
        type="text"
        name="snippet-title"
        placeholder="My snippet"
        :disabled="loading"
      />

      <RyderSelect
        v-model="form.language"
        label="Language"
        name="snippet-language"
        placeholder="e.g. TypeScript"
        :items="SNIPPET_LANGUAGES"
        :disabled="loading"
      />

      <div>
        <div class="font-mono">
          <RyderTextarea
            v-model="form.code"
            label="Code"
            name="snippet-code"
            placeholder="Paste your code here..."
            :rows="12"
            :disabled="loading"
          />
        </div>
        <p v-if="errors.code" class="mt-1 text-xs text-red-500">
          {{ errors.code }}
        </p>
      </div>

      <RyderTextarea
        v-model="form.description"
        label="Description"
        name="snippet-description"
        placeholder="What does this snippet do?"
        :rows="3"
        :disabled="loading"
      />

      <p v-if="submitError" class="text-sm text-red-500">
        {{ submitError }}
      </p>
    </div>

    <RyderButton
      type="submit"
      size="md"
      block
      :loading="loading"
      :disabled="!form.code.trim() || loading"
    >
      {{ editing ? "Save changes" : "Create snippet" }}
    </RyderButton>
  </form>
</template>
