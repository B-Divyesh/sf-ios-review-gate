# Verification handoff 11 — FAIL

## Outcome

Candidate `62ff8d7ceb460f87fd38efa8c8f8a0874050b907` at
<https://ios-review-gate.sociobot.in> **FAILS independent verification**.
No product code was changed. Full evidence is in
`.factory/verification-11.md`.

## Release blockers and defects

1. **High:** the required pre-install clean-clone claim run was 18/25. Seven
   web claim commands exited 127 with `vite: not found`. They all pass after
   `npm ci`, but the acceptance contract makes the initial failures blocking.
2. **Medium:** the cache rule targets `/assets/index-*`, while Vite emits
   `/assets/main-*`. Live hashed JS/CSS therefore receive only
   `public, must-revalidate, max-age=30` instead of one-year immutable caching.

## Evidence summary

- First-read and one-click sample gates: PASS.
- After `npm ci`, `npm test`: PASS — 18 Rust, 5 Node, 24 Playwright tests.
- `cargo +1.85.0 test --all-targets --locked`: PASS.
- `cargo fmt --check`: PASS.
- `cargo clippy --all-targets --locked -- -D warnings`: PASS.
- `npm run build`: PASS; `dist/site/` produced.
- `cargo package --locked --allow-dirty`: PASS; fresh consumer install and
  `inspect`, `check`, `demo`, JSON, packet, and invalid-input recovery worked.
- Live/local hashes match for HTML, JS, CSS, service worker, and 404.
- Desktop/mobile, light/dark Axe scans: zero serious/critical findings.
- Keyboard, focus, 44 px targets, 200% text, reduced motion: PASS.
- Demo request/storage privacy and service-worker offline reload: PASS.
- Checkout: 303 to hosted checkout. Verify allowance: 30 requests; request 31
  returns 429 with `Retry-After: 4`.
- Mobile Lighthouse: 100 performance, 100 accessibility, 100 best practices,
  100 SEO; LCP 1.6 s, TBT 20 ms, CLS 0.026.

## Reproduce

From a fresh clone at the candidate, run every `test` value in
`.factory/claims.json` before installing dependencies to reproduce the seven
blocking failures. Then run:

```sh
npm ci
npm test
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
npm run build
cargo package --locked --allow-dirty
```

To verify the caching defect:

```sh
curl -I https://ios-review-gate.sociobot.in/assets/main-B1aPlnQU.js
curl -I https://ios-review-gate.sociobot.in/assets/main-FbCzFwnk.css
```

## Next steps

Make each claim command runnable in the required clean-clone context. Correct
the asset cache route, redeploy, confirm immutable headers, and repeat all 25
claims before the general suite.
