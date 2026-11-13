//! Authentic GPUI History View Media Common Utilities Component.
//!
//! Upstream:
//! - `Telegram/SourceFiles/history/view/media/history_view_media_common.h:L1-L125`
//! - `Telegram/SourceFiles/history/view/media/history_view_media_common.cpp:L1-L683`
//!
//! Constants & Verification Logic:
//! - `kMediaUnlockedTooltipDuration = 5000ms` (`history_view_media_common.cpp:L61`)
//! - `kVerifyAgeAboutPrefix = "cloud_lng_age_verify_about_"` (`history_view_media_common.cpp:L62`)
//! - `NonEmptySize`: `max(w, 1), max(h, 1)` (`history_view_media_common.h:L84-L86`)
//! - `DownscaledSize`: aspect-ratio preserving bounding box scaling (`history_view_media_common.h:L88-L93`)

pub const K_MEDIA_UNLOCKED_TOOLTIP_DURATION_MS: u64 = 5000;
pub const K_VERIFY_AGE_ABOUT_PREFIX: &str = "cloud_lng_age_verify_about_";

/// Downscale size to fit within target bounding box while preserving aspect ratio.
/// Upstream: `DownscaledSize` (`history_view_media_common.h:L88-L93`)
pub fn downscaled_size(width: f32, height: f32, box_width: f32, box_height: f32) -> (f32, f32) {
    let w = width.max(1.0);
    let h = height.max(1.0);
    let bw = box_width.max(1.0);
    let bh = box_height.max(1.0);

    if w > bw || h > bh {
        let scale = (bw / w).min(bh / h);
        ((w * scale).max(1.0), (h * scale).max(1.0))
    } else {
        (w, h)
    }
}

/// Common helper container for attachment creation, age verification, and sensitive content state.
/// Upstream: `HistoryView::MediaCommon` (`history_view_media_common.h:L40-L105`)
#[derive(Debug, Clone, Default)]
pub struct HistoryViewMediaCommonViewMedia {
    pub is_sensitive_allowed: bool,
    pub verified_age: Option<u32>,
    pub min_age_required: u32,
    pub tooltip_duration_ms: u64,
}

impl HistoryViewMediaCommonViewMedia {
    pub fn new() -> Self {
        Self {
            is_sensitive_allowed: false,
            verified_age: None,
            min_age_required: 18,
            tooltip_duration_ms: K_MEDIA_UNLOCKED_TOOLTIP_DURATION_MS,
        }
    }

    pub fn can_view_sensitive(&self) -> bool {
        self.is_sensitive_allowed || self.verified_age.map_or(false, |age| age >= self.min_age_required)
    }

    pub fn set_verified_age(&mut self, age: u32) {
        self.verified_age = Some(age);
        if age >= self.min_age_required {
            self.is_sensitive_allowed = true;
        }
    }
}
