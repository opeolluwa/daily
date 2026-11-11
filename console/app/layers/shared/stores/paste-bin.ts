import type { CreatePasteBin, PasteBins, UpdatePasteBin } from "@daily/core";
import { defineStore } from "pinia";
import { invoke } from "../utils/invoke";

export type PasteBin = PasteBins;

export type CreatePasteBinPayload = Partial<CreatePasteBin> & {
  content: string;
};

export type UpdatePasteBinPayload = Partial<UpdatePasteBin>;

export const usePasteBinStore = defineStore("paste_bin_store", {
  state: () => ({
    pasteBins: [] as PasteBin[],
    recent: [] as PasteBin[],
    loading: false,
    recentLoading: false,
  }),

  getters: {
    pinned: (state): PasteBin[] =>
      state.pasteBins.filter((paste) => paste.isPinned),
  },

  actions: {
    async fetchPasteBins() {
      this.loading = true;
      try {
        this.pasteBins = await invoke<PasteBin[]>("get_all_paste_bins", {
          meta: await getWorkspaceMeta(),
        });
      } catch (error) {
        console.error("[paste-bin] failed to fetch", error);
      } finally {
        this.loading = false;
      }
    },

    async fetchRecentPasteBins() {
      this.recentLoading = true;
      try {
        this.recent = await invoke<PasteBin[]>(
          "get_recently_added_paste_bins",
          {
            meta: await getWorkspaceMeta(),
          },
        );
      } finally {
        this.recentLoading = false;
      }
    },

    async createPasteBin(payload: CreatePasteBinPayload): Promise<PasteBin> {
      const created = await invoke<PasteBin>("create_paste_bin", {
        pasteBin: {
          ...payload,
          isPinned: payload.isPinned ?? false,
        },
        meta: await getWorkspaceMeta(),
      });

      this.pasteBins.unshift(created);
      return created;
    },

    async updatePasteBin(
      identifier: string,
      payload: UpdatePasteBinPayload,
    ): Promise<PasteBin> {
      const updated = await invoke<PasteBin>("update_paste_bin", {
        identifier,
        pasteBin: payload,
        meta: await getWorkspaceMeta(),
      });

      const idx = this.pasteBins.findIndex((p) => p.identifier === identifier);
      if (idx !== -1) this.pasteBins[idx] = updated;

      return updated;
    },

    async deletePasteBin(identifier: string) {
      await invoke("delete_paste_bin", {
        identifier,
        meta: await getWorkspaceMeta(),
      });

      this.pasteBins = this.pasteBins.filter(
        (paste) => paste.identifier !== identifier,
      );
    },

    async duplicatePasteBin(
      recordIdentifier: string,
      previousWorkspaceIdentifier: string,
      targetWorkspaceIdentifier: string,
    ) {
      await invoke("duplicate_paste_bin", {
        recordIdentifier,
        previousWorkspaceIdentifier,
        targetWorkspaceIdentifier,
        meta: await getWorkspaceMeta(),
      });

      await this.fetchPasteBins();
    },

    async transferPasteBin(
      recordIdentifier: string,
      previousWorkspaceIdentifier: string,
      targetWorkspaceIdentifier: string,
    ) {
      await invoke("transfer_paste_bin", {
        recordIdentifier,
        previousWorkspaceIdentifier,
        targetWorkspaceIdentifier,
        meta: await getWorkspaceMeta(),
      });

      this.pasteBins = this.pasteBins.filter(
        (paste) => paste.identifier !== recordIdentifier,
      );
    },
  },
});
