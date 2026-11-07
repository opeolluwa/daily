<script setup lang="ts">
import { useTodoStore } from "@shared/stores/todo";
import { useUserPreferenceStore } from "@shared/stores/workspace-profile";

const todoStore = useTodoStore();
const userPreferenceStore = useUserPreferenceStore();

const now = ref(new Date());
let clockTimer: ReturnType<typeof setInterval>;

onMounted(() => {
  clockTimer = setInterval(() => {
    now.value = new Date();
  }, 60_000);
});
onUnmounted(() => clearInterval(clockTimer));

const greeting = computed(() => {
  const h = now.value.getHours();
  if (h < 12) return "Good morning";
  if (h < 17) return "Good afternoon";
  return "Good evening";
});

const today = computed(() =>
  now.value.toLocaleDateString("en-US", {
    weekday: "long",
    month: "long",
    day: "numeric",
  }),
);

const firstName = computed(
  () => userPreferenceStore.preference?.firstName || "there",
);

const activeTodoCount = computed(() => todoStore.activeTodos.length);
</script>

<template>
  <div
    class="relative -mx-6 -mt-8 overflow-hidden border-b border-gray-100 bg-linear-to-br from-primary-500/10 via-violet-400/5 to-transparent px-6 pt-7 pb-6 dark:border-gray-800 dark:from-primary-500/12 dark:via-violet-500/6 dark:to-transparent"
  >
    <div
      class="pointer-events-none absolute -top-10 right-0 size-52 rounded-full bg-primary-300/20 blur-3xl dark:bg-primary-500/10"
    />
    <div
      class="pointer-events-none absolute bottom-0 left-1/2 size-36 translate-y-1/2 rounded-full bg-violet-300/15 blur-2xl dark:bg-violet-500/8"
    />

    <div class="relative">
      <p
        class="mb-1.5 text-xs font-semibold tracking-widest uppercase text-gray-400 dark:text-gray-500"
      >
        {{ today }}
      </p>
      <h1
        class="text-2xl font-bold tracking-tight text-gray-900 sm:text-3xl dark:text-white"
      >
        {{ greeting }}, {{ firstName }}
      </h1>
      <p class="mt-1.5 text-sm text-gray-500 dark:text-gray-400">
        <template v-if="activeTodoCount > 0">
          <strong class="text-gray-700 dark:text-gray-300">{{
            activeTodoCount
          }}</strong>
          active {{ activeTodoCount === 1 ? "todo" : "todos" }} today.
        </template>
        <template v-else> You're all caught up. </template>
      </p>
    </div>
  </div>
</template>
