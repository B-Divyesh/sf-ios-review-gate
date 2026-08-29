# Adversarial first-read review 3 — iOS Review Gate

Date: 2026-08-29 UTC

Candidate: `e08bcf96af8d2e64752f983cefa8ed1da19d3b9d`

Deployed product source: `e5eb710ff9bc65061e8a6558b5c57d30e01fd9f4`

Live site: <https://ios-review-gate.sociobot.in>

Verdict: **PASS**

There are zero findings. The job, audience, and first action are clear without
scrolling at both requested widths. The website and CLI demos are immediately
usable and isolated. All 25 registered claim commands pass from a clean clone,
and no public claim on the landing page or README is unlisted. Every finding
from reviews 1 and 2 is fixed in both the deployed behavior and current code.

The deployed build predates the candidate only by two documentation commits.
`git diff e5eb710..e08bcf9` contains only `.factory/handoff.md` and
`.factory/verification-12.md`; the product code under review is identical.

## Cold first read

I opened `/` cold in separate fresh browser contexts at 390×844 and 1440×900.
I recorded these answers at `scrollY = 0` before navigating or scrolling.

| Question | 390×844 | 1440×900 |
| --- | --- | --- |
| What does this do? | It checks an iOS release before review and produces a Markdown review packet. | Same. |
| For whom? | Small iOS teams preparing to queue a build. | Same. |
| What should I click first? | **Try it with sample data**. | Same. |

The exact text supplying those answers is “Check your iOS release before
review,” “For small iOS teams that need one Markdown review packet before they
queue a build,” and “Try it with sample data.” The adjacent result is “Open a
checked sample and its review packet.”

The primary action ends at 686.3 px on mobile and 671.6 px on desktop. The last
of the three required facts ends at 826.1 px on mobile and 772.6 px on desktop.
All required first-screen content is therefore visible without scrolling.

## Findings

None. There are no blocking, major, or minor findings.

## Copy audit

Counts treat a URL, path, flag, hyphenated term, version, or price as one word.
Code blocks and data examples are excluded. Metadata sentences, meaningful alt
text, figcaptions, and conditional landing messages are included.

### Landing-page sentences

| # | Words | Sentence |
| ---: | ---: | --- |
| 1 | 10 | Check iOS release metadata, screenshots, privacy answers, and queue timing. |
| 2 | 7 | Print a local review packet before submission. |
| 3 | 15 | For small iOS teams that need one Markdown review packet before they queue a build. |
| 4 | 8 | Open a checked sample and its review packet. |
| 5 | 6 | Release files stay on your machine. |
| 6 | 7 | The demo works offline after one visit. |
| 7 | 6 | Checks and review packets cost $0. |
| 8 | 15 | An exploded drafting view connects an app archive, metadata sheets, screenshots, and a review queue. |
| 9 | 10 | The website sample and command use the same bundled checker. |
| 10 | 8 | Errors name the mismatch and the next fix. |
| 11 | 15 | Terminal output shows the bundled release passing every check and writing a Markdown review packet. |
| 12 | 13 | The bundled demo checks a complete sample release and writes its review packet. |
| 13 | 13 | Read version, build, bundle ID, and privacy use from an .xcarchive or .ipa. |
| 14 | 12 | List localized copy, screenshot paths, privacy answers, and queue timing in YAML. |
| 15 | 3 | Run one command. |
| 16 | 10 | Fix holds and warnings, then keep the Markdown review packet. |
| 17 | 11 | Build the single Rust binary, then keep release.yaml beside the app. |
| 18 | 8 | The CLI reads the paths you give it. |
| 19 | 9 | It has no telemetry and sends no release data. |
| 20 | 13 | It does not upload builds, scrape App Store Connect, or predict Apple's decision. |
| 21 | 11 | Core checks and review packet export work without a Team license. |
| 22 | 8 | Verified Team licenses enable the local policy download. |
| 23 | 8 | Team policies support queue histories beyond three submissions. |
| 24 | 12 | Build a shared policy file for queue limits and approved reason codes. |
| 25 | 8 | Select the Apple reason codes your team permits. |
| 26 | 3 | Have a license? |
| 27 | 3 | Paste it here. |
| 28 | 4 | Verified on this device. |
| 29 | 3 | License not active. |
| 30 | 8 | Check the token or use Buy Team license. |
| 31 | 5 | Could not check the license. |
| 32 | 5 | Connect once and try again. |
| 33 | 5 | License removed from this browser. |
| 34 | 3 | Team policy downloaded. |
| 35 | 7 | Pass it to the CLI with --policy. |
| 36 | 4 | License no longer active. |
| 37 | 6 | Sociobot hosts checkout and license verification. |
| 38 | 6 | For billing help, email billing@sociobot.in. |
| 39 | 4 | See privacy and terms. |
| 40 | 10 | Check an iOS release and print its Markdown review packet. |

Maximum: 15 words. Average: 8.05 words.

### README sentences

| # | Words | Sentence |
| ---: | ---: | --- |
| 1 | 14 | Check an iOS release and print a Markdown review packet before App Store submission. |
| 2 | 16 | It is for small iOS teams that want one local packet before they queue a build. |
| 3 | 12 | The CLI reads an .xcarchive or .ipa and compares it with release.yaml. |
| 4 | 11 | It checks identity, localized metadata, screenshots, privacy declarations, and queue timing. |
| 5 | 12 | It does not upload a build or connect to App Store Connect. |
| 6 | 9 | Build the single binary with Rust 1.85 or newer. |
| 7 | 7 | Build the CLI with the command above. |
| 8 | 12 | The command copies the bundled Harbor Log files to a temporary directory. |
| 9 | 9 | It runs the check and prints the review-packet path. |
| 10 | 8 | It does not read or change your files. |
| 11 | 6 | Open the website sample at https://ios-review-gate.sociobot.in/?demo=1. |
| 12 | 7 | Exit code 0 means the check passed. |
| 13 | 11 | Exit code 2 means the inputs were read but checks failed. |
| 14 | 8 | Invalid files or arguments use exit code 1. |
| 15 | 5 | Use JSON output in automation. |
| 16 | 10 | The JSON object contains passed, summary, findings, queue, and packet_path. |
| 17 | 13 | Write extracted identity and privacy declarations to JSON when another tool needs them. |
| 18 | 8 | The checker supports .xcarchive directories and .ipa files. |
| 19 | 10 | It reads the app's property lists and privacy manifests locally. |
| 20 | 11 | You can also pass an existing JSON export with check --metadata. |
| 21 | 5 | release.yaml records the intended submission. |
| 22 | 5 | See examples/sample/release.yaml for every field. |
| 23 | 7 | Screenshot paths are relative to that file. |
| 24 | 8 | Every review packet names the rules version used. |
| 25 | 16 | Version apple-2026.1 checks supported locales, localized fields, screenshot sets and sizes, and the privacy manifest. |
| 26 | 10 | It also checks collected-data answers and every declared reason code. |
| 27 | 17 | Supported locale identifiers, screenshot set keys, and exact portrait or landscape sizes are in rules/apple-2026.1.yaml. |
| 28 | 15 | Use a listed locale such as en-US and a device key such as iphone-69. |
| 29 | 15 | Unknown identifiers, unknown device keys, and decodable files at the wrong dimensions produce a HOLD. |
| 30 | 9 | The bundled iphone-69 sample is 1320×2868 pixels. |
| 31 | 14 | Every reason in accessed_apis must appear in the Apple rules for its API category. |
| 32 | 15 | A Team policy can narrow that list with approved_reason_codes; it cannot make another code Apple-approved. |
| 33 | 17 | Each queued submission needs a version, build, submitted date, and one status: waiting_for_review, in_review, pending_developer_release, or completed. |
| 34 | 9 | Unknown or incomplete queue entries produce a HOLD result. |
| 35 | 16 | Review and buffer days must be zero or positive and must fit a real calendar date. |
| 36 | 9 | Each submitted_on date must be on or before intended_submission. |
| 37 | 7 | Use npm run build:site for the site. |
| 38 | 8 | Use npm run build:cli for the release binary. |
| 39 | 10 | The CLI has no telemetry and makes no network requests. |
| 40 | 8 | Inputs and review packets remain on your machine. |
| 41 | 11 | Core checks and review packet export work without a Team license. |
| 42 | 11 | A Team license costs $39 once and adds local policy downloads. |
| 43 | 8 | Team policies support queue histories beyond three submissions. |
| 44 | 8 | Purchases and license verification use Sociobot's billing API. |
| 45 | 11 | Pass a Team policy from the website builder with --policy team-policy.yaml. |
| 46 | 6 | See examples/team-policy.yaml for the file shape. |
| 47 | 7 | See the website privacy page and terms. |
| 48 | 6 | Email billing@sociobot.in for billing help. |
| 49 | 6 | Do not publish from a worker. |
| 50 | 7 | Before a factory release, run this command. |
| 51 | 1 | MIT. |
| 52 | 2 | See LICENSE. |

Maximum: 17 words. Average: 9.67 words.

### Copy flags, headings, and actions

- No sentence exceeds 22 words. No banned marketing word, marketing adjective,
  metaphor, mood heading, slogan, or inconsistent product term remains.
- Technical names such as `.xcarchive`, `.ipa`, `release.yaml`, and
  `approved_reason_codes` are exact inputs or schema fields, not unexplained
  promotional jargon.
- The headings name their sections: “See the check result before submission,”
  “Build one review packet,” “Inspect your archive,” “Describe the release,”
  “Print the review packet,” “Run the checker in your repository,” “Your
  release files stay local,” and “Keep team rules beside the release.”
- Result actions are explicit: **Try it with sample data**, **Buy Team
  license**, **Verify Team license**, **Download Team policy**, **Remove license
  from this browser**, **Reset demo**, **Install the CLI**, and **Return home**.
- Terminology is consistent: **check** is the operation, **checker/CLI** is the
  tool, **Markdown review packet/review packet** is the output, **Team license**
  is the entitlement, **Team policy** is the downloaded file, and **PASS/HOLD**
  are decisions.

No copy finding requires a rewrite.

## Demo and sandbox

### Website demo

- One click on **Try it with sample data** changes `/` to `/?demo=1`.
- The first 390×844 demo viewport already shows the persistent “Demo — sample
  data, nothing is saved” banner and Harbor Log 2.4.0, build 108, `en-US`, a
  present privacy manifest, and a PASS decision.
- **Reset demo** restores the same complete sample and PASS result.
- The banner includes **Install the CLI** as the explicit real-use exit.
- Seeded real release, license, verdict, and draft values remained byte-for-byte
  unchanged. The demo created no cookies or IndexedDB database.
- All demo requests were same-origin. Cache
  `ios-review-gate-e5eb710ff9bc` held 11 same-origin static responses, and the
  populated sample reloaded offline.
- The live delayed-license race from F-2-1 also passed: releasing the pending
  verification response after demo entry changed no storage and caused no
  cross-origin request.

### CLI demo

I ran `cargo run -- demo` from a fresh temporary directory containing only a
sentinel file. The command printed:

```text
Demo — sample data, nothing from your projects was read.
Workspace: /tmp/ios-review-gate-demo-6922-1788046956
PASS — 0 errors, 0 warnings
Review packet: /tmp/ios-review-gate-demo-6922-1788046956/submission-packet.md
```

The sentinel hash was unchanged, no file appeared beside it, and the reported
workspace contained only the bundled metadata, release YAML, screenshot, and
generated review packet.

## Registered claims

I cloned the candidate without local build artifacts into
`/tmp/ios-review-gate-review3.ZIGNHM/repo` and ran every exact `test` command
from `.factory/claims.json`. All 25 passed.

| Claim | Exact test | Result |
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
| `one-click-demo` | `npm run verify:browser-claim -- --grep @claim:one-click-demo` | PASS |
| `browser-demo-local` | `npm run verify:browser-claim -- --grep @claim:browser-demo-local` | PASS |
| `offline-shell` | `npm run verify:browser-claim -- --grep @claim:offline-shell` | PASS |
| `rust-msrv` | `cargo +1.85.0 test --all-targets --locked` | PASS |
| `license-restore` | `npm run verify:browser-claim -- --grep @claim:license-restore` | PASS |
| `team-purchase` | `npm run verify:browser-claim -- --grep @claim:team-purchase` | PASS |
| `team-policy-download` | `npm run verify:browser-claim -- --grep @claim:team-policy-download` | PASS |
| `team-queue-history` | `cargo test claim_team_policy_supports_queue_history_beyond_three_submissions` | PASS |
| `license-metadata` | `node --test --test-name-pattern='@claim:license-metadata' tests/site.test.js` | PASS |
| `version-metadata` | `npm run verify:browser-claim -- --grep @claim:version-metadata` | PASS |

The landing page and README claim cross-check found no unlisted claim. The live
checkout returned 303 to hosted checkout. A recorded valid license response on
the live page exposed four reason-code controls and downloaded the selected
codes with the chosen queue limit. No claim was left untested.

## Earlier-finding regression audit

I read both earlier reviews, both polish reports, and the repository handoff.
Each closure below was checked on the live site and in current source or its
registered regression test.

| Earlier finding | Live confirmation | Code/test confirmation |
| --- | --- | --- |
| F-1-1 — offline-cache scope | The live cache contained only 11 same-origin static responses; demo reload worked offline. | `offline-shell` is registered and passed. |
| F-1-2 — merchant statement | No merchant-of-record claim appears live. | No Dodo merchant copy remains in site or README source. |
| F-1-3 — refund handling | No refund promise appears; billing contact is visible. | No refund copy remains; the mail link is present. |
| F-1-4 — refund deactivation | No refund-deactivation promise appears. | No refund-deactivation string remains. |
| F-1-5 — binary publication | The site makes no binary-publication promise. | README gives the source install path and no future publication claim. |
| F-1-6 — exact package version | Every live footer shows `CLI v0.1.0 · build e5eb710ff9bc`. | `version-metadata` derives and compares Cargo, CLI, SPA, and 404 values; it passed. |
| F-1-7 — embedded-rule wording | The demo packet names `apple-2026.1` without a broader embedding promise. | `markdown-packet` passed on the exact narrowed claim. |
| F-1-8 — aggregate-test prose | No public prose claims what `npm test` covers. | README lists commands without the removed coverage promise. |
| F-1-9 — build-output prose | No public prose promises particular build output paths. | The removed sentence remains absent; the build independently passed. |
| F-1-10 — static-host prose | No public static-host behavior promise appears. | The removed sentence remains absent; host contracts still pass. |
| F-1-11 — MIT statement | Live terms state the MIT license. | `license-metadata` passed across LICENSE, Cargo, README, and terms. |
| F-1-12 — archive import | Live install and procedure copy accept `.xcarchive` and `.ipa`. | `inspect` and direct `check --archive` exist; `archive-inspection` passed both formats. |
| F-1-13 — facts below desktop fold | All three facts end by 772.6 px at 1440×900. | The desktop fold regression test passed. |
| F-1-14 — incomplete 404 | An unknown live URL returns the designed shared shell with HTTP 404 and complete metadata. | `404.html` uses the shared bundle; route metadata/navigation tests passed. |
| F-1-15 — long README sentence | No README sentence exceeds 17 words. | The split rules copy remains in README. |
| F-1-16 — decorative labels | The old drafting labels and `RELEASE / 02.4` are absent live. | Source search found none of the former labels. |
| F-1-17 — check/gate jargon | Live operation wording consistently uses “check.” | The old “preflight” and metaphorical gate strings are absent. |
| F-1-18 — competing output names | Live copy uses “Markdown review packet,” then “review packet.” | The older packet aliases are absent from public source. |
| F-1-19 — vague demo exit | The live banner action is **Install the CLI**. | It targets `/#install`; its regression test passed. |
| F-1-20 — vague 404 action | The live recovery action is **Return home**. | The 404 regression test asserts that action. |
| F-1-21 — “realistic” demo copy | README concretely names Harbor Log and temp-directory behavior. | “Realistic sample” is absent; `bundled-cli-demo` passed. |
| F-1-22 — “real checker” wording | The demo says “the same checker as the CLI.” | `same-checker-demo` passed. |
| F-2-1 — pending license write in demo | The delayed live race changed no storage and sent no post-entry cross-origin request. | Abort-controller and route-generation guards remain; `browser-demo-local` passed. |
| F-2-2 — missing reason-code controls | A verified live state showed four controls; its YAML contained the selected categories and queue limit. | Controls derive from the bundled rules; `team-policy-download` passed the file through the CLI. |
| F-2-3 — unlisted screenshot dimension | The README still states the exact fixture size. | `sample-screenshot-dimensions` decoded and confirmed 1320×2868. |
| F-2-4 — unlisted footer version | Live footer version and build match the deployed source. | `version-metadata` is registered and passed on every route and 404. |
| F-2-5 — remaining decorative label | `RELEASE / 02.4` is absent live. | The element and CSS are absent from current source. |

No earlier finding is unfixed, half-fixed, or regressed.

## Structure, links, accessibility, and identity

- `/`, `/demo`, `/privacy`, and `/terms` returned 200. An unknown path
  returned the designed page with HTTP 404.
- Every route has a route-specific title in the required pattern, exactly one
  h1, one main landmark, `lang=en`, a description, canonical, Open Graph and
  Twitter metadata, the product artwork, favicon, Apple touch icon, shared
  header, and shared footer.
- Back navigation restored the demo URL and moved focus to “Inspect a complete
  sample release.” Route changes use the polite live region.
- The link crawl found no dead destination. Internal routes returned 200,
  checkout returned 303, Param Factory returned 200, and mail links were
  explicit. The 404 skip link remains a working in-page fragment on the
  intentional 404 response.
- Playwright Axe reported zero violations on home, demo, privacy, terms, and
  404 at 390 px. The factory URL verifier found one h1, `lang`, main, complete
  image alt text, labeled buttons, and zero application console errors.
- Keyboard activation, 44 px targets, 200% text reflow, visible focus, dark
  contrast, and reduced motion all passed the full browser suite.
- The page uses the documented drafting-sheet identity: warm paper, navy grid,
  vermilion marks, mono labels, squared controls, original blueprint art, and
  a single reduced-motion-safe inspection pass. It is not a generic SaaS
  template.
- Initial JavaScript is 20.72 KB raw and 7.04 KB gzip. CSS is 11.17 KB raw and
  3.40 KB gzip.

## Quality gates

From the clean clone:

- `npm test`: PASS — 18 Rust, 7 Node contract, and 24 Playwright tests.
- `npm run build`: PASS — release CLI and `dist/site/` produced.
- `cargo +1.85.0 test --all-targets --locked`: PASS.
- `cargo fmt --check`: PASS.
- `cargo clippy --all-targets --locked -- -D warnings`: PASS.
- `cargo package --locked --allow-dirty`: PASS and package verification PASS.
- `/opt/fleet/lib/verify-url.sh`: PASS; 817 ms observed load, zero errors.

## Missed leverage

No obvious implied feature is missing. Direct `.xcarchive` and `.ipa` import,
Markdown and JSON output, queue planning, and shared policy export cover the
brief's useful adjacent steps. Sync would contradict the local-first boundary.
An AI step would add uncertainty to deterministic release-rule checks, so the
absence of AI is appropriate. No provider or Azure key is embedded.

## What would make this perfect

Nothing remains to change under this review. Preserve the current clean-clone
claim suite, demo isolation checks, and live/source parity on future releases.
