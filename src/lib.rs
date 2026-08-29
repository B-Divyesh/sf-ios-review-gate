use chrono::{Days, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

pub const RULESET: &str = "apple-2026.1";

#[derive(Debug, Deserialize)]
struct RuleSet {
    id: String,
    localized_field_limits: BTreeMap<String, usize>,
    screenshots_per_set: ScreenshotLimits,
    required_reason_apis: BTreeMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct ScreenshotLimits {
    minimum: usize,
    maximum: usize,
}

fn official_rules() -> RuleSet {
    let rules: RuleSet = serde_yaml::from_str(include_str!("../rules/apple-2026.1.yaml"))
        .expect("the bundled rule file must be valid");
    debug_assert_eq!(rules.id, RULESET);
    rules
}

#[derive(Debug, Deserialize)]
pub struct ArtifactMetadata {
    pub bundle_id: String,
    pub version: String,
    pub build: String,
    pub privacy_manifest: bool,
    #[serde(default)]
    pub privacy_tracking: bool,
    #[serde(default)]
    pub privacy_collected_data: Vec<String>,
    #[serde(default)]
    pub accessed_apis: Vec<AccessedApi>,
}

#[derive(Debug, Deserialize)]
pub struct AccessedApi {
    pub category: String,
    #[serde(default)]
    pub reasons: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Release {
    pub app_name: String,
    pub bundle_id: String,
    pub version: String,
    pub build: String,
    pub submitted_by: String,
    pub intended_submission: NaiveDate,
    pub locales: BTreeMap<String, LocalizedMetadata>,
    pub screenshots: BTreeMap<String, BTreeMap<String, Vec<String>>>,
    pub privacy: ReleasePrivacy,
    #[serde(default)]
    pub queue: QueueConfig,
}

#[derive(Debug, Deserialize)]
pub struct LocalizedMetadata {
    pub name: String,
    pub subtitle: String,
    pub description: String,
    pub keywords: String,
    pub release_notes: String,
}

#[derive(Debug, Deserialize)]
pub struct ReleasePrivacy {
    #[serde(default)]
    pub tracking: bool,
    #[serde(default)]
    pub collected_data: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct QueueConfig {
    #[serde(default = "default_review_days")]
    pub typical_review_days: i64,
    #[serde(default = "default_buffer_days")]
    pub buffer_days: i64,
    #[serde(default)]
    pub active_submissions: Vec<QueuedSubmission>,
}

#[derive(Debug, Deserialize)]
pub struct TeamPolicy {
    pub name: String,
    #[serde(default = "default_history_limit")]
    pub max_active_submissions: usize,
    #[serde(default)]
    pub additional_reason_codes: BTreeMap<String, Vec<String>>,
}

fn default_history_limit() -> usize {
    3
}
fn default_review_days() -> i64 {
    2
}
fn default_buffer_days() -> i64 {
    2
}

#[derive(Debug, Deserialize)]
pub struct QueuedSubmission {
    pub version: String,
    pub build: String,
    pub status: String,
    pub submitted_on: NaiveDate,
}

const ACTIVE_QUEUE_STATUSES: &[&str] = &[
    "waiting_for_review",
    "in_review",
    "pending_developer_release",
];
const QUEUE_STATUSES: &[&str] = &[
    "waiting_for_review",
    "in_review",
    "pending_developer_release",
    "completed",
];

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Serialize, Clone)]
pub struct Finding {
    pub code: String,
    pub severity: Severity,
    pub message: String,
    pub fix: String,
}

#[derive(Debug, Serialize)]
pub struct Summary {
    pub errors: usize,
    pub warnings: usize,
    pub checks: usize,
}

#[derive(Debug, Serialize)]
pub struct QueuePlan {
    pub intended_submission: NaiveDate,
    pub estimated_decision: NaiveDate,
    pub buffered_decision: NaiveDate,
    pub active_submissions: usize,
}

#[derive(Debug, Serialize)]
pub struct GateReport {
    pub passed: bool,
    pub ruleset: &'static str,
    pub policy: Option<String>,
    pub summary: Summary,
    pub findings: Vec<Finding>,
    pub queue: QueuePlan,
    pub packet_path: Option<PathBuf>,
}

fn finding(
    code: &str,
    severity: Severity,
    message: impl Into<String>,
    fix: impl Into<String>,
) -> Finding {
    Finding {
        code: code.into(),
        severity,
        message: message.into(),
        fix: fix.into(),
    }
}

fn required_value(findings: &mut Vec<Finding>, code: &str, label: &str, value: &str, fix: &str) {
    if value.trim().is_empty() {
        findings.push(finding(
            code,
            Severity::Error,
            format!("{label} is required and cannot be blank."),
            fix,
        ));
    }
}

fn checked_queue_date(
    start: NaiveDate,
    days: i64,
    multiplier: u64,
    label: &str,
    code: &str,
    fix: &str,
    findings: &mut Vec<Finding>,
) -> Option<NaiveDate> {
    let Ok(days) = u64::try_from(days) else {
        return None;
    };
    let Some(total_days) = days.checked_mul(multiplier) else {
        findings.push(finding(
            code,
            Severity::Error,
            format!("{label} is too large to produce a calendar date."),
            fix,
        ));
        return None;
    };
    let Some(date) = start.checked_add_days(Days::new(total_days)) else {
        findings.push(finding(
            code,
            Severity::Error,
            format!("{label} is too large to produce a calendar date."),
            fix,
        ));
        return None;
    };
    Some(date)
}

fn is_supported_image(path: &Path) -> bool {
    let Ok(bytes) = fs::read(path) else {
        return false;
    };
    let png = bytes.starts_with(b"\x89PNG\r\n\x1a\n");
    let jpeg = bytes.len() >= 4
        && bytes[0] == 0xff
        && bytes[1] == 0xd8
        && bytes[2] == 0xff
        && bytes.ends_with(&[0xff, 0xd9]);
    png || jpeg
}

pub fn check(metadata: &ArtifactMetadata, release: &Release, release_dir: &Path) -> GateReport {
    check_with_policy(metadata, release, release_dir, None)
}

pub fn check_with_policy(
    metadata: &ArtifactMetadata,
    release: &Release,
    release_dir: &Path,
    policy: Option<&TeamPolicy>,
) -> GateReport {
    let mut findings = Vec::new();
    let rules = official_rules();
    required_value(
        &mut findings,
        "identity.app_name_missing",
        "App name",
        &release.app_name,
        "Set app_name in release.yaml.",
    );
    required_value(
        &mut findings,
        "identity.owner_missing",
        "Release owner",
        &release.submitted_by,
        "Set submitted_by in release.yaml.",
    );
    for (code, label, value) in [
        (
            "identity.bundle_id_missing",
            "Artifact bundle ID",
            metadata.bundle_id.as_str(),
        ),
        (
            "identity.version_missing",
            "Artifact version",
            metadata.version.as_str(),
        ),
        (
            "identity.build_missing",
            "Artifact build",
            metadata.build.as_str(),
        ),
        (
            "identity.release_bundle_id_missing",
            "Release bundle ID",
            release.bundle_id.as_str(),
        ),
        (
            "identity.release_version_missing",
            "Release version",
            release.version.as_str(),
        ),
        (
            "identity.release_build_missing",
            "Release build",
            release.build.as_str(),
        ),
    ] {
        required_value(
            &mut findings,
            code,
            label,
            value,
            "Set this identity value in both the artifact export and release.yaml.",
        );
    }
    if metadata.bundle_id != release.bundle_id {
        findings.push(finding(
            "identity.bundle_id",
            Severity::Error,
            format!(
                "Bundle ID is {} in the artifact but {} in release.yaml.",
                metadata.bundle_id, release.bundle_id
            ),
            "Set both inputs to the same bundle ID.",
        ));
    }
    if metadata.version != release.version {
        findings.push(finding(
            "identity.version",
            Severity::Error,
            format!(
                "Version is {} in the artifact but {} in release.yaml.",
                metadata.version, release.version
            ),
            "Set release.yaml to the archived marketing version.",
        ));
    }
    if metadata.build != release.build {
        findings.push(finding(
            "identity.build",
            Severity::Error,
            format!(
                "Build is {} in the artifact but {} in release.yaml.",
                metadata.build, release.build
            ),
            "Set release.yaml to the archived build number.",
        ));
    }
    if !metadata.privacy_manifest {
        findings.push(finding(
            "privacy.manifest_missing",
            Severity::Error,
            "The artifact export says PrivacyInfo.xcprivacy is missing.",
            "Add a privacy manifest to the app target and export metadata again.",
        ));
    }
    if metadata.privacy_tracking != release.privacy.tracking {
        findings.push(finding(
            "privacy.tracking_mismatch",
            Severity::Error,
            "Tracking differs between the artifact and release.yaml.",
            "Make the release privacy answer match the built app.",
        ));
    }
    let mut built_data = metadata.privacy_collected_data.clone();
    built_data.sort();
    built_data.dedup();
    let mut declared_data = release.privacy.collected_data.clone();
    declared_data.sort();
    declared_data.dedup();
    if built_data != declared_data {
        findings.push(finding(
            "privacy.data_mismatch",
            Severity::Error,
            format!(
                "Collected data differs. Artifact: {:?}; release: {:?}.",
                built_data, declared_data
            ),
            "Declare the same collected-data categories in both inputs.",
        ));
    }

    let mut approved = rules.required_reason_apis;
    if let Some(policy) = policy {
        for (category, reasons) in &policy.additional_reason_codes {
            approved
                .entry(category.clone())
                .or_default()
                .extend(reasons.iter().cloned());
        }
    }
    for api in &metadata.accessed_apis {
        match approved.get(api.category.as_str()) {
            None => findings.push(finding(
                "privacy.api_unknown",
                Severity::Warning,
                format!("{} is not covered by ruleset {}.", api.category, RULESET),
                "Review Apple's current required-reason API list.",
            )),
            Some(valid) if !api.reasons.iter().any(|reason| valid.contains(reason)) => findings
                .push(finding(
                    "privacy.reason_missing",
                    Severity::Error,
                    format!(
                        "{} has no approved reason code in the export.",
                        api.category
                    ),
                    format!("Add an approved reason: {}.", valid.join(", ")),
                )),
            _ => {}
        }
    }

    if release.locales.is_empty() {
        findings.push(finding(
            "locales.empty",
            Severity::Error,
            "No localized metadata is listed.",
            "Add at least one locale under locales.",
        ));
    }
    for (locale, fields) in &release.locales {
        for (name, value) in [
            ("name", &fields.name),
            ("subtitle", &fields.subtitle),
            ("description", &fields.description),
            ("keywords", &fields.keywords),
            ("release_notes", &fields.release_notes),
        ] {
            if value.trim().is_empty() {
                findings.push(finding(
                    "locales.field_empty",
                    Severity::Error,
                    format!("{} has no {}.", locale, name),
                    format!("Add {} for {}.", name, locale),
                ));
            } else if let Some(limit) = rules.localized_field_limits.get(name) {
                if value.chars().count() > *limit {
                    findings.push(finding(
                        "locales.field_too_long",
                        Severity::Error,
                        format!(
                            "{} {} has {} characters; the {} limit is {}.",
                            locale,
                            name,
                            value.chars().count(),
                            RULESET,
                            limit
                        ),
                        format!("Shorten {} to {} characters or fewer.", name, limit),
                    ));
                }
            }
        }
        match release.screenshots.get(locale) {
            None => findings.push(finding(
                "screenshots.locale_missing",
                Severity::Error,
                format!("{} has no screenshot set.", locale),
                format!("Add screenshots for {}.", locale),
            )),
            Some(devices) if devices.is_empty() => findings.push(finding(
                "screenshots.devices_empty",
                Severity::Error,
                format!("{} has no screenshot device set.", locale),
                "Add at least one device set.",
            )),
            Some(devices) => {
                for (device, paths) in devices {
                    if paths.len() < rules.screenshots_per_set.minimum {
                        findings.push(finding(
                            "screenshots.set_empty",
                            Severity::Error,
                            format!("{} / {} has no screenshots.", locale, device),
                            "Add at least one screenshot path.",
                        ));
                    } else if paths.len() > rules.screenshots_per_set.maximum {
                        findings.push(finding(
                            "screenshots.set_too_large",
                            Severity::Error,
                            format!(
                                "{} / {} has {} screenshots; the {} limit is {}.",
                                locale,
                                device,
                                paths.len(),
                                RULESET,
                                rules.screenshots_per_set.maximum
                            ),
                            format!(
                                "Keep at most {} screenshots in this set.",
                                rules.screenshots_per_set.maximum
                            ),
                        ));
                    }
                    for raw in paths {
                        let path = release_dir.join(raw);
                        let ext = path
                            .extension()
                            .and_then(|v| v.to_str())
                            .unwrap_or("")
                            .to_ascii_lowercase();
                        if !matches!(ext.as_str(), "png" | "jpg" | "jpeg") {
                            findings.push(finding(
                                "screenshots.format",
                                Severity::Error,
                                format!("{} is not PNG or JPEG.", raw),
                                "Export the screenshot as PNG or JPEG.",
                            ));
                        } else if !path.is_file() {
                            findings.push(finding(
                                "screenshots.file_missing",
                                Severity::Error,
                                format!("{} does not exist.", raw),
                                "Fix the path or add the screenshot file.",
                            ));
                        } else if !is_supported_image(&path) {
                            findings.push(finding(
                                "screenshots.invalid_image",
                                Severity::Error,
                                format!("{} is not a readable PNG or JPEG image.", raw),
                                "Export a complete PNG or JPEG screenshot and replace this file.",
                            ));
                        }
                    }
                }
            }
        }
    }
    for locale in release.screenshots.keys() {
        if !release.locales.contains_key(locale) {
            findings.push(finding(
                "screenshots.locale_orphan",
                Severity::Warning,
                format!("{} has screenshots but no localized metadata.", locale),
                "Add localized metadata or remove the screenshot set.",
            ));
        }
    }
    let history_limit = policy.map_or(3, |item| item.max_active_submissions.max(3));
    if release.queue.active_submissions.len() > history_limit {
        findings.push(finding(
            "queue.history_limit",
            Severity::Warning,
            format!(
                "The queue has more than the configured {} active submissions.",
                history_limit
            ),
            "Raise max_active_submissions in a Team policy or shorten the queue history.",
        ));
    }
    for (index, item) in release.queue.active_submissions.iter().enumerate() {
        let entry = index + 1;
        required_value(
            &mut findings,
            "queue.version_missing",
            &format!("Queue entry {entry} version"),
            &item.version,
            "Set version for every queued submission.",
        );
        required_value(
            &mut findings,
            "queue.build_missing",
            &format!("Queue entry {entry} build"),
            &item.build,
            "Set build for every queued submission.",
        );
        if !QUEUE_STATUSES.contains(&item.status.as_str()) {
            findings.push(finding(
                "queue.status_invalid",
                Severity::Error,
                format!(
                    "Queue entry {entry} has unsupported status {:?}. Allowed statuses: {}.",
                    item.status,
                    QUEUE_STATUSES.join(", ")
                ),
                "Set status to the current App Store review state.",
            ));
        }
    }
    let active_count = release
        .queue
        .active_submissions
        .iter()
        .filter(|item| {
            ACTIVE_QUEUE_STATUSES.contains(&item.status.as_str())
                // An unknown status can be a misspelling of an active state. Count it
                // conservatively while the resulting HOLD is repaired.
                || !QUEUE_STATUSES.contains(&item.status.as_str())
        })
        .count();
    if active_count > 0 {
        findings.push(finding(
            "queue.active",
            Severity::Warning,
            format!(
                "{} active submission may affect this release window.",
                active_count
            ),
            "Confirm whether to finish or remove the active submission before queuing this build.",
        ));
    }
    if release.queue.typical_review_days < 0 {
        findings.push(finding(
            "queue.review_days_negative",
            Severity::Error,
            "Typical review days cannot be negative.",
            "Set typical_review_days to zero or a positive number.",
        ));
    }
    if release.queue.buffer_days < 0 {
        findings.push(finding(
            "queue.buffer_days_negative",
            Severity::Error,
            "Buffer days cannot be negative.",
            "Set buffer_days to zero or a positive number.",
        ));
    }
    let active_slots = u64::try_from(active_count)
        .unwrap_or(u64::MAX)
        .saturating_add(1);
    let estimated_decision = checked_queue_date(
        release.intended_submission,
        release.queue.typical_review_days,
        active_slots,
        "Typical review days",
        "queue.review_days_out_of_range",
        "Set typical_review_days so the estimated decision stays within the calendar range.",
        &mut findings,
    )
    .unwrap_or(release.intended_submission);
    let buffered_decision = checked_queue_date(
        estimated_decision,
        release.queue.buffer_days,
        1,
        "Buffer days",
        "queue.buffer_days_out_of_range",
        "Set buffer_days so the buffered decision stays within the calendar range.",
        &mut findings,
    )
    .unwrap_or(estimated_decision);
    let queue = QueuePlan {
        intended_submission: release.intended_submission,
        estimated_decision,
        buffered_decision,
        active_submissions: active_count,
    };
    let errors = findings
        .iter()
        .filter(|f| f.severity == Severity::Error)
        .count();
    let warnings = findings.len() - errors;
    GateReport {
        passed: errors == 0,
        ruleset: RULESET,
        policy: policy.map(|item| item.name.clone()),
        summary: Summary {
            errors,
            warnings,
            checks: 8,
        },
        findings,
        queue,
        packet_path: None,
    }
}

pub fn run_files(
    metadata_path: &Path,
    release_path: &Path,
    output: Option<&Path>,
) -> Result<GateReport, String> {
    run_files_with_policy(metadata_path, release_path, output, None)
}

pub fn run_files_with_policy(
    metadata_path: &Path,
    release_path: &Path,
    output: Option<&Path>,
    policy_path: Option<&Path>,
) -> Result<GateReport, String> {
    let metadata: ArtifactMetadata = serde_json::from_str(
        &fs::read_to_string(metadata_path)
            .map_err(|e| format!("Could not read {}: {}", metadata_path.display(), e))?,
    )
    .map_err(|e| format!("Could not parse {}: {}", metadata_path.display(), e))?;
    let release: Release = serde_yaml::from_str(
        &fs::read_to_string(release_path)
            .map_err(|e| format!("Could not read {}: {}", release_path.display(), e))?,
    )
    .map_err(|e| format!("Could not parse {}: {}", release_path.display(), e))?;
    let release_dir = release_path.parent().unwrap_or_else(|| Path::new("."));
    let policy: Option<TeamPolicy> = policy_path
        .map(|path| {
            serde_yaml::from_str(
                &fs::read_to_string(path)
                    .map_err(|e| format!("Could not read {}: {}", path.display(), e))?,
            )
            .map_err(|e| format!("Could not parse {}: {}", path.display(), e))
        })
        .transpose()?;
    let mut report = check_with_policy(&metadata, &release, release_dir, policy.as_ref());
    if let Some(path) = output {
        fs::write(path, render_packet(&metadata, &release, &report))
            .map_err(|e| format!("Could not write {}: {}", path.display(), e))?;
        report.packet_path = Some(path.to_path_buf());
    }
    Ok(report)
}

pub fn render_packet(
    metadata: &ArtifactMetadata,
    release: &Release,
    report: &GateReport,
) -> String {
    let generated = Utc::now().date_naive();
    let mark = if report.passed { "PASS" } else { "HOLD" };
    let policy_line = report
        .policy
        .as_ref()
        .map(|name| format!("  \nTeam policy: `{name}`"))
        .unwrap_or_default();
    let mut out = format!(
        "# App Review packet — {} {} ({})\n\n**Decision: {}**  \nGenerated: {}  \nRules: `{}`{}  \nOwner: {}\n\n## Artifact\n\n- Bundle ID: `{}`\n- Version: `{}`\n- Build: `{}`\n- Privacy manifest: {}\n\n## Gate findings\n\n",
        release.app_name,
        release.version,
        release.build,
        mark,
        generated,
        RULESET,
        policy_line,
        release.submitted_by,
        metadata.bundle_id,
        metadata.version,
        metadata.build,
        if metadata.privacy_manifest {
            "present"
        } else {
            "missing"
        }
    );
    if report.findings.is_empty() {
        out.push_str("No findings. The checked inputs agree.\n");
    }
    for f in &report.findings {
        out.push_str(&format!(
            "- **{:?} · `{}`** — {} Fix: {}\n",
            f.severity, f.code, f.message, f.fix
        ));
    }
    out.push_str(&format!("\n## Queue plan\n\n- Intended submission: {}\n- Estimated decision: {}\n- Buffered decision: {}\n- Active submissions: {}\n\n## Localizations\n\n", report.queue.intended_submission, report.queue.estimated_decision, report.queue.buffered_decision, report.queue.active_submissions));
    for locale in release.locales.keys() {
        out.push_str(&format!(
            "- `{}`: metadata and screenshot paths checked\n",
            locale
        ));
    }
    out.push_str("\n## Decision record\n\n- [ ] Reviewer confirmed the artifact identity.\n- [ ] Reviewer confirmed privacy answers.\n- [ ] Reviewer confirmed screenshots and localized copy.\n- [ ] Release owner accepted the queue window.\n\nThis packet is a local preflight record. It is not an Apple approval.\n");
    out
}
