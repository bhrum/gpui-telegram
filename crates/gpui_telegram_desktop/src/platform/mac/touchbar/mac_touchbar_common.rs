//! macOS TouchBar Common Geometry Metrics and Scaling
//! Upstream Source: Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_common.h, Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_common.mm
//! Subsystem: platform | Feature ID: TCG-N46 | Disposition: rust_domain_or_server_1to1

use gpui_telegram_core::error::Result;
use serde::{Deserialize, Serialize};

/// Common geometric metrics and layout formulas for macOS TouchBar.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacTouchbarCommonMacTouchbar {
    pub circle_diameter: i32,
    pub icon_size: i32,
    pub width_scale_factor: f32,
}

impl MacTouchbarCommonMacTouchbar {
    /// Upstream constant: `kCircleDiameter = 30`
    pub const CIRCLE_DIAMETER: i32 = 30;

    pub fn new() -> Self {
        Self {
            circle_diameter: Self::CIRCLE_DIAMETER,
            icon_size: 22,
            width_scale_factor: 1.2,
        }
    }

    /// Calculates required TouchBar width from label text.
    ///
    /// Upstream formula: `(int)ceil([[NSTextField labelWithString:s] frame].size.width) * 1.2`
    pub fn width_from_string(&self, s: &str) -> i32 {
        let approx_char_width = 8.5f32;
        let base_width = (s.chars().count() as f32 * approx_char_width).ceil();
        (base_width * self.width_scale_factor).ceil() as i32
    }

    /// Sizing helper for standard circular touchbar items.
    pub fn standard_circle_size(&self) -> (i32, i32) {
        (self.circle_diameter, self.circle_diameter)
    }

    pub fn set_width_scale_factor(&mut self, factor: f32) -> Result<()> {
        self.width_scale_factor = factor.max(1.0);
        Ok(())
    }
}

impl Default for MacTouchbarCommonMacTouchbar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_touchbar_metrics() {
        let tb = MacTouchbarCommonMacTouchbar::new();
        assert_eq!(tb.circle_diameter, 30);
        assert_eq!(tb.standard_circle_size(), (30, 30));

        let w = tb.width_from_string("Bold");
        assert!(w >= 40 && w <= 50);
    }
}
