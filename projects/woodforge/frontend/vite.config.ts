import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
  plugins: [react()],
  server: {
    port: 3000,
    headers: {
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cross-Origin-Embedder-Policy': 'require-corp',
    },
  },
  resolve: {
    alias: {
      '@wasm': path.resolve(__dirname, '../crates/wasm-bridge/pkg'),
    },
  },
  optimizeDeps: {
    exclude: ['woodforge-wasm-bridge'],
  },
});
