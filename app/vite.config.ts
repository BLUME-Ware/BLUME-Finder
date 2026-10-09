import { svelte } from "@sveltejs/vite-plugin-svelte";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [svelte()],
  clearScreen: false,
  server: {
    host: "localhost",
    port: 5173,
    strictPort: true,
  },
  build: {
    // Every asset stays a file served by the app itself.
    assetsInlineLimit: 0,
  },
});
