<script setup lang="ts">
import type { Task } from "~/stores/tasks";
import type { Reminder } from "~/stores/reminder";
import { useTaskStore } from "~/stores/tasks";
import { useReminderStore } from "~/stores/reminder";

definePageMeta({ layout: false });

const taskStore = useTaskStore();
const reminderStore = useReminderStore();

const today = new Date();

function formatDate(year: number, month: number, day: number): string {
  const d = new Date(year, month, day);
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, "0")}-${String(d.getDate()).padStart(2, "0")}`;
}

const todayStr = formatDate(
  today.getFullYear(),
  today.getMonth(),
  today.getDate(),
);

const currentYear = ref(today.getFullYear());
const currentMonth = ref(today.getMonth());
const selectedDate = ref<string>(todayStr);

const monthNames = [
  "January", "February", "March", "April", "May", "June",
  "July", "August", "September", "October", "November", "December",
];
const dayNames = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

interface CalendarItem {
  id: string;
  title: string;
  date: string;
  type: "task" | "reminder";
  recurring?: boolean;
  task?: Task;
  reminder?: Reminder;
}

function parseDate(dateStr: string): Date {
  const [y, m, d] = dateStr.split("-").map(Number);
  return new Date(y, m - 1, d);
}

function matchesRecurrence(
  rule: string,
  startDate: Date,
  candidateDate: Date,
): boolean {
  if (candidateDate < startDate) return false;

  const dayOfWeek = candidateDate.getDay();

  switch (rule) {
    case "daily":
      return true;
    case "weekly":
      return dayOfWeek === startDate.getDay();
    case "biweekly": {
      if (dayOfWeek !== startDate.getDay()) return false;
      const diffMs = candidateDate.getTime() - startDate.getTime();
      const diffWeeks = Math.round(diffMs / (7 * 24 * 60 * 60 * 1000));
      return diffWeeks % 2 === 0;
    }
    case "monthly":
      return candidateDate.getDate() === startDate.getDate();
    case "yearly":
      return (
        candidateDate.getDate() === startDate.getDate() &&
        candidateDate.getMonth() === startDate.getMonth()
      );
    case "weekdays":
      return dayOfWeek >= 1 && dayOfWeek <= 5;
    default:
      return false;
  }
}

const visibleDates = computed(() => calendarDays.value.map((d) => d.date));

const calendarItems = computed<CalendarItem[]>(() => {
  const items: CalendarItem[] = [];

  for (const task of taskStore.tasks) {
    if (task.dueDate) {
      items.push({
        id: `task-${task.identifier}`,
        title: task.title,
        date: task.dueDate,
        type: "task",
        task,
      });
    }
  }

  for (const reminder of reminderStore.reminders) {
    if (!reminder.remindAt) continue;

    const originDate = reminder.remindAt.slice(0, 10);
    const endDateStr = reminder.endDate ?? null;
    const rule = reminder.recurrenceRule;

    if (reminder.recurring && rule) {
      const startDate = parseDate(originDate);
      const endDate = endDateStr ? parseDate(endDateStr) : null;
      for (const dateStr of visibleDates.value) {
        const candidate = parseDate(dateStr);
        if (endDate && candidate > endDate) continue;
        if (matchesRecurrence(rule, startDate, candidate)) {
          items.push({
            id: `reminder-${reminder.identifier}-${dateStr}`,
            title: reminder.title,
            date: dateStr,
            type: "reminder",
            recurring: true,
            reminder,
          });
        }
      }
    } else if (endDateStr) {
      for (const dateStr of visibleDates.value) {
        if (dateStr >= originDate && dateStr <= endDateStr) {
          items.push({
            id: `reminder-${reminder.identifier}-${dateStr}`,
            title: reminder.title,
            date: dateStr,
            type: "reminder",
            reminder,
          });
        }
      }
    } else {
      items.push({
        id: `reminder-${reminder.identifier}`,
        title: reminder.title,
        date: originDate,
        type: "reminder",
        reminder,
      });
    }
  }

  return items;
});

function itemsOnDate(date: string): CalendarItem[] {
  return calendarItems.value.filter((item) => item.date === date);
}

const calendarDays = computed(() => {
  const year = currentYear.value;
  const month = currentMonth.value;
  const firstDay = new Date(year, month, 1).getDay();
  const daysInMonth = new Date(year, month + 1, 0).getDate();
  const daysInPrevMonth = new Date(year, month, 0).getDate();

  const days: { date: string; day: number; isCurrentMonth: boolean }[] = [];

  for (let i = firstDay - 1; i >= 0; i--) {
    const d = daysInPrevMonth - i;
    const prevMonth = month === 0 ? 11 : month - 1;
    const prevYear = month === 0 ? year - 1 : year;
    days.push({ date: formatDate(prevYear, prevMonth, d), day: d, isCurrentMonth: false });
  }

  for (let d = 1; d <= daysInMonth; d++) {
    days.push({ date: formatDate(year, month, d), day: d, isCurrentMonth: true });
  }

  const remaining = 42 - days.length;
  for (let d = 1; d <= remaining; d++) {
    const nextMonth = month === 11 ? 0 : month + 1;
    const nextYear = month === 11 ? year + 1 : year;
    days.push({ date: formatDate(nextYear, nextMonth, d), day: d, isCurrentMonth: false });
  }

  return days;
});

function prevMonth() {
  if (currentMonth.value === 0) {
    currentMonth.value = 11;
    currentYear.value--;
  } else currentMonth.value--;
}

function nextMonth() {
  if (currentMonth.value === 11) {
    currentMonth.value = 0;
    currentYear.value++;
  } else currentMonth.value++;
}

function goToToday() {
  currentYear.value = today.getFullYear();
  currentMonth.value = today.getMonth();
  selectedDate.value = todayStr;
}

const selectedDateItems = computed(() => itemsOnDate(selectedDate.value));

const upcomingItems = computed(() =>
  [...calendarItems.value]
    .filter((item) => item.date >= todayStr)
    .sort((a, b) => a.date.localeCompare(b.date))
    .slice(0, 5),
);

function formatDisplayDate(dateStr: string): string {
  const [year, month, day] = dateStr.split("-").map(Number);
  return new Date(year, month - 1, day).toLocaleDateString("en-US", {
    weekday: "long",
    month: "long",
    day: "numeric",
    year: "numeric",
  });
}

// Context menu
const contextMenu = ref(false);
const contextMenuPos = ref({ x: 0, y: 0 });
const contextDate = ref<string>("");

function openContextMenu(event: MouseEvent, date: string) {
  event.preventDefault();
  contextDate.value = date;
  contextMenuPos.value = { x: event.clientX, y: event.clientY };
  contextMenu.value = true;
}

function closeContextMenu() {
  contextMenu.value = false;
}

// Dialogs
const taskDialogOpen = ref(false);
const reminderDialogOpen = ref(false);
const editingTask = ref<Task | null>(null);
const editingReminder = ref<Reminder | null>(null);
const dialogInitialDate = ref<string | null>(null);

function addTaskFromMenu() {
  editingTask.value = null;
  dialogInitialDate.value = contextDate.value;
  taskDialogOpen.value = true;
  closeContextMenu();
}

function addReminderFromMenu() {
  editingReminder.value = null;
  dialogInitialDate.value = contextDate.value;
  reminderDialogOpen.value = true;
  closeContextMenu();
}

function openEditItem(item: CalendarItem) {
  dialogInitialDate.value = null;
  if (item.type === "task" && item.task) {
    editingTask.value = item.task;
    taskDialogOpen.value = true;
  } else if (item.type === "reminder" && item.reminder) {
    editingReminder.value = item.reminder;
    reminderDialogOpen.value = true;
  }
}

function priorityColor(priority: string) {
  switch (priority) {
    case "high": return "text-rose-500";
    case "medium": return "text-amber-500";
    case "low": return "text-emerald-500";
    default: return "text-gray-400";
  }
}

onMounted(() => {
  taskStore.fetchTasks();
  reminderStore.fetchReminders();
});
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <!-- Month navigation -->
      <div class="flex items-center justify-between mb-6">
        <div class="flex items-center gap-2">
          <button
            class="p-1.5 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
            @click="prevMonth"
          >
            <UIcon name="heroicons:chevron-left" class="size-4 text-gray-500" />
          </button>
          <h2 class="text-base font-semibold text-gray-800 dark:text-gray-100 w-40 text-center">
            {{ monthNames[currentMonth] }} {{ currentYear }}
          </h2>
          <button
            class="p-1.5 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
            @click="nextMonth"
          >
            <UIcon name="heroicons:chevron-right" class="size-4 text-gray-500" />
          </button>
        </div>
        <button
          class="px-3 py-1.5 text-xs font-medium text-primary-600 dark:text-primary-400 border border-primary-200 dark:border-primary-800 rounded-lg hover:bg-primary-50 dark:hover:bg-primary-950 transition-colors"
          @click="goToToday"
        >
          Today
        </button>
      </div>

      <!-- Day headers -->
      <div class="grid grid-cols-7 mb-1">
        <div
          v-for="day in dayNames"
          :key="day"
          class="text-center text-xs font-medium text-gray-400 dark:text-gray-500 py-2"
        >
          {{ day }}
        </div>
      </div>

      <!-- Calendar grid -->
      <div class="grid grid-cols-7 gap-px bg-gray-100 dark:bg-gray-800 rounded-xl overflow-hidden border border-gray-100 dark:border-gray-800">
        <button
          v-for="cell in calendarDays"
          :key="cell.date"
          class="relative bg-white dark:bg-gray-900 min-h-20 p-2 text-left transition-colors hover:bg-gray-50 dark:hover:bg-gray-800/80"
          :class="{ 'opacity-40': !cell.isCurrentMonth }"
          @click="selectedDate = cell.date"
          @contextmenu="openContextMenu($event, cell.date)"
        >
          <span
            class="inline-flex items-center justify-center size-6 rounded-full text-sm mb-1 transition-colors"
            :class="[
              cell.date === todayStr
                ? 'bg-primary-500 text-white font-semibold'
                : selectedDate === cell.date
                  ? 'bg-gray-100 dark:bg-gray-700 font-medium text-gray-800 dark:text-gray-100'
                  : 'text-gray-600 dark:text-gray-400',
            ]"
          >
            {{ cell.day }}
          </span>
          <div class="flex flex-col gap-0.5">
            <div
              v-for="item in itemsOnDate(cell.date).slice(0, 2)"
              :key="item.id"
              class="flex items-center gap-1 text-xs px-1.5 py-0.5 rounded truncate"
              :class="
                item.type === 'task'
                  ? 'bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300'
                  : 'bg-amber-100 dark:bg-amber-900 text-amber-700 dark:text-amber-300'
              "
            >
              <UIcon
                :name="item.type === 'task' ? 'heroicons:check-circle' : item.recurring ? 'heroicons:arrow-path' : 'heroicons:bell'"
                class="size-3 shrink-0"
              />
              <span class="truncate">{{ item.title }}</span>
            </div>
            <span
              v-if="itemsOnDate(cell.date).length > 2"
              class="text-xs text-gray-400 px-1"
            >
              +{{ itemsOnDate(cell.date).length - 2 }} more
            </span>
          </div>
        </button>
      </div>

      <!-- Context menu -->
      <Teleport to="body">
        <div
          v-if="contextMenu"
          class="fixed inset-0 z-50"
          @click="closeContextMenu"
          @contextmenu.prevent="closeContextMenu"
        >
          <div
            class="absolute z-50 w-48 py-1 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg shadow-lg"
            :style="{ left: `${contextMenuPos.x}px`, top: `${contextMenuPos.y}px` }"
            @click.stop
          >
            <button
              class="flex items-center gap-2.5 w-full px-3 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
              @click="addTaskFromMenu"
            >
              <UIcon name="heroicons:check-circle" class="size-4 text-primary-500" />
              Add Task
            </button>
            <button
              class="flex items-center gap-2.5 w-full px-3 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
              @click="addReminderFromMenu"
            >
              <UIcon name="heroicons:bell" class="size-4 text-amber-500" />
              Add Reminder
            </button>
          </div>
        </div>
      </Teleport>
    </template>

    <template #side_content>
      <!-- Selected day -->
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        {{ formatDisplayDate(selectedDate) }}
      </h2>

      <div v-if="selectedDateItems.length" class="flex flex-col gap-2 mb-6">
        <button
          v-for="item in selectedDateItems"
          :key="item.id"
          class="flex items-start gap-2.5 p-2.5 rounded-lg bg-gray-50 dark:bg-gray-800 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors text-left w-full"
          @click="openEditItem(item)"
        >
          <UIcon
            :name="item.type === 'task' ? 'heroicons:check-circle' : item.recurring ? 'heroicons:arrow-path' : 'heroicons:bell'"
            class="size-4 mt-0.5 shrink-0"
            :class="item.type === 'task' ? 'text-primary-500' : 'text-amber-500'"
          />
          <div class="flex-1 min-w-0">
            <p class="text-sm text-gray-700 dark:text-gray-300 truncate">
              {{ item.title }}
            </p>
            <div class="flex items-center gap-2 mt-0.5">
              <span class="text-xs text-gray-400 capitalize">{{ item.type }}</span>
              <span
                v-if="item.recurring"
                class="text-xs text-amber-500 capitalize"
              >
                {{ item.reminder?.recurrenceRule }}
              </span>
              <span
                v-if="item.type === 'task' && item.task"
                class="text-xs font-medium capitalize"
                :class="priorityColor(item.task.priority)"
              >
                {{ item.task.priority }}
              </span>
              <span
                v-if="item.type === 'reminder' && item.reminder"
                class="text-xs text-gray-400"
              >
                {{ new Date(item.reminder.remindAt).toLocaleTimeString("en-US", { hour: "numeric", minute: "2-digit" }) }}
              </span>
            </div>
          </div>
        </button>
      </div>
      <p v-else class="text-xs text-gray-400 mb-6">No items scheduled</p>

      <USeparator class="my-4" />

      <!-- Upcoming -->
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Upcoming
      </h2>
      <div v-if="upcomingItems.length" class="flex flex-col gap-1">
        <button
          v-for="item in upcomingItems"
          :key="item.id"
          class="flex items-start gap-3 py-2.5 px-3 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors text-left w-full"
          @click="selectedDate = item.date; openEditItem(item)"
        >
          <UIcon
            :name="item.type === 'task' ? 'heroicons:check-circle' : 'heroicons:bell'"
            class="size-4 mt-0.5 shrink-0"
            :class="item.type === 'task' ? 'text-primary-500' : 'text-amber-500'"
          />
          <div class="flex-1 min-w-0">
            <p class="text-sm font-medium text-gray-700 dark:text-gray-300 truncate">
              {{ item.title }}
            </p>
            <p class="text-xs text-gray-400">
              {{ formatDisplayDate(item.date) }}
            </p>
          </div>
        </button>
      </div>
      <p v-else class="text-xs text-gray-400">No upcoming items</p>
    </template>

    <template #modal_content>
      <TaskDialog
        v-model:open="taskDialogOpen"
        :task="editingTask"
        :initial-date="dialogInitialDate"
      />
      <ReminderDialog
        v-model:open="reminderDialogOpen"
        :reminder="editingReminder"
        :initial-date="dialogInitialDate"
      />
    </template>
  </NuxtLayout>
</template>
