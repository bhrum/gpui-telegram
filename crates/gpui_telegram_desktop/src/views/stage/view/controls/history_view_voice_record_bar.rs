//! Voice record bar stage integration wrapper.
//! Upstream reference: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_voice_record_bar.cpp`
//! Upstream header: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_voice_record_bar.h`

use crate::views::composer::controls::{
    StopType, VoiceRecordBar, VoiceRecordState, ANIMATION_THRESHOLD, AUDIO_VOICE_MAX_LENGTH_SECS,
    AUDIO_VOICE_UPDATE_VIEW_MS, LOCK_HEIGHT_PX,
};
use gpui::*;
use serde::{Deserialize, Serialize};

pub struct HistoryViewVoiceRecordBarViewControls {
    pub bar: VoiceRecordBar,
}

impl HistoryViewVoiceRecordBarViewControls {
    pub fn new() -> Self {
        Self {
            bar: VoiceRecordBar::new(),
        }
    }

    pub fn start_holding(&mut self, is_video: bool) {
        self.bar.start_holding(is_video);
    }

    pub fn handle_drag(&mut self, dx: f32, dy: f32) {
        self.bar.handle_drag(dx, dy);
    }

    pub fn handle_release(&mut self) {
        self.bar.handle_release();
    }

    pub fn tick_duration(&mut self, delta_ms: u64) {
        self.bar.tick_duration(delta_ms);
    }

    pub fn stop(&mut self, stop_type: StopType) {
        self.bar.stop(stop_type);
    }

    pub fn is_locked(&self) -> bool {
        self.bar.is_locked()
    }

    pub fn is_active(&self) -> bool {
        self.bar.is_active()
    }
}

impl Default for HistoryViewVoiceRecordBarViewControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewVoiceRecordBarViewControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_active = self.bar.is_active();
        let dur = self.bar.formatted_duration();

        div()
            .flex()
            .items_center()
            .justify_between()
            .h_10()
            .px_3()
            .bg(rgb(0x18222d))
            .border_t_1()
            .border_color(rgb(0x2a394a))
            .text_xs()
            .text_color(rgb(0xffffff))
            .child(format!("Voice record bar [{}]: {}", dur, if is_active { "Recording" } else { "Idle" }))
    }
}
