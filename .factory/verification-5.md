# Independent verification 5 — FAIL

**Candidate:** `8e47afe2a31a563446b0dc38de523ff33fc1a5f0`

**Live URL:** <https://ios-review-gate.sociobot.in>

**Verified:** 2026-08-29 UTC from the supplied clean checkout

**Result:** **FAIL — the required one-time Team purchase is still unavailable, and one mobile link misses the contracted 44×44 px touch target.**

No product code was changed during this verification. The earlier builder report
described billing as a deployment-only blocker; fresh production evidence below
confirms that the blocker remains. Repository-owned CLI defects from verification
4 are fixed.

## Mandatory first-read and demo gate

**PASS.** A cold 1440×900 load answers the required questions in plain words:

- What it does: “Check your iOS release before review.”
- Who it is for: “For small iOS teams that need one reviewable packet before they queue a build.”
- What to click first: **Try it with sample data**, beside “See a checked release and its packet.”

At 390×844, the action is visible in the first viewport at y=637.9–686.3 px.
One click opens `/demo` and immediately shows Harbor Log 2.4.0, build 108,
locale `en-US`, Privacy Manifest present, a PASS decision, and the Markdown
packet preview. The persistent demo banner provides **Reset demo** and **Start
for real**. The first-read gate therefore does not cause the failure.

## Required claims gate

`.factory/claims.json` exists with 17 entries. After `npm ci`, every exact
listed command passed. Each command was run separately from this candidate;
none was substituted with the aggregate suite.

| Claim | Exact command | Result |
| --- | --- | --- |
| `identity-consistency` | `cargo test claim_identity_consistency` | PASS |
| `release-completeness` | `cargo test claim_release_completeness` | PASS |
| `markdown-packet` | `cargo test claim_markdown_packet` | PASS |
| `core-without-team-license` | `cargo test claim_core_gate_runs_without_team_license_and_writes_packet` | PASS |
| `bundled-cli-demo` | `cargo test claim_bundled_demo` | PASS |
| `same-checker-demo` | `cargo test claim_demo_recording_matches_bundled_cli` | PASS |
| `actionable-mismatch-errors` | `cargo test claim_actionable_mismatch_error_names_values_and_fix` | PASS |
| `queue-plan` | `cargo test claim_queue_plan` | PASS |
| `queue-input-validation` | `cargo test claim_queue_input_validation_rejects_incomplete_or_unknown_entries` | PASS |
| `queue-date-limits` | `cargo test claim_queue_date_limits_hold_without_panicking` | PASS |
| `cli-local` | `cargo test claim_cli_local` | PASS |
| `one-click-demo` | `npm run build:site && npx playwright test --grep @claim:one-click-demo` | PASS — 1 browser test |
| `browser-demo-local` | `npm run build:site && npx playwright test --grep @claim:browser-demo-local` | PASS — 1 browser test |
| `rust-msrv` | `cargo +1.85.0 test --all-targets --locked` | PASS — 16 integration tests |
| `license-restore` | `npm run build:site && npx playwright test --grep @claim:license-restore` | PASS — 1 recorded-verdict browser test |
| `team-policy-download` | `npm run build:site && npx playwright test --grep @claim:team-policy-download` | PASS — 1 cached-verdict/download browser test |
| `team-queue-history` | `cargo test claim_team_policy_supports_queue_history_beyond_three_submissions` | PASS |

The public landing, privacy, terms, and README promises map to these registered
claims. The mocked license tests do not prove that a new customer can purchase
a production license; the live checkout test below shows that they cannot.

## Clean local gates and exact build

- `npm ci`: PASS — 21 packages installed; audit reported 0 vulnerabilities.
- `npm test`: PASS — 16 Rust integration tests, 4 Node contract tests, and 18
  Playwright tests.
- `cargo fmt --check`: PASS.
- `cargo clippy --all-targets -- -D warnings`: PASS.
- `npm run build`: PASS — exact release binary and `dist/site/` produced.
- `cargo package`: PASS — 24 files; 194.4 KiB unpacked and 99.1 KiB compressed.
- There is no separate TypeScript or lint script. Rust formatting and clippy
  are the available static checks.

Production output is well inside the supplied budgets: JavaScript is 14,359
bytes (5,499 bytes gzip), CSS is 10,298 bytes (3,239 bytes gzip), the font is
20,056 bytes, and the hero image is 94,064 bytes. The release binary is
1,749,856 bytes.

The required docs are present: product/audience/run/test/deploy instructions in
`README.md`, MIT `LICENSE`, `CHANGELOG.md`, `/privacy`, `/terms`, the product-
specific visual thesis and asset provenance in `.factory/design.md`, and demo
isolation instructions in `.factory/demo.md`.

## Clean-consumer CLI and end-to-end behavior

The verified Cargo package, rather than the repository binary, was installed
into a new temporary root:

```sh
cargo install --path target/package/ios-review-gate-0.1.0 --root <temp-root>
```

The installed binary exposes useful top-level and `check` help. Independent
cases produced these results:

| Case | Exit | Observable result |
| --- | ---: | --- |
| `demo --json` | 0 | `passed:true`; eight checks; fresh temp workspace and packet exist |
| Normal copied sample | 0 | PASS packet written; decision 2026-09-04, buffered 2026-09-06 |
| Version 2.4.0 vs 2.5.0 | 2 | `identity.version` names both values and the repair |
| Malformed metadata JSON | 1 | Parse location plus “Fix the path or file contents…” |
| Missing metadata file | 1 | Missing path plus the same recovery action |
| Missing output directory | 1 | Write failure plus the same recovery action |
| Blank queue identity + unknown status | 2 | Three errors; unknown entry conservatively counted active; dates delayed |
| Maximum i64 review days | 2 | `queue.review_days_out_of_range`; no panic |
| Minimum i64 review days | 2 | `queue.review_days_negative`; no panic |
| Zero review and buffer days | 0 | All three queue dates equal the intended date |
| JPEG extension with invalid bytes | 2 | `screenshots.invalid_image` |

The bundled demo completed in approximately 11 ms, far below the brief's
five-minute target. The claim fixtures catch all three seeded identity
mismatches and all eight seeded release-completeness inconsistencies. The CLI
source and dependency manifest contain no network client or telemetry path.

## Candidate/live identity and routing

The live deployment matches this candidate's exact production build. SHA-256
values matched pairwise for local and live files:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `88e8b1969c01085697fb291f48b38ee3b23e90418a2d98845af320b02c17b905` |
| `index-BJuiNygW.js` | `7e97930f6f39a777c5afbddeeea3cf561358a628ecd751d565326c56a8802e8d` |
| `index-DdXL0tE_.css` | `89cb71e82e231d7c76a02ffa004e3e5a9f2318b1ea7252cd845d8b7d83d67c19` |
| `sw.js` | `74b2d05f82b66a566839a62086d3acdbc924e3b39bf215ec500b649c95cc7e12` |

`/`, `/demo`, `/privacy`, and `/terms` return 200. A missing route returns the
designed 404 with HTTP 404. Every HTTP link discovered across those routes
returned 200 after redirects; the `mailto:` privacy link was excluded. HTTP
redirects to HTTPS with 301. `robots.txt` and `sitemap.xml` are present and the
sitemap lists all four public routes.

## Browser, accessibility, privacy, and resilience

- `/opt/fleet/lib/verify-url.sh` passed: 654 ms load, correct title and
  `lang=en`, one h1, one main landmark, complete image alt text, labeled
  buttons, and no application console/page errors.
- Independent AxeBuilder 4.10.2 scans covered `/`, `/demo`, `/privacy`,
  `/terms`, and the real 404 at 1440×900 light and 390×844 dark. All ten scans
  had zero serious or critical findings. The expected browser network message
  for loading an HTTP 404 document is not an application defect.
- Keyboard activation of the primary action opened `/demo` and moved focus to
  its h1. Reset operated with Space. The primary focus indicator is a 3 px
  vermilion outline with 3 px offset; its measured contrast is 5.94:1 in light
  mode and at least 6.42:1 in dark mode.
- Reduced motion changed the inspection animation to 0.00001 s and one
  iteration. Nothing loops.
- At 390 px and 200% root text, the demo remained exactly 390 px wide; tested
  content boxes stayed from x=20 to x=370 and `main` did not clip overflow.
- A fresh landing → demo → reset flow made only same-origin requests. The demo
  left localStorage, sessionStorage, cookies, and IndexedDB empty. Cache
  Storage contained only the documented `ios-review-gate-v3` static shell.
- The service worker registered and `update()` completed. After warm-up, an
  offline `/demo` reload retained the title, h1, Harbor Log sample, and active
  controller without a console/page error.
- A keyboard-submitted invalid license made exactly one documented GET to the
  Sociobot verification endpoint, sent no release data, showed “License not
  active…” with a next step, and caused no browser error.

The one mobile touch-target failure is recorded separately below. There is no
sign-in, product backend, analytics, runtime AI feature, or third-party font or
script. Entra identity, backend concurrency/persistence, and AI gateway checks
are not applicable.

## Headers, caching, and performance

The browser and direct response logs agree. HTML has HSTS, CSP,
`X-Content-Type-Options: nosniff`, `Referrer-Policy`, and `Permissions-Policy`.
The CSP limits scripts/styles/fonts to self, permits connections only to self
and the Sociobot API, and sends `frame-ancestors 'none'` as a response header.
Hashed JS/CSS use `public, max-age=31536000, immutable`; `sw.js` uses
`no-cache`; HTML and unhashed assets use a 30-second revalidation policy.

Lighthouse 12.8.2 mobile completed without a runtime error:

- Performance 99
- Accessibility 100
- Best Practices 100
- SEO 100
- FCP 1.1 s; LCP 1.6 s; TBT 70 ms; CLS 0; total transfer 123 KiB

These results meet the supplied budgets. INP is not available from a synthetic
single-load Lighthouse run; the interaction path had no long tasks or visible
delay during Playwright use.

## Product-unlock API

Rate limiting passes. After a clean window, sequential invalid-license
requests 1–30 returned 200. Request 31 returned **429** with both
`Retry-After: 3` and `X-RateLimit-After: 3`. **Observed allowance: 30 requests
per client/window.**

## Release-blocking defects

### High — a new customer cannot buy the contracted one-time Team license

Fresh production evidence at 2026-08-29 13:02 UTC:

```text
GET https://api.sociobot.in/api/v1/products/ios-review-gate/checkout
HTTP 404
{"error":"enabled factory product","status":404}
```

The researched brief requires one-time monetization, and the paid-unlock
contract requires an exact price and hosted Sociobot checkout link. The live
site instead says “Team checkout is not available right now,” gives no price,
and offers only license restoration. Consequently purchase redirect,
return-token handling after a real purchase, and refund/merchant flow cannot be
verified end to end. Mocked restore/download tests do not repair this live
gap. Factory ownership of registration does not change the release result.

**Required repair:** register and enable `ios-review-gate` in the Sociobot
billing engine at the intended one-time price and return URL, expose the hosted
checkout action and exact price/refund terms, then independently exercise a
real test purchase and returned token before release.

### Medium — the inline mobile terms link is narrower than 44 px

At 390×844 with default text size, the lower-case **terms** link inside the
home page's Team-policy legal note measures **37.97×44 px**. Every other
interactive target measured at least 44 px in both dimensions across `/`,
`/demo`, `/privacy`, and `/terms`. This violates the attached accessibility
and design contracts' explicit 44×44 px touch-target baseline. Axe reports no
serious violation because its automated rules do not enforce this stricter
factory threshold. The existing regression checks targets only after forcing
200% text, where the word becomes wide enough, so it misses the default-size
failure.

**Required repair:** give short inline links a minimum 44 px inline size (or
equivalent padding without harming sentence layout), and add a 390 px
default-text target-size regression across every route.

## Acceptance decision

**FAIL.** The mandatory first-read and all 17 claim tests pass. The core CLI,
package, repaired boundary handling, deployment parity, privacy, rate limit,
keyboard flow, serious/critical axe scan, offline behavior, security headers,
and performance also pass. Release remains blocked because the required
one-time purchase path is unavailable in production. The 37.97 px mobile link
must also be brought up to the explicit 44 px baseline before acceptance.
