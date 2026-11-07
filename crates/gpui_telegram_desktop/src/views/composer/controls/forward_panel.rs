//! Forward panel component showing forward source attribution, caption settings, and multi-option carousel.
//! Upstream reference: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_forward_panel.cpp`
//! Upstream header: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_forward_panel.h`

use super::draft_options::ForwardOptions;
use gpui::*;
use serde::{Deserialize, Serialize};

/// Version indicators from upstream C++
/// Upstream: `constexpr auto kNameWithCaptionsVersion = -2;`
/// Upstream: `constexpr auto kNameNoCaptionsVersion = -3;`
pub const NAME_WITH_CAPTIONS_VERSION: i32 = -2;
pub const NAME_NO_CAPTIONS_VERSION: i32 = -3;

/// Forwarded message item description
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForwardedItem {
    pub id: u64,
    pub from_name: String,
    pub text_preview: String,
    pub has_media: bool,
    pub has_caption: bool,
}

/// Forward panel state and business logic model
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForwardPanelModel {
    pub items: Vec<ForwardedItem>,
    pub options: ForwardOptions,
}

impl ForwardPanelModel {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            options: ForwardOptions::PreserveInfo,
        }
    }

    pub fn with_items(items: Vec<ForwardedItem>) -> Self {
        Self {
            items,
            options: ForwardOptions::PreserveInfo,
        }
    }

    /// Cycle to next forward option:
    /// PreserveInfo -> NoNames -> NoNamesAndCaptions -> PreserveInfo
    /// Upstream: `ForwardPanel::editToNextOption()` (`history_view_forward_panel.cpp:L245-255`)
    pub fn edit_to_next_option(&mut self) -> ForwardOptions {
        self.options = match self.options {
            ForwardOptions::PreserveInfo => ForwardOptions::NoNames,
            ForwardOptions::NoNames => ForwardOptions::NoNamesAndCaptions,
            ForwardOptions::NoNamesAndCaptions => ForwardOptions::PreserveInfo,
        };
        self.options
    }

    /// Header text reflecting current forward count and attribution privacy
    pub fn header_text(&self) -> String {
        let count = self.items.len();
        if count == 0 {
            return String::new();
        }

        match self.options {
            ForwardOptions::PreserveInfo => {
                let first_author = self.items[0].from_name.as_str();
                if count == 1 {
                    format!("Forward message from {}", first_author)
                } else {
                    format!("Forward {} messages from {}", count, first_author)
                }
            }
            ForwardOptions::NoNames => {
                if count == 1 {
                    "Forward message (hide sender name)".to_string()
                } else {
                    format!("Forward {} messages (hide sender names)", count)
                }
            }
            ForwardOptions::NoNamesAndCaptions => {
                if count == 1 {
                    "Forward message (hide name and caption)".to_string()
                } else {
                    format!("Forward {} messages (hide names and captions)", count)
                }
            }
        }
    }

    /// Preview body text for the forwarded items
    pub fn preview_text(&self) -> String {
        if self.items.is_empty() {
            return String::new();
        }
        if self.options == ForwardOptions::NoNamesAndCaptions
            && self.items.iter().any(|item| item.has_media)
        {
            return "[Media without caption]".to_string();
        }
        self.items[0].text_preview.clone()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn remove_item(&mut self, id: u64) {
        self.items.retain(|item| item.id != id);
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl Default for ForwardPanelModel {
    fn default() -> Self {
        Self::new()
    }
}

/// GPUI View component for the forward panel above composer
pub struct ForwardPanelView {
    pub model: ForwardPanelModel,
    pub is_visible: bool,
}

impl ForwardPanelView {
    pub fn new() -> Self {
        Self {
            model: ForwardPanelModel::new(),
            is_visible: true,
        }
    }

    pub fn set_items(&mut self, items: Vec<ForwardedItem>) {
        self.model.items = items;
        self.is_visible = !self.model.items.is_empty();
    }
}

impl Default for ForwardPanelView {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for ForwardPanelView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible || self.model.is_empty() {
            return div().into_any_element();
        }

        let header = self.model.header_text();
        let preview = self.model.preview_text();
        let opt_label = match self.model.options {
            ForwardOptions::PreserveInfo => "Show sender",
            ForwardOptions::NoNames => "Hide sender",
            ForwardOptions::NoNamesAndCaptions => "Hide captions",
        };

        div()
            .flex()
            .items_center()
            .justify_between()
            .h_10()
            .px_3()
            .bg(rgb(0x18222d))
            .border_l_2()
            .border_color(rgb(0x2481cc))
            .rounded_r_md()
            .text_sm()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_0()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x2481cc))
                            .child(header),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x8e8e93))
                            .truncate()
                            .child(preview),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // Toggle option pill
                    .child(
                        div()
                            .px_2()
                            .py_0p5()
                            .rounded_sm()
                            .bg(rgb(0x2a394a))
                            .text_color(rgb(0xffffff))
                            .text_xs()
                            .child(opt_label),
                    )
                    // Close / cancel forward button
                    .child(
                        div()
                            .text_color(rgb(0x8e8e93))
                            .text_xs()
                            .child("x"),
                    ),
            )
            .into_any_element()
    }
}
