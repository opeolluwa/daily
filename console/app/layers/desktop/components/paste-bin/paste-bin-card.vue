<script setup lang="ts">
import { usePasteBinStore } from "@shared/stores/paste-bin";
import { useWorkspacesStore } from "@shared/stores/workspaces";

const props = defineProps<{
  identifier: string;
  title: string | null;
  content: string;
  isPinned: boolean;
  updatedAt: string;
  active?: boolean;
}>();

const emit = defineEmits<{
  select: [identifier: string];
  edit: [identifier: string];
}>();

const pasteBinStore = usePasteBinStore();
const workspaceStore = useWorkspacesStore();
const toast = useToast();

const copied = ref(false);

const displayTitle = computed(() => props.title?.trim() || "Untitled paste");

const preview = computed(() => {
  const text = props.content.replace(/\s+/g, " ").trim();
  return text.length > 220 ? `${text.slice(0, 220)}…` : text;
});

const formattedDate = computed(() =>
  new Date(props.updatedAt).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
  }),
);

async function copyContent() {
  await navigator.clipboard.writeText(props.content);
  copied.value = true;
  setTimeout(() => (copied.value = false), 1500);
}

async function togglePin() {
  await pasteBinStore.updatePasteBin(props.identifier, {
    isPinned: !props.isPinned,
  });
}

async function confirmDelete() {
  await pasteBinStore.deletePasteBin(props.identifier);
  toast.add({
    color: "success",
    icon: "heroicons:check-circle",
    title: "Paste Removed",
    description: "The paste has been moved to the recycle bin",
  });
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
  <div
    class="rounded-lg border transition-colors cursor-pointer"
    :class="
      active
        ? 'border-primary-300 bg-primary-50/60 dark:border-primary-800 dark:bg-primary-950/40'
        : 'border-gray-100 bg-white hover:border-gray-200 dark:border-gray-700 dark:bg-gray-800 dark:hover:border-gray-600'
    "
    @click="emit('select', identifier)"
  >
    <div class="p-4">
      <div class="flex items-start justify-between gap-3">
        <div class="min-w-0 flex-1">
          <h3
            class="truncate text-sm font-medium text-gray-800 dark:text-gray-200"
          >
            {{ displayTitle }}
          </h3>
          <p
            class="mt-1 line-clamp-2 text-xs leading-relaxed text-gray-400 dark:text-gray-500"
          >
            {{ preview || "Empty paste" }}
          </p>
        </div>
        <UIcon
          v-if="isPinned"
          name="ri:pushpin-2-fill"
          class="mt-0.5 size-4 shrink-0 text-primary-500"
        />
      </div>
    </div>

    <div
      class="flex items-center justify-between border-t border-gray-50 px-4 py-2 dark:border-gray-700"
    >
      <p class="text-xs text-gray-400">{{ formattedDate }}</p>

      <div class="flex items-center gap-2" @click.stop>
        <button
          class="rounded p-1 text-xs font-medium transition-colors"
          :class="
            copied
              ? 'text-green-500'
              : 'text-primary-600 hover:text-primary-700 dark:text-primary-400'
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
          :show-edit="true"
          @edit-record="emit('edit', identifier)"
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
  </div>
</template>
