<script setup lang="ts">
import { useWorkspacesStore } from "~/stores/workspaces";

const workspaceStore = useWorkspacesStore();

const form = reactive({ name: "", description: "" });
const errors = reactive({ name: "", description: "" });
const loading = ref(false);
const submitError = ref("");

function validate(): boolean {
  errors.name = form.name.trim() ? "" : "Name is required";
  errors.description = form.description.trim() ? "" : "Description is required";
  return !errors.name && !errors.description;
}

async function handleSubmit() {
  if (!validate()) return;
  loading.value = true;
  submitError.value = "";
  try {
    await workspaceStore.createWorkspace({
      name: form.name.trim(),
      description: form.description.trim(),
    });
    reloadNuxtApp();
  } catch (e) {
    console.error(e);
    submitError.value = "Failed to create workspace. Please try again.";
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <UModal
    :open="true"
    :close="false"
    :dismissible="false"
    @update:open="() => {}"
  >
    <template #header>
      <div class="flex flex-col gap-1">
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
          Welcome to Daily
        </h2>
        <p class="text-sm text-gray-500 dark:text-gray-400">
          Create your first workspace to get started. You can have multiple
          workspaces for different projects or contexts, and switch between them
          seamlessly.
        </p>
      </div>
    </template>

    <template #body>
      <form class="flex flex-col gap-4" @submit.prevent="handleSubmit">
        <div>
          <RyderInput
            v-model="form.name"
            label="Name"
            type="text"
            name="workspace-name"
            placeholder="Daily"
            :disabled="loading"
          />
          <p v-if="errors.name" class="mt-1 text-xs text-red-500">
            {{ errors.name }}
          </p>
        </div>

        <div>
          <RyderInput
            v-model="form.description"
            label="Description"
            type="text"
            name="workspace-description"
            placeholder="Organize files and tasks"
            :disabled="loading"
          />
          <p v-if="errors.description" class="mt-1 text-xs text-red-500">
            {{ errors.description }}
          </p>
        </div>

        <p v-if="submitError" class="text-sm text-red-500">
          {{ submitError }}
        </p>

        <div class="flex justify-end pt-1">
          <RyderButton
            type="submit"
            color="primary"
            size="md"
            class="w-fit"
            :loading="loading"
            :disabled="loading"
          >
            Save and continue
          </RyderButton>
        </div>
      </form>
    </template>
  </UModal>
</template>
