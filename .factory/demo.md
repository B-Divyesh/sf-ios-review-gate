# Demo

- Website: `https://ios-review-gate.sociobot.in/?demo=1` or local `/?demo=1`. `/demo` is an equivalent direct route.
- Entry: from the landing page, choose **Try it with sample data** once.
- CLI: `ios-review-gate demo` or `cargo run -- demo`.
- Sample: Harbor Log 2.4.0 build 108, one `en-US` localization, one iPhone screenshot, matching privacy answers, and no active submissions.
- Website reset: choose **Reset demo**. The sample returns to Harbor Log 2.4.0 with a PASS result.
- Website exit: choose **Install the CLI**. This leaves demo mode and opens the real install instructions.
- Browser isolation: the demo writes no personal release data to localStorage, sessionStorage, IndexedDB, or cookies. Its `ios-review-gate-v6` cache contains same-origin static shell files only. License storage is separate and is not read in demo mode.
- CLI reset: run the command again. Each run creates a new `ios-review-gate-demo-*` directory under the system temporary directory and prints its path.
- Isolation: the website demo is static and holds its sample in memory. The CLI demo reads compiled sample bytes and writes only to its new temporary directory.
