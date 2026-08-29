# Handoff — iOS Review Gate 0.1.0 repair

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
