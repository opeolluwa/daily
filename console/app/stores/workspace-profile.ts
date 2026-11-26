import type { UpdateUserProfile, UserProfile } from "@daily/core";
import { defineStore } from "pinia";
import { invoke } from "../utils/invoke";

export const useUserPreferenceStore = defineStore("user_profile_store", {
  state: () => ({
    preference: null as UserProfile | null,
    loading: false,
  }),

  getters: {
    fullName: (state) =>
      state.preference
        ? `${state.preference.firstName} ${state.preference.lastName}`.trim()
        : "",
  },

  actions: {
    async fetchPreference() {
      this.loading = true;
      try {
        this.preference = await invoke<UserProfile>("get_user_profile");
      } catch (error) {
        console.error("[preferences] failed to fetch", error);
      } finally {
        this.loading = false;
      }
    },

    async updatePreference(
      payload: Partial<UpdateUserProfile>,
    ): Promise<UserProfile> {
      if (!this.preference) {
        await this.fetchPreference();
      }
      if (!this.preference) {
        this.preference = await invoke<UserProfile>("get_user_profile");
      }
      const updated = await invoke<UserProfile>("update_user_profile", {
        identifier: this.preference.identifier,
        profile: payload,
      });
      this.preference = updated;
      return updated;
    },
  },
  persist: true,
});
