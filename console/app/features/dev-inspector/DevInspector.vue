<script setup lang="ts">
import { ref, computed } from "vue";
import { useEventListener, useActiveElement } from "@vueuse/core";
import { useDrawVue, isTypingElement } from "@drawvue/core";
import HistoryTab from "./tabs/HistoryTab.vue";
import ElementsTab from "./tabs/ElementsTab.vue";
import StateTab from "./tabs/StateTab.vue";
import ActionsTab from "./tabs/ActionsTab.vue";
import LayersTab from "./tabs/LayersTab.vue";
import StorageTab from "./tabs/StorageTab.vue";
import { useTestHook } from "./useTestHook";

const isOpen = ref(false);
const activeTab = ref<"history" | "elements" | "layers" | "state" | "actions" | "storage">(
  "history",
);

const ctx = useDrawVue();
const h = useTestHook();

const TABS = [
  { id: "history" as const, label: "History" },
  { id: "elements" as const, label: "Elements" },
  { id: "layers" as const, label: "Layers" },
  { id: "state" as const, label: "State" },
  { id: "actions" as const, label: "Actions" },
  { id: "storage" as const, label: "Storage" },
];

const activeEl = useActiveElement();
useEventListener(document, "keydown", (e: KeyboardEvent) => {
  if (isTypingElement(activeEl.value)) return;
  if (e.key === "`") {
    e.preventDefault();
    isOpen.value = !isOpen.value;
  }
});

const selectedIds = computed(
  () => (h?.selectedIds as { value: ReadonlySet<string> })?.value ?? new Set<string>(),
);
const history = computed(() => h?.history as Record<string, unknown> | undefined);

const currentElementCount = computed(
  () => ctx.elements.elements.value.filter((el) => !el.isDeleted).length,
);
const currentSelectedCount = computed(() => selectedIds.value.size);

const historyCount = computed(() => {
  if (!history.value) return 0;
  const undoLen = (history.value.undoStack as { value: readonly unknown[] })?.value?.length ?? 0;
  const redoLen = (history.value.redoStack as { value: readonly unknown[] })?.value?.length ?? 0;
  return undoLen + redoLen;
});

function tabBadge(tabId: string): number | null {
  if (tabId === "history" && historyCount.value > 0) return historyCount.value;
  if (tabId === "elements" && currentElementCount.value > 0) return currentElementCount.value;
  if (tabId === "layers") return 3;
  return null;
}

function handleSelect(id: string): void {
  const selectFn = h?.select as ((id: string) => void) | undefined;
  if (!selectFn) return;
  selectFn(id);
  const markDirty = h?.markInteractiveDirty as (() => void) | undefined;
  if (markDirty) markDirty();
}
</script>

<template>
  <Teleport to="body">
    <!-- Toggle button -->
    <button
      class="fixed bottom-4 right-4 z-[60] flex h-8 w-8 items-center justify-center rounded-lg border border-edge/40 bg-surface/80 text-foreground/50 shadow-lg backdrop-blur-md transition-all hover:bg-surface hover:text-accent"
      :class="{ 'text-accent border-accent/40': isOpen }"
      aria-label="Toggle Dev Inspector"
      title="Dev Inspector (`)"
      @click="isOpen = !isOpen"
    >
      <svg
        class="h-4 w-4"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <path d="M12 20h9M16.5 3.5a2.12 2.12 0 013 3L7 19l-4 1 1-4z" />
      </svg>
    </button>

    <!-- Panel -->
    <div
      class="fixed top-16 right-3 bottom-4 z-[60] flex w-[380px] max-w-[calc(100vw-1.5rem)] flex-col overflow-hidden rounded-lg border border-edge/40 bg-surface/80 shadow-lg backdrop-blur-md transition-transform duration-300 ease-out"
      :class="isOpen ? 'translate-x-0' : 'translate-x-[calc(100%+2rem)]'"
    >
      <!-- Header -->
      <div
        class="flex shrink-0 items-center justify-between border-b border-edge/30 px-3 py-2.5"
      >
        <div class="flex items-center gap-2">
          <div
            class="flex h-[18px] w-[18px] items-center justify-center rounded-md bg-accent/15 text-accent"
          >
            <svg
              class="h-[11px] w-[11px]"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
            >
              <path d="M12 20h9M16.5 3.5a2.12 2.12 0 013 3L7 19l-4 1 1-4z" />
            </svg>
          </div>
          <span
            class="text-xs font-semibold uppercase tracking-wider text-foreground/60"
            >Dev Inspector</span
          >
        </div>
        <button
          class="flex h-6 w-6 items-center justify-center rounded text-foreground/40 transition-colors hover:bg-subdued/40 hover:text-foreground"
          @click="isOpen = false"
        >
          <svg
            class="h-3.5 w-3.5"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
          >
            <line x1="18" y1="6" x2="6" y2="18" />
            <line x1="6" y1="6" x2="18" y2="18" />
          </svg>
        </button>
      </div>

      <!-- Tabs -->
      <div
        class="mx-2 mt-2 flex shrink-0 gap-0.5 overflow-x-auto rounded-lg border border-edge/30 bg-surface/60 p-0.5"
      >
        <button
          v-for="tab in TABS"
          :key="tab.id"
          class="flex flex-1 items-center justify-center gap-1 whitespace-nowrap rounded-md px-1.5 py-1.5 text-[10px] font-medium transition-colors"
          :class="
            activeTab === tab.id
              ? 'bg-accent/20 text-accent'
              : 'text-foreground/50 hover:bg-subdued/40 hover:text-foreground/80'
          "
          @click="activeTab = tab.id"
        >
          <span>{{ tab.label }}</span>
          <span
            v-if="tabBadge(tab.id) != null"
            class="rounded-full px-1.5 py-px font-mono text-[9px] font-semibold leading-snug"
            :class="
              activeTab === tab.id
                ? 'bg-accent/20 text-accent'
                : 'bg-subdued/50 text-foreground/50'
            "
          >
            {{ tabBadge(tab.id) }}
          </span>
        </button>
      </div>

      <!-- Tab content -->
      <div class="flex-1 overflow-y-auto">
        <HistoryTab
          v-if="activeTab === 'history' && history"
          :undo-stack="history.undoStack as any"
          :redo-stack="history.redoStack as any"
          :can-undo="(history.canUndo as any)?.value ?? false"
          :can-redo="(history.canRedo as any)?.value ?? false"
          :current-element-count="currentElementCount"
          :current-selected-count="currentSelectedCount"
        />

        <ElementsTab
          v-if="activeTab === 'elements'"
          :elements="ctx.elements.elements.value"
          :selected-ids="selectedIds"
          @select="handleSelect"
        />

        <LayersTab v-if="activeTab === 'layers'" />

        <StateTab v-if="activeTab === 'state'" />

        <ActionsTab
          v-if="activeTab === 'actions'"
          :registry="ctx.actionRegistry"
        />

        <StorageTab v-if="activeTab === 'storage'" />
      </div>

      <!-- Status bar -->
      <div
        class="flex shrink-0 items-center justify-between border-t border-edge/30 bg-surface/40 px-3 py-1.5 font-mono text-[10px] text-foreground/40"
      >
        <span class="flex items-center gap-1.5">
          <span class="inline-block h-1.5 w-1.5 rounded-full bg-green-500 dark:bg-green-400" />
          Live
        </span>
        <span>` to toggle</span>
      </div>
    </div>
  </Teleport>
</template>
