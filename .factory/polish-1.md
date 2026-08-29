# Polish round 1 — finding closure

Candidate repaired: `7919904066750612e8ecec51bd8e635c4310d929`  
Implementation commit: `c0b41ec1e46835f842a120018a21e04360070c33`  
Live URL: <https://ios-review-gate.sociobot.in>  
Date: 29 August 2026 UTC

Every finding in `.factory/review-1.md` is closed. There were no earlier `review-*.md` or `polish-*.md` reports.

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Registered `offline-shell`. Cache `v6` contains only same-origin route documents and static assets; `?demo=1` has an offline fallback. | `@claim:offline-shell`; clean-clone exact claim PASS; live cache inventory contained 11 allowed URLs; offline live reload kept Harbor Log visible. |
| F-1-2 | Removed the untestable Dodo merchant statement. Copy now states only the tested Sociobot checkout and verification path. | `@claim:team-purchase`; live checkout returned 303; landing and legal source contain no merchant claim. |
| F-1-3 | Removed refund-handling promises and added the visible `billing@sociobot.in` support route. | Live `/` and `/terms`; internal-link crawl; copy audit has no refund statement. |
| F-1-4 | Removed the refund-deactivation promise from landing and terms. | Source/copy search has no refund or deactivation claim; live `/terms` cold check. |
| F-1-5 | Removed the future binary-publication claim. README gives the working source-install command only. | Clean clone `cargo install` path is documented; `cargo package --locked` PASS. |
| F-1-6 | Removed the redundant package-version sentence. | README copy audit; `ios-review-gate --version` remains package-generated. |
| F-1-7 | Narrowed rules copy to the observable packet behavior and extended the packet claim wording. | `cargo test claim_markdown_packet`; output contains `Rules: apple-2026.1`. |
| F-1-8 | Removed the prose promise describing `npm test`; commands remain available without an unregistered coverage claim. | README copy audit; clean clone `npm test` independently PASS. |
| F-1-9 | Removed the prose build-output promise. | Clean clone `npm run build` PASS and produced both artifacts as internal handoff evidence. |
| F-1-10 | Removed the public Static Web Apps behavior promise while keeping its contract tests. | `factory contracts and static host configuration are valid JSON`; live deep links, headers, caches, and HTTP 404 checked directly. |
| F-1-11 | Registered `license-metadata` and compared all public/package license sources. | Exact clean-clone `@claim:license-metadata` PASS; LICENSE, Cargo, README, and `/terms` agree on MIT. |
| F-1-12 | Added `inspect --archive` and direct `check --archive` support for `.xcarchive` and `.ipa`. The importer reads binary/XML property lists and merges Apple privacy manifests locally. A Harbor Log archive ships in `examples/archive/`. | `cargo test claim_archive_inspection_extracts_xcarchive_and_ipa_then_checks_release`; clean clone PASS for both formats and a resulting PASS packet. |
| F-1-13 | Tightened desktop hero scale/spacing and placed all three required facts in the hero copy. | `desktop first screen contains the action and all three facts`; live 1440×900 bottoms were 715.9, 744.3, and 772.6 px. Screenshot: `.factory/evidence/polish-1-live-home.png`. |
| F-1-14 | Rebuilt standalone 404 metadata and shared shell: canonical, OG/Twitter, apple icon, Install, Param Factory, version/build, legal links, and noindex. | `the deployable not-found page has complete metadata, shared navigation, and recovery`; live random URL returned HTTP 404. Screenshot: `.factory/evidence/polish-1-live-404.png`. |
| F-1-15 | Split the 29-word rules sentence into two sentences of 16 and 10 words. | `.factory/copy-audit.md`; no README sentence exceeds 22 words. |
| F-1-16 | Removed `SHEET 01`, `PROCEDURE / 03`, `COMMAND / 01`, `BOUNDARY / LOCAL`, and the 404 lore label. Kept the useful Team policy label. | Live home/404 screenshots and source search. |
| F-1-17 | Standardized the operation as `check` and the tool as `checker`; removed public `preflight` and metaphorical `gate` usage. | `.factory/copy-audit.md` terminology table and source search. |
| F-1-18 | Standardized the output as “Markdown review packet” on first mention and “review packet” thereafter. Renamed packet headings to “Check findings” and “Reviewer sign-off.” | `cargo test claim_markdown_packet`; README/site copy audit. |
| F-1-19 | Replaced “Start for real” with “Install the CLI,” linked to `/#install`. | `@claim:one-click-demo`; dark mobile demo test asserts the link; screenshot: `.factory/evidence/polish-1-live-demo-mobile.png`. |
| F-1-20 | Replaced “Return to the gate” with “Return home.” | 404 browser test and live HTTP 404 screenshot. |
| F-1-21 | Replaced “realistic sample files” with the concrete Harbor Log copy and exact temp/check/result behavior. | README copy audit; `cargo test claim_bundled_demo`. |
| F-1-22 | Replaced “real checker” with “the same checker as the CLI.” | `cargo test claim_demo_recording_matches_bundled_cli`; live demo screenshot. |

## Controller-required acceptance work

- First screen: the job-led headline, audience, sample action, result, and three facts fit at both 1440×900 and 390×844. Mobile fact bottom: 826.1 px.
- Demo: the first action opens isolated `/?demo=1` in one click. Banner copy is “Demo — sample data, nothing is saved,” with Reset demo and Install the CLI. Seeded real storage stayed unchanged and all requests stayed same-origin.
- Claims: `.factory/claims.json` now has 23 unique claims. Every exact command passed individually from clean clone `/tmp/ios-review-gate-clean.Iony6n`.
- Routing: `/`, `/demo`, `/?demo=1`, `/privacy`, and `/terms` return 200. A random unknown route returns the complete 404 with HTTP 404. SPA navigation and history move focus to the new h1 and announce the title.
- Accessibility/mobile: 23 Playwright checks passed. Local and live axe CLI scans found zero violations on all four real routes. Dark/mobile axe, 200% reflow, 44 px controls, focus, reduced motion, and keyboard activation are covered.
- Privacy/offline: the demo preserves seeded real release/license/draft storage. Live cache inventory contained 11 same-origin static responses and `?demo=1` reloaded offline.
- Copy: `.factory/copy-audit.md` has every landing and README sentence, counts, banned-word result, and terminology table. `.factory/catalog-description.txt` is verb-first and 81 characters.

## Visual evidence

- `.factory/evidence/polish-1-live-home.png` — deployed desktop landing page.
- `.factory/evidence/polish-1-live-demo-mobile.png` — deployed 390 px dark demo and banner.
- `.factory/evidence/polish-1-live-404.png` — deployed standalone 404 shell.

## Live parity

Local/live SHA-256 values matched for `index.html`, hashed JS, hashed CSS, `sw.js`, and `404.html`. Live Lighthouse mobile: Performance 100, Accessibility 100, Best Practices 100, SEO 100; FCP 1.1 s, LCP 1.6 s, TBT 30 ms, CLS 0.026.

Unresolved findings: none.
