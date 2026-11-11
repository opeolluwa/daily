import type {
  CreateWhiteboard,
  UpdateWhiteboard,
  WhiteboardFiles,
  Whiteboards,
} from "@daily/core";
import { defineStore } from "pinia";
import { invoke } from "../utils/invoke";

export type Whiteboard = Whiteboards;

export type CreateWhiteboardPayload = Partial<CreateWhiteboard>;

export type UpdateWhiteboardPayload = Partial<UpdateWhiteboard>;

export const useWhiteboardStore = defineStore("whiteboards_store", {
  state: () => ({
    whiteboards: [] as Whiteboard[],
    loading: false,
  }),

  actions: {
    async fetchWhiteboards() {
      this.loading = true;
      try {
        this.whiteboards = await invoke<Whiteboard[]>("get_all_whiteboards", {
          meta: await getWorkspaceMeta(),
        });
      } catch (error) {
        console.error("[whiteboards] failed to fetch", error);
      } finally {
        this.loading = false;
      }
    },

    async getWhiteboard(identifier: string): Promise<Whiteboard | null> {
      return invoke<Whiteboard | null>("get_whiteboard", {
        identifier,
        meta: await getWorkspaceMeta(),
      });
    },

    async createWhiteboard(
      payload: CreateWhiteboardPayload,
    ): Promise<Whiteboard> {
      const created = await invoke<Whiteboard>("create_whiteboard", {
        whiteboard: payload,
        meta: await getWorkspaceMeta(),
      });

      this.whiteboards.unshift(created);
      return created;
    },

    async updateWhiteboard(
      identifier: string,
      payload: UpdateWhiteboardPayload,
    ): Promise<Whiteboard> {
      const updated = await invoke<Whiteboard>("update_whiteboard", {
        identifier,
        whiteboard: payload,
        meta: await getWorkspaceMeta(),
      });

      const idx = this.whiteboards.findIndex(
        (w) => w.identifier === identifier,
      );
      if (idx !== -1) this.whiteboards[idx] = updated;

      return updated;
    },

    async deleteWhiteboard(identifier: string) {
      await invoke("delete_whiteboard", {
        identifier,
        meta: await getWorkspaceMeta(),
      });

      this.whiteboards = this.whiteboards.filter(
        (w) => w.identifier !== identifier,
      );
    },

    async transferWhiteboard(
      recordIdentifier: string,
      previousWorkspaceIdentifier: string,
      targetWorkspaceIdentifier: string,
    ) {
      await invoke("transfer_whiteboard", {
        recordIdentifier,
        previousWorkspaceIdentifier,
        targetWorkspaceIdentifier,
        meta: await getWorkspaceMeta(),
      });

      this.whiteboards = this.whiteboards.filter(
        (w) => w.identifier !== recordIdentifier,
      );
    },

    async saveWhiteboardFile(
      whiteboardIdentifier: string,
      fileId: string,
      mimeType: string,
      data: string,
    ) {
      await invoke("save_whiteboard_file", {
        whiteboardIdentifier,
        fileId,
        mimeType,
        data,
        meta: await getWorkspaceMeta(),
      });
    },

    async getWhiteboardFiles(
      whiteboardIdentifier: string,
    ): Promise<WhiteboardFiles[]> {
      return invoke<WhiteboardFiles[]>("get_whiteboard_files", {
        whiteboardIdentifier,
        meta: await getWorkspaceMeta(),
      });
    },

    async deleteWhiteboardFile(fileId: string) {
      await invoke("delete_whiteboard_file", {
        fileId,
        meta: await getWorkspaceMeta(),
      });
    },
  },
});
