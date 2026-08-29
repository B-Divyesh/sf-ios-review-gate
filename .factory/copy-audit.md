# Copy audit — polish round 2

Audited 29 August 2026. Counts treat a URL, path, flag, hyphenated term, or version as one word. Code blocks are excluded. No sentence exceeds 22 words. No banned plain-words term appears.

## Landing page sentences

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

## README sentences

| # | Words | Sentence |
| ---: | ---: | --- |
| 1 | 14 | Check an iOS release and print a Markdown review packet before App Store submission. |
| 2 | 16 | It is for small iOS teams that want one local packet before they queue a build. |
| 3 | 12 | The CLI reads an .xcarchive or .ipa and compares it with release.yaml. |
| 4 | 11 | It checks identity, localized metadata, screenshots, privacy declarations, and queue timing. |
| 5 | 12 | It does not upload a build or connect to App Store Connect. |
| 6 | 9 | Build the single binary with Rust 1.85 or newer. |
| 7 | 7 | Build the CLI with the command above. |
| 9 | 12 | The command copies the bundled Harbor Log files to a temporary directory. |
| 10 | 9 | It runs the check and prints the review-packet path. |
| 11 | 8 | It does not read or change your files. |
| 12 | 6 | Open the website sample at https://ios-review-gate.sociobot.in/?demo=1. |
| 13 | 7 | Exit code 0 means the check passed. |
| 14 | 11 | Exit code 2 means the inputs were read but checks failed. |
| 15 | 8 | Invalid files or arguments use exit code 1. |
| 16 | 5 | Use JSON output in automation. |
| 17 | 10 | The JSON object contains passed, summary, findings, queue, and packet_path. |
| 18 | 13 | Write extracted identity and privacy declarations to JSON when another tool needs them. |
| 19 | 8 | The checker supports .xcarchive directories and .ipa files. |
| 20 | 10 | It reads the app's property lists and privacy manifests locally. |
| 21 | 11 | You can also pass an existing JSON export with check --metadata. |
| 22 | 5 | release.yaml records the intended submission. |
| 23 | 5 | See examples/sample/release.yaml for every field. |
| 24 | 7 | Screenshot paths are relative to that file. |
| 25 | 8 | Every review packet names the rules version used. |
| 26 | 16 | Version apple-2026.1 checks supported locales, localized fields, screenshot sets and sizes, and the privacy manifest. |
| 27 | 10 | It also checks collected-data answers and every declared reason code. |
| 28 | 17 | Supported locale identifiers, screenshot set keys, and exact portrait or landscape sizes are in rules/apple-2026.1.yaml. |
| 29 | 15 | Use a listed locale such as en-US and a device key such as iphone-69. |
| 30 | 15 | Unknown identifiers, unknown device keys, and decodable files at the wrong dimensions produce a HOLD. |
| 31 | 9 | The bundled iphone-69 sample is 1320×2868 pixels. |
| 32 | 14 | Every reason in accessed_apis must appear in the Apple rules for its API category. |
| 33 | 15 | A Team policy can narrow that list with approved_reason_codes; it cannot make another code Apple-approved. |
| 34 | 17 | Each queued submission needs a version, build, submitted date, and one status: waiting_for_review, in_review, pending_developer_release, or completed. |
| 35 | 9 | Unknown or incomplete queue entries produce a HOLD result. |
| 36 | 16 | Review and buffer days must be zero or positive and must fit a real calendar date. |
| 37 | 9 | Each submitted_on date must be on or before intended_submission. |
| 38 | 7 | Use npm run build:site for the site. |
| 39 | 8 | Use npm run build:cli for the release binary. |
| 40 | 10 | The CLI has no telemetry and makes no network requests. |
| 41 | 8 | Inputs and review packets remain on your machine. |
| 42 | 11 | Core checks and review packet export work without a Team license. |
| 43 | 11 | A Team license costs $39 once and adds local policy downloads. |
| 44 | 8 | Team policies support queue histories beyond three submissions. |
| 45 | 8 | Purchases and license verification use Sociobot's billing API. |
| 46 | 11 | Pass a Team policy from the website builder with --policy team-policy.yaml. |
| 47 | 6 | See examples/team-policy.yaml for the file shape. |
| 48 | 7 | See the website privacy page and terms. |
| 49 | 6 | Email billing@sociobot.in for billing help. |
| 50 | 6 | Do not publish from a worker. |
| 51 | 7 | Before a factory release, run this command. |
| 52 | 1 | MIT. |
| 53 | 2 | See LICENSE. |

## First-screen and terminology checks

- Headline: “Check your iOS release before review” — 7 words, job-led.
- Audience sentence: 15 words.
- Primary action: “Try it with sample data.”
- Adjacent result: “Open a checked sample and its review packet.”
- Three facts: local release files, offline demo, and $0 core checks.
- Catalog: “Check an iOS archive and release plan, then print a Markdown review packet.” — 13 words, 73 characters.

| Concept | One term |
| --- | --- |
| Operation | check |
| Tool | checker or CLI |
| Output | Markdown review packet; review packet after first mention |
| Artifact inputs | .xcarchive or .ipa |
| Paid entitlement | Team license |
| Paid file | Team policy |
| Outcome | PASS or HOLD decision |

Flags: none.
