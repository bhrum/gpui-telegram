//! Upstream: `Telegram/SourceFiles/ui/effects/message_sending_animation_common.h:L12-L26`
//!
//! Message sending flight animation metadata. Records origin geometry, media type,
//! and crop settings for smooth view-to-view transition when dispatching messages.

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageSendingFlightType {
    None,
    Sticker,
    Gif,
    Emoji,
    Text,
}

#[derive(Debug, Clone)]
pub struct MessageSendingAnimationCommonEffects {
    pub flight_type: MessageSendingFlightType,
    pub local_msg_id: Option<i64>,
    pub start_x: f32,
    pub start_y: f32,
    pub start_width: f32,
    pub start_height: f32,
    pub crop: bool,
}

impl MessageSendingAnimationCommonEffects {
    pub fn new(flight_type: MessageSendingFlightType, x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            flight_type,
            local_msg_id: None,
            start_x: x,
            start_y: y,
            start_width: width,
            start_height: height,
            crop: false,
        }
    }

    pub fn with_local_id(mut self, id: i64) -> Self {
        self.local_msg_id = Some(id);
        self
    }

    pub fn set_crop(&mut self, crop: bool) {
        self.crop = crop;
    }

    pub fn is_active(&self) -> bool {
        self.flight_type != MessageSendingFlightType::None
    }
}

impl Default for MessageSendingAnimationCommonEffects {
    fn default() -> Self {
        Self::new(MessageSendingFlightType::None, 0.0, 0.0, 0.0, 0.0)
    }
}

impl Render for MessageSendingAnimationCommonEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_0()
            .invisible()
    }
}
