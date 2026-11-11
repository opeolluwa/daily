import { computed, ref, type Ref } from "vue";
import { createGlobalState } from "@vueuse/core";
import { THEME } from "./types";
import type { Theme } from "./types";

/**
 * In-memory theme mirror for the canvas renderer. The host app owns the
 * actual light/dark mode and keeps `theme` in sync from outside.
 */
export const useTheme = createGlobalState(() => {
  const theme: Ref<Theme> = ref(THEME.LIGHT);
  const isDark = computed(() => theme.value === THEME.DARK);

  return { theme, isDark };
});
