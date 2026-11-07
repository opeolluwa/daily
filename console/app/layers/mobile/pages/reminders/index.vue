<script setup lang="ts">
import { useReminderStore } from "@shared/stores/reminder";
import ReminderCard from "@mobile/components/reminder/reminder-card.vue";

definePageMeta({ name: "Reminders" });

const reminderStore = useReminderStore();
const router = useRouter();
const { searchQuery, clearSearch } = useAppSearch();
const { notify } = useAppNotification();

const filter = ref<"all" | "upcoming" | "recurring">("all");
const filters = ["all", "upcoming", "recurring"] as const;

const filteredReminders = computed(() => {
  let list = reminderStore.reminders;

  if (filter.value === "upcoming") list = reminderStore.upcomingReminders;
  if (filter.value === "recurring") list = reminderStore.recurringReminders;

  const query = searchQuery.value.trim().toLowerCase();
  if (query) {
    list = list.filter(
      (reminder) =>
        reminder.title.toLowerCase().includes(query) ||
        reminder.description?.toLowerCase().includes(query),
    );
  }

  return list;
});

function openCreate() {
  router.push("/reminders/create-reminder");
}

function openEdit(identifier: string) {
  router.push({ path: "/reminders/edit-reminder", query: { id: identifier } });
}

async function handleDelete(identifier: string) {
  await reminderStore.deleteReminder(identifier);
  notify({ message: "Reminder moved to recycle bin", type: "success" });
}

function clearFilters() {
  searchQuery.value = "";
  filter.value = "all";
}

onMounted(() => {
  reminderStore.fetchReminders();
});

onUnmounted(() => clearSearch());
</script>

<template>
  <RyderPullToRefresh @refresh="() => reminderStore.fetchReminders()">
    <div>
      <!-- Create reminder FAB -->
      <RyderFab
        v-if="!reminderStore.loading && reminderStore.reminders.length > 0"
        class="!bottom-24"
        @click="openCreate"
      />

      <!-- Loading -->
      <div v-if="reminderStore.loading" class="flex flex-col gap-2">
        <USkeleton v-for="i in 4" :key="i" class="h-20 rounded-lg" />
      </div>

      <template v-else>
        <!-- Empty state: no reminders at all -->
        <div v-if="reminderStore.reminders.length === 0">
          <RyderEmptyState
            title="No reminders yet"
            description="Create your first reminder to stay on track."
            icon="ri:alarm-line"
            action-label="create reminder"
            @action="openCreate"
          />
        </div>

        <template v-else>
          <!-- Filter tabs -->
          <div class="mb-4 flex gap-2">
            <button
              v-for="option in filters"
              :key="option"
              class="rounded-full px-3 py-1.5 text-xs font-medium capitalize transition-colors"
              :class="
                filter === option
                  ? 'bg-primary-500 text-white'
                  : 'bg-gray-100 text-gray-600 hover:bg-gray-200 dark:bg-gray-800 dark:text-gray-400 dark:hover:bg-gray-700'
              "
              @click="filter = option"
            >
              {{ option }}
            </button>
          </div>

          <!-- Empty state: filter/search has no results -->
          <RyderEmptyState
            v-if="filteredReminders.length === 0"
            title="No results found"
            description="Try a different search term or filter."
            icon="heroicons:magnifying-glass"
            action-label="clear filters"
            @action="clearFilters"
          />

          <!-- Reminder list -->
          <div v-else class="flex flex-col gap-2">
            <ReminderCard
              v-for="reminder in filteredReminders"
              :key="reminder.identifier"
              :reminder="reminder"
              @edit="openEdit"
              @delete="handleDelete"
            />
          </div>
        </template>
      </template>
    </div>
  </RyderPullToRefresh>
</template>
