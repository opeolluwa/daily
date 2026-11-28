<script setup lang="ts">
import { computed } from "vue";

const props = withDefaults(
  defineProps<{
    progress: number;
    size?: number;
  }>(),
  { size: 34 },
);

const radius = 14;
const circumference = 2 * Math.PI * radius;
const clamped = computed(() =>
  Math.min(100, Math.max(0, Math.round(props.progress))),
);
const offset = computed(
  () => circumference * (1 - clamped.value / 100),
);
</script>

<template>
  <div
    class="relative inline-flex shrink-0 items-center justify-center"
    :style="{ width: `${props.size}px`, height: `${props.size}px` }"
  >
    <svg viewBox="0 0 36 36" class="size-full -rotate-90">
      <circle
        cx="18"
        cy="18"
        :r="radius"
        fill="none"
        stroke="currentColor"
        stroke-width="4"
        class="text-gray-200 dark:text-gray-700"
      />
      <circle
        cx="18"
        cy="18"
        :r="radius"
        fill="none"
        stroke="currentColor"
        stroke-width="4"
        stroke-linecap="round"
        :stroke-dasharray="circumference"
        :stroke-dashoffset="offset"
        class="text-[#42b883] transition-all duration-300"
      />
    </svg>
    <span
      class="absolute font-mono text-[9px] leading-none font-semibold text-gray-600 dark:text-gray-300"
    >
      {{ clamped }}%
    </span>
  </div>
</template>
