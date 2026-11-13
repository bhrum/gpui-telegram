//! ConversationStage view component (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.4).
//!
//! Full 1:1 Telegram Desktop (`tdesktop`) Central History Stage:
//! - Top Bar: Peer Profile, Topic `#` Breadcrumb, `🔍` Search-in-Chat, `📞` Audio Call,
//!   `📹` Video/Group Call, `📅` Scheduled Queue, `ℹ️` Info Drawer, `⋮` More Menu
//! - Top Contextual Bars:
//!   1. Floating Audio/Voice Player Bar (`▶/❚❚`, Title, Scrub, `1x/1.5x/2x` Speed, `[->A]` Transcribe)
//!   2. Group Voice Chat Live Banner (`🔊 Live Voice Chat • [Join]`)
//!   3. Real-Time Chat Translation Banner (`🌐 Translate Entire Chat`)
//!   4. Pinned Message Bar + Multi-Select Action Toolbar (`Forward / Delete / Copy`)
//!   5. In-Chat Search Filter Bar (`ChatSearchIn`)
//! - 11 Canonical Message Cards + Hover/Right-Click Message Action Strip
//!   (`Reply`, `Edit`, `Forward`, `Pin`, `Copy`, `Translate`, `Select`, `Delete`, `[->A] Transcribe`, `⚡ Instant View`, `⭐ Paid Star`)

use crate::views::stage::cards::*;
use crate::views::tdesktop_theme::TdesktopThemePalette;
use gpui::prelude::FluentBuilder;
use gpui::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct ActiveAudioTrack {
    pub message_id: u64,
    pub sender_or_title: String,
    pub duration_secs: u32,
    pub position_secs: f32,
    pub speed: f32,
    pub is_playing: bool,
}

#[derive(Debug, Clone)]
pub enum StageActionRequest {
    ReplyToMessage {
        message_id: u64,
        sender_name: String,
        snippet: String,
    },
    EditMessage {
        message_id: u64,
        current_text: String,
    },
    ForwardMessages(Vec<u64>),
    DeleteMessages(Vec<u64>),
    TranslateMessage(String),
    CopyText(String),
    OpenInstantView {
        title: String,
        url: String,
    },
    OpenCreatePoll,
    OpenExportChat,
    PaidStarReaction(u64),
}

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
    pub chat_histories: HashMap<u64, Vec<MessageItemModel>>,

    // 1:1 Telegram Desktop Extended Interactive State
    pub active_topic_badge: Option<String>,
    pub is_search_bar_open: bool,
    pub in_chat_search_query: String,
    pub is_more_menu_open: bool,
    pub show_translate_banner: bool,
    pub is_chat_translated: bool,
    pub show_group_call_banner: bool,
    pub active_audio_player: Option<ActiveAudioTrack>,
    pub transcribed_voice_ids: HashSet<u64>,
    pub revealed_spoiler_ids: HashSet<u64>,
    pub selected_message_ids: Vec<u64>,
    pub active_action_menu_msg_id: Option<u64>,
    pub show_scheduled_section: bool,
    pub scheduled_messages: HashMap<u64, Vec<(u64, String, String)>>,
    pub pending_stage_action: Option<StageActionRequest>,
    pub palette: TdesktopThemePalette,
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
            chat_histories: HashMap::new(),
            active_topic_badge: None,
            is_search_bar_open: false,
            in_chat_search_query: String::new(),
            is_more_menu_open: false,
            show_translate_banner: true,
            is_chat_translated: false,
            show_group_call_banner: false,
            active_audio_player: None,
            transcribed_voice_ids: HashSet::new(),
            revealed_spoiler_ids: HashSet::new(),
            selected_message_ids: Vec::new(),
            active_action_menu_msg_id: None,
            show_scheduled_section: false,
            scheduled_messages: HashMap::new(),
            pending_stage_action: None,
            palette: TdesktopThemePalette::default(),
        };

        stage.populate_demo_messages();
        stage.populate_secondary_chat_histories();
        stage
    }

    pub fn set_chat(
        &mut self,
        chat_id: u64,
        title: &str,
        subtitle: &str,
        initials: &str,
        color: u32,
    ) {
        if let Some(prev_id) = self.active_chat_id {
            self.chat_histories.insert(prev_id, self.messages.clone());
        }
        self.active_chat_id = Some(chat_id);
        self.chat_title = title.to_string();
        self.chat_subtitle = subtitle.to_string();
        self.chat_avatar_initials = initials.to_string();
        self.chat_avatar_color = color;
        self.selected_message_ids.clear();
        self.active_action_menu_msg_id = None;
        self.is_more_menu_open = false;
        self.show_scheduled_section = false;
        self.show_group_call_banner = chat_id == 2;
        if chat_id != 2 {
            self.active_topic_badge = None;
        }
        if let Some(saved) = self.chat_histories.get(&chat_id) {
            self.messages = saved.clone();
        } else {
            self.messages = vec![MessageItemModel {
                id: chat_id * 1000 + 1,
                sender_id: 0,
                sender_name: "System".to_string(),
                avatar_initials: "S".to_string(),
                avatar_color: color,
                direction: MessageDirection::Incoming,
                timestamp: "Today".to_string(),
                delivery_status: DeliveryStatus::Read,
                reply_to: None,
                reactions: vec![],
                content: MessageContent::Service {
                    text: format!("Conversation created: {}", title),
                    service_type: ServiceType::GroupCreated,
                },
                bot_keyboard: None,
                is_pinned: false,
            }];
        }
    }

    pub fn toggle_inspector(&mut self) {
        self.is_inspector_open = !self.is_inspector_open;
    }

    pub fn add_message(&mut self, message: MessageItemModel) {
        self.messages.push(message);
        if let Some(cid) = self.active_chat_id {
            self.chat_histories.insert(cid, self.messages.clone());
        }
    }

    pub fn edit_message_text(&mut self, message_id: u64, new_text: String) {
        if let Some(msg) = self.messages.iter_mut().find(|m| m.id == message_id) {
            if let MessageContent::Text {
                ref mut text,
                ref mut is_edited,
            } = msg.content
            {
                *text = new_text;
                *is_edited = true;
            }
        }
        if let Some(cid) = self.active_chat_id {
            self.chat_histories.insert(cid, self.messages.clone());
        }
    }

    pub fn delete_messages_by_ids(&mut self, ids: &[u64]) {
        self.messages.retain(|m| !ids.contains(&m.id));
        self.selected_message_ids.retain(|id| !ids.contains(id));
        if let Some(cid) = self.active_chat_id {
            self.chat_histories.insert(cid, self.messages.clone());
        }
    }

    pub fn toggle_select_message(&mut self, msg_id: u64) {
        if let Some(pos) = self.selected_message_ids.iter().position(|&x| x == msg_id) {
            self.selected_message_ids.remove(pos);
        } else {
            self.selected_message_ids.push(msg_id);
        }
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

    pub fn extract_message_snippet(msg: &MessageItemModel) -> String {
        match &msg.content {
            MessageContent::Text { text, .. } => text.lines().next().unwrap_or(text).to_string(),
            MessageContent::Media { caption, .. } => caption
                .clone()
                .unwrap_or_else(|| "[Photo / Video Media]".to_string()),
            MessageContent::Voice { duration_secs, .. } => {
                format!("🎙️ Voice message ({}s)", duration_secs)
            }
            MessageContent::Album { caption, items } => caption
                .clone()
                .unwrap_or_else(|| format!("🖼️ Media Album ({} items)", items.len())),
            MessageContent::Location { title, .. } => format!("📍 {}", title),
            MessageContent::Poll { question, .. } => format!("📊 Poll: {}", question),
            MessageContent::Invoice { title, amount_stars, .. } => {
                format!("⭐ {} ({} Stars)", title, amount_stars)
            }
            MessageContent::Story { user_name, .. } => format!("⭕ Story from {}", user_name),
            MessageContent::Call { status_text, .. } => format!("📞 {}", status_text),
            MessageContent::Service { text, .. } => text.clone(),
            MessageContent::BotKeyboard { .. } => "[Bot Keyboard]".to_string(),
        }
    }

    fn populate_demo_messages(&mut self) {
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
                ReactionItem {
                    emoji: "👍".to_string(),
                    count: 3,
                    user_reacted: true,
                },
                ReactionItem {
                    emoji: "🔥".to_string(),
                    count: 2,
                    user_reacted: false,
                },
            ],
            content: MessageContent::Text {
                text: "Yes, absolutely! Check https://github.com/telegramdesktop/tdesktop — the 5-bit waveform voice messages and ||120fps Metal shaders|| look great.".to_string(),
                is_edited: false,
            },
            bot_keyboard: None,
            is_pinned: false,
        });

        let sample_waveform: Vec<u8> = vec![
            4, 8, 12, 16, 22, 28, 30, 26, 20, 14, 10, 6, 8, 15, 24, 31, 27, 21, 15, 11, 7, 12, 18,
            25, 29, 23, 17, 12, 8, 5, 3, 2, 5, 10, 18, 26, 31, 28, 22, 16, 12, 8, 14, 20, 27, 24,
            18, 11,
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
                    AlbumItem {
                        id: 1,
                        media_type: MediaType::Photo,
                        url: "".to_string(),
                        aspect_ratio: 1.33,
                        preview_color: 0x2C3E50,
                        caption: None,
                    },
                    AlbumItem {
                        id: 2,
                        media_type: MediaType::Photo,
                        url: "".to_string(),
                        aspect_ratio: 1.33,
                        preview_color: 0x34495E,
                        caption: None,
                    },
                    AlbumItem {
                        id: 3,
                        media_type: MediaType::Photo,
                        url: "".to_string(),
                        aspect_ratio: 1.33,
                        preview_color: 0x16A085,
                        caption: None,
                    },
                    AlbumItem {
                        id: 4,
                        media_type: MediaType::Photo,
                        url: "".to_string(),
                        aspect_ratio: 1.33,
                        preview_color: 0x27AE60,
                        caption: None,
                    },
                    AlbumItem {
                        id: 5,
                        media_type: MediaType::Photo,
                        url: "".to_string(),
                        aspect_ratio: 1.33,
                        preview_color: 0x2980B9,
                        caption: None,
                    },
                ],
                caption: Some("Desktop views design mockups collage (5 items)".to_string()),
            },
            bot_keyboard: None,
            is_pinned: false,
        });

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
                    PollOption {
                        id: 1,
                        text: "GPUI (Rust Native Metal/Vulkan)".to_string(),
                        votes: 42,
                        percentage: 84.0,
                        is_selected: true,
                        is_correct: true,
                    },
                    PollOption {
                        id: 2,
                        text: "Electron Chromium DOM".to_string(),
                        votes: 5,
                        percentage: 10.0,
                        is_selected: false,
                        is_correct: false,
                    },
                    PollOption {
                        id: 3,
                        text: "Qt Widgets QPainter".to_string(),
                        votes: 3,
                        percentage: 6.0,
                        is_selected: false,
                        is_correct: false,
                    },
                ],
                total_votes: 50,
                is_closed: false,
                selected_option_id: Some(1),
            },
            bot_keyboard: None,
            is_pinned: false,
        });

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
                description: "Pro tier compute access with 60fps WebRTC screen streaming"
                    .to_string(),
                amount_stars: 250,
                currency: "Telegram Stars".to_string(),
                status: InvoiceStatus::Paid,
                receipt_id: Some("TG-STARS-948123".to_string()),
            },
            bot_keyboard: None,
            is_pinned: false,
        });

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
                text: "Agent task center update: 4 background jobs running. Select an action below:"
                    .to_string(),
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
                        button_type: InlineButtonType::Callback(
                            "action_remote_computer".to_string(),
                        ),
                    },
                ],
                vec![
                    InlineKeyboardButton {
                        text: "View Task Matrix".to_string(),
                        button_type: InlineButtonType::WebApp(
                            "https://task.example.com".to_string(),
                        ),
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
        self.chat_histories.insert(
            2,
            vec![
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
                    reactions: vec![ReactionItem {
                        emoji: "🚀".to_string(),
                        count: 5,
                        user_reacted: true,
                    }],
                    content: MessageContent::Text {
                        text: "> Supergroup Forum Topic\n```rust\npub fn render_gpui_frame() { /* 0ms drop */ }\n```\nSumTree virtual list benchmark passed at 100,000 rows!".to_string(),
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
                    reactions: vec![ReactionItem {
                        emoji: "🔥".to_string(),
                        count: 3,
                        user_reacted: false,
                    }],
                    content: MessageContent::Text {
                        text: "Awesome! All 28 subsystems in gpui_telegram have full 1:1 UI/UX parity.".to_string(),
                        is_edited: false,
                    },
                    bot_keyboard: None,
                    is_pinned: false,
                },
            ],
        );

        self.chat_histories.insert(
            3,
            vec![MessageItemModel {
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
                    text: "📌 #notes Local Rust server on http://127.0.0.1:8080 with SQLite WAL + Tantivy FTS5.".to_string(),
                    is_edited: false,
                },
                bot_keyboard: None,
                is_pinned: true,
            }],
        );

        self.chat_histories.insert(
            4,
            vec![MessageItemModel {
                id: 401,
                sender_id: 4,
                sender_name: "Autonomous Agent Bot".to_string(),
                avatar_initials: "🤖".to_string(),
                avatar_color: 0x1ABC9C,
                direction: MessageDirection::Incoming,
                timestamp: "10:48".to_string(),
                delivery_status: DeliveryStatus::Read,
                reply_to: None,
                reactions: vec![ReactionItem {
                    emoji: "👍".to_string(),
                    count: 2,
                    user_reacted: true,
                }],
                content: MessageContent::Text {
                    text: "Task Center: Background build finished with status 0. Ready for Talk Mode or Remote Computer session.".to_string(),
                    is_edited: false,
                },
                bot_keyboard: Some(vec![vec![
                    InlineKeyboardButton {
                        text: "Launch Talk Mode".to_string(),
                        button_type: InlineButtonType::Callback("action_talk_mode".to_string()),
                    },
                    InlineKeyboardButton {
                        text: "Remote Computer".to_string(),
                        button_type: InlineButtonType::Callback("action_remote_computer".to_string()),
                    },
                ]]),
                is_pinned: false,
            }],
        );

        self.chat_histories.insert(
            5,
            vec![MessageItemModel {
                id: 501,
                sender_id: 5,
                sender_name: "Telegram Announcements".to_string(),
                avatar_initials: "📢".to_string(),
                avatar_color: 0xE67E22,
                direction: MessageDirection::Incoming,
                timestamp: "Yesterday".to_string(),
                delivery_status: DeliveryStatus::Read,
                reply_to: None,
                reactions: vec![ReactionItem {
                    emoji: "❤️".to_string(),
                    count: 128,
                    user_reacted: true,
                }],
                content: MessageContent::Text {
                    text: "Welcome to GPUI Telegram Desktop 7.4.2! Featuring 120fps Metal rendering, Instant View, Forum Topics, TON Wallet, and E2EE Calls.".to_string(),
                    is_edited: false,
                },
                bot_keyboard: None,
                is_pinned: false,
            }],
        );
    }
}

impl Default for ConversationStage {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for ConversationStage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;
        let title = self.chat_title.clone();
        let subtitle = if let Some(ref topic) = self.active_topic_badge {
            format!("# {} • {}", topic, self.chat_subtitle)
        } else {
            self.chat_subtitle.clone()
        };
        let initials = self.chat_avatar_initials.clone();
        let avatar_color = self.chat_avatar_color;
        let pinned = self.pinned_message.clone();
        let is_inspector_open = self.is_inspector_open;
        let search_open = self.is_search_bar_open;
        let search_q = self.in_chat_search_query.clone();
        let more_open = self.is_more_menu_open;
        let show_tr_bar = self.show_translate_banner;
        let is_translated = self.is_chat_translated;
        let show_gc_bar = self.show_group_call_banner;
        let audio_track = self.active_audio_player.clone();
        let sel_ids = self.selected_message_ids.clone();
        let show_sched = self.show_scheduled_section;
        let cid = self.active_chat_id.unwrap_or(1);
        let sched_list = self
            .scheduled_messages
            .get(&cid)
            .cloned()
            .unwrap_or_default();

        let q_low = search_q.to_lowercase();
        let filtered_messages: Vec<MessageItemModel> = self
            .messages
            .iter()
            .filter(|m| {
                if q_low.is_empty() {
                    true
                } else {
                    Self::extract_message_snippet(m)
                        .to_lowercase()
                        .contains(&q_low)
                }
            })
            .cloned()
            .collect();
        let match_count = filtered_messages.len();

        let mut message_rows = Vec::with_capacity(filtered_messages.len());
        for msg in filtered_messages {
            message_rows.push(self.render_interactive_message_row(msg, cx));
        }

        div()
            .relative()
            .flex_1()
            .min_h(px(0.0))
            .w_full()
            .overflow_hidden()
            .bg(rgb(pal.history_bg))
            .flex()
            .flex_col()
            // 1. Stage Top App Bar (52px)
            .child(
                div()
                    .h(px(52.0))
                    .flex_shrink_0()
                    .w_full()
                    .bg(rgb(pal.top_bar_bg))
                    .border_b_1()
                    .border_color(rgb(pal.border_fg))
                    .px(px(16.0))
                    .flex()
                    .items_center()
                    .justify_between()
                    // Left: Peer Avatar + Title + Online/Topic Subtitle
                    .child(
                        div()
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .gap(px(12.0))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _event, _window, cx| {
                                    this.toggle_inspector();
                                    cx.notify();
                                }),
                            )
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
                                            .text_color(rgb(pal.text_primary))
                                            .child(title),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(rgb(pal.text_secondary))
                                            .child(subtitle),
                                    ),
                            ),
                    )
                    // Right Action Icons: Search-in-Chat, Scheduled, Voice Call, Video Call, Drawer, More
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            // Scheduled Queue Badge Button
                            .child(
                                div()
                                    .cursor_pointer()
                                    .px(px(6.0))
                                    .py(px(4.0))
                                    .rounded(px(6.0))
                                    .bg(if show_sched {
                                        rgb(pal.accent)
                                    } else {
                                        rgba(0x00000000)
                                    })
                                    .hover(move |s| s.bg(rgb(pal.input_bg)))
                                    .text_size(px(13.0))
                                    .text_color(if show_sched {
                                        rgb(0xFFFFFF)
                                    } else {
                                        rgb(pal.text_secondary)
                                    })
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _e, _w, cx| {
                                            this.show_scheduled_section =
                                                !this.show_scheduled_section;
                                            cx.notify();
                                        }),
                                    )
                                    .child(if sched_list.is_empty() {
                                        "📅".to_string()
                                    } else {
                                        format!("📅{}", sched_list.len())
                                    }),
                            )
                            // In-Chat Search Toggle
                            .child(
                                div()
                                    .cursor_pointer()
                                    .px(px(6.0))
                                    .py(px(4.0))
                                    .rounded(px(6.0))
                                    .hover(move |s| s.bg(rgb(pal.input_bg)))
                                    .text_size(px(15.0))
                                    .text_color(if search_open {
                                        rgb(pal.accent)
                                    } else {
                                        rgb(pal.text_secondary)
                                    })
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _e, _w, cx| {
                                            this.is_search_bar_open = !this.is_search_bar_open;
                                            if !this.is_search_bar_open {
                                                this.in_chat_search_query.clear();
                                            }
                                            cx.notify();
                                        }),
                                    )
                                    .child("🔍"),
                            )
                            // Voice Call
                            .child(
                                div()
                                    .cursor_pointer()
                                    .px(px(6.0))
                                    .py(px(4.0))
                                    .rounded(px(6.0))
                                    .hover(move |s| s.bg(rgb(pal.input_bg)))
                                    .text_size(px(15.0))
                                    .text_color(rgb(pal.text_secondary))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _event, _window, cx| {
                                            this.pending_call_request = true;
                                            cx.notify();
                                        }),
                                    )
                                    .child("📞"),
                            )
                            // Remote Computer / Video Stream
                            .child(
                                div()
                                    .cursor_pointer()
                                    .px(px(6.0))
                                    .py(px(4.0))
                                    .rounded(px(6.0))
                                    .hover(move |s| s.bg(rgb(pal.input_bg)))
                                    .text_size(px(15.0))
                                    .text_color(rgb(pal.text_secondary))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _event, _window, cx| {
                                            this.pending_bot_action =
                                                Some("Remote Computer".to_string());
                                            cx.notify();
                                        }),
                                    )
                                    .child("🖥️"),
                            )
                            // Right Info Drawer Toggle
                            .child(
                                div()
                                    .cursor_pointer()
                                    .px(px(6.0))
                                    .py(px(4.0))
                                    .rounded(px(6.0))
                                    .hover(move |s| s.bg(rgb(pal.input_bg)))
                                    .text_size(px(15.0))
                                    .text_color(if is_inspector_open {
                                        rgb(pal.accent)
                                    } else {
                                        rgb(pal.text_secondary)
                                    })
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _event, _window, cx| {
                                            this.toggle_inspector();
                                            cx.notify();
                                        }),
                                    )
                                    .child("ℹ️"),
                            )
                            // Chat More Menu (`⋮`)
                            .child(
                                div()
                                    .cursor_pointer()
                                    .px(px(8.0))
                                    .py(px(4.0))
                                    .rounded(px(6.0))
                                    .hover(move |s| s.bg(rgb(pal.input_bg)))
                                    .text_size(px(16.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_secondary))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _e, _w, cx| {
                                            this.is_more_menu_open = !this.is_more_menu_open;
                                            cx.notify();
                                        }),
                                    )
                                    .child("⋮"),
                            ),
                    ),
            )
            // 1b. Chat More Dropdown Toolbar (when `⋮` is open)
            .when(more_open, |el| {
                el.child(
                    div()
                        .w_full()
                        .px(px(16.0))
                        .py(px(6.0))
                        .bg(rgb(pal.modal_bg))
                        .border_b_1()
                        .border_color(rgb(pal.border_fg))
                        .flex()
                        .flex_wrap()
                        .items_center()
                        .gap(px(8.0))
                        .child(
                            div()
                                .cursor_pointer()
                                .px(px(10.0))
                                .py(px(4.0))
                                .rounded(px(6.0))
                                .bg(rgb(pal.input_bg))
                                .text_size(px(11.5))
                                .text_color(rgb(pal.text_primary))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.is_chat_translated = !this.is_chat_translated;
                                        this.is_more_menu_open = false;
                                        cx.notify();
                                    }),
                                )
                                .child("🌐 切换整页双语翻译"),
                        )
                        .child(
                            div()
                                .cursor_pointer()
                                .px(px(10.0))
                                .py(px(4.0))
                                .rounded(px(6.0))
                                .bg(rgb(pal.input_bg))
                                .text_size(px(11.5))
                                .text_color(rgb(pal.text_primary))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.show_group_call_banner = !this.show_group_call_banner;
                                        this.is_more_menu_open = false;
                                        cx.notify();
                                    }),
                                )
                                .child("🔊 开启/收起群组语聊房"),
                        )
                        .child(
                            div()
                                .cursor_pointer()
                                .px(px(10.0))
                                .py(px(4.0))
                                .rounded(px(6.0))
                                .bg(rgb(pal.input_bg))
                                .text_size(px(11.5))
                                .text_color(rgb(pal.text_primary))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.is_more_menu_open = false;
                                        this.pending_stage_action =
                                            Some(StageActionRequest::OpenCreatePoll);
                                        cx.notify();
                                    }),
                                )
                                .child("📊 发起投票"),
                        )
                        .child(
                            div()
                                .cursor_pointer()
                                .px(px(10.0))
                                .py(px(4.0))
                                .rounded(px(6.0))
                                .bg(rgb(pal.input_bg))
                                .text_size(px(11.5))
                                .text_color(rgb(pal.text_primary))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.is_more_menu_open = false;
                                        this.pending_stage_action =
                                            Some(StageActionRequest::OpenExportChat);
                                        cx.notify();
                                    }),
                                )
                                .child("📤 导出聊天记录"),
                        )
                        .child(
                            div()
                                .cursor_pointer()
                                .px(px(10.0))
                                .py(px(4.0))
                                .rounded(px(6.0))
                                .bg(rgb(pal.input_bg))
                                .text_size(px(11.5))
                                .text_color(rgb(pal.danger))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.messages.clear();
                                        this.is_more_menu_open = false;
                                        cx.notify();
                                    }),
                                )
                                .child("🧹 清空历史消息"),
                        ),
                )
            })
            // 1c. Top Floating Audio / Voice Player Bar (`MediaPlayerWidget`)
            .when_some(audio_track, |el, track| {
                let tid = track.message_id;
                el.child(
                    div()
                        .h(px(38.0))
                        .flex_shrink_0()
                        .w_full()
                        .px(px(16.0))
                        .bg(rgb(pal.modal_bg))
                        .border_b_1()
                        .border_color(rgb(pal.border_fg))
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(10.0))
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .w(px(26.0))
                                        .h(px(26.0))
                                        .rounded_full()
                                        .bg(rgb(pal.accent))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_size(px(11.0))
                                        .text_color(rgb(0xFFFFFF))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _e, _w, cx| {
                                                if let Some(ref mut tr) = this.active_audio_player {
                                                    tr.is_playing = !tr.is_playing;
                                                }
                                                cx.notify();
                                            }),
                                        )
                                        .child(if track.is_playing { "❚❚" } else { "▶" }),
                                )
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(pal.text_primary))
                                        .child(format!(
                                            "🎵 {} • {:.0}s / {}s",
                                            track.sender_or_title,
                                            track.position_secs,
                                            track.duration_secs
                                        )),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                // Playback Speed Pill (1x / 1.5x / 2x)
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .px(px(7.0))
                                        .py(px(2.0))
                                        .rounded(px(5.0))
                                        .bg(rgb(pal.input_bg))
                                        .text_size(px(11.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(pal.accent))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _e, _w, cx| {
                                                if let Some(ref mut tr) = this.active_audio_player {
                                                    tr.speed = if (tr.speed - 1.0).abs() < 0.1 {
                                                        1.5
                                                    } else if (tr.speed - 1.5).abs() < 0.1 {
                                                        2.0
                                                    } else {
                                                        1.0
                                                    };
                                                }
                                                cx.notify();
                                            }),
                                        )
                                        .child(format!("{:.1}X", track.speed)),
                                )
                                // Voice-to-Text `[->A]` Button
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .px(px(7.0))
                                        .py(px(2.0))
                                        .rounded(px(5.0))
                                        .bg(rgb(pal.input_bg))
                                        .text_size(px(11.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(pal.accent))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _e, _w, cx| {
                                                this.transcribed_voice_ids.insert(tid);
                                                cx.notify();
                                            }),
                                        )
                                        .child("[->A] 转文字"),
                                )
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .text_size(px(12.0))
                                        .text_color(rgb(pal.text_secondary))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _e, _w, cx| {
                                                this.active_audio_player = None;
                                                cx.notify();
                                            }),
                                        )
                                        .child("✕"),
                                ),
                        ),
                )
            })
            // 1d. Group Voice Chat Banner (`GroupCallBar`)
            .when(show_gc_bar, |el| {
                el.child(
                    div()
                        .h(px(38.0))
                        .flex_shrink_0()
                        .w_full()
                        .px(px(16.0))
                        .bg(rgb(0x1B3B5F))
                        .border_b_1()
                        .border_color(rgb(pal.border_fg))
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_size(px(12.5))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xFFFFFF))
                                .child("🔊 群组实时语音/屏幕共享房间 (Live Voice Chat • 4 speaking)"),
                        )
                        .child(
                            div()
                                .cursor_pointer()
                                .px(px(12.0))
                                .py(px(4.0))
                                .rounded_full()
                                .bg(rgb(0x4CAF50))
                                .text_size(px(11.5))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xFFFFFF))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.pending_call_request = true;
                                        cx.notify();
                                    }),
                                )
                                .child("加入语聊 (Join)"),
                        ),
                )
            })
            // 1e. Real-Time Chat Translation Banner (`TranslateBar`)
            .when(show_tr_bar, |el| {
                el.child(
                    div()
                        .h(px(34.0))
                        .flex_shrink_0()
                        .w_full()
                        .px(px(16.0))
                        .bg(rgb(pal.top_bar_bg))
                        .border_b_1()
                        .border_color(rgb(pal.border_fg))
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .cursor_pointer()
                                .text_size(px(12.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(pal.accent))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.is_chat_translated = !this.is_chat_translated;
                                        cx.notify();
                                    }),
                                )
                                .child(if is_translated {
                                    "🌐 已开启全会话实时中文双语翻译 (点击显示原文 Show Original)"
                                } else {
                                    "🌐 将此对话实时翻译为简体中文 (Translate Chat)"
                                }),
                        )
                        .child(
                            div()
                                .cursor_pointer()
                                .text_size(px(11.5))
                                .text_color(rgb(pal.text_secondary))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.show_translate_banner = false;
                                        cx.notify();
                                    }),
                                )
                                .child("✕"),
                        ),
                )
            })
            // 1f. In-Chat Search Bar (`ChatSearchIn`)
            .when(search_open, |el| {
                el.child(
                    div()
                        .h(px(38.0))
                        .flex_shrink_0()
                        .w_full()
                        .px(px(16.0))
                        .bg(rgb(pal.input_bg))
                        .border_b_1()
                        .border_color(rgb(pal.border_fg))
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_size(px(12.5))
                                .text_color(rgb(pal.text_primary))
                                .child(if search_q.is_empty() {
                                    "🔍 在当前聊天中按键盘输入关键词过滤消息...".to_string()
                                } else {
                                    format!("🔍 搜索词: \"{}\" • 匹配 {} 条消息", search_q, match_count)
                                }),
                        )
                        .child(
                            div()
                                .cursor_pointer()
                                .text_size(px(12.0))
                                .text_color(rgb(pal.accent))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.in_chat_search_query.clear();
                                        this.is_search_bar_open = false;
                                        cx.notify();
                                    }),
                                )
                                .child("关闭搜索 ✕"),
                        ),
                )
            })
            // 2. Multi-Select Toolbar OR Pinned Message Bar
            .child(if !sel_ids.is_empty() {
                let ids_for_fwd = sel_ids.clone();
                let ids_for_del = sel_ids.clone();
                div()
                    .h(px(40.0))
                    .flex_shrink_0()
                    .w_full()
                    .px(px(16.0))
                    .bg(rgb(pal.modal_bg))
                    .border_b_1()
                    .border_color(rgb(pal.accent))
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(10.0))
                            .child(
                                div()
                                    .cursor_pointer()
                                    .px(px(12.0))
                                    .py(px(4.0))
                                    .rounded(px(6.0))
                                    .bg(rgb(pal.accent))
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _e, _w, cx| {
                                            this.pending_stage_action = Some(
                                                StageActionRequest::ForwardMessages(
                                                    ids_for_fwd.clone(),
                                                ),
                                            );
                                            cx.notify();
                                        }),
                                    )
                                    .child(format!("↪ 转发 ({})", sel_ids.len())),
                            )
                            .child(
                                div()
                                    .cursor_pointer()
                                    .px(px(12.0))
                                    .py(px(4.0))
                                    .rounded(px(6.0))
                                    .bg(rgb(pal.danger))
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _e, _w, cx| {
                                            this.pending_stage_action = Some(
                                                StageActionRequest::DeleteMessages(
                                                    ids_for_del.clone(),
                                                ),
                                            );
                                            cx.notify();
                                        }),
                                    )
                                    .child(format!("🗑 删除 ({})", sel_ids.len())),
                            ),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .text_size(px(12.0))
                            .text_color(rgb(pal.text_secondary))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _e, _w, cx| {
                                    this.selected_message_ids.clear();
                                    cx.notify();
                                }),
                            )
                            .child("取消多选 Cancel"),
                    )
                    .into_any_element()
            } else if let Some(pin_text) = pinned {
                div()
                    .h(px(36.0))
                    .flex_shrink_0()
                    .w_full()
                    .bg(rgb(pal.top_bar_bg))
                    .border_b_1()
                    .border_color(rgb(pal.border_fg))
                    .px(px(16.0))
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .border_l_2()
                            .border_color(rgb(pal.accent))
                            .pl(px(8.0))
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.accent))
                                    .child("📌 置顶消息:"),
                            )
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(rgb(pal.text_primary))
                                    .child(pin_text),
                            ),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .text_size(px(12.0))
                            .text_color(rgb(pal.text_secondary))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _event, _window, cx| {
                                    this.pinned_message = None;
                                    cx.notify();
                                }),
                            )
                            .child("✕"),
                    )
                    .into_any_element()
            } else {
                div().h(px(0.0)).into_any_element()
            })
            // 2b. Scheduled Messages Panel (when `📅` toggled open)
            .when(show_sched, |el| {
                el.child(
                    div()
                        .w_full()
                        .px(px(20.0))
                        .py(px(10.0))
                        .bg(rgb(pal.modal_bg))
                        .border_b_1()
                        .border_color(rgb(pal.accent))
                        .flex()
                        .flex_col()
                        .gap(px(6.0))
                        .child(
                            div()
                                .text_size(px(12.5))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(pal.accent))
                                .child(format!(
                                    "📅 定时发送队列 Scheduled Messages ({})",
                                    sched_list.len()
                                )),
                        )
                        .when(sched_list.is_empty(), |empty_el| {
                            empty_el.child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(rgb(pal.text_secondary))
                                    .child("当前暂无定时消息（在底部输入文字后点击 ⏱ -> 定时发送即可加入队列）"),
                            )
                        })
                        .children(sched_list.into_iter().map(|(sid, stxt, when_lbl)| {
                            div()
                                .p(px(8.0))
                                .rounded(px(6.0))
                                .bg(rgb(pal.input_bg))
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .text_size(px(12.5))
                                        .text_color(rgb(pal.text_primary))
                                        .child(format!("[{}] {}", when_lbl, stxt)),
                                )
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .px(px(8.0))
                                        .py(px(3.0))
                                        .rounded(px(5.0))
                                        .bg(rgb(pal.accent))
                                        .text_size(px(11.0))
                                        .text_color(rgb(0xFFFFFF))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _e, _w, cx| {
                                                let cur_c = this.active_chat_id.unwrap_or(1);
                                                if let Some(vec_s) =
                                                    this.scheduled_messages.get_mut(&cur_c)
                                                {
                                                    if let Some(pos) =
                                                        vec_s.iter().position(|(id, _, _)| *id == sid)
                                                    {
                                                        let (mid, body, _) = vec_s.remove(pos);
                                                        this.add_message(MessageItemModel {
                                                            id: mid,
                                                            sender_id: 1,
                                                            sender_name: "You".to_string(),
                                                            avatar_initials: "ME".to_string(),
                                                            avatar_color: 0x3498DB,
                                                            direction: MessageDirection::Outgoing,
                                                            timestamp: "Sent from Schedule".to_string(),
                                                            delivery_status: DeliveryStatus::Read,
                                                            reply_to: None,
                                                            reactions: vec![],
                                                            content: MessageContent::Text {
                                                                text: body,
                                                                is_edited: false,
                                                            },
                                                            bot_keyboard: None,
                                                            is_pinned: false,
                                                        });
                                                    }
                                                }
                                                cx.notify();
                                            }),
                                        )
                                        .child("立即发送"),
                                )
                        })),
                )
            })
            // 3. Virtualized Message Transcript Area
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
    fn render_interactive_message_row(
        &self,
        msg: MessageItemModel,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let pal = self.palette;
        let msg_id = msg.id;
        let is_outgoing = msg.is_outgoing();
        let is_selected = self.selected_message_ids.contains(&msg_id);
        let is_menu_open = self.active_action_menu_msg_id == Some(msg_id);
        let is_transcribed = self.transcribed_voice_ids.contains(&msg_id);

        if let MessageContent::Service {
            ref text,
            service_type,
        } = msg.content
        {
            return div()
                .w_full()
                .flex_shrink_0()
                .flex()
                .justify_center()
                .child(ServiceMessageView::render(text, service_type))
                .into_any_element();
        }

        let media_click_info = match &msg.content {
            MessageContent::Media {
                url, media_type, ..
            } => Some((url.clone(), *media_type == MediaType::Video)),
            MessageContent::Album { .. } => {
                Some(("https://example.com/album_photo.jpg".to_string(), false))
            }
            MessageContent::Story { .. } => {
                Some(("https://example.com/story_stream.mp4".to_string(), true))
            }
            MessageContent::Location {
                latitude,
                longitude,
                ..
            } => Some((
                format!("https://maps.google.com/?q={},{}", latitude, longitude),
                false,
            )),
            _ => None,
        };

        let is_voice_msg = matches!(msg.content, MessageContent::Voice { .. });
        let is_call_msg = matches!(msg.content, MessageContent::Call { .. });
        let is_invoice_msg = matches!(msg.content, MessageContent::Invoice { .. });
        let has_spoiler_text = if let MessageContent::Text { ref text, .. } = msg.content {
            text.contains("||")
        } else {
            false
        };
        let detected_iv_url = if let MessageContent::Text { ref text, .. } = msg.content {
            TextMessageView::extract_first_url(text)
        } else {
            None
        };

        let snippet_for_ops = Self::extract_message_snippet(&msg);
        let sender_for_reply = msg.sender_name.clone();
        let full_text_for_edit = if let MessageContent::Text { ref text, .. } = msg.content {
            Some(text.clone())
        } else {
            None
        };

        let bubble = self.render_message_bubble(&msg, cx);

        // Poll Vote Buttons
        let poll_vote_bar: Option<AnyElement> = if let MessageContent::Poll {
            ref options,
            is_closed: false,
            ..
        } = msg.content
        {
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
                        .bg(if is_sel {
                            rgb(pal.accent)
                        } else {
                            rgb(pal.input_bg)
                        })
                        .hover(move |s| s.bg(rgb(pal.accent)))
                        .text_size(px(11.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(if is_sel {
                            rgb(0xFFFFFF)
                        } else {
                            rgb(pal.text_primary)
                        })
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _event, _window, cx| {
                                this.vote_poll(msg_id, opt_id);
                                cx.notify();
                            }),
                        )
                        .child(format!("🗳️ 投选 #{} ({:.0}%)", opt_id, opt.percentage))
                        .into_any_element(),
                );
            }
            Some(
                div()
                    .flex()
                    .flex_wrap()
                    .items_center()
                    .gap(px(6.0))
                    .children(vote_btns)
                    .into_any_element(),
            )
        } else {
            None
        };

        // Inline Bot Keyboard
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
                            .hover(move |s| s.bg(rgb(pal.accent)))
                            .border_1()
                            .border_color(rgb(0x2B5278))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _event, _window, cx| {
                                    this.pending_bot_action = Some(action_label.clone());
                                    cx.notify();
                                }),
                            )
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

        // Quick-Reaction Pills + Paid Star Reaction + Action Menu Trigger (`⋯`)
        let mut reaction_pills = Vec::with_capacity(7);
        for emoji in ["👍", "🔥", "❤️", "🎉"] {
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
                    .bg(if reacted {
                        rgb(pal.dialogs_active_bg)
                    } else {
                        rgb(pal.input_bg)
                    })
                    .text_size(px(11.0))
                    .text_color(if reacted {
                        rgb(0xFFFFFF)
                    } else {
                        rgb(pal.text_secondary)
                    })
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _event, _window, cx| {
                            this.toggle_reaction(msg_id, &emoji_str);
                            cx.notify();
                        }),
                    )
                    .child(if count > 0 {
                        format!("{} {}", emoji, count)
                    } else {
                        emoji.to_string()
                    })
                    .into_any_element(),
            );
        }
        // Paid Star Reaction (`⭐`)
        reaction_pills.push(
            div()
                .cursor_pointer()
                .px(px(6.0))
                .py(px(2.0))
                .rounded_full()
                .bg(rgba(0xF1C40F22))
                .text_size(px(11.0))
                .text_color(rgb(0xF1C40F))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _e, _w, cx| {
                        this.toggle_reaction(msg_id, "⭐");
                        this.pending_stage_action =
                            Some(StageActionRequest::PaidStarReaction(msg_id));
                        cx.notify();
                    }),
                )
                .child("⭐")
                .into_any_element(),
        );
        // Voice-to-Text `[->A]` Pill on Voice Messages
        if is_voice_msg {
            reaction_pills.push(
                div()
                    .cursor_pointer()
                    .px(px(7.0))
                    .py(px(2.0))
                    .rounded_full()
                    .bg(rgb(pal.input_bg))
                    .text_size(px(10.5))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(pal.accent))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _e, _w, cx| {
                            if this.transcribed_voice_ids.contains(&msg_id) {
                                this.transcribed_voice_ids.remove(&msg_id);
                            } else {
                                this.transcribed_voice_ids.insert(msg_id);
                            }
                            cx.notify();
                        }),
                    )
                    .child("[->A] 转文字")
                    .into_any_element(),
            );
        }
        // Message Operations Menu Trigger (`↩ / ⋯ 操作`)
        reaction_pills.push(
            div()
                .cursor_pointer()
                .px(px(7.0))
                .py(px(2.0))
                .rounded_full()
                .bg(if is_menu_open {
                    rgb(pal.accent)
                } else {
                    rgb(pal.input_bg)
                })
                .text_size(px(10.5))
                .font_weight(FontWeight::BOLD)
                .text_color(if is_menu_open {
                    rgb(0xFFFFFF)
                } else {
                    rgb(pal.text_secondary)
                })
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _e, _w, cx| {
                        this.active_action_menu_msg_id =
                            if this.active_action_menu_msg_id == Some(msg_id) {
                                None
                            } else {
                                Some(msg_id)
                            };
                        cx.notify();
                    }),
                )
                .child("⋯ 菜单")
                .into_any_element(),
        );

        let row_align = if is_outgoing {
            div().w_full().flex_shrink_0().flex().justify_end()
        } else {
            div().w_full().flex_shrink_0().flex().justify_start()
        };

        let sender_title_for_audio = msg.sender_name.clone();
        let snip_reply = snippet_for_ops.clone();
        let snip_pin = snippet_for_ops.clone();
        let snip_copy = snippet_for_ops.clone();
        let snip_tr = snippet_for_ops.clone();
        let iv_title = msg.sender_name.clone();

        row_align
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .when(is_selected, |el| {
                        el.p(px(4.0))
                            .rounded(px(10.0))
                            .bg(rgba(0x2AABEE22))
                            .border_1()
                            .border_color(rgb(pal.accent))
                    })
                    .child(
                        div()
                            .cursor_pointer()
                            .on_mouse_down(
                                MouseButton::Right,
                                cx.listener(move |this, _e, _w, cx| {
                                    this.active_action_menu_msg_id =
                                        if this.active_action_menu_msg_id == Some(msg_id) {
                                            None
                                        } else {
                                            Some(msg_id)
                                        };
                                    cx.notify();
                                }),
                            )
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _event, _window, cx| {
                                    if has_spoiler_text && !this.revealed_spoiler_ids.contains(&msg_id) {
                                        this.revealed_spoiler_ids.insert(msg_id);
                                        cx.notify();
                                        return;
                                    }
                                    if let Some(ref iv_url) = detected_iv_url {
                                        this.pending_stage_action =
                                            Some(StageActionRequest::OpenInstantView {
                                                title: format!(
                                                    "Instant View • Shared by {}",
                                                    iv_title
                                                ),
                                                url: iv_url.clone(),
                                            });
                                        cx.notify();
                                    } else if let Some((ref url, is_video)) = media_click_info {
                                        this.pending_media_url = Some((url.clone(), is_video));
                                        cx.notify();
                                    } else if is_voice_msg {
                                        let mut dur_out = 42u32;
                                        let mut pos_out = 0.0f32;
                                        let mut spd_out = 1.0f32;
                                        let mut play_out = true;
                                        if let Some(m) =
                                            this.messages.iter_mut().find(|m| m.id == msg_id)
                                        {
                                            if let MessageContent::Voice {
                                                ref mut is_playing,
                                                ref mut playback_position_secs,
                                                playback_speed,
                                                duration_secs,
                                                ..
                                            } = m.content
                                            {
                                                *is_playing = !*is_playing;
                                                if *is_playing {
                                                    *playback_position_secs =
                                                        (*playback_position_secs + 6.0)
                                                            % (duration_secs as f32).max(1.0);
                                                }
                                                dur_out = duration_secs;
                                                pos_out = *playback_position_secs;
                                                spd_out = playback_speed;
                                                play_out = *is_playing;
                                            }
                                        }
                                        this.active_audio_player = Some(ActiveAudioTrack {
                                            message_id: msg_id,
                                            sender_or_title: format!(
                                                "{} (Voice Note)",
                                                sender_title_for_audio
                                            ),
                                            duration_secs: dur_out,
                                            position_secs: pos_out,
                                            speed: spd_out,
                                            is_playing: play_out,
                                        });
                                        cx.notify();
                                    } else if is_call_msg {
                                        this.pending_call_request = true;
                                        cx.notify();
                                    } else if is_invoice_msg {
                                        if let Some(m) =
                                            this.messages.iter_mut().find(|m| m.id == msg_id)
                                        {
                                            if let MessageContent::Invoice {
                                                ref mut status, ..
                                            } = m.content
                                            {
                                                *status = match *status {
                                                    InvoiceStatus::Pending => InvoiceStatus::Paid,
                                                    InvoiceStatus::Paid => InvoiceStatus::Refunded,
                                                    InvoiceStatus::Refunded => InvoiceStatus::Paid,
                                                };
                                            }
                                        }
                                        cx.notify();
                                    } else {
                                        this.active_action_menu_msg_id =
                                            if this.active_action_menu_msg_id == Some(msg_id) {
                                                None
                                            } else {
                                                Some(msg_id)
                                            };
                                        cx.notify();
                                    }
                                }),
                            )
                            .child(bubble),
                    )
                    // Voice-to-Text Transcript Card (when `[->A]` is active)
                    .when(is_voice_msg && is_transcribed, |el| {
                        el.child(
                            div()
                                .max_w(px(360.0))
                                .p(px(8.0))
                                .rounded(px(8.0))
                                .bg(rgb(pal.modal_bg))
                                .border_l_2()
                                .border_color(rgb(pal.accent))
                                .text_size(px(12.0))
                                .text_color(rgb(pal.text_primary))
                                .child("🗣️ [语音转文字 Transcript]: “我们已经完成了 GPUI 原生 120 帧渲染管线与 5-bit 语音波形器的全量迁移。”"),
                        )
                    })
                    // Expanded Telegram Message Context Action Toolbar (`Reply / Edit / Forward / Pin / Copy / Translate / Select / Delete`)
                    .when(is_menu_open, |el| {
                        el.child(
                            div()
                                .p(px(6.0))
                                .rounded(px(8.0))
                                .bg(rgb(pal.modal_bg))
                                .border_1()
                                .border_color(rgb(pal.accent))
                                .flex()
                                .flex_wrap()
                                .items_center()
                                .gap(px(6.0))
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .px(px(8.0))
                                        .py(px(3.0))
                                        .rounded(px(5.0))
                                        .bg(rgb(pal.input_bg))
                                        .text_size(px(11.0))
                                        .text_color(rgb(pal.text_primary))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _e, _w, cx| {
                                                this.active_action_menu_msg_id = None;
                                                this.pending_stage_action =
                                                    Some(StageActionRequest::ReplyToMessage {
                                                        message_id: msg_id,
                                                        sender_name: sender_for_reply.clone(),
                                                        snippet: snip_reply.clone(),
                                                    });
                                                cx.notify();
                                            }),
                                        )
                                        .child("↩ 回复"),
                                )
                                .when_some(
                                    if is_outgoing { full_text_for_edit } else { None },
                                    |ed_el, cur_txt| {
                                        ed_el.child(
                                            div()
                                                .cursor_pointer()
                                                .px(px(8.0))
                                                .py(px(3.0))
                                                .rounded(px(5.0))
                                                .bg(rgb(pal.input_bg))
                                                .text_size(px(11.0))
                                                .text_color(rgb(pal.text_primary))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(move |this, _e, _w, cx| {
                                                        this.active_action_menu_msg_id = None;
                                                        this.pending_stage_action =
                                                            Some(StageActionRequest::EditMessage {
                                                                message_id: msg_id,
                                                                current_text: cur_txt.clone(),
                                                            });
                                                        cx.notify();
                                                    }),
                                                )
                                                .child("✎ 编辑"),
                                        )
                                    },
                                )
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .px(px(8.0))
                                        .py(px(3.0))
                                        .rounded(px(5.0))
                                        .bg(rgb(pal.input_bg))
                                        .text_size(px(11.0))
                                        .text_color(rgb(pal.text_primary))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _e, _w, cx| {
                                                this.active_action_menu_msg_id = None;
                                                this.pending_stage_action = Some(
                                                    StageActionRequest::ForwardMessages(vec![
                                                        msg_id,
                                                    ]),
                                                );
                                                cx.notify();
                                            }),
                                        )
                                        .child("↪ 转发"),
                                )
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .px(px(8.0))
                                        .py(px(3.0))
                                        .rounded(px(5.0))
                                        .bg(rgb(pal.input_bg))
                                        .text_size(px(11.0))
                                        .text_color(rgb(pal.text_primary))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _e, _w, cx| {
                                                this.pinned_message = Some(snip_pin.clone());
                                                if let Some(m) =
                                                    this.messages.iter_mut().find(|m| m.id == msg_id)
                                                {
                                                    m.is_pinned = !m.is_pinned;
                                                }
                                                this.active_action_menu_msg_id = None;
                                                cx.notify();
                                            }),
                                        )
                                        .child("📌 置顶"),
                                )
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .px(px(8.0))
                                        .py(px(3.0))
                                        .rounded(px(5.0))
                                        .bg(rgb(pal.input_bg))
                                        .text_size(px(11.0))
                                        .text_color(rgb(pal.text_primary))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _e, _w, cx| {
                                                this.active_action_menu_msg_id = None;
                                                this.pending_stage_action = Some(
                                                    StageActionRequest::CopyText(snip_copy.clone()),
                                                );
                                                cx.notify();
                                            }),
                                        )
                                        .child("📋 复制"),
                                )
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .px(px(8.0))
                                        .py(px(3.0))
                                        .rounded(px(5.0))
                                        .bg(rgb(pal.input_bg))
                                        .text_size(px(11.0))
                                        .text_color(rgb(pal.text_primary))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _e, _w, cx| {
                                                this.active_action_menu_msg_id = None;
                                                this.pending_stage_action =
                                                    Some(StageActionRequest::TranslateMessage(
                                                        snip_tr.clone(),
                                                    ));
                                                cx.notify();
                                            }),
                                        )
                                        .child("🌐 翻译"),
                                )
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .px(px(8.0))
                                        .py(px(3.0))
                                        .rounded(px(5.0))
                                        .bg(rgb(pal.input_bg))
                                        .text_size(px(11.0))
                                        .text_color(rgb(pal.text_primary))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _e, _w, cx| {
                                                this.toggle_select_message(msg_id);
                                                this.active_action_menu_msg_id = None;
                                                cx.notify();
                                            }),
                                        )
                                        .child("☑ 多选"),
                                )
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .px(px(8.0))
                                        .py(px(3.0))
                                        .rounded(px(5.0))
                                        .bg(rgb(pal.input_bg))
                                        .text_size(px(11.0))
                                        .text_color(rgb(pal.danger))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _e, _w, cx| {
                                                this.active_action_menu_msg_id = None;
                                                this.pending_stage_action = Some(
                                                    StageActionRequest::DeleteMessages(vec![
                                                        msg_id,
                                                    ]),
                                                );
                                                cx.notify();
                                            }),
                                        )
                                        .child("🗑 删除"),
                                ),
                        )
                    })
                    .when_some(poll_vote_bar, |el, bar| el.child(bar))
                    .when_some(bot_kb_element, |el, kb| el.child(kb))
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .items_center()
                            .gap(px(4.0))
                            .children(reaction_pills),
                    ),
            )
            .into_any_element()
    }

    fn render_message_bubble(&self, msg: &MessageItemModel, cx: &mut Context<Self>) -> AnyElement {
        match &msg.content {
            MessageContent::Text { text, is_edited } => {
                let revealed = self.revealed_spoiler_ids.contains(&msg.id);
                let inline_tr = if self.is_chat_translated && !msg.is_outgoing() {
                    Some(format!("中文译文：{}", text))
                } else {
                    None
                };
                TextMessageView::render_rich(
                    msg,
                    text,
                    *is_edited,
                    self.palette,
                    revealed,
                    inline_tr,
                )
                .into_any_element()
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
            )
            .into_any_element(),
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
            )
            .into_any_element(),
            MessageContent::Album { items, caption } => {
                AlbumMessageView::render(msg, items, caption.as_deref()).into_any_element()
            }
            MessageContent::Location {
                latitude,
                longitude,
                title,
                address,
            } => LocationMessageView::render(msg, *latitude, *longitude, title, address)
                .into_any_element(),
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
            )
            .into_any_element(),
            MessageContent::Service { text, service_type } => {
                ServiceMessageView::render(text, *service_type).into_any_element()
            }
            MessageContent::Call {
                call_type,
                direction,
                duration_secs,
                status_text,
            } => CallMessageView::render(msg, *call_type, *direction, *duration_secs, status_text)
                .into_any_element(),
            MessageContent::Story {
                user_name,
                avatar_initials,
                caption,
                preview_color,
                expires_in,
            } => StoryMessageView::render(
                msg,
                user_name,
                avatar_initials,
                caption,
                *preview_color,
                expires_in,
            )
            .into_any_element(),
            MessageContent::Invoice {
                title,
                description,
                amount_stars,
                currency,
                status,
                receipt_id,
            } => InvoiceMessageView::render(
                msg,
                title,
                description,
                *amount_stars,
                currency,
                *status,
                receipt_id.as_deref(),
            )
            .into_any_element(),
            MessageContent::BotKeyboard { rows } => {
                InlineBotKeyboardView::render(rows).into_any_element()
            }
        }
    }
}
