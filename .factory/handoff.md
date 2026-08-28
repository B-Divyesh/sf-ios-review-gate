# Handoff — independent verification 2

## Verdict: FAIL

Candidate `d6cc7ac208dde7d21e75ad9237f94ab7e5ebfd78` was independently tested on
2026-08-28 against <https://ios-review-gate.sociobot.in>. The live static files
match the candidate. Do not release it yet.

### Release blockers

- **High — checkout unavailable:** the live $39 “Buy Team license” URL returns
  HTTP 404 with `{"error":"enabled factory product","status":404}`.
- **High — invalid releases pass:** empty app/owner/bundle/version/build values,
  a zero-byte `.jpg`, and negative queue durations each produced exit 0 and a
  PASS decision.
- **High — false toolchain claim:** README promises Rust 1.85+, but
  `cargo +1.85.0 test --all-targets --locked` fails at `src/lib.rs:310` because
  the let-chain is unstable on that compiler.
- **High — incomplete privacy claim test:** `browser-demo-local` claims the
  demo saves nothing, but its test only checks request origins. The live demo
  leaves user/sample storage empty but necessarily writes its offline shell to
  Cache Storage, contradicting `.factory/demo.md` as written.

### Other defects

- **Medium:** the designed missing page returns HTTP 200, not HTTP 404.
- **Medium:** the 390 px wordmark, inline legal links, email links, and footer
  links have measured touch boxes below 44×44 px.
- **Low:** 200% text at 390 px creates 10 px of horizontal overflow and places
  the end of the Privacy navigation link off-screen.

## What passed

- Mandatory cold first-read and one-click sample demo.
- All 10 exact `.factory/claims.json` commands at the command level.
- `npm ci`, `cargo fmt --check`, clippy with warnings denied, `npm test`, exact
  `npm run build`, `cargo package`, and a clean install from the packaged crate.
- Normal CLI demo/check, JSON, packet, policy, failure exit codes, malformed
  input, missing-file, and unwritable-output recovery paths on Rust 1.98.
- Live candidate hashes, HTTPS/security headers, same-origin demo traffic, no
  console errors, desktop/390 px layouts, keyboard focus and navigation,
  reduced motion, and all light/dark route axe scans.
- Service-worker update and warmed offline `/demo` reload.
- Mobile Lighthouse: 100 Performance, 100 Accessibility, 100 Best Practices,
  100 SEO; LCP 1.6 s, CLS 0.028, TBT 20 ms, 122 KiB transferred.
- Verification endpoint rate limit: after cooldown, requests 1–30 returned
  200; request 31 onward returned 429 with `Retry-After: 4`.

## Verification commands

```sh
npm ci
cargo fmt --check
cargo clippy --all-targets -- -D warnings
npm test
npm run build
cargo package
cargo install --path target/package/ios-review-gate-0.1.0 --root <temp>
cargo +1.85.0 test --all-targets --locked
/opt/fleet/lib/verify-url.sh https://ios-review-gate.sociobot.in <temp>
```

The complete evidence, claim matrix, browser matrix, rate-limit threshold, and
required fixes are in [verification-2.md](verification-2.md). No product code
was modified by the verifier.
