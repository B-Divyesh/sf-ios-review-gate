import './style.css';

const app = document.querySelector('#app');
const status = document.querySelector('#route-status');
const PRODUCT = 'ios-review-gate';
const LICENSE_KEY = `sb_license:${PRODUCT}`;
const VERDICT_KEY = `${LICENSE_KEY}:verdict`;
const API = 'https://api.sociobot.in/api/v1';
let licenseActive = false;
let licenseNotice = '';

const terminal = `<figure class="terminal-sheet" aria-labelledby="terminal-caption">
  <img src="/assets/terminal-recording.svg" width="1000" height="540" alt="Terminal output shows the bundled release passing every check and writing a packet.">
  <figcaption id="terminal-caption">The bundled demo checks a complete sample release and writes its packet.</figcaption>
</figure>`;

const header = () => `<header class="site-header"><nav aria-label="Main navigation">
  <a class="wordmark" href="/" data-link aria-label="iOS Review Gate home"><span aria-hidden="true" class="mark">RG</span><span>iOS Review Gate</span></a>
  <div class="nav-links"><a href="/demo" data-link>Demo</a><a href="/#install">Install</a><a href="/privacy" data-link>Privacy</a></div>
</nav></header>`;

const footer = () => `<footer><p>Check an iOS release and print its review packet.</p><nav aria-label="Footer navigation"><a href="/privacy" data-link>Privacy</a><a href="/terms" data-link>Terms</a><a href="https://hello-factory.sociobot.in" rel="external">Built by Param Factory <span class="sr-only">(external)</span></a></nav><p class="build">v0.1.0 · build 2026.08.28</p></footer>`;

const shell = (content, demoMode = false) => `${demoMode ? `<aside class="demo-banner" aria-label="Demo mode"><strong>Demo — sample data, nothing is saved</strong><span><button id="reset-demo">Reset demo</button><a href="/" data-link>Start for real</a></span></aside>` : ''}${header()}<main id="main" tabindex="-1">${content}</main>${footer()}`;

function paidSection() {
  const controls = licenseActive
    ? `<div class="license-active"><p class="active-label">Team license active</p><p>Build a shared policy file for queue limits and approved reason codes.</p><form id="policy-form"><label for="policy-name">Policy name</label><input id="policy-name" name="name" value="Release team" required><label for="history-limit">Active submission limit</label><input id="history-limit" name="limit" type="number" min="3" max="99" value="8" required><button class="button secondary" type="submit">Download Team policy</button></form><button class="text-button" id="remove-license" type="button">Remove license from this browser</button></div>`
    : `<a class="button primary" href="${API}/products/${PRODUCT}/checkout">Buy Team license</a><form id="license-form"><label for="license-token">Have a license? Paste it here.</label><input id="license-token" name="license" autocomplete="off" required><button class="button secondary" type="submit">Verify Team license</button></form>`;
  return `<section class="paid section-rule" aria-labelledby="paid-title"><div><p class="section-no">TEAM LICENSE / ONCE</p><h2 id="paid-title">Keep team rules beside the release</h2><p>The free gate includes every core check and packet export. Team adds a local policy builder and queue histories beyond three submissions.</p></div><div class="price-box"><p class="price"><span>$</span>39</p><p>One-time purchase.</p>${controls}<p id="license-status" class="legal-note" aria-live="polite">${escapeHtml(licenseNotice)}</p><p class="legal-note">Sociobot is the merchant of record. See <a href="/terms" data-link>terms</a>.</p></div></section>`;
}

function landing() {
  return shell(`<section class="hero drafting-grid">
    <div class="hero-copy"><p class="eyebrow">Local App Review preflight · rules apple-2026.1</p><h1>Check your iOS release before review</h1>
      <p class="lede">For small iOS teams that need one reviewable packet before they queue a build.</p>
      <div class="hero-action"><a class="button primary" href="/demo" data-link>Try it with sample data</a><span>See a checked release and its packet.</span></div>
      <ul class="facts"><li>Runs on your machine.</li><li>No App Store access.</li><li>$39 once for Team rules.</li></ul>
    </div>
    <div class="hero-art"><span class="measure-note" aria-hidden="true">RELEASE / 02.4</span><img src="/assets/release-blueprint.webp" width="768" height="512" alt="An exploded drafting view connects an app archive, metadata sheets, screenshots, and a review queue." fetchpriority="high"><span class="inspection-line" aria-hidden="true"></span></div>
  </section>
  <section class="preview section-rule" aria-labelledby="preview-title"><div><p class="section-no">SHEET 01</p><h2 id="preview-title">See the gate result before submission</h2><p>The same checker powers the command and the sample. Errors name the mismatch and the next fix.</p></div>${terminal}</section>
  <section class="how drafting-grid" aria-labelledby="how-title"><p class="section-no">PROCEDURE / 03</p><h2 id="how-title">Build one decision record</h2><ol class="steps"><li><span>01</span><div><h3>Export archive facts</h3><p>Write version, build, bundle ID, and privacy use to local JSON.</p></div></li><li><span>02</span><div><h3>Describe the release</h3><p>List localized copy, screenshot paths, privacy answers, and queue timing in YAML.</p></div></li><li><span>03</span><div><h3>Print the packet</h3><p>Run one command. Fix holds, review warnings, and keep the dated Markdown file.</p></div></li></ol></section>
  <section id="install" class="install section-rule" aria-labelledby="install-title"><div><p class="section-no">COMMAND / 01</p><h2 id="install-title">Run the gate in your repository</h2><p>Build the single Rust binary, then keep <code>release.yaml</code> beside the app.</p></div><pre tabindex="0" aria-label="Install and run command"><code>cargo install --git https://github.com/B-Divyesh/sf-ios-review-gate
ios-review-gate check \\
  --metadata build/metadata.json \\
  --release release.yaml \\
  --output release-packet.md</code></pre></section>
  <section class="limits drafting-grid" aria-labelledby="limits-title"><p class="section-no">BOUNDARY / LOCAL</p><h2 id="limits-title">Your release stays local</h2><div class="two-col"><p>The CLI reads the paths you give it. It has no telemetry and sends no release data.</p><p>It does not upload builds, scrape App Store Connect, or predict Apple's decision.</p></div></section>
  ${paidSection()}`);
}

function demo() {
  return shell(`<section class="demo-page drafting-grid"><div><p class="eyebrow">Bundled sandbox · no setup</p><h1>Inspect a complete sample release</h1><p class="lede">This recording comes from the shipped Harbor Log sample and the real checker.</p><div class="sample-ledger"><h2>Release facts</h2><dl><div><dt>App</dt><dd>Harbor Log 2.4.0</dd></div><div><dt>Build</dt><dd>108</dd></div><div><dt>Locale</dt><dd>en-US</dd></div><div><dt>Privacy</dt><dd>Manifest present</dd></div><div><dt>Decision</dt><dd class="stamp-pass">PASS</dd></div></dl></div></div>${terminal}</section><section class="packet-preview section-rule" aria-labelledby="packet-title"><div><p class="section-no">OUTPUT / MARKDOWN</p><h2 id="packet-title">The packet records the decision</h2><p>It names the checked rules, artifact identity, queue dates, findings, and reviewer sign-off.</p></div><pre tabindex="0" aria-label="Sample App Review packet"><code># App Review packet — Harbor Log 2.4.0 (108)

Decision: PASS
Rules: apple-2026.1
Owner: Priya Raman

Queue plan
Intended submission: 2026-09-02
Buffered decision: 2026-09-06</code></pre></section>`, true);
}

function legal(kind) {
  const privacy = kind === 'privacy';
  return shell(`<article class="legal"><p class="eyebrow">Effective 28 August 2026</p><h1>${privacy ? 'Privacy for iOS Review Gate' : 'Terms for iOS Review Gate'}</h1>${privacy ? `<h2>Release files stay on your machine</h2><p>The CLI reads only the paths you provide. It does not send telemetry or release data.</p><h2>Website storage</h2><p>The demo stores nothing. If you paste or receive a license, your browser stores the token and its last verification result. You can clear them in browser settings.</p><h2>License verification</h2><p>The browser sends a license token to the Sociobot billing API. It sends no release files. Sociobot processes purchases as merchant of record.</p><h2>Contact</h2><p>Email <a href="mailto:privacy@sociobot.in">privacy@sociobot.in</a> with a privacy question.</p>` : `<h2>Local preflight, not Apple approval</h2><p>The checker finds inconsistencies in the files you provide. It does not guarantee approval or replace Apple's current guidance.</p><h2>License</h2><p>The free CLI is available under the MIT License. A Team purchase is a one-time license for the local policy builder and longer queue histories.</p><h2>Purchases and refunds</h2><p>Sociobot is the merchant of record. A refund revokes its license token. Purchase support is available at <a href="mailto:support@sociobot.in">support@sociobot.in</a>.</p><h2>Warranty</h2><p>The software is provided as is, without warranty. You remain responsible for each submission.</p>`}</article>`);
}

function notFound() {
  return shell(`<section class="not-found drafting-grid"><div><p class="section-no">SHEET / NOT FOUND</p><h1>This release sheet is missing</h1><p>The address does not match a page in this packet.</p><a class="button primary" href="/" data-link>Return to the gate</a></div><div class="missing-mark" aria-hidden="true">404</div></section>`);
}

function escapeHtml(value) {
  const node = document.createElement('span'); node.textContent = value; return node.innerHTML;
}

const routes = {
  '/': { title: 'iOS Review Gate — check a release before review', description: 'Check iOS release metadata, screenshots, privacy answers, and queue timing. Print a local review packet before submission.', render: landing },
  '/demo': { title: 'Demo — iOS Review Gate', description: 'Inspect a complete sample iOS release and the Markdown packet produced by the local gate.', render: demo },
  '/privacy': { title: 'Privacy — iOS Review Gate', description: 'Read how iOS Review Gate keeps release files local and handles license tokens.', render: () => legal('privacy') },
  '/terms': { title: 'Terms — iOS Review Gate', description: 'Read the license, purchase, refund, and warranty terms for iOS Review Gate.', render: () => legal('terms') },
};

function bindPage() {
  document.querySelectorAll('[data-link]').forEach(link => link.addEventListener('click', navigate));
  document.querySelector('#reset-demo')?.addEventListener('click', () => render(true));
  document.querySelector('#license-form')?.addEventListener('submit', restoreLicense);
  document.querySelector('#policy-form')?.addEventListener('submit', downloadPolicy);
  document.querySelector('#remove-license')?.addEventListener('click', removeLicense);
}

function render(focus = false) {
  const route = routes[location.pathname];
  document.title = route?.title || 'Page not found — iOS Review Gate';
  const description = route?.description || 'This iOS Review Gate page does not exist.';
  document.querySelector('meta[name="description"]')?.setAttribute('content', description);
  document.querySelector('meta[property="og:title"]')?.setAttribute('content', document.title);
  document.querySelector('meta[property="og:description"]')?.setAttribute('content', description);
  document.querySelector('meta[name="twitter:title"]')?.setAttribute('content', document.title);
  document.querySelector('meta[name="twitter:description"]')?.setAttribute('content', description);
  const canonical = document.querySelector('link[rel="canonical"]');
  if (canonical) canonical.href = `https://ios-review-gate.sociobot.in${route ? location.pathname : '/404'}`;
  app.innerHTML = route ? route.render() : notFound();
  bindPage();
  if (focus) {
    const h1 = document.querySelector('h1'); h1.setAttribute('tabindex', '-1'); h1.focus();
    status.textContent = document.title; scrollTo(0, 0);
  }
}

function navigate(event) {
  if (event.button !== 0 || event.metaKey || event.ctrlKey) return;
  event.preventDefault(); history.pushState({}, '', event.currentTarget.href); render(true); initializeLicense();
}
window.addEventListener('popstate', () => { render(true); initializeLicense(); });

async function verifyLicense(token) {
  const cached = JSON.parse(localStorage.getItem(VERDICT_KEY) || 'null');
  if (cached?.token === token && Date.now() - cached.checkedAt < 86400000) return cached.valid;
  try {
    const response = await fetch(`${API}/products/${PRODUCT}/verify?license=${encodeURIComponent(token)}`);
    if (!response.ok) throw new Error('verification failed');
    const verdict = await response.json();
    localStorage.setItem(VERDICT_KEY, JSON.stringify({ token, valid: verdict.valid, checkedAt: Date.now() }));
    return verdict.valid;
  } catch {
    return cached?.token === token ? Boolean(cached.valid) : null;
  }
}

async function restoreLicense(event) {
  event.preventDefault();
  const token = String(new FormData(event.currentTarget).get('license') || '').trim();
  if (!token) return;
  const button = event.currentTarget.querySelector('button');
  button.textContent = 'Checking license…'; button.disabled = true;
  localStorage.setItem(LICENSE_KEY, token);
  const verdict = await verifyLicense(token);
  licenseActive = verdict === true;
  licenseNotice = verdict === null ? 'Could not check the license. Connect once and try again.' : licenseActive ? 'Verified on this device.' : 'License not active. Check the token or buy a license.';
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
  const yaml = `name: ${JSON.stringify(name)}\nmax_active_submissions: ${limit}\nadditional_reason_codes: {}\n`;
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
  if (location.pathname === '/demo') return;
  const received = new URLSearchParams(location.search).get('license');
  if (received) { localStorage.setItem(LICENSE_KEY, received); history.replaceState({}, '', location.pathname); }
  const token = received || localStorage.getItem(LICENSE_KEY);
  if (!token) return;
  const cached = JSON.parse(localStorage.getItem(VERDICT_KEY) || 'null');
  licenseActive = cached?.token === token && cached.valid === true;
  if (licenseActive) renderPaidPanel();
  const valid = await verifyLicense(token);
  if (valid !== null && valid !== licenseActive) { licenseActive = valid; renderPaidPanel(); }
  if (valid === false) { licenseNotice = 'License no longer active. Check the token or buy a license.'; renderPaidPanel(); }
}

render();
initializeLicense();
if ('serviceWorker' in navigator) window.addEventListener('load', () => navigator.serviceWorker.register('/sw.js'));
