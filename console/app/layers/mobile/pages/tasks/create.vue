<script setup lang="ts">
import TaskForm from "@mobile/components/task/Form.vue";

definePageMeta({ name: "New task" });

const router = useRouter();
const { notify } = useAppNotification();

function handleSaved() {
  notify({ message: "Task created", type: "success" });
  router.replace("/tasks");
}
</script>

<template>
  <TaskForm @saved="handleSaved" />
</template>
