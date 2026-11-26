<script setup lang="ts">
import { SNIPPET_LANGUAGES } from "~/data/languages";

definePageMeta({ layout: false, name: "Edit Snippet" });

const route = useRoute();
const router = useRouter();
const snippetStore = useSnippetStore();
const toast = useToast();

const id = computed(() => route.query.id as string | undefined);
const snippet = computed(
  () => snippetStore.snippets.find((s) => s.identifier === id.value) ?? null,
);

const form = reactive({
  title: "",
  language: "",
  code: "",
  description: "",
});

const submitting = ref(false);
const loaded = ref(false);

function hydrate() {
  if (!snippet.value) return;
  form.title = snippet.value.title ?? "";
  form.language = snippet.value.language ?? "";
  form.code = snippet.value.code;
  form.description = snippet.value.description ?? "";
  loaded.value = true;
}

watch(snippet, () => hydrate(), { immediate: true });

onMounted(async () => {
  if (snippetStore.snippets.length === 0) {
    await snippetStore.fetchSnippets();
  }
  hydrate();
});

async function handleSubmit() {
  if (!snippet.value || !form.code.trim()) return;
  submitting.value = true;
  try {
    await snippetStore.updateSnippet(snippet.value.identifier, {
      title: form.title.trim() || null,
      language: form.language.trim() || null,
      code: form.code,
      description: form.description.trim() || null,
    });
    toast.add({
      color: "success",
      icon: "heroicons:check-circle",
      title: "Snippet Updated",
      description: "Your changes have been saved",
    });
    await router.push("/snippets");
  } catch {
    toast.add({
      color: "error",
      icon: "heroicons:x-circle",
      title: "Something Went Wrong",
      description: "Failed to update snippet",
    });
  } finally {
    submitting.value = false;
  }
}

function handleCancel() {
  router.back();
}
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <div class="max-w-3xl">
        <div class="mb-6 flex items-center gap-3">
          <h1 class="text-lg font-semibold text-gray-800 dark:text-gray-100">
            Edit Snippet
          </h1>
        </div>

        <div
          v-if="!snippet"
          class="flex flex-col items-center justify-center py-20 text-center"
        >
          <UIcon
            name="heroicons:code-bracket"
            class="size-7 text-gray-400 mb-3"
          />
          <p class="text-sm text-gray-600 dark:text-gray-400 mb-3">
            Snippet not found
          </p>
          <UButton size="sm" variant="outline" @click="router.push('/snippets')"
            >Back to snippets</UButton
          >
        </div>

        <form v-else class="flex flex-col gap-4" @submit.prevent="handleSubmit">
          <RyderInput
            v-model="form.title"
            placeholder="My snippet"
            label="Title"
            name="snippet title"
            :disabled="submitting"
          />

          <RyderSelect
            v-model="form.language"
            name="language"
            label="Language"
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
            name="snippet-description"
            placeholder="What does this snippet do?"
            :rows="3"
            :disabled="submitting"
          />

          <div class="flex justify-end gap-2 pt-2">
            <UButton
              type="button"
              color="neutral"
              variant="ghost"
              :disabled="submitting"
              @click="handleCancel"
            >
              Cancel
            </UButton>
            <UButton
              type="submit"
              :loading="submitting"
              :disabled="!form.code.trim()"
            >
              Save
            </UButton>
          </div>
        </form>
      </div>
    </template>
  </NuxtLayout>
</template>
