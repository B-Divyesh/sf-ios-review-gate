# Adversarial first-read review 1 — iOS Review Gate

Date: 2026-08-29 UTC

Candidate: `bfde49f0ead27b2140f31db3b7b1ba4343324e8e`

Live site: <https://ios-review-gate.sociobot.in>

Verdict: **FAIL**

The core CLI, demo, deployment, accessibility checks, and all 20 registered
claim commands pass. The release still fails the stated zero-finding standard:
public claims remain outside the claims registry, the desktop first screen
omits the three required facts, and several copy and route-contract defects
remain.

## Cold first read

I opened the live site in new browser contexts at 390×844 and 1440×900 before
scrolling.

| Question | First-read answer |
| --- | --- |
| What does this do? | It checks an iOS release before App Review and produces a review packet. |
| For whom? | Small iOS teams preparing to queue a build. |
| What should I click first? | **Try it with sample data**. |

The blocking first-read test therefore passes at both widths. The exact text
that provides the answer is “Check your iOS release before review”, “For small
iOS teams that need one reviewable packet before they queue a build”, and “Try
it with sample data”. At 390 px the action and all three facts are visible by
826 px. At 1440×900 the action ends at 893 px, but the facts start at 916.6 px;
that separate structure defect is F-1-13.

## Findings

### Blocking — claims registry is incomplete

Each sentence below is a claim a visitor can rely on, but no entry in
`.factory/claims.json` states and tests that claim. The earlier claims-registry
failure class has therefore recurred, although the specific promises repaired
after verification 3 remain covered.

#### F-1-1 — The offline-cache scope is an unlisted claim

- Quote/location: `/privacy`, “Its offline shell uses Cache Storage for static
  site files only.”
- Why this fails: `browser-demo-local` asserts the cache name, and an aggregate
  test exercises offline reload, but no registered claim states or verifies
  which responses the cache contains.
- Fix: add an `offline-shell` claim and a Playwright test that lists every
  cached URL, confirms all are same-origin static assets, then reloads `/demo`
  offline. Otherwise remove this sentence.

#### F-1-2 — The merchant-of-record statement is unlisted

- Quote/location: landing price note and README “Privacy and price”, “Dodo is
  the merchant of record for Sociobot purchases.”
- Why this fails: `team-purchase` checks the $39 label and Sociobot checkout
  URL but stubs the destination; it does not prove the merchant statement.
- Fix: register the claim and test that the live/test Sociobot checkout redirects
  to the expected Dodo host and identifies this product, or remove the name.

#### F-1-3 — Refund handling is an unlisted claim

- Quote/location: landing price note, “Dodo handles refunds”; README, “Dodo is
  the merchant of record for Sociobot purchases and handles refunds.”
- Why this fails: no claim or test observes the advertised refund route or
  policy.
- Fix: add a `refund-handling` claim with a stable billing-contract test and a
  user-visible refund path, or replace it with a tested support/contact route.

#### F-1-4 — Refund deactivation is an unlisted claim

- Quote/location: landing price note and `/terms`, “A refund deactivates the
  license.”
- Why this fails: none of the license tests applies a recorded refunded
  entitlement and confirms that verification becomes inactive.
- Fix: add a recorded refunded-license fixture and exact claim test, or remove
  the sentence.

#### F-1-5 — Binary publication is an unlisted claim

- Quote/location: README “Install”, “The factory publishes release binaries
  after review.”
- Why this fails: no claim checks that a downloadable release binary exists;
  the sentence also gives no download link.
- Fix: until binaries exist, write “Release binaries are not published yet;
  build the CLI with the command above.” When published, link them and add a
  clean download/smoke test.

#### F-1-6 — The exact package version is an unlisted claim

- Quote/location: README “Install”, “The package starts at version `0.1.0`.”
- Why this fails: the value is observable and version-specific, but no registry
  entry runs the installed binary with `--version`.
- Fix: add a `package-version` claim that compares Cargo metadata and installed
  CLI output, or omit the sentence because the package metadata already shows
  the version.

#### F-1-7 — Embedded rule versioning is only partly covered

- Quote/location: README “Input files”, “Rules are versioned in the binary and
  named in every packet.”
- Why this fails: `markdown-packet` checks that one packet names
  `apple-2026.1`; its registered claim does not state that rules are embedded
  and versioned in the binary.
- Fix: add an `embedded-rule-version` claim that runs an installed package
  outside the repository and asserts the rule version and decisions without a
  rules file, or narrow the sentence to the tested packet behavior.

#### F-1-8 — The aggregate test description is unlisted

- Quote/location: README “Develop and verify”, “`npm test` runs Rust unit and
  integration tests plus site tests.”
- Why this fails: the command passed in this review, but it is still absent
  from the claim registry.
- Fix: register a non-recursive build-contract test that inspects and exercises
  each underlying test script, or present the commands without making a prose
  coverage promise.

#### F-1-9 — The build-output statement is unlisted

- Quote/location: README “Develop and verify”, “`npm run build` produces the
  single binary in `target/release/` and the deployable site at `dist/site/`.”
- Why this fails: this review observed both outputs, but no claim entry owns
  that public build contract.
- Fix: add a `build-artifacts` claim that builds from clean state and asserts
  both artifacts, or remove the sentence.

#### F-1-10 — The static-host behavior is unlisted

- Quote/location: README “Develop and verify”, “The included Static Web Apps
  configuration supplies fallback, cache, and security headers.”
- Why this fails: the Node contract test parses the config, but no registered
  claim states or tests the deployed header/cache/fallback outcomes.
- Fix: register a `static-host-contract` claim covering deep-link fallback,
  404 status, cache policy, and all security headers, or remove the promise.

#### F-1-11 — The MIT-license statement is unlisted

- Quote/location: README “License”, “MIT.”; `/terms`, “The free CLI is
  available under the MIT License.”
- Why this fails: `LICENSE` exists, but the public legal statement has no
  claims entry or consistency check against package metadata.
- Fix: register a `license-metadata` claim that compares `LICENSE`,
  `Cargo.toml`, README, and terms, or consolidate the statement to one generated
  source.

### Major

#### F-1-12 — The CLI stops before the archive-import step

- Quote/location: landing “Write version, build, bundle ID, and privacy use to
  local JSON”; README, “`metadata.json` is a local export from your archive or
  IPA inspection step.”
- Why this fails: a first-time iOS team is told to create the most specialized
  input manually, but no command or documented tool creates it. The advertised
  job is checking a release, not checking a hand-authored intermediate file.
- Fix: add `ios-review-gate inspect --archive <path.xcarchive|ipa>` (or accept
  that path directly in `check`) to extract bundle/version/build and privacy
  declarations locally. Ship an archive fixture and claim-test the extracted
  JSON plus the resulting packet. An AI feature is not warranted here; a
  deterministic local importer is the useful missing step.

### Minor

#### F-1-13 — Desktop hides all three required facts below the first screen

- Quote/location: landing facts, “Runs on your machine”, “No App Store access”,
  and “Checks and packets need no Team license.”
- Why this fails: at 1440×900 the facts begin at 916.6 px. The site-structure
  contract requires privacy/offline/price facts on the first screen.
- Fix: reduce the desktop hero heading/vertical spacing or place the facts
  beside the action so all three end above 900 px. Add a desktop fold-boundary
  test, not only an assertion that the action is visible at 390 px.

#### F-1-14 — The deployed 404 omits required metadata and shared footer content

- Quote/location: live unknown route and `site/public/404.html`.
- Why this fails: it returns a correct designed HTTP 404, but has no canonical,
  Open Graph/Twitter metadata, or apple-touch icon. Its footer omits “Built by
  Param Factory” and the version/build ID, and its header omits Install. It is
  not the consistent route shell required by the site structure.
- Fix: give the standalone 404 the same metadata and header/footer contract as
  other routes, with canonical `/404`, while retaining `noindex` and HTTP 404.

#### F-1-15 — One README sentence exceeds the 22-word cap

- Quote/location: README “Input files”, 29 words: “Version `apple-2026.1`
  covers supported App Store locale identifiers, required localized fields,
  accepted PNG/JPEG screenshot device sets and pixel sizes, privacy manifest
  presence, collected-data agreement, and every declared reason code.”
- Why this fails: it compresses seven checks into one sentence.
- Fix: “Version `apple-2026.1` checks supported locales, localized fields,
  screenshot sets and sizes, and the privacy manifest. It also checks
  collected-data answers and every declared reason code.”

#### F-1-16 — Decorative drafting labels carry no user information

- Quote/location: landing labels “SHEET 01”, “PROCEDURE / 03”, “COMMAND / 01”,
  and “BOUNDARY / LOCAL”.
- Why this fails: these are product lore; the following headings already name
  each section. A screen-reader or skim reader learns nothing from them.
- Fix: delete these labels. Keep “Team policy”, which names a real concept.

#### F-1-17 — “Preflight” and “gate” add avoidable jargon

- Quote/location: “Local App Review preflight”, “See the gate result before
  submission”, “Run the gate in your repository”, README “runs the gate”, and
  “the gate passed”.
- Why this fails: the page alternates between gate, checker, and check for the
  same operation. “Preflight” assumes release-process vocabulary.
- Fix: use **check** for the operation and **checker** for the CLI. Rewrite the
  eyebrow as “Checks metadata, screenshots, privacy, and queue timing”; use
  “See the check result”, “Run the checker”, and “the check passed”.

#### F-1-18 — The output has five competing names

- Quote/location: landing/README use “review packet”, “decision packet”, “App
  Review packet”, “decision record”, and “dated Markdown file”.
- Why this fails: a first-time visitor cannot tell whether these are one output
  or several.
- Fix: call it a **Markdown review packet** on first mention and **review
  packet** thereafter. Reserve “decision” for the PASS/HOLD value inside it.

#### F-1-19 — The demo exit action does not name its result

- Quote/location: `/demo` banner, “Start for real”.
- Why this fails: it returns to the landing page; it neither starts the CLI nor
  says where the visitor will land.
- Fix: use “Install the CLI” and link directly to `/#install`, or “Return home”
  if navigation home is the intended result.

#### F-1-20 — The 404 action repeats the product metaphor

- Quote/location: 404, “Return to the gate”.
- Why this fails: it does not name the destination in ordinary navigation
  language.
- Fix: rewrite it as “Return home”.

#### F-1-21 — “Realistic” is subjective demo copy

- Quote/location: README “Try the bundled demo”, “The command copies realistic
  sample files…”
- Why this fails: “realistic” is an unmeasured marketing adjective; the sample
  already has a concrete name and contents.
- Fix: “The command copies the bundled Harbor Log files to a temporary
  directory, runs the check, and prints the review-packet path.”

#### F-1-22 — “Real checker” is vague emphasis

- Quote/location: `/demo`, “This recording comes from the shipped Harbor Log
  sample and the real checker.”
- Why this fails: “real” does not explain provenance; the registered claim is
  specifically that the browser recording and CLI use the same checker.
- Fix: “This recording uses the shipped Harbor Log sample and the same checker
  as the CLI.”

## Copy audit

Counts treat a hyphenated term, path, URL, version, or price as one word. Code
blocks and data examples are not sentences. Metadata sentences and meaningful
image alt text are included. Only README sentence 22 exceeds 22 words. No
banned plain-words term appears.

### Landing page sentences

| # | Words | Sentence |
| ---: | ---: | --- |
| 1 | 10 | Check iOS release metadata, screenshots, privacy answers, and queue timing. |
| 2 | 7 | Print a local review packet before submission. |
| 3 | 14 | For small iOS teams that need one reviewable packet before they queue a build. |
| 4 | 7 | See a checked release and its packet. |
| 5 | 4 | Runs on your machine. |
| 6 | 4 | No App Store access. |
| 7 | 7 | Checks and packets need no Team license. |
| 8 | 15 | An exploded drafting view connects an app archive, metadata sheets, screenshots, and a review queue. |
| 9 | 10 | The website sample and command use the same bundled checker. |
| 10 | 8 | Errors name the mismatch and the next fix. |
| 11 | 13 | Terminal output shows the bundled release passing every check and writing a packet. |
| 12 | 12 | The bundled demo checks a complete sample release and writes its packet. |
| 13 | 11 | Write version, build, bundle ID, and privacy use to local JSON. |
| 14 | 12 | List localized copy, screenshot paths, privacy answers, and queue timing in YAML. |
| 15 | 3 | Run one command. |
| 16 | 10 | Fix holds, review warnings, and keep the dated Markdown file. |
| 17 | 11 | Build the single Rust binary, then keep release.yaml beside the app. |
| 18 | 8 | The CLI reads the paths you give it. |
| 19 | 9 | It has no telemetry and sends no release data. |
| 20 | 13 | It does not upload builds, scrape App Store Connect, or predict Apple’s decision. |
| 21 | 10 | Core checks and packet export work without a Team license. |
| 22 | 8 | Verified Team licenses enable the local policy download. |
| 23 | 8 | Team policies support queue histories beyond three submissions. |
| 24 | 3 | Have a license? |
| 25 | 3 | Paste it here. |
| 26 | 11 | Build a shared policy file for queue limits and approved reason codes. |
| 27 | 4 | Verified on this device. |
| 28 | 3 | License not active. |
| 29 | 8 | Check the token or use Buy Team license. |
| 30 | 6 | Could not check the license. |
| 31 | 5 | Connect once and try again. |
| 32 | 6 | License removed from this browser. |
| 33 | 3 | Team policy downloaded. |
| 34 | 7 | Pass it to the CLI with --policy. |
| 35 | 4 | License no longer active. |
| 36 | 9 | Dodo is the merchant of record for Sociobot purchases. |
| 37 | 3 | Dodo handles refunds. |
| 38 | 5 | A refund deactivates the license. |
| 39 | 4 | See privacy and terms. |
| 40 | 9 | Check an iOS release and print its review packet. |

### README sentences

| # | Words | Sentence |
| ---: | ---: | --- |
| 1 | 14 | Check an iOS release and print a dated review packet before App Store submission. |
| 2 | 17 | It is for small iOS teams that want a repository-local decision record instead of another upload service. |
| 3 | 9 | The CLI compares an archive metadata export with release.yaml. |
| 4 | 14 | It checks version and build values, localized metadata, screenshots, privacy declarations, and queue timing. |
| 5 | 12 | It does not upload a build or connect to App Store Connect. |
| 6 | 9 | Build the single binary with Rust 1.85 or newer. |
| 7 | 6 | The package starts at version 0.1.0. |
| 8 | 7 | The factory publishes release binaries after review. |
| 9 | 18 | The command copies realistic sample files to a temporary directory, runs the gate, and prints the packet path. |
| 10 | 8 | It does not read or change your files. |
| 11 | 6 | The website demo is at https://ios-review-gate.sociobot.in/demo. |
| 12 | 7 | Exit code 0 means the gate passed. |
| 13 | 11 | Exit code 2 means the inputs were read but checks failed. |
| 14 | 8 | Invalid files or arguments use exit code 1. |
| 15 | 5 | Use JSON output in automation. |
| 16 | 10 | The JSON object contains passed, summary, findings, queue, and packet_path. |
| 17 | 12 | metadata.json is a local export from your archive or IPA inspection step. |
| 18 | 5 | release.yaml records the intended submission. |
| 19 | 5 | See examples/sample/release.yaml for every field. |
| 20 | 7 | Screenshot paths are relative to that file. |
| 21 | 11 | Rules are versioned in the binary and named in every packet. |
| 22 | **29** | **Version apple-2026.1 covers supported App Store locale identifiers, required localized fields, accepted PNG/JPEG screenshot device sets and pixel sizes, privacy manifest presence, collected-data agreement, and every declared reason code.** |
| 23 | 15 | Supported locale identifiers, screenshot set keys, and exact portrait or landscape sizes are in rules/apple-2026.1.yaml. |
| 24 | 14 | Use a listed locale such as en-US and a device key such as iphone-69. |
| 25 | 15 | Unknown identifiers, unknown device keys, and decodable files at the wrong dimensions produce a HOLD. |
| 26 | 7 | The bundled iphone-69 sample is 1320×2868 pixels. |
| 27 | 14 | Every reason in accessed_apis must appear in the Apple rules for its API category. |
| 28 | 15 | A Team policy can narrow that list with approved_reason_codes; it cannot make another code Apple-approved. |
| 29 | 17 | Each queued submission needs a version, build, submitted date, and one status: waiting_for_review, in_review, pending_developer_release, or completed. |
| 30 | 9 | Unknown or incomplete queue entries produce a HOLD result. |
| 31 | 16 | Review and buffer days must be zero or positive and must fit a real calendar date. |
| 32 | 9 | Each submitted_on date must be on or before intended_submission. |
| 33 | 11 | npm test runs Rust unit and integration tests plus site tests. |
| 34 | 15 | npm run build produces the single binary in target/release/ and the deployable site at dist/site/. |
| 35 | 11 | Deploy dist/site/ to any static host with SPA fallback to index.html. |
| 36 | 12 | The included Static Web Apps configuration supplies fallback, cache, and security headers. |
| 37 | 9 | Build only the deployable site with npm run build:site. |
| 38 | 9 | Build only the release binary with npm run build:cli. |
| 39 | 10 | The CLI has no telemetry and makes no network requests. |
| 40 | 7 | Inputs and packets remain on your machine. |
| 41 | 10 | Core checks and packet export work without a Team license. |
| 42 | 11 | A Team license costs $39 once and adds local policy downloads. |
| 43 | 8 | Team policies support queue histories beyond three submissions. |
| 44 | 8 | Purchases and license verification use Sociobot’s billing API. |
| 45 | 11 | Pass a Team policy from the website builder with --policy team-policy.yaml. |
| 46 | 6 | See examples/team-policy.yaml for the file shape. |
| 47 | 7 | See the website privacy page and terms. |
| 48 | 12 | Dodo is the merchant of record for Sociobot purchases and handles refunds. |
| 49 | 6 | Do not publish from a worker. |
| 50 | 7 | Before a factory release, run cargo package. |
| 51 | 1 | MIT. |
| 52 | 2 | See LICENSE. |

### Headings, labels, and actions

| Source | Words | Copy | Result |
| --- | ---: | --- | --- |
| Landing eyebrow | 6 | Local App Review preflight · rules apple-2026.1 | Flag: jargon; rewrite in F-1-17. |
| Landing h1 | 7 | Check your iOS release before review | Pass: job-led and under nine words. |
| Primary action | 6 | Try it with sample data | Pass: explicit sample result. |
| Section label | 2 | SHEET 01 | Flag: decorative; delete per F-1-16. |
| Section heading | 6 | See the gate result before submission | Flag: “gate”; rewrite per F-1-17. |
| Section label | 2 | PROCEDURE / 03 | Flag: decorative; delete per F-1-16. |
| Section heading | 4 | Build one decision record | Flag: output term; rewrite “Build one review packet”. |
| Step heading | 3 | Export archive facts | Pass for the intended technical audience. |
| Step heading | 3 | Describe the release | Pass. |
| Step heading | 3 | Print the packet | Flag: normalize to “Print the review packet”. |
| Section label | 2 | COMMAND / 01 | Flag: decorative; delete per F-1-16. |
| Section heading | 6 | Run the gate in your repository | Flag: “gate”; rewrite per F-1-17. |
| Section label | 2 | BOUNDARY / LOCAL | Flag: decorative; delete per F-1-16. |
| Section heading | 4 | Your release stays local | Pass. |
| Section label | 2 | TEAM POLICY | Pass: names a real section. |
| Section heading | 6 | Keep team rules beside the release | Pass. |
| Purchase action | 3 | Buy Team license | Pass: names the result. |
| Restore action | 3 | Verify Team license | Pass: names the result. |
| README h1 | 3 | iOS Review Gate | Pass as the document title. |
| README h2s | 1–4 | Install; Try the bundled demo; Check a release; Input files; Queue input; Develop and verify; Privacy and price; Publishing; License | Pass: each names its section. |

The landing action labels satisfy the result-naming rule. The demo and 404
actions are reviewed separately in F-1-19 and F-1-20.

## Demo and sandbox

- **PASS:** one click from `/` opens `/demo` and immediately shows Harbor Log
  2.4.0, build 108, locale `en-US`, privacy manifest present, PASS, terminal
  output, and a Markdown packet excerpt.
- **PASS:** the banner remains visible and says “Demo — no personal data is
  saved”, with Reset demo and Start for real.
- **PASS:** Reset demo restores the same complete sample and PASS state.
- **PASS:** a fresh direct `/demo` context seeded with unrelated local/session
  values did not read, change, or delete them; it made zero cross-origin
  requests and created no cookie or IndexedDB database. The only browser
  persistence was the documented `ios-review-gate-v5` static cache.
- **PASS:** after warming the service worker, `/demo` reloaded offline with its
  title, h1, sample, and PASS result intact.
- **PASS:** `cargo run -- demo` from a temporary working directory left a
  sentinel file byte-for-byte unchanged and wrote the sample and packet only
  under a new `/tmp/ios-review-gate-demo-*` directory.

## Registered claims

All exact commands were run independently in a disposable clone. `npm ci`
also passed in a second untouched clone. No registered claim test failed.

| Claim | Result | Evidence |
| --- | --- | --- |
| `identity-consistency` | PASS | Three seeded identity mismatches were reported. |
| `release-completeness` | PASS | Metadata, privacy, locale, image, device, and reason fixtures were rejected. |
| `markdown-packet` | PASS | A dated PASS packet with rule and queue sections was written. |
| `core-without-team-license` | PASS | Eight checks and a packet completed without a policy. |
| `bundled-cli-demo` | PASS | Demo created a new temporary workspace and packet. |
| `same-checker-demo` | PASS | Bundled files, CLI result, and recording markers matched. |
| `actionable-mismatch-errors` | PASS | Mismatch values and repair instruction were asserted. |
| `queue-plan` | PASS | Active-submission and buffer dates matched expectations. |
| `queue-input-validation` | PASS | Blank/unknown/future entries produced HOLD findings. |
| `queue-date-limits` | PASS | Extreme durations produced HOLD without panic. |
| `cli-local` | PASS | Source and dependency network-client markers were absent. |
| `cli-exit-codes` | PASS | PASS/HOLD/invalid-input exit codes matched 0/2/1. |
| `cli-json-schema` | PASS | All five documented JSON fields were present. |
| `one-click-demo` | PASS | One action opened the populated sample. |
| `browser-demo-local` | PASS | Demo storage and request assertions passed. |
| `rust-msrv` | PASS | All targets passed under Rust 1.85.0 with the lockfile. |
| `license-restore` | PASS | Recorded valid verification stored and restored a token. |
| `team-purchase` | PASS | UI showed $39 once and requested the exact Sociobot checkout URL. |
| `team-policy-download` | PASS | Licensed policy YAML contained the selected queue limit. |
| `team-queue-history` | PASS | A Team limit accepted the five-entry history rejected by the default. |

The live checkout independently returned HTTP 303 to
`checkout.dodopayments.com`. The live verification endpoint returned 200 for
requests 1–30 and 429 with `Retry-After: 4` for requests 31–35.

## Earlier-finding regression audit

There are no earlier `review-*.md` or `polish-*.md` files. I read the complete
existing handoff and rechecked every defect class it records.

| Earlier finding | Current confirmation |
| --- | --- |
| Production checkout missing; no observed verification rate limit | Fixed: live checkout is 303 to Dodo; request 31 is 429 with `Retry-After`. |
| Blank identity, corrupt images, and negative durations could pass | Fixed in code and passing regression tests. |
| Rust 1.85 failed | Fixed: exact registered Rust 1.85 command passes. |
| Browser privacy test omitted storage/cache scope | Fixed for the registered wording; live direct-demo request/storage log also passes. F-1-1 concerns the additional cache-content sentence. |
| Missing route returned 200 | Fixed: unknown live route returns the designed page with HTTP 404. |
| Small touch targets and 200% overflow | Fixed: live 390 px route matrix has no target below 44 px or normal overflow; local 200% tests pass. |
| Five visitor promises were absent from claims.json | Fixed for those five exact promises; their registered commands pass. F-1-1–F-1-11 are additional public claims. |
| Invalid queue entry could pass; extreme dates could panic | Fixed in code and passing queue regressions. |
| Team checkout missing and inactive-license recovery misleading | Fixed: checkout is available; invalid token leaves the purchase route visible. |
| Truncated screenshot signatures could pass | Fixed: PNG/JPEG decoding regressions pass. |
| Wrong screenshot dimensions/device keys could pass | Fixed: 1×1 and unknown-device fixtures produce HOLD. |
| Rust formatting failed | Fixed: `cargo fmt --check` passes. |
| Malformed cached license verdict crashed recovery | Fixed: browser regression passes with no page/console error. |
| Exit-code and JSON promises were unregistered | Fixed: both claims and exact tests exist and pass. |
| Mobile wordmark accessible name omitted visible “RG” | Fixed live and in code: name is “RG — iOS Review Gate home”. |
| Invalid sibling/Team privacy reasons could pass | Fixed: both fixtures produce HOLD. |
| Unknown locale could pass | Fixed: `INVALID_LOCALE` produces HOLD. |
| Queue submission after intended date could pass | Fixed: future chronology produces HOLD. |

## Structure, accessibility, links, and identity

- `/`, `/demo`, `/privacy`, and `/terms` return 200; an unknown route returns
  404. Every page has one h1 and one main landmark. SPA back navigation restores
  `/demo` and focuses its h1.
- The four SPA routes have route-specific titles, descriptions, canonicals,
  Open Graph/Twitter metadata, SVG favicon, and apple-touch icon. F-1-14 records
  the standalone 404 omissions.
- Every crawled internal link returns 200. The checkout returns 303, Param
  Factory returns 200, and the only exempt link is `mailto:`.
- Live desktop/mobile light Axe scans across all routes found no serious or
  critical violation. `verify-url.sh` passed with no console error, correct
  language/title/landmarks, complete alt text, and labeled controls.
- `npm test` passed 16 Rust tests, four Node tests, and 22 Playwright tests.
  `npm run build`, `cargo fmt --check`, and strict Clippy passed. Built JS is
  14,808 bytes before gzip; the production build produced `dist/site/`.
- Live HTML, JS, and CSS SHA-256 values match the local production build.
- The warm-paper drafting grid, mono annotations, vermilion marks, blueprint
  illustration, squared controls, and inspection-line motion are distinct and
  match `.factory/design.md`; this is not a generic SaaS template. Original
  asset provenance and self-hosted font provenance are recorded.

## Missed leverage

F-1-12 is the only obvious missing capability implied by the brief: direct,
local archive/IPA metadata import. No AI step is justified for deterministic
release metadata and rule checking, and the product embeds no provider key.
Packet export already exists; account sync would conflict with the local-first
scope.

## What would make this perfect

Resolve F-1-1 through F-1-22. In particular, either register and test every
remaining public promise or remove it; add direct archive/IPA import; keep the
three facts inside the desktop first viewport; bring the 404 into the shared
metadata/header/footer contract; and replace decorative, vague, or inconsistent
copy with the proposed plain terms. Then rerun the complete cold-read, demo,
claims, history, route, link, accessibility, privacy, offline, build, and copy
audit from a fresh clone. The pass criterion is zero remaining findings.
