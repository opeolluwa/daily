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

const todoStore = useTodoStore();

const { searchQuery, setSearch, clearSearch } = useAppSearch();
const filter = ref<"all" | "active" | "completed">("all");
const sortBy = ref<TodoSort>("priority-high");
const dialogOpen = ref(false);
const editingTodo = ref<any>(null);
const activeTab = ref("1");
const isListViewMode = computed(() => activeTab.value === "0");
const isEmptyList = computed(() => !todoStore.todos.length);
import type { TabsItem } from "@nuxt/ui";

const items = [
  {
    label: "List",
    slot: "list" as const,
  },
  {
    label: "Board",
    slot: "board" as const,
  },
] satisfies TabsItem[];

const filteredTodos = computed(() => {
  let list = todoStore.todos;

  if (filter.value === "active") list = list.filter((t: Todo) => !t.done);
  if (filter.value === "completed") list = list.filter((t: Todo) => t.done);

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
    todoStore.todos.find((t: Todo) => t.identifier === identifier) ?? null;
  dialogOpen.value = true;
}

onMounted(async () => {
  setSearch({ placeholder: "Search todos..." });
  await todoStore.fetchTodos();
});

onUnmounted(() => clearSearch());
</script>

<template>
  <NuxtLayout name="default" :show-side-content="isListViewMode ^isEmptyList">
    <template #page_title>
      <RyderPageHeader
        title="Task"
        v-if="todoStore.todos.length"
        subtitle="Plan you tasks"
        cta-text="New Todo"
        cta-icon="heroicons:plus"
        @cta="
          editingTodo = null;
          dialogOpen = true;
        "
      />
    </template>

    <template #main_content>
      <div
        v-if="
          !todoStore.loading && todoStore.todos.length > 0 && isListViewMode
        "
        class="flex items-center gap-2 mb-4"
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

      <!-- Todo list -->
      <UTabs
        v-if="todoStore.todos.length"
        color="neutral"
        variant="link"
        :items="items"
        :content="true"
        class="w-full"
        v-model:model-value="activeTab"
      >
        <template #list>
          <div class="flex flex-col gap-2">
            <TodoCard
              v-for="todo in filteredTodos"
              :key="todo.identifier"
              :todo="todo"
              @toggle="handleToggle"
              @edit="handleEdit"
              @delete="handleDelete"
            /></div
        ></template>
        <template #board>
          <DnDProvider>
            <TaskBoard
              @edit="handleEdit"
              @delete="handleDelete"
              @toggle="handleToggle"
            />
          </DnDProvider>
        </template>
      </UTabs>
    </template>

    <template #side_content>

        <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
          Summary
        </h2>
        <div class="flex flex-col gap-3 mb-6">
          <div class="bg-primary-50 dark:bg-primary-950 rounded-lg p-3">
            <p
              class="text-2xl font-semibold text-primary-700 dark:text-primary-300"
            >
              {{ todoStore.activeTodos.length }}
            </p>
            <p class="text-xs text-primary-500 dark:text-primary-400">
              Active tasks
            </p>
          </div>
          <div class="bg-emerald-50 dark:bg-emerald-950 rounded-lg p-3">
            <p
              class="text-2xl font-semibold text-emerald-700 dark:text-emerald-300"
            >
              {{ todoStore.completedTodos.length }}
            </p>
            <p class="text-xs text-emerald-500 dark:text-emerald-400">
              Completed
            </p>
          </div>
        </div>

        <USeparator class="my-4" />

        <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
          Priority
        </h2>
        <div class="flex flex-col gap-2">
          <div class="flex items-center justify-between text-sm">
            <div class="flex items-center gap-2">
              <UIcon name="heroicons:flag" class="size-4 text-rose-500" />
              <span class="text-gray-600 dark:text-gray-400">High</span>
            </div>
            <span class="text-xs text-gray-400">{{
              todoStore.highPriorityCount
            }}</span>
          </div>
          <div class="flex items-center justify-between text-sm">
            <div class="flex items-center gap-2">
              <UIcon name="heroicons:flag" class="size-4 text-amber-500" />
              <span class="text-gray-600 dark:text-gray-400">Medium</span>
            </div>
            <span class="text-xs text-gray-400">{{
              todoStore.mediumPriorityCount
            }}</span>
          </div>
          <div class="flex items-center justify-between text-sm">
            <div class="flex items-center gap-2">
              <UIcon name="heroicons:flag" class="size-4 text-emerald-500" />
              <span class="text-gray-600 dark:text-gray-400">Low</span>
            </div>
            <span class="text-xs text-gray-400">{{
              todoStore.lowPriorityCount
            }}</span>
          </div>
        </div>
      </template>


    <template #modal_content>
      <TodoDialog v-model:open="dialogOpen" :todo="editingTodo" />
    </template>
  </NuxtLayout>
</template>
