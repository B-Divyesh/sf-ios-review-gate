# Independent verification 7 — FAIL

**Candidate:** `cc9b95653bddf3ba5fe7744d1ebe328943c278bf`

**Live URL:** <https://ios-review-gate.sociobot.in>

**Verified:** 2026-08-29 UTC from the supplied clean checkout

**Result:** **FAIL — the CLI can issue a PASS packet for screenshots that App Store submission cannot accept.**

No product code was changed. The repaired truncated-image case now works and
the candidate is deployed, but fresh independent cases expose a broader
screenshot-completeness defect. Formatting, recovery, claims-registry, and
accessibility findings also remain.

## Release-blocking findings

### High — invalid App Store screenshot sizes and device sets receive PASS

The researched brief requires screenshot completeness as part of a realistic
pre-submission gate. The checker only enforces a nonempty set, a count of 1–10,
PNG/JPEG extension, file presence, and successful decoding. It does not
validate the device-set key, dimensions, or orientation against its versioned
rules. The rules file contains only minimum and maximum counts.

Fresh evidence against the packaged crate installed in a clean Cargo root:

| Input | Exit | Report | Packet |
| --- | ---: | --- | --- |
| Shipped `iphone-69/home.jpg`, measured **900×600** | 0 | `passed:true`; 0 findings | PASS |
| Valid, decodable **1×1 JPEG** in `iphone-69` | 0 | `passed:true`; 0 findings | PASS |
| Shipped image under device set **`not-a-device`** | 0 | `passed:true`; 0 findings | PASS |

The 1×1 reproduction used the public packaged binary:

```sh
convert -size 1x1 xc:red screenshots/en-US/iphone-69/home.jpg
ios-review-gate check \
  --metadata metadata.json --release release.yaml \
  --output packet.md --json
# exit 0; passed:true; findings:[]; packet contains **Decision: PASS**
```

This is a false release decision, not a cosmetic omission. A decodable image
is not necessarily an App Store screenshot, and the shipped “complete sample”
itself uses a non-App-Store 900×600 size. Relevant implementation:
`src/lib.rs:234-246`, `src/lib.rs:468-523`, and
`rules/apple-2026.1.yaml:10-12`.

Required repair: put accepted iOS device-set names, pixel dimensions, and
orientations in the versioned rules; reject unknown sets and wrong dimensions;
ship an accepted-size sample; and extend the registered
`release-completeness` claim with 1×1 and unknown-device fixtures.

### Medium — the available Rust formatting gate fails

Both the current toolchain and Rust 1.85 report the same `cargo fmt --check`
failure at `tests/cli.rs:73`. Rustfmt requires the `splice` arguments to be
expanded across lines. `cargo clippy --all-targets --locked -- -D warnings`
passes, but the requested available lint/format gate does not.

### Medium — corrupt cached license state cannot recover

With `sb_license:ios-review-gate` present and its verdict value set to malformed
JSON, a live page load emits an uncaught `SyntaxError`. Trying to verify a new
token reaches the same unguarded parse and leaves the action stuck at
“Checking license…”. There is no in-product way to clear the bad verdict in
this state. The parses precede the protected fetch path at
`site/src/main.js:120` and `site/src/main.js:181`.

Required repair: parse cached state defensively, discard malformed or
wrong-shaped values, continue with verification, and add a recovery regression
that asserts no console/page error.

### Medium — public CLI output claims are absent from `claims.json`

README lines 35 and 43 promise the `0`/`1`/`2` exit-code contract and the JSON
object fields. Tests named `invalid_input_has_actionable_error_and_exit_one`
and `failed_gate_exits_two_and_json_is_parseable` exist, but neither public
promise has a `.factory/claims.json` entry or an exact registered claim test.
The supplied claims contract says any unlisted README claim fails review.

Required repair: register the documented exit-code and JSON schema promises
and give each exactly one claim test command, or remove the promises.

### Medium — visible mobile wordmark is missing from its accessible name

Lighthouse's Axe 4.13.0 `label-content-name-mismatch` audit reports a
**serious** WCAG 2.1 A finding. At 390 px the visible link text is `RG`, while
the accessible name is `iOS Review Gate home`; the visible label is not in the
name. This affects speech-input targeting. The pinned Axe 4.10.2 default
profile does not enable this experimental rule, which is why the repository
suite and default matrix miss it.

Required repair: include the visible wordmark in the accessible name (and
cover the responsive state), then run the label-in-name rule explicitly.

## Mandatory first-read and one-click demo

**PASS.** A fresh 1440×900 live load answers the three mandatory questions:

- What: “Check your iOS release before review.”
- Who: “For small iOS teams that need one reviewable packet before they queue a build.”
- First action: **Try it with sample data**, beside “See a checked release and its packet.”

The action is also fully visible at 390×844 (`y=637.92–686.25`). One click
opens `/demo` and immediately shows Harbor Log 2.4.0, build 108, locale
`en-US`, Privacy Manifest present, a PASS decision, and packet output. The
persistent banner includes **Reset demo** and **Start for real**.

## Required claims gate

`.factory/claims.json` exists with 18 entries. Per the instruction to run these
before other QA, the five browser commands initially reported `vite: not found`
in the dependency-free clone. After the repository's documented clean setup
(`npm ci`: 21 packages, 0 vulnerabilities), every exact command was rerun and
passed:

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
| `one-click-demo` | `npm run build:site && npx playwright test --grep @claim:one-click-demo` | PASS after `npm ci` |
| `browser-demo-local` | `npm run build:site && npx playwright test --grep @claim:browser-demo-local` | PASS after `npm ci` |
| `rust-msrv` | `cargo +1.85.0 test --all-targets --locked` | PASS |
| `license-restore` | `npm run build:site && npx playwright test --grep @claim:license-restore` | PASS after `npm ci` |
| `team-purchase` | `npm run build:site && npx playwright test --grep @claim:team-purchase` | PASS after `npm ci` |
| `team-policy-download` | `npm run build:site && npx playwright test --grep @claim:team-policy-download` | PASS after `npm ci` |
| `team-queue-history` | `cargo test claim_team_policy_supports_queue_history_beyond_three_submissions` | PASS |

The registered `release-completeness` fixture now correctly rejects the prior
four-byte JPEG and eight-byte PNG defects. It does not cover dimensions or
device-set validity, so its passing result does not prove the brief's broader
screenshot-completeness job.

## Clean local gates and exact production build

- `npm ci`: PASS — 21 packages; 0 audit vulnerabilities.
- `npm test`: PASS — 16 Rust integration tests, 4 Node tests, 20 Playwright tests.
- `cargo +1.85.0 test --all-targets --locked`: PASS — 16 integration tests.
- `cargo fmt --check`: **FAIL**.
- `cargo +1.85.0 fmt --check`: **FAIL** with the same diff.
- `cargo clippy --all-targets --locked -- -D warnings`: PASS.
- `npm run build`: PASS — release binary and `dist/site/` produced.
- `cargo package --locked`: PASS — 24 files, 201.1 KiB unpacked, 100.8 KiB compressed.

There is no separate JavaScript typecheck or lint script. Production output is
within budget: JS 14,615 bytes (5,540 gzip), CSS 10,353 bytes (3,235 gzip),
font 20,056 bytes, hero WebP 94,064 bytes, and release binary 2,351,376 bytes.

README, MIT license, changelog, privacy/terms, demo documentation, copy audit,
visual thesis and provenance, robots, sitemap, metadata, and designed 404 are
present.

## Clean-consumer CLI exercise

The generated crate was installed from `target/package/ios-review-gate-0.1.0`
into a new Cargo root. Top-level help and `check --help` are useful. `demo
--json` returned exit 0, `passed:true`, eight checks, a new temporary workspace,
and an 800-byte Markdown packet.

| Independent case | Exit | Result |
| --- | ---: | --- |
| Normal shipped sample | 0 | PASS and packet written |
| Version 2.4.0 vs 2.5.0 | 2 | `identity.version` HOLD |
| Four-byte JPEG | 2 | `screenshots.invalid_image` HOLD |
| Eight-byte PNG signature | 2 | `screenshots.invalid_image` HOLD |
| Localized name at 30 characters | 0 | PASS |
| Localized name at 31 characters | 2 | `locales.field_too_long` HOLD |
| Negative review days | 2 | `queue.review_days_negative` HOLD |
| Maximum i64 review days | 2 | `queue.review_days_out_of_range` HOLD; no panic |
| Zero review and buffer days | 0 | All queue dates equal intended date |
| Malformed metadata JSON | 1 | Parse location and repair instruction |
| Missing metadata file | 1 | Missing path and repair instruction |
| Missing packet parent | 1 | Write error and repair instruction |
| Unknown API category | 0 | PASS with `privacy.api_unknown` warning |
| 1×1 decodable JPEG | **0** | **False PASS and PASS packet** |
| Unknown device-set key | **0** | **False PASS and PASS packet** |

## Candidate/live identity, routing, headers, and links

The live deployment matches the candidate's exact production output:

| Artifact | Matching local/live SHA-256 |
| --- | --- |
| `index.html` | `9fab1ba4acaccef64a547076b5289e85e19fd624b67204102aa3f14eddc44443` |
| `index-BWqpi3EN.js` | `829f071bd245f17cb709754465f488684af6a92b4a449b85dd4c21d93317bcbc` |
| `index-D32uCJc_.css` | `dacfb912fc39a7435c2da7f0347164915201e28a3d8f53286bce8d8a4bfc262b` |
| `sw.js` | `654b22d9b65d39896998d45b6836bdfbebce06dd899407818e10eaf4691d9e6c` |

`/`, `/demo`, `/privacy`, `/terms`, `robots.txt`, and `sitemap.xml` return
200. An unknown route returns the designed page with HTTP 404. HTTP redirects
to HTTPS with 301. All discovered internal and external web links resolve; the
checkout reaches Dodo and the factory link returns 200.

HTML responses send HSTS, CSP with response-header `frame-ancestors 'none'`,
`nosniff`, Referrer-Policy, and Permissions-Policy. HTML revalidates after 30
seconds, hashed JS/CSS are one-year immutable, and `sw.js` is `no-cache`.

## Browser, accessibility, privacy, and offline evidence

- `/opt/fleet/lib/verify-url.sh` passed in 678 ms: title, `lang=en`, one h1,
  main, image alt text, labels, and no normal-load console/page errors.
- AxeBuilder 4.10.2 scanned `/`, `/demo`, `/privacy`, `/terms`, and a real 404
  at desktop/mobile in light/dark: 20 cases, zero default-profile
  serious/critical violations, no overflow, and no visible target below 44 px.
  The expected browser resource message for the deliberate 404 was the only
  matrix console item. The separate label-in-name finding above comes from
  Lighthouse's newer Axe 4.13.0 experimental rule.
- Keyboard Tab reaches the primary action after six presses. Its focus is a
  visible 3 px vermilion outline (5.94:1 against paper). Enter opens the demo
  and focuses its h1; Space operates Reset demo. No trap was observed.
- At 200% text and 390 px, the demo remains within 390 px and packet content is
  visible. Reduced motion changes animation duration to 0.00001 s and one run.
- A fresh landing → demo → reset flow made seven same-origin requests only,
  with no errors. localStorage, sessionStorage, cookies, and IndexedDB remained
  empty. Cache Storage contained only `ios-review-gate-v4` static assets.
- Service-worker `update()` completed. With an active controller, an offline
  `/demo` reload retained the title, sample h1, Harbor Log facts, and PASS view.
- Desktop and 390 px screenshots were visually inspected. The drafting-sheet
  identity matches `.factory/design.md`; mobile keeps the action and three
  facts in the first viewport.

No sign-in, product backend, runtime AI, analytics, third-party font, or
third-party runtime script exists. Entra, product-server concurrency and
persistence, and AI gateway checks are not applicable.

## Billing, unlock, and rate limiting

- Production checkout returns 303 to `checkout.dodopayments.com`.
- Hosted checkout loads **iOS Review Gate Team**, **$39.00**, and the one-time
  license description. No live purchase was made.
- A live invalid-token restore made only the documented API request, stored
  the token/verdict, showed the inactive notice, and produced no page error.
- Recorded-valid claim tests cover return-token storage, URL stripping, and
  policy download content.
- CORS permits the product origin and documented methods/headers.
- In a fresh rate window, requests 1–30 returned 200; request 31 returned 429.
  Full capture showed `retry-after: 1` and `x-ratelimit-after: 1`.
  **Observed allowance: 30 requests per client/window.**

## Performance

Lighthouse 12.8.2 mobile:

- Performance 97
- Accessibility 100 (with the separate experimental audit finding noted above)
- Best Practices 100
- SEO 100
- FCP 1.1 s; LCP 1.6 s; TBT 140 ms; CLS 0.026; transfer 123 KiB

The supplied performance and bundle budgets pass. Synthetic INP was not
available; tested interactions responded without visible delay.

## Acceptance decision

**FAIL.** The candidate is deployed and most engineering, privacy, payment,
responsive, offline, security, and performance checks pass. Release remains
blocked because the core CLI returns PASS for images and device sets that do
not constitute valid iOS App Store screenshots. Rust formatting also fails,
the paid-license UI cannot recover from corrupt cached state, README contains
unregistered public API claims, and the responsive wordmark has a serious
label-in-name finding under the newer Axe audit.
