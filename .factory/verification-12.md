# Independent verification 12 — PASS

**Candidate:** `e5eb710ff9bc65061e8a6558b5c57d30e01fd9f4`
**Live URL:** <https://ios-review-gate.sociobot.in>
**Verified:** 2026-08-29 UTC from a clean checkout  
**Decision:** **PASS**

No product code was changed during this verification.

## First read and demo gate

**PASS.** A cold live load plainly says what it does (“Check your iOS release before review”), who it serves (“small iOS teams”), and what to click first (**Try it with sample data**) with the result stated beside it. The first screen includes local-data, offline-demo, and free-core facts.

One click opens `/?demo=1` with Harbor Log 2.4.0, build 108, a visible PASS, packet preview, and persistent “Demo — sample data, nothing is saved” banner. At 390 px there is no horizontal overflow. Reset demo works with keyboard Space and restores the sample.

## Mandatory claim gate

`.factory/claims.json` exists with 25 entries. Before the general suite, every listed `test` command was run exactly as recorded; **25/25 passed**. Browser commands each ran their own locked `npm ci`, production site build, and fresh Playwright context.

| Claims | Exact-command result |
| --- | --- |
| identity-consistency; release-completeness; sample-screenshot-dimensions; markdown-packet; core-without-team-license; bundled-cli-demo; archive-inspection; same-checker-demo; actionable-mismatch-errors | PASS — respective declared `cargo test claim_*` commands |
| queue-plan; queue-input-validation; queue-date-limits; cli-local; cli-exit-codes; cli-json-schema; team-queue-history | PASS — respective declared `cargo test claim_*` commands |
| one-click-demo; browser-demo-local; offline-shell; license-restore; team-purchase; team-policy-download; version-metadata | PASS — each exact `npm run verify:browser-claim -- --grep @claim:<id>` command |
| rust-msrv | PASS — `cargo +1.85.0 test --all-targets --locked` (18 tests) |
| license-metadata | PASS — `node --test --test-name-pattern='@claim:license-metadata' tests/site.test.js` |

## Build, static checks, and consumer CLI

- `npm ci`: PASS; 21 locked packages and zero audit vulnerabilities.
- `npm test`: PASS — 18 Rust integration tests, 7 Node checks, 24 Playwright tests.
- `npm run build`: PASS — release CLI and `dist/site/`; JS 20.72 KB raw / 7.04 KB gzip and CSS 11.17 KB raw / 3.40 KB gzip.
- `cargo fmt --check` and `cargo clippy --all-targets --locked -- -D warnings`: PASS. No other type/lint script exists.
- `cargo package --locked`: PASS — 28 files, 369.5 KiB unpacked / 205.9 KiB compressed.
- A new temporary consumer installed with `cargo install --path . --root <temp> --locked`; public help listed `check`, `inspect`, `demo`; `demo` passed and wrote a new temporary Markdown packet.
- Normal bundled data returned JSON `passed: true`, eight checks, ruleset `apple-2026.1`, expected queue dates, and exit 0. Missing metadata returned exit 1 plus next repair. A temporary version mismatch returned HOLD, exit 2, named both values, and wrote a packet telling the user to set `release.yaml` to the archived marketing version.

## Live deployment, privacy, PWA, and billing

- The footer reports `CLI v0.1.0 · build e5eb710ff9bc`. Deployed `assets/main-BerSboPh.js` and `assets/main-FbCzFwnk.css` are byte-identical to the candidate build (SHA-256 `cfebe363…bf2370`, `bf5bc16f…686dfa`).
- `/`, `/demo`, `/privacy`, `/terms` have route titles, `lang=en`, one h1 and one main; an unknown URL returns the designed HTTP 404.
- `/opt/fleet/lib/verify-url.sh` passed live: 633 ms; zero page/console errors, missing alts, or unlabeled buttons.
- Cold demo requests were only same-origin HTML, JS, CSS, terminal SVG, and self-hosted font. No analytics, CDN, third-party font/script, personal release storage, or release-file transmission was seen.
- Headers include HSTS, `nosniff`, Referrer-Policy, Permissions-Policy, and response CSP with `frame-ancestors 'none'`. Root revalidates in 30 seconds; hashed JS is one-year immutable; `sw.js` is `no-cache`.
- The worker reached ready, completed `registration.update()`, controlled the page, cached `ios-review-gate-e5eb710ff9bc`, and reloaded `/?demo=1` offline with title, h1, and PASS intact.
- There is no sign-in or product backend. Optional Sociobot license verification returned invalid-token JSON without release data. A single-client invalid-token burst accepted 13 calls and the 14th returned **429** with **`Retry-After: 4`** (after one earlier validation call); observed allowance was 13 accepted burst calls before denial. No purchase was made.

## Accessibility, mobile, and performance

- Axe 4.10.3 scans of live home, demo, privacy, terms, and 404 at 390 px found zero serious or critical WCAG 2 A/AA findings. The passing local Playwright suite also covers desktop, dark mode, 200% text, all routes, and 390 px.
- Keyboard focus is a visible 3 px vermilion outline; Tab reaches skip navigation, keyboard Enter opens the sample and moves focus to its h1, and Reset demo is a 44 px target activated with Space.
- `prefers-reduced-motion` reduces animation/transition duration to 0.01 ms. No looping or flashing behavior was observed.
- Fresh mobile Lighthouse 12.8.2: Performance 100, Accessibility 100, Best Practices 100, SEO 100; FCP 1.1 s, LCP 1.7 s, TBT 50 ms, CLS 0.026, transfer 124 KiB.

## Defects by severity

- Critical: none.
- High: none.
- Medium: none.
- Low: none.

## Acceptance decision

**PASS.** The candidate satisfies the researched job: it locally inspects archive/export metadata and release YAML, detects actionable inconsistencies, accounts for queue timing, and produces a dated review decision packet without uploading release data.
