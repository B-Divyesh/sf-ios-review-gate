# Handoff — adversarial first-read review 2

## Outcome

Review 2 is complete for candidate
`d168878a6108510408f1c447c605610d33e22f19` and the matching deployment at
<https://ios-review-gate.sociobot.in>.

Verdict: **FAIL**. The complete evidence and fixes are in
[`review-2.md`](review-2.md).

No product code was changed. This work order only adds the review and replaces
the handoff with the current review result.

## Findings left for the repair round

1. F-2-1: a pending landing-page license response can write real license state
   after the one-click demo banner appears.
2. F-2-2: the paid builder promises approved-reason controls but downloads an
   empty `approved_reason_codes` map with no such controls.
3. F-2-3: the exact 1320×2868 sample dimension is not registered or asserted.
4. F-2-4 / F-1-6 (reopened): footer version/build text leaves the earlier
   unlisted-version defect unresolved.
5. F-2-5 / F-1-16 (reopened): “RELEASE / 02.4” leaves the earlier
   decorative-label defect partly unresolved.

All five are blocking under the requested zero-finding, complete-claims, demo
sandbox, and history rules.

## Verification performed

- Fresh live cold reads at 390×844 and 1440×900.
- Direct and one-click demo flows, reset, seeded storage, delayed-response
  race, request log, Cache Storage inventory, and offline reload.
- CLI demo from a temporary directory with an unchanged sentinel.
- Every exact command for all 23 registered claims from clean clone
  `/tmp/ios-review-gate-review2.NsQKKK/repo`: all passed.
- `npm test`: 17 Rust tests, five Node tests, and 23 Playwright tests passed.
- `npm run build`, `cargo fmt --check`, and
  `cargo clippy --all-targets --locked -- -D warnings`: passed.
- Live route/title/metadata/404/history/link crawl and Axe scan: passed apart
  from the documented copy/claim findings.
- `/opt/fleet/lib/verify-url.sh`: passed with no home-page console error.
- Live/local SHA-256 parity: matched for HTML, JS, CSS, service worker, and
  404 page.

## How to verify

```sh
npm ci
npm test
npm run build
cargo +1.85.0 test --all-targets --locked
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
```

For the key regression, seed a stored Team license, delay its verification
response on `/`, immediately open **Try it with sample data**, and release the
response after `/?demo=1` is visible. No real-storage key may be added or
changed.

## Known gaps

There are no unperformed items in the requested review. Product repair and
deployment are intentionally outside this reviewer work order.
