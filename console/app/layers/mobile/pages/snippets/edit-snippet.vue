<script setup lang="ts">
import SnippetForm from "@mobile/components/snippet/form.vue";
import { useSnippetStore } from "@shared/stores/snippets";

definePageMeta({ name: "Edit snippet" });

const route = useRoute();
const router = useRouter();
const snippetStore = useSnippetStore();
const { notify } = useAppNotification();

const id = computed(() =>
  typeof route.query.id === "string" ? route.query.id : undefined,
);

const snippet = computed(
  () =>
    snippetStore.snippets.find((entry) => entry.identifier === id.value) ??
    null,
);

function handleSaved() {
  notify({ message: "Snippet updated", type: "success" });
  router.replace("/snippets");
}

onMounted(async () => {
  if (snippetStore.snippets.length === 0) {
    await snippetStore.fetchSnippets();
  }
});
</script>

<template>
  <div>
    <div v-if="snippetStore.loading && !snippet" class="flex flex-col gap-4">
      <USkeleton class="h-16 rounded-lg" />
      <USkeleton class="h-32 rounded-lg" />
      <USkeleton class="h-10 rounded-lg" />
    </div>

    <div
      v-else-if="!snippet"
      class="flex flex-col items-center justify-center py-20 text-center"
    >
      <div class="mb-4 p-3 rounded-full bg-gray-100 dark:bg-gray-800">
        <UIcon name="heroicons:code-bracket" class="size-7 text-gray-400" />
      </div>
      <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
        Snippet not found
      </h3>
      <button
        class="text-xs text-primary-500 hover:text-primary-600 font-medium mt-2"
        @click="router.push('/snippets')"
      >
        Back to snippets
      </button>
    </div>

    <SnippetForm v-else :snippet="snippet" @saved="handleSaved" />
  </div>
</template>
