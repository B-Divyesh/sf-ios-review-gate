# Independent verification — FAIL

**Candidate:** `a3612cbdd3ca02b29dac0b111c2a52029e143d4a`  
**Live URL:** <https://ios-review-gate.sociobot.in>  
**Verified:** 2026-08-28 (fresh clone, no product-code changes)  
**Result:** **FAIL — release-blocking defects remain.**

## First-read and demo result

Cold-reading the live landing page answers all required questions in plain
words: it checks an iOS release before App Review; it is for small iOS teams;
and the first action is **Try it with sample data**, which says it will show a
checked release and its packet. One click opens `/demo`, showing the Harbor Log
2.4.0 / build 108 sample, a PASS result, a sample-data banner, Reset demo, and
Start for real. This requirement **passes**.

## Required claims — all passed locally

`npm ci` was run first. Every exact command in `.factory/claims.json` was then
run from this clean checkout. The CLI claims use bundled `demo` data; browser
claims use the local demo entry point.

| Claim | Exact test | Result / observable evidence |
| --- | --- | --- |
| identity-consistency | `cargo test claim_identity_consistency` | PASS — three seeded version/build/bundle-ID mismatches reported |
| release-completeness | `cargo test claim_release_completeness` | PASS — seeded privacy, localization, screenshot, and reason-code findings reported |
| markdown-packet | `cargo test claim_markdown_packet` | PASS — dated packet contains decision, rules, queue, and decision record |
| bundled-cli-demo | `cargo test claim_bundled_demo` | PASS — new temp workspace and packet path asserted |
| queue-plan | `cargo test claim_queue_plan` | PASS — active submission plus review/buffer dates asserted |
| cli-local | `cargo test claim_cli_local` | PASS — source/manifest have no network client marker |
| one-click-demo | `npm run build:site && npx playwright test --grep @claim:one-click-demo` | PASS — 1 Playwright test; one click reaches complete Harbor Log sample |
| browser-demo-local | `npm run build:site && npx playwright test --grep @claim:browser-demo-local` | PASS — 1 Playwright test; no cross-origin request |
| license-restore | `npm run build:site && npx playwright test --grep @claim:license-restore` | PASS — 1 Playwright test with recorded verification response |
| team-policy-download | `npm run build:site && npx playwright test --grep @claim:team-policy-download` | PASS — 1 Playwright test; downloaded YAML contains selected values |

The passing license claims mock the billing API; they do not prove the deployed
checkout is registered. Live testing below shows that it is not.

## Local build, CLI, and end-to-end checks

- `npm test`: PASS — 10 Rust integration tests, 3 Node contract tests, and 11
  Playwright tests.
- `cargo fmt --check`: PASS.
- `cargo clippy --all-targets -- -D warnings`: PASS.
- `npm run build`: PASS. Production assets: JS 14,124 bytes (5.42 KB gzip),
  CSS 9,581 bytes (3.05 KB gzip); both are below budget.
- `cargo package`: PASS — package verified; 24 files, 172.4 KB (94.9 KB
  compressed).
- Clean consumer check: `cargo install --path /work/repo --root <mktemp>`
  installed the public binary. `--help`, `demo --json`, and missing-file
  recovery were exercised. Demo returned `passed: true` and a packet path;
  invalid paths returned exit code 1 and the actionable retry instruction.
- Normal and failure paths are additionally covered by the production test
  suite: a passing sample, seeded identity/release errors (exit 2), invalid
  input (exit 1), JSON output, and Team policy limits/reason codes.

## Live deployment checks

- The live page was cold-opened at desktop and at 390×844. No page or console
  errors occurred; mobile had no horizontal page overflow. The demo’s requests
  were only same-origin assets.
- `/opt/fleet/lib/verify-url.sh https://ios-review-gate.sociobot.in <temp>`:
  PASS — HTTP 200, title present, `lang=en`, one h1, main landmark, no missing
  image alt text, no unlabeled button, and no console error (2.375 s load in
  that run).
- Built/live parity: SHA-256 values of deployed `index-r8gbPusK.js`,
  `index--NtfzEGb.css`, and `sw.js` exactly equal this candidate’s `dist/site`
  artifacts. The live footer identifies `v0.1.0 · build 2026.08.28`.
- Security and cache policy: HTTPS, HSTS, CSP, `nosniff`, Referrer-Policy, and
  Permissions-Policy are present. Hashed JS/CSS use
  `public, max-age=31536000, immutable`; `sw.js` uses `no-cache`.
- Privacy: the CLI has no network client; landing/demo loads made no
  cross-origin request. An explicit invalid-license verification made only the
  documented Sociobot API request and produced a recoverable “License not
  active” message. No sign-in flow exists.
- Service worker: registered successfully; after an online warm-up, `/demo`
  reloaded offline with its sample content and no console error. There is no
  web manifest, so this is not presented as an installable PWA.
- Lighthouse mobile run against live `/`: Performance 100, Accessibility 100,
  Best Practices 100, SEO 100; LCP 1.5 s, CLS 0.028, TBT 90 ms. This landing
  audit does not cover the dark `/demo` state below.

## Release-blocking defects

### High — advertised paid checkout is not registered in production

**Evidence:** `GET https://api.sociobot.in/api/v1/products/ios-review-gate/checkout`
returned HTTP **404** with `{"error":"enabled factory product","status":404}` on
2026-08-28. The live page advertises “Buy Team license” for a $39 one-time
purchase and points directly to that endpoint.

**Impact:** a user cannot buy the stated Team license. This is a live-only
failure; the repository test intercepts only the verify response and does not
exercise checkout.

**Required fix:** register/enable the `ios-review-gate` Sociobot product with
the correct return URL, then repeat a real checkout-link smoke test.

### High — product-unlock verification has no observed rate limit

**Evidence:** a burst of 30 concurrent GET requests to
`/api/v1/products/ios-review-gate/verify?license=qa-rate-limit-token` returned
**30 × HTTP 200**. None returned HTTP 429 and no `Retry-After` header was
observed. Threshold observed: **none through 30 concurrent requests**.

**Impact:** the required server-side/product-unlock rate-limit behavior is
absent from the live endpoint.

**Required fix:** enforce an appropriate per-IP/token limit on the verification
endpoint and return HTTP 429 with `Retry-After`; document and retest the
observed threshold.

### High — mobile code blocks fail axe `scrollable-region-focusable`

**Evidence:** independent axe 4.10.2 runs at 390×844 report a **serious**
`scrollable-region-focusable` violation on the landing command `<pre>` and on
the demo packet `<pre>` (in both light and dark treatment). Their horizontal
overflow is usable with a pointer but the region has neither focus nor
focusable content for keyboard users.

**Required fix:** make each scrollable code region keyboard focusable (for
example, an appropriate `tabindex="0"` and accessible label) or eliminate the
overflow. Add mobile axe coverage to the test suite.

### High — dark demo banner has serious contrast failures

**Evidence:** independent axe 4.10.2 at 390×844, dark scheme, `/demo` reports
**serious** `color-contrast` failures for “Demo — sample data, nothing is
saved”, Reset demo, and Start for real. It measured white (`#fff`) on the dark
mode banner background (`#f5f0df`) at **1.14:1**, below the required 4.5:1.

**Required fix:** define contrasting banner foreground/background tokens for
dark mode and add a dark `/demo` axe assertion.

## Non-blocking observations

- The local Playwright axe test passes because it runs the landing page at its
  default desktop viewport and only dark-tests `/`; it misses the mobile
  horizontally-scrollable code samples and dark demo banner.
- Live route content, titles, one-h1 structure, keyboard activation of the
  sample action, visible focus styling, reduced motion, legal pages, and
  same-origin demo behavior otherwise passed the exercised checks.

## Acceptance decision

**FAIL.** Do not release or mark this candidate accepted until the paid endpoint
is registered, the unlock API rate limits, and the serious mobile/dark
accessibility defects are fixed and independently retested.
