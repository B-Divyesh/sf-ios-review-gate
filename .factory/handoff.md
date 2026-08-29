# Handoff — iOS Review Gate polish round 1

## Independent verification 10

**PASS** for candidate `0408e389692712681465ddc9940d124bb48e1f3d` at
<https://ios-review-gate.sociobot.in>. The fresh verifier report is
[`verification-10.md`](verification-10.md). It recorded clean-install,
all-claims, unit/integration/e2e, Rust 1.85, format/clippy, production build,
packed-consumer, live parity, privacy, accessibility, offline/service-worker,
header/cache, checkout, and 30-request rate-limit evidence. No release
defects remain; product code was not changed during verification.

## Outcome

All 22 findings from adversarial review 1 are resolved and deployed at <https://ios-review-gate.sociobot.in>. The product remains a Rust CLI with a static Vite landing/docs site and retains the blueprint drafting-sheet identity.

Implementation commit: `c0b41ec1e46835f842a120018a21e04360070c33`. Azure Static Web Apps deployment ID: `2ca9e981-214f-4d56-b814-958f03401a07`.

## What changed

- Added direct, local `.xcarchive` and `.ipa` inspection. `inspect --archive` emits JSON; `check --archive` checks the extracted data without an intermediate file.
- Added a shipped Harbor Log `.xcarchive` fixture. Tests assemble and inspect the equivalent IPA.
- Added direct `/?demo=1` rendering with a persistent sample-data banner, Reset demo, Install the CLI, separate behavior from stored real data, and offline reload.
- Expanded `.factory/claims.json` to 23 claims, including archive import, cache scope, and MIT consistency.
- Rewrote first-screen, README, packet, demo, legal, and 404 language around “check,” “checker,” and “Markdown review packet.” Removed every unproved merchant/refund/publication/build-host promise.
- Completed the standalone 404 metadata and shared header/footer contract.
- Tightened desktop/mobile hero spacing so the action and all three facts fit the first viewport.
- Updated the service worker to cache only the same-origin static shell and serve the query demo offline.
- Updated catalog copy, copy audit, demo documentation, changelog, and the full finding map in `.factory/polish-1.md`.

## Verification evidence

Clean clone: `/tmp/ios-review-gate-clean.Iony6n` at `c0b41ec`.

- `npm ci`: PASS; 21 packages, 0 vulnerabilities.
- Every exact command in all 23 `.factory/claims.json` entries: PASS individually; final marker `ALL_CLAIMS_PASS`.
- `npm test`: PASS — 17 Rust integration tests, 5 Node contract tests, 23 Playwright tests.
- `npm run build`: PASS — release CLI and `dist/site/` produced.
- `cargo +1.85.0 test --all-targets --locked`: PASS — 17 integration tests.
- `cargo fmt --check`: PASS.
- `cargo clippy --all-targets --locked -- -D warnings`: PASS.
- `cargo package --locked`: PASS — 27 files, 360.0 KiB unpacked, 203.3 KiB compressed.
- Packaged/direct CLI smoke: `.xcarchive` inspection emitted Harbor Log identity/privacy JSON; direct archive check returned PASS with eight checks.

Browser, accessibility, privacy, and offline:

- `/opt/fleet/lib/verify-url.sh` live: PASS in 685 ms; 200, correct title/lang, one h1/main, complete alt text, labeled buttons, no console/page errors.
- Axe CLI 4.10.3 live: zero violations on `/`, `/?demo=1`, `/privacy`, and `/terms`.
- Playwright axe: zero serious/critical issues across route, dark, and mobile matrices.
- Keyboard navigation, visible 3 px focus, route h1 focus, 44 px controls, 200% text reflow, and reduced motion: PASS.
- Seeded real release, license, and draft storage stayed unchanged through demo/reset. All demo requests were same-origin.
- Live cache `ios-review-gate-v6`: 11 allowed same-origin static responses. `/?demo=1` reloaded offline with the sample visible.
- Desktop facts ended at 715.9, 744.3, and 772.6 px inside 1440×900. Mobile facts ended at 826.1 px inside 390×844. No horizontal overflow.

Performance and deployment:

- Live Lighthouse mobile: Performance 100, Accessibility 100, Best Practices 100, SEO 100.
- Live FCP 1.1 s, LCP 1.6 s, TBT 30 ms, CLS 0.026.
- Production JS 14.75 KiB raw / 5.49 KiB gzip; CSS 10.42 KiB raw / 3.25 KiB gzip; font 20,056 bytes; hero WebP 94,064 bytes.
- `/`, `/demo`, `/?demo=1`, `/privacy`, `/terms`, `robots.txt`, and `sitemap.xml`: HTTP 200.
- Random unknown live URL: HTTP 404 with designed metadata-complete shell.
- Live checkout: HTTP 303 to the hosted checkout.
- Live/local SHA-256 matched for `index.html`, hashed JS, hashed CSS, `sw.js`, and `404.html`.
- Response headers include HSTS, CSP with header-delivered `frame-ancestors 'none'`, nosniff, Referrer-Policy, and Permissions-Policy.

Screenshots:

- `.factory/evidence/polish-1-live-home.png`
- `.factory/evidence/polish-1-live-demo-mobile.png`
- `.factory/evidence/polish-1-live-404.png`

## Run and verify

```sh
npm ci
npm test
npm run build
cargo +1.85.0 test --all-targets --locked
cargo clippy --all-targets --locked -- -D warnings
cargo package --locked
```

Try the direct importer:

```sh
cargo run -- inspect --archive examples/archive/HarborLog.xcarchive
cargo run -- check \
  --archive examples/archive/HarborLog.xcarchive \
  --release examples/sample/release.yaml \
  --output review-packet.md
```

Deploy command used:

```sh
npm ci && npm run build:site
/opt/fleet/lib/deploy-static.sh ios-review-gate dist/site
```

## Known gaps and next steps

No reviewed defect or acceptance item remains. Registry publication is intentionally left to the factory owner, as required by the CLI publishing contract.
