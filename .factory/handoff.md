# Handoff — iOS Review Gate 0.1.0 repair

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
- Static Web Apps now has a `404` response override to the product-specific
  `404.html`, with HTTP status 404 at deployment. The SPA recovery view remains
  available during local Vite development.
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

After this handoff is committed, run:

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
