//! ShareMessagePhraseFactory: Toast message generation for message forwarding and sharing.
//!
//! Upstream reference:
//! - `Telegram/SourceFiles/chat_helpers/share_message_phrase_factory.h:L20-L37`
//! - `Telegram/SourceFiles/chat_helpers/share_message_phrase_factory.cpp:L20-L82`
#![allow(dead_code)]

use gpui::*;

/// Arguments for forwarding phrase generation (upstream: `ForwardedMessagePhraseArgs`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForwardedPhraseArgs {
    pub to_count: usize,
    pub single_message: bool,
    pub to1_name: Option<String>,
    pub to1_is_self: bool,
    pub to2_name: Option<String>,
    pub to_self_with_premium_is_empty: bool,
}

/// Generate localized confirmation toast phrase (upstream: `share_message_phrase_factory.cpp:L20-L61`).
pub fn format_forwarded_phrase(args: &ForwardedPhraseArgs) -> String {
    if args.to_count <= 1 {
        if args.to1_is_self {
            if args.to_self_with_premium_is_empty {
                return String::new();
            }
            if args.single_message {
                "Message forwarded to Saved Messages".to_string()
            } else {
                "Messages forwarded to Saved Messages".to_string()
            }
        } else {
            let name = args.to1_name.as_deref().unwrap_or("chat");
            if args.single_message {
                format!("Message forwarded to {}", name)
            } else {
                format!("Messages forwarded to {}", name)
            }
        }
    } else if args.to_count == 2 {
        let name1 = args.to1_name.as_deref().unwrap_or("user");
        let name2 = args.to2_name.as_deref().unwrap_or("chat");
        if args.single_message {
            format!("Message forwarded to {} and {}", name1, name2)
        } else {
            format!("Messages forwarded to {} and {}", name1, name2)
        }
    } else if args.single_message {
        format!("Message forwarded to {} chats", args.to_count)
    } else {
        format!("Messages forwarded to {} chats", args.to_count)
    }
}

/// Determine icon asset key for toast (upstream: `share_message_phrase_factory.cpp:L63-L71`).
pub fn forwarded_phrase_icon(args: &ForwardedPhraseArgs) -> &'static str {
    if args.to_count <= 1 && args.to1_is_self {
        "toast/saved_messages"
    } else {
        "toast/forward"
    }
}

/// Authentic ShareMessagePhraseFactory state.
pub struct ShareMessagePhraseFactory {
    pub last_phrase: String,
    pub last_icon: &'static str,
    pub updated_at: i64,
}

impl ShareMessagePhraseFactory {
    pub fn new() -> Self {
        Self {
            last_phrase: String::new(),
            last_icon: "toast/forward",
            updated_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn generate(&mut self, args: &ForwardedPhraseArgs) -> String {
        let phrase = format_forwarded_phrase(args);
        self.last_phrase = phrase.clone();
        self.last_icon = forwarded_phrase_icon(args);
        self.updated_at = chrono::Utc::now().timestamp();
        phrase
    }
}

impl Default for ShareMessagePhraseFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for ShareMessagePhraseFactory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .p(px(8.0))
            .bg(rgb(0x333333))
            .rounded(px(6.0))
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0xFFFFFF))
                    .child(if self.last_phrase.is_empty() {
                        "No phrase generated".to_string()
                    } else {
                        self.last_phrase.clone()
                    })
            )
    }
}
