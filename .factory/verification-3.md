# Independent verification 3 — FAIL

**Candidate:** `5b686d1e26666416dbfcd69f1a879a047f2b2bbe`  
**Live URL:** <https://ios-review-gate.sociobot.in>  
**Verified:** 2026-08-29 UTC, from the supplied clean checkout  
**Result:** **FAIL — claims-registry contract is incomplete.**

No product source code was changed during verification.

## First-read and demo gate

**PASS.** A cold live desktop page says what it does — “Check your iOS release before review”; who it is for — “small iOS teams”; and what to do first — **Try it with sample data**, with the outcome “See a checked release and its packet.” One click opens `/demo`, immediately showing the Harbor Log 2.4.0, build 108, PASS sample and the persistent “Demo — no personal data is saved” banner with Reset demo and Start for real.

## Required claims

`.factory/claims.json` exists. `npm ci` was run first, then every exact listed command was run against the bundled CLI sample or local browser demo as specified. All 11 commands passed:

| Claim | Result |
| --- | --- |
| identity-consistency | PASS — one seeded identity test passed |
| release-completeness | PASS — one seeded metadata/privacy/screenshot test passed |
| markdown-packet | PASS — dated Markdown packet test passed |
| bundled-cli-demo | PASS — fresh temporary workspace/packet test passed |
| queue-plan | PASS — active-submission/buffer test passed |
| cli-local | PASS — local-only/no-network-client test passed |
| one-click-demo | PASS — 1 Playwright test |
| browser-demo-local | PASS — 1 Playwright test |
| rust-msrv | PASS — `cargo +1.85.0 test --all-targets --locked`, 11 tests |
| license-restore | PASS — 1 recorded-verdict Playwright test |
| team-policy-download | PASS — 1 recorded-verdict/download Playwright test |

## Local product and package checks

- `npm test`: PASS. Rust, Node, build, and 16-Playwright-test stages completed; focused E2E runs also passed.
- `npm run build`: PASS — release CLI and `dist/site/` were produced. Vite reports JS 14.23 KB (5.46 KB gzip) and CSS 10.09 KB (3.16 KB gzip), under the 200 KB/50 KB limits.
- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo package`: PASS. Package verification produced 24 files, 181.3 KiB unpacked (96.6 KiB compressed).
- A clean `cargo install --path target/package/ios-review-gate-0.1.0 --root <temporary-root>` installed the public binary. `--help` worked; `demo --json` returned `passed:true` and a new temporary packet; missing input returned exit 1 with an actionable retry instruction.
- Independent normal input produced a dated PASS packet. A boundary fixture with blank app/owner, a non-image `.jpg`, and negative review/buffer days returned exit 2 with five specific errors and no packet path.

## Live deployment, privacy, accessibility, and resilience

- **Candidate parity: PASS.** SHA-256 hashes of live `index.html`, `sw.js`, `index-Cc8rM2SS.js`, and `index-DRkq_nZI.css` exactly match `dist/site/` built from this candidate.
- `/opt/fleet/lib/verify-url.sh https://ios-review-gate.sociobot.in` passed: HTTP 200, 798 ms, correct title/lang, one h1, main landmark, no missing alt text or unlabeled buttons, and no page/console error.
- Independent AxeBuilder 4.10.2 scans at 1440×900 and 390×844 in light and dark treatment covered `/`, `/demo`, `/privacy`, `/terms`, and the designed missing route. Every scan had zero serious or critical violations. Each ordinary route had exactly one h1 and one main; mobile width equalled scroll width. The deliberately requested missing route returned HTTP 404.
- Focus, dark-demo contrast, keyboard sample activation, mobile focusable code samples, 200% text reflow/44 px targets, and warmed offline demo behavior passed focused Playwright tests (4+1 passed). The offline test verifies the v3 shell includes emitted JS/CSS and reloads the complete sample offline.
- A fresh live `/demo` request log contained only same-origin document, JS, CSS, and terminal-recording asset requests. The CLI manifest/source has no runtime network client. The optional license path is separately documented to call only `https://api.sociobot.in` with a license token, never release files.
- Security/cache checks pass: HTTPS/HSTS, CSP with `frame-ancestors 'none'`, `nosniff`, Referrer-Policy, and Permissions-Policy are present. Hashed JS is one-year immutable and `sw.js` is `no-cache`.
- Product-unlock rate limiting: after a six-second clean window, 35 sequential invalid-token requests to the live verify endpoint returned 200 for requests 1–30; requests 31–35 returned **429** with `Retry-After` (3, 3, 2, 2, 2). Observed allowance is **30 requests per client/window**.

## Release-blocking defect

### High — visitor-reliant page claims are absent from `.factory/claims.json`

The claims policy requires every statement a visitor could rely on to have a named claim and one observable sandbox test; it explicitly makes an unlisted claim a failed review. The required existing claim tests all pass, but the following live landing statements have no matching registry entry/test:

- “Free checks and packets.” and “The free gate includes every core check and packet export.”
- “The same checker powers the command and the sample.”
- “Errors name the mismatch and the next fix.”
- “A Team license adds a local policy builder and queue histories beyond three submissions.”

For example, `markdown-packet` proves a packet is written but does not prove the all-core-checks/free-tier statement; `team-policy-download` proves a licensed YAML download but not the stated queue-history behavior; and no test ties the web recording/sample to the CLI checker artifact. This is a documentation/test-contract defect, not a failure of the exercised CLI paths.

**Required repair:** either remove/narrow these claims or add one claim entry per relied-on promise with an observable demo/CLI test (including a test that establishes the sample/recording provenance). Re-run all claims afterwards.

## Acceptance decision

**FAIL.** All product behavior, deployment parity, accessibility, privacy, offline behavior, package, and product-unlock rate-limit checks passed fresh. The candidate cannot pass the factory contract until its public claims are fully registered and testable.
