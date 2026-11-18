import { defineStore } from "pinia";
import { invoke } from "../utils/invoke";

export interface NotificationSettings {
  identifier: string;
  tasksDue: boolean;
  reminders: boolean;
  syncComplete: boolean;
  appUpdates: boolean;
  userIdentifier: string | null;
  createdAt: string;
  updatedAt: string;
}

export const useNotificationSettingsStore = defineStore(
  "notification_settings_store",
  {
    state: () => ({
      settings: null as NotificationSettings | null,
      loading: false,
    }),

    getters: {
      tasksDue: (state) => state.settings?.tasksDue ?? true,
      reminders: (state) => state.settings?.reminders ?? true,
      syncComplete: (state) => state.settings?.syncComplete ?? false,
      appUpdates: (state) => state.settings?.appUpdates ?? true,
    },

    actions: {
      async fetchSettings() {
        this.loading = true;
        try {
          this.settings = await invoke<NotificationSettings>(
            "get_notification_settings",
          );
        } finally {
          this.loading = false;
        }
      },

      async updateSettings(
        payload: Partial<
          Pick<
            NotificationSettings,
            "tasksDue" | "reminders" | "syncComplete" | "appUpdates"
          >
        >,
      ) {
        if (!this.settings) await this.fetchSettings();
        if (!this.settings) return;

        this.settings = await invoke<NotificationSettings>(
          "update_notification_settings",
          {
            identifier: this.settings.identifier,
            settings: payload,
          },
        );
      },

      async toggle(
        key: "tasksDue" | "reminders" | "syncComplete" | "appUpdates",
      ) {
        const current = this.settings?.[key] ?? this[key];
        await this.updateSettings({ [key]: !current });
      },
    },
  },
);
