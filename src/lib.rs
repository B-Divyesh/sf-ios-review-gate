use chrono::{Days, NaiveDate, Utc};
use image::{GenericImageView, ImageFormat};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

pub const RULESET: &str = "apple-2026.1";

#[derive(Debug, Deserialize)]
struct RuleSet {
    id: String,
    localized_field_limits: BTreeMap<String, usize>,
    supported_locales: Vec<String>,
    screenshots_per_set: ScreenshotLimits,
    screenshot_device_sets: BTreeMap<String, ScreenshotDeviceSet>,
    required_reason_apis: BTreeMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct ScreenshotLimits {
    minimum: usize,
    maximum: usize,
}

#[derive(Debug, Deserialize)]
struct ScreenshotDeviceSet {
    display: String,
    sizes: Vec<ScreenshotSize>,
}

#[derive(Debug, Deserialize)]
struct ScreenshotSize {
    orientation: String,
    width: u32,
    height: u32,
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
    // The old key is accepted so existing policy files still load, but these
    // codes now narrow the immutable Apple rules instead of expanding them.
    #[serde(
        default,
        rename = "approved_reason_codes",
        alias = "additional_reason_codes"
    )]
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

fn screenshot_dimensions(path: &Path, extension: &str) -> Option<(u32, u32)> {
    let bytes = fs::read(path).ok()?;
    let format = match extension {
        "png" => ImageFormat::Png,
        "jpg" | "jpeg" => ImageFormat::Jpeg,
        _ => return None,
    };

    // Fully decode the declared image format. This validates the stream rather
    // than accepting a filename or a handful of magic bytes as a screenshot.
    image::load_from_memory_with_format(&bytes, format)
        .ok()
        .map(|image| image.dimensions())
}

fn accepted_screenshot_sizes(device: &ScreenshotDeviceSet) -> String {
    device
        .sizes
        .iter()
        .map(|size| format!("{} {}×{}", size.orientation, size.width, size.height))
        .collect::<Vec<_>>()
        .join(", ")
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

    let approved = &rules.required_reason_apis;
    for api in &metadata.accessed_apis {
        match approved.get(api.category.as_str()) {
            None => findings.push(finding(
                "privacy.api_unknown",
                Severity::Error,
                format!("{} is not covered by ruleset {}.", api.category, RULESET),
                "Use a required-reason API category from the bundled rules, or update the ruleset before release.",
            )),
            Some(valid) if api.reasons.is_empty() => findings.push(finding(
                    "privacy.reason_missing",
                    Severity::Error,
                    format!(
                        "{} has no approved reason code in the export.",
                        api.category
                    ),
                    format!("Add an approved reason: {}.", valid.join(", ")),
                )),
            Some(valid) => {
                for reason in &api.reasons {
                    if !valid.contains(reason) {
                        findings.push(finding(
                            "privacy.reason_invalid",
                            Severity::Error,
                            format!(
                                "{} declares reason {}, which is not allowed by {}.",
                                api.category, reason, RULESET
                            ),
                            format!(
                                "Remove {} or replace it with an approved reason: {}.",
                                reason,
                                valid.join(", ")
                            ),
                        ));
                    }
                }
            }
        }
    }
    if let Some(policy) = policy {
        for (category, team_approved) in &policy.additional_reason_codes {
            match approved.get(category) {
                None => findings.push(finding(
                    "policy.reason_api_unknown",
                    Severity::Error,
                    format!(
                        "Team policy {} lists unknown API category {}.",
                        policy.name, category
                    ),
                    "Remove the category or use one from the bundled Apple rules.",
                )),
                Some(apple_approved) => {
                    for reason in team_approved {
                        if !apple_approved.contains(reason) {
                            findings.push(finding(
                                "policy.reason_invalid",
                                Severity::Error,
                                format!(
                                    "Team policy {} lists {} for {}, but {} does not allow it.",
                                    policy.name, reason, category, RULESET
                                ),
                                format!(
                                    "Remove {} or choose from: {}.",
                                    reason,
                                    apple_approved.join(", ")
                                ),
                            ));
                        }
                    }
                    for api in metadata
                        .accessed_apis
                        .iter()
                        .filter(|api| api.category == *category)
                    {
                        for reason in api
                            .reasons
                            .iter()
                            .filter(|reason| apple_approved.contains(reason))
                        {
                            if !team_approved.contains(reason) {
                                findings.push(finding(
                                    "privacy.reason_not_team_approved",
                                    Severity::Error,
                                    format!(
                                        "{} is Apple-approved for {}, but Team policy {} does not approve it.",
                                        reason, category, policy.name
                                    ),
                                    format!(
                                        "Use a Team-approved reason for {} or add this Apple-approved reason to the policy.",
                                        category
                                    ),
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    let mut locale_identifiers = BTreeSet::new();
    locale_identifiers.extend(release.locales.keys());
    locale_identifiers.extend(release.screenshots.keys());
    for locale in locale_identifiers {
        if !rules.supported_locales.contains(locale) {
            findings.push(finding(
                "locales.identifier_unknown",
                Severity::Error,
                format!("{} is not an App Store locale in {}.", locale, RULESET),
                "Use a locale identifier from the bundled rules, such as en-US, fr-FR, ja, or zh-Hans.",
            ));
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
                    let device_rule = rules.screenshot_device_sets.get(device);
                    if device_rule.is_none() {
                        findings.push(finding(
                            "screenshots.device_unknown",
                            Severity::Error,
                            format!(
                                "{} / {} is not a device set in {}.",
                                locale, device, RULESET
                            ),
                            format!(
                                "Use one of these device sets: {}.",
                                rules
                                    .screenshot_device_sets
                                    .keys()
                                    .map(String::as_str)
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            ),
                        ));
                    }
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
                        } else if let Some((width, height)) = screenshot_dimensions(&path, &ext) {
                            if let Some(device_rule) = device_rule {
                                let accepted = device_rule
                                    .sizes
                                    .iter()
                                    .any(|size| size.width == width && size.height == height);
                                if !accepted {
                                    findings.push(finding(
                                        "screenshots.dimensions",
                                        Severity::Error,
                                        format!(
                                            "{} is {}×{}; {} accepts {}.",
                                            raw,
                                            width,
                                            height,
                                            device_rule.display,
                                            accepted_screenshot_sizes(device_rule)
                                        ),
                                        format!(
                                            "Export this screenshot at one accepted {} portrait or landscape size.",
                                            device_rule.display
                                        ),
                                    ));
                                }
                            }
                        } else {
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
        if item.submitted_on > release.intended_submission {
            findings.push(finding(
                "queue.submitted_after_intended",
                Severity::Error,
                format!(
                    "Queue entry {entry} was submitted on {}, after the intended submission on {}.",
                    item.submitted_on, release.intended_submission
                ),
                "Correct submitted_on or move intended_submission after this queued submission.",
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
