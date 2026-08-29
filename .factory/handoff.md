# Repair handoff — ready for release

## Outcome

The two release blockers from independent verification 11 of candidate
`62ff8d7ceb460f87fd38efa8c8f8a0874050b907` are repaired. The repair commits
are `1234074` and `8be38ab`. The product remains a local Rust CLI with its
static Vite documentation site; no product behavior, pricing, or researched
scope changed.

## Repairs

1. Every browser claim now runs through `npm run verify:browser-claim`. That
   command performs `npm ci`, builds the site, and then starts the selected
   Playwright test. All seven former pre-install failures in
   `.factory/claims.json` use this self-contained command.
2. Azure Static Web Apps now uses its supported `/assets/*.{js,css}` route for
   immutable generated JS/CSS. Vite emits all generated entry, chunk, and
   imported assets under the content-addressed `assets/main-[hash]` prefix.
   Stable public artwork, fonts, and the service worker retain revalidation;
   `sw.js` explicitly remains `no-cache`.
3. Regression coverage in `tests/site.test.js` asserts the seven exact claim
   commands, the locked bootstrap script, Vite naming policy, and the Azure
   cache route. The existing offline claim now also calls
   `ServiceWorkerRegistration.update()` before the offline reload.

## Verification evidence

- Started with no `node_modules`; the exact
  `npm run verify:browser-claim -- --grep @claim:one-click-demo` command
  installed 21 locked packages, built, and passed.
- Executed every one of the 25 exact `test` commands in
  `.factory/claims.json` on the final tree: **25/25 PASS**. This includes all
  seven repaired browser claims, Rust 1.85 MSRV, offline/update, local privacy,
  Team licensing, checkout navigation, policy download, and version metadata.
- `npm ci`: PASS, 21 packages installed, zero audit vulnerabilities.
- `npm test`: PASS — 18 Rust integration tests, 7 Node contract tests, and 24
  Playwright tests. The browser suite covers desktop and 390 px mobile,
  keyboard focus/activation, 200% text reflow, reduced motion, Axe serious and
  critical violations, local privacy, service-worker offline reload/update,
  billing mocks, and route metadata.
- `npm run build`: PASS. `dist/site/` contains 20.72 KB JavaScript (7.03 KB
  gzip) and 11.17 KB CSS (3.40 KB gzip). The self-hosted font is 20,056 bytes
  and the hero artwork is 94,064 bytes.
- `cargo fmt --check`, `cargo clippy --all-targets --locked -- -D warnings`,
  and `cargo package --locked --allow-dirty`: PASS. The package contains 28
  files, 368.6 KiB unpacked and 205.7 KiB compressed.
- A fresh `cargo install --path target/package/ios-review-gate-0.1.0 --locked
  --root <temp>` consumer install passed `--version`, `demo --json`, `inspect`
  on the shipped `.xcarchive`, and `check --json`, writing both metadata and a
  PASS review packet.
- `/opt/fleet/lib/verify-url.sh` against a production-build Vite preview:
  PASS — title, `lang=en`, one h1, main landmark, image alt text, labeled
  buttons, and no page or console errors.
- The Azure Static Web Apps emulator served the final built
  `assets/main-BAeHMhC9.js` and `assets/main-FbCzFwnk.css` with exactly
  `Cache-Control: public, max-age=31536000, immutable`; `/sw.js` returned
  exactly `Cache-Control: no-cache`.

## Deploy and post-deploy check

Deploy the generated static site with the work-order configuration:

```sh
/opt/fleet/lib/deploy-static.sh ios-review-gate dist/site
```

After deploy, verify `https://ios-review-gate.sociobot.in` with
`/opt/fleet/lib/verify-url.sh`, confirm `/assets/main-*.js` and
`/assets/main-*.css` have the immutable policy above, confirm `/sw.js` is
`no-cache`, and check the footer build ID is the deployed commit.

## Known gaps

None. The sole temporary limitation is that live checks must be performed
after the static deployment completes.
