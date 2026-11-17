<script setup lang="ts">
const { keyboardHeight } = useVirtualKeyboard();
</script>

<template>
  <nav
    class="bottom-0 sticky -mx-6 z-30 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-app-dark-800 px-6 transition-[bottom] duration-100 ease-out"
    :style="keyboardHeight > 0 ? { bottom: `${keyboardHeight}px` } : undefined"
  >
    <slot />
  </nav>
</template>
