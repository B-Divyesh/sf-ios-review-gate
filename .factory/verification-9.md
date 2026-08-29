# Independent verification 9 — PASS

**Candidate:** `cbc52f2ffd3ac21e36c5f4bb629892c3dc51fbb3`  
**Live URL:** <https://ios-review-gate.sociobot.in>  
**Verified:** 2026-08-29 UTC from a clean checkout; no product code changed.  
**Decision:** **PASS — release candidate meets the acceptance contract.**

## First read and demo

A cold browser load says, in plain words, “Check your iOS release before
review,” identifies “small iOS teams,” and makes **Try it with sample data**
the first clear action. Its adjacent text says “See a checked release and its
packet.” One Enter press reaches `/demo`, focuses its h1, and shows the Harbor
Log 2.4.0 / build 108 PASS sample plus the persistent “Demo — no personal data
is saved” banner, Reset demo, and Start for real. This passes the first-read
and one-click sandbox gates.

## Required claims

After `npm ci` (21 packages, zero audit vulnerabilities), every exact command
in `.factory/claims.json` passed. The complete captured rerun ended
`FINAL_EXIT=0`.

| Claim IDs whose exact registered tests passed |
| --- |
| identity-consistency; release-completeness; markdown-packet; core-without-team-license; bundled-cli-demo |
| same-checker-demo; actionable-mismatch-errors; queue-plan; queue-input-validation; queue-date-limits |
| cli-local; cli-exit-codes; cli-json-schema; rust-msrv; team-queue-history |
| one-click-demo; browser-demo-local; license-restore; team-purchase; team-policy-download |

The commands were the registered 15 `cargo test …` / Rust-1.85 command and
the five registered `npm run build:site && npx playwright test --grep
@claim:…` commands. Browser claim tests used the local demo entry point; the
CLI claim tests use only bundled sample data and temporary workspaces.

## Local build, package, and CLI behavior

- `npm test`: PASS — 16 Rust integration tests, 4 Node contract tests, and 22
  Playwright tests.
- `npm run build`: PASS — produces `target/release/ios-review-gate` and
  `dist/site/`.
- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and
  `cargo +1.85.0 test --all-targets --locked`: PASS.
- `cargo package --locked`: PASS — 24 files, 336.9 KiB unpacked / 198.0 KiB
  compressed.
- A new temporary Cargo consumer installed with `cargo install --path .
  --locked --root <temp>`. `--version`, help, and `demo --json` worked; the
  demo returned `passed:true`, eight checks, and a packet path. Missing inputs
  returned exit 1 with “Fix the path or file contents, then run the command
  again.”
- Independent failure-path exercise: changing only the sample archive version
  to `9.9.9` returned exit 2, `passed:false`, an `identity.version` finding
  naming both values and the repair, and a Markdown HOLD packet.

Production assets are within budget: initial JavaScript is 14,808 bytes
(5,626 gzip), CSS 10,353 bytes (3,235 gzip), self-hosted font 20,056 bytes,
and hero WebP 94,064 bytes.

## Live deployment, accessibility, privacy, and resilience

- Local candidate and deployment SHA-256 values match exactly for
  `index.html` (`9ae202…02fe3`), `index-BpPNF1CI.js`
  (`a20f94…a8a3c4`), `index-D32uCJc_.css` (`dacfb9…bfc262`), and `sw.js`
  (`1d0fff…e41720d53`).
- `/`, `/demo`, `/privacy`, and `/terms` return 200. An unknown route returns
  the designed 404 with status 404. All examined real routes have their own
  title, one h1, and one main landmark.
- Fresh Playwright + axe scans at 1440px light and 390px dark found zero
  serious/critical violations on home, demo, privacy, and terms. Mobile had no
  document horizontal overflow. The visible keyboard focus ring is a 3px
  vermilion outline; reduced motion sets the inspection animation to
  `0.00001s`. There were no console/page errors on the real routes. (The
  browser naturally logs the deliberately requested 404 response as a failed
  resource; it is not an application error.)
- `/opt/fleet/lib/verify-url.sh` passed: 200, title, `lang=en`, one h1, main,
  zero missing `alt`, zero unlabeled buttons, no console/page error, 707 ms
  measured load in that run.
- Landing and demo request logs contained only same-origin static requests.
  Demo storage remained isolated as documented; its only cache is the static
  `ios-review-gate-v5` service-worker cache. After warm-up and
  `registration.update()`, `/demo` reloaded offline with its title, h1, and
  PASS result, with no errors.
- HTTPS responses send HSTS, `X-Content-Type-Options: nosniff`, Referrer
  Policy, Permissions Policy, and header-delivered CSP with
  `frame-ancestors 'none'`. Hashed assets are immutable for one year and
  `sw.js` is `no-cache`.

The available Lighthouse 12.8 runner could not complete in this container:
the supplied Chromium tab crashed even with the expected root flags. This is a
runner limitation, not a page error (Playwright Chromium loaded the same live
page successfully). Its substitutes above cover the required semantic,
accessibility, error, request, cache, and bundle-budget checks.

## Paid endpoint and request allowance

The real checkout endpoint returns HTTP 303 to a hosted Dodo session. A real
invalid-token UI verification made only the documented `api.sociobot.in`
request, produced the recoverable “License not active” message, and logged no
browser error. The endpoint returns `Cache-Control: no-store`.

Rate limiting is enforced. A single-client 40-request concurrent verification
burst produced 29 HTTP 200 and 11 HTTP 429 responses, all denied responses
including `Retry-After: 2` or `3`. After the retry window, an immediate
single-client sequential run first received HTTP 429 on request 22 with
`Retry-After: 4`. Therefore the observed current-window allowance was 21
sequential requests; parallel admission varied as expected but denial and
retry guidance are present.

## Defects by severity

- Critical: none.
- High: none.
- Medium: none.
- Low: none.

The Lighthouse browser-start limitation above is a test-environment note, not
a product defect.
