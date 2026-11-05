import {
  settingsTabForPath,
  type SettingsTab,
} from "@opeolluwa/ryder/utils/settingsTabs.js";

const items: SettingsTab[] = [
  {
    key: "profile",
    label: "Profile",
    icon: "i-lucide-user",
    desc: "Personal info & preferences",
    to: "/settings/profile",
  },
  {
    key: "appearance",
    label: "Appearance",
    icon: "heroicons:paint-brush",
    desc: "Theme & font size",
    to: "/settings/appearance",
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
    key: "backup",
    label: "Backup & Sync",
    icon: "heroicons:cloud-arrow-up",
    desc: "Backup configuration",
    to: "/settings/backup",
  },
  {
    key: "about",
    label: "About",
    icon: "heroicons:information-circle",
    desc: "App version & info",
    to: "/settings/about",
  },
];

export { settingsTabForPath, type SettingsTab };
export default items;
