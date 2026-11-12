// @ts-check
import withNuxt from "./.nuxt/eslint.config.mjs";

export default withNuxt({
  name: "daily/ignores",
  ignores: [
    // Generated Rust build artifacts (only covered by src-tauri/.gitignore,
    // which eslint-config-flat-gitignore does not read).
    "src-tauri/target/**",
    // Generated native scaffolding (iOS/Android projects).
    "src-tauri/gen/**",
    // Separate pnpm workspace package with its own toolchain.
    "packages/**",
  ],
});
