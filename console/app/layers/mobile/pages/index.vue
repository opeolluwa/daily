<script setup lang="ts">
import { useTodoStore } from "@shared/stores/todo";
import { useUserPreferenceStore } from "@shared/stores/workspace-profile";

import HomeHeader from "@mobile/components/home/HomeHeader.vue";

definePageMeta({ layout: "default", name: "Home" });

const todoStore = useTodoStore();
const userPreferenceStore = useUserPreferenceStore();

async function refreshAll() {
  await Promise.all([
    todoStore.fetchTodos(),
    userPreferenceStore.fetchPreference(),
  ]);
}

onMounted(refreshAll);
</script>

<template>
  <RyderPullToRefresh @refresh="refreshAll">
    <HomeHeader />
  </RyderPullToRefresh>
</template>
