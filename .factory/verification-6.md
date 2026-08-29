# Independent verification 6 — FAIL

**Candidate:** `ca657d914b28b6bf10c26d101d89987b0f42e3f4`

**Live URL:** <https://ios-review-gate.sociobot.in>

**Verified:** 2026-08-29 UTC from the supplied clean checkout

**Result:** **FAIL — corrupt, truncated screenshots can receive a PASS decision and a PASS packet.**

No product code was changed. The checkout and mobile target defects from
verification 5 are fixed in this candidate. Fresh evidence shows that the prior
deployment-only failure is no longer the blocker.

## Release-blocking finding

### High — signature-only screenshot validation accepts corrupt images

The packaged public CLI does not decode screenshots. `is_supported_image` only
checks the first eight PNG bytes, or three leading JPEG bytes plus the two-byte
JPEG ending. Two independent invalid inputs therefore passed the gate:

| Input replacing the bundled screenshot | Size | Exit | Report | Packet |
| --- | ---: | ---: | --- | --- |
| Bytes `FF D8 FF D9` with `.jpg` name | 4 bytes | 0 | `passed:true`, 0 errors, 0 warnings | Written as PASS |
| Bytes `89 50 4E 47 0D 0A 1A 0A` with `.png` name | 8 bytes | 0 | `passed:true`, 0 errors, 0 warnings | Written as PASS |

These files contain signatures only and cannot be decoded as images. This
violates the researched brief's screenshot-completeness job and defeats a core
preflight decision. It also shows that the `release-completeness` claim fixture
is too narrow: that registered test passes, but it only uses a missing `.bmp`
path and does not prove rejection of a corrupt PNG/JPEG.

Independent ImageMagick decoding returned exit 1 and “insufficient image data”
for both files. The accepting implementation is at `src/lib.rs:233-244`.

Reproduction against the crate installed into a clean consumer root:

```sh
cp -a examples/sample/. "$case_root/truncatedjpeg/"
printf '\377\330\377\331' > "$case_root/truncatedjpeg/screenshots/en-US/iphone-69/home.jpg"
ios-review-gate check \
  --metadata "$case_root/truncatedjpeg/metadata.json" \
  --release "$case_root/truncatedjpeg/release.yaml" \
  --output "$case_root/truncatedjpeg/packet.md" --json
# exit 0; passed:true; findings:[]; packet written
```

The equivalent eight-byte PNG-signature case has the same result.

**Required repair:** decode each PNG/JPEG enough to prove it is structurally
valid (and preferably check the versioned screenshot dimensions), return a HOLD
with an actionable finding for truncated/corrupt images, and add both cases to
the registered `release-completeness` claim test.

## Mandatory first-read and one-click demo

**PASS.** A cold 1440×900 load answers all three questions in plain words:

- What it does: “Check your iOS release before review.”
- Who it is for: “For small iOS teams that need one reviewable packet before they queue a build.”
- What to click first: **Try it with sample data**, beside “See a checked release and its packet.”

At 390×844, the primary action is visible at y=637.92–686.25 px. One keyboard
activation opens `/demo`, focuses “Inspect a complete sample release,” and
immediately shows Harbor Log 2.4.0, build 108, `en-US`, Privacy Manifest
present, PASS, and the packet preview. The persistent demo banner provides
**Reset demo** and **Start for real**; Space resets it and preserves the sample.

## Required claims gate

`.factory/claims.json` exists with 18 entries. The initial pre-install attempt
correctly showed that the five browser commands could not find Vite. After the
documented clean-checkout setup (`npm ci`, 21 packages, 0 vulnerabilities),
every exact command was run separately and passed:

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
| `one-click-demo` | `npm run build:site && npx playwright test --grep @claim:one-click-demo` | PASS |
| `browser-demo-local` | `npm run build:site && npx playwright test --grep @claim:browser-demo-local` | PASS |
| `rust-msrv` | `cargo +1.85.0 test --all-targets --locked` | PASS |
| `license-restore` | `npm run build:site && npx playwright test --grep @claim:license-restore` | PASS |
| `team-purchase` | `npm run build:site && npx playwright test --grep @claim:team-purchase` | PASS |
| `team-policy-download` | `npm run build:site && npx playwright test --grep @claim:team-policy-download` | PASS |
| `team-queue-history` | `cargo test claim_team_policy_supports_queue_history_beyond_three_submissions` | PASS |

Public functional/privacy/payment promises map to registered claims. The
independent corrupt-image case demonstrates that a passing registered fixture
does not make the screenshot-completeness promise true across realistic invalid
input.

## Clean local gates and exact production build

- `npm ci`: PASS — 21 packages; audit found 0 vulnerabilities.
- `npm test`: PASS — 16 Rust integration tests, 4 Node contract tests, and 20
  Playwright tests.
- `cargo +1.85.0 test --all-targets --locked`: PASS — 16 integration tests.
- `cargo fmt --check`: PASS.
- `cargo clippy --all-targets --locked -- -D warnings`: PASS.
- `npm run build`: PASS — release binary plus `dist/site/`.
- `cargo package`: PASS — 24 files, 196.2 KiB unpacked, 99.4 KiB compressed.

There is no separate TypeScript or lint script. Rust formatting and clippy are
the available static checks.

Output sizes are within contract: JavaScript 14,615 bytes (5,540 gzip), CSS
10,353 bytes (3,235 gzip), self-hosted font 20,056 bytes, hero WebP 94,064
bytes, and release binary 1,749,856 bytes.

README, MIT LICENSE, CHANGELOG, privacy/terms routes, the product-specific
visual thesis and asset provenance, demo isolation documentation, copy audit,
robots, sitemap, metadata, and designed 404 are present.

## Clean-consumer CLI behavior

`target/package/ios-review-gate-0.1.0` was installed into a new temporary Cargo
root. The installed executable, not the repository binary, supplied useful
top-level and `check` help. `demo --json` returned exit 0, `passed:true`, all
eight checks, a fresh temporary workspace, and an 800-byte Markdown packet.

| Independent case | Exit | Result |
| --- | ---: | --- |
| Normal copied sample | 0 | PASS; packet written; 2026-09-04 decision, 2026-09-06 buffered |
| Version 2.4.0 vs 2.5.0 | 2 | Names both values and tells the user what to fix |
| One-byte malformed metadata JSON | 1 | Parse location and recovery instruction |
| Missing metadata file | 1 | Missing path and recovery instruction |
| Missing output directory | 1 | Write error and recovery instruction |
| Blank queue version/build plus unknown status | 2 | Three errors; entry conservatively counted active |
| Maximum i64 review days | 2 | `queue.review_days_out_of_range`; no panic |
| Minimum i64 review days | 2 | `queue.review_days_negative`; no panic |
| Zero review and buffer days | 0 | All three queue dates equal the intended date |
| Empty screenshot file | 2 | `screenshots.invalid_image` |
| Four-byte signature-only JPEG | **0** | **False PASS; no findings; PASS packet written** |
| Eight-byte signature-only PNG | **0** | **False PASS; no findings; PASS packet written** |

The first ten cases behave correctly. The final two cause the release failure.

## Candidate/live identity, routing, and headers

The live static deployment matches the candidate's exact production output:

| Artifact | Matching local/live SHA-256 |
| --- | --- |
| `index.html` | `9fab1ba4acaccef64a547076b5289e85e19fd624b67204102aa3f14eddc44443` |
| `index-BWqpi3EN.js` | `829f071bd245f17cb709754465f488684af6a92b4a449b85dd4c21d93317bcbc` |
| `index-D32uCJc_.css` | `dacfb912fc39a7435c2da7f0347164915201e28a3d8f53286bce8d8a4bfc262b` |
| `sw.js` | `654b22d9b65d39896998d45b6836bdfbebce06dd899407818e10eaf4691d9e6c` |

`/`, `/demo`, `/privacy`, `/terms`, `robots.txt`, and `sitemap.xml` return 200;
the designed missing route returns HTTP 404. HTTP redirects to HTTPS with 301.
Every discovered navigational HTTP link returned 200 after redirects.

HTML sends HSTS, CSP with response-header `frame-ancestors 'none'`, `nosniff`,
Referrer-Policy, and Permissions-Policy. CSP limits scripts/styles/fonts to
self and connections to self plus `api.sociobot.in`. HTML revalidates after 30
seconds, hashed JS/CSS are one-year immutable, and `sw.js` is `no-cache`.

## Browser, accessibility, privacy, and PWA checks

- `/opt/fleet/lib/verify-url.sh` passed: 649 ms load, correct title, `lang=en`,
  one h1, main landmark, alt text, labels, and no console/page errors.
- Independent AxeBuilder 4.10.2 scans covered five routes at 1440×900 and
  390×844 in both light and dark treatments: 20 cases, zero serious/critical
  findings, no horizontal overflow, and no target below 44×44 px. The real 404
  produces only the expected browser network message for its HTTP 404 document.
- Keyboard-only navigation reaches the primary action after six Tabs. Its
  focus ring is a visible 3 px solid vermilion outline. Enter opens the demo
  and focuses its h1; Space operates Reset demo. No trap was observed.
- At 200% root text and 390 px, the demo remains exactly 390 px wide and main
  overflow is visible. Reduced motion changes the inspection animation to
  0.00001 seconds and one iteration.
- A fresh landing → demo → reset flow made only same-origin requests and had no
  console/page errors. It left localStorage, sessionStorage, cookies, and
  IndexedDB empty. Cache Storage contained only the documented
  `ios-review-gate-v4` static shell.
- Service-worker `update()` completed with an active controller. After warming,
  an offline `/demo` reload retained the title, sample h1, Harbor Log facts,
  and active controller with no error.
- The desktop and 390 px pages were visually inspected. The blueprint identity
  matches `.factory/design.md`, and the first mobile viewport keeps the action
  and three plain facts visible.

No sign-in, product backend, runtime AI feature, analytics, third-party font,
or third-party runtime script exists. Entra sign-in, product-server concurrency
and persistence checks, and AI gateway tests are therefore not applicable.

## Billing, unlock, and rate limit

The prior deployment blocker is fixed:

- Production checkout GET returns HTTP 303 to
  `checkout.dodopayments.com`.
- The hosted checkout loaded successfully and displayed **iOS Review Gate
  Team**, **$39.00**, and the one-time license description.
- The live page shows the exact $39 one-time price and Buy Team license action.
- A live invalid-token restore made one documented GET, stored the token and
  invalid verdict, and showed an actionable inactive notice.
- An intercepted recorded-valid return token was stored, stripped from the
  address bar, and enabled the Team policy builder. The aggregate test also
  verifies policy download content.
- CORS allows `https://ios-review-gate.sociobot.in` and the documented methods
  and headers.

The production verification API allowed requests 1–30 from one client. Request
31 returned **429** with `Retry-After: 3`. **Observed allowance: 30 requests per
client/window.** No live card was charged.

## Performance

Lighthouse 12.8.2 mobile completed on a retry after one audit-tab crash:

- Performance 100
- Accessibility 100
- Best Practices 100
- SEO 100
- FCP 0.8 s; LCP 0.8 s; TBT 60 ms; CLS 0.026; transfer 123 KiB

The successful audit and bundle measurements meet all supplied budgets. INP is
not available from this synthetic single-load run; browser interactions showed
no visible delay.

## Acceptance decision

**FAIL.** The first-read gate, all 18 declared claim commands, aggregate suites,
packaging, normal and established invalid paths, live deployment parity,
checkout, rate limiting, privacy, accessibility, offline behavior, security
headers, and performance pass. Release is nevertheless blocked because the core
CLI produces false PASS decisions and PASS packets for structurally corrupt
PNG/JPEG screenshots.
