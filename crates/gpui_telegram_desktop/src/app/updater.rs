//! Native Fabushi Desktop auto-updater & one-click self-installer.
//!
//! Checks `bhrum/gpui-telegram` GitHub Releases (primary `latest-mac.json` manifest
//! with automatic fallback to GitHub Releases API `/releases/latest`), downloads and
//! SHA-256 verifies `.zip`/`.tar.gz` update archives, unpacks the staged `.app` bundle,
//! and atomically replaces the installed application bundle before relaunching.

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

use futures::StreamExt;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const DEFAULT_GITHUB_REPO: &str = "bhrum/gpui-telegram";
pub const DEFAULT_MANIFEST_URL: &str =
    "https://github.com/bhrum/gpui-telegram/releases/latest/download/latest-mac.json";
pub const DEFAULT_GITHUB_LATEST_API_URL: &str =
    "https://api.github.com/repos/bhrum/gpui-telegram/releases/latest";
pub const CURRENT_APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Metadata describing an installable update release discovered by the updater.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AvailableUpdateInfo {
    pub version: String,
    pub asset_url: String,
    pub sha256: Option<String>,
    pub size_bytes: Option<u64>,
    pub release_notes: Option<String>,
    pub dmg_url: Option<String>,
    pub pkg_url: Option<String>,
}

/// Per-platform downloadable artifact entry inside `latest-mac.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformAssetEntry {
    pub zip_url: String,
    #[serde(default)]
    pub zip_sha256: Option<String>,
    #[serde(default)]
    pub zip_size: Option<u64>,
    #[serde(default)]
    pub dmg_url: Option<String>,
    #[serde(default)]
    pub pkg_url: Option<String>,
}

/// Primary release manifest JSON (`latest-mac.json`) published to GitHub Releases.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateReleaseManifest {
    pub version: String,
    #[serde(default)]
    pub tag_name: Option<String>,
    #[serde(default)]
    pub published_at: Option<String>,
    #[serde(default)]
    pub release_notes: Option<String>,
    pub platforms: HashMap<String, PlatformAssetEntry>,
}

/// Observable state machine for in-app update discovery, download, staging, and restart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DesktopUpdateState {
    Idle {
        last_checked_ok: bool,
        last_error: Option<String>,
    },
    Checking,
    UpToDate {
        checked_version: String,
    },
    Available {
        info: AvailableUpdateInfo,
    },
    Downloading {
        info: AvailableUpdateInfo,
        downloaded_bytes: u64,
        total_bytes: Option<u64>,
        progress_pct: u8,
    },
    Installing {
        version: String,
    },
    ReadyToRestart {
        version: String,
        staged_app_path: PathBuf,
    },
    Failed {
        error: String,
        retry_info: Option<AvailableUpdateInfo>,
    },
}

impl Default for DesktopUpdateState {
    fn default() -> Self {
        Self::Idle {
            last_checked_ok: false,
            last_error: None,
        }
    }
}

/// Snapshot projected into settings views and menu drawers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopUpdateSnapshot {
    pub current_version: String,
    pub latest_version: Option<String>,
    pub status_text: String,
    pub progress_pct: Option<u8>,
    pub is_busy: bool,
}

impl DesktopUpdateState {
    /// Returns update metadata when a downloadable or retryable release is known.
    pub fn actionable_info(&self) -> Option<&AvailableUpdateInfo> {
        match self {
            Self::Available { info } | Self::Downloading { info, .. } => Some(info),
            Self::Failed { retry_info, .. } => retry_info.as_ref(),
            _ => None,
        }
    }

    /// Formats a localized status projection for Settings and menu indicators.
    pub fn to_snapshot(&self, is_chinese: bool) -> DesktopUpdateSnapshot {
        let current = CURRENT_APP_VERSION.to_string();
        match self {
            Self::Idle {
                last_checked_ok,
                last_error,
            } => {
                let status_text = if let Some(err) = last_error {
                    if is_chinese {
                        format!("上次检查失败: {err}")
                    } else {
                        format!("Last check failed: {err}")
                    }
                } else if *last_checked_ok {
                    if is_chinese {
                        format!("已是最新版本 (v{current})")
                    } else {
                        format!("Up to date (v{current})")
                    }
                } else if is_chinese {
                    format!("当前版本 Fabushi v{current}")
                } else {
                    format!("Current version Fabushi v{current}")
                };
                DesktopUpdateSnapshot {
                    current_version: current,
                    latest_version: None,
                    status_text,
                    progress_pct: None,
                    is_busy: false,
                }
            }
            Self::Checking => DesktopUpdateSnapshot {
                current_version: current,
                latest_version: None,
                status_text: if is_chinese {
                    "正在检查更新…".to_string()
                } else {
                    "Checking for updates…".to_string()
                },
                progress_pct: None,
                is_busy: true,
            },
            Self::UpToDate { checked_version } => DesktopUpdateSnapshot {
                current_version: current,
                latest_version: None,
                status_text: if is_chinese {
                    format!("当前已是最新版本 (v{checked_version})")
                } else {
                    format!("You're up to date (v{checked_version})")
                },
                progress_pct: None,
                is_busy: false,
            },
            Self::Available { info } => DesktopUpdateSnapshot {
                current_version: current,
                latest_version: Some(info.version.clone()),
                status_text: if is_chinese {
                    format!("发现新版本 Fabushi v{}，可点击一键更新", info.version)
                } else {
                    format!("Fabushi v{} is available for one-click update", info.version)
                },
                progress_pct: None,
                is_busy: false,
            },
            Self::Downloading {
                info, progress_pct, ..
            } => DesktopUpdateSnapshot {
                current_version: current,
                latest_version: Some(info.version.clone()),
                status_text: if is_chinese {
                    format!("正在下载 Fabushi v{} ({progress_pct}%)…", info.version)
                } else {
                    format!("Downloading Fabushi v{} ({progress_pct}%)…", info.version)
                },
                progress_pct: Some(*progress_pct),
                is_busy: true,
            },
            Self::Installing { version } => DesktopUpdateSnapshot {
                current_version: current,
                latest_version: Some(version.clone()),
                status_text: if is_chinese {
                    format!("正在安装 Fabushi v{version} 并准备重启…")
                } else {
                    format!("Installing Fabushi v{version} & restarting…")
                },
                progress_pct: Some(100),
                is_busy: true,
            },
            Self::ReadyToRestart { version, .. } => DesktopUpdateSnapshot {
                current_version: current,
                latest_version: Some(version.clone()),
                status_text: if is_chinese {
                    format!("Fabushi v{version} 已就绪，正在重启…")
                } else {
                    format!("Fabushi v{version} is ready, restarting…")
                },
                progress_pct: Some(100),
                is_busy: true,
            },
            Self::Failed { error, retry_info } => DesktopUpdateSnapshot {
                current_version: current,
                latest_version: retry_info.as_ref().map(|i| i.version.clone()),
                status_text: if is_chinese {
                    format!("更新失败: {error}")
                } else {
                    format!("Update failed: {error}")
                },
                progress_pct: None,
                is_busy: false,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedSemver {
    major: u64,
    minor: u64,
    patch: u64,
    prerelease: Option<Vec<PreSegment>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PreSegment {
    Num(u64),
    Text(String),
}

/// Normalizes a version string by trimming whitespace and leading tag prefixes (`macos-main-v`, `v`, `V`).
pub fn normalize_version_string(raw: &str) -> String {
    let mut s = raw.trim();
    for prefix in ["macos-main-v", "macos-main-", "v", "V"] {
        if let Some(rest) = s.strip_prefix(prefix) {
            s = rest;
            break;
        }
    }
    if let Some((before_build, _)) = s.split_once('+') {
        s = before_build;
    }
    s.trim().to_string()
}

fn parse_semver(raw: &str) -> Option<ParsedSemver> {
    let norm = normalize_version_string(raw);
    if norm.is_empty() {
        return None;
    }
    let (core, pre_raw) = match norm.split_once('-') {
        Some((c, p)) => (c, Some(p)),
        None => (norm.as_str(), None),
    };
    let mut parts = core.split('.');
    let major = parts.next()?.parse::<u64>().ok()?;
    let minor = parts.next().unwrap_or("0").parse::<u64>().ok()?;
    let patch_raw = parts.next().unwrap_or("0");
    let patch = patch_raw.parse::<u64>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    let prerelease = pre_raw.and_then(|p| {
        let trimmed = p.trim();
        if trimmed.is_empty() {
            return None;
        }
        let segs = trimmed
            .split('.')
            .map(|seg| match seg.parse::<u64>() {
                Ok(n) => PreSegment::Num(n),
                Err(_) => PreSegment::Text(seg.to_ascii_lowercase()),
            })
            .collect();
        Some(segs)
    });
    Some(ParsedSemver {
        major,
        minor,
        patch,
        prerelease,
    })
}

fn compare_prerelease(a: &[PreSegment], b: &[PreSegment]) -> Ordering {
    for (seg_a, seg_b) in a.iter().zip(b.iter()) {
        let ord = match (seg_a, seg_b) {
            (PreSegment::Num(x), PreSegment::Num(y)) => x.cmp(y),
            (PreSegment::Num(_), PreSegment::Text(_)) => Ordering::Less,
            (PreSegment::Text(_), PreSegment::Num(_)) => Ordering::Greater,
            (PreSegment::Text(x), PreSegment::Text(y)) => x.cmp(y),
        };
        if ord != Ordering::Equal {
            return ord;
        }
    }
    a.len().cmp(&b.len())
}

/// Compares two semantic version strings (stable > prerelease for identical major.minor.patch).
pub fn compare_semver(left: &str, right: &str) -> Ordering {
    match (parse_semver(left), parse_semver(right)) {
        (Some(a), Some(b)) => {
            let core_ord = (a.major, a.minor, a.patch).cmp(&(b.major, b.minor, b.patch));
            if core_ord != Ordering::Equal {
                return core_ord;
            }
            match (&a.prerelease, &b.prerelease) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Greater,
                (Some(_), None) => Ordering::Less,
                (Some(pa), Some(pb)) => compare_prerelease(pa, pb),
            }
        }
        _ => normalize_version_string(left).cmp(&normalize_version_string(right)),
    }
}

/// Returns `true` when `candidate` is strictly newer than `current`.
pub fn is_newer_version(current: &str, candidate: &str) -> bool {
    let clean_cand = normalize_version_string(candidate);
    if clean_cand.is_empty() {
        return false;
    }
    compare_semver(candidate, current) == Ordering::Greater
}

fn current_platform_keys() -> Vec<&'static str> {
    if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        vec!["darwin-arm64", "macos-arm64", "darwin-universal", "macos-universal"]
    } else if cfg!(all(target_os = "macos", target_arch = "x86_64")) {
        vec!["darwin-x86_64", "darwin-x64", "macos-x86_64", "macos-x64", "darwin-universal", "macos-universal"]
    } else if cfg!(target_os = "linux") {
        vec!["linux-x86_64", "linux-arm64"]
    } else {
        vec!["darwin-arm64", "darwin-universal"]
    }
}

/// Parses a `latest-mac.json` manifest payload into [`AvailableUpdateInfo`] if an asset matches this host.
pub fn parse_release_manifest(json_str: &str) -> Result<AvailableUpdateInfo, String> {
    let manifest: UpdateReleaseManifest =
        serde_json::from_str(json_str).map_err(|e| format!("Invalid manifest JSON: {e}"))?;
    let version = normalize_version_string(&manifest.version);
    if version.is_empty() {
        return Err("Manifest version is empty".to_string());
    }

    let mut chosen: Option<&PlatformAssetEntry> = None;
    for key in current_platform_keys() {
        if let Some(entry) = manifest.platforms.get(key) {
            chosen = Some(entry);
            break;
        }
    }
    if chosen.is_none() {
        chosen = manifest.platforms.values().next();
    }
    let entry = chosen.ok_or_else(|| "Manifest has no platform assets".to_string())?;
    if entry.zip_url.trim().is_empty() {
        return Err("Manifest asset zip_url is empty".to_string());
    }

    Ok(AvailableUpdateInfo {
        version,
        asset_url: entry.zip_url.trim().to_string(),
        sha256: entry
            .zip_sha256
            .as_ref()
            .map(|s| s.trim().to_ascii_lowercase())
            .filter(|s| !s.is_empty()),
        size_bytes: entry.zip_size,
        release_notes: manifest.release_notes,
        dmg_url: entry.dmg_url.clone(),
        pkg_url: entry.pkg_url.clone(),
    })
}

#[derive(Debug, Deserialize)]
struct GitHubReleaseDto {
    #[serde(default)]
    tag_name: String,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    body: Option<String>,
    #[serde(default)]
    assets: Vec<GitHubAssetDto>,
}

#[derive(Debug, Deserialize)]
struct GitHubAssetDto {
    name: String,
    browser_download_url: String,
    #[serde(default)]
    size: Option<u64>,
    #[serde(default)]
    digest: Option<String>,
}

fn score_github_asset(name: &str) -> Option<i32> {
    let lower = name.to_ascii_lowercase();
    if lower.ends_with(".blockmap") || lower.ends_with(".sig") || lower.ends_with(".sha256") {
        return None;
    }
    let is_zip = lower.ends_with(".zip");
    let is_tar = lower.ends_with(".tar.gz") || lower.ends_with(".tgz");
    if !is_zip && !is_tar {
        return None;
    }
    if lower.contains("linux") || lower.contains(".exe") || lower.contains("win32") {
        return None;
    }
    let arch_score = if cfg!(target_arch = "aarch64") {
        if lower.contains("arm64") || lower.contains("aarch64") {
            40
        } else if lower.contains("universal") {
            30
        } else if lower.contains("x86_64") || lower.contains("x64") {
            return None;
        } else {
            10
        }
    } else if cfg!(target_arch = "x86_64") {
        if lower.contains("x86_64") || lower.contains("x64") {
            40
        } else if lower.contains("universal") {
            30
        } else if lower.contains("arm64") || lower.contains("aarch64") {
            return None;
        } else {
            10
        }
    } else {
        10
    };
    let brand_score = if lower.contains("fabushi") { 20 } else { 5 };
    let fmt_score = if is_zip { 15 } else { 8 };
    Some(arch_score + brand_score + fmt_score)
}

fn extract_sha256_digest(raw: Option<&str>) -> Option<String> {
    let s = raw?.trim();
    let hex = s.strip_prefix("sha256:").unwrap_or(s).trim();
    if hex.len() == 64 && hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        Some(hex.to_ascii_lowercase())
    } else {
        None
    }
}

/// Parses a GitHub Releases API `/releases/latest` JSON payload into [`AvailableUpdateInfo`].
pub fn parse_github_release_json(json_str: &str) -> Result<AvailableUpdateInfo, String> {
    let dto: GitHubReleaseDto =
        serde_json::from_str(json_str).map_err(|e| format!("Invalid GitHub release JSON: {e}"))?;
    let raw_ver = if !dto.tag_name.trim().is_empty() {
        dto.tag_name.as_str()
    } else {
        dto.name.as_deref().unwrap_or("")
    };
    let version = normalize_version_string(raw_ver);
    if version.is_empty() {
        return Err("GitHub release tag_name is empty".to_string());
    }

    let best = dto
        .assets
        .iter()
        .filter_map(|a| score_github_asset(&a.name).map(|score| (score, a)))
        .max_by_key(|(score, _)| *score)
        .map(|(_, a)| a)
        .ok_or_else(|| "No matching macOS archive asset found in release".to_string())?;

    let dmg_url = dto
        .assets
        .iter()
        .find(|a| a.name.to_ascii_lowercase().ends_with(".dmg"))
        .map(|a| a.browser_download_url.clone());
    let pkg_url = dto
        .assets
        .iter()
        .find(|a| a.name.to_ascii_lowercase().ends_with(".pkg"))
        .map(|a| a.browser_download_url.clone());

    Ok(AvailableUpdateInfo {
        version,
        asset_url: best.browser_download_url.clone(),
        sha256: extract_sha256_digest(best.digest.as_deref()),
        size_bytes: best.size,
        release_notes: dto.body.filter(|b| !b.trim().is_empty()),
        dmg_url,
        pkg_url,
    })
}

fn file_url_to_path(url: &str) -> Option<PathBuf> {
    let rest = url.strip_prefix("file://")?;
    let clean = rest.strip_prefix("localhost").unwrap_or(rest);
    let decoded = percent_decode_path(clean);
    Some(PathBuf::from(decoded))
}

fn percent_decode_path(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let (Some(hi), Some(lo)) = (hex_val(bytes[i + 1]), hex_val(bytes[i + 2])) {
                out.push((hi << 4) | lo);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn build_http_client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .user_agent(format!("Fabushi-Desktop/{CURRENT_APP_VERSION}"))
        .timeout(Duration::from_secs(60))
        .build()
        .map_err(|e| format!("Failed to construct HTTP client: {e}"))
}

fn parse_any_release_payload(body: &str) -> Result<AvailableUpdateInfo, String> {
    if let Ok(info) = parse_release_manifest(body) {
        return Ok(info);
    }
    parse_github_release_json(body)
}

fn shell_single_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

fn find_app_bundle_in_dir(dir: &Path) -> Option<PathBuf> {
    let entries = fs::read_dir(dir).ok()?;
    let mut dirs = Vec::new();
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            let is_app = p
                .extension()
                .and_then(|e| e.to_str())
                .is_some_and(|e| e.eq_ignore_ascii_case("app"));
            if is_app && p.join("Contents").join("Info.plist").is_file() {
                return Some(p);
            }
            dirs.push(p);
        }
    }
    for sub in dirs {
        if let Some(found) = find_app_bundle_in_dir(&sub) {
            return Some(found);
        }
    }
    None
}

fn verify_staged_app_bundle(app_dir: &Path) -> Result<(), String> {
    let plist = app_dir.join("Contents").join("Info.plist");
    if !plist.is_file() {
        return Err(format!(
            "Staged bundle missing Info.plist: {}",
            plist.display()
        ));
    }
    let macos_dir = app_dir.join("Contents").join("MacOS");
    let entries = fs::read_dir(&macos_dir)
        .map_err(|e| format!("Staged bundle missing Contents/MacOS: {e}"))?;
    let mut found_exec = false;
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_file() {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let _ = fs::set_permissions(&p, fs::Permissions::from_mode(0o755));
            }
            found_exec = true;
        }
    }
    if !found_exec {
        return Err(format!(
            "Staged bundle contains no executable in {}",
            macos_dir.display()
        ));
    }
    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("/usr/bin/xattr")
            .args(["-cr", &app_dir.to_string_lossy()])
            .status();
    }
    Ok(())
}

/// Core desktop auto-updater service handling remote discovery, SHA-256 verification, and self-replacement.
pub struct DesktopUpdater;

impl DesktopUpdater {
    /// Checks `feed_override` (if provided) or `DEFAULT_MANIFEST_URL` -> `DEFAULT_GITHUB_LATEST_API_URL`.
    pub async fn check_for_updates(
        feed_override: Option<&str>,
        current_version: &str,
    ) -> Result<Option<AvailableUpdateInfo>, String> {
        let trimmed_override = feed_override.map(str::trim).filter(|s| !s.is_empty());
        let info = if let Some(custom_url) = trimmed_override {
            if let Some(local_path) = file_url_to_path(custom_url) {
                let body = fs::read_to_string(&local_path)
                    .map_err(|e| format!("Failed to read local update feed {}: {e}", local_path.display()))?;
                parse_any_release_payload(&body)?
            } else {
                let client = build_http_client()?;
                let resp = client
                    .get(custom_url)
                    .send()
                    .await
                    .map_err(|e| format!("Update check request failed: {e}"))?;
                if !resp.status().is_success() {
                    return Err(format!("Update feed returned HTTP {}", resp.status()));
                }
                let body = resp
                    .text()
                    .await
                    .map_err(|e| format!("Failed reading update feed response: {e}"))?;
                parse_any_release_payload(&body)?
            }
        } else {
            let client = build_http_client()?;
            let manifest_attempt = async {
                let resp = client
                    .get(DEFAULT_MANIFEST_URL)
                    .send()
                    .await
                    .map_err(|e| format!("Manifest check failed: {e}"))?;
                if !resp.status().is_success() {
                    return Err(format!("Manifest returned HTTP {}", resp.status()));
                }
                let body = resp
                    .text()
                    .await
                    .map_err(|e| format!("Failed reading manifest body: {e}"))?;
                parse_release_manifest(&body)
            }
            .await;

            match manifest_attempt {
                Ok(info) => info,
                Err(_) => {
                    let resp = client
                        .get(DEFAULT_GITHUB_LATEST_API_URL)
                        .header("Accept", "application/vnd.github+json")
                        .send()
                        .await
                        .map_err(|e| format!("GitHub release check failed: {e}"))?;
                    if !resp.status().is_success() {
                        return Err(format!(
                            "GitHub releases API returned HTTP {}",
                            resp.status()
                        ));
                    }
                    let body = resp
                        .text()
                        .await
                        .map_err(|e| format!("Failed reading GitHub release response: {e}"))?;
                    parse_github_release_json(&body)?
                }
            }
        };

        if is_newer_version(current_version, &info.version) {
            Ok(Some(info))
        } else {
            Ok(None)
        }
    }

    /// Streams `info.asset_url`, validates SHA-256 if present, extracts the `.app` bundle, and returns its staged path.
    pub async fn download_and_stage_update(
        info: &AvailableUpdateInfo,
        staging_root: &Path,
        mut on_progress: impl FnMut(u64, Option<u64>, u8),
    ) -> Result<PathBuf, String> {
        fs::create_dir_all(staging_root)
            .map_err(|e| format!("Failed to create staging root {}: {e}", staging_root.display()))?;

        let stage_dir = staging_root.join(format!(
            "stage-{}-{}",
            normalize_version_string(&info.version),
            std::process::id()
        ));
        if stage_dir.exists() {
            let _ = fs::remove_dir_all(&stage_dir);
        }
        fs::create_dir_all(&stage_dir)
            .map_err(|e| format!("Failed to create staging directory: {e}"))?;

        let lower_url = info.asset_url.to_ascii_lowercase();
        let archive_ext = if lower_url.ends_with(".tar.gz") || lower_url.ends_with(".tgz") {
            "tar.gz"
        } else {
            "zip"
        };
        let archive_path = stage_dir.join(format!("update.{archive_ext}"));

        let mut hasher = Sha256::new();
        let mut downloaded: u64 = 0;

        if let Some(src_path) = file_url_to_path(&info.asset_url) {
            let bytes = fs::read(&src_path).map_err(|e| {
                format!(
                    "Failed to read local update archive {}: {e}",
                    src_path.display()
                )
            })?;
            let total = Some(bytes.len() as u64);
            hasher.update(&bytes);
            downloaded = bytes.len() as u64;
            fs::write(&archive_path, &bytes)
                .map_err(|e| format!("Failed to copy local update archive: {e}"))?;
            on_progress(downloaded, total, 100);
        } else {
            let client = build_http_client()?;
            let resp = client
                .get(&info.asset_url)
                .send()
                .await
                .map_err(|e| format!("Failed downloading update archive: {e}"))?;
            if !resp.status().is_success() {
                return Err(format!("Download returned HTTP {}", resp.status()));
            }
            let total = resp.content_length().or(info.size_bytes);
            let mut file = fs::File::create(&archive_path)
                .map_err(|e| format!("Failed creating update archive file: {e}"))?;
            let mut stream = resp.bytes_stream();
            let mut last_pct: u8 = 0;
            on_progress(0, total, 0);
            while let Some(chunk_res) = stream.next().await {
                let chunk = chunk_res
                    .map_err(|e| format!("Stream error while downloading update: {e}"))?;
                file.write_all(&chunk)
                    .map_err(|e| format!("Disk write error while saving update: {e}"))?;
                hasher.update(&chunk);
                downloaded = downloaded.saturating_add(chunk.len() as u64);
                let pct = match total {
                    Some(t) if t > 0 => ((downloaded.saturating_mul(100)) / t).min(100) as u8,
                    _ => 50,
                };
                if pct != last_pct {
                    last_pct = pct;
                    on_progress(downloaded, total, pct);
                }
            }
            file.sync_all()
                .map_err(|e| format!("Failed flushing downloaded archive: {e}"))?;
            on_progress(downloaded, total.or(Some(downloaded)), 100);
        }

        let actual_hex = format!("{:x}", hasher.finalize());
        if let Some(expected) = info.sha256.as_deref() {
            if !actual_hex.eq_ignore_ascii_case(expected.trim()) {
                let _ = fs::remove_file(&archive_path);
                return Err(format!(
                    "SHA-256 verification failed: expected {}, got {}",
                    expected, actual_hex
                ));
            }
        }

        let extract_dir = stage_dir.join("extracted");
        fs::create_dir_all(&extract_dir)
            .map_err(|e| format!("Failed creating extraction dir: {e}"))?;

        let status = if archive_ext == "zip" {
            if Path::new("/usr/bin/ditto").exists() {
                Command::new("/usr/bin/ditto")
                    .args([
                        "-x",
                        "-k",
                        &archive_path.to_string_lossy(),
                        &extract_dir.to_string_lossy(),
                    ])
                    .status()
            } else {
                Command::new("unzip")
                    .args([
                        "-q",
                        "-o",
                        &archive_path.to_string_lossy(),
                        "-d",
                        &extract_dir.to_string_lossy(),
                    ])
                    .status()
            }
        } else {
            Command::new("tar")
                .args([
                    "-xzf",
                    &archive_path.to_string_lossy(),
                    "-C",
                    &extract_dir.to_string_lossy(),
                ])
                .status()
        }
        .map_err(|e| format!("Failed launching archive extractor: {e}"))?;

        if !status.success() {
            return Err(format!("Archive extraction exited with status {status}"));
        }

        let staged_app = find_app_bundle_in_dir(&extract_dir)
            .ok_or_else(|| "Extracted update archive did not contain a valid .app bundle".to_string())?;
        verify_staged_app_bundle(&staged_app)?;
        Ok(staged_app)
    }

    /// Resolves which `.app` bundle path should be replaced on update given the running executable path.
    pub fn resolve_target_app_bundle(exe_path: &Path) -> PathBuf {
        for anc in exe_path.ancestors() {
            let is_app = anc
                .extension()
                .and_then(|e| e.to_str())
                .is_some_and(|e| e.eq_ignore_ascii_case("app"));
            if is_app && anc.join("Contents").join("MacOS").is_dir() {
                return anc.to_path_buf();
            }
        }
        let sys_apps = PathBuf::from("/Applications/Fabushi.app");
        let sys_writable = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("/Applications/.fabushi_write_probe");
        match sys_writable {
            Ok(_) => {
                let _ = fs::remove_file("/Applications/.fabushi_write_probe");
                sys_apps
            }
            Err(_) => {
                if let Ok(home) = std::env::var("HOME") {
                    let user_apps = PathBuf::from(home).join("Applications");
                    let _ = fs::create_dir_all(&user_apps);
                    user_apps.join("Fabushi.app")
                } else {
                    sys_apps
                }
            }
        }
    }

    /// Atomically replaces `target_app` with `staged_app` synchronously (with rollback if replacement fails).
    pub fn replace_app_bundle_in_place(staged_app: &Path, target_app: &Path) -> Result<(), String> {
        verify_staged_app_bundle(staged_app)?;
        if let Some(parent) = target_app.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed creating target parent directory: {e}"))?;
        }
        let backup_app = target_app.with_extension("app.bak");
        if backup_app.exists() {
            let _ = fs::remove_dir_all(&backup_app);
        }
        let had_existing = target_app.exists();
        if had_existing {
            fs::rename(target_app, &backup_app)
                .map_err(|e| format!("Failed moving existing bundle to backup: {e}"))?;
        }
        if let Err(e) = fs::rename(staged_app, target_app) {
            if had_existing && backup_app.exists() {
                let _ = fs::rename(&backup_app, target_app);
            }
            return Err(format!("Failed moving staged bundle into place: {e}"));
        }
        if backup_app.exists() {
            let _ = fs::remove_dir_all(&backup_app);
        }
        #[cfg(target_os = "macos")]
        {
            let _ = Command::new("/usr/bin/xattr")
                .args(["-cr", &target_app.to_string_lossy()])
                .status();
            let temp_root = fs::canonicalize(std::env::temp_dir()).unwrap_or_else(|_| std::env::temp_dir());
            let canon_target = fs::canonicalize(target_app).unwrap_or_else(|_| target_app.to_path_buf());
            let in_temp_dir = canon_target.starts_with(&temp_root) || target_app.starts_with(std::env::temp_dir());
            if !in_temp_dir && std::env::var_os("FABUSHI_SKIP_OS_REGISTER").is_none() {
                let lsreg = "/System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/LaunchServices.framework/Versions/A/Support/lsregister";
                if Path::new(lsreg).exists() {
                    let _ = Command::new(lsreg)
                        .args(["-f", &target_app.to_string_lossy()])
                        .status();
                }
                if Path::new("/usr/bin/mdimport").exists() {
                    let _ = Command::new("/usr/bin/mdimport")
                        .arg(&target_app.to_string_lossy().into_owned())
                        .status();
                }
            }
        }
        Ok(())
    }

    /// Spawns a detached POSIX helper that waits for `current_pid` to exit, atomically swaps
    /// `staged_app` into `target_app`, re-registers LaunchServices/Spotlight, and optionally relaunches.
    pub fn apply_staged_update_and_relaunch(
        staged_app: &Path,
        target_app: &Path,
        current_pid: u32,
        relaunch: bool,
    ) -> Result<PathBuf, String> {
        verify_staged_app_bundle(staged_app)?;
        let stage_parent = staged_app
            .parent()
            .and_then(|p| p.parent())
            .unwrap_or_else(|| Path::new("/tmp"));
        let script_path = stage_parent.join("apply_update.sh");

        let staged_q = shell_single_quote(&staged_app.to_string_lossy());
        let target_q = shell_single_quote(&target_app.to_string_lossy());
        let backup_q = shell_single_quote(&target_app.with_extension("app.bak").to_string_lossy());
        let stage_dir_q = shell_single_quote(&stage_parent.to_string_lossy());
        let target_parent_q = shell_single_quote(
            &target_app
                .parent()
                .unwrap_or_else(|| Path::new("/Applications"))
                .to_string_lossy(),
        );
        let relaunch_line = if relaunch {
            format!("/usr/bin/open -n {target_q} >/dev/null 2>&1 || true")
        } else {
            String::new()
        };

        let script = format!(
            r#"#!/bin/sh
set -u
OLD_PID={current_pid}
STAGED={staged_q}
TARGET={target_q}
BACKUP={backup_q}
TARGET_PARENT={target_parent_q}
STAGE_DIR={stage_dir_q}

i=0
while [ "$i" -lt 60 ]; do
  if ! kill -0 "$OLD_PID" 2>/dev/null; then
    break
  fi
  sleep 0.25
  i=$((i + 1))
done

mkdir -p "$TARGET_PARENT" || true
rm -rf "$BACKUP" || true

swap_bundles() {{
  if [ -e "$TARGET" ]; then
    mv "$TARGET" "$BACKUP" || return 1
  fi
  if ! mv "$STAGED" "$TARGET"; then
    if [ -e "$BACKUP" ]; then
      mv "$BACKUP" "$TARGET" || true
    fi
    return 1
  fi
  rm -rf "$BACKUP" || true
  return 0
}}

if ! swap_bundles; then
  if command -v osascript >/dev/null 2>&1; then
    CMD="mkdir -p '$TARGET_PARENT' && rm -rf '$BACKUP' && ([ ! -e '$TARGET' ] || mv '$TARGET' '$BACKUP') && mv '$STAGED' '$TARGET' && rm -rf '$BACKUP'"
    ESCAPED=$(printf '%s' "$CMD" | sed 's/\\/\\\\/g; s/"/\\"/g')
    if ! osascript -e "do shell script \"$ESCAPED\" with administrator privileges" >/dev/null 2>&1; then
      if [ -e "$BACKUP" ] && [ ! -e "$TARGET" ]; then
        mv "$BACKUP" "$TARGET" || true
      fi
      exit 1
    fi
  else
    exit 1
  fi
fi

/usr/bin/xattr -cr "$TARGET" >/dev/null 2>&1 || true
LSREG="/System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/LaunchServices.framework/Versions/A/Support/lsregister"
if [ -x "$LSREG" ]; then
  "$LSREG" -f "$TARGET" >/dev/null 2>&1 || true
fi
if command -v mdimport >/dev/null 2>&1; then
  mdimport "$TARGET" >/dev/null 2>&1 || true
fi
{relaunch_line}
rm -rf "$STAGE_DIR" >/dev/null 2>&1 || true
"#
        );

        fs::write(&script_path, script.as_bytes())
            .map_err(|e| format!("Failed writing updater helper script: {e}"))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&script_path, fs::Permissions::from_mode(0o700))
                .map_err(|e| format!("Failed marking updater script executable: {e}"))?;
        }

        Command::new("/bin/sh")
            .arg(&script_path)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("Failed spawning updater helper process: {e}"))?;

        Ok(script_path)
    }
}
