<script setup lang="ts">
const { keyboardHeight } = useVirtualKeyboard();
</script>

<template>
  <nav
    class="sticky z-30 -mx-6 px-6 bg-gray-50 dark:bg-app-dark-800 transition-[bottom] duration-100 ease-out border-gray-200 dark:border-gray-700 md:top-0 md:border-b max-md:bottom-0 max-md:border-t"
    :style="keyboardHeight > 0 ? { bottom: `${keyboardHeight}px` } : undefined"
  >
    <slot />
  </nav>
</template>
