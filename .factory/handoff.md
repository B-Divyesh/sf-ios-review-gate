# Handoff — polish round 2

## Outcome

All findings from `review-1.md` and `review-2.md` are resolved and deployed at <https://ios-review-gate.sociobot.in>. The Rust CLI and static Vite site remain the original artifact and deployment classes. No known gap remains.

The repair preserves the warm drafting-sheet identity. It adds race-safe demo isolation, a real rule-code policy builder, complete claim coverage, and one shared built shell for routed pages and the 404.

## What changed

- Demo entry now aborts pending license verification and rejects stale asynchronous writes with a route-generation guard.
- Team buyers can choose all four reason codes from bundled `apple-2026.1`; the downloaded YAML passes the real CLI.
- Claims now cover the exact 1320×2868 sample and the generated version/build footer. There are 25 registered claims.
- Footer version comes from `Cargo.toml`; its build identifier comes from the deployed Git commit.
- The Vite-built 404 shares navigation, footer, styling, metadata, and accessibility behavior with the SPA.
- The last decorative release label was removed. First-screen job, audience, action, result, and three facts still fit at 390×844 and 1440×900.
- The catalog description is verb-first and 75 characters.

## Verification

From clean clone `/tmp/ios-review-gate-polish2-claims.8kCNE3/repo`, every exact command in `.factory/claims.json` passed independently: 25 of 25.

```sh
npm ci
npm test
npm run build
cargo +1.85.0 test --all-targets --locked
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo package --locked --allow-dirty
```

- `npm test`: 18 Rust tests, 5 Node contract tests, and 24 Playwright tests passed.
- Vite output: 20.72 KB JavaScript raw / 7.04 KB gzip; 11.17 KB CSS raw / 3.40 KB gzip.
- Production `verify-url.sh`: 200 response, correct title and language, one h1, main landmark, no missing alt text, no unlabeled buttons, zero console errors.
- Live routes: `/`, `/demo`, `/privacy`, and `/terms` returned 200 with unique titles and canonical URLs. A cold unknown path returned the designed page with HTTP 404 and canonical `/404`.
- Live navigation moved focus to the new h1 and browser Back restored the prior route.
- Live mobile/desktop first-screen fact bottoms: 826.1/844 px and 772.6/900 px. Neither viewport overflowed horizontally.
- Live demo race: a held real-license response was released after `?demo=1`; local and session storage stayed byte-for-byte unchanged.
- Live offline: cache `ios-review-gate-v7` contained 11 same-origin shell responses and reloaded the populated demo offline.
- Live policy: all four bundled reason controls rendered; selected YAML contained allowed codes and omitted an unchecked code.
- Live Lighthouse mobile: Performance 100, Accessibility 100, Best Practices 100, SEO 100; FCP 1.06 s, LCP 1.51 s, TBT 0 ms, CLS 0.026.
- Local/live SHA-256 matched for HTML, JavaScript, CSS, service worker, and 404 output.

Evidence:

- [Desktop home](evidence/polish-2-live-home/screenshot-desktop.png)
- [Mobile home](evidence/polish-2-live-home/screenshot-mobile.png)
- [Mobile demo](evidence/polish-2-live-demo-mobile.png)
- [Mobile policy builder](evidence/polish-2-live-policy-mobile.png)
- [Designed 404](evidence/polish-2-live-404.png)
- [Verifier report](evidence/polish-2-live-home/verify.json)
- [Finding-by-finding closure](polish-2.md)

## Run and deploy

Run `npm ci`, then `npm test`. Build the CLI and site with `npm run build`. The deployable static root is `dist/site`.

Deployment used the work-order configuration: `npm ci && npm run build:site`, then `/opt/fleet/lib/deploy-static.sh ios-review-gate dist/site`.

## Known gaps and next steps

None.
