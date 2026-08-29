# Demo

- Website: `https://ios-review-gate.sociobot.in/demo` or local `/demo`. From the landing page, choose **Try it with sample data**.
- CLI: `ios-review-gate demo` or `cargo run -- demo`.
- Sample: Harbor Log 2.4.0 build 108, one `en-US` localization, one iPhone screenshot, matching privacy answers, and no active submissions.
- Website reset: choose **Reset demo**. The web demo writes no personal release data to localStorage, sessionStorage, IndexedDB, or cookies. Its `ios-review-gate-v5` service-worker cache stores only the static offline shell in Cache Storage. License storage is separate and is not read by the demo.
- CLI reset: run the command again. Each run creates a new `ios-review-gate-demo-*` directory under the system temporary directory and prints its path.
- Isolation: the website demo is static and holds its sample in memory. The CLI demo reads compiled sample bytes and writes only to its new temporary directory.
