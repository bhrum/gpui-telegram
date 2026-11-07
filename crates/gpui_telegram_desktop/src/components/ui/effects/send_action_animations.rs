//! Upstream: `Telegram/SourceFiles/ui/effects/send_action_animations.cpp:L20-L26`, `L135-L161`
//! Upstream: `Telegram/SourceFiles/ui/effects/send_action_animations.h:L18-L56`
//!
//! Chat send action animations (typing dots, voice recording arcs, upload arrows,
//! video round progress, speaking waves) shown in header subtext and conversation rows.

use gpui::*;

pub const TYPING_DOTS_COUNT: usize = 3;
pub const RECORD_ARCS_COUNT: usize = 4;
pub const UPLOAD_ARROWS_COUNT: usize = 3;
pub const ROUND_PERIOD_MS: u64 = 800;
pub const SPEAKING_DURATION_MS: u64 = 3200;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SendActionType {
    Typing,
    RecordVoice,
    UploadVoice,
    RecordVideo,
    UploadVideo,
    UploadPhoto,
    UploadDocument,
    ChooseSticker,
    Speaking,
}

#[derive(Debug, Clone)]
pub struct SendActionAnimationsEffects {
    action_type: SendActionType,
    elapsed_ms: u64,
    dot_radius: f32,
}

impl SendActionAnimationsEffects {
    pub fn new(action_type: SendActionType) -> Self {
        Self {
            action_type,
            elapsed_ms: 0,
            dot_radius: 2.5,
        }
    }

    pub fn tick(&mut self, delta_ms: u64) {
        self.elapsed_ms = (self.elapsed_ms + delta_ms) % SPEAKING_DURATION_MS;
    }

    /// C++: `send_action_animations.cpp:L144-L154`:
    /// Computes typing dot radius oscillation with circular ease-out
    pub fn dot_scale(&self, dot_index: usize) -> f32 {
        let period_ms = 900.0;
        let delta_time = 150.0;
        let current = (self.elapsed_ms as f32 + period_ms - (dot_index as f32 * delta_time)) % period_ms;
        let half_period = period_ms / 2.0;
        if current < half_period {
            let t = current / half_period;
            1.0 + 0.6 * (1.0 - (t - 1.0) * (t - 1.0)).sqrt()
        } else {
            let t = (current - half_period) / half_period;
            1.6 - 0.6 * (1.0 - (t - 1.0) * (t - 1.0)).sqrt()
        }
    }

    pub fn action_type(&self) -> SendActionType {
        self.action_type
    }

    pub fn set_action_type(&mut self, action_type: SendActionType) {
        self.action_type = action_type;
    }
}

impl Default for SendActionAnimationsEffects {
    fn default() -> Self {
        Self::new(SendActionType::Typing)
    }
}

impl Render for SendActionAnimationsEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let base_r = self.dot_radius;
        div()
            .flex()
            .items_center()
            .gap(px(3.0))
            .children((0..TYPING_DOTS_COUNT).map(|i| {
                let r = base_r * self.dot_scale(i);
                div()
                    .size(px(r * 2.0))
                    .rounded(px(r))
                    .bg(rgb(0x2AABEE))
            }))
    }
}
