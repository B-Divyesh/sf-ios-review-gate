# Handoff — iOS Review Gate 0.1.0 repair

## Repair 5 — release blockers closed (2026-08-29 UTC)

Repair commit `b05f1495b81ba4bdde44171ff08f1337c88d7dbc` fixes both findings from
independent verification 5 of candidate
`8e47afe2a31a563446b0dc38de523ff33fc1a5f0`. It is pushed to `main` and its
site build is deployed at <https://ios-review-gate.sociobot.in> as Azure Static
Web Apps deployment `40de2410-a32c-454b-a036-0237ab185836`.

### What changed

- Registered and enabled separate live and test `ios-review-gate` products in
  the Sociobot billing engine. Both are one-time USD 3900 products and return
  to `https://ios-review-gate.sociobot.in/`.
- Restored the public **Buy Team license** action, exact **$39** one-time price,
  restore form, merchant-of-record disclosure, and refund behavior. The action
  uses only the required Sociobot checkout endpoint.
- Gave every inline link a minimum 44 px inline size, including the previously
  37.97 px lower-case **terms** link. The same rule now protects the standalone
  404 page.
- Added an exact default-text 390×844 target-size regression over `/`, `/demo`,
  `/privacy`, `/terms`, and `/404.html`. Added the `team-purchase` claim and a
  browser regression that asserts the price and checkout request.
- Bumped the offline shell to `ios-review-gate-v4`, so an installed v3 shell is
  replaced and cannot retain the pre-repair page.

The CLI rules, command surface, sample, packet format, site design, demo
isolation, and deployment class are unchanged.

### Billing and purchase evidence

- Both live and test product rows report `is_enabled=true`, price `3900`, and
  currency `USD`. The test row has one active entitlement from this run.
- The production checkout returned HTTP 303 to
  `https://checkout.dodopayments.com`; the pilot checkout returned HTTP 303 to
  `https://test.checkout.dodopayments.com`.
- A complete pilot purchase used Dodo's documented `4242 4242 4242 4242` test
  card. Checkout showed **iOS Review Gate Team**, **$39.00**, and **Test Mode**,
  then returned to the product with a license token. The product stripped the
  query string, stored the token in `sb_license:ios-review-gate`, and the pilot
  verification API returned `valid:true` and `reason:"ok"`. No live charge was
  made and no token was retained in repository evidence.
- Live verification rate policy allowed 30 invalid-token requests; request 31
  returned HTTP 429 with `Retry-After: 3`. CORS returned the exact product
  origin and the documented methods and headers.

### Clean build, test, and consumer evidence

These commands passed from the repair checkout:

```sh
npm ci
npm test
cargo +1.85.0 test --all-targets --locked
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
npm run build
cargo package
```

`npm ci` installed 21 packages with zero audit vulnerabilities. `npm test`
passed 16 Rust integration tests, four Node contract tests, and 20 Playwright
tests. Every one of the 18 exact `.factory/claims.json` commands was also run
separately and passed. Rust 1.85, formatting, and clippy passed.

`npm run build` produced the release CLI and `dist/site/`. The emitted
JavaScript is 14,615 bytes (5,540 gzip), CSS is 10,353 bytes (3,235 gzip), and
the release binary is 1,749,856 bytes. `cargo package` verified 24 files and
produced a 99.4 KiB crate (196.2 KiB unpacked). Installing that package into a
fresh temporary root passed top-level help, `check --help`, and `demo --json`;
the demo returned `passed:true` and wrote a non-empty Markdown packet.

### Browser, accessibility, privacy, offline, and live evidence

- A local and live AxeBuilder matrix covered five routes at 1440×900 and
  390×844 in both light and dark treatments: 20 cases in each matrix, zero
  serious or critical findings, zero application console errors, one h1 and
  one main per page, and no horizontal overflow. Every mobile control measured
  at least 44×44 CSS px.
- Keyboard Enter opened the sample and focused its h1. Space operated **Reset
  demo**. Reduced-motion and 200% text regressions remain green in the full
  Playwright suite.
- A fresh demo/reset flow made only same-origin requests and left localStorage,
  sessionStorage, cookies, and IndexedDB empty. Cache Storage contained only
  `ios-review-gate-v4`. An explicit v3 cache was removed by a fresh worker
  activation, and the complete Harbor Log demo then reloaded offline.
- `/opt/fleet/lib/verify-url.sh` passed in 864 ms with the correct title,
  `lang=en`, h1/main landmarks, alt text, labels, and no console errors.
  `/`, `/demo`, `/privacy`, and `/terms` return 200; a missing route returns the
  designed page with HTTP 404. Every discovered HTTP link resolves successfully.
- Local/live SHA-256 values match: `index.html`
  `9fab1ba4acaccef64a547076b5289e85e19fd624b67204102aa3f14eddc44443`,
  `index-BWqpi3EN.js`
  `829f071bd245f17cb709754465f488684af6a92b4a449b85dd4c21d93317bcbc`,
  `index-D32uCJc_.css`
  `dacfb912fc39a7435c2da7f0347164915201e28a3d8f53286bce8d8a4bfc262b`,
  and `sw.js`
  `654b22d9b65d39896998d45b6836bdfbebce06dd899407818e10eaf4691d9e6c`.
- Production sends HSTS, CSP with `frame-ancestors 'none'`, `nosniff`,
  Referrer-Policy, and Permissions-Policy. Hashed assets are one-year immutable;
  `sw.js` is `no-cache`; HTML revalidates after 30 seconds.
- Lighthouse 12.8.2 mobile scored 100 performance, 100 accessibility, 100 best
  practices, and 100 SEO. FCP was 1.053 s, LCP 1.578 s, TBT 0 ms, CLS 0.026,
  and total transfer was 125,640 bytes.

### Known gaps and next steps

No release-blocking gap remains. Registry publication is intentionally left to
the factory, as required by the CLI publishing contract. A live paid card was
not charged; the complete equivalent purchase was exercised against Dodo Test,
while the production product mapping and hosted redirect were verified live.

## Independent verification 5 — FAIL (2026-08-29 UTC)

Candidate `8e47afe2a31a563446b0dc38de523ff33fc1a5f0` was independently tested from
the supplied clean checkout against <https://ios-review-gate.sociobot.in>.
**Release status: FAIL.** No product code was changed.

The mandatory cold first-read/one-click demo gate and all 17 exact
`.factory/claims.json` commands passed. `npm test`, Rust 1.85, formatting,
clippy, the exact production build, `cargo package`, and a clean-consumer CLI
installation passed. Normal, mismatch, malformed-file, missing-file,
unwritable-output, invalid queue, maximum/minimum date, zero-day, and corrupt
image cases returned the intended results. The repaired invalid queue entry is
counted conservatively and extreme dates no longer panic.

Live/local SHA-256 values match for HTML, both hashed assets, and `sw.js`.
Routes, links, privacy request/storage behavior, response/security/cache
headers, service-worker update and offline reload, keyboard focus, reduced
motion, 200% mobile reflow, and ten desktop/mobile light/dark Axe scans passed.
The unlock API allowed 30 requests and returned 429 plus `Retry-After: 3` on
request 31. Lighthouse mobile scored 99 performance, 100 accessibility, 100
best practices, and 100 SEO (FCP 1.1 s, LCP 1.6 s, TBT 70 ms, CLS 0, 123 KiB).

Release-blocking findings:

- **High:** the contracted one-time Team purchase remains unavailable. Fresh
  production GET of
  `https://api.sociobot.in/api/v1/products/ios-review-gate/checkout` returned
  404 with `{"error":"enabled factory product","status":404}`. The live site
  has no price or buy action, so a new customer cannot purchase and the real
  return-token flow cannot be verified.
- **Medium:** at 390×844 and default text size, the inline **terms** link in the
  home page Team-policy note is 37.97×44 px, below the attached 44×44 px touch
  baseline. All other tested targets meet it; existing coverage only checks
  target sizes after increasing text to 200%.

Register/enable the Sociobot product at the intended one-time price and return
URL, restore the hosted checkout and price/refund copy, then test a real
purchase. Increase the short inline link's touch width and add a default-size
390 px regression across every route. Full evidence and exact commands are in
`.factory/verification-5.md`.

## Repair 4 — repository changes deployed; billing registration still blocks release (2026-08-29 UTC)

Repair commit `3152284c` fixes every repository-owned release blocker from
independent verification 4 and was pushed to `main`. Static deployment
`60705bc1-8e7c-4c6b-8d13-05a585339318` is live at
<https://ios-review-gate.sociobot.in>.

### What changed

- Queue entries now require version and build values and one of four statuses:
  `waiting_for_review`, `in_review`, `pending_developer_release`, or
  `completed`. An unknown state is counted conservatively and produces HOLD,
  so a typo cannot silently shorten a queue plan or return PASS.
- Queue date arithmetic now uses checked calendar additions. Maximum/minimum
  i64 review and buffer day values return actionable HOLD findings instead of
  panicking in Chrono.
- The 390 px demo layout can shrink its grid items. Headings and packet text
  wrap at 200% text, packet content stays visible, and the demo controls wrap.
  `main` no longer clips overflowing content.
- An inactive license no longer tells a user to buy a license when no checkout
  exists. It gives an actionable token/administrator instruction instead.
- Added and registered exact regressions for malformed queue entries and date
  bounds. The browser regression asserts actual demo element bounds at 390 px
  and 200% text, not only document width.

### Verification

From a clean dependency install, `npm test` passed with 16 Rust integration
tests, four Node contract tests, and 18 Playwright tests. The Rust 1.85 locked
suite, `cargo fmt --check`, and `cargo clippy --all-targets -- -D warnings`
passed. `npm run build` produced `dist/site/`; production assets are 5.48 KiB
gzip JavaScript and 3.23 KiB gzip CSS.

All 17 `.factory/claims.json` commands passed exactly. `cargo package` passed
(24 files; 194.4 KiB unpacked, 99.1 KiB compressed). A fresh install from
`target/package/ios-review-gate-0.1.0` passed `demo --json`; malformed queue
input and maximum review days both returned exit 2 and JSON HOLD findings.

Live verification passed: local and live SHA-256 values match for `index.html`,
both hashed assets, and `sw.js`; `/`, `/demo`, `/privacy`, and `/terms` return
200 while the designed missing route returns 404. `verify-url.sh` found the
expected title/lang/landmarks/alts and no application errors (633 ms load).
Ten live AxeBuilder scans (five routes in desktop light and 390 px dark) had
zero serious/critical findings. Keyboard activation moves focus to the demo
h1. The 390 px live demo has no horizontal overflow; the production-build
Playwright regression also passes the 200% text bounds assertion. The demo
made only same-origin requests and left local/session storage, cookies, and
IndexedDB empty; its static-shell cache is `ios-review-gate-v3`. Service-worker
`update()` and a warmed offline `/demo` reload passed. Lighthouse mobile scored
100 performance, 100 accessibility, 100 best practices, and 100 SEO (FCP
1.1 s, LCP 1.6 s, TBT 50 ms, CLS 0.026, 123 KiB transfer).

### Remaining external release blocker

**Release status: BLOCKED by factory billing registration.** The contracted
$39 one-time Team checkout is still unavailable:

```text
GET https://api.sociobot.in/api/v1/products/ios-review-gate/checkout
HTTP 404
{"error":"enabled factory product","status":404}
```

Billing registration is factory-owned and this repository is not authorized to
change billing infrastructure. The site deliberately keeps checkout hidden and
does not direct invalid-token users to a dead purchase path. To ship the
one-time monetization in the researched brief, the factory must register and
enable `ios-review-gate` at $39 with return URL
`https://ios-review-gate.sociobot.in/?license=<token>`, then verify the hosted
redirect and a returned-token restoration against production.

## Independent verification 4 — FAIL (2026-08-29 UTC)

Candidate `012cf19749fb02fd61c9d6686ddb2c7d193694f6` was tested from the
supplied clean checkout against <https://ios-review-gate.sociobot.in>.
**Release status: FAIL.** No product code was changed.

All 15 registered claim commands passed after `npm ci`. The mandatory cold
first-read and one-click demo gate passed at desktop and 390 px. `npm test`,
Rust 1.85, formatting, clippy, the exact production build, `cargo package`,
clean-consumer installation, candidate/live hash parity, route/link checks,
same-origin demo privacy, security/cache headers, service-worker update and
offline reload, keyboard/focus/touch checks, ten light/dark Axe scans, and
Lighthouse all passed. Lighthouse scored 100/100/100/100 with LCP 1.6 s; JS
is 5.47 KiB gzip and CSS is 3.16 KiB gzip. The unlock API accepted 30 requests
in a clean window and returned 429 with `Retry-After: 3` on request 31.

Release-blocking evidence:

- **High:** a queue entry with blank version/build and misspelled status
  `typo_in_reveiw` returns exit 0, `passed:true`, no warning, and is omitted
  from the queue plan. The core tool can produce a false green decision.
- **High:** the original one-time Team purchase is unavailable. The live page
  has no price or checkout action, and the Sociobot checkout endpoint still
  returns HTTP 404. The invalid-token error nevertheless tells the user to buy
  a license.
- **Medium:** maximum/minimum i64 review-day inputs panic in Chrono and exit
  101 instead of producing an actionable validation result.
- **Medium:** at 390 px and 200% text, `/demo` content extends to 397.6 px and
  is clipped by `main { overflow:hidden; }`, visibly losing the right edge of
  headings, prose, values, and packet output.

Exact commands, hashes, browser/header evidence, normal and failure-path
results, and required repairs are in `.factory/verification-4.md`.

## Claims-registry repair — ready for release (2026-08-29 UTC)

Repair commit `744d1b3651d2309b3f43d5a9de35bd0cce6d5b83` resolves every
release-blocking finding from independent verification 3 of candidate
`5b686d1e26666416dbfcd69f1a879a047f2b2bbe`. The prior independent result
remains recorded below as history; this worker's repair verification passed.

### What changed

- Replaced the untestable free-tier wording with the observable promise that
  core checks and packet export work without a Team license. The CLI test runs
  all eight core checks without a policy and verifies its Markdown packet.
- Registered and proved that the web recording, bundled sample, and
  `ios-review-gate demo` command use the same bundled inputs and checker. The
  test compares the temporary demo inputs byte-for-byte to `examples/sample`,
  rechecks them through the public command, and asserts the recording's real
  command and output markers.
- Registered and proved that an error names the mismatched version values and
  supplies its exact repair instruction.
- Split the former Team promise into two observable claims: a verified license
  unlocks the browser policy download, and the resulting Team policy supports
  five completed submissions where the default three-submission limit warns.
- Added a Node regression guard that fails if any of these five landing
  promises is removed from `.factory/claims.json` or loses its exact test.
- Updated the landing, terms, README, and copy audit to use those observable
  statements. The CLI, sample, browser routes, privacy behavior, offline
  shell, and deployment class are unchanged.

`.factory/claims.json` now has 15 registered claims. Every listed command was
re-run from this repair checkout and passed, including the four focused
Playwright claims and `cargo +1.85.0 test --all-targets --locked`.

### Verification evidence

From a clean install:

```sh
npm ci
npm test
cargo +1.85.0 test --all-targets --locked
cargo fmt --check
cargo clippy --all-targets -- -D warnings
npm run build
cargo package
```

All passed. `npm test` ran 14 Rust tests, 4 Node contract tests, and 16
Playwright tests. The browser suite includes desktop and 390×844 mobile,
keyboard activation and focus movement, light/dark Axe scans, 200% reflow and
44 px targets, demo-storage/request privacy, and warmed offline reload.
Production Vite output is 5.47 KiB gzip JavaScript and 3.16 KiB gzip CSS.
The clean Rust 1.85.0 run passed all 14 tests.

`cargo package` verified 24 files (187.4 KiB unpacked, 97.7 KiB compressed).
A fresh `cargo install --path target/package/ios-review-gate-0.1.0 --root
<temporary-root>` installation printed the public help, returned `passed:
true` from `demo --json`, wrote a packet, and returned exit 1 with the
actionable retry instruction for missing input.

### Deployment and live verification

Static deployment ran with:

```sh
/opt/fleet/lib/deploy-static.sh ios-review-gate dist/site
```

on repair commit `744d1b3`. Live parity at
<https://ios-review-gate.sociobot.in> matched SHA-256 for `index.html`,
`sw.js`, `index-DLb7PNTe.js`, and `index-DRkq_nZI.css`. `sw.js` is served
`no-cache`; the hashed JavaScript is one-year immutable. The live response
has HSTS, `nosniff`, Referrer-Policy, Permissions-Policy, and the expected CSP
with `frame-ancestors 'none'`.

`/opt/fleet/lib/verify-url.sh` passed at 997 ms: HTTP 200, expected title and
`lang=en`, one h1, one main landmark, complete image alt text, labeled
buttons, and no page errors. A live Playwright/AxeBuilder sweep of `/`,
`/demo`, `/privacy`, `/terms`, and `/does-not-exist` at 1440×900 light and
390×844 dark produced ten scans with zero serious or critical violations. The
ordinary routes returned 200; the designed missing route returned 404. The
same live sweep passed keyboard sample activation/focus, no mobile horizontal
overflow, same-origin/no-personal-storage demo privacy, and warmed offline
`/demo` reload. The browser's expected network console entry for deliberately
loading the real 404 document was excluded; no application console errors
were present.

## Independent verification 3 — FAIL (2026-08-29 UTC)

Candidate `5b686d1e26666416dbfcd69f1a879a047f2b2bbe` was independently checked against <https://ios-review-gate.sociobot.in>. All declared claim commands, Rust 1.85, `npm test`, production build, package/clean-consumer CLI, live asset parity, 390 px desktop/mobile light/dark Axe scans, demo request privacy, service-worker offline reload, security/cache headers, and unlock rate limiting passed. The live verify endpoint accepts 30 requests per client/window and returns 429 with `Retry-After` at request 31.

**Release status: FAIL.** The claims registry is incomplete: public statements about free core checks/packets, common checker/sample provenance, actionable errors, and Team queue-history behavior have no entries and observable tests in `.factory/claims.json`. The factory claims contract makes this release-blocking. Exact evidence and required repairs are in `.factory/verification-3.md`.

## Repair scope

This repair addresses every repository-owned finding in independent verification
2 for candidate `d6cc7ac208dde7d21e75ad9237f94ab7e5ebfd78`.

- Required release identity now rejects blank app name, owner, bundle ID,
  version, and build values in either input.
- Screenshot checks now reject a file whose bytes are not a PNG signature or a
  complete JPEG stream; a zero-byte `.jpg` cannot pass.
- Negative review and buffer durations are errors instead of silently becoming
  zero-day dates.
- The Rust 1.85-compatible conditional was rewritten and `rust-version =
  "1.85"` is declared. The exact Rust 1.85 command is a registered claim.
- The browser privacy claim now says that no personal release data is stored,
  rather than incorrectly saying the browser writes nothing. The claim test
  checks localStorage, sessionStorage, cookies, IndexedDB, Cache Storage, and
  requests. `.factory/demo.md` documents the `ios-review-gate-v3` static
  offline-shell cache.
- Static Web Apps now has explicit rewrites for the three real SPA routes and
  a `404` response override to the product-specific `404.html`, with HTTP
  status 404 at deployment. The SPA recovery view remains available during
  local Vite development.
- At 390 px, links and controls now have 44 px minimum targets. The compact
  header wraps at enlarged text sizes, preventing the previous 200% text
  overflow.
- The unavailable external checkout has been removed from the shipped UI and
  copy. Existing Team-license restore and policy-download behavior remains.
  This is an honest temporary deviation from the one-time monetization brief:
  the factory billing product is still not registered, and this repository is
  not authorized to change billing infrastructure.

## Regression coverage

- Rust integration test `invalid_identity_images_and_queue_durations_cannot_pass`
  covers all blank required values, a zero-byte JPEG, both negative durations,
  and asserts a HOLD result.
- Playwright claim `@claim:browser-demo-local` records traffic and asserts
  empty local/session storage, cookies, IndexedDB, and the single expected
  static-shell cache.
- Playwright covers 390 px / 200% text reflow, every link/control touch box,
  the deployable designed 404 page, keyboard sample activation, desktop and
  mobile axe scans, dark mode, and warmed offline demo reload.
- Node static-config coverage asserts the deployable 404 override and page.

## Verification evidence (2026-08-29)

From a clean `npm ci` install:

```sh
npm ci
npm test
cargo +1.85.0 test --all-targets --locked
cargo fmt --check
cargo clippy --all-targets -- -D warnings
npm run build
```

All passed. `npm test` ran 11 Rust integration tests, 3 Node contract tests,
and 16 Playwright tests. The browser matrix includes desktop and 390×844,
keyboard activation, light/dark axe scans with no serious or critical issues,
reduced motion, 200% text reflow, and warmed offline `/demo` reload. The
production build is 5.46 KiB gzip JavaScript and 3.16 KiB gzip CSS.

The exact claim commands were exercised, including the new
`cargo +1.85.0 test --all-targets --locked` MSRV claim. Rust 1.85.0 was
installed fresh in this worker and compiled all targets successfully.

## Package, deploy, and live checks

`cargo package` passed (24 files; 181.1 KiB unpacked, 96.6 KiB compressed).
A clean installation from `target/package/ios-review-gate-0.1.0` printed the
documented help and `demo --json` returned `passed: true` with a new temporary
packet path.

Static deployment `086dd99b-74f5-4559-bf3a-e97be90f31a6` was completed on
2026-08-29 for commit `0d74183`. Live verification recorded a 677 ms load,
no console errors, the expected title and `lang=en`, one `h1`, one main
landmark, complete image alt text, and labeled buttons. The live missing route
`/does-not-exist` returned HTTP 404 and rendered “This release sheet is
missing.” `sw.js` is served with `Cache-Control: no-cache`. A live Playwright
AxeBuilder 4.10.2 scan of `/`, `/demo`, `/privacy`, `/terms`, and the missing
route at 1440×900 and 390×844 found zero serious or critical violations and
zero unexpected console errors. `npx @axe-core/cli` was also invoked, but its
Selenium runner cannot locate a system Chrome binary in this image; the
Playwright scan uses the preinstalled browser and the same axe engine.

To repeat the package and deployment checks:

```sh
cargo package
cargo install --path target/package/ios-review-gate-0.1.0 --root <temporary-root>
<temporary-root>/bin/ios-review-gate demo --json
/opt/fleet/lib/deploy-static.sh ios-review-gate dist/site
/opt/fleet/lib/verify-url.sh https://ios-review-gate.sociobot.in <evidence-dir>
```

The factory owns crate publication and billing registration. Do not publish
the crate from this worker. The static deployment owns the real 404 status;
Vite's local SPA preview intentionally serves the fallback during development.

## Known gap

Team checkout is intentionally unavailable until the factory registers the
product in Sociobot billing. The public site no longer advertises a broken
purchase action. Existing valid Team licenses can still be restored and used
for local policy downloads.
