<script setup lang="ts">
import { useReminderStore, type Reminder } from "~/stores/reminder";
import ReminderDialog from "~/components/reminders/ReminderDialog.vue";

definePageMeta({ layout: false });

const reminderStore = useReminderStore();
const { searchQuery, setSearch, clearSearch } = useAppSearch();
const filter = ref<"all" | "upcoming" | "recurring">("all");
const dialogOpen = ref(false);
const editingReminder = ref<Reminder | null>(null);

const filteredReminders = computed(() => {
  let list = reminderStore.reminders;

  if (filter.value === "upcoming") list = reminderStore.upcomingReminders;
  if (filter.value === "recurring") list = reminderStore.recurringReminders;

  const q = searchQuery.value.trim().toLowerCase();
  if (q) {
    list = list.filter(
      (r) =>
        r.title.toLowerCase().includes(q) ||
        r.description?.toLowerCase().includes(q),
    );
  }

  return list;
});

function formatRemindAt(remindAt: string): string {
  return new Date(remindAt).toLocaleString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
    hour: "numeric",
    minute: "2-digit",
  });
}

async function handleDelete(identifier: string) {
  await reminderStore.deleteReminder(identifier);
}

function handleEdit(identifier: string) {
  editingReminder.value =
    reminderStore.reminders.find((r) => r.identifier === identifier) ?? null;
  dialogOpen.value = true;
}

onMounted(async () => {
  setSearch({ placeholder: "Search reminders..." });
  await reminderStore.fetchReminders();
});

onUnmounted(() => clearSearch());
</script>

<template>
  <NuxtLayout name="default">
    <template #page_title>
      <h1
        v-if="reminderStore.reminders.length > 0"
        class="text-2xl font-semibold text-gray-800 dark:text-gray-100"
      >
        Reminders
      </h1>
    </template>

    <template #primary_cta>
      <AppPrimaryCta
        v-if="reminderStore.reminders.length !== 0"
        label="New Reminder"
        icon="heroicons:plus"
        @click="
          editingReminder = null;
          dialogOpen = true;
        "
      />
    </template>

    <template #main_content>
      <!-- Filter tabs -->
      <div
        v-if="!reminderStore.loading && reminderStore.reminders.length > 0"
        class="flex gap-1 bg-gray-100 dark:bg-gray-800 rounded-lg p-0.5 mb-4 w-fit"
      >
        <button
          v-for="f in ['all', 'upcoming', 'recurring'] as const"
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

      <!-- Loading -->
      <div v-if="reminderStore.loading" class="flex flex-col gap-2">
        <USkeleton v-for="i in 4" :key="i" class="h-16 rounded-lg" />
      </div>

      <RyderEmptyState
        v-else-if="reminderStore.reminders.length === 0"
        title="No reminders yet"
        description="Create your first reminder to stay on track."
        icon="heroicons:clock"
        action-label="Create reminder"
        @action="
          editingReminder = null;
          dialogOpen = true;
        "
      />

      <RyderEmptyState
        v-else-if="filteredReminders.length === 0"
        title="No results found"
        description="Try a different search or filter."
        icon="heroicons:magnifying-glass"
        action-label="Clear filters"
        @action="
          searchQuery = '';
          filter = 'all';
        "
      />

      <!-- Reminder list -->
      <div v-else class="flex flex-wrap gap-3">
        <RyderCard
          v-for="reminder in filteredReminders"
          :key="reminder.identifier"
          :ui="{ root: '!p-0 overflow-hidden cursor-pointer w-72 hover:shadow-sm transition-shadow' }"
          @click="handleEdit(reminder.identifier)"
        >
          <div class="p-4 flex items-center gap-3">
            <div
              class="shrink-0 size-10 rounded-lg flex items-center justify-center"
              :class="
                reminder.recurring
                  ? 'bg-primary-100 dark:bg-primary-900'
                  : 'bg-gray-100 dark:bg-gray-800'
              "
            >
              <UIcon
                :name="reminder.recurring ? 'heroicons:arrow-path' : 'heroicons:bell'"
                class="size-5"
                :class="
                  reminder.recurring
                    ? 'text-primary-600 dark:text-primary-400'
                    : 'text-gray-400 dark:text-gray-500'
                "
              />
            </div>
            <div class="min-w-0 flex-1">
              <h3 class="text-sm font-medium text-gray-800 dark:text-gray-200 truncate">
                {{ reminder.title }}
              </h3>
              <p class="text-xs text-gray-400 mt-0.5">
                {{ formatRemindAt(reminder.remindAt) }}
              </p>
            </div>
          </div>

          <div
            class="px-4 py-2 border-t border-gray-50 dark:border-gray-700 flex items-center justify-between"
          >
            <p v-if="reminder.recurring && reminder.recurrenceRule" class="text-xs text-primary-500 capitalize">
              {{ reminder.recurrenceRule }}
            </p>
            <p v-else class="text-xs text-gray-400">One-time</p>
            <div class="flex items-center gap-2" @click.stop>
              <button
                class="text-xs font-medium text-primary-600 dark:text-primary-400 hover:text-primary-700 transition-colors"
                @click="handleEdit(reminder.identifier)"
              >
                Edit
              </button>
              <button
                class="p-1 rounded-md text-gray-400 hover:text-rose-500 hover:bg-rose-50 dark:hover:bg-rose-950 transition-colors"
                @click="handleDelete(reminder.identifier)"
              >
                <UIcon name="heroicons:trash" class="size-3.5" />
              </button>
            </div>
          </div>
        </RyderCard>
      </div>
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
            {{ reminderStore.upcomingReminders.length }}
          </p>
          <p class="text-xs text-primary-500 dark:text-primary-400">Upcoming</p>
        </div>
        <div class="bg-violet-50 dark:bg-violet-950 rounded-lg p-3">
          <p
            class="text-2xl font-semibold text-violet-700 dark:text-violet-300"
          >
            {{ reminderStore.recurringReminders.length }}
          </p>
          <p class="text-xs text-violet-500 dark:text-violet-400">Recurring</p>
        </div>
      </div>

      <USeparator class="my-4" />

      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Total
      </h2>
      <p class="text-2xl font-semibold text-gray-700 dark:text-gray-200">
        {{ reminderStore.reminders.length }}
      </p>
      <p class="text-xs text-gray-400 dark:text-gray-500">reminders</p>
    </template>

    <template #modal_content>
      <ReminderDialog v-model:open="dialogOpen" :reminder="editingReminder" />
    </template>
  </NuxtLayout>
</template>
