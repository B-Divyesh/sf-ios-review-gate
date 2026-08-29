import { execFileSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { defineConfig } from 'vite';

const cargoManifest = readFileSync(resolve('Cargo.toml'), 'utf8');
const cliVersion = cargoManifest.match(/^version = "([^"]+)"$/m)?.[1];
if (!cliVersion) throw new Error('Cargo.toml package version is missing');

const buildId = (process.env.FACTORY_BUILD_ID
  || process.env.GITHUB_SHA
  || execFileSync('git', ['rev-parse', 'HEAD'], { encoding: 'utf8' }))
  .trim()
  .slice(0, 12);

const versionServiceWorker = () => ({
  name: 'version-service-worker-cache',
  closeBundle() {
    const serviceWorkerPath = resolve('dist/site/sw.js');
    const serviceWorker = readFileSync(serviceWorkerPath, 'utf8');
    if (!serviceWorker.includes('__BUILD_ID__')) {
      throw new Error('sw.js is missing its cache-version placeholder');
    }
    writeFileSync(serviceWorkerPath, serviceWorker.replaceAll('__BUILD_ID__', buildId));
  },
});

export default defineConfig({
  publicDir: 'site/public',
  plugins: [versionServiceWorker()],
  define: {
    __CLI_VERSION__: JSON.stringify(cliVersion),
    __BUILD_ID__: JSON.stringify(buildId),
  },
  build: {
    outDir: 'dist/site',
    emptyOutDir: true,
    target: 'es2022',
    rollupOptions: {
      input: {
        index: resolve('index.html'),
        404: resolve('404.html'),
      },
      output: {
        // Keep every Vite-generated, content-addressed asset beneath the same
        // cache route. Files copied from public/ deliberately keep their
        // stable names and remain revalidatable.
        entryFileNames: 'assets/main-[hash].js',
        chunkFileNames: 'assets/main-[hash].js',
        assetFileNames: 'assets/main-[hash][extname]',
      },
    },
  },
});
