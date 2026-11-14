//! GPUI 0.2.2 AssetSource implementation and vector SVG icon builders.

use std::borrow::Cow;
use std::collections::BTreeSet;
use std::path::{Component, Path, PathBuf};

use gpui::{AssetSource, Result, Rgba, SharedString, Styled, px, rgb};

pub use super::assets_embedded::IconName;
use super::assets_embedded::lookup_embedded_asset;

/// Dual embedded + disk AssetSource so both `cargo run` and `GPUI Telegram.app` load icons,
/// language packs, theme files, and wallpapers cleanly without external path dependencies.
#[derive(Debug, Clone, Default)]
pub struct TelegramAssetSource;

impl TelegramAssetSource {
    /// Creates a new Telegram Desktop asset source.
    pub const fn new() -> Self {
        Self
    }

    /// Returns compiled-in SVG bytes for an icon directly without touching disk.
    pub const fn embedded_icon_bytes(icon: IconName) -> &'static [u8] {
        icon.embedded_bytes()
    }

    fn normalize_rel_path(path: &str) -> Option<String> {
        let mut trimmed = path.trim().trim_start_matches('/');
        if let Some(rest) = trimmed.strip_prefix("assets/") {
            trimmed = rest;
        }
        if trimmed.is_empty() {
            return None;
        }
        let mut parts = Vec::new();
        for comp in Path::new(trimmed).components() {
            match comp {
                Component::Normal(seg) => parts.push(seg.to_string_lossy().into_owned()),
                Component::CurDir => {}
                Component::ParentDir | Component::RootDir | Component::Prefix(_) => return None,
            }
        }
        if parts.is_empty() {
            None
        } else {
            Some(parts.join("/"))
        }
    }

    fn normalize_prefix(path: &str) -> Option<String> {
        let trimmed = path.trim().trim_matches('/');
        let stripped = if trimmed == "assets" {
            ""
        } else {
            trimmed.strip_prefix("assets/").unwrap_or(trimmed)
        };
        if stripped.is_empty() || stripped == "." {
            return Some(String::new());
        }
        Self::normalize_rel_path(stripped)
    }

    fn candidate_asset_roots() -> Vec<PathBuf> {
        let mut roots = Vec::with_capacity(5);
        if let Ok(exe) = std::env::current_exe() {
            if let Some(macos_dir) = exe.parent() {
                roots.push(macos_dir.join("../Resources/assets"));
                roots.push(macos_dir.join("../Resources"));
            }
        }
        if let Ok(cwd) = std::env::current_dir() {
            roots.push(cwd.join("crates/gpui_telegram_desktop/assets"));
            roots.push(cwd.join("assets"));
        }
        roots.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets"));
        roots
    }

    fn collect_embedded_entries(prefix: &str, out: &mut BTreeSet<String>) {
        let embedded = IconName::ALL
            .iter()
            .map(|icon| icon.path())
            .chain(["langs/en.strings", "langs/zh-CN.strings"]);
        for rel in embedded {
            let matches = prefix.is_empty()
                || rel
                    .strip_prefix(prefix)
                    .is_some_and(|rest| rest.starts_with('/'));
            if matches {
                out.insert(rel.to_owned());
            }
        }
    }
}

impl AssetSource for TelegramAssetSource {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        let Some(rel) = Self::normalize_rel_path(path) else {
            return Ok(None);
        };
        if let Some(bytes) = lookup_embedded_asset(&rel) {
            return Ok(Some(Cow::Borrowed(bytes)));
        }
        for root in Self::candidate_asset_roots() {
            let candidate = root.join(&rel);
            if candidate.is_file() {
                if let Ok(bytes) = std::fs::read(&candidate) {
                    return Ok(Some(Cow::Owned(bytes)));
                }
            }
        }
        Ok(None)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let Some(prefix) = Self::normalize_prefix(path) else {
            return Ok(Vec::new());
        };
        let mut entries = BTreeSet::new();
        Self::collect_embedded_entries(&prefix, &mut entries);
        for root in Self::candidate_asset_roots() {
            let dir = if prefix.is_empty() {
                root
            } else {
                root.join(&prefix)
            };
            let Ok(read_dir) = std::fs::read_dir(&dir) else {
                continue;
            };
            for entry in read_dir.flatten() {
                let name = entry.file_name().to_string_lossy().into_owned();
                if name.starts_with('.') {
                    continue;
                }
                let joined = if prefix.is_empty() {
                    name
                } else {
                    format!("{prefix}/{name}")
                };
                entries.insert(joined);
            }
        }
        Ok(entries.into_iter().map(SharedString::from).collect())
    }
}

/// Returns the asset source key for an icon (for example `"icons/search.svg"`).
pub const fn tg_icon_path(icon: IconName) -> &'static str {
    icon.path()
}

/// Builds a crisp GPUI vector SVG icon sized to `size_px` and tinted with `0xRRGGBB`.
pub fn tg_icon(icon: IconName, size_px: f32, color: u32) -> gpui::Svg {
    gpui::svg()
        .path(tg_icon_path(icon))
        .size(px(size_px))
        .flex_shrink_0()
        .text_color(rgb(color))
}

/// Builds a crisp GPUI vector SVG icon sized to `size_px` and tinted with an [`Rgba`] color.
pub fn tg_icon_rgba(icon: IconName, size_px: f32, rgba_color: Rgba) -> gpui::Svg {
    gpui::svg()
        .path(tg_icon_path(icon))
        .size(px(size_px))
        .flex_shrink_0()
        .text_color(rgba_color)
}
