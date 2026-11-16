<script setup lang="ts">
import { SNIPPET_LANGUAGES } from "~/data/languages";

definePageMeta({ layout: false, name: "New Snippet" });

const snippetStore = useSnippetStore();
const toast = useToast();
const router = useRouter();

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

async function handleSubmit() {
  if (!form.code.trim()) return;
  submitting.value = true;
  try {
    await snippetStore.createSnippet({
      title: form.title.trim() || null,
      language: form.language.trim() || null,
      code: form.code,
      description: form.description.trim() || null,
    });
    toast.add({
      color: "success",
      icon: "heroicons:check-circle",
      title: "Snippet Created",
      description: "Your snippet has been saved",
    });
    resetForm();
    await router.push("/snippets");
  } catch {
    toast.add({
      color: "error",
      icon: "heroicons:x-circle",
      title: "Something Went Wrong",
      description: "Failed to create snippet",
    });
  } finally {
    submitting.value = false;
  }
}

function handleCancel() {
  resetForm();
  router.back();
}
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <div class="max-w-3xl">
        <div class="mb-6 flex items-center gap-3">
          <UButton
            icon="heroicons:arrow-left"
            color="neutral"
            variant="ghost"
            size="sm"
            @click="handleCancel"
          />
          <h1 class="text-lg font-semibold text-gray-800 dark:text-gray-100">
            New Snippet
          </h1>
        </div>

        <form class="flex flex-col gap-4" @submit.prevent="handleSubmit">
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
            <UButton type="submit" :loading="submitting" :disabled="!form.code.trim()">
              Create
            </UButton>
          </div>
        </form>
      </div>
    </template>
  </NuxtLayout>
</template>
