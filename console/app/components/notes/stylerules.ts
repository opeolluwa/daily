export function getDmVars(isDark: boolean): Record<string, string | number> {
  return isDark
    ? {
        "--dm-primary": "var(--color-primary-400)",
        "--dm-primary-hover": "var(--color-primary-300)",
        "--dm-primary-surface":
          "color-mix(in srgb, var(--color-primary-400) 15%, transparent)",
        "--dm-accent": "var(--color-primary-400)",
        "--dm-accent-surface":
          "color-mix(in srgb, var(--color-primary-400) 15%, transparent)",
        "--dm-bg": "transparent",
        "--dm-surface": "var(--app-dark-700)",
        "--dm-border-color": "var(--color-surface-700)",
      }
    : {
        "--dm-primary": "var(--color-primary-500)",
        "--dm-primary-hover": "var(--color-primary-600)",
        "--dm-primary-surface":
          "color-mix(in srgb, var(--color-primary-500) 10%, transparent)",
        "--dm-accent": "var(--color-primary-500)",
        "--dm-accent-surface":
          "color-mix(in srgb, var(--color-primary-500) 10%, transparent)",
        "--dm-bg": "transparent",
        "--dm-block-handle-gutter": 0,
      };
}
