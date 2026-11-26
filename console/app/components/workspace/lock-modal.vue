<script setup lang="ts">
import { useWorkspacesStore } from "~/stores/workspaces";

const workspaceStore = useWorkspacesStore();

const open = computed(() => workspaceStore.isCurrentWorkspaceLocked);

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
  <RyderCreateDialog
    :open="open"
    title="Workspace locked"
    :description="`Enter the password to access ${workspaceName}`"
    submit-label="Unlock"
    :loading="loading"
    :submit-disabled="!password || loading"
    :dismissible="false"
    @submit="submit"
  >
    <div class="flex flex-col gap-4">
      <RyderPasswordInput
        v-model="password"
        label="Password"
        name="lock-password"
        placeholder="Enter password"
        required
        :disabled="loading"
      />
      <p v-if="error" class="text-sm text-red-500">
        {{ error }}
      </p>
    </div>
  </RyderCreateDialog>
</template>
