//! Channel & Bot About / Description View in History
//! Faithful upstream C++ extraction:
//! - `history_view_about_view.cpp:L30-L150`
//! - `history_view_about_view.h:L20-L89`

use gpui::*;

pub const ABOUT_VIEW_PREMIUM_REQUIRED_WIDTH: f32 = 186.0;
pub const ABOUT_VIEW_STARS_PER_MESSAGE_WIDTH: f32 = 226.0;
pub const ABOUT_VIEW_CHAT_INTRO_WIDTH: f32 = 224.0;
pub const ABOUT_VIEW_NEW_PEER_WIDTH: f32 = 320.0;
pub const ABOUT_VIEW_NEW_THREAD_ABOUT_MAX_WIDTH: f32 = 180.0;
pub const ABOUT_VIEW_NEW_THREAD_ABOUT_ICON_OUTER: f32 = 60.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AboutViewType {
    #[default]
    ChannelAbout,
    BotIntro,
    DirectMessagesLocked,
    PremiumRequired,
}

#[derive(Debug, Clone)]
pub struct HistoryViewAboutViewView {
    pub view_type: AboutViewType,
    pub title: String,
    pub description: String,
    pub is_verified: bool,
    pub subscriber_count: Option<u64>,
    pub custom_bot_commands: Vec<(String, String)>,
}

impl HistoryViewAboutViewView {
    pub fn new(title: String, description: String, view_type: AboutViewType) -> Self {
        Self {
            view_type,
            title,
            description,
            is_verified: false,
            subscriber_count: None,
            custom_bot_commands: Vec::new(),
        }
    }

    pub fn set_verified(&mut self, verified: bool) {
        self.is_verified = verified;
    }

    pub fn set_subscribers(&mut self, count: u64) {
        self.subscriber_count = Some(count);
    }
}

impl Default for HistoryViewAboutViewView {
    fn default() -> Self {
        Self::new("Channel Info".into(), "Official channel description".into(), AboutViewType::ChannelAbout)
    }
}

impl Render for HistoryViewAboutViewView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let title = &self.title;
        let desc = &self.description;

        div()
            .flex()
            .flex_col()
            .items_center()
            .w_full()
            .max_w(px(ABOUT_VIEW_NEW_PEER_WIDTH))
            .mx_auto()
            .my(px(16.0))
            .p(px(16.0))
            .bg(rgba(0x00000006))
            .rounded(px(16.0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_size(px(16.0))
                    .text_color(rgb(0x222222))
                    .child(title.clone())
            )
            .child(
                div()
                    .mt(px(6.0))
                    .text_size(px(13.0))
                    .text_color(rgb(0x666666))
                    .child(desc.clone())
            )
    }
}
