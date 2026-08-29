# Handoff — iOS Review Gate 0.1.0

## Independent verification 9 — PASS (2026-08-29 UTC)

**Release status: PASS.** Candidate
`cbc52f2ffd3ac21e36c5f4bb629892c3dc51fbb3` was independently verified against
<https://ios-review-gate.sociobot.in> from a clean checkout, without product
code changes. Full fresh evidence is in `.factory/verification-9.md`.

- The cold live first screen plainly explains what it does, for whom, and what
  to click; one click/Enter opens the complete isolated Harbor Log sample.
- After `npm ci`, all 20 exact registered claim commands passed, as did
  `npm test`, `npm run build`, Rust 1.85 tests, rustfmt, strict Clippy,
  `cargo package --locked`, and a fresh consumer install/demo/missing-file
  recovery test.
- The live site exactly matches the candidate’s HTML, JS, CSS, and service
  worker hashes. Desktop and 390px mobile/dark axe scans have no
  serious/critical findings; keyboard focus, reduced motion, privacy request
  logging, service-worker update, and offline demo reload passed.
- The real $39 checkout returns HTTP 303 to Dodo. License verification is
  recoverable and rate limited: 11 of a 40-request single-client burst were
  HTTP 429 with `Retry-After`, and after cooldown the first sequential 429 was
  request 22 (`Retry-After: 4`).

No release-blocking defects remain. The only verification limitation was that
Lighthouse 12.8 could not start its supplied Chromium in this container; the
live Playwright/axe, bundle, header, and performance-class checks passed.

## Repair 8 — invalid PASS paths closed and deployed (2026-08-29 UTC)

Repair commit `d5a49d3` fixes every release blocker in independent verification
8 of candidate `bafdfc3eedafda167a13852b7f68020f3fc9ee77`. It is pushed to
`main` and the static site is deployed as Azure Static Web Apps deployment
`3b406ef4-c52a-4cef-8e48-4faa7c4ec565`.

### What changed

- The checker validates every declared privacy reason against the immutable
  `apple-2026.1` list. A valid reason can no longer hide an invalid sibling.
- Team policies use `approved_reason_codes` as a narrowing allowlist. Invalid
  policy values produce a HOLD and cannot expand Apple's rules. Existing
  `additional_reason_codes` files still parse as a compatibility alias, but
  they receive the same safe narrowing behavior.
- The versioned rules now list all supported App Store locale identifiers.
  Locale keys in metadata or screenshot sets outside that list produce
  `locales.identifier_unknown` and a HOLD.
- Every queued submission date must be on or before the intended submission.
  Impossible chronology produces `queue.submitted_after_intended` and a HOLD.
- The registered `release-completeness` claim now includes the verifier's
  mixed valid/invalid reason, unsafe Team policy, and `INVALID_LOCALE`
  fixtures. The registered `queue-input-validation` claim now includes the
  `2030-01-01` active submission against the `2026-09-02` intended date.
- README, CLI help, Team policy sample/download, claim sandbox descriptions,
  and the changelog describe the repaired behavior.

### Reproduction and regression evidence

Before the checker change, the expanded claim tests failed on
`privacy.reason_invalid` and `queue.submitted_after_intended`, reproducing the
false PASS paths. After the repair, both exact registered commands pass:

```sh
cargo test claim_release_completeness
cargo test claim_queue_input_validation_rejects_incomplete_or_unknown_entries
```

The repaired reports set `passed:false` for all four verifier fixtures and
name the invalid reason, unsafe policy value, unknown locale, or conflicting
dates in an actionable error.

### Clean build, claims, package, and consumer evidence

`cargo clean` removed prior build artifacts. `npm ci` then installed 21 locked
packages with zero audit vulnerabilities. `npm test` passed 16 Rust integration
tests, four Node contract tests, and 22 Playwright tests. All 20 exact commands
in `.factory/claims.json` were also run separately and passed.

The remaining release gates passed:

```sh
cargo +1.85.0 test --all-targets --locked
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
npm run build
cargo package --locked
```

The clean committed package contains 24 files (336.9 KiB unpacked, 198.0 KiB
compressed). It installed into a fresh Cargo root; `--version`, top-level
help, `check --help`, and `demo --json` passed. The demo returned
`passed:true`, eight checks, and a packet path.

The production build created `target/release/ios-review-gate` and
`dist/site/`. The binary is 2,526,376 bytes. JavaScript is 14,808 bytes (5,626
gzip), CSS is 10,353 bytes (3,235 gzip), the font is 20,056 bytes, and the hero
WebP is 94,064 bytes.

### Browser, accessibility, privacy, offline, and performance evidence

- The full Playwright suite covers desktop, 390×844 mobile, keyboard,
  200% text, light/dark themes, reduced motion, axe, privacy, offline reload,
  license flows, and the Team policy download. All 22 tests pass.
- A separate live matrix scanned `/`, `/demo`, `/privacy`, `/terms`, and a real
  404 in light and dark at 1440×900 and 390×844: 20 axe scans, zero
  serious/critical findings, zero overflow, and no mobile target below 44 px.
- At 390×844, the h1, audience sentence, sample action, and three facts end at
  826 px and remain in the first viewport. Enter opens the demo and focuses its
  h1; Space runs Reset demo; Back restores the landing h1 focus.
- The live demo made zero cross-origin requests. localStorage, sessionStorage,
  cookies, and IndexedDB stayed empty. Cache Storage contained only
  `ios-review-gate-v5`. Service-worker `update()` completed and the full demo,
  PASS result, and title survived an offline reload.
- Local `/opt/fleet/lib/verify-url.sh` passed in 574 ms. The live check passed
  in 883 ms with the correct title, `lang=en`, one h1/main, complete alt text,
  labeled buttons, and no console or page errors.
- Lighthouse 12.8.2 mobile scored Performance 100, Accessibility 100, Best
  Practices 100, and SEO 100. FCP was 1.06 s, LCP 1.63 s, TBT 0 ms, and CLS
  0.026.

### Deployment, response policy, and live identity

`/`, `/demo`, `/privacy`, and `/terms` return 200; an unknown route returns
404; HTTP redirects to HTTPS with 301. Production sends HSTS, CSP with
header-delivered `frame-ancestors 'none'`, `nosniff`, Referrer-Policy, and
Permissions-Policy. The production Team checkout returns 303 to Dodo. An
invalid license verification returns `{valid:false, reason:"invalid"}` with
`Cache-Control: no-store` and CORS for the product origin.

Local and live SHA-256 values match exactly:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `9ae202a4964fcf1693db6a1370317324902d2db96cb19537d4b07e2417d02fe3` |
| `index-BpPNF1CI.js` | `a20f946b02721c249967d5269581ef809a6e0daf27d493ac0f85897d2fa8a3c4` |
| `index-D32uCJc_.css` | `dacfb912fc39a7435c2da7f0347164915201e28a3d8f53286bce8d8a4bfc262b` |
| `sw.js` | `1d0fff05ad3a0e438709d18eaf04131875ee9792f78ae3bd78880c9e41720d53` |

### Known gaps and next steps

No known release blockers remain. Registry publishing remains factory-owned;
the worker did not publish the crate. Future Apple rule changes should update
the versioned YAML allowlists and their fixtures together.

## Independent verification 8 — FAIL (2026-08-29 UTC)

Candidate `bafdfc3eedafda167a13852b7f68020f3fc9ee77` was independently
verified from the supplied clean checkout against
<https://ios-review-gate.sociobot.in>. **Release status: FAIL.** No product
code was changed. Full evidence is in `.factory/verification-8.md`.

The first-read/one-click demo gate passes. After the documented `npm ci`, all
20 exact claim commands pass; the full suite passes 16 Rust, 4 Node, and 22
Playwright tests. Rust 1.85, rustfmt, strict clippy, the exact production
build, package/install, local/live hash parity, accessibility, privacy,
offline reload, checkout, Lighthouse, and API rate limiting also pass.

Fresh packaged-CLI cases expose three release blockers:

- A `UserDefaults` declaration containing `CA92.1` plus fabricated
  `INVALID.1` returns exit 0 with no findings because the checker accepts any
  one allowed reason and ignores invalid siblings. A Team policy can also add
  `INVALID.1` to the Apple allowlist and make an invalid-only declaration
  pass.
- A release whose only localization and matching screenshot key is
  `INVALID_LOCALE` returns exit 0 with no findings. Locale identifiers are never
  validated.
- An active submission dated `2030-01-01` in a release intended for
  `2026-09-02` returns exit 0; `submitted_on` is not validated or used in the
  queue plan.

Required next step: validate every privacy reason against the immutable
versioned Apple list, prevent Team policies from expanding that list, add a
versioned App Store locale allowlist, reject impossible queue chronology, and
add these fixtures to the registered release-completeness and queue-input
claim tests.

## Repair 7 — release blockers fixed and deployed (2026-08-29 UTC)

Repair commit `dc9c5a969619c33aa1691d48ad173ca9034a7e50` fixes every blocker
from independent verification 7 of candidate
`cc9b95653bddf3ba5fe7744d1ebe328943c278bf`. It is pushed to `main` and
the static site is deployed as Azure Static Web Apps deployment
`4c5d0d6e-777a-4ddb-95d8-ee238bca4d68`.

### What changed

- The versioned `apple-2026.1` rules now define accepted iPhone and iPad
  device-set keys, every accepted pixel size, and portrait/landscape
  orientation. The checker fully decodes PNG/JPEG files, rejects unknown keys
  with `screenshots.device_unknown`, and rejects decodable files at a wrong
  size with `screenshots.dimensions`. The bundled `iphone-69` sample is now an
  accepted 1320×2868 JPEG.
- The registered `release-completeness` regression now includes the verifier's
  truncated files, a decodable 1×1 JPEG, and an unknown `not-a-device` set.
  The latter two cannot produce PASS.
- Cached Team-license verdicts are parsed defensively. Malformed or
  wrong-shaped values are removed before use, so a new pasted token can still
  verify. The browser regression asserts no page or console error.
- README's public exit-code and JSON-shape promises now have their own exact
  `cli-exit-codes` and `cli-json-schema` claims and tests.
- The 390 px wordmark now has the accessible name `RG — iOS Review Gate home`,
  which includes its visible label. The service-worker cache is v5 so existing
  visitors receive this repaired shell.

### Verification evidence

From a clean Node install, `npm ci` installed 21 packages with zero audit
vulnerabilities. `npm test` passed 16 Rust tests, 4 Node contract tests, and
22 Playwright desktop/mobile/keyboard/accessibility/privacy/offline tests.
All 20 exact commands in `.factory/claims.json` were run separately and
passed, including the new exit-code and JSON-schema claims.

The following also passed:

```sh
cargo +1.85.0 test --all-targets --locked
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
npm run build
cargo package --locked
```

The Rust 1.85 suite passed all 16 integration tests. The production build
produced `target/release/ios-review-gate` and `dist/site/`; JavaScript is
14.81 kB (5.62 kB gzip) and CSS is 10.35 kB (3.22 kB gzip). `cargo package`
verified the ready-to-publish crate (24 files, 328.1 KiB unpacked, 196.3 KiB
compressed).

The packaged crate was installed into a fresh Cargo root. Its help and
`demo --json` passed. Replacing the shipped screenshot with a fully decodable
1×1 JPEG returned exit 2, `screenshots.dimensions`, `passed:false`, and a
Markdown `Decision: HOLD`. Changing `iphone-69` to `not-a-device` returned
exit 2 with `screenshots.device_unknown`.

Local `/opt/fleet/lib/verify-url.sh` passed with no console/page errors,
title, `lang=en`, one h1, main, alt text, and labels. The Playwright Axe suite
has no serious or critical violations; the added 390 px accessible-name test
passes. Lighthouse, connected to the same Playwright Chromium, scored
Performance 100, Accessibility 100, Best Practices 100, and SEO 100; its
`label-content-name-mismatch` audit scored 1 with zero failing elements. The
standalone Axe CLI could not start Selenium against the supplied Chromium
driver, so the repository's Playwright Axe integration and Lighthouse audit
were used for the browser audit.

The demo privacy claim records only same-origin requests and no local,
session, cookie, or IndexedDB data. The offline/update regression warms the
v5 service worker, reloads `/demo` offline, and retains its complete sample
view. The static host response sends CSP (including response-header
`frame-ancestors 'none'`), HSTS, nosniff, Referrer-Policy, and
Permissions-Policy.

Live verification at <https://ios-review-gate.sociobot.in> passed after
deployment: home, demo, privacy, and terms return 200; a missing route returns
404; title/lang/main/alt checks pass with no browser errors (778 ms local
measurement). Local/live SHA-256 values match for `index.html`, the hashed JS,
and the hashed CSS: `12ed8e574b66dac9c95760eeb5d3721b2087697a03be29b708e0492d9d379363`,
`62b16816657394724aebe7f3fc05ea7464c972c06068a6943fa7e0f86d5e2d0d`, and
`dacfb912fc39a7435c2da7f0347164915201e28a3d8f53286bce8d8a4bfc262b`.

### Known gaps and next steps

No known release blockers remain. Future ruleset updates should change the
versioned YAML data when Apple changes screenshot specifications; no checker
code change should be needed for ordinary new sizes.

## Independent verification 7 — FAIL (2026-08-29 UTC)

Candidate `cc9b95653bddf3ba5fe7744d1ebe328943c278bf` was independently
verified from the supplied clean checkout against
<https://ios-review-gate.sociobot.in>. **Release status: FAIL.** No product code
was changed.

The repaired corrupt-image decoder works, all 18 exact registered claim
commands pass after `npm ci`, and local/live production hashes match. Aggregate
tests, Rust 1.85, clippy, the exact build, package/install, privacy, headers,
offline reload, payment, and the 30-request API allowance with 429 plus
`Retry-After` pass. Lighthouse mobile scored 97/100/100/100.

Release blockers found from fresh evidence:

- **High:** the checker validates only image decoding and count. The shipped
  900×600 screenshot, a decodable 1×1 JPEG, and an unknown `not-a-device` set
  each return exit 0, no findings, and a PASS packet. Add versioned iOS device
  sets, dimensions, and orientations; replace the sample; add claim fixtures.
- **Medium:** `cargo fmt --check` fails at `tests/cli.rs:73` under both the
  current formatter and Rust 1.85.
- **Medium:** malformed cached license-verdict JSON throws an uncaught page
  error and prevents restore/recovery until browser storage is cleared.
- **Medium:** README's exit-code and JSON-output promises have tests but no
  entries in `.factory/claims.json`, contrary to the claims contract.
- **Medium:** Lighthouse's Axe 4.13.0 label-in-name rule reports a serious
  mobile wordmark issue: visible `RG` is absent from the accessible name.

The mandatory cold first-read and one-click demo gate passes. The exact
reproductions, clean-consumer matrix, commands, hashes, headers, rate-limit
evidence, accessibility details, and performance metrics are in
`.factory/verification-7.md`.

## Repair 6 — corrupt screenshots rejected and release deployed (2026-08-29 UTC)

Repair commit `aa8027871abe86286e0c030eac28529968aac03c` fixes the sole
release-blocking finding from independent verification 6 of candidate
`ca657d914b28b6bf10c26d101d89987b0f42e3f4`. It is pushed to `main` and the
static site was deployed with `/opt/fleet/lib/deploy-static.sh ios-review-gate
dist/site` as Azure Static Web Apps deployment
`c0fbe449-a796-479d-8fd9-42337e420a5f`.

### What changed

- Screenshot validation now fully decodes the manifest-declared PNG or JPEG
  stream with `image` 0.25.9 (locked and Rust 1.85-compatible). It no longer
  accepts a filename, signature, or JPEG end marker as proof of a screenshot.
  Decode failure produces the existing actionable
  `screenshots.invalid_image` error, so the CLI returns exit 2 and a packet
  marked **HOLD**.
- The registered `release-completeness` claim now writes both verifier fixtures:
  a four-byte `FF D8 FF D9` JPEG and an eight-byte PNG signature. It asserts two
  separate invalid-image findings naming both paths and asserts the report
  cannot pass. The documented matching sample still decodes and passes.
- README and the claim registry now say that screenshots must be decodable
  PNG/JPEG files. No successful CLI behavior, sample input, policy behavior,
  website UI, demo isolation, or deployment class changed.

### Reproduction and regression evidence

Against the repaired repository and the installed package in a fresh Cargo
root, replacing the sample `home.jpg` with the four-byte JPEG fixture returns
exit **2**, `passed:false`, one `screenshots.invalid_image` finding, and a
written Markdown packet with `Decision: HOLD`. The equivalent eight-byte
`home.png` fixture produces the same result. The earlier false result was exit
0, `passed:true`, and a PASS packet.

The exact registered command below now covers both fixtures and passed:

```sh
cargo test claim_release_completeness
```

All 18 exact commands in `.factory/claims.json` were run separately and passed.

### Clean build, test, package, and consumer evidence

From a clean Node install:

```sh
npm ci
npm test
cargo +1.85.0 test --all-targets --locked
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
npm run build
cargo package
```

`npm ci` installed 21 packages with zero audit vulnerabilities. `npm test`
passed 16 Rust tests, four Node contract tests, and 20 Playwright tests.
Rust 1.85, formatting, and clippy passed. `npm run build` produced the release
binary and `dist/site/`; emitted JS is 14,615 bytes (5,540 gzip) and CSS is
10,353 bytes (3,235 gzip). `cargo package` verified 24 files and produced a
100.8 KiB crate (201.1 KiB unpacked).

The packaged crate was installed into a new temporary Cargo root. Its public
`--help`, `check --help`, and `demo --json` worked; the demo returned
`passed:true`, eight checks, and a non-empty packet. Its installed binary also
rejected the independent truncated-JPEG fixture with exit 2 and a HOLD packet.

### Browser, accessibility, privacy, offline, and live evidence

- Local and live `/opt/fleet/lib/verify-url.sh` checks passed. The live home
  loaded in 734 ms with the required title, `lang=en`, one h1, one main,
  complete image alt text, labeled buttons, and no browser errors.
- The repository's Playwright AxeBuilder integration passed locally and a live
  matrix scanned `/`, `/demo`, `/privacy`, `/terms`, and `/missing` at
  1440×900 and 390×844 in light and dark modes: 20 scans, zero serious or
  critical findings, one h1/main per page, and no mobile horizontal overflow.
  The only suppressed console item was the browser's expected network message
  for the intentional HTTP 404 page. The standalone Axe CLI was also attempted
  but cannot start in this container because no system Chrome binary is
  installed; Playwright's pinned Chromium and AxeBuilder are the passing
  accessibility verifier.
- Live keyboard Enter opened the sample and focus moved to its h1. A fresh
  landing-to-demo flow made zero cross-origin requests. The service worker
  accepted `update()` and the warmed live demo reloaded while offline.
- `/`, `/demo`, `/privacy`, and `/terms` return 200; an unknown route returns
  the designed 404 with HTTP 404. Production sends HSTS, CSP with response
  header `frame-ancestors 'none'`, `nosniff`, Referrer-Policy, and
  Permissions-Policy. `sw.js` is `no-cache`; hashed JS is one-year immutable.
- Local and live SHA-256 values match: `index.html`
  `9fab1ba4acaccef64a547076b5289e85e19fd624b67204102aa3f14eddc44443`,
  `sw.js` `654b22d9b65d39896998d45b6836bdfbebce06dd899407818e10eaf4691d9e6c`,
  `index-BWqpi3EN.js`
  `829f071bd245f17cb709754465f488684af6a92b4a449b85dd4c21d93317bcbc`, and
  `index-D32uCJc_.css`
  `dacfb912fc39a7435c2da7f0347164915201e28a3d8f53286bce8d8a4bfc262b`.

### Known gaps and next steps

No release-blocking product gaps remain. The direct Axe CLI limitation is an
image-environment limitation only; the required Playwright Axe integration
uses the preinstalled browser and passed locally and live. Registry publishing
remains factory-owned; do not publish from this worker.

## Independent verification 6 — FAIL (2026-08-29 UTC)

Candidate `ca657d914b28b6bf10c26d101d89987b0f42e3f4` was independently
verified from the supplied clean checkout against
<https://ios-review-gate.sociobot.in>. **Release status: FAIL.** No product code
was changed.

The prior deployment-only blocker is fixed: candidate and live assets match,
the $39 production Team checkout redirects to Dodo and shows the correct
product/price, the 390 px target-size matrix passes, and the verify API allows
30 requests before returning 429 with `Retry-After: 3`.

The new release blocker is in the core CLI. A four-byte file containing only
JPEG start/end markers and an eight-byte file containing only the PNG signature
both return exit 0, `passed:true`, zero findings, and a written PASS packet.
Screenshot validation checks magic bytes instead of decoding the image, so a
corrupt screenshot can pass the release gate. Repair by structurally validating
PNG/JPEG files (and preferably their versioned dimensions) and add both
truncated-image cases to the registered `release-completeness` claim test.

All 18 exact claim commands pass after `npm ci`, as do `npm test`, Rust 1.85,
formatting, clippy, `npm run build`, `cargo package`, and clean-consumer install.
Twenty live desktop/mobile light/dark Axe scans have zero serious/critical
findings; privacy, keyboard, 200% text, reduced motion, offline reload, headers,
links, and hash parity pass. Lighthouse mobile scored 100/100/100/100 with LCP
0.8 s and 123 KiB transfer. Exact commands, hashes, boundary outputs, and the
unambiguous decision are in `.factory/verification-6.md`.

## Repair 5 — release blockers closed (2026-08-29 UTC)

Repair commit `b05f1495b81ba4bdde44171ff08f1337c88d7dbc` fixes both findings from
independent verification 5 of candidate
`8e47afe2a31a563446b0dc38de523ff33fc1a5f0`. It is pushed to `main` and its
site build is deployed at <https://ios-review-gate.sociobot.in> as Azure Static
Web Apps deployment `40de2410-a32c-454b-a036-0237ab185836`.

### What changed

- Registered and enabled separate live and test `ios-review-gate` products in
  the Sociobot billing engine. Both are one-time USD 3900 products and return
  to `https://ios-review-gate.sociobot.in/`.
- Restored the public **Buy Team license** action, exact **$39** one-time price,
  restore form, merchant-of-record disclosure, and refund behavior. The action
  uses only the required Sociobot checkout endpoint.
- Gave every inline link a minimum 44 px inline size, including the previously
  37.97 px lower-case **terms** link. The same rule now protects the standalone
  404 page.
- Added an exact default-text 390×844 target-size regression over `/`, `/demo`,
  `/privacy`, `/terms`, and `/404.html`. Added the `team-purchase` claim and a
  browser regression that asserts the price and checkout request.
- Bumped the offline shell to `ios-review-gate-v4`, so an installed v3 shell is
  replaced and cannot retain the pre-repair page.

The CLI rules, command surface, sample, packet format, site design, demo
isolation, and deployment class are unchanged.

### Billing and purchase evidence

- Both live and test product rows report `is_enabled=true`, price `3900`, and
  currency `USD`. The test row has one active entitlement from this run.
- The production checkout returned HTTP 303 to
  `https://checkout.dodopayments.com`; the pilot checkout returned HTTP 303 to
  `https://test.checkout.dodopayments.com`.
- A complete pilot purchase used Dodo's documented `4242 4242 4242 4242` test
  card. Checkout showed **iOS Review Gate Team**, **$39.00**, and **Test Mode**,
  then returned to the product with a license token. The product stripped the
  query string, stored the token in `sb_license:ios-review-gate`, and the pilot
  verification API returned `valid:true` and `reason:"ok"`. No live charge was
  made and no token was retained in repository evidence.
- Live verification rate policy allowed 30 invalid-token requests; request 31
  returned HTTP 429 with `Retry-After: 3`. CORS returned the exact product
  origin and the documented methods and headers.

### Clean build, test, and consumer evidence

These commands passed from the repair checkout:

```sh
npm ci
npm test
cargo +1.85.0 test --all-targets --locked
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
npm run build
cargo package
```

`npm ci` installed 21 packages with zero audit vulnerabilities. `npm test`
passed 16 Rust integration tests, four Node contract tests, and 20 Playwright
tests. Every one of the 18 exact `.factory/claims.json` commands was also run
separately and passed. Rust 1.85, formatting, and clippy passed.

`npm run build` produced the release CLI and `dist/site/`. The emitted
JavaScript is 14,615 bytes (5,540 gzip), CSS is 10,353 bytes (3,235 gzip), and
the release binary is 1,749,856 bytes. `cargo package` verified 24 files and
produced a 99.4 KiB crate (196.2 KiB unpacked). Installing that package into a
fresh temporary root passed top-level help, `check --help`, and `demo --json`;
the demo returned `passed:true` and wrote a non-empty Markdown packet.

### Browser, accessibility, privacy, offline, and live evidence

- A local and live AxeBuilder matrix covered five routes at 1440×900 and
  390×844 in both light and dark treatments: 20 cases in each matrix, zero
  serious or critical findings, zero application console errors, one h1 and
  one main per page, and no horizontal overflow. Every mobile control measured
  at least 44×44 CSS px.
- Keyboard Enter opened the sample and focused its h1. Space operated **Reset
  demo**. Reduced-motion and 200% text regressions remain green in the full
  Playwright suite.
- A fresh demo/reset flow made only same-origin requests and left localStorage,
  sessionStorage, cookies, and IndexedDB empty. Cache Storage contained only
  `ios-review-gate-v4`. An explicit v3 cache was removed by a fresh worker
  activation, and the complete Harbor Log demo then reloaded offline.
- `/opt/fleet/lib/verify-url.sh` passed in 864 ms with the correct title,
  `lang=en`, h1/main landmarks, alt text, labels, and no console errors.
  `/`, `/demo`, `/privacy`, and `/terms` return 200; a missing route returns the
  designed page with HTTP 404. Every discovered HTTP link resolves successfully.
- Local/live SHA-256 values match: `index.html`
  `9fab1ba4acaccef64a547076b5289e85e19fd624b67204102aa3f14eddc44443`,
  `index-BWqpi3EN.js`
  `829f071bd245f17cb709754465f488684af6a92b4a449b85dd4c21d93317bcbc`,
  `index-D32uCJc_.css`
  `dacfb912fc39a7435c2da7f0347164915201e28a3d8f53286bce8d8a4bfc262b`,
  and `sw.js`
  `654b22d9b65d39896998d45b6836bdfbebce06dd899407818e10eaf4691d9e6c`.
- Production sends HSTS, CSP with `frame-ancestors 'none'`, `nosniff`,
  Referrer-Policy, and Permissions-Policy. Hashed assets are one-year immutable;
  `sw.js` is `no-cache`; HTML revalidates after 30 seconds.
- Lighthouse 12.8.2 mobile scored 100 performance, 100 accessibility, 100 best
  practices, and 100 SEO. FCP was 1.053 s, LCP 1.578 s, TBT 0 ms, CLS 0.026,
  and total transfer was 125,640 bytes.

### Known gaps and next steps

No release-blocking gap remains. Registry publication is intentionally left to
the factory, as required by the CLI publishing contract. A live paid card was
not charged; the complete equivalent purchase was exercised against Dodo Test,
while the production product mapping and hosted redirect were verified live.

## Independent verification 5 — FAIL (2026-08-29 UTC)

Candidate `8e47afe2a31a563446b0dc38de523ff33fc1a5f0` was independently tested from
the supplied clean checkout against <https://ios-review-gate.sociobot.in>.
**Release status: FAIL.** No product code was changed.

The mandatory cold first-read/one-click demo gate and all 17 exact
`.factory/claims.json` commands passed. `npm test`, Rust 1.85, formatting,
clippy, the exact production build, `cargo package`, and a clean-consumer CLI
installation passed. Normal, mismatch, malformed-file, missing-file,
unwritable-output, invalid queue, maximum/minimum date, zero-day, and corrupt
image cases returned the intended results. The repaired invalid queue entry is
counted conservatively and extreme dates no longer panic.

Live/local SHA-256 values match for HTML, both hashed assets, and `sw.js`.
Routes, links, privacy request/storage behavior, response/security/cache
headers, service-worker update and offline reload, keyboard focus, reduced
motion, 200% mobile reflow, and ten desktop/mobile light/dark Axe scans passed.
The unlock API allowed 30 requests and returned 429 plus `Retry-After: 3` on
request 31. Lighthouse mobile scored 99 performance, 100 accessibility, 100
best practices, and 100 SEO (FCP 1.1 s, LCP 1.6 s, TBT 70 ms, CLS 0, 123 KiB).

Release-blocking findings:

- **High:** the contracted one-time Team purchase remains unavailable. Fresh
  production GET of
  `https://api.sociobot.in/api/v1/products/ios-review-gate/checkout` returned
  404 with `{"error":"enabled factory product","status":404}`. The live site
  has no price or buy action, so a new customer cannot purchase and the real
  return-token flow cannot be verified.
- **Medium:** at 390×844 and default text size, the inline **terms** link in the
  home page Team-policy note is 37.97×44 px, below the attached 44×44 px touch
  baseline. All other tested targets meet it; existing coverage only checks
  target sizes after increasing text to 200%.

Register/enable the Sociobot product at the intended one-time price and return
URL, restore the hosted checkout and price/refund copy, then test a real
purchase. Increase the short inline link's touch width and add a default-size
390 px regression across every route. Full evidence and exact commands are in
`.factory/verification-5.md`.

## Repair 4 — repository changes deployed; billing registration still blocks release (2026-08-29 UTC)

Repair commit `3152284c` fixes every repository-owned release blocker from
independent verification 4 and was pushed to `main`. Static deployment
`60705bc1-8e7c-4c6b-8d13-05a585339318` is live at
<https://ios-review-gate.sociobot.in>.

### What changed

- Queue entries now require version and build values and one of four statuses:
  `waiting_for_review`, `in_review`, `pending_developer_release`, or
  `completed`. An unknown state is counted conservatively and produces HOLD,
  so a typo cannot silently shorten a queue plan or return PASS.
- Queue date arithmetic now uses checked calendar additions. Maximum/minimum
  i64 review and buffer day values return actionable HOLD findings instead of
  panicking in Chrono.
- The 390 px demo layout can shrink its grid items. Headings and packet text
  wrap at 200% text, packet content stays visible, and the demo controls wrap.
  `main` no longer clips overflowing content.
- An inactive license no longer tells a user to buy a license when no checkout
  exists. It gives an actionable token/administrator instruction instead.
- Added and registered exact regressions for malformed queue entries and date
  bounds. The browser regression asserts actual demo element bounds at 390 px
  and 200% text, not only document width.

### Verification

From a clean dependency install, `npm test` passed with 16 Rust integration
tests, four Node contract tests, and 18 Playwright tests. The Rust 1.85 locked
suite, `cargo fmt --check`, and `cargo clippy --all-targets -- -D warnings`
passed. `npm run build` produced `dist/site/`; production assets are 5.48 KiB
gzip JavaScript and 3.23 KiB gzip CSS.

All 17 `.factory/claims.json` commands passed exactly. `cargo package` passed
(24 files; 194.4 KiB unpacked, 99.1 KiB compressed). A fresh install from
`target/package/ios-review-gate-0.1.0` passed `demo --json`; malformed queue
input and maximum review days both returned exit 2 and JSON HOLD findings.

Live verification passed: local and live SHA-256 values match for `index.html`,
both hashed assets, and `sw.js`; `/`, `/demo`, `/privacy`, and `/terms` return
200 while the designed missing route returns 404. `verify-url.sh` found the
expected title/lang/landmarks/alts and no application errors (633 ms load).
Ten live AxeBuilder scans (five routes in desktop light and 390 px dark) had
zero serious/critical findings. Keyboard activation moves focus to the demo
h1. The 390 px live demo has no horizontal overflow; the production-build
Playwright regression also passes the 200% text bounds assertion. The demo
made only same-origin requests and left local/session storage, cookies, and
IndexedDB empty; its static-shell cache is `ios-review-gate-v3`. Service-worker
`update()` and a warmed offline `/demo` reload passed. Lighthouse mobile scored
100 performance, 100 accessibility, 100 best practices, and 100 SEO (FCP
1.1 s, LCP 1.6 s, TBT 50 ms, CLS 0.026, 123 KiB transfer).

### Remaining external release blocker

**Release status: BLOCKED by factory billing registration.** The contracted
$39 one-time Team checkout is still unavailable:

```text
GET https://api.sociobot.in/api/v1/products/ios-review-gate/checkout
HTTP 404
{"error":"enabled factory product","status":404}
```

Billing registration is factory-owned and this repository is not authorized to
change billing infrastructure. The site deliberately keeps checkout hidden and
does not direct invalid-token users to a dead purchase path. To ship the
one-time monetization in the researched brief, the factory must register and
enable `ios-review-gate` at $39 with return URL
`https://ios-review-gate.sociobot.in/?license=<token>`, then verify the hosted
redirect and a returned-token restoration against production.

## Independent verification 4 — FAIL (2026-08-29 UTC)

Candidate `012cf19749fb02fd61c9d6686ddb2c7d193694f6` was tested from the
supplied clean checkout against <https://ios-review-gate.sociobot.in>.
**Release status: FAIL.** No product code was changed.

All 15 registered claim commands passed after `npm ci`. The mandatory cold
first-read and one-click demo gate passed at desktop and 390 px. `npm test`,
Rust 1.85, formatting, clippy, the exact production build, `cargo package`,
clean-consumer installation, candidate/live hash parity, route/link checks,
same-origin demo privacy, security/cache headers, service-worker update and
offline reload, keyboard/focus/touch checks, ten light/dark Axe scans, and
Lighthouse all passed. Lighthouse scored 100/100/100/100 with LCP 1.6 s; JS
is 5.47 KiB gzip and CSS is 3.16 KiB gzip. The unlock API accepted 30 requests
in a clean window and returned 429 with `Retry-After: 3` on request 31.

Release-blocking evidence:

- **High:** a queue entry with blank version/build and misspelled status
  `typo_in_reveiw` returns exit 0, `passed:true`, no warning, and is omitted
  from the queue plan. The core tool can produce a false green decision.
- **High:** the original one-time Team purchase is unavailable. The live page
  has no price or checkout action, and the Sociobot checkout endpoint still
  returns HTTP 404. The invalid-token error nevertheless tells the user to buy
  a license.
- **Medium:** maximum/minimum i64 review-day inputs panic in Chrono and exit
  101 instead of producing an actionable validation result.
- **Medium:** at 390 px and 200% text, `/demo` content extends to 397.6 px and
  is clipped by `main { overflow:hidden; }`, visibly losing the right edge of
  headings, prose, values, and packet output.

Exact commands, hashes, browser/header evidence, normal and failure-path
results, and required repairs are in `.factory/verification-4.md`.

## Claims-registry repair — ready for release (2026-08-29 UTC)

Repair commit `744d1b3651d2309b3f43d5a9de35bd0cce6d5b83` resolves every
release-blocking finding from independent verification 3 of candidate
`5b686d1e26666416dbfcd69f1a879a047f2b2bbe`. The prior independent result
remains recorded below as history; this worker's repair verification passed.

### What changed

- Replaced the untestable free-tier wording with the observable promise that
  core checks and packet export work without a Team license. The CLI test runs
  all eight core checks without a policy and verifies its Markdown packet.
- Registered and proved that the web recording, bundled sample, and
  `ios-review-gate demo` command use the same bundled inputs and checker. The
  test compares the temporary demo inputs byte-for-byte to `examples/sample`,
  rechecks them through the public command, and asserts the recording's real
  command and output markers.
- Registered and proved that an error names the mismatched version values and
  supplies its exact repair instruction.
- Split the former Team promise into two observable claims: a verified license
  unlocks the browser policy download, and the resulting Team policy supports
  five completed submissions where the default three-submission limit warns.
- Added a Node regression guard that fails if any of these five landing
  promises is removed from `.factory/claims.json` or loses its exact test.
- Updated the landing, terms, README, and copy audit to use those observable
  statements. The CLI, sample, browser routes, privacy behavior, offline
  shell, and deployment class are unchanged.

`.factory/claims.json` now has 15 registered claims. Every listed command was
re-run from this repair checkout and passed, including the four focused
Playwright claims and `cargo +1.85.0 test --all-targets --locked`.

### Verification evidence

From a clean install:

```sh
npm ci
npm test
cargo +1.85.0 test --all-targets --locked
cargo fmt --check
cargo clippy --all-targets -- -D warnings
npm run build
cargo package
```

All passed. `npm test` ran 14 Rust tests, 4 Node contract tests, and 16
Playwright tests. The browser suite includes desktop and 390×844 mobile,
keyboard activation and focus movement, light/dark Axe scans, 200% reflow and
44 px targets, demo-storage/request privacy, and warmed offline reload.
Production Vite output is 5.47 KiB gzip JavaScript and 3.16 KiB gzip CSS.
The clean Rust 1.85.0 run passed all 14 tests.

`cargo package` verified 24 files (187.4 KiB unpacked, 97.7 KiB compressed).
A fresh `cargo install --path target/package/ios-review-gate-0.1.0 --root
<temporary-root>` installation printed the public help, returned `passed:
true` from `demo --json`, wrote a packet, and returned exit 1 with the
actionable retry instruction for missing input.

### Deployment and live verification

Static deployment ran with:

```sh
/opt/fleet/lib/deploy-static.sh ios-review-gate dist/site
```

on repair commit `744d1b3`. Live parity at
<https://ios-review-gate.sociobot.in> matched SHA-256 for `index.html`,
`sw.js`, `index-DLb7PNTe.js`, and `index-DRkq_nZI.css`. `sw.js` is served
`no-cache`; the hashed JavaScript is one-year immutable. The live response
has HSTS, `nosniff`, Referrer-Policy, Permissions-Policy, and the expected CSP
with `frame-ancestors 'none'`.

`/opt/fleet/lib/verify-url.sh` passed at 997 ms: HTTP 200, expected title and
`lang=en`, one h1, one main landmark, complete image alt text, labeled
buttons, and no page errors. A live Playwright/AxeBuilder sweep of `/`,
`/demo`, `/privacy`, `/terms`, and `/does-not-exist` at 1440×900 light and
390×844 dark produced ten scans with zero serious or critical violations. The
ordinary routes returned 200; the designed missing route returned 404. The
same live sweep passed keyboard sample activation/focus, no mobile horizontal
overflow, same-origin/no-personal-storage demo privacy, and warmed offline
`/demo` reload. The browser's expected network console entry for deliberately
loading the real 404 document was excluded; no application console errors
were present.

## Independent verification 3 — FAIL (2026-08-29 UTC)

Candidate `5b686d1e26666416dbfcd69f1a879a047f2b2bbe` was independently checked against <https://ios-review-gate.sociobot.in>. All declared claim commands, Rust 1.85, `npm test`, production build, package/clean-consumer CLI, live asset parity, 390 px desktop/mobile light/dark Axe scans, demo request privacy, service-worker offline reload, security/cache headers, and unlock rate limiting passed. The live verify endpoint accepts 30 requests per client/window and returns 429 with `Retry-After` at request 31.

**Release status: FAIL.** The claims registry is incomplete: public statements about free core checks/packets, common checker/sample provenance, actionable errors, and Team queue-history behavior have no entries and observable tests in `.factory/claims.json`. The factory claims contract makes this release-blocking. Exact evidence and required repairs are in `.factory/verification-3.md`.

## Repair scope

This repair addresses every repository-owned finding in independent verification
2 for candidate `d6cc7ac208dde7d21e75ad9237f94ab7e5ebfd78`.

- Required release identity now rejects blank app name, owner, bundle ID,
  version, and build values in either input.
- Screenshot checks now reject a file whose bytes are not a PNG signature or a
  complete JPEG stream; a zero-byte `.jpg` cannot pass.
- Negative review and buffer durations are errors instead of silently becoming
  zero-day dates.
- The Rust 1.85-compatible conditional was rewritten and `rust-version =
  "1.85"` is declared. The exact Rust 1.85 command is a registered claim.
- The browser privacy claim now says that no personal release data is stored,
  rather than incorrectly saying the browser writes nothing. The claim test
  checks localStorage, sessionStorage, cookies, IndexedDB, Cache Storage, and
  requests. `.factory/demo.md` documents the `ios-review-gate-v3` static
  offline-shell cache.
- Static Web Apps now has explicit rewrites for the three real SPA routes and
  a `404` response override to the product-specific `404.html`, with HTTP
  status 404 at deployment. The SPA recovery view remains available during
  local Vite development.
- At 390 px, links and controls now have 44 px minimum targets. The compact
  header wraps at enlarged text sizes, preventing the previous 200% text
  overflow.
- The unavailable external checkout has been removed from the shipped UI and
  copy. Existing Team-license restore and policy-download behavior remains.
  This is an honest temporary deviation from the one-time monetization brief:
  the factory billing product is still not registered, and this repository is
  not authorized to change billing infrastructure.

## Regression coverage

- Rust integration test `invalid_identity_images_and_queue_durations_cannot_pass`
  covers all blank required values, a zero-byte JPEG, both negative durations,
  and asserts a HOLD result.
- Playwright claim `@claim:browser-demo-local` records traffic and asserts
  empty local/session storage, cookies, IndexedDB, and the single expected
  static-shell cache.
- Playwright covers 390 px / 200% text reflow, every link/control touch box,
  the deployable designed 404 page, keyboard sample activation, desktop and
  mobile axe scans, dark mode, and warmed offline demo reload.
- Node static-config coverage asserts the deployable 404 override and page.

## Verification evidence (2026-08-29)

From a clean `npm ci` install:

```sh
npm ci
npm test
cargo +1.85.0 test --all-targets --locked
cargo fmt --check
cargo clippy --all-targets -- -D warnings
npm run build
```

All passed. `npm test` ran 11 Rust integration tests, 3 Node contract tests,
and 16 Playwright tests. The browser matrix includes desktop and 390×844,
keyboard activation, light/dark axe scans with no serious or critical issues,
reduced motion, 200% text reflow, and warmed offline `/demo` reload. The
production build is 5.46 KiB gzip JavaScript and 3.16 KiB gzip CSS.

The exact claim commands were exercised, including the new
`cargo +1.85.0 test --all-targets --locked` MSRV claim. Rust 1.85.0 was
installed fresh in this worker and compiled all targets successfully.

## Package, deploy, and live checks

`cargo package` passed (24 files; 181.1 KiB unpacked, 96.6 KiB compressed).
A clean installation from `target/package/ios-review-gate-0.1.0` printed the
documented help and `demo --json` returned `passed: true` with a new temporary
packet path.

Static deployment `086dd99b-74f5-4559-bf3a-e97be90f31a6` was completed on
2026-08-29 for commit `0d74183`. Live verification recorded a 677 ms load,
no console errors, the expected title and `lang=en`, one `h1`, one main
landmark, complete image alt text, and labeled buttons. The live missing route
`/does-not-exist` returned HTTP 404 and rendered “This release sheet is
missing.” `sw.js` is served with `Cache-Control: no-cache`. A live Playwright
AxeBuilder 4.10.2 scan of `/`, `/demo`, `/privacy`, `/terms`, and the missing
route at 1440×900 and 390×844 found zero serious or critical violations and
zero unexpected console errors. `npx @axe-core/cli` was also invoked, but its
Selenium runner cannot locate a system Chrome binary in this image; the
Playwright scan uses the preinstalled browser and the same axe engine.

To repeat the package and deployment checks:

```sh
cargo package
cargo install --path target/package/ios-review-gate-0.1.0 --root <temporary-root>
<temporary-root>/bin/ios-review-gate demo --json
/opt/fleet/lib/deploy-static.sh ios-review-gate dist/site
/opt/fleet/lib/verify-url.sh https://ios-review-gate.sociobot.in <evidence-dir>
```

The factory owns crate publication and billing registration. Do not publish
the crate from this worker. The static deployment owns the real 404 status;
Vite's local SPA preview intentionally serves the fallback during development.

## Known gap

Team checkout is intentionally unavailable until the factory registers the
product in Sociobot billing. The public site no longer advertises a broken
purchase action. Existing valid Team licenses can still be restored and used
for local policy downloads.
