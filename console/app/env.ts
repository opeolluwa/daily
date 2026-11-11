import { useMediaQuery } from "@vueuse/core";

export const IS_WEB =
  typeof window !== "undefined" && !window.__TAURI_INTERNALS__;
export const IS_TAURI = !IS_WEB;
export const CORE_API = IS_WEB ? window?.core : undefined;
export const CORE_EXISTS = IS_WEB && window.core !== undefined;

export const isPreferredDark = useMediaQuery("(prefers-color-scheme: dark)");
