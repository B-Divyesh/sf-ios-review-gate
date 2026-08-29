# Independent verification 11 — FAIL

**Candidate:** `62ff8d7ceb460f87fd38efa8c8f8a0874050b907`  
**Live URL:** <https://ios-review-gate.sociobot.in>  
**Verified:** 2026-08-29 UTC from the clean candidate checkout  
**Decision:** **FAIL — seven mandatory claim commands failed in the required pre-install clean-clone run, and production hashed assets lack immutable caching.**

No product code was changed during verification.

## Mandatory first-read and demo gate

**PASS.** A cold 1440×900 load answers all three questions without scrolling:

- What it does: “Check your iOS release before review.”
- Who it serves: “For small iOS teams that need one Markdown review packet before they queue a build.”
- What to do: **Try it with sample data**, with “Open a checked sample and its review packet” beside it.

The first screen also shows the three local/offline/price facts. At 390×844,
the action and all three facts remain visible. One click opens `/?demo=1` and
immediately shows Harbor Log 2.4.0, build 108, privacy-manifest status, a PASS
decision, and the Markdown packet. The persistent banner says “Demo — sample
data, nothing is saved” and offers Reset demo and Install the CLI.

## Mandatory claim gate

`.factory/claims.json` exists and contains 25 entries. Before dependency
installation or the general QA suite, every listed `test` command was executed
exactly as written. The result was **18 passed, 7 failed**.

| Claim | Exact command | Initial clean-clone result |
| --- | --- | --- |
| `identity-consistency` | `cargo test claim_identity_consistency` | PASS |
| `release-completeness` | `cargo test claim_release_completeness` | PASS |
| `sample-screenshot-dimensions` | `cargo test claim_sample_screenshot_dimensions_match_the_documented_fixture` | PASS |
| `markdown-packet` | `cargo test claim_markdown_packet` | PASS |
| `core-without-team-license` | `cargo test claim_core_gate_runs_without_team_license_and_writes_packet` | PASS |
| `bundled-cli-demo` | `cargo test claim_bundled_demo` | PASS |
| `archive-inspection` | `cargo test claim_archive_inspection_extracts_xcarchive_and_ipa_then_checks_release` | PASS |
| `same-checker-demo` | `cargo test claim_demo_recording_matches_bundled_cli` | PASS |
| `actionable-mismatch-errors` | `cargo test claim_actionable_mismatch_error_names_values_and_fix` | PASS |
| `queue-plan` | `cargo test claim_queue_plan` | PASS |
| `queue-input-validation` | `cargo test claim_queue_input_validation_rejects_incomplete_or_unknown_entries` | PASS |
| `queue-date-limits` | `cargo test claim_queue_date_limits_hold_without_panicking` | PASS |
| `cli-local` | `cargo test claim_cli_local` | PASS |
| `cli-exit-codes` | `cargo test claim_cli_exit_codes` | PASS |
| `cli-json-schema` | `cargo test claim_cli_json_schema` | PASS |
| `one-click-demo` | `npm run build:site && npx playwright test --grep @claim:one-click-demo` | **FAIL, exit 127: `vite: not found`** |
| `browser-demo-local` | `npm run build:site && npx playwright test --grep @claim:browser-demo-local` | **FAIL, exit 127: `vite: not found`** |
| `offline-shell` | `npm run build:site && npx playwright test --grep @claim:offline-shell` | **FAIL, exit 127: `vite: not found`** |
| `rust-msrv` | `cargo +1.85.0 test --all-targets --locked` | PASS — 18 tests on Rust 1.85.0 |
| `license-restore` | `npm run build:site && npx playwright test --grep @claim:license-restore` | **FAIL, exit 127: `vite: not found`** |
| `team-purchase` | `npm run build:site && npx playwright test --grep @claim:team-purchase` | **FAIL, exit 127: `vite: not found`** |
| `team-policy-download` | `npm run build:site && npx playwright test --grep @claim:team-policy-download` | **FAIL, exit 127: `vite: not found`** |
| `team-queue-history` | `cargo test claim_team_policy_supports_queue_history_beyond_three_submissions` | PASS |
| `license-metadata` | `node --test --test-name-pattern='@claim:license-metadata' tests/site.test.js` | PASS |
| `version-metadata` | `npm run build && npx playwright test --grep @claim:version-metadata` | **FAIL, exit 127 after the CLI build: `vite: not found`** |

After `npm ci`, the seven failed commands were rerun exactly and all seven
passed. That proves the features work with dependencies installed, but does
not change the required initial result: the acceptance contract says any
failing claim test is release-blocking. The web claim commands are not
self-contained for the required clean-clone invocation.

## Local quality gates after installation

- `npm ci`: PASS — 21 packages installed; zero audit vulnerabilities.
- `npm test`: PASS — 18 Rust integration tests, 5 Node tests, and 24
  Playwright tests.
- `cargo fmt --check`: PASS.
- `cargo clippy --all-targets --locked -- -D warnings`: PASS.
- `npm run build`: PASS — release CLI plus exact Vite production build in
  `dist/site/`.
- Production assets: JavaScript 20,723 bytes raw / 7,034 bytes gzip; CSS
  11,172 bytes raw / 3,407 bytes gzip; font 20,056 bytes; hero art 94,064
  bytes. All are below the stated budgets.
- `cargo package --locked --allow-dirty`: PASS — 28 files, 365.6 KiB
  unpacked / 204.8 KiB compressed.

There is no TypeScript checker or JavaScript lint script. Rust formatting and
Clippy are the available static checks.

## Packaged CLI and end-to-end behavior

The generated crate was unpacked into a new temporary consumer and installed
with `cargo install --path … --locked --root …`; the existing repository
binary was not reused.

- `--version` reported `ios-review-gate 0.1.0`; top-level and subcommand help
  describe `check`, `inspect`, and `demo` without an interactive prompt.
- `demo --json` returned exit 0, `passed: true`, eight checks, queue dates,
  and a new temporary Markdown packet path.
- `inspect` extracted bundle ID, version, build, manifest state, collected
  data, and reason codes from the shipped `.xcarchive`.
- A real `check --archive … --release … --output … --json` returned PASS and
  wrote a dated packet naming `apple-2026.1`, artifact identity, queue plan,
  localization, and reviewer sign-off.
- Missing inputs returned exit 1 with “Fix the path or file contents, then
  run the command again.”
- Seeded mismatch, malformed input, incomplete/unknown queue entry, extreme
  date, invalid image, `.ipa`, and Team-policy cases are covered by the
  passing Rust claim suite. HOLD uses exit 2; invalid input uses exit 1.

The CLI has no network client or telemetry dependency. Its runtime surface is
local filesystem input/output only.

## Live deployment identity and routes

The live deployment matches the candidate. SHA-256 values matched exactly for
`index.html`, `404.html`, `sw.js`, `assets/main-B1aPlnQU.js`, and
`assets/main-FbCzFwnk.css`. The live footer reports
`CLI v0.1.0 · build 62ff8d7ceb46`.

- `/`, `/demo`, `/privacy`, and `/terms` return 200 with route-specific
  titles, `lang=en`, exactly one h1, and one main landmark.
- A cold unknown path returns the designed page with HTTP 404 and a way home.
- All ordinary internal links return 200; checkout returns the intended 303;
  the factory link returns 200; mail links are explicit.
- `/opt/fleet/lib/verify-url.sh` passed in 719 ms with no page/console errors,
  missing alt text, or unlabeled buttons.

## Accessibility, responsive behavior, and resilience

- Independent Axe 4.10.2 scans covered home, demo, privacy, terms, and 404 at
  1440×900 and 390×844 in both light and dark modes: **zero serious or
  critical violations**.
- Every visible mobile interactive target measured at least 44×44 CSS pixels.
  All tested pages had `scrollWidth === clientWidth` at 390 px.
- At 200% root text size on 390 px, width remained 390 px and no tested
  heading, paragraph, form control, or link was clipped.
- Keyboard Tab reached the skip link, navigation, sample action, scrollable
  code, purchase link, labeled license field, button, and footer. Focus uses a
  visible 3 px vermilion outline with a 3 px offset. Enter on the demo action
  changed route and focused the new h1; Space activated Reset demo.
- Reduced-motion contexts reported only `0.00001s` animation/transition
  durations. No loop or flash was observed.
- Ordinary routes produced no console or page errors. The expected main
  document 404 generated the browser's standard failed-resource console line.

## Privacy, service worker, and headers

A fresh browser was seeded with unrelated `real:release` and `real:draft`
values, then entered and reset the demo. Both values remained byte-for-byte
unchanged. Cookies were empty, IndexedDB had no databases, and the only cache
was `ios-review-gate-v7`. Every demo request was same-origin; no analytics,
CDN, or third-party font/script request occurred.

The service worker reached `ready`, `registration.update()` completed, and
the cache contained 11 same-origin shell/static URLs. With the browser then
offline, `/?demo=1` reloaded with title, h1, and PASS sample intact and no
errors.

Browser response headers include HSTS, `nosniff`, Referrer-Policy,
Permissions-Policy, and a header-delivered CSP with `frame-ancestors 'none'`.
The CSP permits only same-origin runtime resources plus the documented
Sociobot API connection.

An explicit invalid-license action sent only the documented request to
`api.sociobot.in` and announced a recoverable error through an
`aria-live="polite"` region.

## Billing endpoint and request allowance

- The $39 Team link returns HTTP 303 to hosted Dodo checkout. No purchase was
  completed during verification.
- After a clean five-second window, 35 sequential invalid-license verification
  calls from one client returned 200 for requests 1–30 and 429 for requests
  31–35. Every denial had `Retry-After: 4` and the body “Too Many Requests!
  Wait for 4s”. **Observed allowance: 30 accepted requests per client/window;
  request 31 is limited.**
- The product has no sign-in flow or product backend. Entra authority,
  backend concurrency, and backend persistence checks are not applicable.
- The brief does not benefit from runtime AI; no AI request or unsupported AI
  claim is present.

## Performance

A clean mobile Lighthouse 12.8.2 run scored Performance 100,
Accessibility 100, Best Practices 100, and SEO 100. FCP was 1.1 s, LCP 1.6 s,
TBT 20 ms, CLS 0.026, and total transfer was 124 KiB. INP has no meaningful
lab interaction sample; keyboard interaction was exercised separately.

## Defects by severity

### High — seven exact claim commands fail in the mandatory clean-clone run

The seven web claim commands invoke local Vite/Playwright tooling without
installing dependencies. In the explicitly required pre-install run, each
failed with exit 127 and `vite: not found`. Any failing claim command is a
release blocker under the acceptance contract. Make every registered command
self-contained from a clean clone, for example by including the deterministic
install step, then rerun all 25 commands in the required order.

### Medium — immutable cache rule does not match emitted asset names

`site/public/staticwebapp.config.json` configures one-year immutable caching
only for `/assets/index-*`, while this production build emits
`/assets/main-B1aPlnQU.js` and `/assets/main-FbCzFwnk.css`. Playwright response
headers and independent `curl -I` both show the live hashed files receive
`Cache-Control: public, must-revalidate, max-age=30`, not the required
long-lived immutable policy. `sw.js` correctly receives `no-cache`. Update the
route pattern to match all emitted hashed assets and verify the deployed
headers.

- Critical: none.
- High: 1.
- Medium: 1.
- Low: none.

## Acceptance decision

**FAIL.** Do not accept candidate `62ff8d7ceb460f87fd38efa8c8f8a0874050b907`.
The product itself works end to end after installation and the live deployment
matches it, but the mandatory claim gate failed exactly as invoked from the
clean clone. The cache-policy mismatch is an additional deployment-quality
defect.
