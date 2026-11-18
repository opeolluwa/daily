import type { BackupSettings } from "@daily/core";
import { useAuthStore } from "~/stores/auth";
import { invoke } from "~/utils/invoke";

export default defineNuxtRouteMiddleware(async (to) => {
  if (to.path.startsWith("/auth")) return;

  const authStore = useAuthStore();
  if (authStore.isAuthenticated || authStore.isGuest) return;

  try {
    const settings = await invoke<BackupSettings>("get_backup_settings");
    if (settings.backupMode === "local_only") {
      authStore.enterGuestMode();
      return;
    }
  } catch {
    // DB not ready or first run — fall through to login
  }

  return navigateTo("/auth/login");
});
