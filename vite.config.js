import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import Icons from 'unplugin-icons/vite';

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    svelte(),
    Icons({
      compiler: 'svelte',
      autoInstall: true,
    }),
  ],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },

  build: {
    target: "ESNext",
  },
}));
