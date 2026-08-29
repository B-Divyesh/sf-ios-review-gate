import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
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

export default defineConfig({
  publicDir: 'site/public',
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
    },
  },
});
