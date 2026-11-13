//! Authentic GPUI Glare Tooltip Component with Periodic Metallic Shine.
//! Reference: Telegram/SourceFiles/ui/widgets/glare_tooltip.cpp:L18-L60
//! Reference: Telegram/SourceFiles/ui/widgets/glare_tooltip.h:L24-L82

#![allow(dead_code, unused_imports, unused_variables)]

/// Glare animation duration step per unit width: 320ms.
/// Reference: `glare_tooltip.cpp:L19`
pub const K_GLARE_DURATION_STEP_MS: u64 = 320;

/// Interval between periodic glare shine sweeps: 1000ms.
/// Reference: `glare_tooltip.cpp:L20`
pub const K_GLARE_TIMEOUT_MS: u64 = 1000;

/// Target symbol for premium feature badge and promotional tooltip.
#[derive(Debug, Clone)]
pub struct GlareTooltipWidgets {
    pub text: String,
    pub is_visible: bool,
    pub opacity: f32,
    pub padding_horizontal: f32,
    pub padding_vertical: f32,
    pub arrow_size: f32,
    pub glare_active: bool,
    pub glare_progress: f32,
    pub last_glare_time_ms: u64,
}

impl Default for GlareTooltipWidgets {
    fn default() -> Self {
        Self::new("Tooltip")
    }
}

impl GlareTooltipWidgets {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            is_visible: false,
            opacity: 0.0,
            padding_horizontal: 12.0,
            padding_vertical: 6.0,
            arrow_size: 5.0,
            glare_active: false,
            glare_progress: 0.0,
            last_glare_time_ms: 0,
        }
    }

    /// Fades the tooltip in or out.
    /// Reference: `glare_tooltip.cpp:L47-L60`
    pub fn fade(&mut self, show: bool) {
        self.is_visible = show;
        self.opacity = if show { 1.0 } else { 0.0 };
        if show {
            self.show_glare();
        } else {
            self.glare_active = false;
        }
    }

    pub fn show_glare(&mut self) {
        self.glare_active = true;
        self.glare_progress = 0.0;
    }

    /// Calculates glare sweep duration based on tooltip geometry.
    /// Reference: `glare_tooltip.cpp:L40-L42`
    pub fn calculate_glare_duration_ms(&self, width: f32, height: f32) -> u64 {
        let glare_size = (height * 3.0).max(1.0);
        let glare_range = width + glare_size;
        ((glare_range * K_GLARE_DURATION_STEP_MS as f32) / glare_size) as u64
    }

    /// Ticks glare progress and restarts periodic glare if due.
    pub fn tick(&mut self, now_ms: u64, width: f32, height: f32) {
        if !self.is_visible {
            return;
        }
        let duration = self.calculate_glare_duration_ms(width, height).max(1);
        if self.glare_active {
            let elapsed = now_ms.saturating_sub(self.last_glare_time_ms);
            self.glare_progress = (elapsed as f32 / duration as f32).min(1.0);
            if self.glare_progress >= 1.0 {
                self.glare_active = false;
                self.last_glare_time_ms = now_ms;
            }
        } else if now_ms >= self.last_glare_time_ms + K_GLARE_TIMEOUT_MS {
            self.show_glare();
            self.last_glare_time_ms = now_ms;
        }
    }
}
