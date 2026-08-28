use assert_cmd::Command;
use ios_review_gate::{
    ArtifactMetadata, Release, Severity, TeamPolicy, check, check_with_policy, run_files,
};
use std::{fs, path::Path};

fn sample() -> (ArtifactMetadata, Release) {
    let metadata = serde_json::from_str(include_str!("../examples/sample/metadata.json")).unwrap();
    let release = serde_yaml::from_str(include_str!("../examples/sample/release.yaml")).unwrap();
    (metadata, release)
}

#[test]
fn documented_sample_passes() {
    let (metadata, release) = sample();
    assert!(check(&metadata, &release, Path::new("examples/sample")).passed);
}

#[test]
fn claim_identity_consistency_catches_version_build_and_bundle_id() {
    let (mut metadata, release) = sample();
    metadata.version = "2.3.9".into();
    metadata.build = "107".into();
    metadata.bundle_id = "in.example.wrong".into();
    let report = check(&metadata, &release, Path::new("examples/sample"));
    for code in ["identity.version", "identity.build", "identity.bundle_id"] {
        assert!(
            report
                .findings
                .iter()
                .any(|item| item.code == code && item.severity == Severity::Error)
        );
    }
}

#[test]
fn claim_release_completeness_catches_seeded_metadata_privacy_and_screenshot_errors() {
    let (mut metadata, mut release) = sample();
    metadata.privacy_manifest = false;
    metadata.privacy_tracking = true;
    metadata.privacy_collected_data = vec!["precise_location".into()];
    metadata.accessed_apis[0].reasons.clear();
    release.locales.get_mut("en-US").unwrap().subtitle.clear();
    release.locales.get_mut("en-US").unwrap().name = "A".repeat(31);
    release
        .screenshots
        .get_mut("en-US")
        .unwrap()
        .get_mut("iphone-69")
        .unwrap()[0] = "missing.bmp".into();
    release
        .screenshots
        .insert("fr-FR".into(), Default::default());
    let report = check(&metadata, &release, Path::new("examples/sample"));
    for code in [
        "privacy.manifest_missing",
        "privacy.tracking_mismatch",
        "privacy.data_mismatch",
        "privacy.reason_missing",
        "locales.field_empty",
        "locales.field_too_long",
        "screenshots.format",
        "screenshots.locale_orphan",
    ] {
        assert!(
            report.findings.iter().any(|item| item.code == code),
            "missing {code}"
        );
    }
}

#[test]
fn team_policy_extends_reason_codes_and_queue_history() {
    let (mut metadata, mut release) = sample();
    metadata.accessed_apis[0].reasons = vec!["TEAM.1".into()];
    release.queue.active_submissions = (0..5)
        .map(|index| serde_yaml::from_str(&format!("version: 2.3.{index}\nbuild: \"{index}\"\nstatus: completed\nsubmitted_on: 2026-08-0{}", index + 1)).unwrap())
        .collect();
    let policy: TeamPolicy = serde_yaml::from_str("name: Mobile team\nmax_active_submissions: 8\nadditional_reason_codes:\n  UserDefaults: [TEAM.1]\n").unwrap();
    let report = check_with_policy(
        &metadata,
        &release,
        Path::new("examples/sample"),
        Some(&policy),
    );
    assert!(report.passed);
    assert_eq!(report.policy.as_deref(), Some("Mobile team"));
    assert!(!report.findings.iter().any(|item| item.code == "privacy.reason_missing" || item.code == "queue.history_limit"));
}

#[test]
fn claim_queue_plan_accounts_for_an_active_submission() {
    let (metadata, mut release) = sample();
    release.queue.active_submissions.push(
        serde_yaml::from_str(
            "version: 2.3.9\nbuild: \"107\"\nstatus: in_review\nsubmitted_on: 2026-08-27",
        )
        .unwrap(),
    );
    let report = check(&metadata, &release, Path::new("examples/sample"));
    assert_eq!(report.queue.estimated_decision.to_string(), "2026-09-06");
    assert_eq!(report.queue.buffered_decision.to_string(), "2026-09-08");
}

#[test]
fn claim_cli_local_has_no_network_client() {
    let manifest = include_str!("../Cargo.toml");
    let sources = format!(
        "{}{}",
        include_str!("../src/main.rs"),
        include_str!("../src/lib.rs")
    );
    for forbidden in [
        "reqwest",
        "ureq",
        "TcpStream",
        "UdpSocket",
        "api.sociobot.in",
    ] {
        assert!(
            !manifest.contains(forbidden) && !sources.contains(forbidden),
            "network client marker found: {forbidden}"
        );
    }
}

#[test]
fn claim_markdown_packet_writes_dated_decision_record() {
    let temp = tempfile::tempdir().unwrap();
    let release = temp.path().join("release.yaml");
    let metadata = temp.path().join("metadata.json");
    let output = temp.path().join("packet.md");
    fs::write(&release, include_str!("../examples/sample/release.yaml")).unwrap();
    fs::write(&metadata, include_str!("../examples/sample/metadata.json")).unwrap();
    let shot = temp.path().join("screenshots/en-US/iphone-69");
    fs::create_dir_all(&shot).unwrap();
    fs::write(
        shot.join("home.jpg"),
        include_bytes!("../examples/sample/screenshots/en-US/iphone-69/home.jpg"),
    )
    .unwrap();
    let report = run_files(&metadata, &release, Some(&output)).unwrap();
    let packet = fs::read_to_string(output).unwrap();
    assert!(report.passed);
    assert!(
        packet.contains("Decision: PASS")
            && packet.contains("Rules: `apple-2026.1`")
            && packet.contains("## Queue plan")
            && packet.contains("## Decision record")
    );
}

#[test]
fn claim_bundled_demo_runs_in_a_new_temp_workspace() {
    let output = Command::cargo_bin("ios-review-gate")
        .unwrap()
        .arg("demo")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();
    assert!(text.contains("sample data, nothing from your projects was read"));
    let packet_line = text
        .lines()
        .find(|line| line.starts_with("Packet: "))
        .unwrap();
    assert!(Path::new(packet_line.trim_start_matches("Packet: ")).is_file());
}

#[test]
fn invalid_input_has_actionable_error_and_exit_one() {
    Command::cargo_bin("ios-review-gate")
        .unwrap()
        .args([
            "check",
            "--metadata",
            "nowhere.json",
            "--release",
            "release.yaml",
        ])
        .assert()
        .code(1)
        .stderr(predicates::str::contains("Fix the path or file contents"));
}

#[test]
fn failed_gate_exits_two_and_json_is_parseable() {
    let temp = tempfile::tempdir().unwrap();
    let metadata = temp.path().join("metadata.json");
    let release = temp.path().join("release.yaml");
    fs::write(
        &metadata,
        include_str!("../examples/sample/metadata.json").replace("\"108\"", "\"999\""),
    )
    .unwrap();
    fs::write(&release, include_str!("../examples/sample/release.yaml")).unwrap();
    let output = Command::cargo_bin("ios-review-gate")
        .unwrap()
        .args([
            "check",
            "--metadata",
            metadata.to_str().unwrap(),
            "--release",
            release.to_str().unwrap(),
            "--json",
        ])
        .assert()
        .code(2)
        .get_output()
        .stdout
        .clone();
    let value: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(value["passed"], false);
}
