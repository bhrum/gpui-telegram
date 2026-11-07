//! Master Compose Controls Component for Telegram chat message authoring.
//! Upstream reference: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_compose_controls.cpp`
//! Upstream header: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_compose_controls.h`

use super::ai_button::ComposeAiButton;
use super::draft_options::{DraftOptionsFsm, DraftSection};
use super::forward_panel::{ForwardPanelModel, ForwardedItem};
use super::voice_record_bar::{VoiceRecordBar, VoiceRecordState};
use gpui::*;
use serde::{Deserialize, Serialize};

/// Mathematical unicode minus sign
/// Upstream: `constexpr auto kMinus = QChar(0x2212);` (`history_view_characters_limit.cpp:L15`)
pub const UNICODE_MINUS: char = '\u{2212}';

/// Over-limit counter threshold cap
/// Upstream: `constexpr auto kLimit = int(999);` (`history_view_characters_limit.cpp:L16`)
pub const CHARACTERS_OVER_LIMIT_CAP: i32 = 999;

/// Standard text message character limit (4,096 characters)
pub const STANDARD_MESSAGE_CHARACTERS_LIMIT: usize = 4096;

/// Media caption character limit (1,024 standard, 2,048 for Premium)
pub const STANDARD_CAPTION_CHARACTERS_LIMIT: usize = 1024;

/// Compose controls operating mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComposeControlsMode {
    Normal,
    Scheduled,
}

impl Default for ComposeControlsMode {
    fn default() -> Self {
        Self::Normal
    }
}

/// Send button active glyph / trigger mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SendButtonType {
    Send,
    RecordVoice,
    RecordRound,
    Schedule,
    SaveEdit,
}

impl Default for SendButtonType {
    fn default() -> Self {
        Self::RecordVoice
    }
}

/// Send progress notification type sent over MTProto
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SendProgressType {
    Typing,
    RecordVideo,
    UploadVideo,
    RecordVoice,
    UploadVoice,
    RecordRound,
    UploadRound,
    ChooseSticker,
    Cancel,
}

/// Write restriction level imposed on the user in the current chat
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WriteRestrictionType {
    None,
    Rights,
    PremiumRequired,
    Frozen,
    Hidden,
}

impl Default for WriteRestrictionType {
    fn default() -> Self {
        Self::None
    }
}

/// Write restriction payload descriptor
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteRestriction {
    pub restriction_type: WriteRestrictionType,
    pub text: String,
    pub button: String,
    pub boosts_to_lift: i32,
}

impl Default for WriteRestriction {
    fn default() -> Self {
        Self {
            restriction_type: WriteRestrictionType::None,
            text: String::new(),
            button: String::new(),
            boosts_to_lift: 0,
        }
    }
}

impl WriteRestriction {
    pub fn is_restricted(&self) -> bool {
        self.restriction_type != WriteRestrictionType::None
    }
}

/// Character limitation status helper
/// Upstream: `CharactersLimitLabel::setLeft` (`history_view_characters_limit.cpp:L53-69`)
pub fn format_character_limit(current_len: usize, max_len: usize) -> (String, bool) {
    let diff = max_len as i64 - current_len as i64;
    if diff >= 0 {
        // Within limit, display remaining characters
        (diff.to_string(), false)
    } else {
        // Exceeded limit: prefix with unicode minus and cap at 999
        let exceeded = (-diff).min(CHARACTERS_OVER_LIMIT_CAP as i64);
        (format!("{}{}", UNICODE_MINUS, exceeded), true)
    }
}

/// Core domain state for the compose controls subsystem
pub struct ComposeControlsModel {
    pub text: String,
    pub mode: ComposeControlsMode,
    pub send_button: SendButtonType,
    pub restriction: WriteRestriction,
    pub slowmode_seconds_left: i32,
    pub reply_msg_id: Option<u64>,
    pub reply_preview: Option<String>,
    pub edit_msg_id: Option<u64>,
    pub webpage_url: Option<String>,
    pub voice_bar: VoiceRecordBar,
    pub draft_options: DraftOptionsFsm,
    pub forward_panel: ForwardPanelModel,
    pub ai_button: ComposeAiButton,
    pub max_characters: usize,
}

impl ComposeControlsModel {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            mode: ComposeControlsMode::Normal,
            send_button: SendButtonType::RecordVoice,
            restriction: WriteRestriction::default(),
            slowmode_seconds_left: 0,
            reply_msg_id: None,
            reply_preview: None,
            edit_msg_id: None,
            webpage_url: None,
            voice_bar: VoiceRecordBar::new(),
            draft_options: DraftOptionsFsm::new(),
            forward_panel: ForwardPanelModel::new(),
            ai_button: ComposeAiButton::new(),
            max_characters: STANDARD_MESSAGE_CHARACTERS_LIMIT,
        }
    }

    /// Update input text and recalculate send button type
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
        self.update_send_button_type();
    }

    /// Automatically select send button glyph:
    /// - If editing: `SaveEdit`
    /// - If scheduled mode: `Schedule`
    /// - If text or forwards or attachments are present: `Send`
    /// - Else: `RecordVoice`
    pub fn update_send_button_type(&mut self) {
        if self.edit_msg_id.is_some() {
            self.send_button = SendButtonType::SaveEdit;
        } else if self.mode == ComposeControlsMode::Scheduled {
            self.send_button = SendButtonType::Schedule;
        } else if !self.text.is_empty() || !self.forward_panel.is_empty() {
            self.send_button = SendButtonType::Send;
        } else {
            self.send_button = SendButtonType::RecordVoice;
        }
    }

    /// Set reply target
    pub fn set_reply(&mut self, msg_id: u64, preview: impl Into<String>) {
        self.reply_msg_id = Some(msg_id);
        self.reply_preview = Some(preview.into());
    }

    /// Clear reply target
    pub fn clear_reply(&mut self) {
        self.reply_msg_id = None;
        self.reply_preview = None;
    }

    /// Set message editing target
    pub fn set_edit_message(&mut self, msg_id: u64, original_text: impl Into<String>) {
        self.edit_msg_id = Some(msg_id);
        self.set_text(original_text);
    }

    /// Cancel editing mode
    pub fn cancel_edit(&mut self) {
        self.edit_msg_id = None;
        self.set_text("");
    }

    /// Character count warning information: returns (label, is_exceeded)
    pub fn character_limit_info(&self) -> (String, bool) {
        format_character_limit(self.text.len(), self.max_characters)
    }
}

impl Default for ComposeControlsModel {
    fn default() -> Self {
        Self::new()
    }
}

/// Authentic GPUI Compose Controls View Component
pub struct ComposeControlsView {
    pub model: ComposeControlsModel,
}

impl ComposeControlsView {
    pub fn new() -> Self {
        Self {
            model: ComposeControlsModel::new(),
        }
    }
}

impl Default for ComposeControlsView {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for ComposeControlsView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_restricted = self.model.restriction.is_restricted();
        let is_voice_active = self.model.voice_bar.is_active();
        let (limit_text, is_limit_exceeded) = self.model.character_limit_info();
        let show_limit_warning = self.model.text.len() > self.model.max_characters.saturating_sub(100);
        let send_btn_text = match self.model.send_button {
            SendButtonType::Send => "Send",
            SendButtonType::RecordVoice => "Mic",
            SendButtonType::RecordRound => "Video",
            SendButtonType::Schedule => "Schedule",
            SendButtonType::SaveEdit => "Save",
        };

        let mut root = div()
            .flex()
            .flex_col()
            .w_full()
            .bg(rgb(0x18222d))
            .border_t_1()
            .border_color(rgb(0x2a394a));

        // Top auxiliary banner: Forward panel or Reply preview
        if let Some(ref preview) = self.model.reply_preview {
            let preview_text = preview.clone();
            root = root.child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_3()
                    .py_1()
                    .bg(rgb(0x1e2c3a))
                    .border_l_2()
                    .border_color(rgb(0x2481cc))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x2481cc))
                                    .child("Reply to message"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x8e8e93))
                                    .child(preview_text),
                            ),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x8e8e93))
                            .child("x"),
                    ),
            );
        }

        // Restricted chat banner (if sender has no rights or channel is frozen)
        if is_restricted {
            let text = self.model.restriction.text.clone();
            root = root.child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .py_3()
                    .bg(rgb(0x1c1e22))
                    .text_xs()
                    .text_color(rgb(0xff5252))
                    .child(text),
            );
        } else if is_voice_active {
            // Voice record bar overlay (when recording)
            let dur = self.model.voice_bar.formatted_duration();
            root = root.child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_3()
                    .py_2()
                    .bg(rgb(0x18222d))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .w(px(10.0))
                                    .h(px(10.0))
                                    .rounded_full()
                                    .bg(rgb(0xff3b30)),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0xffffff))
                                    .font_weight(FontWeight::BOLD)
                                    .child(dur),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x8e8e93))
                                    .child("< Slide left to cancel | ^ Swipe up to lock"),
                            ),
                    ),
            );
        } else {
            // Main composer input bar
            let mut bar = div()
                .flex()
                .items_center()
                .gap_2()
                .px_3()
                .py_2()
                // Attachment clip button
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(0x8e8e93))
                        .hover(|s| s.text_color(rgb(0x2481cc)))
                        .child("📎"),
                )
                // Input text display field
                .child(
                    div()
                        .flex_1()
                        .px_2()
                        .py_1()
                        .rounded_md()
                        .bg(rgb(0x242f3d))
                        .text_sm()
                        .text_color(if self.model.text.is_empty() {
                            rgb(0x8e8e93)
                        } else {
                            rgb(0xffffff)
                        })
                        .child(if self.model.text.is_empty() {
                            "Write a message...".to_string()
                        } else {
                            self.model.text.clone()
                        }),
                );

            if show_limit_warning {
                bar = bar.child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(if is_limit_exceeded {
                            rgb(0xff3b30) // Red error
                        } else {
                            rgb(0x8e8e93) // Subtext
                        })
                        .child(limit_text),
                );
            }

            bar = bar
                // AI compose sparkle button
                .child(
                    div()
                        .px_2()
                        .py_1()
                        .rounded_md()
                        .bg(rgb(0x2a394a))
                        .text_xs()
                        .text_color(rgb(0xffffff))
                        .child("AI *"),
                )
                // Send or Mic button
                .child(
                    div()
                        .px_3()
                        .py_1()
                        .rounded_md()
                        .bg(rgb(0x2481cc))
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xffffff))
                        .child(send_btn_text),
                );

            root = root.child(bar);
        }

        root
    }
}
