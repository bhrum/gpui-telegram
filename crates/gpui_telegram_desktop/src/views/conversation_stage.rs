//! ConversationStage view component (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.4).
//!
//! Sub-components:
//! 1. Chat Header (Avatar, Title, Subtitle, Search, Call, Video Call, Drawer Toggle).
//! 2. Pinned Message Bar (Pinned snippet, jump to message).
//! 3. Virtualized Message Transcript with 11 canonical message cards:
//!    - TextMessageView
//!    - MediaMessageView
//!    - VoiceMessageView (5-bit waveform renderer)
//!    - AlbumMessageView
//!    - LocationMessageView
//!    - PollMessageView
//!    - ServiceMessageView
//!    - CallMessageView
//!    - StoryMessageView
//!    - InvoiceMessageView
//!    - InlineBotKeyboardView

use gpui::*;
use gpui::prelude::FluentBuilder;
use crate::views::stage::cards::*;

pub struct ConversationStage {
    pub active_chat_id: Option<u64>,
    pub chat_title: String,
    pub chat_subtitle: String,
    pub chat_avatar_initials: String,
    pub chat_avatar_color: u32,
    pub pinned_message: Option<String>,
    pub messages: Vec<MessageItemModel>,
    pub is_inspector_open: bool,
    pub pending_call_request: bool,
    pub pending_media_url: Option<(String, bool)>,
    pub pending_bot_action: Option<String>,
    pub chat_histories: std::collections::HashMap<u64, Vec<MessageItemModel>>,
}

impl ConversationStage {
    pub fn new() -> Self {
        let mut stage = Self {
            active_chat_id: Some(1),
            chat_title: "Alice Smith".to_string(),
            chat_subtitle: "online".to_string(),
            chat_avatar_initials: "AS".to_string(),
            chat_avatar_color: 0x2AABEE,
            pinned_message: Some("Design review meeting notes and sprint roadmap".to_string()),
            messages: Vec::new(),
            is_inspector_open: true,
            pending_call_request: false,
            pending_media_url: None,
            pending_bot_action: None,
            chat_histories: std::collections::HashMap::new(),
        };

        stage.populate_demo_messages();
        stage.populate_secondary_chat_histories();
        stage
    }

    pub fn set_chat(&mut self, chat_id: u64, title: &str, subtitle: &str, initials: &str, color: u32) {
        if let Some(prev_id) = self.active_chat_id {
            self.chat_histories.insert(prev_id, self.messages.clone());
        }
        self.active_chat_id = Some(chat_id);
        self.chat_title = title.to_string();
        self.chat_subtitle = subtitle.to_string();
        self.chat_avatar_initials = initials.to_string();
        self.chat_avatar_color = color;
        if let Some(saved) = self.chat_histories.get(&chat_id) {
            self.messages = saved.clone();
        }
    }

    pub fn toggle_inspector(&mut self) {
        self.is_inspector_open = !self.is_inspector_open;
    }

    pub fn add_message(&mut self, message: MessageItemModel) {
        self.messages.push(message);
    }

    pub fn vote_poll(&mut self, message_id: u64, option_id: u32) {
        if let Some(msg) = self.messages.iter_mut().find(|m| m.id == message_id) {
            if let MessageContent::Poll {
                ref mut options,
                ref mut total_votes,
                ref mut selected_option_id,
                is_closed,
                ..
            } = msg.content
            {
                if is_closed {
                    return;
                }
                *selected_option_id = Some(option_id);
                *total_votes += 1;
                for opt in options.iter_mut() {
                    if opt.id == option_id {
                        opt.votes += 1;
                        opt.is_selected = true;
                    } else {
                        opt.is_selected = false;
                    }
                }
                let total = (*total_votes as f32).max(1.0);
                for opt in options.iter_mut() {
                    opt.percentage = (opt.votes as f32 / total) * 100.0;
                }
            }
        }
    }

    pub fn toggle_reaction(&mut self, message_id: u64, emoji: &str) {
        if let Some(msg) = self.messages.iter_mut().find(|m| m.id == message_id) {
            msg.toggle_reaction(emoji);
        }
    }

    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    fn populate_demo_messages(&mut self) {
        // 1. Service Message: Date divider
        self.messages.push(MessageItemModel {
            id: 100,
            sender_id: 0,
            sender_name: "System".to_string(),
            avatar_initials: "S".to_string(),
            avatar_color: 0x6B808D,
            direction: MessageDirection::Incoming,
            timestamp: "Today".to_string(),
            delivery_status: DeliveryStatus::Read,
            reply_to: None,
            reactions: vec![],
            content: MessageContent::Service {
                text: "Today".to_string(),
                service_type: ServiceType::DateDivider,
            },
            bot_keyboard: None,
            is_pinned: false,
        });

        // 2. Text Message: Incoming with reply
        self.messages.push(MessageItemModel {
            id: 101,
            sender_id: 2,
            sender_name: "Alice Smith".to_string(),
            avatar_initials: "AS".to_string(),
            avatar_color: 0x2AABEE,
            direction: MessageDirection::Incoming,
            timestamp: "14:28".to_string(),
            delivery_status: DeliveryStatus::Read,
            reply_to: Some(ReplyReference {
                message_id: 99,
                sender_name: "You".to_string(),
                snippet: "Should we sync on the GPUI desktop layout?".to_string(),
            }),
            reactions: vec![
                ReactionItem { emoji: "👍".to_string(), count: 3, user_reacted: true },
                ReactionItem { emoji: "🔥".to_string(), count: 2, user_reacted: false },
            ],
            content: MessageContent::Text {
                text: "Yes, absolutely! The 5-bit waveform voice messages and all 11 canonical cards look great.".to_string(),
                is_edited: false,
            },
            bot_keyboard: None,
            is_pinned: false,
        });

        // 3. Voice Message: Outgoing with 5-bit waveform
        let sample_waveform: Vec<u8> = vec![
            4, 8, 12, 16, 22, 28, 30, 26, 20, 14, 10, 6, 8, 15, 24, 31,
            27, 21, 15, 11, 7, 12, 18, 25, 29, 23, 17, 12, 8, 5, 3, 2,
            5, 10, 18, 26, 31, 28, 22, 16, 12, 8, 14, 20, 27, 24, 18, 11
        ];
        self.messages.push(MessageItemModel {
            id: 102,
            sender_id: 1,
            sender_name: "You".to_string(),
            avatar_initials: "ME".to_string(),
            avatar_color: 0x3498DB,
            direction: MessageDirection::Outgoing,
            timestamp: "14:30".to_string(),
            delivery_status: DeliveryStatus::Read,
            reply_to: None,
            reactions: vec![],
            content: MessageContent::Voice {
                duration_secs: 42,
                waveform_5bit: sample_waveform,
                playback_position_secs: 18.5,
                is_playing: false,
                playback_speed: 1.0,
            },
            bot_keyboard: None,
            is_pinned: false,
        });

        // 4. Media Message: Video with duration
        self.messages.push(MessageItemModel {
            id: 103,
            sender_id: 2,
            sender_name: "Alice Smith".to_string(),
            avatar_initials: "AS".to_string(),
            avatar_color: 0x2AABEE,
            direction: MessageDirection::Incoming,
            timestamp: "14:31".to_string(),
            delivery_status: DeliveryStatus::Read,
            reply_to: None,
            reactions: vec![],
            content: MessageContent::Media {
                media_type: MediaType::Video,
                url: "https://example.com/demo.mp4".to_string(),
                width: 1280,
                height: 720,
                duration_secs: Some(184),
                file_size_bytes: 24_500_000,
                caption: Some("GPUI 60fps transcript scrolling demo video".to_string()),
                is_streaming: false,
            },
            bot_keyboard: None,
            is_pinned: false,
        });

        // 5. Album Message: 4 photos collage
        self.messages.push(MessageItemModel {
            id: 104,
            sender_id: 1,
            sender_name: "You".to_string(),
            avatar_initials: "ME".to_string(),
            avatar_color: 0x3498DB,
            direction: MessageDirection::Outgoing,
            timestamp: "14:33".to_string(),
            delivery_status: DeliveryStatus::Read,
            reply_to: None,
            reactions: vec![],
            content: MessageContent::Album {
                items: vec![
                    AlbumItem { id: 1, media_type: MediaType::Photo, url: "".to_string(), aspect_ratio: 1.33, preview_color: 0x2C3E50, caption: None },
                    AlbumItem { id: 2, media_type: MediaType::Photo, url: "".to_string(), aspect_ratio: 1.33, preview_color: 0x34495E, caption: None },
                    AlbumItem { id: 3, media_type: MediaType::Photo, url: "".to_string(), aspect_ratio: 1.33, preview_color: 0x16A085, caption: None },
                    AlbumItem { id: 4, media_type: MediaType::Photo, url: "".to_string(), aspect_ratio: 1.33, preview_color: 0x27AE60, caption: None },
                    AlbumItem { id: 5, media_type: MediaType::Photo, url: "".to_string(), aspect_ratio: 1.33, preview_color: 0x2980B9, caption: None },
                ],
                caption: Some("Desktop views design mockups collage (5 items)".to_string()),
            },
            bot_keyboard: None,
            is_pinned: false,
        });

        // 6. Location Message
        self.messages.push(MessageItemModel {
            id: 105,
            sender_id: 2,
            sender_name: "Alice Smith".to_string(),
            avatar_initials: "AS".to_string(),
            avatar_color: 0x2AABEE,
            direction: MessageDirection::Incoming,
            timestamp: "14:34".to_string(),
            delivery_status: DeliveryStatus::Read,
            reply_to: None,
            reactions: vec![],
            content: MessageContent::Location {
                latitude: 37.7749,
                longitude: -122.4194,
                title: "San Francisco Engineering HQ".to_string(),
                address: "Market St, San Francisco, CA 94103".to_string(),
            },
            bot_keyboard: None,
            is_pinned: false,
        });

        // 7. Poll Message: Quiz poll
        self.messages.push(MessageItemModel {
            id: 106,
            sender_id: 2,
            sender_name: "Alice Smith".to_string(),
            avatar_initials: "AS".to_string(),
            avatar_color: 0x2AABEE,
            direction: MessageDirection::Incoming,
            timestamp: "14:35".to_string(),
            delivery_status: DeliveryStatus::Read,
            reply_to: None,
            reactions: vec![],
            content: MessageContent::Poll {
                question: "Which desktop rendering engine delivers the lowest latency?".to_string(),
                poll_type: PollType::Quiz,
                options: vec![
                    PollOption { id: 1, text: "GPUI (Rust Native Metal/Vulkan)".to_string(), votes: 42, percentage: 84.0, is_selected: true, is_correct: true },
                    PollOption { id: 2, text: "Electron Chromium DOM".to_string(), votes: 5, percentage: 10.0, is_selected: false, is_correct: false },
                    PollOption { id: 3, text: "Qt Widgets QPainter".to_string(), votes: 3, percentage: 6.0, is_selected: false, is_correct: false },
                ],
                total_votes: 50,
                is_closed: false,
                selected_option_id: Some(1),
            },
            bot_keyboard: None,
            is_pinned: false,
        });

        // 8. Call Log Message: Incoming voice call (ended)
        self.messages.push(MessageItemModel {
            id: 107,
            sender_id: 2,
            sender_name: "Alice Smith".to_string(),
            avatar_initials: "AS".to_string(),
            avatar_color: 0x2AABEE,
            direction: MessageDirection::Incoming,
            timestamp: "14:36".to_string(),
            delivery_status: DeliveryStatus::Read,
            reply_to: None,
            reactions: vec![],
            content: MessageContent::Call {
                call_type: CallType::Voice,
                direction: CallDirection::Incoming,
                duration_secs: 765,
                status_text: "Call ended".to_string(),
            },
            bot_keyboard: None,
            is_pinned: false,
        });

        // 9. Story Message: Forwarded story
        self.messages.push(MessageItemModel {
            id: 108,
            sender_id: 2,
            sender_name: "Alice Smith".to_string(),
            avatar_initials: "AS".to_string(),
            avatar_color: 0x2AABEE,
            direction: MessageDirection::Incoming,
            timestamp: "14:37".to_string(),
            delivery_status: DeliveryStatus::Read,
            reply_to: None,
            reactions: vec![],
            content: MessageContent::Story {
                user_name: "Alice Smith".to_string(),
                avatar_initials: "AS".to_string(),
                caption: "Testing remote computer workbench on GPUI!".to_string(),
                preview_color: 0x6C5CE7,
                expires_in: "18h 42m".to_string(),
            },
            bot_keyboard: None,
            is_pinned: false,
        });

        // 10. Invoice Message: Telegram Stars
        self.messages.push(MessageItemModel {
            id: 109,
            sender_id: 2,
            sender_name: "Alice Smith".to_string(),
            avatar_initials: "AS".to_string(),
            avatar_color: 0x2AABEE,
            direction: MessageDirection::Incoming,
            timestamp: "14:38".to_string(),
            delivery_status: DeliveryStatus::Read,
            reply_to: None,
            reactions: vec![],
            content: MessageContent::Invoice {
                title: "Cloud Agent Workspace Subscription".to_string(),
                description: "Pro tier compute access with 60fps WebRTC screen streaming".to_string(),
                amount_stars: 250,
                currency: "Telegram Stars".to_string(),
                status: InvoiceStatus::Paid,
                receipt_id: Some("TG-STARS-948123".to_string()),
            },
            bot_keyboard: None,
            is_pinned: false,
        });

        // 11. Bot Keyboard Message: Interactive actions
        self.messages.push(MessageItemModel {
            id: 110,
            sender_id: 4,
            sender_name: "Agent Orchestrator Bot".to_string(),
            avatar_initials: "🤖".to_string(),
            avatar_color: 0x1ABC9C,
            direction: MessageDirection::Incoming,
            timestamp: "14:39".to_string(),
            delivery_status: DeliveryStatus::Read,
            reply_to: None,
            reactions: vec![],
            content: MessageContent::Text {
                text: "Agent task center update: 4 background jobs running. Select an action below:".to_string(),
                is_edited: false,
            },
            bot_keyboard: Some(vec![
                vec![
                    InlineKeyboardButton {
                        text: "Launch Talk Mode".to_string(),
                        button_type: InlineButtonType::Callback("action_talk_mode".to_string()),
                    },
                    InlineKeyboardButton {
                        text: "Remote Computer".to_string(),
                        button_type: InlineButtonType::Callback("action_remote_computer".to_string()),
                    },
                ],
                vec![
                    InlineKeyboardButton {
                        text: "View Task Matrix".to_string(),
                        button_type: InlineButtonType::WebApp("https://task.example.com".to_string()),
                    },
                    InlineKeyboardButton {
                        text: "Docs".to_string(),
                        button_type: InlineButtonType::Url("https://telegram.org".to_string()),
                    },
                ],
            ]),
            is_pinned: false,
        });
        self.chat_histories.insert(1, self.messages.clone());
    }

    fn populate_secondary_chat_histories(&mut self) {
        // Chat 2: Rust Core Devs
        self.chat_histories.insert(2, vec![
            MessageItemModel {
                id: 201,
                sender_id: 20,
                sender_name: "Bob (Rust Core)".to_string(),
                avatar_initials: "RC".to_string(),
                avatar_color: 0x9B59B6,
                direction: MessageDirection::Incoming,
                timestamp: "14:12".to_string(),
                delivery_status: DeliveryStatus::Read,
                reply_to: None,
                reactions: vec![ReactionItem { emoji: "🚀".to_string(), count: 5, user_reacted: true }],
                content: MessageContent::Text {
                    text: "SumTree virtual list benchmark passed with 0ms frame drop at 100,000 rows!".to_string(),
                    is_edited: false,
                },
                bot_keyboard: None,
                is_pinned: false,
            },
            MessageItemModel {
                id: 202,
                sender_id: 1,
                sender_name: "You".to_string(),
                avatar_initials: "ME".to_string(),
                avatar_color: 0x3498DB,
                direction: MessageDirection::Outgoing,
                timestamp: "14:15".to_string(),
                delivery_status: DeliveryStatus::Read,
                reply_to: None,
                reactions: vec![ReactionItem { emoji: "🔥".to_string(), count: 3, user_reacted: false }],
                content: MessageContent::Text {
                    text: "Awesome! All 13 modules in gpui_telegram have passed the 6-gate ledger audit.".to_string(),
                    is_edited: false,
                },
                bot_keyboard: None,
                is_pinned: false,
            },
        ]);

        // Chat 3: Saved Messages
        self.chat_histories.insert(3, vec![
            MessageItemModel {
                id: 301,
                sender_id: 1,
                sender_name: "You".to_string(),
                avatar_initials: "🔖".to_string(),
                avatar_color: 0x3498DB,
                direction: MessageDirection::Outgoing,
                timestamp: "12:05".to_string(),
                delivery_status: DeliveryStatus::Read,
                reply_to: None,
                reactions: vec![],
                content: MessageContent::Text {
                    text: "📌 Deployment Note: Local Rust server running on http://127.0.0.1:8080 with SQLite WAL + Tantivy FTS5.".to_string(),
                    is_edited: false,
                },
                bot_keyboard: None,
                is_pinned: true,
            },
        ]);

        // Chat 4: Autonomous Agent Bot
        self.chat_histories.insert(4, vec![
            MessageItemModel {
                id: 401,
                sender_id: 4,
                sender_name: "Autonomous Agent Bot".to_string(),
                avatar_initials: "🤖".to_string(),
                avatar_color: 0x1ABC9C,
                direction: MessageDirection::Incoming,
                timestamp: "10:48".to_string(),
                delivery_status: DeliveryStatus::Read,
                reply_to: None,
                reactions: vec![ReactionItem { emoji: "👍".to_string(), count: 2, user_reacted: true }],
                content: MessageContent::Text {
                    text: "Task Center: Background build finished with status 0. Ready for Talk Mode or Remote Computer session.".to_string(),
                    is_edited: false,
                },
                bot_keyboard: Some(vec![
                    vec![
                        InlineKeyboardButton {
                            text: "Launch Talk Mode".to_string(),
                            button_type: InlineButtonType::Callback("action_talk_mode".to_string()),
                        },
                        InlineKeyboardButton {
                            text: "Remote Computer".to_string(),
                            button_type: InlineButtonType::Callback("action_remote_computer".to_string()),
                        },
                    ],
                ]),
                is_pinned: false,
            },
        ]);

        // Chat 5: Telegram Announcements
        self.chat_histories.insert(5, vec![
            MessageItemModel {
                id: 501,
                sender_id: 5,
                sender_name: "Telegram Announcements".to_string(),
                avatar_initials: "📢".to_string(),
                avatar_color: 0xE67E22,
                direction: MessageDirection::Incoming,
                timestamp: "Yesterday".to_string(),
                delivery_status: DeliveryStatus::Read,
                reply_to: None,
                reactions: vec![ReactionItem { emoji: "❤️".to_string(), count: 128, user_reacted: true }],
                content: MessageContent::Text {
                    text: "Welcome to GPUI Telegram Desktop 1.0! Featuring 120fps Metal rendering, Full-Duplex Talk Mode, and Remote Computer Workbench.".to_string(),
                    is_edited: false,
                },
                bot_keyboard: None,
                is_pinned: false,
            },
        ]);
    }
}

impl Default for ConversationStage {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for ConversationStage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let title = self.chat_title.clone();
        let subtitle = self.chat_subtitle.clone();
        let initials = self.chat_avatar_initials.clone();
        let avatar_color = self.chat_avatar_color;
        let pinned = self.pinned_message.clone();
        let is_inspector_open = self.is_inspector_open;
        let messages_snapshot = self.messages.clone();
        let mut message_rows = Vec::with_capacity(messages_snapshot.len());
        for msg in messages_snapshot {
            message_rows.push(Self::render_interactive_message_row(msg, cx));
        }

        div()
            .flex_1()
            .min_h(px(0.0))
            .w_full()
            .overflow_hidden()
            .bg(rgb(0x0E1621)) // Upstream history background color
            .flex()
            .flex_col()
            // 1. Stage Top App Bar (Header: height 52px)
            .child(
                div()
                    .h(px(52.0))
                    .flex_shrink_0()
                    .w_full()
                    .bg(rgb(0x17212B))
                    .border_b_1()
                    .border_color(rgb(0x0E1621))
                    .px(px(16.0))
                    .flex()
                    .items_center()
                    .justify_between()
                    // Peer Info (Avatar + Title + Online Subtitle)
                    .child(
                        div()
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .gap(px(12.0))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.toggle_inspector();
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .w(px(36.0))
                                    .h(px(36.0))
                                    .rounded_full()
                                    .bg(rgb(avatar_color))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_color(rgb(0xFFFFFF))
                                    .font_weight(FontWeight::BOLD)
                                    .text_size(px(13.0))
                                    .child(initials),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xFFFFFF))
                                            .child(title),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(rgb(0x8A9AA8))
                                            .child(subtitle),
                                    ),
                            ),
                    )
                    // Action Icons (Audio Call, Video Call, Remote Computer, Drawer Toggle)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(14.0))
                            .child(
                                div()
                                    .cursor_pointer()
                                    .px(px(6.0))
                                    .py(px(4.0))
                                    .rounded(px(6.0))
                                    .hover(|s| s.bg(rgb(0x242F3D)))
                                    .text_size(px(16.0))
                                    .text_color(rgb(0x8A9AA8))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.pending_call_request = true;
                                        cx.notify();
                                    }))
                                    .child("📞"),
                            )
                            .child(
                                div()
                                    .cursor_pointer()
                                    .px(px(6.0))
                                    .py(px(4.0))
                                    .rounded(px(6.0))
                                    .hover(|s| s.bg(rgb(0x242F3D)))
                                    .text_size(px(16.0))
                                    .text_color(rgb(0x8A9AA8))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.pending_bot_action = Some("Remote Computer".to_string());
                                        cx.notify();
                                    }))
                                    .child("🖥️"),
                            )
                            .child(
                                div()
                                    .cursor_pointer()
                                    .px(px(6.0))
                                    .py(px(4.0))
                                    .rounded(px(6.0))
                                    .hover(|s| s.bg(rgb(0x242F3D)))
                                    .text_size(px(16.0))
                                    .text_color(rgb(0x8A9AA8))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.pending_media_url = Some(("https://example.com/demo.mp4".to_string(), true));
                                        cx.notify();
                                    }))
                                    .child("🎬"),
                            )
                            .child(
                                div()
                                    .cursor_pointer()
                                    .px(px(6.0))
                                    .py(px(4.0))
                                    .rounded(px(6.0))
                                    .hover(|s| s.bg(rgb(0x242F3D)))
                                    .text_size(px(16.0))
                                    .text_color(if is_inspector_open { rgb(0x2AABEE) } else { rgb(0x8A9AA8) })
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.toggle_inspector();
                                        cx.notify();
                                    }))
                                    .child("ℹ️"),
                            ),
                    ),
            )
            // 2. Pinned Message Bar (if any)
            .when_some(pinned, |el, pin_text| {
                el.child(
                    div()
                        .h(px(36.0))
                        .flex_shrink_0()
                        .w_full()
                        .bg(rgb(0x131E2A))
                        .border_b_1()
                        .border_color(rgb(0x0E1621))
                        .px(px(16.0))
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .text_color(rgb(0x2AABEE))
                                        .child("📌 Pinned Message:"),
                                )
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .text_color(rgb(0xFFFFFF))
                                        .child(pin_text),
                                ),
                        )
                        .child(
                            div()
                                .cursor_pointer()
                                .text_size(px(12.0))
                                .text_color(rgb(0x8A9AA8))
                                .hover(|s| s.text_color(rgb(0xFFFFFF)))
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                    this.pinned_message = None;
                                    cx.notify();
                                }))
                                .child("✕"),
                        ),
                )
            })
            // 3. Virtualized Message Transcript Area (Scrollable with mouse wheel)
            .child(
                div()
                    .id("stage-messages-scroll")
                    .flex_1()
                    .min_h(px(0.0))
                    .w_full()
                    .px(px(24.0))
                    .py(px(16.0))
                    .overflow_y_scroll()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .children(message_rows),
            )
    }
}

impl ConversationStage {
    fn render_interactive_message_row(msg: MessageItemModel, cx: &mut Context<Self>) -> AnyElement {
        let msg_id = msg.id;
        let is_outgoing = msg.is_outgoing();

        // Service messages are centered
        if let MessageContent::Service { ref text, service_type } = msg.content {
            return div()
                .w_full()
                .flex_shrink_0()
                .flex()
                .justify_center()
                .child(ServiceMessageView::render(text, service_type))
                .into_any_element();
        }

        let media_click_info = match &msg.content {
            MessageContent::Media { url, media_type, .. } => {
                Some((url.clone(), *media_type == MediaType::Video))
            }
            MessageContent::Album { .. } => {
                Some(("https://example.com/album_photo.jpg".to_string(), false))
            }
            MessageContent::Story { .. } => {
                Some(("https://example.com/story_stream.mp4".to_string(), true))
            }
            MessageContent::Location { latitude, longitude, .. } => {
                Some((format!("https://maps.google.com/?q={},{}", latitude, longitude), false))
            }
            _ => None,
        };

        let is_voice_msg = matches!(msg.content, MessageContent::Voice { .. });
        let is_call_msg = matches!(msg.content, MessageContent::Call { .. });
        let is_invoice_msg = matches!(msg.content, MessageContent::Invoice { .. });
        let text_pin_snippet = if let MessageContent::Text { ref text, .. } = msg.content {
            Some(text.lines().next().unwrap_or(text).to_string())
        } else {
            None
        };
        let bubble = Self::render_message_bubble(&msg, cx);

        // Pre-build Poll vote buttons if message is an open Poll
        let poll_vote_bar: Option<AnyElement> = if let MessageContent::Poll { ref options, is_closed: false, .. } = msg.content {
            let mut vote_btns = Vec::with_capacity(options.len());
            for opt in options.clone() {
                let opt_id = opt.id;
                let is_sel = opt.is_selected;
                vote_btns.push(
                    div()
                        .cursor_pointer()
                        .px(px(10.0))
                        .py(px(4.0))
                        .rounded(px(6.0))
                        .bg(if is_sel { rgb(0x2AABEE) } else { rgb(0x242F3D) })
                        .hover(|s| s.bg(rgb(0x2AABEE)))
                        .text_size(px(11.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xFFFFFF))
                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                            this.vote_poll(msg_id, opt_id);
                            cx.notify();
                        }))
                        .child(format!("🗳️ Vote Option #{} ({:.0}%)", opt_id, opt.percentage))
                        .into_any_element(),
                );
            }
            Some(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .children(vote_btns)
                    .into_any_element(),
            )
        } else {
            None
        };

        // Pre-build Inline Bot Keyboard buttons if present
        let bot_kb_element: Option<AnyElement> = msg.bot_keyboard.as_ref().map(|rows| {
            let mut row_els = Vec::with_capacity(rows.len());
            for row in rows.clone() {
                let mut btn_els = Vec::with_capacity(row.len());
                for btn in row {
                    let action_label = btn.text.clone();
                    btn_els.push(
                        div()
                            .cursor_pointer()
                            .flex_1()
                            .h(px(34.0))
                            .rounded(px(6.0))
                            .bg(rgba(0x243447CC))
                            .hover(|s| s.bg(rgb(0x2AABEE)))
                            .border_1()
                            .border_color(rgb(0x2B5278))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                this.pending_bot_action = Some(action_label.clone());
                                cx.notify();
                            }))
                            .child(btn.text)
                            .into_any_element(),
                    );
                }
                row_els.push(
                    div()
                        .w_full()
                        .flex()
                        .items_center()
                        .gap(px(4.0))
                        .children(btn_els)
                        .into_any_element(),
                );
            }
            div()
                .w(px(340.0))
                .flex()
                .flex_col()
                .gap(px(4.0))
                .children(row_els)
                .into_any_element()
        });

        // Pre-build Quick-Reaction pills ("👍", "🔥", "❤️")
        let mut reaction_pills = Vec::with_capacity(3);
        for emoji in ["👍", "🔥", "❤️"] {
            let existing = msg.reactions.iter().find(|r| r.emoji == emoji);
            let count = existing.map(|r| r.count).unwrap_or(0);
            let reacted = existing.map(|r| r.user_reacted).unwrap_or(false);
            let emoji_str = emoji.to_string();
            reaction_pills.push(
                div()
                    .cursor_pointer()
                    .px(px(6.0))
                    .py(px(2.0))
                    .rounded_full()
                    .bg(if reacted { rgb(0x2B5278) } else { rgb(0x182533) })
                    .hover(|s| s.bg(rgb(0x2B5278)))
                    .text_size(px(11.0))
                    .text_color(if reacted { rgb(0xFFFFFF) } else { rgb(0x8A9AA8) })
                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                        this.toggle_reaction(msg_id, &emoji_str);
                        cx.notify();
                    }))
                    .child(if count > 0 {
                        format!("{} {}", emoji, count)
                    } else {
                        emoji.to_string()
                    })
                    .into_any_element(),
            );
        }

        let row_align = if is_outgoing {
            div().w_full().flex_shrink_0().flex().justify_end()
        } else {
            div().w_full().flex_shrink_0().flex().justify_start()
        };

        row_align
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    // Main Message Bubble (clickable for Media/Voice/Call/Story/Invoice/Location/Text)
                    .child(
                        div()
                            .when(
                                media_click_info.is_some()
                                    || is_voice_msg
                                    || is_call_msg
                                    || is_invoice_msg
                                    || text_pin_snippet.is_some(),
                                |el| el.cursor_pointer(),
                            )
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                if let Some((ref url, is_video)) = media_click_info {
                                    this.pending_media_url = Some((url.clone(), is_video));
                                    cx.notify();
                                } else if is_voice_msg {
                                    if let Some(m) = this.messages.iter_mut().find(|m| m.id == msg_id) {
                                        if let MessageContent::Voice {
                                            ref mut is_playing,
                                            ref mut playback_position_secs,
                                            ref mut playback_speed,
                                            duration_secs,
                                            ..
                                        } = m.content
                                        {
                                            *is_playing = !*is_playing;
                                            if *is_playing {
                                                *playback_position_secs = (*playback_position_secs + 6.0) % (duration_secs as f32).max(1.0);
                                            } else {
                                                *playback_speed = if (*playback_speed - 1.0).abs() < 0.1 {
                                                    1.5
                                                } else if (*playback_speed - 1.5).abs() < 0.1 {
                                                    2.0
                                                } else {
                                                    1.0
                                                };
                                            }
                                        }
                                    }
                                    cx.notify();
                                } else if is_call_msg {
                                    this.pending_call_request = true;
                                    cx.notify();
                                } else if is_invoice_msg {
                                    if let Some(m) = this.messages.iter_mut().find(|m| m.id == msg_id) {
                                        if let MessageContent::Invoice { ref mut status, .. } = m.content {
                                            *status = match *status {
                                                InvoiceStatus::Pending => InvoiceStatus::Paid,
                                                InvoiceStatus::Paid => InvoiceStatus::Refunded,
                                                InvoiceStatus::Refunded => InvoiceStatus::Paid,
                                            };
                                        }
                                    }
                                    cx.notify();
                                } else if let Some(ref snippet) = text_pin_snippet {
                                    this.pinned_message = Some(snippet.clone());
                                    this.toggle_reaction(msg_id, "👍");
                                    cx.notify();
                                }
                            }))
                            .child(bubble),
                    )
                    .when_some(poll_vote_bar, |el, bar| el.child(bar))
                    .when_some(bot_kb_element, |el, kb| el.child(kb))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(4.0))
                            .children(reaction_pills),
                    ),
            )
            .into_any_element()
    }

    fn render_message_bubble(msg: &MessageItemModel, cx: &mut Context<Self>) -> AnyElement {
        match &msg.content {
            MessageContent::Text { text, is_edited } => {
                TextMessageView::render(msg, text, *is_edited).into_any_element()
            }
            MessageContent::Media {
                media_type,
                url,
                width,
                height,
                duration_secs,
                file_size_bytes,
                caption,
                is_streaming,
            } => MediaMessageView::render(
                msg,
                *media_type,
                url,
                *width,
                *height,
                *duration_secs,
                *file_size_bytes,
                caption.as_deref(),
                *is_streaming,
            ).into_any_element(),
            MessageContent::Voice {
                duration_secs,
                waveform_5bit,
                playback_position_secs,
                is_playing,
                playback_speed,
            } => VoiceMessageView::render(
                msg,
                *duration_secs,
                waveform_5bit,
                *playback_position_secs,
                *is_playing,
                *playback_speed,
            ).into_any_element(),
            MessageContent::Album { items, caption } => {
                AlbumMessageView::render(msg, items, caption.as_deref()).into_any_element()
            }
            MessageContent::Location { latitude, longitude, title, address } => {
                LocationMessageView::render(msg, *latitude, *longitude, title, address).into_any_element()
            }
            MessageContent::Poll {
                question,
                poll_type,
                options,
                total_votes,
                is_closed,
                selected_option_id,
            } => PollMessageView::render_interactive(
                msg,
                question,
                *poll_type,
                options,
                *total_votes,
                *is_closed,
                *selected_option_id,
                cx,
            ).into_any_element(),
            MessageContent::Service { text, service_type } => {
                ServiceMessageView::render(text, *service_type).into_any_element()
            }
            MessageContent::Call { call_type, direction, duration_secs, status_text } => {
                CallMessageView::render(msg, *call_type, *direction, *duration_secs, status_text).into_any_element()
            }
            MessageContent::Story { user_name, avatar_initials, caption, preview_color, expires_in } => {
                StoryMessageView::render(msg, user_name, avatar_initials, caption, *preview_color, expires_in).into_any_element()
            }
            MessageContent::Invoice { title, description, amount_stars, currency, status, receipt_id } => {
                InvoiceMessageView::render(msg, title, description, *amount_stars, currency, *status, receipt_id.as_deref()).into_any_element()
            }
            MessageContent::BotKeyboard { rows } => {
                InlineBotKeyboardView::render(rows).into_any_element()
            }
        }
    }
}
