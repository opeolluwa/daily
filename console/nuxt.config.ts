import { resolve } from "path";

const isMobile = process.env.NUXT_APP_TARGET === "mobile";
// const isMobile = true

export default defineNuxtConfig({
  srcDir: "app",
  extends: [
    "./app/layers/shared",
    isMobile ? "./app/layers/mobile" : "./app/layers/desktop",
  ],

  alias: {
    "@desktop": resolve(__dirname, "app/layers/desktop"),
    "@mobile": resolve(__dirname, "app/layers/mobile"),
    "@shared": resolve(__dirname, "app/layers/shared"),
    "@features": resolve(__dirname, "app/layers/desktop/features"),
  },

  icon: {
    // Fully offline: never fall back to the remote Iconify API.
    fallbackToApi: false,
    clientBundle: {
      scan: {
        // The default scan omits `.ts`, but many icons live in data/*.ts.
        globInclude: ["**/*.{vue,jsx,tsx,ts,md,mdc,mdx,yml,yaml}"],
        globExclude: [
          "node_modules",
          "dist",
          ".output",
          ".nuxt",
          "build",
          "coverage",
          "test",
          "tests",
          ".*",
          "src-tauri",
        ],
      },
    },
  },

  compatibilityDate: "2025-07-15",
  devtools: { enabled: true },
  ssr: false,

  googleFonts: {
    families: {
      Lato: [400, 700],
    },
    display: "swap",
    download: true,
  },

  runtimeConfig: {
    public: {
      apiBaseUrl: "http://localhost:8000",
    },
  },

  modules: [
    "@nuxt/eslint",
    "@nuxt/image",
    "@nuxt/ui",
    "@nuxt/eslint",
    "@nuxtjs/device",
    "@nuxtjs/google-fonts",
    "@nuxtjs/i18n",
    "@pinia/nuxt",
    "pinia-plugin-persistedstate/nuxt",
    "@vueuse/nuxt",
    "nuxt-seo-utils",
  ],

  css: [
    "~/assets/css/main.css",
    "highlight.js/styles/atom-one-dark.css",
    "@domternal/theme",
    isMobile ? "~/assets/css/mobile.css" : "~/assets/css/desktop.css",
  ],

  i18n: {
    defaultLocale: "en",
    locales: ["en"],
    strategy: "no_prefix",
  },

  site: {
    url: "https://localhost",
    name: "Daily Console",
  },

  colorMode: {
    preference: "system",
    fallback: "light",
    globalName: "__NUXT_COLOR_MODE__",
    componentName: "ColorScheme",
    classPrefix: "",
    classSuffix: "",
    storage: "localStorage",
    storageKey: "nuxt-color-mode",
  },

  devServer: { host: "0" },

  vite: {
    clearScreen: false,
    resolve: {
      alias: {
        // @excalidraw/math only exposes "./*" subpaths for types, not runtime JS.
        // @excalidraw/element imports "@excalidraw/math/ellipse" at runtime,
        // which the bundler can't resolve. The main entry re-exports everything,
        // so we alias the subpath to the main package.
        "@excalidraw/math/ellipse": "@excalidraw/math",
      },
    },
    envPrefix: ["VITE_", "TAURI_"],
    server: {
      strictPort: true,
      proxy: {
        "/sync": "http://localhost:80007",
        "/segments": "http://localhost:8000",
        "/blobs": "http://localhost:8000",
        "/realtime": { target: "ws://localhost:8000", ws: true },
      },
    },
    worker: {
      format: "es",
    },
    optimizeDeps: {
      include: [
        "@nuxt/ui > prosemirror-state",
        "@nuxt/ui > prosemirror-transform",
        "@nuxt/ui > prosemirror-model",
        "@nuxt/ui > prosemirror-view",
        "@nuxt/ui > prosemirror-gapcursor",
      ],
      // PGlite ships its own wasm + FS assets; pre-bundling it breaks
      // `new PGlite("idb://lunar")` in dev ("Invalid FS bundle size").
      exclude: ["@electric-sql/pglite", "@sqlite.org/sqlite-wasm"],
    },
  },

  ignore: ["**/src-tauri/**"],
});
