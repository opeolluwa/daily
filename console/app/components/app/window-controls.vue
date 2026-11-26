<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import { platform } from "@tauri-apps/plugin-os";

const appWindow = getCurrentWindow();
const isMacOS = platform().toLowerCase() === "macos";
</script>

<template>
  <div
    class="fixed top-0 left-0 right-0 h-8 z-9999 flex items-center px-4 rounded-t-2xl"
    data-tauri-drag-region
  >
    <div v-if="isMacOS" class="traffic-lights shrink-0">
      <span class="btn close" @click="appWindow.close()" />
      <span class="btn minimize" @click="appWindow.minimize()" />
      <span class="btn maximize" @click="appWindow.toggleMaximize()" />
    </div>

    <div v-else class="flex items-center shrink-0 ml-auto">
      <button
        class="p-1 hover:bg-gray-200 dark:hover:bg-gray-700 rounded"
        aria-label="Minimize"
        @click="appWindow.minimize()"
      >
        <UIcon name="heroicons:minus" class="size-4" />
      </button>
      <button
        class="p-1 hover:bg-gray-200 dark:hover:bg-gray-700 rounded"
        aria-label="Maximize"
        @click="appWindow.toggleMaximize()"
      >
        <UIcon name="lucide:maximize" class="size-4" />
      </button>
      <button
        class="p-1 hover:bg-red-500 hover:text-white rounded"
        aria-label="Close"
        @click="appWindow.close()"
      >
        <UIcon name="heroicons:x-mark" class="size-4" />
      </button>
    </div>
  </div>
</template>

<style scoped>
.traffic-lights {
  display: flex;
  gap: 8px;
}

.btn {
  width: 11px;
  height: 12px;
  border-radius: 50%;
  display: inline-block;
  cursor: pointer;
}

.close {
  background: #ff5f57;
}

.minimize {
  background: #ffbd2e;
}

.maximize {
  background: #28c840;
}

.traffic-lights:hover .btn::after {
  content: "";
  display: block;
  width: 100%;
  height: 100%;
  background-size: 8px;
  background-repeat: no-repeat;
  background-position: center;
}

.close:hover::after {
  content: "✕";
  font-size: 8px;
  color: #4d0000;
  text-align: center;
}

.minimize:hover::after {
  content: "–";
  font-size: 10px;
  color: #664d00;
  text-align: center;
}

.maximize:hover::after {
  content: "+";
  font-size: 10px;
  color: #003300;
  text-align: center;
}
</style>
