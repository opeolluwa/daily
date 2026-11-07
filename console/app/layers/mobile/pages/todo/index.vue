<script setup lang="ts">
import { useTodoStore } from "@shared/stores/todo";

definePageMeta({ name: "Tasks" });

const todoStore = useTodoStore();
const { notify } = useAppNotification();

const showCreatePopup = ref(false);

onMounted(() => {
  todoStore.fetchTodos();
});

function handleCreated() {
  notify({ message: "Task created", type: "success" });
}
</script>

<template>
  <RyderPullToRefresh @refresh="() => todoStore.fetchTodos()">
    <div>
      <!-- Create task FAB -->
      <RyderFab
        v-if="todoStore.todos.length !== 0"
        class="!bottom-24"
        @click="showCreatePopup = true"
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
          @action="showCreatePopup = true"
        />
      </div>

      <!-- Todo list -->
      <div v-else class="flex flex-col gap-2">
        <TodoCard
          v-for="todo in todoStore.todos"
          :key="todo.identifier"
          :todo="todo"
          @toggle="(id, done) => todoStore.toggleDone(id, done)"
          @delete="(id) => todoStore.deleteTodo(id)"
        />
      </div>

      <!-- Create task popup -->
      <TodoCreatePopup
        v-model:open="showCreatePopup"
        @created="handleCreated"
      />
    </div>
  </RyderPullToRefresh>
</template>
