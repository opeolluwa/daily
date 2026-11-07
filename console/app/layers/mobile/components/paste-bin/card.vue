<script setup lang="ts">
import { usePasteBinStore } from "@shared/stores/paste-bin";
import { useWorkspacesStore } from "@shared/stores/workspaces";
import MetaControls from "@mobile/components/meta/meta-controls.vue";

const props = defineProps<{
  identifier: string;
  title: string | null;
  content: string;
  isPinned: boolean;
  updatedAt: string;
}>();

const emit = defineEmits<{
  edit: [identifier: string];
}>();

const pasteBinStore = usePasteBinStore();
const workspaceStore = useWorkspacesStore();
const { notify } = useAppNotification();

const copied = ref(false);

const displayTitle = computed(() => props.title?.trim() || "Untitled paste");

const preview = computed(() => {
  const text = props.content.replace(/\s+/g, " ").trim();
  return text.length > 140 ? `${text.slice(0, 140)}…` : text;
});

const formattedDate = computed(() =>
  new Date(props.updatedAt).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  }),
);

async function copyContent() {
  await navigator.clipboard.writeText(props.content);
  copied.value = true;
  setTimeout(() => (copied.value = false), 1500);
  notify({ message: "Copied to clipboard", type: "success" });
}

async function togglePin() {
  await pasteBinStore.updatePasteBin(props.identifier, {
    isPinned: !props.isPinned,
  });
}

async function confirmDelete() {
  await pasteBinStore.deletePasteBin(props.identifier);
  notify({ message: "Paste moved to recycle bin", type: "success" });
}

const handleDuplicate = async (targetWorkspaceId: string) => {
  await pasteBinStore.duplicatePasteBin(
    props.identifier,
    workspaceStore.activeWorkspaceId,
    targetWorkspaceId,
  );
};

const handleTransfer = async (targetWorkspaceId: string) => {
  await pasteBinStore.transferPasteBin(
    props.identifier,
    workspaceStore.activeWorkspaceId,
    targetWorkspaceId,
  );
};
</script>

<template>
  <RyderCard
    :ui="{
      root: 'cursor-pointer overflow-hidden transition-shadow hover:shadow-sm !rounded-lg !border-gray-100 dark:!border-gray-700 dark:!bg-gray-800 !p-0',
    }"
    @click="emit('edit', identifier)"
  >
    <div class="p-4">
      <div class="flex items-start justify-between gap-2">
        <h3
          class="truncate text-sm font-medium text-gray-800 dark:text-gray-200"
        >
          {{ displayTitle }}
        </h3>
        <UIcon
          v-if="isPinned"
          name="ri:pushpin-2-fill"
          class="mt-0.5 size-4 shrink-0 text-primary-500"
        />
      </div>
      <p
        class="mt-1.5 line-clamp-2 text-xs leading-relaxed text-gray-400 dark:text-gray-500"
      >
        {{ preview || "Empty paste" }}
      </p>
    </div>

    <div
      class="flex items-center justify-between border-t border-gray-50 px-4 py-2 dark:border-gray-700"
    >
      <p class="text-xs text-gray-400">{{ formattedDate }}</p>

      <div class="flex items-center gap-2" @click.stop>
        <button
          class="rounded p-1 text-xs font-medium transition-colors"
          :class="
            copied ? 'text-green-500' : 'text-primary-600 dark:text-primary-400'
          "
          @click="copyContent"
        >
          {{ copied ? "Copied!" : "Copy" }}
        </button>
        <button
          class="rounded p-1 text-gray-400 transition-colors hover:text-gray-600 dark:hover:text-gray-300"
          :aria-label="isPinned ? 'Unpin paste' : 'Pin paste'"
          @click="togglePin"
        >
          <UIcon
            :name="isPinned ? 'ri:unpin-line' : 'ri:pushpin-2-line'"
            class="size-4"
          />
        </button>
        <MetaControls
          item-name="paste"
          :show-edit="false"
          @delete-record="confirmDelete"
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
