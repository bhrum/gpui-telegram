//! AI compose button with 640ms phase-sparkle animation cycle and persistent first-time tooltip.
//! Upstream reference: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_compose_ai_button.cpp`
//! Upstream tooltip: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_compose_ai_tooltip.cpp`

use gpui::*;
use serde::{Deserialize, Serialize};

/// Animation cycle duration in milliseconds
/// Upstream: `constexpr auto kAnimationDuration = crl::time(640);` (`history_view_compose_ai_button.cpp:L17`)
pub const AI_ANIMATION_DURATION_MS: u64 = 640;

/// Tooltip vertical offset skip in pixels
/// Upstream: `st::historyAiComposeTooltipSkip = 8px;` (`history_view_compose_ai_tooltip.cpp:L82`)
pub const TOOLTIP_SKIP_PX: f32 = 8.0;

/// Calculate dual star opacities according to 4-phase sparkle cycle
/// Upstream: `ComposeAiButton::paintEvent` (`history_view_compose_ai_button.cpp:L77-90`)
pub fn calc_sparkle_opacities(progress: f32) -> (f32, f32) {
    let p = progress.clamp(0.0, 1.0);
    if p < 0.25 {
        let star1 = 1.0 - (p / 0.25);
        let star2 = 1.0;
        (star1, star2)
    } else if p < 0.50 {
        let star1 = 0.0;
        let star2 = 1.0 - ((p - 0.25) / 0.25);
        (star1, star2)
    } else if p < 0.75 {
        let star1 = (p - 0.50) / 0.25;
        let star2 = 0.0;
        (star1, star2)
    } else {
        let star1 = 1.0;
        let star2 = (p - 0.75) / 0.25;
        (star1, star2)
    }
}

/// Tooltip manager with local preference dismissal persistence
/// Upstream: `ComposeTooltipManager` (`history_view_compose_ai_tooltip.cpp:L20-91`)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiTooltipManager {
    pub pref_key: String,
    pub is_dismissed: bool,
    pub is_visible: bool,
    pub tooltip_text: String,
}

impl AiTooltipManager {
    pub fn new(pref_key: impl Into<String>, tooltip_text: impl Into<String>) -> Self {
        Self {
            pref_key: pref_key.into(),
            is_dismissed: false,
            is_visible: false,
            tooltip_text: tooltip_text.into(),
        }
    }

    /// User dismissed tooltip, remember persistently
    /// Upstream: `hideAndRemember()` (`history_view_compose_ai_tooltip.cpp:L48-54`)
    pub fn hide_and_remember(&mut self) {
        self.is_dismissed = true;
        self.is_visible = false;
    }

    /// Update visibility based on whether the host button is shown
    pub fn update_visibility(&mut self, button_shown: bool) {
        self.is_visible = button_shown && !self.is_dismissed;
    }

    pub fn should_display(&self, button_visible: bool) -> bool {
        button_visible && !self.is_dismissed
    }
}

impl Default for AiTooltipManager {
    fn default() -> Self {
        Self::new(
            "compose_ai_tooltip_dismissed",
            "Generate or refine messages with AI",
        )
    }
}

/// GPUI View component for Compose AI Button with Sparkle Animation
pub struct ComposeAiButton {
    pub progress: f32,
    pub is_premium: bool,
    pub tooltip: AiTooltipManager,
    pub is_hovered: bool,
}

impl ComposeAiButton {
    pub fn new() -> Self {
        Self {
            progress: 0.0,
            is_premium: false,
            tooltip: AiTooltipManager::default(),
            is_hovered: false,
        }
    }

    pub fn set_animation_progress(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 1.0);
    }

    pub fn tick_animation(&mut self, delta_ms: u64) {
        let delta_ratio = delta_ms as f32 / AI_ANIMATION_DURATION_MS as f32;
        self.progress = (self.progress + delta_ratio) % 1.0;
    }
}

impl Default for ComposeAiButton {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for ComposeAiButton {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let (star1_opacity, star2_opacity) = calc_sparkle_opacities(self.progress);
        let show_tooltip = self.tooltip.should_display(true);
        let tooltip_text = self.tooltip.tooltip_text.clone();

        let mut root = div()
            .relative()
            .flex()
            .items_center()
            .justify_center()
            .w_8()
            .h_8()
            .rounded_full()
            .bg(rgb(0x1e2c3a))
            .hover(|s| s.bg(rgb(0x2481cc)))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_0p5()
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0xffffff))
                            .font_weight(FontWeight::BOLD)
                            .child("AI"),
                    )
                    // Animated sparkle stars
                    .child(
                        div()
                            .text_xs()
                            .opacity(star1_opacity)
                            .text_color(rgb(0xffd700))
                            .child("*"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .opacity(star2_opacity)
                            .text_color(rgb(0xffd700))
                            .child("*"),
                    ),
            );

        if show_tooltip {
            root = root.child(
                div()
                    .absolute()
                    .bottom(px(32.0 + TOOLTIP_SKIP_PX))
                    .left(px(-80.0))
                    .w(px(200.0))
                    .p_2()
                    .bg(rgb(0x18222d))
                    .border_1()
                    .border_color(rgb(0x2a394a))
                    .rounded_md()
                    .shadow_lg()
                    .text_xs()
                    .text_color(rgb(0xffffff))
                    .child(tooltip_text),
            );
        }

        root
    }
}
