<script setup lang="ts">
withDefaults(
  defineProps<{
    alt?: string;
  }>(),
  { alt: "Daily" },
);
</script>

<template>
  <img
    src="/icon.png"
    :alt="alt"
    draggable="false"
    class="object-contain select-none"
  >
</template>
