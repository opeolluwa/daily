import { watch } from "vue";
import { startSync, scheduleSync, stopSync } from "../utils/sync/engine";
import { useAuthStore } from "../stores/auth";
import { useWorkspacesStore } from "../stores/workspaces";

export default defineNuxtPlugin((nuxtApp) => {
  const config = useRuntimeConfig();
  const auth = useAuthStore();
  const workspaces = useWorkspacesStore();

  nuxtApp.hook("app:mounted", () => {
    startSync(String(config.public.apiBaseUrl));
  });

  watch(
    () => [auth.accessToken, workspaces.activeWorkspaceId] as const,
    ([token, workspaceIdentifier]) => {
      if (token && workspaceIdentifier) scheduleSync(0);
    },
  );

  if (import.meta.hot) {
    import.meta.hot.dispose(() => stopSync());
  }
});
