# Adversarial first-read review 2 — iOS Review Gate

Date: 2026-08-29 UTC

Candidate: `d168878a6108510408f1c447c605610d33e22f19`

Live site: <https://ios-review-gate.sociobot.in>

Verdict: **FAIL**

The cold first read, direct demo, CLI demo, registered claims, build, routes,
accessibility checks, and visual identity pass. The product does not meet the
required zero-finding standard. Five blocking findings remain: the one-click
demo can write a real license verdict after demo mode opens, the paid policy
builder promises a capability it does not provide, two public quantitative or
version statements are absent from the claims registry, and a decorative
label from the earlier copy defect class remains.

## Cold first read

I opened the live site in separate fresh browser contexts at 390×844 and
1440×900. I did not scroll before recording these answers.

| Question | 390×844 | 1440×900 |
| --- | --- | --- |
| What does this do? | Checks an iOS release before review and produces a Markdown review packet. | Same. |
| For whom? | Small iOS teams preparing to queue a build. | Same. |
| What should I click first? | **Try it with sample data**. | Same. |

The exact first-screen text is “Check your iOS release before review,” “For
small iOS teams that need one Markdown review packet before they queue a
build,” and “Try it with sample data.” The adjacent result says “Open a
checked sample and its review packet.”

The action ended at 686.3 px on mobile and 671.6 px on desktop. All three facts
ended by 826.1 px on mobile and 772.6 px on desktop. The blocking first-read
test passes at both widths.

## Findings

### Blocking

#### F-2-1 — The one-click demo can write real license state after demo mode opens

- Exact text/location: landing action “Try it with sample data”; demo banner
  “Demo — sample data, nothing is saved.”
- Evidence: I seeded `sb_license:ios-review-gate=real-license`, delayed the
  landing page's license-verification response, and immediately selected the
  sample action. At the click, `/?demo=1` showed the banner and the seeded
  storage was unchanged. The pending response then completed while the demo
  banner was visible and added
  `sb_license:ios-review-gate:verdict={"token":"real-license",...}`. The
  request started on `/` and was fulfilled on `/?demo=1`.
- Code location: `initializeLicense()` starts `verifyLicense()` on the landing
  route. `navigate()` renders demo mode but does not cancel or invalidate that
  pending request. `verifyLicense()` writes the verdict without checking the
  current route.
- Why this fails: “nothing is saved” is false for the required one-click path
  when a visitor already has a license. Demo mode writes to the real license
  namespace, contrary to the demo sandbox contract. The existing
  `browser-demo-local` test opens the demo directly, so it cannot catch this
  race.
- Concrete fix: cancel pending verification when entering demo mode and guard
  every storage write with a route/session generation check. Extend
  `@claim:browser-demo-local` to start at `/` with seeded real keys, hold the
  verification response, click the sample action, release the response, and
  assert byte-for-byte storage equality and no post-click cross-origin work.

#### F-2-2 — The paid policy builder promises approved-reason controls that do not exist

- Exact quote/location: licensed Team panel, “Build a shared policy file for
  queue limits and approved reason codes.”
- Evidence: the live verified-license state exposes only **Policy name** and
  **Active submission limit**. Its download is:

  ```yaml
  name: "Release team"
  max_active_submissions: 8
  approved_reason_codes: {}
  ```

- Claims gap: `team-policy-download` promises and tests only the chosen queue
  limit. No registered claim covers choosing approved reason codes. The test
  explicitly expects an empty map.
- Why this fails: a visitor deciding whether to buy the $39 Team license is
  told the builder creates both parts of the policy, but it cannot select or
  save any reason code.
- Concrete fix: either add controls populated from the bundled Apple rules and
  claim-test the selected codes through the downloaded policy and CLI, or
  rewrite the sentence to “Build a shared policy file for queue limits.”

#### F-2-3 — The README's exact screenshot dimension is an unlisted quantitative claim

- Exact quote/location: README, “The bundled `iphone-69` sample is 1320×2868
  pixels.”
- Claims gap: no `.factory/claims.json` entry states this number. The
  `release-completeness` test proves that the sample is accepted for a device
  set that allows multiple sizes; it does not assert the advertised dimensions.
- Why this fails: quantitative statements must be owned by an exact sandbox
  test. A visitor may use this number when preparing screenshots.
- Concrete fix: add a `sample-screenshot-dimensions` claim whose test decodes
  the shipped JPEG and asserts width 1320 and height 2868, or remove the
  sentence.

#### F-2-4 / F-1-6 (reopened) — The footer still makes the removed exact-version claim

- Exact quote/location: footer on every route and the designed 404,
  “v0.1.0 · build 2026.08.29.”
- Claims gap: there is no package-version or build-identity entry in
  `.factory/claims.json`. `license-metadata` checks licensing only. No
  registered test compares the footer with Cargo/package metadata or
  `ios-review-gate --version`.
- Why this fails: F-1-6 required the unregistered `0.1.0` claim to be tested or
  removed. The README sentence was removed, but the same public version claim
  remains in the shared footer. Under the history rule, this is a blocking
  recurrence of F-1-6.
- Concrete fix: retain the structure-required footer but register a
  `version-metadata` claim. Test the Cargo version, installed CLI output, SPA
  footer, 404 footer, and a real generated build identifier from one source.

#### F-2-5 / F-1-16 (reopened) — A decorative drafting label remains

- Exact quote/location: desktop hero art, “RELEASE / 02.4.”
- Why this fails: it is an unexplained, aria-hidden version-like label. It
  conveys no task, result, price, privacy fact, or navigation information and
  competes with the footer's `v0.1.0`. The plain-words rule rejects decorative
  lore labels. F-1-16 was therefore only partly fixed, so the same defect class
  is blocking again.
- Concrete fix: remove the `measure-note` span. Keep the drafting identity in
  the grid, line work, typography, and original illustration rather than
  invented release notation.

No non-blocking findings remain outside these five items.

## Copy audit

Counts treat a URL, path, flag, version, hyphenated term, or price as one word.
Code blocks and data examples are excluded. Metadata, meaningful alt text,
figcaptions, and conditional landing-page messages are included. No sentence
exceeds 22 words, no banned marketing word appears, and average sentence
length is below 14 words. The content defects are F-2-2 through F-2-5.

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
| 25 | 3 | Have a license? |
| 26 | 3 | Paste it here. |
| 27 | 4 | Verified on this device. |
| 28 | 3 | License not active. |
| 29 | 8 | Check the token or use Buy Team license. |
| 30 | 5 | Could not check the license. |
| 31 | 5 | Connect once and try again. |
| 32 | 5 | License removed from this browser. |
| 33 | 3 | Team policy downloaded. |
| 34 | 7 | Pass it to the CLI with --policy. |
| 35 | 4 | License no longer active. |
| 36 | 6 | Sociobot hosts checkout and license verification. |
| 37 | 6 | For billing help, email billing@sociobot.in. |
| 38 | 4 | See privacy and terms. |
| 39 | 10 | Check an iOS release and print its Markdown review packet. |

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

### Headings, labels, terms, and actions

- The headline has seven words and names the job. The audience sentence has
  15 words. The adjacent action names the sample result.
- “See the check result before submission,” “Build one review packet,”
  “Inspect your archive,” “Describe the release,” “Print the review packet,”
  “Run the checker in your repository,” “Your release files stay local,” and
  “Keep team rules beside the release” make sense outside their layout.
- **Try it with sample data**, **Buy Team license**, **Verify Team license**,
  **Download Team policy**, **Remove license from this browser**, **Reset
  demo**, **Install the CLI**, and **Return home** are result-naming actions.
- Operation terminology is consistently **check**; the tool is **checker** or
  **CLI**; the output is **Markdown review packet**, then **review packet**;
  outcomes are **PASS** and **HOLD**.
- “RELEASE / 02.4” is the only non-informative drafting label and is F-2-5.
- “v0.1.0 · build 2026.08.29” is a useful structure label but an unregistered
  public claim, recorded as F-2-4.

## Demo and sandbox verification

### Website

- PASS: one click from `/` opens `/?demo=1`.
- PASS: at 390×844, the first demo viewport contains the persistent banner,
  Harbor Log 2.4.0, build 108, locale `en-US`, privacy-manifest status, and
  PASS decision. The PASS mark ends at 807.9 px.
- PASS: the banner provides **Reset demo** and **Install the CLI**.
- PASS: Reset restores the same sample and PASS state.
- PASS: a fresh direct `/?demo=1` context preserved seeded real local/session
  values, created no cookie or IndexedDB database, and made no cross-origin
  request.
- PASS: cache `ios-review-gate-v6` held 11 same-origin route/static responses;
  the populated demo reloaded offline.
- **FAIL:** the required landing-to-demo path has the pending-license race in
  F-2-1. The direct-entry claim does not cover the full one-click transition.

### CLI

I ran the built CLI from a new temporary directory containing only a sentinel:

```text
Demo — sample data, nothing from your projects was read.
Workspace: /tmp/ios-review-gate-demo-8691-1788036293
PASS — 0 errors, 0 warnings
Review packet: /tmp/ios-review-gate-demo-8691-1788036293/submission-packet.md
```

The sentinel SHA-256 was unchanged. The command wrote no file in the working
directory and placed the sample and packet only in its new temporary workspace.

## Registered claims

I cloned candidate `d168878` into
`/tmp/ios-review-gate-review2.NsQKKK/repo`, ran `npm ci`, and ran every exact
`test` command in `.factory/claims.json` independently. All 23 passed.

| Claim | Result | Observed evidence |
| --- | --- | --- |
| `identity-consistency` | PASS | Seeded version, build, and bundle mismatches were caught. |
| `release-completeness` | PASS | Locale, metadata, image, device, privacy, and reason fixtures were checked. |
| `markdown-packet` | PASS | Dated packet named `apple-2026.1`. |
| `core-without-team-license` | PASS | Eight checks and packet completed without Team policy. |
| `bundled-cli-demo` | PASS | New temporary workspace and packet were created. |
| `archive-inspection` | PASS | `.xcarchive` and assembled `.ipa` produced checked metadata and PASS. |
| `same-checker-demo` | PASS | Bundled CLI markers matched the website recording. |
| `actionable-mismatch-errors` | PASS | Values and next repair were asserted. |
| `queue-plan` | PASS | Active submission and buffer changed dates as specified. |
| `queue-input-validation` | PASS | Blank, unknown, and future entries produced HOLD. |
| `queue-date-limits` | PASS | Extreme durations produced HOLD without panic. |
| `cli-local` | PASS | Source and manifest contained no tested network-client markers. |
| `cli-exit-codes` | PASS | PASS/HOLD/invalid returned 0/2/1. |
| `cli-json-schema` | PASS | All five documented JSON fields were present. |
| `one-click-demo` | PASS | Empty-state landing opened the populated demo in one click. |
| `browser-demo-local` | PASS | Direct demo preserved seeded storage and used same-origin requests. |
| `offline-shell` | PASS | Cache scope and offline sample reload passed. |
| `rust-msrv` | PASS | All targets passed with Rust 1.85.0 and lockfile. |
| `license-restore` | PASS | Recorded valid response stored and restored a token. |
| `team-purchase` | PASS | $39 label and exact Sociobot checkout request passed. |
| `team-policy-download` | PASS | Chosen queue limit appeared in local YAML. |
| `team-queue-history` | PASS | Team limit accepted the five-entry history. |
| `license-metadata` | PASS | LICENSE, Cargo, README, and terms agreed on MIT. |

The live checkout independently returned HTTP 303 to hosted checkout. F-2-2,
F-2-3, and F-2-4 are public claims with no matching claims entry. No listed
claim test failed; the registry itself is incomplete.

## Earlier-finding regression audit

I read `.factory/review-1.md`, `.factory/polish-1.md`, and the complete prior
`.factory/handoff.md`, then checked every earlier finding in live behavior and
source.

| Earlier finding | Review-2 confirmation |
| --- | --- |
| F-1-1 offline cache scope | Fixed: registered claim passed; live cache contained only 11 same-origin route/static responses and reloaded offline. |
| F-1-2 merchant statement | Fixed: Dodo merchant copy is absent from landing, legal pages, and README. |
| F-1-3 refund handling | Fixed: refund promise is absent; billing email is visible. |
| F-1-4 refund deactivation | Fixed: claim is absent. |
| F-1-5 binary publication | Fixed: future publication claim is absent. |
| F-1-6 exact package version | **Regressed/half-fixed:** README sentence is gone, but every footer still claims `v0.1.0`; see F-2-4. |
| F-1-7 embedded rule wording | Fixed: narrowed packet claim is registered and passed. |
| F-1-8 aggregate test prose | Fixed: coverage prose is absent. |
| F-1-9 build-output prose | Fixed: output promise is absent. |
| F-1-10 static-host prose | Fixed: public host promise is absent; deployed behavior also passed. |
| F-1-11 MIT statement | Fixed: `license-metadata` exists and passed. |
| F-1-12 archive import | Fixed: direct `.xcarchive`/`.ipa` import exists and its exact claim passed. |
| F-1-13 desktop facts below fold | Fixed: all three facts end by 772.6 px in the 900 px viewport. |
| F-1-14 incomplete 404 shell | Fixed: live 404 has complete metadata, shared header/footer, recovery, and HTTP 404. |
| F-1-15 29-word README sentence | Fixed: split into 16- and 10-word sentences. |
| F-1-16 decorative drafting labels | **Half-fixed:** the quoted labels were removed, but “RELEASE / 02.4” remains; see F-2-5. |
| F-1-17 check/gate jargon | Fixed: public operation terminology is “check”; “Gate” remains only in the product name. |
| F-1-18 output naming | Fixed: copy uses Markdown review packet/review packet. |
| F-1-19 vague demo exit | Fixed: banner action is “Install the CLI” and targets `/#install`. |
| F-1-20 vague 404 action | Fixed: action is “Return home.” |
| F-1-21 subjective “realistic” | Fixed: Harbor Log and temporary-directory behavior are named. |
| F-1-22 vague “real checker” | Fixed: demo says “the same checker as the CLI.” |

## Structure, accessibility, links, and identity

- `/`, `/demo`, `/privacy`, and `/terms` returned 200. A random unknown route
  returned the designed page with HTTP 404.
- Every route had its route-specific title, one h1, one main landmark,
  `lang=en`, a description, canonical, Open Graph/Twitter card, favicon, Apple
  touch icon, shared header, and shared footer.
- Back navigation restored `/demo`, moved focus to “Inspect a complete sample
  release,” announced the route through the live region, and reset scroll.
- All normal internal links returned 200. Checkout returned 303, Param Factory
  returned 200, and `mailto:` links were explicit. The 404 skip link correctly
  remains an in-page link on the 404 response.
- Live Axe scans at 390 px found zero violations on home, demo, privacy, terms,
  and 404. `/opt/fleet/lib/verify-url.sh` passed in 716 ms with no console or
  page error, one h1/main, complete alt text, and labeled buttons.
- Response headers include HSTS, `nosniff`, Referrer-Policy,
  Permissions-Policy, and a header-delivered CSP with
  `frame-ancestors 'none'`.
- The clean-clone production build matched live SHA-256 values for
  `index.html`, hashed JS, hashed CSS, `sw.js`, and `404.html`.
- The built JS is 14.75 KiB raw / 5.49 KiB gzip. `npm test` passed 17 Rust,
  five Node, and 23 Playwright tests. `npm run build`, `cargo fmt --check`, and
  strict Clippy passed.
- The drafting-paper palette, mono annotations, vermilion marks, blueprint
  illustration, squared controls, and inspection-line motion are distinct and
  match `.factory/design.md`. The layout is not a generic SaaS template.

F-2-5 is a copy defect inside that otherwise distinct identity.

## Missed leverage

No additional feature is warranted. The earlier missing `.xcarchive`/`.ipa`
import now exists. Packet export exists. Sync would conflict with the local
scope, and an AI step would add uncertainty to deterministic release-rule
checks. No provider or Azure key is embedded.

## What would make this perfect

Resolve all five findings, then rerun the complete review rather than only the
new tests. Specifically: cancel or invalidate landing license work on demo
entry; make the paid reason-code builder real or narrow its copy; register and
assert the sample dimensions; generate and claim-test one version/build label;
and remove “RELEASE / 02.4.” The pass bar remains zero findings and no untested
claim.
