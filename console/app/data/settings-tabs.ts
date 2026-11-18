import {
  settingsTabForPath,
  type SettingsTab,
} from "@opeolluwa/ryder/utils/settingsTabs.js";

export interface ExtendedSettingsTab extends SettingsTab {
  modal?: boolean;
}

const items: ExtendedSettingsTab[] = [
  {
    key: "profile",
    label: "Profile",
    icon: "i-lucide-user",
    desc: "Personal info & preferences",
    to: "/settings/profile",
  },
  {
    key: "notifications",
    label: "Notifications",
    icon: "heroicons:bell",
    desc: "Notification preferences",
    to: "/settings/notifications",
  },

  {
    key: "locale",
    label: "Locale",
    icon: "heroicons:language",
    desc: "Language settings",
    to: "/settings/locale",
  },
  {
    key: "workspaces",
    label: "Workspaces",
    icon: "heroicons:briefcase",
    desc: "Manage workspaces",
    to: "/settings/workspaces",
  },
  {
    key: "file-storage",
    label: "Backup & Sync",
    icon: "heroicons:folder",
    desc: "File storage configuration",
    to: "/settings/file-storage",
  },
  {
    key: "advanced",
    label: "Advanced",
    icon: "heroicons:cog-8-tooth",
    desc: "Data backup & sync strategy",
    to: "/settings/advanced",
    modal: true,
  },
  {
    key: "about",
    label: "About",
    icon: "heroicons:information-circle",
    desc: "App version & info",
    to: "/settings/about",
  },
];

export { settingsTabForPath, type SettingsTab, type ExtendedSettingsTab };
export default items;
