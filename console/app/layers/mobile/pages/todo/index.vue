<script setup lang="ts">
import { useTodoStore } from "@shared/stores/todo";

definePageMeta({ name: "Tasks" });

const todoStore = useTodoStore();
const router = useRouter();

function openCreate() {
  router.push("/todo/create-todo");
}

function openEdit(identifier: string) {
  router.push({ path: "/todo/edit-todo", query: { id: identifier } });
}

onMounted(() => {
  todoStore.fetchTodos();
});
</script>

<template>
  <RyderPullToRefresh @refresh="() => todoStore.fetchTodos()">
    <div>
      <!-- Create task FAB -->
      <RyderFab
        v-if="todoStore.todos.length !== 0"
        class="!bottom-24"
        @click="openCreate"
      />
      <!-- Loading -->
      <div v-if="todoStore.loading" class="flex flex-col gap-2">
        <USkeleton v-for="i in 4" :key="i" class="h-16 rounded-lg" />
      </div>

      <!-- Empty state: no todos at all -->
      <div v-else-if="todoStore.todos.length === 0">
        <RyderEmptyState
          title="No task yet"
          description="Create your first task to get started."
          icon="ri:calendar-todo-line"
          action-label="create task"
          @action="openCreate"
        />
      </div>

      <!-- Todo list -->
      <div v-else class="flex flex-col gap-2">
        <TodoCard
          v-for="todo in todoStore.todos"
          :key="todo.identifier"
          :todo="todo"
          @toggle="(id, done) => todoStore.toggleDone(id, done)"
          @edit="openEdit"
          @delete="(id) => todoStore.deleteTodo(id)"
        />
      </div>
    </div>
  </RyderPullToRefresh>
</template>
