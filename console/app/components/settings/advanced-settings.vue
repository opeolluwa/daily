<script setup lang="ts">
import type { BackupMode } from "@daily/core";
import { useDataBackupStore } from "~/stores/data-backup";

const emit = defineEmits<{ close: [] }>();

const store = useDataBackupStore();

const selectedMode = ref<BackupMode>("local_only");
const serverUrl = ref("");
const saving = ref(false);

onMounted(async () => {
  await store.fetchSettings();
  selectedMode.value = store.backupMode as BackupMode;
  serverUrl.value = store.serverUrl;
});

const modeOptions: {
  key: BackupMode;
  label: string;
  desc: string;
  icon: string;
}[] = [
  {
    key: "local_only",
    label: "Local Only",
    desc: "Data stays on this device. No account needed.",
    icon: "heroicons:computer-desktop",
  },
  {
    key: "cloud_only",
    label: "Cloud Only",
    desc: "Data stored on your server. Requires login.",
    icon: "heroicons:cloud",
  },
  {
    key: "local_first",
    label: "Local First",
    desc: "Save locally, sync to cloud when available.",
    icon: "heroicons:arrow-down-tray",
  },
  {
    key: "cloud_first",
    label: "Cloud First",
    desc: "Primary storage on cloud, local cache.",
    icon: "heroicons:cloud-arrow-up",
  },
];

const showServerUrl = computed(() => selectedMode.value !== "local_only");

async function save() {
  saving.value = true;
  try {
    await store.updateSettings({
      backupMode: selectedMode.value,
      serverUrl: showServerUrl.value ? serverUrl.value : null,
    });
    emit("close");
  } finally {
    saving.value = false;
  }
}
</script>

<template>
  <div class="flex flex-col gap-4 mt-4">
    <div class="bg-white dark:bg-gray-800 rounded-lg p-5">
      <h2 class="text-sm font-semibold text-gray-700 dark:text-gray-200 mb-1">
        Data Backup Strategy
      </h2>
      <p class="text-xs text-gray-400 mb-4">
        Choose how your data is stored and synced.
      </p>

      <div class="flex flex-col gap-2 mb-5">
        <button
          v-for="opt in modeOptions"
          :key="opt.key"
          class="flex items-start gap-3 p-3 rounded-lg border transition-colors text-left"
          :class="
            selectedMode === opt.key
              ? 'border-primary-400 bg-primary-50 dark:bg-primary-950 dark:border-primary-600'
              : 'border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700'
          "
          @click="selectedMode = opt.key"
        >
          <div
            class="mt-0.5 size-8 rounded-md flex items-center justify-center shrink-0"
            :class="
              selectedMode === opt.key
                ? 'bg-primary-100 dark:bg-primary-900'
                : 'bg-gray-100 dark:bg-gray-700'
            "
          >
            <UIcon
              :name="opt.icon"
              class="size-4"
              :class="
                selectedMode === opt.key
                  ? 'text-primary-600 dark:text-primary-400'
                  : 'text-gray-500 dark:text-gray-400'
              "
            />
          </div>
          <div class="flex-1 min-w-0">
            <p
              class="text-sm font-medium"
              :class="
                selectedMode === opt.key
                  ? 'text-primary-700 dark:text-primary-300'
                  : 'text-gray-700 dark:text-gray-200'
              "
            >
              {{ opt.label }}
            </p>
            <p class="text-xs text-gray-400 mt-0.5">{{ opt.desc }}</p>
          </div>
          <UIcon
            v-if="selectedMode === opt.key"
            name="heroicons:check-circle"
            class="size-4 text-primary-500 shrink-0 mt-1"
          />
        </button>
      </div>

      <div v-if="showServerUrl" class="mb-5">
        <label
          class="block text-xs font-medium text-gray-600 dark:text-gray-300 mb-1"
        >
          Server URL
        </label>
        <input
          v-model="serverUrl"
          type="url"
          placeholder="https://sync.example.com"
          class="w-full rounded-lg border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900 px-3 py-2 text-sm text-gray-800 dark:text-gray-200 placeholder-gray-400 dark:placeholder-gray-500 focus:border-primary-400 focus:outline-none focus:ring-1 focus:ring-primary-400"
        >
      </div>

      <div
        v-if="store.lastSync"
        class="mb-5 text-xs text-gray-400 dark:text-gray-500"
      >
        Last synced: {{ new Date(store.lastSync).toLocaleString() }}
      </div>

      <button
        class="px-4 py-2 text-sm font-medium rounded-lg bg-primary-500 text-white hover:bg-primary-600 transition-colors disabled:opacity-50"
        :disabled="saving"
        @click="save"
      >
        {{ saving ? "Saving..." : "Save" }}
      </button>
    </div>
  </div>
</template>
