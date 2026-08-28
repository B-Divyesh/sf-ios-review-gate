# Independent verification 2 — FAIL

**Candidate:** `d6cc7ac208dde7d21e75ad9237f94ab7e5ebfd78`  
**Live URL:** <https://ios-review-gate.sociobot.in>  
**Verified:** 2026-08-28 UTC from a clean candidate worktree  
**Result:** **FAIL — release-blocking product, compatibility, claim, and checkout defects remain.**

No product code was changed during this verification.

## Mandatory first-read gate

**PASS.** In fresh desktop (1440×900) and mobile (390×844) browser contexts,
the cold first screen says:

- What it does: “Check your iOS release before review.”
- Who it is for: “For small iOS teams that need one reviewable packet before
  they queue a build.”
- What to click first: “Try it with sample data,” followed by “See a checked
  release and its packet.”

The primary action is visible without setup and opens `/demo` in one click.
The resulting page immediately shows Harbor Log 2.4.0, build 108, the release
facts, a PASS result, the packet preview, and the persistent demo banner.

## Required claim commands

`.factory/claims.json` exists. Every exact command was run before the general
test suite. All ten commands exited 0:

| Claim | Exact command | Result |
| --- | --- | --- |
| `identity-consistency` | `cargo test claim_identity_consistency` | PASS — one matching test |
| `release-completeness` | `cargo test claim_release_completeness` | PASS — one matching test |
| `markdown-packet` | `cargo test claim_markdown_packet` | PASS — one matching test |
| `bundled-cli-demo` | `cargo test claim_bundled_demo` | PASS — one matching test |
| `queue-plan` | `cargo test claim_queue_plan` | PASS — one matching test |
| `cli-local` | `cargo test claim_cli_local` | PASS — one matching test |
| `one-click-demo` | `npm run build:site && npx playwright test --grep @claim:one-click-demo` | PASS — 1 test |
| `browser-demo-local` | `npm run build:site && npx playwright test --grep @claim:browser-demo-local` | Command passes, but does not test the full claim; see High defect 4 |
| `license-restore` | `npm run build:site && npx playwright test --grep @claim:license-restore` | PASS — 1 test with recorded API response |
| `team-policy-download` | `npm run build:site && npx playwright test --grep @claim:team-policy-download` | PASS — 1 test and downloaded YAML assertion |

The command-level result does not override the defective `browser-demo-local`
claim test described below. It asserts only outbound origins, although its
claim also promises that the demo saves nothing.

## Clean local quality gates

- `npm ci`: PASS; 21 packages installed, 0 vulnerabilities reported.
- `cargo fmt --check`: PASS.
- `cargo clippy --all-targets -- -D warnings`: PASS.
- `npm test`: PASS — 10 Rust integration tests, 3 Node tests, and 14
  Playwright tests.
- `npm run build`: PASS — release CLI plus exact Vite production build.
- `cargo package`: PASS — 24 files, 174.6 KiB unpacked and 95.3 KiB
  compressed.
- Production site assets: 14,225-byte JS (5.44 KB gzip) and 9,781-byte CSS
  (3.08 KB gzip), below the 200 KB/50 KB budgets. Release binary: 1.7 MB.

There is no separate TypeScript or lint script. Rust formatting and clippy are
the available static checks.

## CLI and clean-consumer exercise

A second clean consumer installation was made from Cargo's verified packaged
tree at `target/package/ios-review-gate-0.1.0`, not from an existing binary.
With the current Rust 1.98 toolchain:

- `--version`, top-level `--help`, and `check --help` were correct.
- Two `demo --json` runs each returned exit 0, `passed: true`, and different
  temporary workspaces containing generated packets.
- The documented sample returned exit 0 and produced a dated Markdown packet.
- A build mismatch returned exit 2, `passed: false`, and
  `identity.build` in valid JSON.
- A missing file, malformed JSON, malformed Team policy, and an unwritable
  output path each returned exit 1 with a concrete error and retry instruction.
- A valid Team policy loaded successfully and appeared by name in JSON output.

Boundary testing also exposed false-PASS behavior described under High defect
2. The product is local-only: its Rust manifest has no network dependency and
the runtime accepts only filesystem inputs and outputs.

## Live deployment and browser evidence

- The deployed `index.html`, emitted JS, emitted CSS, and `sw.js` have the same
  SHA-256 values as this candidate's `dist/site` files. The live footer shows
  `v0.1.0 · build 2026.08.28`.
- `/opt/fleet/lib/verify-url.sh` passed: HTTP 200, correct title and `lang=en`,
  one `h1`, one main landmark, complete image alt text, no unlabeled buttons,
  and no console errors. Its measured load was 728 ms.
- Independent axe 4.10.2 runs covered `/`, `/demo`, `/privacy`, `/terms`, and
  the designed missing route in light and dark modes at 1440×900 and 390×844.
  There were no serious or critical findings.
- Keyboard traversal reached the skip link, navigation, sample action,
  scrollable command, license controls, and footer. Each received the designed
  3 px focus ring. Enter on the sample action opened `/demo`, moved focus to
  its `h1`, and browser Back restored `/` and focused its `h1`.
- Reduced motion changed the hero animation from 0.7 s to 0.00001 s. No page
  or console errors occurred in the exercised routes.
- Demo requests were same-origin only. Local and session storage remained
  empty, and IndexedDB had no databases. Cache Storage did contain the
  `ios-review-gate-v2` application shell; this matters to the claim defect.
- Service-worker registration and `update()` completed. After warm-up, `/demo`
  reloaded offline with the correct title, heading, sample, and no error.
- HTTPS, HSTS, CSP, `nosniff`, Referrer-Policy, and Permissions-Policy were
  present. HTTP redirects to HTTPS. Hashed JS/CSS use one-year immutable cache
  headers; `sw.js` uses `no-cache`.
- A successful mobile Lighthouse run scored Performance 100, Accessibility
  100, Best Practices 100, and SEO 100. FCP was 1.1 s, LCP 1.6 s, TBT 20 ms,
  CLS 0.028, and transferred resources totalled 122 KiB.
- The site has no sign-in flow, backend, analytics, third-party runtime font,
  or AI feature. Entra identity, backend concurrency/persistence, and AI
  gateway checks are not applicable.

## Billing endpoint evidence

- Checkout still fails: `GET
  https://api.sociobot.in/api/v1/products/ios-review-gate/checkout` returned
  HTTP 404 with `{"error":"enabled factory product","status":404}`.
- Rate limiting is now present. After a clean six-second cooldown, 40 rapid
  sequential requests to the verification endpoint returned 30 HTTP 200s,
  then 10 HTTP 429s. The first 429 was request 31 and every 429 carried
  `Retry-After: 4`. **Observed threshold: 30 accepted requests per window;
  request 31 was limited.** This resolves the prior rate-limit defect.
- A normal invalid license returned HTTP 200 with `valid:false` and
  `reason:"invalid"`. The live browser displayed “License not active. Check
  the token or buy a license.” The only cross-origin request was the documented
  Sociobot verification call.

## Release-blocking defects

### High — advertised $39 Team checkout is unavailable

The live “Buy Team license” link points to the endpoint above, which returns
HTTP 404 rather than hosted checkout. The paid feature cannot be purchased.
Register and enable the product with its $39 price and correct return URL, then
exercise the real redirect and return flow.

### High — incomplete or invalid release inputs can receive a PASS packet

Fresh CLI cases produced these results:

- Setting `app_name`, `bundle_id`, `version`, `build`, and `submitted_by` to
  empty strings in the required files returned exit **0**, `passed:true`, zero
  errors, and a packet with blank artifact identity and owner.
- Replacing the declared screenshot with a zero-byte file named `home.jpg`
  returned exit **0**, `passed:true`, and zero errors. The checker validates
  only extension and existence, not that the file is a usable image.
- Setting `typical_review_days: -5` and `buffer_days: -7` returned exit **0**
  and silently converted both durations to zero.

These are invalid inputs for a reviewable App Store submission, but the gate
issues a green decision. Validate required identity/owner values, image
integrity (and applicable screenshot constraints), and non-negative queue
settings before producing PASS.

### High — documented Rust 1.85 minimum does not compile

README says “Rust 1.85 or newer.” In a newly installed Rust 1.85.0 toolchain,
`cargo +1.85.0 test --all-targets --locked` failed with exit 101 at
`src/lib.rs:310`: `let expressions in this position are unstable`. The
manifest also has no `rust-version`. This install requirement is absent from
`.factory/claims.json` and is false on the stated minimum. Rewrite the
let-chain for 1.85 and declare/test MSRV, or publish the actual minimum and add
its claim test.

### High — `browser-demo-local` does not prove its full privacy claim

The claim is “The browser demo saves nothing and sends no cross-origin
requests,” but its test records only request origins. It never inspects
localStorage, sessionStorage, IndexedDB, Cache Storage, cookies, or writes.
Fresh live evidence found no demo data in local/session storage or IndexedDB,
but the service worker writes the `ios-review-gate-v2` shell to Cache Storage.
`.factory/demo.md` also says the website writes “no browser storage,” which is
literally false. Narrow the public claim to user/sample data, document the
required offline shell cache, and make the named test assert the revised
storage guarantee.

## Other defects

### Medium — the designed not-found route returns HTTP 200

`/does-not-exist` renders the intended not-found page, but both curl and the
browser receive HTTP 200. The static-host contract requires a real 404 response
override. Add a deployable `404.html`/response override while preserving the
designed recovery page.

### Medium — several 390 px touch targets are below 44 px

Measured target boxes include the 36×44 px wordmark, the 38×15 px inline terms
link, and footer links 24.8 px high on every route. Privacy and support email
links are 19 px high. These miss the attached 44×44 touch-target baseline even
though axe reports no serious issue.

### Low — 200% text on the 390 px layout introduces horizontal overflow

At 200% root text size, the page becomes 400 px wide in a 390 px viewport and
the right edge of the Privacy navigation link begins off-screen. Desktop at
1280 px did not overflow. Reflow the compact header at enlarged mobile text.

## Acceptance decision

**FAIL.** The repaired mobile axe, dark-theme contrast, offline shell, and
verification rate limit all pass fresh retesting. Release remains blocked by
the dead paid checkout, core false-PASS inputs, false Rust compatibility
statement, and incomplete privacy-claim test. The HTTP status and mobile touch
target defects should be corrected in the same repair cycle.
