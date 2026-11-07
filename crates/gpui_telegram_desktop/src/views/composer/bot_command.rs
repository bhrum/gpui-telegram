//! BotCommand: Command formatting and dispatch for chat bots.
//!
//! Upstream reference:
//! - `Telegram/SourceFiles/chat_helpers/bot_command.h:L15-L30`
//! - `Telegram/SourceFiles/chat_helpers/bot_command.cpp:L19-L47`
#![allow(dead_code)]

use gpui::*;

/// Outgoing bot command execution request (upstream: `Bot::SendCommandRequest`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendCommandRequest {
    pub peer_id: u64,
    pub command: String,
    pub context_msg_id: Option<i32>,
    pub reply_to_msg_id: Option<i32>,
}

/// Wrap command with target bot username if in a group with multiple bots
/// (upstream: `Bot::WrapCommandInChat`, `bot_command.cpp:L32-L47`).
pub fn wrap_command_in_chat(command: &str, bot_username: &str, has_bots: bool) -> String {
    if bot_username.is_empty() || !has_bots {
        return command.to_string();
    }
    let has_at = command.find('@').map_or(false, |pos| pos >= 2);
    if !has_at {
        format!("{}@{}", command, bot_username)
    } else {
        command.to_string()
    }
}

/// Registered bot command entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BotCommandEntry {
    pub command: String,
    pub description: String,
    pub bot_username: String,
}

/// Authentic BotCommand component state.
pub struct BotCommand {
    pub active_commands: Vec<BotCommandEntry>,
    pub filter_query: String,
    pub selected_index: usize,
    pub updated_at: i64,
}

impl BotCommand {
    pub fn new() -> Self {
        Self {
            active_commands: Vec::new(),
            filter_query: String::new(),
            selected_index: 0,
            updated_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn register_command(
        &mut self,
        command: impl Into<String>,
        description: impl Into<String>,
        bot_username: impl Into<String>,
    ) {
        self.active_commands.push(BotCommandEntry {
            command: command.into(),
            description: description.into(),
            bot_username: bot_username.into(),
        });
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn set_filter(&mut self, query: impl Into<String>) {
        self.filter_query = query.into();
        self.selected_index = 0;
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn filtered_commands(&self) -> Vec<&BotCommandEntry> {
        let q = self.filter_query.trim_start_matches('/');
        self.active_commands
            .iter()
            .filter(|c| q.is_empty() || c.command.to_lowercase().contains(&q.to_lowercase()))
            .collect()
    }

    pub fn select_next(&mut self) {
        let count = self.filtered_commands().len();
        if count > 0 {
            self.selected_index = (self.selected_index + 1) % count;
        }
    }

    pub fn select_prev(&mut self) {
        let count = self.filtered_commands().len();
        if count > 0 {
            self.selected_index = if self.selected_index == 0 {
                count - 1
            } else {
                self.selected_index - 1
            };
        }
    }
}

impl Default for BotCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for BotCommand {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let matches = self.filtered_commands();

        div()
            .flex()
            .flex_col()
            .w_full()
            .bg(rgb(0xFFFFFF))
            .p(px(6.0))
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .rounded(px(6.0))
            .children(matches.into_iter().enumerate().map(|(idx, entry)| {
                let is_sel = idx == self.selected_index;
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .p(px(4.0))
                    .bg(if is_sel { rgb(0xEBF4FB) } else { rgb(0xFFFFFF) })
                    .rounded(px(4.0))
                    .child(
                        div()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x2AABEE))
                            .child(format!("/{}", entry.command))
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(0x666666))
                            .child(entry.description.clone())
                    )
            }))
    }
}
