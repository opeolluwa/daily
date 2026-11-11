<script setup lang="ts">
import { useTaskStore } from "@shared/stores/tasks";
import { useUserPreferenceStore } from "@shared/stores/workspace-profile";

import HomeHeader from "@mobile/components/home/HomeHeader.vue";

definePageMeta({ layout: "default", name: "Home" });

const taskStore = useTaskStore();
const userPreferenceStore = useUserPreferenceStore();

async function refreshAll() {
  await Promise.all([
    taskStore.fetchTasks(),
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
