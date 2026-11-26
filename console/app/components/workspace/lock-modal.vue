<script setup lang="ts">
import { useWorkspacesStore } from "~/stores/workspaces";

const emit = defineEmits<{ unlocked: [] }>();

const workspaceStore = useWorkspacesStore();

const password = ref("");
const error = ref("");
const loading = ref(false);

const workspaceName = computed(
  () => workspaceStore.currentWorkspace?.name ?? "this workspace",
);

async function submit() {
  const id = workspaceStore.activeWorkspaceId;
  if (!password.value || !id) return;
  loading.value = true;
  error.value = "";
  try {
    const ok = await workspaceStore.verifyWorkspacePassword(id, password.value);
    if (ok) {
      workspaceStore.unlockWorkspace(id);
      emit("unlocked");
    } else {
      error.value = "Incorrect password. Please try again.";
    }
  } catch {
    error.value = "Failed to verify password.";
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <div class="flex flex-1 flex-col items-center justify-center relative overflow-hidden">
    <!-- Card -->
    <div
      class="relative w-full max-w-md rounded-2xl border border-gray-100 dark:border-gray-700 bg-white dark:bg-gray-800 shadow-lg px-10 py-10 text-center"
    >
      <!-- Lock icon badge -->
      <div
        class="mx-auto flex size-16 items-center justify-center rounded-full bg-primary-50 dark:bg-primary-950/40"
      >
        <UIcon
          name="heroicons:lock-closed"
          class="size-8 text-primary-500 dark:text-primary-400"
        />
      </div>

      <h2 class="mt-5 text-2xl font-bold text-gray-900 dark:text-white">
        Workspace locked
      </h2>
      <p class="mt-2 text-sm text-gray-500 dark:text-gray-400">
        Enter the password to access
        <span class="font-semibold text-gray-700 dark:text-gray-200 capitalize">{{
          workspaceName
        }}</span>.
      </p>

      <form class="mt-6 flex flex-col gap-4 text-left" @submit.prevent="submit">
        <RyderPasswordInput
          v-model="password"
          label="Password"
          name="lock-password"
          placeholder="Enter password"
          required
          :disabled="loading"
        />
        <p v-if="error" class="text-xs text-red-500">
          {{ error }}
        </p>
        <RyderButton
          type="submit"
          color="primary"
          block
          size="md"
          class="w-full"
          :loading="loading"
          :disabled="!password || loading"
        >
          Unlock
        </RyderButton>
      </form>
    </div>
  </div>
</template>
