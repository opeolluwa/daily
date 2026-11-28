import { invoke as tauriInvoke } from "@tauri-apps/api/core";

type InvokeArgs = Record<string, unknown>;

const MUTATING =
  /^(create_|update_|delete_|mark_|change_|duplicate_|transfer_|restore_|purge_)/;

export async function invoke<T>(cmd: string, args?: InvokeArgs): Promise<T> {
  const result = await tauriInvoke<T>(cmd, args);

  if (typeof window !== "undefined" && MUTATING.test(cmd)) {
    void import("./sync/engine")
      .then(({ scheduleSync }) => scheduleSync())
      .catch(() => undefined);
  }

  return result;
}
