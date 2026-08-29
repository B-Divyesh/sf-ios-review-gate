import './style.css';
import rulesSource from '../../rules/apple-2026.1.yaml?raw';

const app = document.querySelector('#app');
const status = document.querySelector('#route-status');
const PRODUCT = 'ios-review-gate';
const LICENSE_KEY = `sb_license:${PRODUCT}`;
const VERDICT_KEY = `${LICENSE_KEY}:verdict`;
const API = 'https://api.sociobot.in/api/v1';
const CHECKOUT = `${API}/products/${PRODUCT}/checkout`;
const VERSION = __CLI_VERSION__;
const BUILD_ID = __BUILD_ID__;
const APPLE_REASON_CODES = [...rulesSource.matchAll(/^  ([A-Za-z]+): \[([^\]]+)\]$/gm)]
  .map(([, category, codes]) => ({ category, codes: codes.split(',').map(code => code.trim()) }));
let licenseActive = false;
let licenseNotice = '';
let routeGeneration = 0;
let licenseController = null;

const terminal = `<figure class="terminal-sheet" aria-labelledby="terminal-caption">
  <img src="/assets/terminal-recording.svg" width="1000" height="540" alt="Terminal output shows the bundled release passing every check and writing a Markdown review packet.">
  <figcaption id="terminal-caption">The bundled demo checks a complete sample release and writes its review packet.</figcaption>
</figure>`;

const header = () => `<header class="site-header"><nav aria-label="Main navigation">
  <a class="wordmark" href="/" data-link aria-label="RG — iOS Review Gate home"><span aria-hidden="true" class="mark">RG</span><span>iOS Review Gate</span></a>
  <div class="nav-links"><a href="/demo" data-link>Demo</a><a href="/#install">Install</a><a href="/privacy" data-link>Privacy</a></div>
</nav></header>`;

const footer = () => `<footer><p>Check an iOS release and print its Markdown review packet.</p><nav aria-label="Footer navigation"><a href="/privacy" data-link>Privacy</a><a href="/terms" data-link>Terms</a><a href="https://hello-factory.sociobot.in" rel="external">Built by Param Factory <span class="sr-only">(external)</span></a></nav><p class="build">CLI v${VERSION} · build ${BUILD_ID}</p></footer>`;

const shell = (content, demoMode = false) => `${demoMode ? `<aside class="demo-banner" aria-label="Demo mode"><strong>Demo — sample data, nothing is saved</strong><span><button id="reset-demo">Reset demo</button><a href="/#install">Install the CLI</a></span></aside>` : ''}${header()}<main id="main" tabindex="-1">${content}</main>${footer()}`;

function paidSection() {
  const reasonControls = APPLE_REASON_CODES.map(({ category, codes }) => codes.map(code => `<label class="reason-option"><input type="checkbox" name="reason" value="${category}|${code}" aria-label="${category} ${code}" checked><span aria-hidden="true"><strong>${category}</strong><small>${code}</small></span></label>`).join('')).join('');
  const controls = licenseActive
    ? `<div class="license-active"><p class="active-label">Team license active</p><p>Build a shared policy file for queue limits and approved reason codes.</p><form id="policy-form"><label for="policy-name">Policy name</label><input id="policy-name" name="name" value="Release team" required><label for="history-limit">Active submission limit</label><input id="history-limit" name="limit" type="number" min="3" max="99" value="8" required><fieldset><legend>Approved reason codes</legend><p>Select the Apple reason codes your team permits.</p><div class="reason-options">${reasonControls}</div></fieldset><button class="button secondary" type="submit">Download Team policy</button></form><button class="text-button" id="remove-license" type="button">Remove license from this browser</button></div>`
    : `<a class="button primary" href="${CHECKOUT}" rel="external">Buy Team license<span class="sr-only"> at Sociobot checkout</span></a><form id="license-form"><label for="license-token">Have a license? Paste it here.</label><input id="license-token" name="license" autocomplete="off" required><button class="button secondary" type="submit">Verify Team license</button></form>`;
  return `<section class="paid section-rule" aria-labelledby="paid-title"><div><p class="section-no">Team policy</p><h2 id="paid-title">Keep team rules beside the release</h2><p>Core checks and review packet export work without a Team license. Verified Team licenses enable the local policy download. Team policies support queue histories beyond three submissions.</p></div><div class="price-box"><p class="price"><span>$</span>39</p><p class="price-note">One-time Team license</p>${controls}<p id="license-status" class="legal-note" aria-live="polite">${escapeHtml(licenseNotice)}</p><p class="legal-note">Sociobot hosts checkout and license verification. For billing help, email <a href="mailto:billing@sociobot.in">billing@sociobot.in</a>. See <a href="/privacy" data-link>privacy</a> and <a href="/terms" data-link>terms</a>.</p></div></section>`;
}

function landing() {
  return shell(`<section class="hero drafting-grid">
    <div class="hero-copy"><p class="eyebrow">Checks metadata, screenshots, privacy, and queue timing</p><h1>Check your iOS release before review</h1>
      <p class="lede">For small iOS teams that need one Markdown review packet before they queue a build.</p>
      <div class="hero-action"><a class="button primary" href="/?demo=1" data-link>Try it with sample data</a><span>Open a checked sample and its review packet.</span></div>
      <ul class="facts"><li>Release files stay on your machine.</li><li>The demo works offline after one visit.</li><li>Checks and review packets cost $0.</li></ul>
    </div>
    <div class="hero-art"><img src="/assets/release-blueprint.webp" width="768" height="512" alt="An exploded drafting view connects an app archive, metadata sheets, screenshots, and a review queue." fetchpriority="high"><span class="inspection-line" aria-hidden="true"></span></div>
  </section>
  <section class="preview section-rule" aria-labelledby="preview-title"><div><h2 id="preview-title">See the check result before submission</h2><p>The website sample and command use the same bundled checker. Errors name the mismatch and the next fix.</p></div>${terminal}</section>
  <section class="how drafting-grid" aria-labelledby="how-title"><h2 id="how-title">Build one review packet</h2><ol class="steps"><li><span>01</span><div><h3>Inspect your archive</h3><p>Read version, build, bundle ID, and privacy use from an .xcarchive or .ipa.</p></div></li><li><span>02</span><div><h3>Describe the release</h3><p>List localized copy, screenshot paths, privacy answers, and queue timing in YAML.</p></div></li><li><span>03</span><div><h3>Print the review packet</h3><p>Run one command. Fix holds and warnings, then keep the Markdown review packet.</p></div></li></ol></section>
  <section id="install" class="install section-rule" aria-labelledby="install-title"><div><h2 id="install-title">Run the checker in your repository</h2><p>Build the single Rust binary, then keep <code>release.yaml</code> beside the app.</p></div><pre tabindex="0" aria-label="Install and run command"><code>cargo install --git https://github.com/B-Divyesh/sf-ios-review-gate
ios-review-gate check \\
  --archive build/HarborLog.xcarchive \\
  --release release.yaml \\
  --output release-packet.md</code></pre></section>
  <section class="limits drafting-grid" aria-labelledby="limits-title"><h2 id="limits-title">Your release files stay local</h2><div class="two-col"><p>The CLI reads the paths you give it. It has no telemetry and sends no release data.</p><p>It does not upload builds, scrape App Store Connect, or predict Apple's decision.</p></div></section>
  ${paidSection()}`);
}

function demo() {
  return shell(`<section class="demo-page drafting-grid"><div><p class="eyebrow">Bundled sample · no setup</p><h1>Inspect a complete sample release</h1><p class="lede">This recording uses the shipped Harbor Log sample and the same checker as the CLI.</p><div class="sample-ledger"><h2>Release facts</h2><dl><div><dt>App</dt><dd>Harbor Log 2.4.0</dd></div><div><dt>Build</dt><dd>108</dd></div><div><dt>Locale</dt><dd>en-US</dd></div><div><dt>Privacy</dt><dd>Manifest present</dd></div><div><dt>Decision</dt><dd class="stamp-pass">PASS</dd></div></dl></div></div>${terminal}</section><section class="packet-preview section-rule" aria-labelledby="packet-title"><div><h2 id="packet-title">The review packet records the decision</h2><p>It names the checked rules, artifact identity, queue dates, findings, and reviewer sign-off.</p></div><pre tabindex="0" aria-label="Sample Markdown review packet"><code># Markdown review packet — Harbor Log 2.4.0 (108)

Decision: PASS
Rules: apple-2026.1
Owner: Priya Raman

Queue plan
Intended submission: 2026-09-02
Buffered decision: 2026-09-06</code></pre></section>`, true);
}

function legal(kind) {
  const privacy = kind === 'privacy';
  return shell(`<article class="legal"><p class="eyebrow">Effective 29 August 2026</p><h1>${privacy ? 'Privacy for iOS Review Gate' : 'Terms for iOS Review Gate'}</h1>${privacy ? `<h2>Release files stay on your machine</h2><p>The CLI reads only the paths you provide. It does not send telemetry or release data.</p><h2>Website storage</h2><p>The demo saves no personal release data. Its offline shell caches same-origin static site files only. If you paste or receive a license, your browser stores the token and its last verification result. You can clear them in browser settings.</p><h2>License verification</h2><p>The browser sends a license token to the Sociobot billing API. It sends no release files.</p><h2>Contact</h2><p>Email <a href="mailto:privacy@sociobot.in">privacy@sociobot.in</a> with a privacy question.</p>` : `<h2>Local check, not Apple approval</h2><p>The checker finds inconsistencies in the files you provide. It does not guarantee approval or replace Apple's current guidance.</p><h2>License</h2><p>The CLI uses the MIT License. A Team license costs $39 once and adds local policy downloads. Team policies support queue histories beyond three submissions.</p><h2>Purchase support</h2><p>Sociobot hosts checkout and license verification. Email <a href="mailto:billing@sociobot.in">billing@sociobot.in</a> for billing help.</p><h2>Warranty</h2><p>The software is provided as is, without warranty. You remain responsible for each submission.</p>`}</article>`);
}

function notFound() {
  return shell(`<section class="not-found drafting-grid"><div><h1>This page does not exist</h1><p>The address does not match a page on this site.</p><a class="button primary" href="/" data-link>Return home</a></div><div class="missing-mark" aria-hidden="true">404</div></section>`);
}

function escapeHtml(value) {
  const node = document.createElement('span'); node.textContent = value; return node.innerHTML;
}

function licenseStorageAllowed(generation = routeGeneration) {
  return generation === routeGeneration && currentRoute() !== routes['/demo'];
}

function cachedLicenseVerdict(generation = routeGeneration) {
  if (!licenseStorageAllowed(generation)) return null;
  const raw = localStorage.getItem(VERDICT_KEY);
  if (!raw) return null;
  try {
    const cached = JSON.parse(raw);
    if (
      cached
      && typeof cached === 'object'
      && typeof cached.token === 'string'
      && typeof cached.valid === 'boolean'
      && Number.isFinite(cached.checkedAt)
      && cached.checkedAt >= 0
    ) return cached;
  } catch {
    // A browser extension, an older build, or manual storage editing can leave invalid JSON.
  }
  if (licenseStorageAllowed(generation)) localStorage.removeItem(VERDICT_KEY);
  return null;
}

const routes = {
  '/': { title: 'iOS Review Gate — check a release before review', description: 'Check iOS release metadata, screenshots, privacy answers, and queue timing. Print a local review packet before submission.', render: landing },
  '/demo': { title: 'Demo — iOS Review Gate', description: 'Inspect a complete sample iOS release and the Markdown review packet produced by the local checker.', render: demo },
  '/privacy': { title: 'Privacy — iOS Review Gate', description: 'Read how iOS Review Gate keeps release files local and handles license tokens.', render: () => legal('privacy') },
  '/terms': { title: 'Terms — iOS Review Gate', description: 'Read the license, purchase support, and warranty terms for iOS Review Gate.', render: () => legal('terms') },
};

function bindPage() {
  document.querySelectorAll('[data-link]').forEach(link => link.addEventListener('click', navigate));
  document.querySelector('#reset-demo')?.addEventListener('click', () => render(true));
  document.querySelector('#license-form')?.addEventListener('submit', restoreLicense);
  document.querySelector('#policy-form')?.addEventListener('submit', downloadPolicy);
  document.querySelector('#remove-license')?.addEventListener('click', removeLicense);
}

function currentRoute() {
  if (location.pathname === '/' && new URLSearchParams(location.search).get('demo') === '1') return routes['/demo'];
  return routes[location.pathname];
}

function render(focus = false) {
  const route = currentRoute();
  document.title = route?.title || 'Page not found — iOS Review Gate';
  const description = route?.description || 'This iOS Review Gate page does not exist.';
  document.querySelector('meta[name="description"]')?.setAttribute('content', description);
  document.querySelector('meta[property="og:title"]')?.setAttribute('content', document.title);
  document.querySelector('meta[property="og:description"]')?.setAttribute('content', description);
  document.querySelector('meta[name="twitter:title"]')?.setAttribute('content', document.title);
  document.querySelector('meta[name="twitter:description"]')?.setAttribute('content', description);
  const canonical = document.querySelector('link[rel="canonical"]');
  if (canonical) canonical.href = `https://ios-review-gate.sociobot.in${route === routes['/demo'] ? '/demo' : route ? location.pathname : '/404'}`;
  app.innerHTML = route ? route.render() : notFound();
  bindPage();
  if (focus) {
    const h1 = document.querySelector('h1'); h1.setAttribute('tabindex', '-1'); h1.focus();
    status.textContent = document.title; scrollTo(0, 0);
  }
}

function navigate(event) {
  if (event.button !== 0 || event.metaKey || event.ctrlKey) return;
  event.preventDefault(); invalidateLicenseWork(); history.pushState({}, '', event.currentTarget.href); render(true); initializeLicense();
}
window.addEventListener('popstate', () => { invalidateLicenseWork(); render(true); initializeLicense(); });

function invalidateLicenseWork() {
  routeGeneration += 1;
  licenseController?.abort();
  licenseController = null;
}

function startLicenseWork() {
  licenseController?.abort();
  const controller = new AbortController();
  licenseController = controller;
  return { controller, generation: routeGeneration };
}

async function verifyLicense(token, task) {
  const cached = cachedLicenseVerdict(task.generation);
  if (cached?.token === token && Date.now() - cached.checkedAt < 86400000) return cached.valid;
  try {
    const response = await fetch(`${API}/products/${PRODUCT}/verify?license=${encodeURIComponent(token)}`, { signal: task.controller.signal });
    if (!response.ok) throw new Error('verification failed');
    const verdict = await response.json();
    if (!licenseStorageAllowed(task.generation) || task.controller.signal.aborted) return null;
    localStorage.setItem(VERDICT_KEY, JSON.stringify({ token, valid: verdict.valid, checkedAt: Date.now() }));
    return verdict.valid;
  } catch {
    if (!licenseStorageAllowed(task.generation) || task.controller.signal.aborted) return null;
    return cached?.token === token ? Boolean(cached.valid) : null;
  }
}

async function restoreLicense(event) {
  event.preventDefault();
  const token = String(new FormData(event.currentTarget).get('license') || '').trim();
  if (!token) return;
  const task = startLicenseWork();
  const button = event.currentTarget.querySelector('button');
  button.textContent = 'Checking license…'; button.disabled = true;
  if (!licenseStorageAllowed(task.generation)) return;
  localStorage.setItem(LICENSE_KEY, token);
  const verdict = await verifyLicense(token, task);
  if (!licenseStorageAllowed(task.generation)) return;
  licenseActive = verdict === true;
  licenseNotice = verdict === null ? 'Could not check the license. Connect once and try again.' : licenseActive ? 'Verified on this device.' : 'License not active. Check the token or use Buy Team license.';
  renderPaidPanel();
}

function removeLicense() {
  localStorage.removeItem(LICENSE_KEY);
  localStorage.removeItem(VERDICT_KEY);
  licenseActive = false;
  licenseNotice = 'License removed from this browser.';
  renderPaidPanel();
}

function downloadPolicy(event) {
  event.preventDefault();
  const data = new FormData(event.currentTarget);
  const name = String(data.get('name')).replace(/[\r\n]/g, ' ').trim();
  const limit = Math.max(3, Math.min(99, Number(data.get('limit')) || 3));
  const selected = data.getAll('reason').map(value => String(value).split('|'));
  const grouped = selected.reduce((result, [category, code]) => {
    if (!result.has(category)) result.set(category, []);
    result.get(category).push(code);
    return result;
  }, new Map());
  const reasonYaml = grouped.size === 0
    ? 'approved_reason_codes: {}\n'
    : `approved_reason_codes:\n${[...grouped].map(([category, codes]) => `  ${category}:\n${codes.map(code => `    - ${code}`).join('\n')}`).join('\n')}\n`;
  const yaml = `name: ${JSON.stringify(name)}\nmax_active_submissions: ${limit}\n${reasonYaml}`;
  const url = URL.createObjectURL(new Blob([yaml], { type: 'text/yaml' }));
  const link = document.createElement('a'); link.href = url; link.download = 'team-policy.yaml'; link.click();
  URL.revokeObjectURL(url);
  licenseNotice = 'Team policy downloaded. Pass it to the CLI with --policy.';
  document.querySelector('#license-status').textContent = licenseNotice;
}

function renderPaidPanel() {
  if (location.pathname !== '/') return;
  const current = document.querySelector('.paid');
  if (!current) return;
  const holder = document.createElement('div'); holder.innerHTML = paidSection();
  current.replaceWith(holder.firstElementChild); bindPage();
}

async function initializeLicense() {
  if (currentRoute() === routes['/demo']) return;
  const task = startLicenseWork();
  const received = new URLSearchParams(location.search).get('license');
  if (received && licenseStorageAllowed(task.generation)) { localStorage.setItem(LICENSE_KEY, received); history.replaceState({}, '', location.pathname); }
  const token = received || localStorage.getItem(LICENSE_KEY);
  if (!token) return;
  const cached = cachedLicenseVerdict(task.generation);
  licenseActive = cached?.token === token && cached.valid === true;
  if (licenseActive) renderPaidPanel();
  const valid = await verifyLicense(token, task);
  if (!licenseStorageAllowed(task.generation)) return;
  if (valid !== null && valid !== licenseActive) { licenseActive = valid; renderPaidPanel(); }
  if (valid === false) { licenseNotice = 'License no longer active. Check the token or use Buy Team license.'; renderPaidPanel(); }
}

render();
initializeLicense();
if ('serviceWorker' in navigator) window.addEventListener('load', () => navigator.serviceWorker.register('/sw.js'));
