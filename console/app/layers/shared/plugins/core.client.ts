// Exposes the in-browser core data layer on `window.core` when running in a
// plain browser (web deployment). The Tauri build talks to the native SQLite
// backend through IPC instead, so this is skipped when Tauri internals are
// present.
import { IS_WEB } from "../env";
import { createCoreConsoleApi } from "../utils/core";

export default defineNuxtPlugin(async () => {
  if (!IS_WEB) return;

  try {
    const api = await createCoreConsoleApi();
    window.core = api;

    const workspaces = await api.workspaces.list_workspaces();
    const defaultWorkspace =
      workspaces.find((w) => w.isDefault) ?? workspaces[0] ?? null;

    console.info(
      "[core] ready — window.core exposes the in-browser data layer (PGlite)",
    );
    if (defaultWorkspace) {
      console.info(
        `[core] default workspace: "${defaultWorkspace.name}" (${defaultWorkspace.identifier}) — pass it as meta.workspaceIdentifier`,
      );
    }
  } catch (error) {
    console.error("[core] failed to initialize in-browser data layer", error);
  }
});
