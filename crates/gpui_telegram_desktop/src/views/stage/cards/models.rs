//! Canonical message card data models and definitions (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.4).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageDirection {
    Outgoing,
    Incoming,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeliveryStatus {
    Sending,
    Sent,
    Read,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactionItem {
    pub emoji: String,
    pub count: u32,
    pub user_reacted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplyReference {
    pub message_id: u64,
    pub sender_name: String,
    pub snippet: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaType {
    Photo,
    Video,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlbumItem {
    pub id: u64,
    pub media_type: MediaType,
    pub url: String,
    pub aspect_ratio: f32,
    pub preview_color: u32,
    pub caption: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PollType {
    Regular,
    Quiz,
    Anonymous,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PollOption {
    pub id: u32,
    pub text: String,
    pub votes: u32,
    pub percentage: f32,
    pub is_selected: bool,
    pub is_correct: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceType {
    UserJoined,
    PinnedMessage,
    SecurityKeyChanged,
    CallEnded,
    GroupCreated,
    DateDivider,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CallType {
    Voice,
    Video,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CallDirection {
    Incoming,
    Outgoing,
    Missed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvoiceStatus {
    Pending,
    Paid,
    Refunded,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InlineButtonType {
    Callback(String),
    Url(String),
    WebApp(String),
    SwitchInline(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InlineKeyboardButton {
    pub text: String,
    pub button_type: InlineButtonType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageContent {
    /// 1. Canonical Text Message
    Text {
        text: String,
        is_edited: bool,
    },
    /// 2. Canonical Media Message (Photo, Video)
    Media {
        media_type: MediaType,
        url: String,
        width: u32,
        height: u32,
        duration_secs: Option<u32>,
        file_size_bytes: u64,
        caption: Option<String>,
        is_streaming: bool,
    },
    /// 3. Canonical Voice Message (with 5-bit waveform renderer)
    Voice {
        duration_secs: u32,
        waveform_5bit: Vec<u8>,
        playback_position_secs: f32,
        is_playing: bool,
        playback_speed: f32,
    },
    /// 4. Canonical Album Message (Photo/Video mosaic collage)
    Album {
        items: Vec<AlbumItem>,
        caption: Option<String>,
    },
    /// 5. Canonical Location Message
    Location {
        latitude: f64,
        longitude: f64,
        title: String,
        address: String,
    },
    /// 6. Canonical Poll Message
    Poll {
        question: String,
        poll_type: PollType,
        options: Vec<PollOption>,
        total_votes: u32,
        is_closed: bool,
        selected_option_id: Option<u32>,
    },
    /// 7. Canonical Service Message (Centered bubble)
    Service {
        text: String,
        service_type: ServiceType,
    },
    /// 8. Canonical Call Log Message
    Call {
        call_type: CallType,
        direction: CallDirection,
        duration_secs: u32,
        status_text: String,
    },
    /// 9. Canonical Story Message
    Story {
        user_name: String,
        avatar_initials: String,
        caption: String,
        preview_color: u32,
        expires_in: String,
    },
    /// 10. Canonical Invoice Message (Stars ⭐ / Payments)
    Invoice {
        title: String,
        description: String,
        amount_stars: u64,
        currency: String,
        status: InvoiceStatus,
        receipt_id: Option<String>,
    },
    /// 11. Canonical Bot Inline Keyboard Card
    BotKeyboard {
        rows: Vec<Vec<InlineKeyboardButton>>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageItemModel {
    pub id: u64,
    pub sender_id: u64,
    pub sender_name: String,
    pub avatar_initials: String,
    pub avatar_color: u32,
    pub direction: MessageDirection,
    pub timestamp: String,
    pub delivery_status: DeliveryStatus,
    pub reply_to: Option<ReplyReference>,
    pub reactions: Vec<ReactionItem>,
    pub content: MessageContent,
    pub bot_keyboard: Option<Vec<Vec<InlineKeyboardButton>>>,
    pub is_pinned: bool,
}

impl MessageItemModel {
    pub fn is_outgoing(&self) -> bool {
        self.direction == MessageDirection::Outgoing
    }

    pub fn toggle_reaction(&mut self, emoji: &str) {
        if let Some(pos) = self.reactions.iter().position(|r| r.emoji == emoji) {
            if self.reactions[pos].user_reacted {
                self.reactions[pos].user_reacted = false;
                self.reactions[pos].count = self.reactions[pos].count.saturating_sub(1);
                if self.reactions[pos].count == 0 {
                    self.reactions.remove(pos);
                }
            } else {
                self.reactions[pos].user_reacted = true;
                self.reactions[pos].count += 1;
            }
        } else {
            self.reactions.push(ReactionItem {
                emoji: emoji.to_string(),
                count: 1,
                user_reacted: true,
            });
        }
    }
}
