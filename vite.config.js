import { defineConfig } from 'vite';

export default defineConfig({
  publicDir: 'site/public',
  build: {
    outDir: 'dist/site',
    emptyOutDir: true,
    target: 'es2022',
  },
});
