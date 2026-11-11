<script setup lang="ts">
import { computed, ref } from "vue";
import { useWorkspacesStore } from "@shared/stores/workspaces";
import { useWhiteboardStore } from "@shared/stores/whiteboards";
import type { Whiteboard } from "@shared/stores/whiteboards";
import { isValidPersistedScene } from "@features/persistence/sceneStorage";

const props = defineProps<{
  whiteboard: Whiteboard;
  searchQuery?: string;
}>();

const router = useRouter();
const whiteboardStore = useWhiteboardStore();
const workspaceStore = useWorkspacesStore();
const currentWorkspaceId = computed(() => workspaceStore.activeWorkspaceId);

const title = computed(() => props.whiteboard.title ?? "Untitled");

const elementCount = computed(() => {
  const payload = props.whiteboard.payload;
  return isValidPersistedScene(payload) ? payload.elementCount : 0;
});

const formattedDate = computed(() =>
  new Date(props.whiteboard.updatedAt).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  }),
);

const titleMatches = computed(() => {
  const q = props.searchQuery?.trim().toLowerCase();
  return !!q && title.value.toLowerCase().includes(q);
});

async function confirmDelete() {
  await whiteboardStore.deleteWhiteboard(props.whiteboard.identifier);
}

async function handleTransfer(targetWorkspaceId: string) {
  await whiteboardStore.transferWhiteboard(
    props.whiteboard.identifier,
    currentWorkspaceId.value,
    targetWorkspaceId,
  );
}

// ── Rename modal ──────────────────────────────────────────────────────
const renameOpen = ref(false);
const renameValue = ref("");
const renameSubmitting = ref(false);

function openRename() {
  renameValue.value = props.whiteboard.title ?? "";
  renameOpen.value = true;
}

async function submitRename() {
  if (renameSubmitting.value) return;
  renameSubmitting.value = true;
  try {
    await whiteboardStore.updateWhiteboard(props.whiteboard.identifier, {
      title: renameValue.value.trim() || null,
    });
    renameOpen.value = false;
  } finally {
    renameSubmitting.value = false;
  }
}
</script>

<template>
  <RyderCard
    :ui="{
      root: '!p-0 overflow-hidden cursor-pointer hover:shadow-sm transition-shadow',
    }"
    @click="router.push(`/whiteboard/${whiteboard.identifier}`)"
  >
    <div class="p-4 flex items-center gap-4">
      <div
        class="shrink-0 size-10 rounded-lg bg-primary-100 dark:bg-primary-900 flex items-center justify-center"
      >
        <UIcon
          name="i-lucide-pen-tool"
          class="size-5 text-primary-600 dark:text-primary-400"
        />
      </div>

      <div class="min-w-0 flex-1">
        <h3 class="text-sm font-medium text-gray-800 dark:text-gray-200 truncate">
          {{ title }}
          <span
            v-if="titleMatches"
            class="ml-1 px-2 py-0.5 rounded bg-primary-100 dark:bg-primary-900 text-xs text-primary-600 dark:text-primary-400"
            >title match</span
          >
        </h3>
        <p class="text-xs text-gray-400 mt-0.5">
          {{ elementCount }} elements · updated {{ formattedDate }}
        </p>
      </div>
    </div>

    <div
      class="px-4 py-2 border-t border-gray-50 dark:border-gray-700 flex items-center justify-between"
    >
      <p class="text-xs text-gray-400">Created {{ formattedDate }}</p>
      <div class="flex items-center gap-2" @click.stop>
        <button
          class="text-xs font-medium text-primary-600 dark:text-primary-400 hover:text-primary-700 transition-colors"
          @click="openRename"
        >
          Rename
        </button>
        <MetaControls
          item-name="whiteboard"
          :show-duplicate="false"
          :show-edit="false"
          @delete-record="confirmDelete"
          @transfer-record="(targetWorkspaceId) => handleTransfer(targetWorkspaceId)"
        />
      </div>
    </div>
  </RyderCard>

  <UModal v-model:open="renameOpen">
    <template #content>
      <div class="px-6 pt-6 pb-2 flex flex-col gap-1">
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
          Rename Whiteboard
        </h2>
        <UInput
          v-model="renameValue"
          placeholder="Whiteboard name"
          class="w-full mt-4"
          @keydown.enter="submitRename"
        />
        <div class="flex justify-end gap-2 mt-4">
          <UButton size="sm" variant="ghost" @click="renameOpen = false">
            Cancel
          </UButton>
          <UButton
            size="sm"
            :disabled="renameSubmitting"
            @click="submitRename"
          >
            Save
          </UButton>
        </div>
      </div>
    </template>
  </UModal>
</template>
