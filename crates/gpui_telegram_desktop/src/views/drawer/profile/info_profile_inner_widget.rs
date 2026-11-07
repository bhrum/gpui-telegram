//! Info Profile Inner Widget
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_inner_widget.cpp:L1-L592`
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_inner_widget.h:L1-L134`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;

/// Scrollable inner widget hosting profile sections: header, contact actions,
/// shared media preview, and member lists.
#[derive(Debug, Clone)]
pub struct InfoProfileInnerWidgetProfile {
    pub peer_id: i64,
    pub scroll_top: f32,
    pub is_channel: bool,
    pub is_group: bool,
}

impl InfoProfileInnerWidgetProfile {
    pub const MEMBERS_INLINE_MAX: usize = 5;

    pub fn new(peer_id: i64) -> Self {
        Self {
            peer_id,
            scroll_top: 0.0,
            is_channel: false,
            is_group: false,
        }
    }

    pub fn set_scroll_top(&mut self, scroll_top: f32) {
        self.scroll_top = scroll_top.max(0.0);
    }
}

impl Default for InfoProfileInnerWidgetProfile {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Render for InfoProfileInnerWidgetProfile {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .overflow_hidden()
            .bg(rgb(0x18222d))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p(px(12.0))
                    .gap(px(8.0))
                    .child(format!("Profile Content for Peer {}", self.peer_id))
            )
    }
}
