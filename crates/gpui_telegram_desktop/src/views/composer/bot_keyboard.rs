//! BotKeyboard: Reply keyboard view component for bots.
//!
//! Upstream reference:
//! - `Telegram/SourceFiles/chat_helpers/bot_keyboard.h:L23-L69`
//! - `Telegram/SourceFiles/chat_helpers/bot_keyboard.cpp:L28-L450`
#![allow(dead_code)]

use gpui::*;

/// Reply keyboard markup flags (upstream: `bot_keyboard.cpp:L344-L354`).
pub mod flags {
    pub const SINGLE_USE: u32 = 0x01;
    pub const RESIZE: u32 = 0x02;
    pub const PERSISTENT: u32 = 0x04;
    pub const FORCE_REPLY: u32 = 0x08;
}

/// Bot keyboard button kind (upstream: `HistoryMessageMarkupButton::Type`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BotKeyboardButtonKind {
    Default,
    Callback(Vec<u8>),
    Url(String),
    RequestPhone,
    RequestLocation,
    RequestPoll,
    SwitchInline { query: String, same_peer: bool },
    WebApp(String),
    Buy,
}

/// Reply keyboard button item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BotKeyboardButton {
    pub text: String,
    pub kind: BotKeyboardButtonKind,
}

impl BotKeyboardButton {
    pub fn new(text: impl Into<String>, kind: BotKeyboardButtonKind) -> Self {
        Self {
            text: text.into(),
            kind,
        }
    }
}

/// Authentic BotKeyboard component state.
pub struct BotKeyboard {
    pub rows: Vec<Vec<BotKeyboardButton>>,
    pub placeholder: Option<String>,
    pub single_use: bool,
    pub persistent: bool,
    pub force_reply: bool,
    pub maximize_size: bool,
    pub max_outer_height: f32,
    pub selected_button: Option<(usize, usize)>,
    pub updated_at: i64,
}

impl BotKeyboard {
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            placeholder: None,
            single_use: false,
            persistent: false,
            force_reply: false,
            maximize_size: false,
            max_outer_height: 240.0,
            selected_button: None,
            updated_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn set_markup(
        &mut self,
        rows: Vec<Vec<BotKeyboardButton>>,
        flags_mask: u32,
        placeholder: Option<String>,
    ) {
        self.rows = rows;
        self.placeholder = placeholder;
        self.force_reply = (flags_mask & flags::FORCE_REPLY) != 0;
        self.maximize_size = (flags_mask & flags::RESIZE) == 0;
        self.single_use = (flags_mask & flags::SINGLE_USE) != 0;
        self.persistent = (flags_mask & flags::PERSISTENT) != 0;
        self.selected_button = None;
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn natural_height(&self) -> f32 {
        let row_height = 36.0;
        let spacing = 6.0;
        if self.rows.is_empty() {
            0.0
        } else {
            (self.rows.len() as f32) * row_height + ((self.rows.len() - 1) as f32) * spacing
        }
    }

    pub fn calculated_height(&self) -> f32 {
        let padding = 8.0; // deltat + deltab
        let h = padding + self.natural_height();
        if self.maximize_size {
            h.max(self.max_outer_height)
        } else {
            h.min(self.max_outer_height)
        }
    }

    pub fn has_markup(&self) -> bool {
        !self.rows.is_empty()
    }
}

impl Default for BotKeyboard {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for BotKeyboard {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.rows.is_empty() {
            return div();
        }

        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(6.0))
            .gap(px(6.0))
            .bg(rgb(0xEDEDED))
            .border_t_1()
            .border_color(rgb(0xD8D8D8))
            .children(self.rows.iter().enumerate().map(|(r_idx, row)| {
                div()
                    .flex()
                    .flex_row()
                    .w_full()
                    .gap(px(6.0))
                    .children(row.iter().enumerate().map(|(c_idx, btn)| {
                        let is_sel = self.selected_button == Some((r_idx, c_idx));
                        div()
                            .flex_1()
                            .h(px(36.0))
                            .rounded(px(6.0))
                            .bg(if is_sel { rgb(0xE0E0E0) } else { rgb(0xFFFFFF) })
                            .hover(|st| st.bg(rgb(0xFAFAFA)))
                            .border_1()
                            .border_color(rgb(0xDCDCDC))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(0x222222))
                                    .child(btn.text.clone())
                            )
                    }))
            }))
    }
}
