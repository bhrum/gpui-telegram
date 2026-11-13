//! History View Add Poll Option View Component (`history_view_add_poll_option`)
//!
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_add_poll_option.h` (L1-L34)
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_add_poll_option.cpp` (L1-L565)
//!
//! Handles in-history inline poll option adding overlay with emoji selector, attach media,
//! character limit clamping (100 chars), and submission event streams.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Maximum length of a poll option text.
/// Upstream: `history_view_add_poll_option.cpp:L159` (`_field->setMaxLength(100)`)
pub const K_POLL_OPTION_MAX_LENGTH: usize = 100;

/// Diameter of radio button in poll option rows.
/// Upstream: `st::historyPollRadio.diameter` (~22px)
pub const K_POLL_RADIO_DIAMETER: f32 = 22.0;

/// Width of emoji button in poll option input.
/// Upstream: `st::historyPollAddOptionEmoji.width` (~32px)
pub const K_POLL_ADD_OPTION_EMOJI_WIDTH: f32 = 32.0;

/// Standard button size in poll option input.
/// Upstream: `st::historyPollAddOptionButtonSize` (~28px)
pub const K_POLL_ADD_OPTION_BUTTON_SIZE: f32 = 28.0;

/// Calculates left overflow skip for emoji button aligning with poll radio indicator.
/// Upstream: `history_view_add_poll_option.cpp:L55-L60` (`EmojiOverflowLeft`)
#[inline]
pub fn calculate_emoji_overflow_left(button_width: f32, radio_diameter: f32) -> f32 {
    ((button_width - radio_diameter) / 2.0).max(0.0)
}

/// Media attachments for poll option.
/// Upstream: `history_view_add_poll_option.cpp:L102-L135` (`setupAttach`, `chooseLink`)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddPollOptionAttachment {
    Photo { file_name: String, size_bytes: u64 },
    Video { file_name: String, duration_sec: u32 },
    File { file_name: String, size_bytes: u64 },
    CustomEmoji { document_id: u64 },
    WebLink { url: String, title: String },
}

/// Status of the poll option input field.
/// Upstream: `history_view_add_poll_option.cpp:L143-L201` (`setupField`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddPollOptionFieldStatus {
    Editing,
    Validating,
    Submitting,
    Completed,
    Dismissed,
}

/// Authentic GPUI View for Add Poll Option overlay in Telegram chat history.
/// Upstream: `history_view_add_poll_option.h:L25-L33`, `history_view_add_poll_option.cpp:L62-L141`
#[derive(Debug, Clone)]
pub struct HistoryViewAddPollOptionView {
    pub poll_id: i64,
    pub message_id: i64,
    pub option_text: String,
    pub placeholder_text: String,
    pub max_characters: usize,
    pub field_status: AddPollOptionFieldStatus,
    pub attachment: Option<AddPollOptionAttachment>,
    pub is_emoji_panel_open: bool,
    pub is_active_field: bool,
}

impl HistoryViewAddPollOptionView {
    /// Creates a new add poll option overlay.
    /// Upstream: `history_view_add_poll_option.cpp:L112-L141`
    pub fn new(poll_id: i64, message_id: i64) -> Self {
        Self {
            poll_id,
            message_id,
            option_text: String::new(),
            placeholder_text: "Add an option...".to_string(),
            max_characters: K_POLL_OPTION_MAX_LENGTH,
            field_status: AddPollOptionFieldStatus::Editing,
            attachment: None,
            is_emoji_panel_open: false,
            is_active_field: true,
        }
    }

    /// Sets the text entered by user, clamping to max 100 UTF-8/code point characters.
    /// Upstream: `history_view_add_poll_option.cpp:L159`
    pub fn set_option_text(&mut self, text: impl Into<String>) {
        let t = text.into();
        if t.chars().count() > self.max_characters {
            self.option_text = t.chars().take(self.max_characters).collect();
        } else {
            self.option_text = t;
        }
    }

    /// Appends an emoji at cursor position.
    /// Upstream: `history_view_add_poll_option.cpp:L243-L246`
    pub fn insert_emoji(&mut self, emoji: &str) {
        if self.option_text.chars().count() + emoji.chars().count() <= self.max_characters {
            self.option_text.push_str(emoji);
        }
    }

    /// Toggles the emoji picker panel.
    /// Upstream: `history_view_add_poll_option.cpp:L224-L241`
    pub fn toggle_emoji_panel(&mut self) {
        self.is_emoji_panel_open = !self.is_emoji_panel_open;
    }

    /// Attaches an external link or media to the poll option.
    /// Upstream: `history_view_add_poll_option.cpp:L84-L88`
    pub fn set_attachment(&mut self, attach: AddPollOptionAttachment) {
        self.attachment = Some(attach);
    }

    /// Clears any current media attachment.
    /// Upstream: `history_view_add_poll_option.cpp:L88`
    pub fn clear_attachment(&mut self) {
        self.attachment = None;
    }

    /// Checks whether the option can be submitted (non-empty trimmed text within limit).
    /// Upstream: `history_view_add_poll_option.cpp:L181-L184`
    pub fn can_submit(&self) -> bool {
        let trimmed = self.option_text.trim();
        !trimmed.is_empty() && trimmed.chars().count() <= self.max_characters
    }

    /// Submits the new option. Returns true if valid.
    pub fn trigger_submit(&mut self) -> bool {
        if self.can_submit() {
            self.field_status = AddPollOptionFieldStatus::Submitting;
            true
        } else {
            false
        }
    }

    /// Cancels or closes the overlay.
    /// Upstream: `history_view_add_poll_option.cpp:L186-L198`
    pub fn cancel(&mut self) {
        if self.is_emoji_panel_open {
            self.is_emoji_panel_open = false;
        } else {
            self.field_status = AddPollOptionFieldStatus::Dismissed;
            self.is_active_field = false;
        }
    }

    /// Returns horizontal layout metrics for the emoji button.
    pub fn overflow_left(&self) -> f32 {
        calculate_emoji_overflow_left(K_POLL_ADD_OPTION_EMOJI_WIDTH, K_POLL_RADIO_DIAMETER)
    }

    /// Remaining character allowance.
    pub fn remaining_characters(&self) -> usize {
        self.max_characters.saturating_sub(self.option_text.chars().count())
    }
}

impl Default for HistoryViewAddPollOptionView {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Render for HistoryViewAddPollOptionView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_submittable = self.can_submit();
        let chars_left = self.remaining_characters();

        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(8.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(6.0))
            .border_1()
            .border_color(if self.is_active_field { rgb(0x2AABEE) } else { rgb(0xE0E0E0) })
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    // Radio button placeholder
                    .child(
                        div()
                            .w(px(K_POLL_RADIO_DIAMETER))
                            .h(px(K_POLL_RADIO_DIAMETER))
                            .rounded_full()
                            .border_2()
                            .border_color(rgb(0xCCCCCC))
                    )
                    // Input preview
                    .child(
                        div()
                            .flex_1()
                            .text_size(px(14.0))
                            .text_color(if self.option_text.is_empty() { rgb(0x999999) } else { rgb(0x222222) })
                            .child(if self.option_text.is_empty() {
                                self.placeholder_text.clone()
                            } else {
                                self.option_text.clone()
                            })
                    )
                    // Emoji button
                    .child(
                        div()
                            .w(px(K_POLL_ADD_OPTION_BUTTON_SIZE))
                            .h(px(K_POLL_ADD_OPTION_BUTTON_SIZE))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(16.0))
                            .child("😊")
                    )
            )
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .pt(px(4.0))
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(if chars_left == 0 { rgb(0xEE3333) } else { rgb(0x888888) })
                            .child(format!("{}/{}", self.option_text.chars().count(), self.max_characters))
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(if is_submittable { rgb(0x2AABEE) } else { rgb(0xCCCCCC) })
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Add Option")
                    )
            )
    }
}
