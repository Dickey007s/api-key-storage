import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [svelte()],

  // Tauri 需要的配置
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: false,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
}));
