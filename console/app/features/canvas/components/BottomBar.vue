<script setup lang="ts">
const { zoom, canUndo, canRedo } = defineProps<{
  zoom: number;
  canUndo: boolean;
  canRedo: boolean;
}>();

const emit = defineEmits<{
  "zoom-in": [];
  "zoom-out": [];
  "zoom-reset": [];
  undo: [];
  redo: [];
}>();

const BTN =
  "flex h-8 w-8 items-center justify-center text-foreground/60 transition-colors hover:text-foreground disabled:opacity-30 disabled:hover:text-foreground/60";
</script>

<template>
  <div class="absolute bottom-4 left-4 z-10 flex items-center gap-2">
    <!-- Zoom controls -->
    <div
      class="flex items-center rounded-lg border border-edge/40 bg-surface/80 shadow-lg backdrop-blur-md"
    >
      <UTooltip text="Zoom out" :content="{ side: 'top' }">
        <button :class="BTN" aria-label="Zoom out" @click="emit('zoom-out')">
          <UIcon name="lucide:minus" class="h-4 w-4" />
        </button>
      </UTooltip>

      <UTooltip text="Reset zoom" :content="{ side: 'top' }">
        <button
          class="min-w-[3.5rem] px-1 py-1 text-center font-mono text-xs text-foreground/60 transition-colors hover:text-foreground"
          aria-label="Reset zoom"
          @click="emit('zoom-reset')"
        >
          {{ Math.round(zoom * 100) }}%
        </button>
      </UTooltip>

      <UTooltip text="Zoom in" :content="{ side: 'top' }">
        <button :class="BTN" aria-label="Zoom in" @click="emit('zoom-in')">
          <UIcon name="lucide:plus" class="h-4 w-4" />
        </button>
      </UTooltip>
    </div>

    <!-- Undo/Redo controls -->
    <div
      class="flex items-center rounded-lg border border-edge/40 bg-surface/80 shadow-lg backdrop-blur-md"
    >
      <UTooltip text="Undo" :kbds="['meta', 'Z']" :content="{ side: 'top' }">
        <button
          :class="BTN"
          :disabled="!canUndo"
          aria-label="Undo"
          @click="emit('undo')"
        >
          <UIcon name="lucide:undo-2" class="h-4 w-4" />
        </button>
      </UTooltip>

      <UTooltip
        text="Redo"
        :kbds="['meta', 'shift', 'Z']"
        :content="{ side: 'top' }"
      >
        <button
          :class="BTN"
          :disabled="!canRedo"
          aria-label="Redo"
          @click="emit('redo')"
        >
          <UIcon name="lucide:redo-2" class="h-4 w-4" />
        </button>
      </UTooltip>
    </div>
  </div>
</template>
