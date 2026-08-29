import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';

test('@claim:one-click-demo opens a complete sample in one click', async ({ page }) => {
  await page.goto('/');
  await page.getByRole('link', { name: 'Try it with sample data' }).click();
  await expect(page).toHaveURL(/\?demo=1$/);
  await expect(page.getByRole('heading', { level: 1, name: 'Inspect a complete sample release' })).toBeVisible();
  await expect(page.getByText('Harbor Log 2.4.0', { exact: true })).toBeVisible();
  await expect(page.getByText('PASS', { exact: true }).first()).toBeVisible();
  await expect(page.getByText('Demo — sample data, nothing is saved')).toBeVisible();
  await page.getByRole('button', { name: 'Reset demo' }).click();
  await expect(page.getByText('Harbor Log 2.4.0', { exact: true })).toBeVisible();
});

test('@claim:browser-demo-local keeps real storage untouched and sends no cross-origin requests', async ({ page }) => {
  let releaseVerification;
  let markVerificationStarted;
  const verificationStarted = new Promise(resolve => { markVerificationStarted = resolve; });
  const verificationRelease = new Promise(resolve => { releaseVerification = resolve; });
  await page.route('https://api.sociobot.in/api/v1/products/ios-review-gate/verify?license=real-license', async route => {
    markVerificationStarted();
    await verificationRelease;
    await route.fulfill({ json: { valid: true, reason: 'ok', expires_at: null } }).catch(() => {});
  });
  const crossOriginAfterDemo = [];
  let demoStarted = false;
  page.on('request', request => {
    if (demoStarted && new URL(request.url()).origin !== 'http://127.0.0.1:4173') crossOriginAfterDemo.push(request.url());
  });
  await page.addInitScript(() => {
    localStorage.setItem('real:release', 'private-project');
    localStorage.setItem('sb_license:ios-review-gate', 'real-license');
    sessionStorage.setItem('real:draft', 'private-draft');
  });
  await page.goto('/');
  await verificationStarted;
  const before = await page.evaluate(() => ({ local: { ...localStorage }, session: { ...sessionStorage } }));
  demoStarted = true;
  await page.getByRole('link', { name: 'Try it with sample data' }).click();
  await expect(page).toHaveURL(/\?demo=1$/);
  releaseVerification();
  await page.getByRole('button', { name: 'Reset demo' }).click();
  await page.waitForTimeout(100);
  const storage = await page.evaluate(async () => ({
    local: { ...localStorage },
    session: { ...sessionStorage },
    cookies: document.cookie,
    databases: (await indexedDB.databases()).map(database => database.name),
    caches: (await caches.keys()).sort(),
  }));
  expect(crossOriginAfterDemo).toEqual([]);
  expect(storage.local).toEqual(before.local);
  expect(storage.session).toEqual(before.session);
  expect(storage.cookies).toBe('');
  expect(storage.databases).toEqual([]);
  expect(storage.caches).toHaveLength(1);
  expect(storage.caches[0]).toMatch(/^ios-review-gate-[a-f0-9]{12}$/);
});

test('routes expose one h1, titles, keyboard focus, and no serious axe findings', async ({ page }) => {
  for (const [path, title] of [['/', /iOS Review Gate/], ['/?demo=1', /^Demo/], ['/demo', /^Demo/], ['/privacy', /^Privacy/], ['/terms', /^Terms/], ['/missing', /^Page not found/]]) {
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

test('mobile code samples are keyboard focusable and have no serious axe findings', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  for (const [path, label] of [['/', 'Install and run command'], ['/?demo=1', 'Sample Markdown review packet']]) {
    await page.goto(path);
    const codeSample = page.getByLabel(label);
    await codeSample.focus();
    await expect(codeSample).toBeFocused();
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations.filter(v => ['serious', 'critical'].includes(v.impact))).toEqual([]);
  }
});

test('dark mobile demo banner has readable controls and no serious axe findings', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.emulateMedia({ colorScheme: 'dark' });
  await page.goto('/?demo=1');
  const banner = page.getByLabel('Demo mode');
  await expect(banner).toContainText('Demo — sample data, nothing is saved');
  await expect(page.getByRole('button', { name: 'Reset demo' })).toBeVisible();
  await expect(page.getByRole('link', { name: 'Install the CLI' })).toHaveAttribute('href', '/#install');
  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations.filter(v => ['serious', 'critical'].includes(v.impact))).toEqual([]);
});

test('@claim:offline-shell caches only same-origin static files and reloads the demo offline', async ({ page, context }) => {
  await page.goto('/?demo=1', { waitUntil: 'networkidle' });
  await page.evaluate(() => navigator.serviceWorker.ready);
  const activeWorker = await page.evaluate(async () => {
    const registration = await navigator.serviceWorker.ready;
    await registration.update();
    return registration.active?.scriptURL || null;
  });
  expect(activeWorker).toMatch(/\/sw\.js$/);
  await expect.poll(() => page.evaluate(() => Boolean(navigator.serviceWorker.controller))).toBe(true);
  await expect.poll(() => page.evaluate(async () => {
    const cacheName = (await caches.keys()).find(name => /^ios-review-gate-[a-f0-9]{12}$/.test(name));
    if (!cacheName) return false;
    const cache = await caches.open(cacheName);
    const paths = (await cache.keys()).map(request => new URL(request.url).pathname);
    return paths.some(path => /^\/assets\/(?:index|main)-.+\.js$/.test(path))
      && paths.some(path => /^\/assets\/(?:index|main)-.+\.css$/.test(path));
  })).toBe(true);
  const cachedUrls = await page.evaluate(async () => {
    const cacheName = (await caches.keys()).find(name => /^ios-review-gate-[a-f0-9]{12}$/.test(name));
    const cache = await caches.open(cacheName);
    return (await cache.keys()).map(request => request.url).sort();
  });
  expect(cachedUrls.length).toBeGreaterThan(7);
  for (const cachedUrl of cachedUrls) {
    const url = new URL(cachedUrl);
    expect(url.origin).toBe('http://127.0.0.1:4173');
    expect(url.pathname === '/' || ['/demo', '/privacy', '/terms'].includes(url.pathname) || url.pathname.startsWith('/assets/')).toBe(true);
  }
  await page.waitForTimeout(250);
  await context.setOffline(true);
  await page.reload({ waitUntil: 'domcontentloaded' });
  await expect(page.getByRole('heading', { level: 1, name: 'Inspect a complete sample release' })).toBeVisible();
  await context.setOffline(false);
});

test('keyboard activation opens the sample and moves focus to its heading', async ({ page }) => {
  await page.goto('/');
  const action = page.getByRole('link', { name: 'Try it with sample data' });
  await action.focus();
  await page.keyboard.press('Enter');
  await expect(page).toHaveURL(/\?demo=1$/);
  await expect(page.getByRole('heading', { level: 1 })).toBeFocused();
});

test('mobile layout keeps the first action visible and avoids horizontal scroll', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/');
  await expect(page.getByRole('link', { name: 'Try it with sample data' })).toBeVisible();
  const widths = await page.evaluate(() => ({ scroll: document.documentElement.scrollWidth, client: document.documentElement.clientWidth }));
  expect(widths.scroll).toBe(widths.client);
});

test('desktop first screen contains the action and all three facts', async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 900 });
  await page.goto('/');
  for (const text of [
    'Try it with sample data',
    'Release files stay on your machine.',
    'The demo works offline after one visit.',
    'Checks and review packets cost $0.',
  ]) {
    const box = await page.getByText(text, { exact: true }).boundingBox();
    expect(box, text).not.toBeNull();
    expect(box.y + box.height, text).toBeLessThanOrEqual(900);
  }
});

test('mobile wordmark includes its visible RG label in its accessible name', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/');
  const wordmark = page.locator('.wordmark').first();
  await expect(wordmark.locator('.mark')).toBeVisible();
  await expect(wordmark.locator('span:last-child')).toBeHidden();
  await expect(wordmark).toHaveAccessibleName('RG — iOS Review Gate home');
});

test('mobile default text gives every interactive control a 44px target on every route', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  for (const path of ['/', '/demo', '/privacy', '/terms', '/404.html']) {
    await page.goto(path);
    const targets = await page.locator('a, button, input').evaluateAll(elements => elements.map(element => {
      const box = element.getBoundingClientRect();
      return { label: element.getAttribute('aria-label') || element.textContent.trim() || element.id, width: box.width, height: box.height };
    }));
    for (const target of targets) {
      expect(target.width, `${path}: ${target.label}`).toBeGreaterThanOrEqual(44);
      expect(target.height, `${path}: ${target.label}`).toBeGreaterThanOrEqual(44);
    }
  }
});

test('mobile 200% text reflows navigation without overflow and all links meet the touch baseline', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  for (const path of ['/', '/demo', '/privacy', '/terms']) {
    await page.goto(path);
    await page.addStyleTag({ content: 'html { font-size: 200% !important; }' });
    const widths = await page.evaluate(() => ({ scroll: document.documentElement.scrollWidth, client: document.documentElement.clientWidth }));
    expect(widths.scroll, path).toBe(widths.client);
    const targets = await page.locator('a, button, input').evaluateAll(elements => elements.map(element => {
      const box = element.getBoundingClientRect();
      return { label: element.getAttribute('aria-label') || element.textContent.trim() || element.id, width: box.width, height: box.height };
    }));
    for (const target of targets) {
      expect(target.width, `${path}: ${target.label}`).toBeGreaterThanOrEqual(44);
      expect(target.height, `${path}: ${target.label}`).toBeGreaterThanOrEqual(44);
    }
  }
});

test('mobile 200% text keeps every demo result edge visible', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/demo');
  await page.addStyleTag({ content: 'html { font-size: 200% !important; }' });

  const layout = await page.evaluate(() => {
    const viewport = document.documentElement.clientWidth;
    const selectors = [
      '.demo-page h1',
      '.demo-page .lede',
      '.sample-ledger',
      '.terminal-sheet',
      '#packet-title',
      '[aria-label="Sample Markdown review packet"]',
    ];
    return {
      viewport,
      mainOverflow: getComputedStyle(document.querySelector('main')).overflowX,
      scrollWidth: document.documentElement.scrollWidth,
      boxes: selectors.map(selector => {
        const box = document.querySelector(selector).getBoundingClientRect();
        return { selector, left: box.left, right: box.right, width: box.width };
      }),
    };
  });

  expect(layout.mainOverflow).not.toBe('hidden');
  expect(layout.scrollWidth).toBe(layout.viewport);
  for (const box of layout.boxes) {
    expect(box.left, `${box.selector} starts inside the viewport`).toBeGreaterThanOrEqual(0);
    expect(box.right, `${box.selector} ends inside the viewport`).toBeLessThanOrEqual(layout.viewport);
    expect(box.width, `${box.selector} has visible width`).toBeGreaterThan(0);
  }
});

test('the deployable not-found page has complete metadata, shared navigation, and recovery', async ({ page }) => {
  await page.goto('/404.html');
  await expect(page.getByRole('heading', { level: 1, name: 'This page does not exist' })).toBeVisible();
  await expect(page.getByRole('link', { name: 'Return home' })).toBeVisible();
  await expect(page.getByRole('link', { name: 'Install' })).toHaveAttribute('href', '/#install');
  await expect(page.getByRole('link', { name: /Built by Param Factory/ })).toBeVisible();
  await expect(page.locator('link[rel="canonical"]')).toHaveAttribute('href', 'https://ios-review-gate.sociobot.in/404');
  await expect(page.locator('meta[property="og:image"]')).toHaveAttribute('content', /og-card\.webp$/);
  await expect(page.locator('link[rel="apple-touch-icon"]')).toHaveCount(1);
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

test('@claim:team-purchase starts the $39 one-time purchase at hosted Sociobot checkout', async ({ page }) => {
  let checkoutRequest = '';
  await page.route('https://api.sociobot.in/api/v1/products/ios-review-gate/checkout', async route => {
    checkoutRequest = route.request().url();
    await route.fulfill({ status: 200, contentType: 'text/html', body: '<title>Hosted checkout</title><main>Hosted checkout</main>' });
  });
  await page.goto('/');
  await expect(page.getByText('$39', { exact: true })).toBeVisible();
  await expect(page.getByText('One-time Team license', { exact: true })).toBeVisible();
  await page.getByRole('link', { name: /Buy Team license/ }).click();
  expect(checkoutRequest).toBe('https://api.sociobot.in/api/v1/products/ios-review-gate/checkout');
});

test('an inactive Team license keeps the hosted checkout available', async ({ page }) => {
  await page.route('https://api.sociobot.in/api/v1/products/ios-review-gate/verify?license=inactive-token', route => route.fulfill({ json: { valid: false, reason: 'invalid', expires_at: null } }));
  await page.goto('/');
  await page.getByLabel('Have a license? Paste it here.').fill('inactive-token');
  await page.getByRole('button', { name: 'Verify Team license' }).click();
  await expect(page.getByText('License not active. Check the token or use Buy Team license.')).toBeVisible();
  await expect(page.getByRole('link', { name: /Buy Team license/ })).toHaveAttribute('href', 'https://api.sociobot.in/api/v1/products/ios-review-gate/checkout');
});

test('a malformed cached license verdict is discarded and a new license can recover', async ({ page }) => {
  const errors = [];
  page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
  page.on('pageerror', error => errors.push(error.message));
  await page.addInitScript(() => {
    localStorage.setItem('sb_license:ios-review-gate', 'corrupt-token');
    localStorage.setItem('sb_license:ios-review-gate:verdict', '{not JSON');
  });
  await page.route('https://api.sociobot.in/api/v1/products/ios-review-gate/verify?license=corrupt-token', route => route.fulfill({ status: 204 }));
  await page.route('https://api.sociobot.in/api/v1/products/ios-review-gate/verify?license=recovered-token', route => route.fulfill({ json: { valid: true, reason: 'ok', expires_at: null } }));
  await page.goto('/');
  await expect.poll(() => page.evaluate(() => localStorage.getItem('sb_license:ios-review-gate:verdict'))).toBeNull();
  await page.getByLabel('Have a license? Paste it here.').fill('recovered-token');
  await page.getByRole('button', { name: 'Verify Team license' }).click();
  await expect(page.getByText('Team license active')).toBeVisible();
  expect(await page.evaluate(() => localStorage.getItem('sb_license:ios-review-gate'))).toBe('recovered-token');
  expect(errors).toEqual([]);
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
  await page.setViewportSize({ width: 390, height: 844 });
  await page.addInitScript(() => {
    localStorage.setItem('sb_license:ios-review-gate', 'test-token');
    localStorage.setItem('sb_license:ios-review-gate:verdict', JSON.stringify({ token: 'test-token', valid: true, checkedAt: Date.now() }));
  });
  await page.goto('/');
  await page.getByLabel('Policy name').fill('Mobile release');
  await page.getByLabel('Active submission limit').fill('7');
  await expect(page.getByRole('group', { name: 'Approved reason codes' })).toBeVisible();
  await expect(page.locator('input[name="reason"]')).toHaveCount(4);
  for (const label of ['DiskSpace E174.1', 'FileTimestamp C617.1', 'SystemBootTime 35F9.1', 'UserDefaults CA92.1']) {
    await expect(page.getByLabel(label)).toBeChecked();
  }
  const accessibility = await new AxeBuilder({ page }).analyze();
  expect(accessibility.violations.filter(v => ['serious', 'critical'].includes(v.impact))).toEqual([]);
  await page.getByLabel('SystemBootTime 35F9.1').uncheck();
  const downloadPromise = page.waitForEvent('download');
  await page.getByRole('button', { name: 'Download Team policy' }).click();
  const download = await downloadPromise;
  expect(download.suggestedFilename()).toBe('team-policy.yaml');
  const content = await (await import('node:fs/promises')).readFile(await download.path(), 'utf8');
  expect(content).toContain('name: "Mobile release"');
  expect(content).toContain('max_active_submissions: 7');
  expect(content).toContain('approved_reason_codes:');
  expect(content).toContain('UserDefaults:\n    - CA92.1');
  expect(content).toContain('DiskSpace:\n    - E174.1');
  expect(content).not.toContain('SystemBootTime:');
  expect(content).not.toContain('additional_reason_codes');
  const cliOutput = execFileSync('cargo', [
    'run', '--quiet', '--locked', '--', 'check',
    '--metadata', 'examples/sample/metadata.json',
    '--release', 'examples/sample/release.yaml',
    '--policy', await download.path(), '--json',
  ], { encoding: 'utf8' });
  const result = JSON.parse(cliOutput);
  expect(result.passed).toBe(true);
  expect(result.policy).toBe('Mobile release');
});

test('@claim:version-metadata shows the Cargo and CLI version with the generated build id', async ({ page }) => {
  const cargo = readFileSync('Cargo.toml', 'utf8');
  const packageJson = JSON.parse(readFileSync('package.json', 'utf8'));
  const version = cargo.match(/^version = "([^"]+)"$/m)?.[1];
  expect(version).toBe(packageJson.version);
  expect(execFileSync('cargo', ['run', '--quiet', '--locked', '--', '--version'], { encoding: 'utf8' }).trim()).toBe(`ios-review-gate ${version}`);
  const buildId = execFileSync('git', ['rev-parse', 'HEAD'], { encoding: 'utf8' }).trim().slice(0, 12);
  for (const path of ['/', '/demo', '/privacy', '/terms', '/404.html']) {
    await page.goto(path);
    await expect(page.locator('footer .build')).toHaveText(`CLI v${version} · build ${buildId}`);
  }
});
