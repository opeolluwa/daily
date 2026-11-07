<script setup lang="ts">
import SnippetForm from "@mobile/components/snippet/form.vue";

definePageMeta({ name: "New snippet" });

const router = useRouter();
const { notify } = useAppNotification();

function handleSaved() {
  notify({ message: "Snippet created", type: "success" });
  router.replace("/snippets");
}
</script>

<template>
  <SnippetForm @saved="handleSaved" />
</template>
