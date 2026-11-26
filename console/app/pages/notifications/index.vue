<script setup lang="ts">
import type { NotificationType } from "@daily/core";
import { ryderPageHeaderOverride } from "~/assets/ryder.css";

definePageMeta({ layout: false });

type NotificationCategory = "system" | "activity" | "alert";

const categoryConfig: Record<
  NotificationCategory,
  { icon: string; color: string; bg: string; label: string }
> = {
  activity: {
    icon: "heroicons:bolt",
    color: "text-surface-500 dark:text-surface-300",
    bg: "bg-surface-100 dark:bg-surface-900",
    label: "Activity",
  },
  system: {
    icon: "heroicons:cog-6-tooth",
    color: "text-gray-500 dark:text-gray-400",
    bg: "bg-gray-100 dark:bg-gray-800",
    label: "System",
  },
  alert: {
    icon: "heroicons:exclamation-triangle",
    color: "text-primary-500 dark:text-primary-400",
    bg: "bg-primary-50 dark:bg-primary-950",
    label: "Alert",
  },
};

function getCategory(type: NotificationType): NotificationCategory {
  if (type === "backup_failed") return "alert";
  if (type.startsWith("workspace_invite") || type.startsWith("item"))
    return "activity";
  return "system";
}

const notificationStore = useNotificationStore();
const notifications = computed(() => notificationStore.notifications);

type FilterTab = "all" | "unread";
const filter = ref<FilterTab>("all");
const activeCategory = ref<NotificationCategory | "all">("all");

const filtered = computed(() => {
  let list = notifications.value;
  if (filter.value === "unread") list = list.filter((n) => !n.isRead);
  if (activeCategory.value !== "all")
    list = list.filter(
      (n) => getCategory(n.notificationType) === activeCategory.value,
    );
  return list;
});

const unreadCount = computed(
  () => notifications.value.filter((n) => !n.isRead).length,
);

const categoryCounts = computed(() =>
  (["activity", "system", "alert"] as NotificationCategory[]).map((c) => ({
    key: c,
    count: notifications.value.filter(
      (n) => getCategory(n.notificationType) === c,
    ).length,
  })),
);

function markRead(identifier: string) {
  notificationStore.markAsRead(identifier);
}

function dismiss(identifier: string) {
  notificationStore.deleteNotification(identifier);
}

function markAllRead() {
  notificationStore.markAllAsRead();
}

function clearAll() {
  notifications.value.forEach((n) =>
    notificationStore.deleteNotification(n.identifier),
  );
}

function relativeTime(iso: string) {
  const diff = Date.now() - new Date(iso).getTime();
  const mins = Math.floor(diff / 60000);
  if (mins < 1) return "just now";
  if (mins < 60) return `${mins}m ago`;
  const hrs = Math.floor(mins / 60);
  if (hrs < 24) return `${hrs}h ago`;
  const days = Math.floor(hrs / 24);
  return `${days}d ago`;
}
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <RyderPageHeader
        v-if="notificationStore.notifications.length > 0"
        :ui="ryderPageHeaderOverride"
        title="Notifications"
      />
      <!-- Toolbar -->
      <div class="flex items-center justify-between mb-5">
        <div class="flex gap-1 bg-gray-100 dark:bg-gray-800 rounded-lg p-0.5">
          <button
            v-for="f in ['all', 'unread'] as const"
            :key="f"
            class="px-3 py-1.5 rounded-md text-xs font-medium transition-colors capitalize"
            :class="
              filter === f
                ? 'bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-100 shadow-sm'
                : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'
            "
            @click="filter = f"
          >
            {{ f }}
            <span
              v-if="f === 'unread' && unreadCount > 0"
              class="ml-1 px-1.5 py-0.5 rounded-full text-[10px] bg-primary-500 text-white"
            >
              {{ unreadCount }}
            </span>
          </button>
        </div>

        <button
          v-if="unreadCount > 0"
          class="text-xs text-primary-500 hover:text-primary-600 font-medium"
          @click="markAllRead"
        >
          Mark all as read
        </button>
      </div>

      <RyderEmptyState
        v-if="filtered.length === 0"
        :title="filter === 'unread' ? 'All caught up' : 'No notifications'"
        :description="
          filter === 'unread'
            ? 'You have no unread notifications.'
            : 'Nothing here yet.'
        "
        icon="heroicons:bell-slash"
      />

      <!-- Notification list -->
      <div v-else class="flex flex-col gap-2">
        <RyderCard
          v-for="item in filtered"
          :key="item.identifier"
          :ui="{
            root: [
              'group relative flex items-start gap-3 transition-all cursor-pointer',
              item.isRead
                ? ''
                : '!border-primary-100 dark:!border-primary-900 hover:!border-primary-200 dark:hover:!border-primary-800',
            ]
              .filter(Boolean)
              .join(' '),
          }"
          @click="markRead(item.identifier)"
        >
          <!-- unread dot -->
          <span
            v-if="!item.isRead"
            class="absolute top-4 right-4 size-2 rounded-full bg-primary-500"
          />

          <!-- category icon -->
          <div
            class="shrink-0 size-8 rounded-lg flex items-center justify-center mt-0.5"
            :class="categoryConfig[getCategory(item.notificationType)].bg"
          >
            <UIcon
              :name="categoryConfig[getCategory(item.notificationType)].icon"
              class="size-4"
              :class="categoryConfig[getCategory(item.notificationType)].color"
            />
          </div>

          <!-- content -->
          <div class="flex-1 min-w-0 pr-6">
            <div class="flex items-baseline gap-2">
              <p
                class="text-sm truncate"
                :class="
                  item.isRead
                    ? 'font-normal text-gray-600 dark:text-gray-400'
                    : 'font-medium text-gray-800 dark:text-gray-100'
                "
              >
                {{ item.title }}
              </p>
              <span class="text-[11px] text-gray-400 shrink-0">
                {{ relativeTime(item.createdAt) }}
              </span>
            </div>
            <p
              class="text-xs text-gray-400 dark:text-gray-500 mt-0.5 leading-snug"
            >
              {{ item.body }}
            </p>
          </div>

          <!-- dismiss button (hover) -->
          <button
            class="absolute bottom-3.5 right-3.5 opacity-0 group-hover:opacity-100 transition-opacity text-gray-300 hover:text-gray-500 dark:hover:text-gray-300"
            aria-label="Dismiss"
            @click.stop="dismiss(item.identifier)"
          >
            <UIcon name="heroicons:x-mark" class="size-3.5" />
          </button>
        </RyderCard>
      </div>
    </template>
  </NuxtLayout>
</template>
