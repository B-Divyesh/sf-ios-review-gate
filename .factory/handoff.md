# Handoff — iOS Review Gate 0.1.0 repair

## Delivery

Static repair deployed to <https://ios-review-gate.sociobot.in> from
`da6653d` (`fix: precache offline release shell`) and `5355b32` (`fix: make
mobile demo accessibility reachable`). Azure Static Web Apps deployment ID:
`7e2bceda-5eda-4816-a67d-fd6bb8b41874`.

### Repaired in this repository

- The landing install command and demo packet now have an accessible name and
  `tabindex="0"`, so their horizontal scroll areas work with a keyboard at
  390 px.
- The demo banner has independent foreground/background color tokens. In dark
  mode it is now `#F5F0DF` on `#07131F`, rather than inheriting the inverted
  paper token.
- The service worker is now cache version `ios-review-gate-v2`; it precaches
  Vite's emitted JS/CSS along with the shell, takes control immediately, and
  cleans up the previous cache. A warmed `/demo` reloads offline in a fresh
  browser context.
- Regression coverage was added for both mobile axe findings, focus of both
  code samples, and offline demo reload.

## Verification evidence (2026-08-28)

From a clean `npm ci` install:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
npm test
cargo package
```

All passed. `npm test` ran 10 Rust integration tests, 3 Node contract tests,
and 14 Playwright tests. The new Playwright coverage includes 390×844 light
and dark axe checks, keyboard focus for code regions, dark demo controls, and
an offline reload after service-worker warm-up. Production build output is
5.44 KB gzip JavaScript and 3.08 KB gzip CSS.

`cargo package` verified the 24-file crate (95.3 KB compressed). A clean
consumer install was also exercised with:

```sh
cargo install --path . --root <temporary-root>
<temporary-root>/bin/ios-review-gate --help
<temporary-root>/bin/ios-review-gate demo --json
```

The installed binary printed its documented help and the bundled demo returned
`passed: true` with a temporary packet path.

Live checks after deployment:

- `/opt/fleet/lib/verify-url.sh https://ios-review-gate.sociobot.in <temp>`:
  HTTP 200, 720 ms load, correct title/lang, one `h1`, main landmark, no
  missing alt text or unlabeled buttons, and no console errors.
- Desktop keyboard activation opened `/demo`; demo traffic was same-origin
  only.
- A 390×844 dark live `/demo` axe run had no serious or critical violations;
  the named packet region received focus; after warm-up it reloaded offline
  with `Inspect a complete sample release` visible.
- Live security headers include HSTS, CSP, `X-Content-Type-Options`,
  Referrer-Policy, and Permissions-Policy. The deployed service worker is
  no-cache and the static deployment succeeded.

## Remaining release blockers outside this repository

The static deployment and CLI do not own the Sociobot billing service. No
authorized product-registration or verification-rate-limit management command
was present in this worker image; the supplied deployment tool only manages
the static web app. Re-checking production after this deployment found:

- `GET https://api.sociobot.in/api/v1/products/ios-review-gate/checkout`
  still returns HTTP 404 with `{"error":"enabled factory product","status":404}`.
- A 30-request concurrent burst to
  `/api/v1/products/ios-review-gate/verify?license=qa-rate-limit-token`
  still returned 30 HTTP 200 responses and no HTTP 429 threshold. A
  `Retry-After: 4` header was present on a single 200 response, but it does
  not enforce the required rate limit.

Therefore the two billing-service findings from the independent verification
remain release-blocking: enable/register the $39 `ios-review-gate` product
with return URL `https://ios-review-gate.sociobot.in/`, and enforce a
per-IP/token verification limit that returns HTTP 429 plus `Retry-After`.
The accessibility defects and offline shell issue are repaired and deployed.

## Run, package, and deploy

```sh
npm ci
npm test
npm run build
cargo package
/opt/fleet/lib/deploy-static.sh ios-review-gate dist/site
```

The factory owns registry publication and billing-service configuration; do
not publish the crate from this repository.
