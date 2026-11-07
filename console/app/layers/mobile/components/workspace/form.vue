<script setup lang="ts">
import {
  useWorkspacesStore,
  type Workspace,
} from "@shared/stores/workspaces";

const props = defineProps<{
  workspace?: Workspace | null;
}>();

const emit = defineEmits<{
  saved: [];
}>();

const workspaceStore = useWorkspacesStore();

const form = reactive({ name: "", description: "" });
const errors = reactive({ name: "" });
const loading = ref(false);
const submitError = ref("");

const editing = computed(() => !!props.workspace);

watch(
  () => props.workspace,
  (workspace) => {
    if (!workspace) return;
    form.name = workspace.name;
    form.description = workspace.description ?? "";
    errors.name = "";
    submitError.value = "";
  },
  { immediate: true },
);

async function handleSubmit() {
  errors.name = form.name.trim() ? "" : "Name is required";
  if (errors.name) return;

  loading.value = true;
  submitError.value = "";
  try {
    if (editing.value && props.workspace) {
      await workspaceStore.updateWorkspace(props.workspace.identifier, {
        name: form.name.trim(),
        description: form.description.trim(),
      });
    } else {
      await workspaceStore.createWorkspace({
        name: form.name.trim(),
        description: form.description.trim(),
      });
    }
    emit("saved");
  } catch (e) {
    console.error(e);
    submitError.value = editing.value
      ? "Failed to update workspace. Please try again."
      : "Failed to create workspace. Please try again.";
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <form class="flex flex-col gap-4" @submit.prevent="handleSubmit">
    <p class="text-sm text-gray-500 dark:text-gray-400">
      Workspaces allow you to organize your notes, tasks and bookmarks.
    </p>

    <div class="flex flex-col gap-4">
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

      <RyderInput
        v-model="form.description"
        label="Description (optional)"
        type="text"
        size="sm"
        name="workspace-description"
        placeholder="Organize files and tasks"
        :disabled="loading"
      />

      <p v-if="submitError" class="text-sm text-red-500">
        {{ submitError }}
      </p>
    </div>

    <div
      class="sticky bottom-0 -mx-6 border-t border-gray-100 bg-white/95 px-6 py-3 backdrop-blur dark:border-gray-700 dark:bg-app-dark-800/95"
    >
      <RyderButton
        type="submit"
        size="md"
        block
        :loading="loading"
        :disabled="loading"
      >
        {{ editing ? "Save changes" : "Create workspace" }}
      </RyderButton>
    </div>
  </form>
</template>
