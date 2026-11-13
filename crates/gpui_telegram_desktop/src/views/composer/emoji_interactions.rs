//! Interactive Full-Screen Animated Emoji Synchronization
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/chat_helpers/emoji_interactions.h:L1-L144`
//! - `Telegram/SourceFiles/chat_helpers/emoji_interactions.cpp:L1-L546`
//! - `Telegram/SourceFiles/chat_helpers/emoji_interactions.cpp:L33-L39` (`kMinDelay`, `kAccumulateDelay`, `kJsonVersion`)
//!
//! Subsystem: chat_helpers, Feature: TCG-N11

use std::collections::HashMap;
use std::time::Duration;

pub const K_MIN_DELAY: Duration = Duration::from_millis(200);
pub const K_ACCUMULATE_DELAY: Duration = Duration::from_millis(1000);
pub const K_ACCEPT_SEEN_SINCE_REQUEST: Duration = Duration::from_millis(3000);
pub const K_MAX_DELAY: Duration = Duration::from_millis(2000);
pub const K_JSON_VERSION: i32 = 1;

/// Single emoji interaction animation descriptor
#[derive(Debug, Clone, PartialEq)]
pub struct InteractionAnimation {
    pub emoticon: String,
    pub document_id: u64,
    pub index: usize,
    pub scheduled_at_ms: i64,
}

/// Emoji Interactions State Machine managing animated full-screen emoji triggers in 1-on-1 chats.
/// Target symbol in ledger: `EmojiInteractions`.
#[derive(Debug, Clone)]
pub struct EmojiInteractions {
    pub outgoing_animations: HashMap<i64, Vec<InteractionAnimation>>, // message_id -> animations
    pub incoming_animations: HashMap<i64, Vec<InteractionAnimation>>,
    pub seen_requests: Vec<(i64, i64)>, // (message_id, timestamp_ms)
    pub is_playing: bool,
    pub last_play_at_ms: i64,
}

impl Default for EmojiInteractions {
    fn default() -> Self {
        Self {
            outgoing_animations: HashMap::new(),
            incoming_animations: HashMap::new(),
            seen_requests: Vec::new(),
            is_playing: false,
            last_play_at_ms: 0,
        }
    }
}

impl EmojiInteractions {
    pub fn new() -> Self {
        Self::default()
    }

    /// Trigger outgoing animated emoji interaction.
    /// Deduplicates consecutive animations from the available variant list per `emoji_interactions.cpp:L104-L113`.
    pub fn start_outgoing(
        &mut self,
        message_id: i64,
        emoticon: &str,
        available_variants: &[u64],
        now_ms: i64,
    ) -> Option<InteractionAnimation> {
        if available_variants.is_empty() {
            return None;
        }

        let animations = self.outgoing_animations.entry(message_id).or_default();
        let last_doc = animations.last().map(|a| a.document_id);

        let count = available_variants.len();
        let selected_doc = if let Some(last) = last_doc {
            if count > 1 {
                available_variants
                    .iter()
                    .copied()
                    .find(|&doc| doc != last)
                    .unwrap_or(available_variants[0])
            } else {
                available_variants[0]
            }
        } else {
            available_variants[0]
        };

        let anim = InteractionAnimation {
            emoticon: emoticon.to_string(),
            document_id: selected_doc,
            index: animations.len(),
            scheduled_at_ms: now_ms,
        };

        animations.push(anim.clone());
        self.last_play_at_ms = now_ms;
        self.is_playing = true;

        Some(anim)
    }

    /// Record incoming seen request from peer
    pub fn record_seen(&mut self, message_id: i64, timestamp_ms: i64) {
        self.seen_requests.push((message_id, timestamp_ms));
    }

    /// Check if interaction should be purged on message edit or deletion
    pub fn on_message_destroyed(&mut self, message_id: i64) {
        self.outgoing_animations.remove(&message_id);
        self.incoming_animations.remove(&message_id);
    }
}
