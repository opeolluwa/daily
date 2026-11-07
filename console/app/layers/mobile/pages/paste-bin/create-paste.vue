<script setup lang="ts">
import PasteBinForm from "@mobile/components/paste-bin/form.vue";

definePageMeta({ name: "New paste" });

const router = useRouter();
const { notify } = useAppNotification();

function handleSaved() {
  notify({ message: "Paste created", type: "success" });
  router.replace("/paste-bin");
}
</script>

<template>
  <PasteBinForm @saved="handleSaved" />
</template>
