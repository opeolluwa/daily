<script setup lang="ts">
import { useUserPreferenceStore } from "@shared/stores/workspace-profile";
import { useWorkspacesStore } from "@shared/stores/workspaces";
import type { Workspace } from "@shared/stores/workspaces";

withDefaults(
  defineProps<{
    open: boolean;
    title?: string;
    description?: string;
    submitLabel?: string;
  }>(),
  {
    title: "Create a New Workspace",
    description: "Workspaces allow you to organize your projects and files.",
    submitLabel: "Save and continue",
  },
);

const emit = defineEmits<{
  "update:open": [value: boolean];
  created: [
    workspace: Workspace,
    initialProfile: { firstName: string; lastName: string; email: string },
  ];
}>();

const workspaceStore = useWorkspacesStore();
const preferenceStore = useUserPreferenceStore();

const form = reactive({ name: "", description: "" });
const errors = reactive({ name: "", description: "" });
const loading = ref(false);
const submitError = ref("");

function resetForm() {
  Object.assign(form, { name: "", description: "" });
  Object.assign(errors, { name: "", description: "" });
  submitError.value = "";
}

function validate(): boolean {
  errors.name = form.name.trim() ? "" : "Name is required";
  errors.description = form.description.trim() ? "" : "Description is required";
  return !errors.name && !errors.description;
}

function handleClose(value: boolean) {
  if (!value) resetForm();
  emit("update:open", value);
}

async function handleSubmit() {
  if (!validate()) return;
  loading.value = true;
  submitError.value = "";
  try {
    const prefSnapshot = preferenceStore.preference;
    const created = await workspaceStore.createWorkspace({
      name: form.name.trim(),
      description: form.description.trim(),
    });
    resetForm();
    emit("update:open", false);
    emit("created", created, {
      firstName: prefSnapshot?.firstName ?? "",
      lastName: prefSnapshot?.lastName ?? "",
      email: prefSnapshot?.email ?? "",
    });
  } catch (e) {
    console.error(e);
    submitError.value = "Failed to create workspace. Please try again.";
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <RyderCreateDialog
    :open="open"
    :title="title"
    :description="description"
    :loading="loading"
    :submit-label="submitLabel"
    :submit-disabled="loading"
    @update:open="handleClose"
    @submit="handleSubmit"
    @cancel="handleClose(false)"
  >
    <div class="flex flex-col gap-4">
      <RyderInput
        v-model="form.name"
        label="Name"
        hint="required"
        type="text"
        name="workspace-name"
        placeholder="Daily"
        :error="errors.name"
        :disabled="loading"
      />
      <RyderInput
        v-model="form.description"
        label="Description"
        hint="required"
        type="text"
        name="workspace-description"
        placeholder="Organize files and tasks"
        :error="errors.description"
        :disabled="loading"
      />
      <p v-if="submitError" class="text-sm text-red-500">
        {{ submitError }}
      </p>
    </div>
  </RyderCreateDialog>
</template>
