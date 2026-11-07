<script setup lang="ts">
import type { Snippet } from "@shared/stores/snippets";
import { useSnippetStore } from "@shared/stores/snippets";
import { useWorkspacesStore } from "@shared/stores/workspaces";
import { highlightSnippet } from "@shared/utils/snippet-highlight";
import MetaControls from "@mobile/components/meta/meta-controls.vue";

const { snippet } = defineProps<{ snippet: Snippet }>();

const emit = defineEmits<{
  delete: [identifier: string];
}>();

const router = useRouter();
const snippetStore = useSnippetStore();
const workspaceStore = useWorkspacesStore();
const { notify } = useAppNotification();

const currentWorkspaceId = computed(() => workspaceStore.activeWorkspaceId);
const copied = ref(false);

const displayTitle = computed(() => snippet.title?.trim() || "Untitled");

const lineCount = computed(() => snippet.code.split("\n").length);

const formattedDate = computed(() =>
  new Date(snippet.createdAt).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  }),
);

const previewCode = computed(() =>
  snippet.code.split("\n").slice(0, 6).join("\n"),
);

const highlightedPreview = computed(() =>
  highlightSnippet(previewCode.value, snippet.language),
);

async function copyCode() {
  await navigator.clipboard.writeText(snippet.code);
  copied.value = true;
  setTimeout(() => (copied.value = false), 1500);
  notify({ message: "Copied to clipboard", type: "success" });
}

function openView() {
  router.push(`/snippets/view-snippet?id=${snippet.identifier}`);
}

function openEdit() {
  router.push(`/snippets/edit-snippet?id=${snippet.identifier}`);
}

const handleDuplicate = async (targetWorkspaceId: string) => {
  await snippetStore.duplicateSnippet(
    snippet.identifier,
    currentWorkspaceId.value,
    targetWorkspaceId,
  );
};

const handleTransfer = async (targetWorkspaceId: string) => {
  await snippetStore.transferSnippet(
    snippet.identifier,
    currentWorkspaceId.value,
    targetWorkspaceId,
  );
};
</script>

<template>
  <RyderCard
    :ui="{
      root: 'cursor-pointer overflow-hidden transition-shadow hover:shadow-sm !rounded-lg !border-gray-100 dark:!border-gray-700 dark:!bg-gray-800 !p-0',
    }"
    @click="openView"
  >
    <div class="p-4">
      <div class="mb-2 flex items-center justify-between gap-2">
        <h3
          class="truncate text-sm font-medium text-gray-800 dark:text-gray-200"
        >
          {{ displayTitle }}
        </h3>
        <span
          v-if="snippet.language"
          class="shrink-0 rounded bg-gray-100 px-2 py-0.5 text-xs text-gray-500 dark:bg-gray-700 dark:text-gray-400"
        >
          {{ snippet.language }}
        </span>
      </div>

      <!-- eslint-disable-next-line vue/no-v-html -->
      <pre
        class="max-h-32 overflow-x-auto rounded-md text-xs"
      ><code class="hljs" v-html="highlightedPreview" /></pre>

      <p class="mt-2 text-xs text-gray-400">
        {{ lineCount }} {{ lineCount === 1 ? "line" : "lines" }}
      </p>
    </div>

    <div
      class="flex items-center justify-between border-t border-gray-50 px-4 py-2 dark:border-gray-700"
      @click.stop
    >
      <p class="text-xs text-gray-400">{{ formattedDate }}</p>

      <div class="flex items-center gap-2">
        <button
          class="rounded p-1 text-xs font-medium transition-colors"
          :class="
            copied ? 'text-green-500' : 'text-primary-600 dark:text-primary-400'
          "
          @click="copyCode"
        >
          {{ copied ? "Copied!" : "Copy" }}
        </button>
        <MetaControls
          item-name="snippet"
          @edit-record="openEdit"
          @delete-record="emit('delete', snippet.identifier)"
          @duplicate-record="
            (targetWorkspaceId) => handleDuplicate(targetWorkspaceId)
          "
          @transfer-record="
            (targetWorkspaceId) => handleTransfer(targetWorkspaceId)
          "
        />
      </div>
    </div>
  </RyderCard>
</template>
