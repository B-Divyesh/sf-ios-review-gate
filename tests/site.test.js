import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile, stat } from 'node:fs/promises';

const immutableAssetRouteMatches = (route, path) => route === '/assets/*.{js,css}' && /^\/assets\/[^/]+\.(?:js|css)$/.test(path);

test('site metadata and landmarks are declared', async () => {
  const html = await readFile('index.html', 'utf8');
  assert.match(html, /<html lang="en">/);
  assert.match(html, /<title>[^<]+ — [^<]+<\/title>/);
  assert.match(html, /rel="canonical"/);
  assert.match(html, /name="description"/);
});

test('site assets stay within static budgets', async () => {
  const css = await readFile('site/src/style.css');
  const js = await readFile('site/src/main.js');
  const font = await stat('site/public/assets/b612-mono.woff2');
  const hero = await stat('site/public/assets/release-blueprint.webp');
  assert.ok(css.byteLength < 50_000, `CSS is ${css.byteLength}`);
  assert.ok(js.byteLength < 150_000, `JS is ${js.byteLength}`);
  assert.ok(font.size < 120_000, `Font is ${font.size}`);
  assert.ok(hero.size < 300_000, `Hero is ${hero.size}`);
});

test('factory contracts and static host configuration are valid JSON', async () => {
  const claims = JSON.parse(await readFile('.factory/claims.json', 'utf8'));
  const host = JSON.parse(await readFile('site/public/staticwebapp.config.json', 'utf8'));
  assert.ok(claims.length >= 1);
  assert.equal(new Set(claims.map(claim => claim.id)).size, claims.length);
  assert.deepEqual(host.routes.slice(0, 3), [
    { route: '/demo', rewrite: '/index.html' },
    { route: '/privacy', rewrite: '/index.html' },
    { route: '/terms', rewrite: '/index.html' },
  ]);
  assert.deepEqual(host.responseOverrides['404'], { rewrite: '/404.html', statusCode: 404 });
  await stat('404.html');
});

test('browser claim commands install their locked tooling before using it', async () => {
  const claims = JSON.parse(await readFile('.factory/claims.json', 'utf8'));
  const packageJson = JSON.parse(await readFile('package.json', 'utf8'));
  const browserClaims = claims.filter(claim => claim.test.startsWith('npm run verify:browser-claim'));

  assert.equal(packageJson.scripts['verify:browser-claim'], 'npm ci && npm run build:site && npx playwright test');
  assert.deepEqual(browserClaims.map(claim => claim.id), [
    'one-click-demo',
    'browser-demo-local',
    'offline-shell',
    'license-restore',
    'team-purchase',
    'team-policy-download',
    'version-metadata',
  ]);
  for (const claim of browserClaims) {
    assert.equal(
      claim.test,
      `npm run verify:browser-claim -- --grep @claim:${claim.id}`,
      `${claim.id} must run from a clean clone before local Vite or Playwright binaries are available`,
    );
  }
});

test('hashed Vite entry assets receive immutable caching while stable public files do not', async () => {
  const host = JSON.parse(await readFile('site/public/staticwebapp.config.json', 'utf8'));
  const vite = await readFile('vite.config.js', 'utf8');
  const serviceWorker = await readFile('site/public/sw.js', 'utf8');
  const immutable = host.routes.find(route => route.headers?.['Cache-Control'] === 'public, max-age=31536000, immutable');

  assert.ok(immutable, 'the static host needs an immutable hashed-asset route');
  assert.equal(immutable.route, '/assets/*.{js,css}');
  assert.match(vite, /entryFileNames: 'assets\/main-\[hash\]\.js'/);
  assert.match(vite, /chunkFileNames: 'assets\/main-\[hash\]\.js'/);
  assert.match(vite, /assetFileNames: 'assets\/main-\[hash\]\[extname\]'/);
  assert.match(vite, /versionServiceWorker/);
  assert.match(serviceWorker, /ios-review-gate-__BUILD_ID__/);
  for (const emittedHashedAsset of ['/assets/main-a1b2c3d4.js', '/assets/main-a1b2c3d4.css']) {
    assert.equal(immutableAssetRouteMatches(immutable.route, emittedHashedAsset), true, `${emittedHashedAsset} must match immutable caching`);
  }
  assert.equal(immutableAssetRouteMatches(immutable.route, '/assets/release-blueprint.webp'), false, 'stable public artwork must remain revalidatable');
  assert.equal(immutableAssetRouteMatches(immutable.route, '/sw.js'), false, 'the service worker must retain its no-cache policy');
});

test('every repaired visitor promise is registered to an exact claim test', async () => {
  const claims = JSON.parse(await readFile('.factory/claims.json', 'utf8'));
  const source = await readFile('site/src/main.js', 'utf8');
  const readme = await readFile('README.md', 'utf8');
  const required = [
    ['core-without-team-license', 'Checks and review packets cost $0.'],
    ['same-checker-demo', 'The website sample and command use the same bundled checker.'],
    ['actionable-mismatch-errors', 'Errors name the mismatch and the next fix.'],
    ['team-policy-download', 'Verified Team licenses enable the local policy download.'],
    ['team-queue-history', 'Team policies support queue histories beyond three submissions.'],
    ['team-purchase', 'One-time Team license'],
    ['archive-inspection', 'Read version, build, bundle ID, and privacy use from an .xcarchive or .ipa.'],
    ['offline-shell', 'The demo works offline after one visit.'],
    ['license-metadata', 'The CLI uses the MIT License.'],
    ['version-metadata', 'CLI v${VERSION} · build ${BUILD_ID}'],
  ];

  for (const [id, publicCopy] of required) {
    const claim = claims.find(item => item.id === id);
    assert.ok(claim, `missing registered claim: ${id}`);
    assert.ok(claim.test, `missing regression command for: ${id}`);
    assert.ok(source.includes(publicCopy), `public copy is not covered: ${publicCopy}`);
  }

  const cliPromises = [
    ['cli-exit-codes', 'Exit code `0` means the check passed.'],
    ['cli-json-schema', 'The JSON object contains `passed`, `summary`, `findings`, `queue`, and `packet_path`.'],
    ['sample-screenshot-dimensions', 'The bundled `iphone-69` sample is 1320×2868 pixels.'],
  ];
  for (const [id, publicCopy] of cliPromises) {
    const claim = claims.find(item => item.id === id);
    assert.ok(claim, `missing registered claim: ${id}`);
    assert.ok(claim.test, `missing regression command for: ${id}`);
    assert.ok(readme.includes(publicCopy), `README promise is not covered: ${publicCopy}`);
  }
});

test('@claim:license-metadata keeps MIT licensing consistent', async () => {
  const license = await readFile('LICENSE', 'utf8');
  const cargo = await readFile('Cargo.toml', 'utf8');
  const readme = await readFile('README.md', 'utf8');
  const site = await readFile('site/src/main.js', 'utf8');
  assert.match(license, /Permission is hereby granted, free of charge/);
  assert.match(license, /THE SOFTWARE IS PROVIDED "AS IS"/);
  assert.match(cargo, /^license = "MIT"$/m);
  assert.match(readme, /## License\n\nMIT\./);
  assert.ok(site.includes('The CLI uses the MIT License.'));
});
