<script setup lang="ts">
import TodoForm from "@mobile/components/todo/form.vue";
import { useTodoStore } from "@shared/stores/todo";

definePageMeta({ name: "Edit task" });

const route = useRoute();
const router = useRouter();
const todoStore = useTodoStore();
const { notify } = useAppNotification();

const id = computed(() =>
  typeof route.query.id === "string" ? route.query.id : undefined,
);

const todo = computed(
  () => todoStore.todos.find((entry) => entry.identifier === id.value) ?? null,
);

function handleSaved() {
  notify({ message: "Task updated", type: "success" });
  router.replace("/todo");
}

onMounted(async () => {
  if (todoStore.todos.length === 0) {
    await todoStore.fetchTodos();
  }
});
</script>

<template>
  <div>
    <div v-if="todoStore.loading && !todo" class="flex flex-col gap-4">
      <USkeleton class="h-16 rounded-lg" />
      <USkeleton class="h-24 rounded-lg" />
      <USkeleton class="h-10 rounded-lg" />
    </div>

    <div
      v-else-if="!todo"
      class="flex flex-col items-center justify-center py-20 text-center"
    >
      <div class="mb-4 p-3 rounded-full bg-gray-100 dark:bg-gray-800">
        <UIcon name="heroicons:calendar" class="size-7 text-gray-400" />
      </div>
      <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
        Task not found
      </h3>
      <button
        class="text-xs text-primary-500 hover:text-primary-600 font-medium mt-2"
        @click="router.push('/todo')"
      >
        Back to tasks
      </button>
    </div>

    <TodoForm v-else :todo="todo" @saved="handleSaved" />
  </div>
</template>
