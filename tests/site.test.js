import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile, stat } from 'node:fs/promises';

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
  assert.equal(host.navigationFallback.rewrite, '/index.html');
});
