//! Draft options controller and FSM for configuring reply quotes, forward attribution, and link preview positions.
//! Upstream reference: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_draft_options.cpp`
//! Upstream header: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_draft_options.h`

use gpui::*;
use serde::{Deserialize, Serialize};

/// Active options section
/// Upstream: `enum class Section { Reply, Forward, Link };` (`history_view_draft_options.cpp:L64-68`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DraftSection {
    Reply,
    Forward,
    Link,
}

impl Default for DraftSection {
    fn default() -> Self {
        Self::Reply
    }
}

/// Forward privacy options
/// Upstream: `Data::ForwardOptions` (`PreserveInfo`, `NoNames`, `NoNamesAndCaptions`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ForwardOptions {
    PreserveInfo,
    NoNames,
    NoNamesAndCaptions,
}

impl Default for ForwardOptions {
    fn default() -> Self {
        Self::PreserveInfo
    }
}

/// WebPage preview customization options
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebPageDraftOptions {
    /// Invert media position: when true, media appears above message text (`history_view_draft_options.cpp:L376`)
    pub invert_media: bool,
    /// Force large media preview instead of compact square thumbnail
    pub force_large_media: bool,
    /// Manual preview link selection override
    pub manual: bool,
    /// Chosen URL when multiple links are present
    pub selected_link: Option<String>,
}

impl Default for WebPageDraftOptions {
    fn default() -> Self {
        Self {
            invert_media: false,
            force_large_media: true,
            manual: false,
            selected_link: None,
        }
    }
}

/// Reply quote configuration options
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplyDraftOptions {
    pub reply_to_msg_id: u64,
    pub quote_text: Option<String>,
    pub quote_offset: usize,
    pub thread_topic_id: Option<u64>,
    pub target_chat_title: Option<String>,
}

impl Default for ReplyDraftOptions {
    fn default() -> Self {
        Self {
            reply_to_msg_id: 0,
            quote_text: None,
            quote_offset: 0,
            thread_topic_id: None,
            target_chat_title: None,
        }
    }
}

/// Draft options synchronization state machine
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DraftOptionsFsm {
    pub active_section: DraftSection,
    pub forward: ForwardOptions,
    pub webpage: WebPageDraftOptions,
    pub reply: ReplyDraftOptions,
    pub available_links: Vec<String>,
}

impl DraftOptionsFsm {
    pub fn new() -> Self {
        Self {
            active_section: DraftSection::Reply,
            forward: ForwardOptions::PreserveInfo,
            webpage: WebPageDraftOptions::default(),
            reply: ReplyDraftOptions::default(),
            available_links: Vec::new(),
        }
    }

    /// Select active configuration section
    pub fn switch_section(&mut self, section: DraftSection) {
        self.active_section = section;
    }

    /// Cycle to next forward option (PreserveInfo -> NoNames -> NoNamesAndCaptions -> PreserveInfo)
    pub fn cycle_forward_options(&mut self) -> ForwardOptions {
        self.forward = match self.forward {
            ForwardOptions::PreserveInfo => ForwardOptions::NoNames,
            ForwardOptions::NoNames => ForwardOptions::NoNamesAndCaptions,
            ForwardOptions::NoNamesAndCaptions => ForwardOptions::PreserveInfo,
        };
        self.forward
    }

    /// Toggle webpage media layout position (above text vs below text)
    pub fn toggle_invert_media(&mut self) -> bool {
        self.webpage.invert_media = !self.webpage.invert_media;
        self.webpage.invert_media
    }

    /// Toggle large media preview size
    pub fn toggle_large_media(&mut self) -> bool {
        self.webpage.force_large_media = !self.webpage.force_large_media;
        self.webpage.force_large_media
    }

    /// Set reply quote text selection
    pub fn set_quote(&mut self, quote: impl Into<String>, offset: usize) {
        self.reply.quote_text = Some(quote.into());
        self.reply.quote_offset = offset;
    }

    /// Clear reply quote selection
    pub fn clear_quote(&mut self) {
        self.reply.quote_text = None;
        self.reply.quote_offset = 0;
    }

    /// Set active URL from available links
    pub fn select_link(&mut self, link: impl Into<String>) {
        let link_str = link.into();
        self.webpage.selected_link = Some(link_str);
        self.webpage.manual = true;
    }
}

impl Default for DraftOptionsFsm {
    fn default() -> Self {
        Self::new()
    }
}

/// GPUI View component for Draft Options Popup / Editor
pub struct DraftOptionsView {
    pub fsm: DraftOptionsFsm,
    pub is_open: bool,
}

impl DraftOptionsView {
    pub fn new() -> Self {
        Self {
            fsm: DraftOptionsFsm::new(),
            is_open: false,
        }
    }

    pub fn open(&mut self, section: DraftSection) {
        self.fsm.switch_section(section);
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }
}

impl Default for DraftOptionsView {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for DraftOptionsView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_open {
            return div().into_any_element();
        }

        let section = self.fsm.active_section;
        let forward_opt = self.fsm.forward;
        let invert_media = self.fsm.webpage.invert_media;
        let large_media = self.fsm.webpage.force_large_media;

        div()
            .flex()
            .flex_col()
            .gap_2()
            .p_3()
            .bg(rgb(0x1e2c3a))
            .border_1()
            .border_color(rgb(0x2a394a))
            .rounded_lg()
            .text_sm()
            // Section selector tabs
            .child(
                div()
                    .flex()
                    .gap_2()
                    .border_b_1()
                    .border_color(rgb(0x2a394a))
                    .pb_2()
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .rounded_sm()
                            .text_xs()
                            .font_weight(if section == DraftSection::Reply {
                                FontWeight::BOLD
                            } else {
                                FontWeight::NORMAL
                            })
                            .bg(if section == DraftSection::Reply {
                                rgb(0x2481cc)
                            } else {
                                rgb(0x18222d)
                            })
                            .text_color(rgb(0xffffff))
                            .child("Reply"),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .rounded_sm()
                            .text_xs()
                            .font_weight(if section == DraftSection::Forward {
                                FontWeight::BOLD
                            } else {
                                FontWeight::NORMAL
                            })
                            .bg(if section == DraftSection::Forward {
                                rgb(0x2481cc)
                            } else {
                                rgb(0x18222d)
                            })
                            .text_color(rgb(0xffffff))
                            .child("Forward"),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .rounded_sm()
                            .text_xs()
                            .font_weight(if section == DraftSection::Link {
                                FontWeight::BOLD
                            } else {
                                FontWeight::NORMAL
                            })
                            .bg(if section == DraftSection::Link {
                                rgb(0x2481cc)
                            } else {
                                rgb(0x18222d)
                            })
                            .text_color(rgb(0xffffff))
                            .child("Link Preview"),
                    ),
            )
            // Options body
            .child(match section {
                DraftSection::Reply => div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x8e8e93))
                            .child("Quote options"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xffffff))
                            .child(if let Some(ref q) = self.fsm.reply.quote_text {
                                format!("Selected quote: \"{}\"", q)
                            } else {
                                "Full message reply (no quote selected)".to_string()
                            }),
                    ),
                DraftSection::Forward => div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x8e8e93))
                            .child("Forward Privacy Setting:"),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .rounded_sm()
                            .bg(rgb(0x2481cc))
                            .text_color(rgb(0xffffff))
                            .text_xs()
                            .child(match forward_opt {
                                ForwardOptions::PreserveInfo => "Show sender name (Preserve info)",
                                ForwardOptions::NoNames => "Hide sender name (No names)",
                                ForwardOptions::NoNamesAndCaptions => {
                                    "Hide sender name and captions"
                                }
                            }),
                    ),
                DraftSection::Link => div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x8e8e93))
                            .child("Link Preview Position & Size:"),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .rounded_sm()
                                    .text_xs()
                                    .bg(if invert_media {
                                        rgb(0x2481cc)
                                    } else {
                                        rgb(0x2a394a)
                                    })
                                    .text_color(rgb(0xffffff))
                                    .child(if invert_media {
                                        "Media Above Text"
                                    } else {
                                        "Media Below Text"
                                    }),
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .rounded_sm()
                                    .text_xs()
                                    .bg(if large_media {
                                        rgb(0x2481cc)
                                    } else {
                                        rgb(0x2a394a)
                                    })
                                    .text_color(rgb(0xffffff))
                                    .child(if large_media {
                                        "Large Preview"
                                    } else {
                                        "Compact Preview"
                                    }),
                            ),
                    ),
            })
            .into_any_element()
    }
}
