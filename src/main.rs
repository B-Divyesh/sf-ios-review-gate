use clap::{Parser, Subcommand};
use ios_review_gate::{run_files, run_files_with_policy};
use std::{
    fs,
    path::PathBuf,
    process::ExitCode,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Parser)]
#[command(
    name = "ios-review-gate",
    version,
    about = "Check an iOS release and print its App Review packet",
    long_about = "Compare a local archive metadata export with release.yaml. Checks version, build, localized metadata, screenshots, privacy declarations, and queue timing. No upload or App Store Connect access."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Check release files and optionally write a Markdown packet
    Check {
        /// Path to the archive metadata JSON export
        #[arg(long)]
        metadata: PathBuf,
        /// Path to release.yaml
        #[arg(long)]
        release: PathBuf,
        /// Markdown packet path
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Optional Team policy YAML for approved reason codes and queue limits
        #[arg(long)]
        policy: Option<PathBuf>,
        /// Print the result as JSON
        #[arg(long)]
        json: bool,
    },
    /// Run a complete check on bundled sample data in a temporary folder
    Demo {
        /// Print the result as JSON
        #[arg(long)]
        json: bool,
    },
}

fn main() -> ExitCode {
    match Cli::parse().command {
        Command::Check {
            metadata,
            release,
            output,
            policy,
            json,
        } => finish(
            run_files_with_policy(&metadata, &release, output.as_deref(), policy.as_deref()),
            json,
        ),
        Command::Demo { json } => {
            let stamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let root = std::env::temp_dir().join(format!(
                "ios-review-gate-demo-{}-{}",
                std::process::id(),
                stamp
            ));
            let shots = root.join("screenshots/en-US/iphone-69");
            if let Err(error) = fs::create_dir_all(&shots) {
                eprintln!("Could not create demo folder: {error}");
                return ExitCode::from(1);
            }
            let metadata = root.join("metadata.json");
            let release = root.join("release.yaml");
            let packet = root.join("submission-packet.md");
            let screenshot = shots.join("home.jpg");
            let files: [(&std::path::Path, &[u8]); 3] = [
                (
                    &metadata,
                    include_bytes!("../examples/sample/metadata.json"),
                ),
                (&release, include_bytes!("../examples/sample/release.yaml")),
                (
                    &screenshot,
                    include_bytes!("../examples/sample/screenshots/en-US/iphone-69/home.jpg"),
                ),
            ];
            for (path, bytes) in files {
                if let Err(error) = fs::write(path, bytes) {
                    eprintln!("Could not write demo data: {error}");
                    return ExitCode::from(1);
                }
            }
            if !json {
                println!(
                    "Demo — sample data, nothing from your projects was read.\nWorkspace: {}",
                    root.display()
                );
            }
            finish(run_files(&metadata, &release, Some(&packet)), json)
        }
    }
}

fn finish(result: Result<ios_review_gate::GateReport, String>, json: bool) -> ExitCode {
    match result {
        Err(error) => {
            eprintln!("{error}\nFix the path or file contents, then run the command again.");
            ExitCode::from(1)
        }
        Ok(report) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&report).expect("serialize report")
                );
            } else {
                println!(
                    "{} — {} errors, {} warnings",
                    if report.passed { "PASS" } else { "HOLD" },
                    report.summary.errors,
                    report.summary.warnings
                );
                for item in &report.findings {
                    println!("- {:?} [{}] {}", item.severity, item.code, item.message);
                }
                if let Some(path) = &report.packet_path {
                    println!("Packet: {}", path.display());
                }
            }
            if report.passed {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(2)
            }
        }
    }
}
