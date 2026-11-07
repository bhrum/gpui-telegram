//! History View Element Base & Core Layout Model (`history_view_element.h/cpp`, `history_view_object.h`).
//! Upstream reference: `Telegram/SourceFiles/history/view/history_view_element.cpp:L82-L3970`
//! Mandate #18 Citations:
//! - `kAttachMessageToPreviousSecondsDelta = 900` (`history_view_element.cpp:L105`)
//! - `kMaxShownLine = 1048576` (`history_view_element.cpp:L106`)
//! - Bubble padding: Left 11px, Top 8px, Right 11px, Bottom 8px (`ui/chat/chat.style:L25`)
//! - Bubble margins: Left 16px, Top 6px, Right 56px, Bottom 2px (`ui/chat/chat.style:L52`)
//! - Bubble attached top margin: 0px (`ui/chat/chat.style:L53`)

use serde::{Deserialize, Serialize};

/// Threshold delta in seconds within which two consecutive messages from the same sender
/// attach their bubbles and collapse margins (15 minutes).
/// C++ upstream reference: `history_view_element.cpp:L105`.
pub const K_ATTACH_MESSAGE_TO_PREVIOUS_SECONDS_DELTA: i64 = 900;

/// Maximum line width in pixels before truncation/scroll fallback.
/// C++ upstream reference: `history_view_element.cpp:L106`.
pub const K_MAX_SHOWN_LINE: i32 = 1024 * 1024;

/// Standard bubble max width in pixels (`ui/chat/chat.style:L14`).
pub const MSG_MAX_WIDTH: f32 = 430.0;

/// Standard bubble min width in pixels (`ui/chat/chat.style:L22`).
pub const MSG_MIN_WIDTH: f32 = 160.0;

/// Avatar size in pixels (`ui/chat/chat.style:L23`).
pub const MSG_PHOTO_SIZE: f32 = 33.0;

/// Avatar horizontal skip in pixels (`ui/chat/chat.style:L24`).
pub const MSG_PHOTO_SKIP: f32 = 40.0;

/// Standard bubble corner radius in pixels (`bubbleRadiusLarge: 16px`).
pub const BUBBLE_RADIUS_LARGE: f32 = 16.0;

/// Attached bubble corner radius in pixels (`bubbleRadiusSmall: 4px`).
pub const BUBBLE_RADIUS_SMALL: f32 = 4.0;

/// Element flags matching C++ `HistoryView::Element::Flag` (`history_view_element.h:L441-L456`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ElementFlags(pub u16);

impl ElementFlags {
    pub const SERVICE_MESSAGE: u16 = 0x0001;
    pub const NEEDS_RESIZE: u16 = 0x0002;
    pub const ATTACHED_TO_PREVIOUS: u16 = 0x0004;
    pub const ATTACHED_TO_NEXT: u16 = 0x0008;
    pub const BUBBLE_ATTACHED_TO_PREVIOUS: u16 = 0x0010;
    pub const BUBBLE_ATTACHED_TO_NEXT: u16 = 0x0020;
    pub const HIDDEN_BY_GROUP: u16 = 0x0040;
    pub const SPECIAL_ONLY_EMOJI: u16 = 0x0080;
    pub const CUSTOM_EMOJI_REPAINTING: u16 = 0x0100;
    pub const SCHEDULED_UNTIL_ONLINE: u16 = 0x0200;
    pub const TOPIC_ROOT_REPLY: u16 = 0x0400;
    pub const MEDIA_OVERRIDDEN: u16 = 0x0800;
    pub const HEAVY_CUSTOM_EMOJI: u16 = 0x1000;
    pub const SUMMARY_SHOWN: u16 = 0x2000;

    pub fn new() -> Self {
        Self(0)
    }

    pub fn contains(&self, flag: u16) -> bool {
        (self.0 & flag) != 0
    }

    pub fn insert(&mut self, flag: u16) {
        self.0 |= flag;
    }

    pub fn remove(&mut self, flag: u16) {
        self.0 &= !flag;
    }

    pub fn set(&mut self, flag: u16, val: bool) {
        if val {
            self.insert(flag);
        } else {
            self.remove(flag);
        }
    }
}

/// History view context enum (`history_view_element.h:L77-L91`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ElementContext {
    #[default]
    History,
    Replies,
    Pinned,
    AdminLog,
    ContactPreview,
    Monoforum,
    SavedSublist,
    TTLViewer,
    ShortcutMessages,
    ScheduledTopic,
    ChatPreview,
    WelcomeMessages,
    MediaEditor,
}

/// Element chat layout mode (`history_view_element.h:L104-L109`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ElementChatMode {
    #[default]
    Default,
    Wide,
    Narrow,
}

/// Per-corner rounding for message bubbles (`history_view_message.cpp:L6646-L6649`).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BubbleRounding {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_left: f32,
    pub bottom_right: f32,
}

impl Default for BubbleRounding {
    fn default() -> Self {
        Self {
            top_left: BUBBLE_RADIUS_LARGE,
            top_right: BUBBLE_RADIUS_LARGE,
            bottom_left: BUBBLE_RADIUS_LARGE,
            bottom_right: BUBBLE_RADIUS_LARGE,
        }
    }
}

impl BubbleRounding {
    /// Compute rounding corners according to attachment status and message direction.
    /// In Telegram Desktop:
    /// - Incoming messages: attachment side is LEFT.
    ///   If attached to previous: top_left = 4px, else 16px.
    ///   If attached to next: bottom_left = 4px, else 16px.
    ///   Right corners remain 16px.
    /// - Outgoing messages: attachment side is RIGHT.
    ///   If attached to previous: top_right = 4px, else 16px.
    ///   If attached to next: bottom_right = 4px, else 16px.
    ///   Left corners remain 16px.
    pub fn for_message(is_outgoing: bool, attached_prev: bool, attached_next: bool) -> Self {
        if is_outgoing {
            Self {
                top_left: BUBBLE_RADIUS_LARGE,
                top_right: if attached_prev { BUBBLE_RADIUS_SMALL } else { BUBBLE_RADIUS_LARGE },
                bottom_left: BUBBLE_RADIUS_LARGE,
                bottom_right: if attached_next { BUBBLE_RADIUS_SMALL } else { BUBBLE_RADIUS_LARGE },
            }
        } else {
            Self {
                top_left: if attached_prev { BUBBLE_RADIUS_SMALL } else { BUBBLE_RADIUS_LARGE },
                top_right: BUBBLE_RADIUS_LARGE,
                bottom_left: if attached_next { BUBBLE_RADIUS_SMALL } else { BUBBLE_RADIUS_LARGE },
                bottom_right: BUBBLE_RADIUS_LARGE,
            }
        }
    }
}

/// Check whether a message should attach to the previous message from the same sender
/// based on sender identity and timestamp delta (`history_view_element.cpp:L105`).
pub fn should_attach_to_previous(
    prev_sender_id: Option<u64>,
    current_sender_id: u64,
    prev_date: Option<i64>,
    current_date: i64,
) -> bool {
    match (prev_sender_id, prev_date) {
        (Some(prev_id), Some(prev_ts)) => {
            prev_id == current_sender_id
                && (current_date - prev_ts).abs() <= K_ATTACH_MESSAGE_TO_PREVIOUS_SECONDS_DELTA
        }
        _ => false,
    }
}

/// Margin metrics for message bubbles in pixels (`ui/chat/chat.style:L52-L53`).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ElementMargins {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl ElementMargins {
    /// Standard message margins (`msgMargin: margins(16px, 6px, 56px, 2px)`).
    /// If attached to previous: top margin collapses to 0px (`msgMarginTopAttached: 0px`).
    pub fn for_message(attached_prev: bool) -> Self {
        Self {
            left: 16.0,
            top: if attached_prev { 0.0 } else { 6.0 },
            right: 56.0,
            bottom: 2.0,
        }
    }

    /// Service message margins (`msgServiceMargin: margins(10px, 10px, 10px, 2px)`).
    pub fn for_service() -> Self {
        Self {
            left: 10.0,
            top: 10.0,
            right: 10.0,
            bottom: 2.0,
        }
    }
}

/// Padding metrics for message bubble content (`ui/chat/chat.style:L25`).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BubblePadding {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Default for BubblePadding {
    fn default() -> Self {
        // `msgPadding: margins(11px, 8px, 11px, 8px)`
        Self {
            left: 11.0,
            top: 8.0,
            right: 11.0,
            bottom: 8.0,
        }
    }
}

/// Base model for any history view element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoryElementBase {
    pub item_id: u64,
    pub sender_id: u64,
    pub sender_name: String,
    pub date: i64,
    pub is_outgoing: bool,
    pub flags: ElementFlags,
    pub context: ElementContext,
    pub chat_mode: ElementChatMode,
    pub bubble_rounding: BubbleRounding,
    pub margins: ElementMargins,
    pub width: f32,
    pub height: f32,
    pub y_pos: f32,
}

impl HistoryElementBase {
    pub fn new(item_id: u64, sender_id: u64, date: i64, is_outgoing: bool) -> Self {
        Self {
            item_id,
            sender_id,
            sender_name: String::new(),
            date,
            is_outgoing,
            flags: ElementFlags::new(),
            context: ElementContext::History,
            chat_mode: ElementChatMode::Default,
            bubble_rounding: BubbleRounding::for_message(is_outgoing, false, false),
            margins: ElementMargins::for_message(false),
            width: 0.0,
            height: 0.0,
            y_pos: 0.0,
        }
    }

    pub fn set_attached_to_previous(&mut self, attached: bool) {
        self.flags.set(ElementFlags::ATTACHED_TO_PREVIOUS, attached);
        self.flags.set(ElementFlags::BUBBLE_ATTACHED_TO_PREVIOUS, attached);
        self.bubble_rounding = BubbleRounding::for_message(
            self.is_outgoing,
            attached,
            self.flags.contains(ElementFlags::BUBBLE_ATTACHED_TO_NEXT),
        );
        self.margins = ElementMargins::for_message(attached);
    }

    pub fn set_attached_to_next(&mut self, attached: bool) {
        self.flags.set(ElementFlags::ATTACHED_TO_NEXT, attached);
        self.flags.set(ElementFlags::BUBBLE_ATTACHED_TO_NEXT, attached);
        self.bubble_rounding = BubbleRounding::for_message(
            self.is_outgoing,
            self.flags.contains(ElementFlags::BUBBLE_ATTACHED_TO_PREVIOUS),
            attached,
        );
    }
}
