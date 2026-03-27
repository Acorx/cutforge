import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { resolve } from 'path';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': resolve(__dirname, './src'),
    },
  },
  server: {
    // Tauri needs this to work properly
    strictPort: true,
  },
  build: {
    // Tauri expects a fixed build location
    outDir: '../dist',
    emptyOutDir: true,
  },
});