<script setup lang="ts">
import WorkspaceForm from "@mobile/components/workspace/form.vue";
import { useWorkspacesStore } from "@shared/stores/workspaces";

definePageMeta({ name: "Edit workspace" });

const route = useRoute();
const router = useRouter();
const workspaceStore = useWorkspacesStore();
const { notify } = useAppNotification();

const id = computed(() =>
  typeof route.query.id === "string" ? route.query.id : undefined,
);

const workspace = computed(
  () =>
    workspaceStore.workspaces?.find((entry) => entry.identifier === id.value) ??
    null,
);

function handleSaved() {
  notify({ message: "Workspace updated", type: "success" });
  router.replace("/settings/workspaces");
}

onMounted(async () => {
  if (!workspaceStore.workspaces?.length) {
    await workspaceStore.fetchWorkspaces();
  }
});
</script>

<template>
  <div>
    <div
      v-if="workspaceStore.loading && !workspace"
      class="flex flex-col gap-4"
    >
      <USkeleton class="h-16 rounded-lg" />
      <USkeleton class="h-16 rounded-lg" />
      <USkeleton class="h-10 rounded-lg" />
    </div>

    <div
      v-else-if="!workspace"
      class="flex flex-col items-center justify-center py-20 text-center"
    >
      <div class="mb-4 p-3 rounded-full bg-gray-100 dark:bg-gray-800">
        <UIcon name="heroicons:briefcase" class="size-7 text-gray-400" />
      </div>
      <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
        Workspace not found
      </h3>
      <button
        class="text-xs text-primary-500 hover:text-primary-600 font-medium mt-2"
        @click="router.push('/settings/workspaces')"
      >
        Back to workspaces
      </button>
    </div>

    <WorkspaceForm v-else :workspace="workspace" @saved="handleSaved" />
  </div>
</template>
