<script setup lang="ts">
import { useSnippetStore } from "@shared/stores/snippets";
import { highlightSnippet } from "@shared/utils/snippet-highlight";

definePageMeta({ name: "View snippet" });

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

const copied = ref(false);
const deleting = ref(false);
const showDeleteConfirm = ref(false);

const highlighted = computed(() => {
  if (!snippet.value) return "";
  return highlightSnippet(snippet.value.code, snippet.value.language);
});

const lineCount = computed(() => snippet.value?.code.split("\n").length ?? 0);

const formattedCreatedAt = computed(() =>
  snippet.value
    ? new Date(snippet.value.createdAt).toLocaleDateString("en-US", {
        month: "short",
        day: "numeric",
        year: "numeric",
      })
    : "",
);

async function copyCode() {
  if (!snippet.value) return;
  await navigator.clipboard.writeText(snippet.value.code);
  copied.value = true;
  setTimeout(() => (copied.value = false), 1500);
  notify({ message: "Copied to clipboard", type: "success" });
}

function openEdit() {
  if (!snippet.value) return;
  router.push(`/snippets/edit-snippet?id=${snippet.value.identifier}`);
}

async function confirmDelete() {
  if (!snippet.value) return;
  deleting.value = true;
  try {
    await snippetStore.deleteSnippet(snippet.value.identifier);
    notify({ message: "Snippet moved to recycle bin", type: "success" });
    router.replace("/snippets");
  } catch (error) {
    console.error(error);
    notify({ message: "Failed to delete snippet", type: "error" });
  } finally {
    deleting.value = false;
    showDeleteConfirm.value = false;
  }
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
      <USkeleton class="h-10 w-40 rounded-lg" />
      <USkeleton class="h-16 rounded-lg" />
      <USkeleton class="h-40 rounded-lg" />
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

    <template v-else>
      <!-- Actions -->
      <div class="mb-5 flex items-center gap-2">
        <RyderButton size="sm" variant="ghost" @click="openEdit">
          <UIcon name="heroicons:pencil-square" class="size-4" />
          Edit
        </RyderButton>
        <RyderButton
          size="sm"
          color="error"
          variant="ghost"
          :disabled="deleting"
          @click="showDeleteConfirm = true"
        >
          <UIcon name="heroicons:trash" class="size-4" />
          Delete
        </RyderButton>
      </div>

      <!-- Title & meta -->
      <div class="mb-4">
        <h1 class="mb-1 text-lg font-semibold text-gray-800 dark:text-gray-100">
          {{ snippet.title ?? "Untitled" }}
        </h1>
        <div class="flex flex-wrap items-center gap-3">
          <span
            v-if="snippet.language"
            class="rounded bg-gray-100 px-2 py-0.5 text-xs text-gray-500 dark:bg-gray-700 dark:text-gray-400"
          >
            {{ snippet.language }}
          </span>
          <span class="text-xs text-gray-400">
            {{ lineCount }} {{ lineCount === 1 ? "line" : "lines" }}
          </span>
          <span class="text-xs text-gray-400">
            {{ formattedCreatedAt }}
          </span>
        </div>
      </div>

      <!-- Description -->
      <p
        v-if="snippet.description"
        class="mb-4 text-sm text-gray-500 dark:text-gray-400"
      >
        {{ snippet.description }}
      </p>

      <!-- Code block -->
      <div class="relative">
        <!-- eslint-disable-next-line vue/no-v-html -->
        <pre
          class="overflow-x-auto rounded-md text-xs"
        ><code class="hljs" v-html="highlighted" /></pre>
        <button
          class="absolute right-2 top-2 rounded px-2 py-1 text-xs font-medium transition-colors"
          :class="
            copied
              ? 'bg-green-600 text-white'
              : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
          "
          @click="copyCode"
        >
          {{ copied ? "Copied!" : "Copy" }}
        </button>
      </div>

      <!-- Delete confirmation -->
      <UModal v-model:open="showDeleteConfirm">
        <template #content>
          <div class="flex flex-col gap-5 px-6 py-6">
            <div class="flex items-start gap-4">
              <div
                class="flex size-10 shrink-0 items-center justify-center rounded-full bg-red-100 dark:bg-red-950"
              >
                <UIcon
                  name="heroicons:trash"
                  class="size-5 text-red-600 dark:text-red-400"
                />
              </div>
              <div class="flex flex-col gap-1">
                <h2
                  class="text-base font-semibold text-gray-900 dark:text-white"
                >
                  Delete snippet
                </h2>
                <p class="text-sm text-gray-500 dark:text-gray-400">
                  Are you sure you want to delete this snippet? This action
                  cannot be undone.
                </p>
              </div>
            </div>

            <div class="flex justify-end gap-2">
              <RyderButton
                size="sm"
                variant="ghost"
                :disabled="deleting"
                @click="showDeleteConfirm = false"
              >
                Cancel
              </RyderButton>
              <RyderButton
                size="sm"
                color="error"
                :loading="deleting"
                @click="confirmDelete"
              >
                Delete
              </RyderButton>
            </div>
          </div>
        </template>
      </UModal>
    </template>
  </div>
</template>
