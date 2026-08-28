import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

test('@claim:one-click-demo opens a complete sample in one click', async ({ page }) => {
  await page.goto('/');
  await page.getByRole('link', { name: 'Try it with sample data' }).click();
  await expect(page).toHaveURL(/\/demo$/);
  await expect(page.getByRole('heading', { level: 1, name: 'Inspect a complete sample release' })).toBeVisible();
  await expect(page.getByText('Harbor Log 2.4.0', { exact: true })).toBeVisible();
  await expect(page.getByText('PASS', { exact: true }).first()).toBeVisible();
  await expect(page.getByText('Demo — sample data, nothing is saved')).toBeVisible();
});

test('@claim:browser-demo-local sends no cross-origin requests', async ({ page }) => {
  const crossOrigin = [];
  page.on('request', request => { if (new URL(request.url()).origin !== 'http://127.0.0.1:4173') crossOrigin.push(request.url()); });
  await page.goto('/demo');
  await page.getByRole('button', { name: 'Reset demo' }).click();
  expect(crossOrigin).toEqual([]);
});

test('routes expose one h1, titles, keyboard focus, and no serious axe findings', async ({ page }) => {
  for (const [path, title] of [['/', /iOS Review Gate/], ['/demo', /^Demo/], ['/privacy', /^Privacy/], ['/terms', /^Terms/], ['/missing', /^Page not found/]]) {
    await page.goto(path);
    await expect(page).toHaveTitle(title);
    await expect(page.locator('main')).toHaveCount(1);
    await expect(page.locator('h1')).toHaveCount(1);
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations.filter(v => ['serious', 'critical'].includes(v.impact))).toEqual([]);
  }
});

test('dark treatment has no serious axe findings', async ({ page }) => {
  await page.emulateMedia({ colorScheme: 'dark', reducedMotion: 'reduce' });
  await page.goto('/');
  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations.filter(v => ['serious', 'critical'].includes(v.impact))).toEqual([]);
  const duration = await page.locator('.inspection-line').evaluate(element => getComputedStyle(element).animationDuration);
  expect(Number.parseFloat(duration)).toBeLessThanOrEqual(0.001);
});

test('keyboard activation opens the sample and moves focus to its heading', async ({ page }) => {
  await page.goto('/');
  const action = page.getByRole('link', { name: 'Try it with sample data' });
  await action.focus();
  await page.keyboard.press('Enter');
  await expect(page).toHaveURL(/\/demo$/);
  await expect(page.getByRole('heading', { level: 1 })).toBeFocused();
});

test('mobile layout keeps the first action visible and avoids horizontal scroll', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/');
  await expect(page.getByRole('link', { name: 'Try it with sample data' })).toBeVisible();
  const widths = await page.evaluate(() => ({ scroll: document.documentElement.scrollWidth, client: document.documentElement.clientWidth }));
  expect(widths.scroll).toBe(widths.client);
});

test('public artwork loads and the page reports no browser errors', async ({ page }) => {
  const errors = [];
  page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
  page.on('pageerror', error => errors.push(error.message));
  await page.goto('/');
  const artwork = page.getByRole('img', { name: /exploded drafting view/i });
  await expect(artwork).toBeVisible();
  expect(await artwork.evaluate(image => image.complete && image.naturalWidth > 0)).toBe(true);
  expect(errors).toEqual([]);
});

test('internal navigation has no dead links', async ({ page, request }) => {
  await page.goto('/');
  const hrefs = await page.locator('a[href]').evaluateAll(links => [...new Set(links.map(link => link.href))]);
  for (const href of hrefs) {
    const url = new URL(href);
    if (url.origin === 'http://127.0.0.1:4173') {
      const response = await request.get(url.origin + url.pathname);
      expect(response.ok(), href).toBe(true);
    }
  }
});

test('@claim:license-restore verifies and stores a Team license', async ({ page }) => {
  await page.route('https://api.sociobot.in/api/v1/products/ios-review-gate/verify?license=test-token', route => route.fulfill({ json: { valid: true, reason: 'ok', expires_at: null } }));
  await page.goto('/');
  await page.getByLabel('Have a license? Paste it here.').fill('test-token');
  await page.getByRole('button', { name: 'Verify Team license' }).click();
  await expect(page.getByText('Team license active')).toBeVisible();
  expect(await page.evaluate(() => localStorage.getItem('sb_license:ios-review-gate'))).toBe('test-token');
});

test('checkout return stores, strips, and can remove a Team license', async ({ page }) => {
  await page.route('https://api.sociobot.in/api/v1/products/ios-review-gate/verify?license=return-token', route => route.fulfill({ json: { valid: true, reason: 'ok', expires_at: null } }));
  await page.goto('/?license=return-token');
  await expect(page).toHaveURL('http://127.0.0.1:4173/');
  await expect(page.getByText('Team license active')).toBeVisible();
  expect(await page.evaluate(() => localStorage.getItem('sb_license:ios-review-gate'))).toBe('return-token');
  await page.getByRole('button', { name: 'Remove license from this browser' }).click();
  await expect(page.getByLabel('Have a license? Paste it here.')).toBeVisible();
  expect(await page.evaluate(() => localStorage.getItem('sb_license:ios-review-gate'))).toBeNull();
});

test('@claim:team-policy-download writes the licensed policy settings', async ({ page }) => {
  await page.addInitScript(() => {
    localStorage.setItem('sb_license:ios-review-gate', 'test-token');
    localStorage.setItem('sb_license:ios-review-gate:verdict', JSON.stringify({ token: 'test-token', valid: true, checkedAt: Date.now() }));
  });
  await page.goto('/');
  await page.getByLabel('Policy name').fill('Mobile release');
  await page.getByLabel('Active submission limit').fill('7');
  const downloadPromise = page.waitForEvent('download');
  await page.getByRole('button', { name: 'Download Team policy' }).click();
  const download = await downloadPromise;
  expect(download.suggestedFilename()).toBe('team-policy.yaml');
  const content = await (await import('node:fs/promises')).readFile(await download.path(), 'utf8');
  expect(content).toContain('name: "Mobile release"');
  expect(content).toContain('max_active_submissions: 7');
});
