//! Compose media edition manager for spoiler toggling, caption inversion, and video cover customization.
//! Upstream reference: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_compose_media_edit_manager.cpp`
//! Upstream header: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_compose_media_edit_manager.h`

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaEditConfig {
    pub spoilered: bool,
    pub invert_caption: bool,
    pub custom_video_cover_path: Option<String>,
}

impl Default for MediaEditConfig {
    fn default() -> Self {
        Self {
            spoilered: false,
            invert_caption: false,
            custom_video_cover_path: None,
        }
    }
}

pub struct HistoryViewComposeMediaEditManagerViewControls {
    pub config: MediaEditConfig,
    pub is_active: bool,
}

impl HistoryViewComposeMediaEditManagerViewControls {
    pub fn new() -> Self {
        Self {
            config: MediaEditConfig::default(),
            is_active: false,
        }
    }

    pub fn toggle_spoiler(&mut self) -> bool {
        self.config.spoilered = !self.config.spoilered;
        self.config.spoilered
    }

    pub fn toggle_invert_caption(&mut self) -> bool {
        self.config.invert_caption = !self.config.invert_caption;
        self.config.invert_caption
    }

    pub fn set_video_cover(&mut self, path: impl Into<String>) {
        self.config.custom_video_cover_path = Some(path.into());
    }

    pub fn clear(&mut self) {
        self.config = MediaEditConfig::default();
        self.is_active = false;
    }
}

impl Default for HistoryViewComposeMediaEditManagerViewControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewComposeMediaEditManagerViewControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let spoilered = self.config.spoilered;
        let inverted = self.config.invert_caption;

        div()
            .flex()
            .items_center()
            .gap_2()
            .px_3()
            .py_1()
            .bg(rgb(0x18222d))
            .text_xs()
            .child(
                div()
                    .px_2()
                    .py_0p5()
                    .rounded_sm()
                    .bg(if spoilered { rgb(0x2481cc) } else { rgb(0x2a394a) })
                    .text_color(rgb(0xffffff))
                    .child(if spoilered { "Spoiler ON" } else { "Spoiler OFF" }),
            )
            .child(
                div()
                    .px_2()
                    .py_0p5()
                    .rounded_sm()
                    .bg(if inverted { rgb(0x2481cc) } else { rgb(0x2a394a) })
                    .text_color(rgb(0xffffff))
                    .child(if inverted { "Caption Above" } else { "Caption Below" }),
            )
    }
}
