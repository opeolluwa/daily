<script setup lang="ts">
import TodoForm from "@mobile/components/todo/form.vue";

definePageMeta({ name: "New task" });

const router = useRouter();
const { notify } = useAppNotification();

function handleSaved() {
  notify({ message: "Task created", type: "success" });
  router.replace("/todo");
}
</script>

<template>
  <TodoForm @saved="handleSaved" />
</template>
