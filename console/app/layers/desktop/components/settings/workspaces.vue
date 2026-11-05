<script setup lang="ts">
import { useWorkspacesStore } from "@shared/stores/workspaces";

const toast = useToast();
const workspaceStore = useWorkspacesStore();

// ── create workspace ──────────────────────────────────────────────────────────
const showCreateModal = ref(false);

function handleCreated() {
  toast.add({
    color: "success",
    icon: "heroicons:check-circle",
    title: "Workspace Created",
    description: "Your workspace is ready",
  });
}

// ── secure workspace ──────────────────────────────────────────────────────────
const secureTargetId = ref<string | null>(null);
const securePassword = ref("");
const secureConfirm = ref("");
const secureError = ref("");
const secureSubmitting = ref(false);

const secureTargetWorkspace = computed(() =>
  secureTargetId.value
    ? workspaceStore.workspaces?.find(
        (w) => w.identifier === secureTargetId.value,
      )
    : null,
);

function handleToggleSecured(identifier: string) {
  const ws = workspaceStore.workspaces?.find(
    (w) => w.identifier === identifier,
  );
  if (!ws) return;
  secureTargetId.value = identifier;
  securePassword.value = "";
  secureConfirm.value = "";
  secureError.value = "";
}

function closeSecureModal() {
  secureTargetId.value = null;
  securePassword.value = "";
  secureConfirm.value = "";
  secureError.value = "";
}

async function submitSecure() {
  if (!secureTargetId.value || !secureTargetWorkspace.value) return;

  if (!secureTargetWorkspace.value.isSecured) {
    // Setting a new password
    if (!securePassword.value) {
      secureError.value = "Password is required.";
      return;
    }
    if (securePassword.value !== secureConfirm.value) {
      secureError.value = "Passwords do not match.";
      return;
    }
  }

  secureSubmitting.value = true;
  secureError.value = "";
  try {
    if (secureTargetWorkspace.value.isSecured) {
      // Removing the password — verify current password first
      const ok = await workspaceStore.verifyWorkspacePassword(
        secureTargetId.value,
        securePassword.value,
      );
      if (!ok) {
        secureError.value = "Incorrect password.";
        return;
      }
      await workspaceStore.updateWorkspace(secureTargetId.value, {
        isSecured: false,
        password: "",
      });
      toast.add({
        color: "success",
        icon: "heroicons:check-circle",
        title: "Password Removed",
        description: "The workspace is no longer protected",
      });
    } else {
      await workspaceStore.updateWorkspace(secureTargetId.value, {
        isSecured: true,
        password: securePassword.value,
      });
      workspaceStore.unlockWorkspace(secureTargetId.value);
      toast.add({
        color: "success",
        icon: "heroicons:check-circle",
        title: "Workspace Secured",
        description: "A password has been set",
      });
    }
    closeSecureModal();
  } catch (e) {
    toast.add({
      color: "error",
      icon: "heroicons:x-circle",
      title: "Something Went Wrong",
      description:
        (e as Error).message || "Failed to update workspace security",
    });
  } finally {
    secureSubmitting.value = false;
  }
}

// ── delete ────────────────────────────────────────────────────────────────────
async function handleDelete(identifier: string) {
  await workspaceStore.deleteWorkspace(identifier);
}

// ── set default ───────────────────────────────────────────────────────────────
async function handleSetDefault(identifier: string) {
  try {
    await workspaceStore.updateWorkspace(identifier, { isDefault: true });
    toast.add({
      color: "success",
      icon: "heroicons:check-circle",
      title: "Default Updated",
      description: "Your default workspace has been changed",
    });
  } catch (e) {
    toast.add({
      color: "error",
      icon: "heroicons:x-circle",
      title: "Something Went Wrong",
      description: (e as Error).message || "Failed to update workspace",
    });
  }
}

// ── toggle hidden ─────────────────────────────────────────────────────────────
async function handleToggleHidden(identifier: string) {
  const ws = workspaceStore.workspaces?.find(
    (w) => w.identifier === identifier,
  );
  if (!ws) return;
  try {
    await workspaceStore.updateWorkspace(identifier, {
      isHidden: !ws.isHidden,
    });
    toast.add({
      color: "success",
      icon: "heroicons:check-circle",
      title: ws.isHidden ? "Workspace Visible" : "Workspace Hidden",
      description: ws.isHidden
        ? "The workspace is now visible"
        : "The workspace has been hidden",
    });
  } catch (e) {
    toast.add({
      color: "error",
      icon: "heroicons:x-circle",
      title: "Something Went Wrong",
      description: (e as Error).message || "Failed to update workspace",
    });
  }
}

// ── edit ──────────────────────────────────────────────────────────────────────
const editingId = ref<string | null>(null);
const editName = ref("");
const editDescription = ref("");
const editSubmitting = ref(false);

function handleEdit(identifier: string) {
  const ws = workspaceStore.workspaces?.find(
    (w) => w.identifier === identifier,
  );
  if (!ws) return;
  editingId.value = identifier;
  editName.value = ws.name;
  editDescription.value = ws.description;
}

function closeEdit() {
  editingId.value = null;
  editName.value = "";
  editDescription.value = "";
}

async function submitEdit() {
  if (!editingId.value) return;
  editSubmitting.value = true;
  try {
    await workspaceStore.updateWorkspace(editingId.value, {
      name: editName.value.trim() || undefined,
      description: editDescription.value.trim(),
    });
    toast.add({
      color: "success",
      icon: "heroicons:check-circle",
      title: "Workspace Updated",
      description: "Your changes have been saved",
    });
    closeEdit();
  } catch (e) {
    toast.add({
      color: "error",
      icon: "heroicons:x-circle",
      title: "Something Went Wrong",
      description: (e as Error).message || "Failed to update workspace",
    });
  } finally {
    editSubmitting.value = false;
  }
}

const workspaces = computed(() => workspaceStore.workspaces ?? []);
</script>

<template>
  <div class="flex flex-col gap-4 mt-4">
    <div class="flex items-center justify-between">
      <h1 class="text-2xl font-semibold text-gray-800 dark:text-gray-100">
        Workspace
      </h1>
      <div class="w-36">
        <RyderButton
          size="md"
          class="w-full"
          :disabled="workspaceStore.loading"
          @click="showCreateModal = true"
        >
          New workspace
        </RyderButton>
      </div>
    </div>

    <!-- Workspace list -->
    <div
      v-for="workspace in workspaces"
      :key="workspace.identifier"
      class="cursor-pointer"
    >
      <WorkspaceCard
        :workspace="workspace"
        @delete="handleDelete"
        @edit="handleEdit"
        @toggle-hidden="handleToggleHidden"
        @set-default="handleSetDefault"
        @toggle-secured="handleToggleSecured"
      />
    </div>

    <!-- Edit modal -->
    <UModal :open="!!editingId" @close="closeEdit">
      <template #content>
        <div class="p-6 flex flex-col gap-4">
          <h3 class="text-sm font-semibold text-gray-800 dark:text-gray-200">
            Edit workspace
          </h3>
          <RyderInput
            v-model="editName"
            label="Name"
            placeholder="Workspace name"
            name="edit-workspace-name"
            :disabled="editSubmitting"
          />
          <RyderInput
            v-model="editDescription"
            label="Description"
            placeholder="Short description"
            name="edit-workspace-description"
            :disabled="editSubmitting"
          />
          <div class="flex items-center gap-2 mt-2">
            <UButton
              size="sm"
              :loading="editSubmitting"
              :disabled="!editName.trim()"
              @click="submitEdit"
            >
              Save
            </UButton>
            <UButton
              variant="ghost"
              size="sm"
              :disabled="editSubmitting"
              @click="closeEdit"
            >
              Cancel
            </UButton>
          </div>
        </div>
      </template>
    </UModal>
    <!-- Secure workspace modal -->
    <UModal :open="!!secureTargetId" @close="closeSecureModal">
      <template #content>
        <div class="p-6 flex flex-col gap-4">
          <h3 class="text-sm font-semibold text-gray-800 dark:text-gray-200">
            {{
              secureTargetWorkspace?.isSecured
                ? "Remove workspace password"
                : "Set workspace password"
            }}
          </h3>
          <p class="text-xs text-gray-500 dark:text-gray-400">
            {{
              secureTargetWorkspace?.isSecured
                ? "Enter the current password to remove protection from this workspace."
                : "This workspace will require a password before it can be accessed."
            }}
          </p>
          <RyderPasswordInput
            v-model="securePassword"
            :label="
              secureTargetWorkspace?.isSecured ? 'Current password' : 'Password'
            "
            placeholder="Enter password"
            name="secure-password"
            :disabled="secureSubmitting"
          />
          <RyderPasswordInput
            v-if="!secureTargetWorkspace?.isSecured"
            v-model="secureConfirm"
            label="Confirm password"
            placeholder="Confirm password"
            name="secure-confirm"
            :disabled="secureSubmitting"
          />
          <p v-if="secureError" class="text-xs text-red-500 dark:text-red-400">
            {{ secureError }}
          </p>
          <div class="flex items-center gap-2 mt-2">
            <UButton
              size="sm"
              :color="secureTargetWorkspace?.isSecured ? 'error' : 'primary'"
              :loading="secureSubmitting"
              :disabled="!securePassword"
              @click="submitSecure"
            >
              {{
                secureTargetWorkspace?.isSecured
                  ? "Remove password"
                  : "Set password"
              }}
            </UButton>
            <UButton
              variant="ghost"
              size="sm"
              :disabled="secureSubmitting"
              @click="closeSecureModal"
            >
              Cancel
            </UButton>
          </div>
        </div>
      </template>
    </UModal>

    <!-- Create workspace modal -->
    <WorkspaceCreateModal
      v-model:open="showCreateModal"
      title="New workspace"
      description="Workspaces allow you to organize your notes, tasks and bookmarks."
      submit-label="Create workspace"
      @created="handleCreated"
    />
  </div>
</template>
