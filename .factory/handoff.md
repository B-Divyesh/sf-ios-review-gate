# Verification handoff — PASS

**Verified candidate:** `e5eb710ff9bc65061e8a6558b5c57d30e01fd9f4`
**Live URL:** <https://ios-review-gate.sociobot.in>
**Date:** 2026-08-29 UTC

## Outcome

**PASS.** Independent verification found no release-blocking defects. The live footer identifies build `e5eb710ff9bc`; deployed JavaScript and CSS are byte-identical to this candidate build.

## What was verified

- All 25 exact `.factory/claims.json` commands passed from the clean checkout, including the self-bootstrapping browser claims and Rust 1.85 MSRV.
- `npm ci`, `npm test`, `npm run build`, `cargo fmt --check`, Clippy with warnings denied, and `cargo package --locked` passed.
- A clean temporary consumer installed the packaged CLI and exercised public help and demo. Independent normal, HOLD, and invalid-input flows returned the documented exit codes and actionable output.
- The live page passed cold first-read and one-click demo checks. The sample is isolated, persistent-bannered, keyboard-operable, and reloads offline after service-worker update.
- Live demo request logging found only same-origin requests. CSP, security headers, and cache policy are present; hashed JavaScript is immutable and `sw.js` is no-cache.
- Live Axe scans had zero serious/critical findings. Desktop/mobile, visible keyboard focus, reduced motion, and Lighthouse passed. Lighthouse mobile: 100 Performance, 100 Accessibility, 100 Best Practices, 100 SEO; LCP 1.7 s, CLS 0.026.
- The optional Sociobot license endpoint rate-limited one invalid-token client after 13 accepted burst calls; call 14 returned 429 with `Retry-After: 4`.

## How to reproduce

```sh
npm ci
npm test
npm run build
cargo +1.85.0 test --all-targets --locked
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo package --locked
cargo run -- demo
```

See `.factory/verification-12.md` for complete evidence, claim commands, and live deployment checks.

## Known gaps / next steps

None found. Deployment and registry publishing remain factory responsibilities; do not publish the crate from this workspace.
