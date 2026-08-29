use assert_cmd::Command;
use image::{DynamicImage, ImageFormat};
use ios_review_gate::{
    ArtifactMetadata, Release, Severity, TeamPolicy, check, check_with_policy, run_files,
};
use std::{fs, path::Path};

fn sample() -> (ArtifactMetadata, Release) {
    let metadata = serde_json::from_str(include_str!("../examples/sample/metadata.json")).unwrap();
    let release = serde_yaml::from_str(include_str!("../examples/sample/release.yaml")).unwrap();
    (metadata, release)
}

fn write_sample_workspace(
    root: &Path,
) -> (std::path::PathBuf, std::path::PathBuf, std::path::PathBuf) {
    let metadata = root.join("metadata.json");
    let release = root.join("release.yaml");
    let screenshot = root.join("screenshots/en-US/iphone-69/home.jpg");
    fs::create_dir_all(screenshot.parent().unwrap()).unwrap();
    fs::write(
        &metadata,
        include_bytes!("../examples/sample/metadata.json"),
    )
    .unwrap();
    fs::write(&release, include_bytes!("../examples/sample/release.yaml")).unwrap();
    fs::write(
        &screenshot,
        include_bytes!("../examples/sample/screenshots/en-US/iphone-69/home.jpg"),
    )
    .unwrap();
    (metadata, release, screenshot)
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
    let temp = tempfile::tempdir().unwrap();
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
        .unwrap()
        .splice(
            ..,
            [
                "missing.bmp".into(),
                "truncated.jpg".into(),
                "truncated.png".into(),
            ],
        );
    fs::write(temp.path().join("truncated.jpg"), [0xff, 0xd8, 0xff, 0xd9]).unwrap();
    fs::write(
        temp.path().join("truncated.png"),
        [0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a],
    )
    .unwrap();
    release
        .screenshots
        .insert("fr-FR".into(), Default::default());
    let report = check(&metadata, &release, temp.path());
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
    let invalid_images: Vec<_> = report
        .findings
        .iter()
        .filter(|item| item.code == "screenshots.invalid_image")
        .collect();
    assert_eq!(
        invalid_images.len(),
        2,
        "both truncated image formats must HOLD"
    );
    for path in ["truncated.jpg", "truncated.png"] {
        assert!(
            invalid_images
                .iter()
                .any(|item| item.message.contains(path)),
            "missing invalid-image finding for {path}"
        );
    }
    assert!(!report.passed, "truncated screenshots must never PASS");

    let (metadata, mut one_pixel_release) = sample();
    let one_pixel = temp.path().join("one-by-one.jpg");
    DynamicImage::new_rgb8(1, 1)
        .save_with_format(&one_pixel, ImageFormat::Jpeg)
        .unwrap();
    one_pixel_release
        .screenshots
        .get_mut("en-US")
        .unwrap()
        .get_mut("iphone-69")
        .unwrap()[0] = "one-by-one.jpg".into();
    let one_pixel_report = check(&metadata, &one_pixel_release, temp.path());
    assert!(
        one_pixel_report
            .findings
            .iter()
            .any(|item| item.code == "screenshots.dimensions" && item.message.contains("1×1")),
        "a decodable 1×1 JPEG must not be accepted as an iPhone screenshot"
    );
    assert!(
        !one_pixel_report
            .findings
            .iter()
            .any(|item| item.code == "screenshots.invalid_image"),
        "the 1×1 fixture proves dimension validation, not decode validation"
    );
    assert!(!one_pixel_report.passed, "a 1×1 screenshot must never PASS");

    let (metadata, mut unknown_device_release) = sample();
    fs::write(
        temp.path().join("valid-screenshot.jpg"),
        include_bytes!("../examples/sample/screenshots/en-US/iphone-69/home.jpg"),
    )
    .unwrap();
    let device_sets = unknown_device_release.screenshots.get_mut("en-US").unwrap();
    device_sets.remove("iphone-69");
    device_sets.insert("not-a-device".into(), vec!["valid-screenshot.jpg".into()]);
    let unknown_device_report = check(&metadata, &unknown_device_release, temp.path());
    assert!(
        unknown_device_report
            .findings
            .iter()
            .any(|item| item.code == "screenshots.device_unknown"),
        "an unknown device-set key must be rejected"
    );
    assert!(
        !unknown_device_report
            .findings
            .iter()
            .any(|item| item.code == "screenshots.dimensions"),
        "the known-good image keeps this fixture focused on the device-set key"
    );
    assert!(
        !unknown_device_report.passed,
        "an unknown screenshot device set must never PASS"
    );

    let (mut metadata, release) = sample();
    metadata.accessed_apis[0].reasons.push("INVALID.1".into());
    let mixed_reason_report = check(&metadata, &release, Path::new("examples/sample"));
    assert!(
        mixed_reason_report.findings.iter().any(|item| {
            item.code == "privacy.reason_invalid"
                && item.message.contains("UserDefaults")
                && item.message.contains("INVALID.1")
        }),
        "every declared reason must be checked, even beside an approved reason"
    );
    assert!(
        !mixed_reason_report.passed,
        "an invalid sibling reason must never be hidden by an approved reason"
    );

    let (mut metadata, release) = sample();
    metadata.accessed_apis[0].reasons = vec!["INVALID.1".into()];
    let policy: TeamPolicy = serde_yaml::from_str(
        "name: Unsafe policy\nmax_active_submissions: 8\nadditional_reason_codes:\n  UserDefaults: [INVALID.1]\n",
    )
    .unwrap();
    let policy_bypass_report = check_with_policy(
        &metadata,
        &release,
        Path::new("examples/sample"),
        Some(&policy),
    );
    assert!(
        policy_bypass_report.findings.iter().any(|item| {
            item.code == "privacy.reason_invalid" && item.message.contains("INVALID.1")
        }),
        "a Team policy must not make a non-Apple reason valid"
    );
    assert!(
        policy_bypass_report.findings.iter().any(|item| {
            item.code == "policy.reason_invalid" && item.message.contains("INVALID.1")
        }),
        "the unsafe Team policy value must be named for repair"
    );
    assert!(
        !policy_bypass_report.passed,
        "a Team policy must never expand the Apple reason allowlist"
    );

    let (metadata, mut unknown_locale_release) = sample();
    let localized = unknown_locale_release.locales.remove("en-US").unwrap();
    unknown_locale_release
        .locales
        .insert("INVALID_LOCALE".into(), localized);
    let screenshots = unknown_locale_release.screenshots.remove("en-US").unwrap();
    unknown_locale_release
        .screenshots
        .insert("INVALID_LOCALE".into(), screenshots);
    let unknown_locale_report = check(
        &metadata,
        &unknown_locale_release,
        Path::new("examples/sample"),
    );
    assert!(
        unknown_locale_report.findings.iter().any(|item| {
            item.code == "locales.identifier_unknown" && item.message.contains("INVALID_LOCALE")
        }),
        "a nonexistent App Store locale must be rejected"
    );
    assert!(
        !unknown_locale_report.passed,
        "a release with only an unknown locale must never PASS"
    );
}

#[test]
fn invalid_identity_images_and_queue_durations_cannot_pass() {
    let (mut metadata, mut release) = sample();
    metadata.bundle_id.clear();
    metadata.version.clear();
    metadata.build.clear();
    release.app_name.clear();
    release.bundle_id.clear();
    release.version.clear();
    release.build.clear();
    release.submitted_by.clear();
    release.queue.typical_review_days = -5;
    release.queue.buffer_days = -7;

    let temp = tempfile::tempdir().unwrap();
    let screenshot = temp.path().join("home.jpg");
    fs::write(&screenshot, []).unwrap();
    release
        .screenshots
        .get_mut("en-US")
        .unwrap()
        .get_mut("iphone-69")
        .unwrap()[0] = "home.jpg".into();

    let report = check(&metadata, &release, temp.path());
    assert!(!report.passed);
    for code in [
        "identity.app_name_missing",
        "identity.owner_missing",
        "identity.bundle_id_missing",
        "identity.version_missing",
        "identity.build_missing",
        "identity.release_bundle_id_missing",
        "identity.release_version_missing",
        "identity.release_build_missing",
        "screenshots.invalid_image",
        "queue.review_days_negative",
        "queue.buffer_days_negative",
    ] {
        assert!(
            report.findings.iter().any(|item| item.code == code),
            "missing {code}"
        );
    }
}

#[test]
fn claim_queue_input_validation_rejects_incomplete_or_unknown_entries() {
    let (metadata, mut release) = sample();
    release.queue.active_submissions.push(
        serde_yaml::from_str(
            "version: \"\"\nbuild: \"\"\nstatus: typo_in_reveiw\nsubmitted_on: 2026-09-03",
        )
        .unwrap(),
    );

    let report = check(&metadata, &release, Path::new("examples/sample"));
    assert!(
        !report.passed,
        "an invalid queue entry must never produce PASS"
    );
    assert_eq!(
        report.queue.active_submissions, 1,
        "unknown status is conservative"
    );
    for code in [
        "queue.version_missing",
        "queue.build_missing",
        "queue.status_invalid",
    ] {
        assert!(
            report.findings.iter().any(|item| item.code == code),
            "missing {code}"
        );
    }

    let (metadata, mut future_release) = sample();
    future_release.queue.active_submissions.push(
        serde_yaml::from_str(
            "version: 2.3.9\nbuild: \"107\"\nstatus: in_review\nsubmitted_on: 2030-01-01",
        )
        .unwrap(),
    );
    let future_report = check(&metadata, &future_release, Path::new("examples/sample"));
    assert!(
        future_report.findings.iter().any(|item| {
            item.code == "queue.submitted_after_intended"
                && item.message.contains("2030-01-01")
                && item.message.contains("2026-09-02")
        }),
        "a queue entry after the intended submission must be rejected"
    );
    assert!(
        !future_report.passed,
        "impossible queue chronology must never produce PASS"
    );
}

#[test]
fn claim_queue_date_limits_hold_without_panicking() {
    for (field, value, code) in [
        (
            "typical_review_days",
            i64::MAX,
            "queue.review_days_out_of_range",
        ),
        (
            "typical_review_days",
            i64::MIN,
            "queue.review_days_negative",
        ),
        ("buffer_days", i64::MAX, "queue.buffer_days_out_of_range"),
        ("buffer_days", i64::MIN, "queue.buffer_days_negative"),
    ] {
        let (metadata, mut release) = sample();
        match field {
            "typical_review_days" => release.queue.typical_review_days = value,
            "buffer_days" => release.queue.buffer_days = value,
            _ => unreachable!(),
        }
        let report = check(&metadata, &release, Path::new("examples/sample"));
        assert!(!report.passed, "{field}={value} must HOLD");
        assert!(
            report.findings.iter().any(|item| item.code == code),
            "missing {code} for {field}={value}"
        );
    }
}

#[test]
fn claim_team_policy_supports_queue_history_beyond_three_submissions() {
    let (metadata, mut release) = sample();
    release.queue.active_submissions = (0..5)
        .map(|index| serde_yaml::from_str(&format!("version: 2.3.{index}\nbuild: \"{index}\"\nstatus: completed\nsubmitted_on: 2026-08-0{}", index + 1)).unwrap())
        .collect();
    let policy: TeamPolicy = serde_yaml::from_str("name: Mobile team\nmax_active_submissions: 8\napproved_reason_codes:\n  UserDefaults: [CA92.1]\n").unwrap();
    let default_report = check(&metadata, &release, Path::new("examples/sample"));
    assert!(
        default_report
            .findings
            .iter()
            .any(|item| item.code == "queue.history_limit"),
        "the free/default history retains its three-submission limit"
    );
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
    let today = chrono::Utc::now().date_naive();
    assert!(report.passed);
    assert!(
        packet.contains("Decision: PASS")
            && packet.contains(&format!("Generated: {today}"))
            && packet.contains("Rules: `apple-2026.1`")
            && packet.contains("## Queue plan")
            && packet.contains("## Decision record")
    );
}

#[test]
fn claim_core_gate_runs_without_team_license_and_writes_packet() {
    let temp = tempfile::tempdir().unwrap();
    let (metadata, release, _) = write_sample_workspace(temp.path());
    let packet = temp.path().join("packet.md");
    let output = Command::cargo_bin("ios-review-gate")
        .unwrap()
        .arg("check")
        .arg("--metadata")
        .arg(&metadata)
        .arg("--release")
        .arg(&release)
        .arg("--output")
        .arg(&packet)
        .arg("--json")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let report: serde_json::Value = serde_json::from_slice(&output).unwrap();

    assert_eq!(report["passed"], true);
    assert_eq!(report["summary"]["checks"].as_u64(), Some(8));
    assert!(report["policy"].is_null());
    assert_eq!(report["packet_path"].as_str(), packet.to_str());
    assert!(
        fs::read_to_string(packet)
            .unwrap()
            .contains("## Decision record")
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
fn claim_demo_recording_matches_bundled_cli() {
    let output = Command::cargo_bin("ios-review-gate")
        .unwrap()
        .arg("demo")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();
    let workspace = text
        .lines()
        .find_map(|line| line.strip_prefix("Workspace: "))
        .map(std::path::PathBuf::from)
        .expect("the demo reports its workspace");
    let packet = text
        .lines()
        .find_map(|line| line.strip_prefix("Packet: "))
        .map(std::path::PathBuf::from)
        .expect("the demo reports its packet");

    assert_eq!(
        fs::read(workspace.join("metadata.json")).unwrap(),
        include_bytes!("../examples/sample/metadata.json").as_slice()
    );
    assert_eq!(
        fs::read(workspace.join("release.yaml")).unwrap(),
        include_bytes!("../examples/sample/release.yaml").as_slice()
    );
    assert!(packet.is_file());

    let check_output = Command::cargo_bin("ios-review-gate")
        .unwrap()
        .arg("check")
        .arg("--metadata")
        .arg(workspace.join("metadata.json"))
        .arg("--release")
        .arg(workspace.join("release.yaml"))
        .arg("--json")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let report: serde_json::Value = serde_json::from_slice(&check_output).unwrap();
    assert_eq!(report["passed"], true);
    assert_eq!(report["summary"]["checks"].as_u64(), Some(8));

    let recording = include_str!("../site/public/assets/terminal-recording.svg");
    assert!(recording.contains("ios-review-gate demo"));
    for expected in [
        "Demo — sample data, nothing from your projects was read.",
        "PASS — 0 errors, 0 warnings",
    ] {
        assert!(
            recording.contains(expected),
            "recording is missing {expected}"
        );
        assert!(text.contains(expected), "CLI demo is missing {expected}");
    }
}

#[test]
fn claim_actionable_mismatch_error_names_values_and_fix() {
    let temp = tempfile::tempdir().unwrap();
    let (metadata, release, _) = write_sample_workspace(temp.path());
    let mismatched = include_str!("../examples/sample/metadata.json")
        .replace("\"version\": \"2.4.0\"", "\"version\": \"2.3.9\"");
    fs::write(&metadata, mismatched).unwrap();

    let output = Command::cargo_bin("ios-review-gate")
        .unwrap()
        .arg("check")
        .arg("--metadata")
        .arg(&metadata)
        .arg("--release")
        .arg(&release)
        .arg("--json")
        .assert()
        .code(2)
        .get_output()
        .stdout
        .clone();
    let report: serde_json::Value = serde_json::from_slice(&output).unwrap();
    let finding = report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["code"] == "identity.version")
        .expect("version mismatch finding");
    let message = finding["message"].as_str().unwrap();
    let fix = finding["fix"].as_str().unwrap();

    assert!(message.contains("2.3.9") && message.contains("2.4.0"));
    assert!(fix.contains("Set release.yaml to the archived marketing version."));
}

#[test]
fn claim_cli_exit_codes() {
    let temp = tempfile::tempdir().unwrap();
    let (metadata, release, _) = write_sample_workspace(temp.path());
    Command::cargo_bin("ios-review-gate")
        .unwrap()
        .arg("check")
        .arg("--metadata")
        .arg(&metadata)
        .arg("--release")
        .arg(&release)
        .assert()
        .success();

    fs::write(
        &metadata,
        include_str!("../examples/sample/metadata.json").replace("\"108\"", "\"999\""),
    )
    .unwrap();
    Command::cargo_bin("ios-review-gate")
        .unwrap()
        .arg("check")
        .arg("--metadata")
        .arg(&metadata)
        .arg("--release")
        .arg(&release)
        .assert()
        .code(2);

    Command::cargo_bin("ios-review-gate")
        .unwrap()
        .arg("check")
        .arg("--metadata")
        .arg(temp.path().join("nowhere.json"))
        .arg("--release")
        .arg(&release)
        .assert()
        .code(1)
        .stderr(predicates::str::contains("Fix the path or file contents"));
}

#[test]
fn claim_cli_json_schema() {
    let temp = tempfile::tempdir().unwrap();
    let (metadata, release, _) = write_sample_workspace(temp.path());
    let output = Command::cargo_bin("ios-review-gate")
        .unwrap()
        .arg("check")
        .arg("--metadata")
        .arg(&metadata)
        .arg("--release")
        .arg(&release)
        .arg("--json")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let value: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert!(value["passed"].is_boolean());
    assert!(value["summary"].is_object());
    assert!(value["findings"].is_array());
    assert!(value["queue"].is_object());
    assert!(value["packet_path"].is_null());
}
