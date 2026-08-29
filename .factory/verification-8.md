# Independent verification 8 — FAIL

**Candidate:** `bafdfc3eedafda167a13852b7f68020f3fc9ee77`  
**Live URL:** <https://ios-review-gate.sociobot.in>  
**Verified:** 2026-08-29 UTC from the supplied clean checkout  
**Result:** **FAIL — the CLI can issue PASS for invalid privacy reason declarations, invalid localization identifiers, and an impossible queue chronology.**

No product code was changed during verification.

## Release-blocking findings

### High — invalid privacy reason declarations can receive PASS

The release-completeness promise says the gate checks privacy declarations and
reason codes. The packaged public CLI returned exit 0, `passed:true`, and no
findings for this metadata entry:

```json
{"category":"UserDefaults","reasons":["CA92.1","INVALID.1"]}
```

`INVALID.1` is not in the candidate's own `apple-2026.1` allowlist. The
checker accepts an entry when **any** reason is allowed and never reports the
invalid sibling (`src/lib.rs:414-433`). A manifest with an approved value can
therefore hide an arbitrary value and still produce a PASS packet.

A Team policy is a second bypass. With metadata containing only
`INVALID.1`, and a policy adding `INVALID.1` under `UserDefaults`, the
packaged CLI again returned exit 0, `passed:true`, and no findings. The code
merges policy values into Apple's allowlist before validation
(`src/lib.rs:405-413`). A local team preference must not be able to declare an
arbitrary value Apple-approved.

Required repair: validate every declared reason against the versioned Apple
allowlist. Team policy may narrow or require organization choices, but it must
not expand Apple's allowed reason codes. Add both bypasses to the registered
`release-completeness` claim test.

### High — a nonexistent App Store locale receives PASS

The packaged CLI accepted a release whose only localization and screenshot
key was `INVALID_LOCALE`. It returned exit 0, `passed:true`, no findings, and a
valid queue result. The checker verifies localized field presence and lengths
but never validates the locale identifier (`src/lib.rs:436-477`). This breaks
the brief's localization-completeness gate: a packet can be green even though
its locale cannot be submitted.

Required repair: put supported App Store locale identifiers in the versioned
rules, reject unknown identifiers, and add an unknown-locale fixture to the
registered release-completeness claim.

### High — an impossible future queue entry receives PASS

A release intended for `2026-09-02` with an active submission dated
`2030-01-01` returned exit 0 and `passed:true`. The only finding was the normal
`queue.active` warning; the tool calculated an estimated decision of
`2026-09-06`. `submitted_on` is parsed but is not validated against the
intended submission or used in the plan.

This is invalid input, not a planning warning, and conflicts with the brief's
realistic queue-plan job. Reject impossible chronology before issuing PASS and
add it to `queue-input-validation`.

## Mandatory first-read and demo gate

**PASS.** A fresh 1440×900 browser context answered the required questions on
the first screen:

- What: “Check your iOS release before review.”
- Who: “For small iOS teams that need one reviewable packet before they queue a build.”
- First action: **Try it with sample data**, beside “See a checked release and its packet.”

At 390×844, the heading, audience sentence, action, explanation, and all three
facts are inside the first viewport. One click opens `/demo`, immediately
showing Harbor Log 2.4.0 build 108, `en-US`, Privacy Manifest present, PASS,
the packet preview, and the persistent demo banner with Reset demo and Start
for real.

## Required claims gate

`.factory/claims.json` exists with 20 entries. Following the literal order in
the work order, all exact commands were first invoked before dependency
installation. The 15 Rust-backed entries passed; the five browser-backed
entries exited 127 at `vite: not found`. After the documented `npm ci` setup,
all 20 exact commands were rerun and exited 0:

| Claim | Post-install result |
| --- | --- |
| identity-consistency | PASS |
| release-completeness | PASS, but under-covers the false-PASS cases above |
| markdown-packet | PASS |
| core-without-team-license | PASS |
| bundled-cli-demo | PASS |
| same-checker-demo | PASS |
| actionable-mismatch-errors | PASS |
| queue-plan | PASS |
| queue-input-validation | PASS, but under-covers future chronology |
| queue-date-limits | PASS |
| cli-local | PASS |
| cli-exit-codes | PASS |
| cli-json-schema | PASS |
| one-click-demo | PASS |
| browser-demo-local | PASS |
| rust-msrv | PASS — all 16 integration tests on Rust 1.85 |
| license-restore | PASS |
| team-purchase | PASS |
| team-policy-download | PASS |
| team-queue-history | PASS |

The passing registered tests do not override the independently observed
false-PASS behavior. The `release-completeness` claim is broader than its
fixtures prove.

## Clean local gates and exact production build

- `npm ci`: PASS — 21 packages installed, 0 audit vulnerabilities.
- `npm test`: PASS — 16 Rust integration tests, 4 Node contract tests, and 22
  Playwright tests.
- `cargo +1.85.0 test --all-targets --locked`: PASS — 16 integration tests.
- `cargo fmt --check`: PASS.
- `cargo clippy --all-targets --locked -- -D warnings`: PASS.
- `npm run build`: PASS — release binary and `dist/site/` produced.
- `cargo package --locked`: PASS — 24 files, 328.1 KiB unpacked and 196.3 KiB
  compressed.

There is no separate JavaScript lint or typecheck script. Production output is
within budget: JS 14,810 bytes (5.62 KiB gzip), CSS 10,353 bytes (3.22 KiB
gzip), font 20,056 bytes, hero WebP 94,064 bytes, and the release binary is
about 2.4 MB.

## Packaged CLI and independent cases

The verified crate at `target/package/ios-review-gate-0.1.0` was installed
into a new Cargo root. `--version`, top-level help, `check --help`, and
`demo --json` worked; demo created a new temporary workspace and packet.

| Case | Exit | Evidence |
| --- | ---: | --- |
| Shipped sample | 0 | PASS; dated 800-byte packet written |
| Fully decodable 1×1 JPEG | 2 | `screenshots.dimensions` |
| Unknown device set | 2 | `screenshots.device_unknown` |
| Localized name at 30 characters | 0 | PASS |
| Localized name at 31 characters | 2 | `locales.field_too_long` |
| Negative review days | 2 | `queue.review_days_negative` |
| Maximum i64 review days | 2 | `queue.review_days_out_of_range`; no panic |
| Zero review and buffer days | 0 | all three dates equal |
| Malformed metadata | 1 | parse location and repair instruction |
| Missing metadata | 1 | missing path and repair instruction |
| Missing output parent | 1 | write error and repair instruction |
| Malformed Team policy | 1 | parse location and repair instruction |
| Valid plus fabricated privacy reason | **0** | **false PASS; no findings** |
| Fabricated reason allowed by Team policy | **0** | **false PASS; no findings** |
| `INVALID_LOCALE` localization | **0** | **false PASS; no findings** |
| Active submission dated after intended release | **0** | **false PASS; warning only** |

## Candidate/live identity, routing, headers, and caching

The live site matches the exact candidate production build:

| Artifact | Matching local/live SHA-256 |
| --- | --- |
| `index.html` | `12ed8e574b66dac9c95760eeb5d3721b2087697a03be29b708e0492d9d379363` |
| `index-DQ9Mltt4.js` | `62b16816657394724aebe7f3fc05ea7464c972c06068a6943fa7e0f86d5e2d0d` |
| `index-D32uCJc_.css` | `dacfb912fc39a7435c2da7f0347164915201e28a3d8f53286bce8d8a4bfc262b` |
| `sw.js` | `1d0fff05ad3a0e438709d18eaf04131875ee9792f78ae3bd78880c9e41720d53` |

`/`, `/demo`, `/privacy`, and `/terms` return 200. An unknown route returns
the designed page with HTTP 404. HTTP redirects to HTTPS with 301. All
discovered internal links and the Param Factory link resolve.

HTML responses include HSTS, CSP with header-delivered `frame-ancestors
'none'`, `nosniff`, Referrer-Policy, and Permissions-Policy. HTML revalidates
after 30 seconds; hashed JS/CSS use one-year immutable caching; `sw.js` uses
`no-cache`.

## Browser, accessibility, privacy, PWA, and performance

- `/opt/fleet/lib/verify-url.sh` passed in 647 ms: correct title and language,
  one h1, one main, complete image alt text, labeled buttons, and no console or
  page errors.
- Independent AxeBuilder 4.10.2 scans covered `/`, `/demo`, `/privacy`,
  `/terms`, and a real 404 in light and dark at 1440×900 and 390×844: 20
  combinations, zero serious/critical findings, one h1/main each, no overflow,
  and no visible target below 44 px.
- Keyboard Tab reached the sample action after six presses. It had a visible
  3 px vermilion focus outline. Enter opened `/demo` and focused its h1; Space
  operated Reset demo; browser Back restored `/` and focused its h1.
- Reduced motion removes the mobile inspection line and sets animation to one
  0.01 ms iteration. No flashing or looping motion was found.
- A fresh demo/reset flow made same-origin requests only. localStorage,
  sessionStorage, cookies, and IndexedDB remained empty. Cache Storage held
  only `ios-review-gate-v5`, matching the disclosed offline shell.
- Service-worker `update()` completed. With an active controller, offline
  `/demo` reload retained its title, sample heading, and PASS result.
- Lighthouse 12.8.2 mobile: Performance 100, Accessibility 100, Best
  Practices 100, SEO 100; FCP 1.1 s, LCP 1.6 s, TBT 60 ms, CLS 0.026, and 123
  KiB transferred.

The site has no sign-in, product backend, analytics, third-party font/script,
or runtime AI. Entra, product-backend concurrency/persistence, and AI gateway
checks are not applicable.

## Billing and request allowance

- Production checkout returns 303 to `checkout.dodopayments.com`. The hosted
  page loaded “iOS Review Gate Team,” `$39.00`, and the one-time license
  description. No purchase was made.
- A live invalid-token restore called only the documented Sociobot endpoint,
  stored the token/verdict as disclosed, showed the inactive notice, and
  emitted no product-page error.
- CORS allows the product origin and documented methods/headers.
- After a six-second clean window, verification requests 1–30 returned 200;
  request 31 returned 429. Its headers included `Retry-After: 3` and
  `X-RateLimit-After: 3`. **Observed allowance: 30 requests per client/window.**

## Acceptance decision

**FAIL.** The deployment, build, package, demo, browser accessibility,
privacy, offline, payment, performance, and rate-limit checks pass. Release is
blocked because the product's core decision can still be green for privacy
reason declarations Apple cannot accept, a nonexistent localization, and an
impossible queue chronology. The broad release-completeness and queue-input
claims need regression fixtures for these cases.
