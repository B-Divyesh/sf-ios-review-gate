# Independent verification 4 — FAIL

**Candidate:** `012cf19749fb02fd61c9d6686ddb2c7d193694f6`

**Live URL:** <https://ios-review-gate.sociobot.in>

**Verified:** 2026-08-29 UTC from the supplied clean checkout

**Result:** **FAIL — the queue gate can issue a false PASS, invalid boundary input can crash the CLI, and the contracted Team purchase remains unavailable.**

No product code was changed during this verification.

## Mandatory first-read and demo gate

**PASS.** A cold 1440×900 load answers all three questions in plain words:

- What it does: “Check your iOS release before review.”
- Who it is for: “For small iOS teams that need one reviewable packet before they queue a build.”
- What to click first: **Try it with sample data**, beside “See a checked release and its packet.”

At 390×844, the heading, audience sentence, primary action, outcome, and three
facts all fit in the first viewport. The action occupies y=638–686 px. One
click opens `/demo` and immediately shows Harbor Log 2.4.0, build 108, locale
`en-US`, Privacy Manifest present, a PASS result, and a Markdown packet. The
page also shows Reset demo and Start for real. Cold-read screenshots were
captured at `/tmp/ios-review-gate-first-read.png`,
`/tmp/ios-review-gate-demo-click.png`, and
`/tmp/ios-review-gate-mobile.png` in the verifier workspace.

## Required claims gate

`.factory/claims.json` exists with 15 entries. After the required `npm ci`,
every exact listed command passed against the bundled CLI demo or local browser
demo. No claim assertion failed.

| Claim | Exact command | Result |
| --- | --- | --- |
| `identity-consistency` | `cargo test claim_identity_consistency` | PASS — one matching seeded test |
| `release-completeness` | `cargo test claim_release_completeness` | PASS — one matching seeded test |
| `markdown-packet` | `cargo test claim_markdown_packet` | PASS — dated packet asserted |
| `core-without-team-license` | `cargo test claim_core_gate_runs_without_team_license_and_writes_packet` | PASS — eight checks and packet asserted |
| `bundled-cli-demo` | `cargo test claim_bundled_demo` | PASS — fresh temporary workspace and packet asserted |
| `same-checker-demo` | `cargo test claim_demo_recording_matches_bundled_cli` | PASS — shipped inputs, CLI result, and recording markers asserted |
| `actionable-mismatch-errors` | `cargo test claim_actionable_mismatch_error_names_values_and_fix` | PASS — values and repair instruction asserted |
| `queue-plan` | `cargo test claim_queue_plan` | PASS — active submission and buffer dates asserted |
| `cli-local` | `cargo test claim_cli_local` | PASS — source/manifest network-client guard |
| `one-click-demo` | `npm run build:site && npx playwright test --grep @claim:one-click-demo` | PASS — 1 Playwright test |
| `browser-demo-local` | `npm run build:site && npx playwright test --grep @claim:browser-demo-local` | PASS — 1 Playwright request/storage test |
| `rust-msrv` | `cargo +1.85.0 test --all-targets --locked` | PASS — all 14 integration tests on Rust 1.85 |
| `license-restore` | `npm run build:site && npx playwright test --grep @claim:license-restore` | PASS — 1 recorded-verdict test |
| `team-policy-download` | `npm run build:site && npx playwright test --grep @claim:team-policy-download` | PASS — 1 cached-verdict/download test |
| `team-queue-history` | `cargo test claim_team_policy_supports_queue_history_beyond_three_submissions` | PASS — default and Team limits asserted |

The landing page and README claims map to the registry entries above. The
passing declared checks do not cover the independent invalid queue-status and
integer-boundary cases described below.

## Clean local gates and package

- `npm ci`: PASS — 21 packages installed, 0 reported vulnerabilities.
- `npm test`: PASS — 14 Rust integration tests, 4 Node contract tests, and 16
  Playwright tests.
- `cargo fmt --check`: PASS.
- `cargo clippy --all-targets -- -D warnings`: PASS.
- `npm run build`: PASS — exact release binary plus `dist/site/`.
- `cargo package`: PASS — 24 files, 187.4 KiB unpacked and 97.7 KiB compressed.
- There is no separate TypeScript or lint script. Rust formatting and clippy
  are the available static checks.
- Production assets are 14,317-byte JavaScript (5.47 KiB gzip), 10,085-byte
  CSS (3.16 KiB gzip), 20,056-byte font, and 94,064-byte hero image. The
  release binary is 1,743,352 bytes. These are below the supplied budgets.

A clean consumer installation was built from Cargo's verified packaged tree,
not from the repository binary:

```sh
cargo install --path target/package/ios-review-gate-0.1.0 --root <temp-root>
```

The installed `ios-review-gate 0.1.0` had useful top-level and `check` help.
`demo --json` returned exit 0, `passed:true`, and a packet in a new temporary
workspace. A copied normal sample returned exit 0 and wrote a dated PASS
packet. A version mismatch returned exit 2 with both values and the repair
instruction. A missing file, malformed JSON, and unwritable output path each
returned exit 1 with an actionable retry sentence. Zero review/buffer days
passed with all queue dates equal to the intended date; a recovered active
submission produced the expected four-day estimated decision and six-day
buffered decision.

## Live deployment, privacy, accessibility, and performance

### Candidate identity and routing

The live deployment matches this candidate. Live and local SHA-256 values were
identical for:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `6a1d61c0f9d4094691d19a639eeee790198a85ebe082487ce6a15451590c2187` |
| `index-DLb7PNTe.js` | `748f139b5638e5d54a35ac4e554072fca1afdcb408ce019f4bcb4d8e2f956be1` |
| `index-DRkq_nZI.css` | `2fb3487af4832e71892456e65bb137b996b1aa0cb172c76736d6c4e0361e1065` |
| `sw.js` | `74b2d05f82b66a566839a62086d3acdbc924e3b39bf215ec500b649c95cc7e12` |

`/`, `/demo`, `/privacy`, and `/terms` returned 200 with route-specific
titles. A missing path returned a designed page with HTTP 404. Every internal
and external HTTP link found across the four routes returned 200; the privacy
mailto link was excluded. HTTP redirects to HTTPS.

### Browser, accessibility, and resilience

- `/opt/fleet/lib/verify-url.sh` passed after creating its required output
  directory: load 821 ms, title and `lang=en`, one h1, one main landmark, all
  image alts present, no unlabeled buttons, and no console/page errors.
- Independent AxeBuilder 4.10.2 scans covered `/`, `/demo`, `/privacy`,
  `/terms`, and the designed 404 at 1440×900 light and 390×844 dark. All ten
  scans had zero serious or critical findings.
- All normal-size routes had one h1 and one main, no horizontal overflow, and
  controls at least 44 CSS px in both dimensions. Keyboard traversal reached
  the skip link, navigation, primary action, code sample, license controls,
  and footer. Focus uses a visible 3 px vermilion ring. Enter opened the demo,
  focused its h1, and browser Back restored `/` and focused its h1.
- Reduced motion changed the inspection animation to 0.00001 s. No unexpected
  application console or page errors occurred.
- Service-worker registration and `update()` completed. After warm-up, a
  network-off reload of `/demo` retained the correct title, heading, sample,
  and no error. Cache Storage contained only `ios-review-gate-v3`.
- Mobile Lighthouse 12.8.2 scored Performance 100, Accessibility 100, Best
  Practices 100, and SEO 100. FCP was 1.1 s, LCP 1.6 s, TBT 10 ms, CLS 0.026,
  and total transfer was 123 KiB.

### Privacy, headers, and caching

A fresh landing-to-demo-to-reset flow requested only the site document and
same-origin JS, CSS, font, and artwork. On `/demo`, localStorage,
sessionStorage, cookies, and IndexedDB were empty; Cache Storage contained the
documented static-shell cache. The optional invalid-license action made one
documented cross-origin GET to `api.sociobot.in`, sent the token in the query,
and sent no release data. There are no analytics or third-party runtime fonts
or scripts.

Live responses include HSTS, `nosniff`, Referrer-Policy, Permissions-Policy,
and a CSP whose `connect-src` allows only self and the Sociobot billing API.
Hashed JS/CSS use `public, max-age=31536000, immutable`; `sw.js` uses
`no-cache`; HTML uses a 30-second revalidation policy.

The site has no sign-in, product backend, analytics, or AI feature. Entra
authority, backend concurrency/persistence, and AI gateway checks are not
applicable. An AI step would not improve the deterministic local preflight
job.

### Product-unlock API

Rate limiting passes. After a six-second clean window, 30 sequential invalid
license requests returned 200. Request 31 returned **429** with
`Retry-After: 3` and `X-RateLimit-After: 3`. **Observed allowance: 30 requests
per client/window.** A normal invalid license returned 200 with `valid:false`,
and the browser showed the recoverable inactive-license state.

## Release-blocking defects

### High — invalid queue entries can receive a clean PASS and a shorter plan

A release containing this syntactically valid queue entry was exercised with
the clean-consumer binary:

```yaml
active_submissions:
  - version: ""
    build: ""
    status: typo_in_reveiw
    submitted_on: 2026-09-03
```

The CLI returned exit **0**, `passed:true`, zero errors, zero warnings, and
`active_submissions:0`. Its decision remained 2026-09-04 instead of accounting
for the queued submission. The checker accepts arbitrary status text and does
not validate a queued submission's version or build. A likely typo therefore
silently removes a real build from the core queue plan and issues a green
decision record.

Changing the same entry to version 2.3.9, build 107, and `status: in_review`
made it count as active and moved the dates to 2026-09-06 / 2026-09-08. Define
and validate the allowed status values, reject blank queue identity, and add a
regression test proving invalid entries cannot pass.

### High — the contracted one-time Team purchase is still unavailable

The researched brief specifies one-time monetization, and the attached paid
unlock contract requires the exact price and a Sociobot checkout action. The
live site instead says checkout is unavailable, shows no price or buy action,
and only supports holders of an existing token. Fresh direct evidence:

```text
GET https://api.sociobot.in/api/v1/products/ios-review-gate/checkout
HTTP 404
{"error":"enabled factory product","status":404}
```

The invalid-license message also says “Check the token or buy a license,” but
the page provides no way to buy one. The free core remains useful and the
deviation is documented, but the acceptance contract's purchase path is not
shippable. Register and enable the product with its price/return URL, restore
the hosted checkout link and exact one-time price, then verify redirect,
return-token storage, and purchase restoration end to end.

### Medium — extreme queue-day values panic instead of returning a validation error

Both `typical_review_days: 9223372036854775807` and
`typical_review_days: -9223372036854775808` are accepted by YAML deserialization
and then panic in `chrono::TimeDelta::days`. Each run exited **101** and printed
`TimeDelta::days out of bounds`; neither returned the documented exit 1 or an
actionable correction. The negative value is even identified as invalid in
the checker, but date calculation happens anyway. Bound the durations before
constructing a date, use checked date arithmetic, and return a normal input
error/HOLD without a panic.

### Medium — `/demo` loses content at 200% text on a 390 px viewport

At 200% root text size, the demo grid's first child grows to 377.6 px from a
20 px left offset, placing its right edge at 397.6 px. `main { overflow:
hidden; }` masks the excess, so the right edge of the eyebrow, h1, explanatory
text, release values, and packet preview is visibly cut off. The document's
reported width still equals 390 px, so the existing overflow regression test
does not detect the loss. Evidence is `/tmp/iosrg-demo-200.png` in the verifier
workspace. Test element bounds/content visibility as well as document width,
and allow the grid item to shrink or wrap without clipping.

## Acceptance decision

**FAIL.** The declared claims, normal CLI flow, build/package, live parity,
privacy, headers, rate limit, ordinary keyboard/accessibility checks, offline
reload, and performance all pass. Release remains blocked by a false-PASS path
in the core queue planner and the unavailable contracted purchase. The CLI
panic and 200% demo clipping should be repaired in the same cycle.
