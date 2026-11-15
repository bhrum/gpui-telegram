//! Demo fixture and stress-test message population helpers for `ConversationStage`.

use crate::views::conversation_stage::ConversationStage;
use crate::views::stage::cards::*;

impl ConversationStage {
    /// Explicit fixture constructor for legacy card/layout tests. Never called by production code.
    pub fn new_demo() -> Self {
        let mut stage = Self::new();
        stage.active_chat_id = Some(1);
        stage.chat_title = "Alice Smith".to_string();
        stage.chat_subtitle = "online".to_string();
        stage.chat_avatar_initials = "AS".to_string();
        stage.chat_avatar_color = 0x2AABEE;
        stage.pinned_message = Some("Design review meeting notes and sprint roadmap".to_string());
        stage.is_inspector_open = true;

        stage.populate_demo_messages();
        stage.chat_histories.insert(1, stage.messages.clone());
        stage.chat_pinned_messages.insert(1, stage.pinned_message.clone());
        stage.populate_secondary_chat_histories();
        stage.sync_pinned_tracker();
        stage.row_height_cache.rebuild_prefix_sums(&stage.messages);
        stage
    }

    pub fn populate_stress_messages(&mut self, count: usize) {
        self.messages.clear();
        for i in 0..count {
            self.messages.push(MessageItemModel {
                id: (i + 1) as u64,
                sender_id: if i % 2 == 0 { 1 } else { 2 },
                sender_name: if i % 2 == 0 { "Alice".to_string() } else { "Bob".to_string() },
                avatar_initials: if i % 2 == 0 { "AS".to_string() } else { "BB".to_string() },
                avatar_color: 0x2AABEE,
                direction: if i % 2 == 0 { MessageDirection::Incoming } else { MessageDirection::Outgoing },
                timestamp: format!("{:02}:{:02}", (i / 60) % 24, i % 60),
                delivery_status: DeliveryStatus::Read,
                reply_to: None,
                reactions: vec![],
                content: MessageContent::Text {
                    text: format!("Stress message index #{} with some arbitrary payload content to simulate variable text line wrapping in conversation history.", i),
                    is_edited: false,
                },
                bot_keyboard: None,
                is_pinned: false,
            });
        }
        self.row_height_cache.mark_dirty();
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
