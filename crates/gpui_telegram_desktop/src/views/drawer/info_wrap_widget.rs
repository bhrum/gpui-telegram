//! Info Wrap Widget Container
//! Upstream: Telegram/SourceFiles/info/info_wrap_widget.h, Telegram/SourceFiles/info/info_wrap_widget.cpp

use gpui::*;
use super::info_controller::{InfoController, InfoKey, InfoSectionType};
use super::info_top_bar::InfoTopBar;
use super::info_content_widget::InfoContentWidget;
use super::info_layer_widget::InfoLayerWidget;

pub const K_DEFAULT_DRAWER_WIDTH: f32 = 320.0;
pub const K_MIN_DRAWER_WIDTH: f32 = 280.0;
pub const K_MAX_DRAWER_WIDTH: f32 = 500.0;

#[derive(Debug, Clone)]
pub struct InfoWrapWidget {
    pub controller: InfoController,
    pub top_bar: InfoTopBar,
    pub content: InfoContentWidget,
    pub layer: InfoLayerWidget,
    pub width: f32,
    pub height: f32,
    pub is_open: bool,
}

impl Default for InfoWrapWidget {
    fn default() -> Self {
        Self {
            controller: InfoController::default(),
            top_bar: InfoTopBar::default(),
            content: InfoContentWidget::default(),
            layer: InfoLayerWidget::default(),
            width: K_DEFAULT_DRAWER_WIDTH,
            height: 600.0,
            is_open: true,
        }
    }
}

impl InfoWrapWidget {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_peer(peer_id: u64) -> Self {
        let mut widget = Self::default();
        widget.controller.set_key(InfoKey::peer(peer_id));
        widget
    }

    pub fn set_geometry(&mut self, width: f32, height: f32) {
        self.width = width.clamp(K_MIN_DRAWER_WIDTH, K_MAX_DRAWER_WIDTH);
        self.height = height.max(200.0);
        self.content.set_geometry(self.width, self.height - self.top_bar.height);
    }

    pub fn show_section(&mut self, section: InfoSectionType) {
        self.controller.show_section(section);
        self.top_bar.set_can_go_back(self.controller.can_go_back());
        let title = match section {
            InfoSectionType::Profile => "Info",
            InfoSectionType::Media => "Shared Media",
            InfoSectionType::CommonGroups => "Common Groups",
            InfoSectionType::Downloads => "Downloads",
            InfoSectionType::Members => "Members",
            InfoSectionType::Settings => "Settings",
            InfoSectionType::Stories => "Stories",
            InfoSectionType::Reactions => "Reactions",
            InfoSectionType::PollResults => "Poll Results",
            InfoSectionType::CommunityRequests => "Join Requests",
            InfoSectionType::SimilarPeers => "Similar Channels",
        };
        self.top_bar.set_title(title);
    }

    pub fn go_back(&mut self) -> bool {
        if let Some(prev) = self.controller.go_back() {
            self.show_section(prev);
            true
        } else {
            false
        }
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_info_wrap_widget_lifecycle() {
        let mut wrap = InfoWrapWidget::with_peer(999);
        assert!(wrap.is_open);
        assert_eq!(wrap.width, K_DEFAULT_DRAWER_WIDTH);

        wrap.show_section(InfoSectionType::Downloads);
        assert_eq!(wrap.top_bar.title, "Downloads");
        assert!(wrap.top_bar.can_go_back);

        assert!(wrap.go_back());
        assert_eq!(wrap.top_bar.title, "Info");
        assert!(!wrap.top_bar.can_go_back);

        wrap.close();
        assert!(!wrap.is_open);
    }
}
