<script setup lang="ts">
definePageMeta({ layout: false });
import { DnDProvider } from "@vue-dnd-kit/core";
import TodoCard from "@desktop/components/todo/todo-card.vue";
import TodoDialog from "@desktop/components/todo/TodoDialog.vue";
import { useTodoStore } from "@shared/stores/todo";
import {
  TODO_SORT_OPTIONS,
  sortTodos,
  type TodoSort,
} from "@shared/utils/sorting";
import { VueDraggable } from "vue-draggable-plus";

type TodoStatus = "planned" | "in_progress" | "rescheduled" | "done";

const columns: {
  label: string;
  status: TodoStatus;
}[] = [
  { label: "Planned", status: "planned" },
  { label: "In Progress", status: "in_progress" },
  { label: "Rescheduled", status: "rescheduled" },
  { label: "Done", status: "done" },
];

const board = reactive<Record<TodoStatus, Todo[]>>({
  planned: [],
  in_progress: [],
  rescheduled: [],
  done: [],
});

function syncBoard() {
  for (const column of columns) {
    board[column.status] = todoStore.todos.filter(
      (todo) => todo.status === column.status,
    );
  }
}

async function handleColumnChange(
  status: TodoStatus,
  event: {
    added?: {
      element: Todo;
    };
  },
) {
  const todo = event.added?.element;

  if (!todo) return;

  todo.status = status;

  await todoStore.updateTodoStatus(todo.identifier, status);
}

const todoStore = useTodoStore();
const activeTasks = computed(() => todoStore.activeTask);
const plannedTasks = computed(() => todoStore.plannedTasks);
const completedTasks = computed(() => todoStore.completed);

const { searchQuery, setSearch, clearSearch } = useAppSearch();
const filter = ref<"all" | "active" | "completed">("all");
const sortBy = ref<TodoSort>("priority-high");
const dialogOpen = ref(false);
const editingTodo = ref<any>(null);

const filteredTodos = computed(() => {
  let list = todoStore.todos;

  if (filter.value === "active") list = list.filter((t) => !t.done);
  if (filter.value === "completed") list = list.filter((t) => t.done);

  const q = searchQuery.value.trim().toLowerCase();
  if (q) {
    list = list.filter(
      (t) =>
        t.title.toLowerCase().includes(q) ||
        t.description?.toLowerCase().includes(q),
    );
  }

  return sortTodos(list, sortBy.value);
});

async function handleToggle(identifier: string, done: boolean) {
  await todoStore.toggleDone(identifier, done);
}

async function handleDelete(identifier: string) {
  await todoStore.deleteTodo(identifier);
}

async function deleteCompleted() {
  await Promise.all(
    todoStore.completedTodos.map((t) => todoStore.deleteTodo(t.identifier)),
  );
}

function handleEdit(identifier: string) {
  editingTodo.value =
    todoStore.todos.find((t) => t.identifier === identifier) ?? null;
  dialogOpen.value = true;
}

onMounted(async () => {
  setSearch({ placeholder: "Search todos..." });
  await todoStore.fetchTodos();
});

onUnmounted(() => clearSearch());
</script>

<template>
  <NuxtLayout name="default">
    <!-- <template #page_title>
      <h1
        v-if="todoStore.todos.length > 0"
        class="text-2xl font-semibold text-gray-800 dark:text-gray-100"
      >
        Todo
      </h1>
    </template> -->

    <!-- <template #primary_cta>
      <AppPrimaryCta
        v-if="todoStore.todos.length !== 0"
        label="New Todo"
        icon="heroicons:plus"
        @click="
          editingTodo = null;
          dialogOpen = true;
        "
      />
    </template> -->

    <template #main_content>
      <!-- Filter tabs -->

      <div
        v-if="!todoStore.loading && todoStore.todos.length > 0"
        class="flex items-center gap-2 mb-4 hidden"
      >
        <div class="flex gap-1 bg-gray-100 dark:bg-gray-800 rounded-lg p-0.5">
          <button
            v-for="f in ['all', 'active', 'completed'] as const"
            :key="f"
            class="px-3 py-1.5 rounded-md text-xs font-medium transition-colors capitalize"
            :class="
              filter === f
                ? 'bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-100 shadow-sm'
                : 'text-gray-500 dark:text-gray-400'
            "
            @click="filter = f"
          >
            {{ f }}
          </button>
        </div>
        <AppSortMenu v-model="sortBy" :options="TODO_SORT_OPTIONS" />
        <button
          v-if="todoStore.completedTodos.length > 0"
          class="ml-auto flex items-center gap-1.5 text-xs text-red-400 hover:text-red-500 transition-colors"
          @click="deleteCompleted"
        >
          <UIcon name="heroicons:trash" class="size-3.5" />
          Clear done ({{ todoStore.completedTodos.length }})
        </button>
      </div>

      <!-- Loading -->
      <div v-if="todoStore.loading" class="flex flex-col gap-2">
        <USkeleton v-for="i in 4" :key="i" class="h-16 rounded-lg" />
      </div>

      <RyderEmptyState
        v-else-if="todoStore.todos.length === 0"
        title="No todos yet"
        description="Create your first todo to get started."
        icon="heroicons:check-circle"
        action-label="Create todo"
        @action="
          editingTodo = null;
          dialogOpen = true;
        "
      />

      <RyderEmptyState
        v-else-if="filteredTodos.length === 0"
        title="No results found"
        description="Try a different search or filter."
        icon="heroicons:magnifying-glass"
        action-label="Clear filters"
        @action="
          searchQuery = '';
          filter = 'all';
        "
      />

      <!-- Todo list -->

      <DnDProvider>
         <TaskBoard />
      </DnDProvider>

      <!-- <div v-else class="flex flex-col gap-2">
        <TodoCard
          v-for="todo in filteredTodos"
          :key="todo.identifier"
          :todo="todo"
          @toggle="handleToggle"
          @edit="handleEdit"
          @delete="handleDelete"
        />
      </div> -->
    </template>

    <template #modal_content>
      <TodoDialog v-model:open="dialogOpen" :todo="editingTodo" />
    </template>
  </NuxtLayout>
</template>
