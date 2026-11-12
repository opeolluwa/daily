<script setup lang="ts">
import {
  useRecycleBinStore,
  type RecycleBinItemType,
} from "~/stores/recycle-bin";
import WarningModal from "~/components/app/WarningModal.vue";

definePageMeta({ layout: false });

const recycleBinStore = useRecycleBinStore();
const { searchQuery, setSearch, clearSearch } = useAppSearch();
const toast = useToast();

const typeFilter = ref<RecycleBinItemType | "all">("all");
const purgeTargetId = ref<string | null>(null);
const confirmEmptyBin = ref(false);
const purging = ref(false);

const purgeTarget = computed(
  () =>
    recycleBinStore.entries.find((e) => e.identifier === purgeTargetId.value) ??
    null,
);

const confirmPurge = computed({
  get: () => purgeTargetId.value !== null,
  set: (value) => {
    if (!value) purgeTargetId.value = null;
  },
});

const purgeMessage = computed(() => {
  const itemType = (purgeTarget.value?.itemType ?? "item").replace("_", " ");

  return `This ${itemType} will be permanently deleted. This action cannot be undone.`;
});

const emptyBinMessage = computed(() => {
  const count = recycleBinStore.entries.length;
  const items = count === 1 ? "item" : "items";

  return `All ${count} ${items} will be permanently deleted. This action cannot be undone.`;
});

const itemTypeOptions: Array<{
  label: string;
  value: RecycleBinItemType | "all";
}> = [
  { label: "All", value: "all" },
  { label: "Notes", value: "note" },
  { label: "Tasks", value: "task" },
  { label: "Bookmarks", value: "bookmark" },
  { label: "Reminders", value: "reminder" },
  { label: "Snippets", value: "snippet" },
  { label: "Paste Bin", value: "paste_bin" },
];

const filteredEntries = computed(() => {
  let list = recycleBinStore.entries;

  if (typeFilter.value !== "all") {
    list = list.filter((e) => e.itemType === typeFilter.value);
  }

  const q = searchQuery.value.trim().toLowerCase();
  if (q) {
    list = list.filter((e) => e.itemType.toLowerCase().includes(q));
  }

  return list;
});

function formatDeletedAt(deletedAt: string): string {
  return new Date(deletedAt).toLocaleString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
    hour: "numeric",
    minute: "2-digit",
  });
}

function itemTypeIcon(type: RecycleBinItemType): string {
  const icons: Record<RecycleBinItemType, string> = {
    note: "heroicons:document-text",
    todo: "heroicons:check-circle",
    bookmark: "heroicons:bookmark",
    reminder: "heroicons:clock",
    snippet: "heroicons:code-bracket",
    paste_bin: "ri:clipboard-line",
  };
  return icons[type] ?? "heroicons:document";
}

async function handlePurge() {
  const identifier = purgeTargetId.value;

  if (!identifier) return;

  purging.value = true;

  try {
    await recycleBinStore.purgeEntry(identifier);
    toast.add({
      color: "success",
      icon: "heroicons:check-circle",
      title: "Permanently Deleted",
      description: "The item has been removed",
    });
    purgeTargetId.value = null;
  } catch {
    toast.add({
      color: "error",
      icon: "heroicons:x-circle",
      title: "Something Went Wrong",
      description: "Failed to delete the item",
    });
  } finally {
    purging.value = false;
  }
}

async function handleRestore(identifier: string) {
  try {
    await recycleBinStore.restoreEntry(identifier);
    toast.add({
      color: "success",
      icon: "heroicons:check-circle",
      title: "Item Restored",
      description: "The item has been restored",
    });
  } catch {
    toast.add({
      color: "error",
      icon: "heroicons:x-circle",
      title: "Something Went Wrong",
      description: "Failed to restore the item",
    });
  }
}

async function handlePurgeAll() {
  purging.value = true;

  try {
    await recycleBinStore.purgeAll();
    toast.add({
      color: "success",
      icon: "heroicons:check-circle",
      title: "Recycle Bin Emptied",
      description: "All items have been permanently removed",
    });
    confirmEmptyBin.value = false;
  } catch {
    toast.add({
      color: "error",
      icon: "heroicons:x-circle",
      title: "Something Went Wrong",
      description: "Failed to empty the recycle bin",
    });
  } finally {
    purging.value = false;
  }
}

onMounted(async () => {
  setSearch({ placeholder: "Search recycle bin..." });
  await recycleBinStore.fetchEntries();
});

onUnmounted(() => {
  clearSearch();
  purgeTargetId.value = null;
  confirmEmptyBin.value = false;
  purging.value = false;
});
</script>

<template>
  <NuxtLayout name="default">
    <template #page_title>
      <h1
        v-if="recycleBinStore.entries.length > 0"
        class="text-2xl font-semibold text-gray-800 dark:text-gray-100"
      >
        Recycle Bin
      </h1>
    </template>

    <template #primary_cta>
      <button
        v-if="recycleBinStore.entries.length !== 0"
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium transition-colors bg-gray-100 dark:bg-gray-800 text-gray-600 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700"
        @click="confirmEmptyBin = true"
      >
        <UIcon name="heroicons:trash" class="size-4" />
        Empty bin
      </button>
    </template>

    <template #main_content>
      <!-- Type filter tabs -->
      <div
        v-if="!recycleBinStore.loading && recycleBinStore.entries.length > 0"
        class="flex gap-1 flex-wrap mb-4"
      >
        <button
          v-for="opt in itemTypeOptions"
          :key="opt.value"
          class="px-3 py-1.5 rounded-lg text-xs font-medium transition-colors capitalize"
          :class="
            typeFilter === opt.value
              ? 'bg-primary-500 text-white'
              : 'bg-gray-100 dark:bg-gray-800 text-gray-500 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-700'
          "
          @click="typeFilter = opt.value"
        >
          {{ opt.label }}
          <span
            v-if="opt.value !== 'all' && recycleBinStore.typeCounts[opt.value]"
            class="ml-1 opacity-60"
          >
            {{ recycleBinStore.typeCounts[opt.value] }}
          </span>
        </button>
      </div>

      <!-- Loading -->
      <div v-if="recycleBinStore.loading" class="flex flex-col gap-2">
        <USkeleton v-for="i in 4" :key="i" class="h-14 rounded-lg" />
      </div>

      <RyderEmptyState
        v-else-if="recycleBinStore.entries.length === 0"
        title="Recycle bin is empty"
        description="Deleted items will appear here."
        icon="heroicons:trash"
      />

      <RyderEmptyState
        v-else-if="filteredEntries.length === 0"
        title="No results found"
        description="Try a different search or filter."
        icon="heroicons:magnifying-glass"
        action-label="Clear filters"
        @action="
          searchQuery = '';
          typeFilter = 'all';
        "
      />

      <!-- Entry list -->
      <div v-else class="flex flex-col gap-2">
        <RyderCard
          v-for="entry in filteredEntries"
          :key="entry.identifier"
          :ui="{ root: '!px-4 !py-3 flex items-center gap-3' }"
        >
          <div class="p-1.5 rounded-md bg-gray-100 dark:bg-gray-700">
            <UIcon
              :name="itemTypeIcon(entry.itemType)"
              class="size-4 text-gray-400 dark:text-gray-500"
            />
          </div>

          <div class="flex-1 min-w-0">
            <p
              class="text-xs font-medium text-gray-500 dark:text-gray-400 capitalize"
            >
              {{ entry.itemType }}
            </p>
            <p class="text-xs text-gray-400 dark:text-gray-500 mt-0.5">
              Deleted {{ formatDeletedAt(entry.deletedAt) }}
            </p>
          </div>

          <button
            class="p-1.5 rounded-md text-gray-400 hover:text-primary-500 hover:bg-primary-50 dark:hover:bg-primary-950 transition-colors shrink-0"
            title="Restore item"
            @click="handleRestore(entry.identifier)"
          >
            <UIcon name="heroicons:arrow-uturn-left" class="size-4" />
          </button>

          <button
            class="p-1.5 rounded-md text-gray-400 hover:text-rose-500 hover:bg-rose-50 dark:hover:bg-rose-950 transition-colors shrink-0"
            title="Permanently delete"
            @click="purgeTargetId = entry.identifier"
          >
            <UIcon name="heroicons:trash" class="size-4" />
          </button>
        </RyderCard>
      </div>

      <WarningModal
        v-model:open="confirmPurge"
        title="Delete permanently?"
        :message="purgeMessage"
        icon="heroicons:trash"
        confirm-label="Delete"
        confirm-color="error"
        :loading="purging"
        @confirm="handlePurge"
      />

      <WarningModal
        v-model:open="confirmEmptyBin"
        title="Empty recycle bin?"
        :message="emptyBinMessage"
        icon="heroicons:trash"
        confirm-label="Empty bin"
        confirm-color="error"
        :loading="purging"
        @confirm="handlePurgeAll"
      />
    </template>

    <template #side_content>
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Summary
      </h2>
      <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-3 mb-6">
        <p class="text-2xl font-semibold text-gray-700 dark:text-gray-200">
          {{ recycleBinStore.entries.length }}
        </p>
        <p class="text-xs text-gray-400 dark:text-gray-500">deleted items</p>
      </div>

      <USeparator class="my-4" />

      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        By type
      </h2>
      <div class="flex flex-col gap-2">
        <div
          v-for="opt in itemTypeOptions.filter((o) => o.value !== 'all')"
          :key="opt.value"
          class="flex items-center justify-between text-sm"
        >
          <div class="flex items-center gap-2">
            <UIcon
              :name="itemTypeIcon(opt.value as RecycleBinItemType)"
              class="size-4 text-gray-400"
            />
            <span class="text-gray-600 dark:text-gray-400 capitalize">{{
              opt.label
            }}</span>
          </div>
          <span class="text-xs text-gray-400">
            {{ recycleBinStore.typeCounts[opt.value] ?? 0 }}
          </span>
        </div>
      </div>
    </template>
  </NuxtLayout>
</template>
