# Polish round 2 — cumulative finding closure

Candidate repaired: `0408e389692712681465ddc9940d124bb48e1f3d`  
Review commit: `1a64a0bad9d0feeba640540fdf05798ec3931139`  
Repair implementation: `c390cf0711dafb579983ef0220761136cc573020`  
Live URL: <https://ios-review-gate.sociobot.in>  
Date: 29 August 2026 UTC

Every finding in `review-1.md` and `review-2.md` is closed. No severity is deferred.

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Kept the registered offline-shell contract and advanced the clean cache to `ios-review-gate-v7`. | `@claim:offline-shell` passed from the clean clone; production held 11 same-origin entries and reloaded `/?demo=1` offline. |
| F-1-2 | Kept the unprovable merchant-of-record statement removed. | `@claim:team-purchase` passed; production copy names only Sociobot checkout and verification. |
| F-1-3 | Kept refund promises removed and retained the billing support address. | Production landing and `/terms` show `billing@sociobot.in`; source search has no refund promise. |
| F-1-4 | Kept refund-deactivation copy removed. | Production and source contain no refund-deactivation statement. |
| F-1-5 | Kept the unproved binary-publication statement removed. | README gives the source install path; `cargo package --locked --allow-dirty` passed. |
| F-1-6 | Registered version/build identity and generated both footer values from Cargo metadata and the Git commit. | `@claim:version-metadata` passed on `/`, `/demo`, `/privacy`, `/terms`, and `/404.html`; production footer showed the deployed build. |
| F-1-7 | Kept the rules statement narrowed to the packet's named rules version. | `claim_markdown_packet_writes_dated_decision_record` passed with `apple-2026.1`. |
| F-1-8 | Kept the unregistered aggregate-test prose removed. | `npm test` independently passed 18 Rust, 5 contract, and 24 browser tests. |
| F-1-9 | Kept the unregistered output prose removed. | `npm run build` passed and produced the release CLI plus `dist/site`. |
| F-1-10 | Kept the public static-host promise removed while retaining the tested host contract. | Contract test passed; production headers, deep links, and HTTP 404 were checked. |
| F-1-11 | Kept MIT metadata registered and consistent. | `@claim:license-metadata` passed across LICENSE, Cargo, README, and terms. |
| F-1-12 | Retained direct `.xcarchive` and `.ipa` inspection and check flows. | `claim_archive_inspection_extracts_xcarchive_and_ipa_then_checks_release` passed. |
| F-1-13 | Retained the compact first screen with all three facts. | Browser fold test passed; production bottoms were 826.1 px at 390×844 and 772.6 px at 1440×900. [Mobile screenshot](evidence/polish-2-live-home/screenshot-mobile.png). |
| F-1-14 | Moved the 404 into the same Vite-built shell so metadata, navigation, legal links, and build identity cannot drift. | Browser 404 test passed; production unknown route returned HTTP 404 with canonical `/404`. [404 screenshot](evidence/polish-2-live-404.png). |
| F-1-15 | Kept the long README sentence split. | `copy-audit.md` records no sentence over 22 words. |
| F-1-16 | Removed the remaining `RELEASE / 02.4` decoration. | Source and production bundle contain no label; home screenshot shows the clean blueprint panel. |
| F-1-17 | Kept operation wording standardized on “check” and “checker.” | `copy-audit.md` terminology table and production cold read. |
| F-1-18 | Kept output wording standardized on “Markdown review packet,” then “review packet.” | `claim_markdown_packet_writes_dated_decision_record` and production copy. |
| F-1-19 | Kept the demo exit action as **Install the CLI** targeting `/#install`. | One-click demo test and production demo banner. [Demo screenshot](evidence/polish-2-live-demo-mobile.png). |
| F-1-20 | Kept the 404 action as **Return home**. | Browser 404 test and production screenshot. |
| F-1-21 | Kept the sample description concrete and named Harbor Log. | `claim_bundled_demo_runs_in_a_new_temp_workspace` passed. |
| F-1-22 | Kept the demo provenance wording as “the same checker as the CLI.” | `claim_demo_recording_matches_bundled_cli` passed and production demo shows the sentence. |
| F-2-1 | Added an abort controller and route-generation guard around license verification and every asynchronous verdict write. Entering demo invalidates pending real-mode work. | Expanded `@claim:browser-demo-local` delays the real-license response across the one-click transition, then proves local/session storage unchanged and no post-entry cross-origin request. The clean-clone and production race checks passed. |
| F-2-2 | Added four checkbox controls derived from bundled `apple-2026.1` rules. The download serializes selected codes by category. | Expanded `@claim:team-policy-download` inspects selected YAML and passes it to the real CLI for a PASS. Live mobile Axe found no serious/critical issue. [Policy screenshot](evidence/polish-2-live-policy-mobile.png). |
| F-2-3 | Registered the exact sample dimensions and added a decoding test. | `claim_sample_screenshot_dimensions_match_the_documented_fixture` decoded the shipped JPEG as 1320×2868. |
| F-2-4 | Replaced hand-written footer values with Cargo version plus a build-time Git identifier on every route and the 404. | `@claim:version-metadata` compares Cargo, package metadata, CLI `--version`, all SPA routes, and the standalone 404. Local/live asset hashes matched. |
| F-2-5 | Removed the last `measure-note` element and its CSS. | Source search and production cold screenshots confirm no decorative release label remains. |

## Verification evidence

- Clean clone: `/tmp/ios-review-gate-polish2-claims.8kCNE3/repo`; all 25 exact commands in `claims.json` passed independently.
- Full suite: `npm test` — 18 Rust, 5 Node contract, and 24 Playwright tests passed.
- Toolchain: Rust 1.85 all-target test, `cargo fmt --check`, and clippy with warnings denied passed.
- Build/package: `npm run build` and `cargo package --locked --allow-dirty` passed. Initial JavaScript was 20.72 KB raw and 7.04 KB gzip; CSS was 11.17 KB raw and 3.40 KB gzip.
- Production verifier: title, `lang`, one h1, main landmark, alt text, labels, and zero console errors passed. Report: [verify.json](evidence/polish-2-live-home/verify.json).
- Production Lighthouse mobile: Performance 100, Accessibility 100, Best Practices 100, SEO 100; FCP 1.06 s, LCP 1.51 s, TBT 0 ms, CLS 0.026.
- Production parity: SHA-256 matched local output for HTML, JS, CSS, service worker, and 404.

Unresolved findings: none.
