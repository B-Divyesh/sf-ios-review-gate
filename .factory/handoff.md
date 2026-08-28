# Handoff — iOS Review Gate 0.1.0

## What shipped

- A Rust `clap` CLI with `check` and `demo` commands, JSON output, stable exit codes, and Markdown packet export.
- Version, build, bundle ID, localized field, screenshot path/count, privacy manifest, collected-data, required-reason API, and queue checks.
- Versioned `apple-2026.1` rules plus optional Team policy overrides for reason codes and queue history limits.
- A bundled Harbor Log sample. `ios-review-gate demo` copies it into a new temporary directory and prints the packet path.
- A static blueprint drafting-sheet site with `/`, `/demo`, `/privacy`, `/terms`, and a designed not-found state.
- A one-click browser demo that reads no real data, a self-hosted terminal recording, and an original generated hero illustration.
- The $39 one-time Team checkout, license return/restore/verification flow, daily verdict cache, local policy builder, and license removal.
- Metadata, sitemap, robots rules, static-host security headers, responsive light/dark treatments, reduced motion, and a service worker.

## Run and verify

```sh
npm ci
npm test
npm run build
cargo package
```

- `npm test`: 10 Rust integration tests, 3 Node contract tests, and 11 Playwright tests passed.
- `cargo fmt --check` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `npm run build` produced `target/release/ios-review-gate` (1.7 MB) and `dist/site/index.html`.
- `cargo package` produced a verified 95 KB crate archive.
- `/opt/fleet/lib/verify-url.sh` reported HTTP 200, one `h1`, `lang=en`, a main landmark, no missing alt text, and no console errors.
- Playwright axe checks found no serious or critical issues in light or dark treatments.
- Every command in `.factory/claims.json` passed from the bundled demo data.

## Measured budgets

Mobile Lighthouse against the production preview on 2026-08-28:

- Performance: 100
- Accessibility: 100
- Best practices: 100
- SEO: 100
- LCP: 1.7 s
- CLS: 0.028
- Total blocking time: 0 ms

Production payloads: 5.42 KB gzip JavaScript, 3.05 KB gzip CSS, 20 KB font, and 94 KB hero WebP.

## Known gaps and next steps

- The factory must register `ios-review-gate` with the Sociobot billing API before checkout can sell licenses.
- Apple policies change. Review and replace the dated `rules/apple-2026.1.yaml` snapshot before a later release.
- The CLI consumes a documented local JSON metadata export. It intentionally does not parse an IPA, upload builds, or call App Store Connect.
- The factory owns binary signing, registry publication, hosting, DNS, and deployment.
