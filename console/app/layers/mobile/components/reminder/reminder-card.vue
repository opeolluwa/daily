<script setup lang="ts">
import type { Reminder } from "@shared/stores/reminder";
import { useReminderStore } from "@shared/stores/reminder";
import { useWorkspacesStore } from "@shared/stores/workspaces";
import MetaControls from "@mobile/components/meta/meta-controls.vue";

const { reminder } = defineProps<{ reminder: Reminder }>();

const emit = defineEmits<{
  edit: [identifier: string];
  delete: [identifier: string];
}>();

const reminderStore = useReminderStore();
const workspaceStore = useWorkspacesStore();
const currentWorkspaceId = computed(() => workspaceStore.activeWorkspaceId);

const isUpcoming = computed(
  () => new Date(reminder.remindAt).getTime() > Date.now(),
);

const formattedRemindAt = computed(() =>
  new Date(reminder.remindAt).toLocaleString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
    hour: "numeric",
    minute: "2-digit",
  }),
);

const handleDuplicate = async (targetWorkspaceId: string) => {
  await reminderStore.duplicateReminder(
    reminder.identifier,
    currentWorkspaceId.value,
    targetWorkspaceId,
  );
};

const handleTransfer = async (targetWorkspaceId: string) => {
  await reminderStore.transferReminder(
    reminder.identifier,
    currentWorkspaceId.value,
    targetWorkspaceId,
  );
};
</script>

<template>
  <RyderCard
    :ui="{
      root: 'group flex items-start gap-3 transition-shadow hover:shadow-sm !rounded-lg !border-gray-100 dark:!border-gray-700 dark:!bg-gray-800',
    }"
  >
    <div
      class="mt-0.5 shrink-0 rounded-md p-1.5"
      :class="
        reminder.recurring
          ? 'bg-primary-50 dark:bg-primary-950'
          : 'bg-gray-100 dark:bg-gray-700'
      "
    >
      <UIcon
        :name="reminder.recurring ? 'heroicons:arrow-path' : 'heroicons:clock'"
        class="size-4"
        :class="
          reminder.recurring
            ? 'text-primary-500'
            : 'text-gray-400 dark:text-gray-500'
        "
      />
    </div>

    <div class="min-w-0 flex-1">
      <div class="flex items-center gap-2">
        <p
          class="truncate text-sm font-medium text-gray-800 dark:text-gray-200"
        >
          {{ reminder.title }}
        </p>
        <span
          v-if="isUpcoming && !reminder.recurring"
          class="shrink-0 rounded-full bg-primary-50 px-2 py-0.5 text-[10px] font-medium text-primary-600 dark:bg-primary-950 dark:text-primary-300"
        >
          Upcoming
        </span>
      </div>
      <p
        v-if="reminder.description"
        class="mt-0.5 truncate text-xs text-gray-400 dark:text-gray-500"
      >
        {{ reminder.description }}
      </p>
      <p class="mt-1 text-xs text-gray-400 dark:text-gray-500">
        {{ formattedRemindAt }}
      </p>
    </div>

    <div class="flex shrink-0 items-center gap-1">
      <MetaControls
        item-name="reminder"
        @edit-record="emit('edit', reminder.identifier)"
        @delete-record="emit('delete', reminder.identifier)"
        @duplicate-record="
          (targetWorkspaceId) => handleDuplicate(targetWorkspaceId)
        "
        @transfer-record="
          (targetWorkspaceId) => handleTransfer(targetWorkspaceId)
        "
      />
    </div>
  </RyderCard>
</template>
