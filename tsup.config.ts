import { defineConfig } from 'tsup';

export default defineConfig({
  entry: ['webview-src/index.ts'],
  format: ['cjs', 'esm'],
  dts: true,
  clean: true,
  sourcemap: true,
  minify: false,
  external: ['@tauri-apps/api'],
});
