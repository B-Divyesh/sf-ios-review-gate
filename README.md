# iOS Review Gate

Check an iOS release and print a dated review packet before App Store submission. It is for small iOS teams that want a repository-local decision record instead of another upload service.

The CLI compares an archive metadata export with `release.yaml`. It checks version and build values, localized metadata, screenshots, privacy declarations, and queue timing. It does not upload a build or connect to App Store Connect.

## Install

Build the single binary with Rust 1.85 or newer:

```sh
cargo install --path .
ios-review-gate --help
```

The package starts at version `0.1.0`. The factory publishes release binaries after review.

## Try the bundled demo

```sh
cargo run -- demo
```

The command copies realistic sample files to a temporary directory, runs the gate, and prints the packet path. It does not read or change your files. The website demo is at `https://ios-review-gate.sociobot.in/demo`.

## Check a release

```sh
ios-review-gate check \
  --metadata build/metadata.json \
  --release release.yaml \
  --output release-packet.md
```

Exit code `0` means the gate passed. Exit code `2` means the inputs were read but checks failed. Invalid files or arguments use exit code `1`.

Use JSON output in automation:

```sh
ios-review-gate check --metadata build/metadata.json --release release.yaml --json
```

The JSON object contains `passed`, `summary`, `findings`, `queue`, and `packet_path`.

## Input files

`metadata.json` is a local export from your archive or IPA inspection step:

```json
{
  "bundle_id": "com.example.harborlog",
  "version": "2.4.0",
  "build": "108",
  "privacy_manifest": true,
  "privacy_tracking": false,
  "privacy_collected_data": ["email_address"],
  "accessed_apis": [{"category": "UserDefaults", "reasons": ["CA92.1"]}]
}
```

`release.yaml` records the intended submission. See [`examples/sample/release.yaml`](examples/sample/release.yaml) for every field. Screenshot paths are relative to that file.

Rules are versioned in the binary and named in every packet. Version `apple-2026.1` covers required localized fields, screenshot existence, privacy manifest presence, collected-data agreement, and approved reason codes for declared API categories.

## Queue input

Each queued submission needs a version, build, submitted date, and one status:
`waiting_for_review`, `in_review`, `pending_developer_release`, or `completed`.
Unknown or incomplete queue entries produce a HOLD result. Review and buffer days
must be zero or positive and must fit a real calendar date.

## Develop and verify

```sh
npm ci
npm run dev
npm test
npm run build
```

`npm test` runs Rust unit and integration tests plus site tests. `npm run build` produces the single binary in `target/release/` and the deployable site at `dist/site/`.

Deploy `dist/site/` to any static host with SPA fallback to `index.html`. The included Static Web Apps configuration supplies fallback, cache, and security headers.

Build only the deployable site with `npm run build:site`. Build only the release binary with `npm run build:cli`.

## Privacy and price

The CLI has no telemetry and makes no network requests. Inputs and packets remain on your machine. Core checks and packet export work without a Team license. A Team license costs $39 once and adds local policy downloads. Team policies support queue histories beyond three submissions. Purchases and license verification use Sociobot's billing API.

Pass a Team policy from the website builder with `--policy team-policy.yaml`. See [`examples/team-policy.yaml`](examples/team-policy.yaml) for the file shape.

See the website [privacy page](https://ios-review-gate.sociobot.in/privacy) and [terms](https://ios-review-gate.sociobot.in/terms). Dodo is the merchant of record for Sociobot purchases and handles refunds.

## Publishing

Do not publish from a worker. Before a factory release, run:

```sh
cargo package
```

## License

MIT. See [LICENSE](LICENSE).
