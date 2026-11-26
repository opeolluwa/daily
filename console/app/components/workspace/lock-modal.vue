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
  <div class="flex flex-1 flex-col items-center justify-center">
    <div
      class="w-full max-w-sm rounded-2xl border border-gray-100 dark:border-gray-700 bg-white dark:bg-gray-800 px-8 py-10 text-center"
    >
      <UIcon
        name="heroicons:lock-closed"
        class="size-10 text-primary-500 dark:text-primary-400 mx-auto"
      />
      <h2 class="mt-4 text-lg font-semibold text-gray-900 dark:text-white">
        Workspace locked
      </h2>
      <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
        Enter the password to access
        <span class="font-medium text-gray-700 dark:text-gray-200 capitalize">{{
          workspaceName
        }}</span>.
      </p>

      <form class="mt-5 flex flex-col gap-3" @submit.prevent="submit">
        <RyderPasswordInput
          v-model="password"
          label="Password"
          name="lock-password"
          placeholder="Enter password"
          :disabled="loading"
        />
        <p v-if="error" class="text-xs text-red-500 text-left">
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
