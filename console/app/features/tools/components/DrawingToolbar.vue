<script setup lang="ts">
import { useToolStore } from "@drawvue/core";
import { TOOL_ICONS } from "./toolIcons";
import type { ToolType } from "@drawvue/core";

const { activeTool, setTool } = useToolStore();

defineExpose({});

const tools: Array<{
  type: ToolType;
  label: string;
  shortcutNumber: number | null;
  shortcutKey: string;
}> = [
  { type: "hand", label: "Hand", shortcutNumber: null, shortcutKey: "H" },
  {
    type: "selection",
    label: "Selection",
    shortcutNumber: 1,
    shortcutKey: "V",
  },
  {
    type: "rectangle",
    label: "Rectangle",
    shortcutNumber: 2,
    shortcutKey: "R",
  },
  { type: "diamond", label: "Diamond", shortcutNumber: 3, shortcutKey: "D" },
  { type: "ellipse", label: "Ellipse", shortcutNumber: 4, shortcutKey: "O" },
  { type: "arrow", label: "Arrow", shortcutNumber: 5, shortcutKey: "A" },
  { type: "text", label: "Text", shortcutNumber: 6, shortcutKey: "T" },
  { type: "freedraw", label: "Pencil", shortcutNumber: 7, shortcutKey: "P" },
  { type: "code", label: "Code", shortcutNumber: 0, shortcutKey: "C" },
  { type: "line", label: "Line", shortcutNumber: 8, shortcutKey: "L" },
  { type: "image", label: "Image", shortcutNumber: 9, shortcutKey: "I" },
  { type: "eraser", label: "Eraser", shortcutNumber: null, shortcutKey: "E" },
];
</script>

<template>
  <div
    class="absolute top-4 left-1/2 z-10 flex -translate-x-1/2 flex-col items-center gap-2"
  >
    <div
      class="flex gap-0.5 rounded-lg border border-edge/40 bg-surface/80 p-1 shadow-lg backdrop-blur-md"
    >
      <UTooltip
        v-for="tool in tools"
        :key="tool.type"
        :text="tool.label"
        :kbds="[tool.shortcutKey]"
        :content="{ side: 'bottom' }"
      >
        <button
          :aria-label="tool.label"
          :aria-pressed="activeTool === tool.type"
          class="relative flex h-9 w-9 items-center justify-center rounded-md transition-colors"
          :class="
            activeTool === tool.type
              ? 'bg-accent/20 text-accent'
              : 'text-foreground/70 hover:bg-subdued/20 hover:text-foreground'
          "
          @click="setTool(tool.type)"
        >
          <svg
            aria-hidden="true"
            class="h-5 w-5"
            :viewBox="TOOL_ICONS[tool.type].viewBox"
            v-html="TOOL_ICONS[tool.type].paths"
          />
          <span
            v-if="tool.shortcutNumber !== null"
            class="absolute right-0.5 bottom-0 text-[10px] opacity-40"
          >
            {{ tool.shortcutNumber }}
          </span>
        </button>
      </UTooltip>
    </div>
    <p class="text-xs text-foreground/40">
      To move canvas, hold Scroll wheel or Space while dragging, or use the hand
      tool
    </p>
  </div>
</template>
