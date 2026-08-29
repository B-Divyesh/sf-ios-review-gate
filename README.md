# iOS Review Gate

Check an iOS release and print a Markdown review packet before App Store submission. It is for small iOS teams that want one local packet before they queue a build.

The CLI reads an `.xcarchive` or `.ipa` and compares it with `release.yaml`. It checks identity, localized metadata, screenshots, privacy declarations, and queue timing. It does not upload a build or connect to App Store Connect.

## Install

Build the single binary with Rust 1.85 or newer.

```sh
cargo install --path .
ios-review-gate --help
```

Build the CLI with the command above.

## Try the bundled demo

```sh
cargo run -- demo
```

The command copies the bundled Harbor Log files to a temporary directory. It runs the check and prints the review-packet path. It does not read or change your files. Open the website sample at `https://ios-review-gate.sociobot.in/?demo=1`.

## Check a release

```sh
ios-review-gate check \
  --archive build/HarborLog.xcarchive \
  --release release.yaml \
  --output release-packet.md
```

Exit code `0` means the check passed. Exit code `2` means the inputs were read but checks failed. Invalid files or arguments use exit code `1`.

Use JSON output in automation.

```sh
ios-review-gate check --archive build/HarborLog.ipa --release release.yaml --json
```

The JSON object contains `passed`, `summary`, `findings`, `queue`, and `packet_path`.

## Inspect an archive

Write extracted identity and privacy declarations to JSON when another tool needs them.

```sh
ios-review-gate inspect \
  --archive build/HarborLog.xcarchive \
  --output build/metadata.json
```

The checker supports `.xcarchive` directories and `.ipa` files. It reads the app's property lists and privacy manifests locally. You can also pass an existing JSON export with `check --metadata`.

```json
{
  "bundle_id": "in.sociobot.harborlog",
  "version": "2.4.0",
  "build": "108",
  "privacy_manifest": true,
  "privacy_tracking": false,
  "privacy_collected_data": ["email_address"],
  "accessed_apis": [{"category": "UserDefaults", "reasons": ["CA92.1"]}]
}
```

`release.yaml` records the intended submission. See [`examples/sample/release.yaml`](examples/sample/release.yaml) for every field. Screenshot paths are relative to that file.

Every review packet names the rules version used. Version `apple-2026.1` checks supported locales, localized fields, screenshot sets and sizes, and the privacy manifest. It also checks collected-data answers and every declared reason code.

Supported locale identifiers, screenshot set keys, and exact portrait or landscape sizes are in [`rules/apple-2026.1.yaml`](rules/apple-2026.1.yaml). Use a listed locale such as `en-US` and a device key such as `iphone-69`. Unknown identifiers, unknown device keys, and decodable files at the wrong dimensions produce a HOLD. The bundled `iphone-69` sample is 1320×2868 pixels.

Every reason in `accessed_apis` must appear in the Apple rules for its API category. A Team policy can narrow that list with `approved_reason_codes`; it cannot make another code Apple-approved.

## Queue input

Each queued submission needs a version, build, submitted date, and one status:
`waiting_for_review`, `in_review`, `pending_developer_release`, or `completed`.
Unknown or incomplete queue entries produce a HOLD result. Review and buffer days
must be zero or positive and must fit a real calendar date. Each `submitted_on`
date must be on or before `intended_submission`.

## Develop and verify

```sh
npm ci
npm run dev
npm test
npm run build
```

Use `npm run build:site` for the site. Use `npm run build:cli` for the release binary.

## Privacy and price

The CLI has no telemetry and makes no network requests. Inputs and review packets remain on your machine. Core checks and review packet export work without a Team license. A Team license costs $39 once and adds local policy downloads. Team policies support queue histories beyond three submissions. Purchases and license verification use Sociobot's billing API.

Pass a Team policy from the website builder with `--policy team-policy.yaml`. See [`examples/team-policy.yaml`](examples/team-policy.yaml) for the file shape.

See the website [privacy page](https://ios-review-gate.sociobot.in/privacy) and [terms](https://ios-review-gate.sociobot.in/terms). Email [billing@sociobot.in](mailto:billing@sociobot.in) for billing help.

## Publishing

Do not publish from a worker. Before a factory release, run this command.

```sh
cargo package
```

## License

MIT. See [LICENSE](LICENSE).
