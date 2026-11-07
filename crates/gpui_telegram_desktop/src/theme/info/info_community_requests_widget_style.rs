//! Community Requests Theme Style Sheet
//! Upstream: Telegram/SourceFiles/info/community_requests/info_community_requests_widget.style
//! Exact metrics, icon offsets and button heights for community join request queues.

use gpui::{px, rgb, Pixels, Rgba};

#[derive(Debug, Clone)]
pub struct InfoCommunityRequestsWidgetCommunityRequestsStyleSheet {
    pub suggest_badge_size: Pixels,
    pub suggest_badge_border: Pixels,
    pub members_top: Pixels,
    pub members_right_skip: Pixels,
    pub pills_skip: Pixels,
    pub message_owner_info_skip: Pixels,
    pub add_all_button_height: Pixels,
    pub decline_all_button_height: Pixels,
    pub text_top: Pixels,
    pub bg_color: Rgba,
    pub text_fg: Rgba,
    pub status_fg: Rgba,
    pub active_fg: Rgba,
    pub light_fg: Rgba,
}

impl Default for InfoCommunityRequestsWidgetCommunityRequestsStyleSheet {
    fn default() -> Self {
        Self {
            suggest_badge_size: px(16.0),
            suggest_badge_border: px(1.0),
            members_top: px(11.0),
            members_right_skip: px(14.0),
            pills_skip: px(5.0),
            message_owner_info_skip: px(12.0),
            add_all_button_height: px(32.0),
            decline_all_button_height: px(32.0),
            text_top: px(7.0),
            bg_color: rgb(0xFFFFFF),
            text_fg: rgb(0x222222),
            status_fg: rgb(0x707579),
            active_fg: rgb(0x2AABEE),
            light_fg: rgb(0x2AABEE),
        }
    }
}

impl InfoCommunityRequestsWidgetCommunityRequestsStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_community_requests_style_metrics() {
        let style = InfoCommunityRequestsWidgetCommunityRequestsStyleSheet::new();
        assert_eq!(style.suggest_badge_size, px(16.0));
        assert_eq!(style.members_top, px(11.0));
        assert_eq!(style.add_all_button_height, px(32.0));
        assert_eq!(style.decline_all_button_height, px(32.0));
    }
}
