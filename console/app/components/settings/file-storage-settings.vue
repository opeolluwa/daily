<script setup lang="ts">
import {
  useBackupSettingsStore,
  type BackupProvider,
} from "~/stores/backup-settings";

const providerOptions: {
  key: BackupProvider;
  label: string;
  desc: string;
  icon: string;
}[] = [
  {
    key: "local",
    label: "Local only",
    desc: "Data stays on this device, no cloud backup.",
    icon: "heroicons:computer-desktop",
  },
  {
    key: "cloud",
    label: "Local + Cloud backup",
    desc: "Save locally, back up images to your cloud storage.",
    icon: "heroicons:cloud-arrow-up",
  },
];

const backupStore = useBackupSettingsStore();

onMounted(() => backupStore.init());
</script>

<template>
  <div class="flex flex-col gap-4 mt-4">
    <!-- ─── File Storage tab ─────────────────────────────────── -->

    <div class="bg-white dark:bg-gray-800 rounded-lg p-5 mt-3">
      <p class="text-xs text-gray-400 mb-4">
        Choose where your whiteboard images are stored.
      </p>

      <div class="flex flex-col gap-2 mb-5">
        <button
          v-for="opt in providerOptions"
          :key="opt.key"
          class="flex items-start gap-3 p-3 rounded-lg border transition-colors text-left"
          :class="
            backupStore.provider === opt.key
              ? 'border-primary-400 bg-primary-50 dark:bg-primary-950 dark:border-primary-600'
              : 'border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700'
          "
          @click="backupStore.provider = opt.key"
        >
          <div
            class="mt-0.5 size-8 rounded-md flex items-center justify-center shrink-0"
            :class="
              backupStore.provider === opt.key
                ? 'bg-primary-100 dark:bg-primary-900'
                : 'bg-gray-100 dark:bg-gray-700'
            "
          >
            <UIcon
              :name="opt.icon"
              class="size-4"
              :class="
                backupStore.provider === opt.key
                  ? 'text-primary-600 dark:text-primary-400'
                  : 'text-gray-500 dark:text-gray-400'
              "
            />
          </div>
          <div class="flex-1 min-w-0">
            <p
              class="text-sm font-medium"
              :class="
                backupStore.provider === opt.key
                  ? 'text-primary-700 dark:text-primary-300'
                  : 'text-gray-700 dark:text-gray-200'
              "
            >
              {{ opt.label }}
            </p>
            <p class="text-xs text-gray-400 mt-0.5">{{ opt.desc }}</p>
          </div>
          <UIcon
            v-if="backupStore.provider === opt.key"
            name="heroicons:check-circle"
            class="size-4 text-primary-500 shrink-0 mt-1"
          />
        </button>
      </div>

      <div
        v-if="backupStore.error"
        class="text-xs p-3 mb-4 rounded-lg bg-red-50 dark:bg-red-950 text-red-700 dark:text-red-300"
      >
        {{ backupStore.error }}
      </div>

      <button
        class="px-4 py-2 text-sm font-medium rounded-lg bg-primary-500 text-white hover:bg-primary-600 transition-colors disabled:opacity-50"
        :disabled="backupStore.saving || !backupStore.available"
        @click="backupStore.save()"
      >
        {{ backupStore.saving ? "Saving..." : "Save" }}
      </button>
    </div>
  </div>
</template>
