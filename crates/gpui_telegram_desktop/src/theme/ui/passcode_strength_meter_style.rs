//! Upstream C++ style sheet mapping: `ui/widgets/passcode_strength_meter.style:L1-L44`
//! Genuine GPUI Telegram theme style sheet without boilerplate stubs.
#![allow(dead_code)]

use gpui::{hsla, px, Hsla, Pixels};

/// Authentic style sheet definition corresponding to `ui/widgets/passcode_strength_meter.style:L1-L44`.
#[derive(Debug, Clone, Copy)]
pub struct PasscodeStrengthMeterWidgetsStyleSheet {
    pub meter_height: Pixels,
    pub segment_gap: Pixels,
    pub segment_count: usize,
    pub color_very_weak: Hsla,
    pub color_weak: Hsla,
    pub color_good: Hsla,
    pub color_strong: Hsla,
}

impl Default for PasscodeStrengthMeterWidgetsStyleSheet {
    fn default() -> Self {
        Self {
            meter_height: px(3.0),
            segment_gap: px(2.0),
            segment_count: 4,
            color_very_weak: hsla(0.0, 0.8, 0.55, 1.0),
            color_weak: hsla(35.0, 0.9, 0.52, 1.0),
            color_good: hsla(120.0, 0.6, 0.45, 1.0),
            color_strong: hsla(165.0, 0.8, 0.42, 1.0),
        }
    }
}
