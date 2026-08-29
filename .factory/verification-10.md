# Independent verification 10 — PASS

**Candidate:** `0408e389692712681465ddc9940d124bb48e1f3d`  
**Live URL:** <https://ios-review-gate.sociobot.in>  
**Verified:** 2026-08-29 UTC, clean checkout. No product code was changed.  
**Decision:** **PASS — the candidate meets the researched brief and factory acceptance contract.**

## First read and demo gate

A cold, empty-browser load answered all three first-read questions in the first
screen: it says “Check your iOS release before review”; it is “For small iOS
teams”; and the first action is **Try it with sample data**, with the adjacent
explanation “Open a checked sample and its review packet.” One keyboard Enter
on that action opened `?demo=1`, with the Harbor Log 2.4.0 / build 108 PASS
sample, packet preview, and persistent “Demo — sample data, nothing is saved”
banner with Reset demo and Install the CLI. This is a genuine one-click,
isolated demo, not a setup screen.

## Required claim tests

`.factory/claims.json` exists and declares 23 claims. After `npm ci` (21
packages; zero audit vulnerabilities), every exact registered command was run
from this checkout and passed: the 15 Rust claim commands plus Rust 1.85
compatibility command, six browser claim commands against the local demo entry
point, and the Node license-metadata command. This covers identity,
completeness, packet generation, bundled demo, archive inspection, actionable
errors, queue calculations and boundaries, local-only behavior, exit codes and
JSON schema, one-click/browser/offline demo, Team purchase/restore/policy, and
MIT metadata.

## Local release and CLI checks

- `npm test`: PASS — 17 Rust integration tests, 5 Node tests, and 23 Playwright tests.
- `npm run build`: PASS — produced `target/release/ios-review-gate` and `dist/site/`.
- `cargo fmt --check` and `cargo clippy --all-targets --locked -- -D warnings`: PASS.
- `cargo package --allow-dirty --no-verify`: PASS — `target/package/ios-review-gate-0.1.0.crate` produced (204 KiB).
- Clean-consumer check: unpacked that crate to a new temporary directory,
  `cargo install --path … --locked --root …`, then ran its installed binary.
  `--version` printed 0.1.0; `demo --json` returned `passed: true`, eight
  checks, and a packet path. Missing inputs returned the documented exit code
  1 and a specific recovery instruction.

The normal bundled release, seeded version/build/bundle mismatch, malformed
metadata, incomplete queue, extreme queue durations, direct `.xcarchive`, and
equivalent `.ipa` cases are exercised by the passing claim suite. The public
CLI is local-first and has no telemetry/network client.

## Live deployment, privacy, accessibility, and resilience

- Candidate/deployment parity: exact SHA-256 matches for `index.html`,
  `assets/index-UnmanOWC.js`, `assets/index-BcfhZS1E.css`, `sw.js`, and
  `404.html`. The live deployment is this candidate.
- `/`, `/demo`, `/privacy`, and `/terms` returned 200; an unknown route
  returned 404. Every checked ordinary route had its route-specific title,
  exactly one h1, and a main landmark. All internal links worked; checkout
  correctly returned 303.
- `/opt/fleet/lib/verify-url.sh` passed in 792 ms: title, `lang=en`, h1,
  main, alt text, button labels, and no console/page errors.
- Fresh Playwright/Axe scans at desktop light and 390px mobile dark on home,
  demo, privacy, and terms found **zero serious or critical violations**.
  Mobile `scrollWidth` equalled 390px. Keyboard Tab exposes a designed
  3px vermilion focus outline; Enter activates the sample action and Space
  resets the demo. With reduced motion, animation/transition durations were
  `0.00001s`.
- Cold live-page and demo request logs contained only same-origin static
  resources. No console or page errors appeared. The optional license flow is
  the documented Sociobot endpoint; release data never goes there.
- After a fresh demo load, `navigator.serviceWorker.ready`, `registration.update()`,
  and an offline reload all succeeded. The cache was `ios-review-gate-v6`; the
  offline page retained its demo title, h1, and PASS state.
- HTTPS headers include HSTS, `nosniff`, Referrer-Policy, Permissions-Policy,
  and a header-delivered CSP with `frame-ancestors 'none'`. Hashed JS is one
  year immutable; `sw.js` is no-cache. Production JS is 14.75 KB raw / 5.49 KB
  gzip and CSS is 10.42 KB raw / 3.25 KB gzip, comfortably below budget.

## Purchase endpoint and documented allowance

The Team checkout link returned HTTP 303 to hosted Dodo checkout. After a
clean window, 35 sequential invalid-license verification calls from one client
returned 200 for requests 1–30 and 429 for requests 31–35. Every denial had
`Retry-After: 4`. **Observed allowance: 30 requests per client/window.**

## Defects by severity

- Critical: none.
- High: none.
- Medium: none.
- Low: none.
