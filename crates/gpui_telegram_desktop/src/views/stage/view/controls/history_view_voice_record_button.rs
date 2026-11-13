//! Voice record pulsating amplitude button with circle hit testing.
//! Upstream reference: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_voice_record_button.cpp`
//! Upstream header: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_voice_record_button.h`

use gpui::*;
use serde::{Deserialize, Serialize};

pub struct HistoryViewVoiceRecordButtonViewControls {
    pub level: u16,
    pub radius_px: f32,
    pub is_holding: bool,
    pub is_hovered: bool,
}

impl HistoryViewVoiceRecordButtonViewControls {
    pub fn new() -> Self {
        Self {
            level: 0,
            radius_px: 22.0,
            is_holding: false,
            is_hovered: false,
        }
    }

    /// Circle hit testing for touch/pointer gestures
    /// Upstream: `VoiceRecordButton::inCircle` (`history_view_voice_record_button.cpp:L250-265`)
    pub fn in_circle(&self, center_offset_x: f32, center_offset_y: f32) -> bool {
        let dist_sq = center_offset_x * center_offset_x + center_offset_y * center_offset_y;
        dist_sq <= self.radius_px * self.radius_px
    }

    pub fn set_level(&mut self, level: u16) {
        self.level = level;
    }
}

impl Default for HistoryViewVoiceRecordButtonViewControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewVoiceRecordButtonViewControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_holding = self.is_holding;
        let pulse_extra = (self.level as f32 / 1000.0).clamp(0.0, 8.0);
        let size = self.radius_px * 2.0 + pulse_extra;

        div()
            .flex()
            .items_center()
            .justify_center()
            .w(px(size))
            .h(px(size))
            .rounded_full()
            .bg(if is_holding { rgb(0xff3b30) } else { rgb(0x2481cc) })
            .hover(|s| s.bg(rgb(0x1e72b4)))
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0xffffff))
                    .child("🎙"),
            )
    }
}
