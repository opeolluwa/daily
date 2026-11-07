<script setup lang="ts">
import { useWorkspacesStore } from "@shared/stores/workspaces";
import WorkspaceCard from "@mobile/components/workspace/workspace-card.vue";
definePageMeta({ name: "Workspaces" });

const { notify } = useAppNotification();
const workspaceStore = useWorkspacesStore();
const router = useRouter();

function openCreate() {
  router.push("/settings/workspaces/create");
}

function openEdit(identifier: string) {
  router.push({
    path: "/settings/workspaces/edit",
    query: { id: identifier },
  });
}

const secureTargetId = ref<string | null>(null);

function handleToggleSecured(identifier: string) {
  const ws = workspaceStore.workspaces?.find(
    (w) => w.identifier === identifier,
  );
  if (!ws) return;
  secureTargetId.value = identifier;
}

function closeSecureModal() {
  secureTargetId.value = null;
}

async function handleDelete(identifier: string) {
  await workspaceStore.deleteWorkspace(identifier);
}

async function handleSetDefault(identifier: string) {
  try {
    await workspaceStore.updateWorkspace(identifier, { isDefault: true });
    notify({ message: "Default workspace updated", type: "success" });
  } catch (e) {
    notify({
      message: (e as Error).message || "Failed to update workspace",
      type: "error",
    });
  }
}

async function handleToggleHidden(identifier: string) {
  const ws = workspaceStore.workspaces?.find(
    (w) => w.identifier === identifier,
  );
  if (!ws) return;
  try {
    await workspaceStore.updateWorkspace(identifier, {
      isHidden: !ws.isHidden,
    });
    notify({
      message: ws.isHidden ? "Workspace is now visible" : "Workspace hidden",
      type: "success",
    });
  } catch (e) {
    notify({
      message: (e as Error).message || "Failed to update workspace",
      type: "error",
    });
  }
}

const workspaces = computed(() => workspaceStore.workspaces ?? []);
</script>

<template>
  <div class="flex flex-col gap-4 mt-4">
    <RyderEmptyState
      v-if="!workspaces.length && !workspaceStore.loading"
      icon="heroicons:briefcase"
      title="No workspaces yet"
      description="Create a workspace to organize your notes, tasks and bookmarks."
      action-label="Create workspace"
      @action="openCreate"
    />

    <div
      v-for="workspace in workspaces"
      :key="workspace.identifier"
      class="cursor-pointer"
    >
      <WorkspaceCard
        :workspace="workspace"
        @delete="handleDelete"
        @edit="openEdit"
        @toggle-hidden="handleToggleHidden"
        @set-default="handleSetDefault"
        @toggle-secured="handleToggleSecured"
      />
    </div>

    <!-- Secure workspace modal -->
    <WorkspaceSecureModal
      :open="!!secureTargetId"
      :workspace-id="secureTargetId"
      @update:open="closeSecureModal"
    />

    <RyderFab class="!bottom-24" @click="openCreate" />
  </div>
</template>
