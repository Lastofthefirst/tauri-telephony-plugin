import { defineConfig } from 'tsup';

export default defineConfig({
  entry: ['guest-js/index.ts'],
  format: ['cjs', 'esm'],
  dts: true,
  clean: true,
  sourcemap: true,
  minify: false,
  external: ['@tauri-apps/api'],
});
