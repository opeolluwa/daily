<script setup lang="ts">
import { provide, watch } from "vue";
import { usePersistence } from "./usePersistence";
import { PERSISTENCE_INSPECTOR_KEY } from "./types";

defineSlots<{ default?: () => unknown }>();
const emit = defineEmits<{ restored: [] }>();
const props = defineProps<{ whiteboardId: string }>();
const { isRestored, saveStatus, error, diagnostics } = usePersistence(
  () => props.whiteboardId,
);

provide(PERSISTENCE_INSPECTOR_KEY, {
  saveStatus,
  isRestored,
  error,
  diagnostics,
});

watch(
  isRestored,
  (value) => {
    if (value) emit("restored");
  },
  { immediate: true },
);
</script>

<template>
  <slot />
</template>
