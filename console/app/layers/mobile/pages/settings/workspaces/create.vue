<script setup lang="ts">
import WorkspaceForm from "@mobile/components/workspace/form.vue";

definePageMeta({ name: "New workspace" });

const router = useRouter();
const { notify } = useAppNotification();

function handleSaved() {
  notify({ message: "Workspace created", type: "success" });
  router.replace("/settings/workspaces");
}
</script>

<template>
  <WorkspaceForm @saved="handleSaved" />
</template>
