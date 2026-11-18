import { defineStore } from "pinia";
import type { BackupSettings, UpdateBackupSettings } from "@daily/core";
import { invoke } from "~/utils/invoke";

export const useDataBackupStore = defineStore("data_backup_store", {
  state: () => ({
    settings: null as BackupSettings | null,
    loading: false,
  }),

  getters: {
    backupMode: (state) => state.settings?.backupMode ?? "local_only",
    serverUrl: (state) => state.settings?.serverUrl ?? "",
    lastSync: (state) => state.settings?.lastSync ?? null,
    isLocalOnly(): boolean {
      return this.backupMode === "local_only";
    },
  },

  actions: {
    async fetchSettings() {
      this.loading = true;
      try {
        this.settings = await invoke<BackupSettings>("get_backup_settings");
      } finally {
        this.loading = false;
      }
    },

    async updateSettings(payload: UpdateBackupSettings) {
      if (!this.settings) await this.fetchSettings();
      if (!this.settings) return;

      this.settings = await invoke<BackupSettings>(
        "update_backup_settings",
        {
          identifier: this.settings.identifier,
          settings: payload,
        },
      );
    },
  },
});
