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
pub use crate::views::stage::cards::*;

pub use crate::views::stage::cards::text_card::{
    render_bubble_tail, BubbleRounding, BUBBLE_RADIUS_LARGE, BUBBLE_RADIUS_SMALL,
    K_ATTACH_MESSAGE_TO_PREVIOUS_SECONDS_DELTA, MSG_MAX_WIDTH, MSG_MIN_WIDTH,
    RichSpanKind, RichTextSpan, parse_markdown_spans,
};

pub fn parse_timestamp_seconds(ts: &str) -> Option<i64> {
    let ts = ts.trim();
    if let Ok(secs) = ts.parse::<i64>() {
        return Some(secs);
    }
    let parts: Vec<&str> = ts.split(':').collect();
    if parts.len() == 2 {
        if let (Ok(h), Ok(m)) = (parts[0].parse::<i64>(), parts[1].parse::<i64>()) {
            return Some(h * 3600 + m * 60);
        }
    } else if parts.len() == 3 {
        if let (Ok(h), Ok(m), Ok(s)) = (parts[0].parse::<i64>(), parts[1].parse::<i64>(), parts[2].parse::<i64>()) {
            return Some(h * 3600 + m * 60 + s);
        }
    }
    None
}

/// Compute consecutive message attachment flags (attached_prev, attached_next)
/// based on identical sender, message direction, and 900-second threshold.
pub fn compute_attachment(messages: &[MessageItemModel], index: usize) -> (bool, bool) {
    if index >= messages.len() {
        return (false, false);
    }
    let current = &messages[index];
    if matches!(current.content, MessageContent::Service { .. }) {
        return (false, false);
    }

    let attached_prev = if index > 0 {
        let prev = &messages[index - 1];
        if matches!(prev.content, MessageContent::Service { .. }) {
            false
        } else if prev.sender_id == current.sender_id && prev.direction == current.direction {
            let prev_sec = parse_timestamp_seconds(&prev.timestamp);
            let cur_sec = parse_timestamp_seconds(&current.timestamp);
            match (prev_sec, cur_sec) {
                (Some(p), Some(c)) => (c - p).abs() <= K_ATTACH_MESSAGE_TO_PREVIOUS_SECONDS_DELTA,
                _ => true,
            }
        } else {
            false
        }
    } else {
        false
    };

    let attached_next = if index + 1 < messages.len() {
        let next = &messages[index + 1];
        if matches!(next.content, MessageContent::Service { .. }) {
            false
        } else if next.sender_id == current.sender_id && next.direction == current.direction {
            let next_sec = parse_timestamp_seconds(&next.timestamp);
            let cur_sec = parse_timestamp_seconds(&current.timestamp);
            match (next_sec, cur_sec) {
                (Some(n), Some(c)) => (n - c).abs() <= K_ATTACH_MESSAGE_TO_PREVIOUS_SECONDS_DELTA,
                _ => true,
            }
        } else {
            false
        }
    } else {
        false
    };

    (attached_prev, attached_next)
}

#[derive(Debug, Clone)]
pub struct RowHeightCache {
    pub measured_heights: std::collections::HashMap<u64, f32>,
    pub prefix_sums: Vec<f32>,
    pub total_height: f32,
    pub dirty: bool,
}

impl RowHeightCache {
    pub fn new() -> Self {
        Self {
            measured_heights: std::collections::HashMap::new(),
            prefix_sums: Vec::new(),
            total_height: 0.0,
            dirty: true,
        }
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn len(&self) -> usize {
        self.prefix_sums.len()
    }

    pub fn is_empty(&self) -> bool {
        self.prefix_sums.is_empty()
    }

    pub fn estimate_row_height(msg: &MessageItemModel, attached_prev: bool) -> f32 {
        let margin = if attached_prev { 2.0 } else { 8.0 };
        match &msg.content {
            MessageContent::Service { .. } => 32.0,
            MessageContent::Text { text, .. } => {
                let explicit_lines = text.split('\n').count();
                let char_wrap = text.len() / 42;
                let lines = explicit_lines.max(char_wrap).max(1) as f32;
                let reply_h = if msg.reply_to.is_some() { 28.0 } else { 0.0 };
                let reaction_h = if !msg.reactions.is_empty() { 24.0 } else { 0.0 };
                let bot_kb_h = if msg.bot_keyboard.is_some() { 38.0 } else { 0.0 };
                (22.0 + reply_h + (lines * 18.0) + reaction_h + bot_kb_h + margin).max(42.0)
            }
            MessageContent::Media { caption, .. } => {
                let caption_lines = caption.as_ref().map_or(0, |c| c.split('\n').count()) as f32;
                240.0 + caption_lines * 18.0 + margin
            }
            MessageContent::Voice { .. } => 56.0 + margin,
            MessageContent::Album { .. } => 260.0 + margin,
            MessageContent::Location { .. } => 180.0 + margin,
            MessageContent::Poll { options, .. } => 52.0 + (options.len() as f32 * 34.0) + 36.0 + margin,
            MessageContent::Call { .. } => 60.0 + margin,
            MessageContent::Story { .. } => 220.0 + margin,
            MessageContent::Invoice { .. } => 240.0 + margin,
            MessageContent::BotKeyboard { .. } => 60.0 + margin,
        }
    }

    pub fn rebuild_prefix_sums(&mut self, messages: &[MessageItemModel]) {
        self.prefix_sums.clear();
        self.prefix_sums.reserve(messages.len());
        let mut accum = 0.0;
        for (idx, msg) in messages.iter().enumerate() {
            let h = match self.measured_heights.get(&msg.id) {
                Some(&measured) => measured,
                None => {
                    let attached_prev = idx > 0 && messages[idx - 1].sender_id == msg.sender_id;
                    Self::estimate_row_height(msg, attached_prev)
                }
            };
            accum += h;
            self.prefix_sums.push(accum);
        }
        self.total_height = accum;
        self.dirty = false;
    }

    pub fn visible_range(
        &self,
        scroll_top: f32,
        viewport_height: f32,
        overscan_px: f32,
        total_items: usize,
    ) -> (usize, usize) {
        if total_items == 0 || self.prefix_sums.is_empty() {
            return (0, 0);
        }
        let top_bound = (scroll_top - overscan_px).max(0.0);
        let bottom_bound = scroll_top + viewport_height + overscan_px;

        let start_idx = match self.prefix_sums.binary_search_by(|&val| {
            val.partial_cmp(&top_bound).unwrap_or(std::cmp::Ordering::Equal)
        }) {
            Ok(ix) => ix,
            Err(ix) => ix.min(total_items.saturating_sub(1)),
        };

        let end_idx = match self.prefix_sums.binary_search_by(|&val| {
            val.partial_cmp(&bottom_bound).unwrap_or(std::cmp::Ordering::Equal)
        }) {
            Ok(ix) => ix,
            Err(ix) => ix.min(total_items.saturating_sub(1)),
        };

        (start_idx, end_idx.max(start_idx).min(total_items.saturating_sub(1)))
    }
}

impl Default for RowHeightCache {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessageContextMenuState {
    pub message_id: u64,
    pub position: Point<Pixels>,
    pub is_outgoing: bool,
    pub is_pinned: bool,
    pub text_snippet: Option<String>,
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
    pub chat_histories: std::collections::HashMap<u64, Vec<MessageItemModel>>,
    pub chat_pinned_messages: std::collections::HashMap<u64, Option<String>>,
    pub scroll_handle: ScrollHandle,
    pub row_height_cache: RowHeightCache,
    pub is_anchored_to_bottom: bool,
    pub chat_scroll_positions: std::collections::HashMap<u64, (Point<Pixels>, bool)>,
    pub unread_incoming_count: usize,
    pub active_context_menu: Option<MessageContextMenuState>,
    pub pending_reply: Option<ReplyReference>,
    pub simulated_max_offset: Option<Size<Pixels>>,
    pub selected_message_ids: Vec<u64>,
}

impl ConversationStage {
    pub fn new() -> Self {
        let mut stage = Self {
            active_chat_id: None,
            chat_title: "Select a chat".to_string(),
            chat_subtitle: String::new(),
            chat_avatar_initials: String::new(),
            chat_avatar_color: 0x242F3D,
            pinned_message: None,
            messages: Vec::new(),
            is_inspector_open: false,
            pending_call_request: false,
            pending_media_url: None,
            pending_bot_action: None,
            chat_histories: std::collections::HashMap::new(),
            chat_pinned_messages: std::collections::HashMap::new(),
            scroll_handle: ScrollHandle::new(),
            row_height_cache: RowHeightCache::new(),
            is_anchored_to_bottom: true,
            chat_scroll_positions: std::collections::HashMap::new(),
            unread_incoming_count: 0,
            active_context_menu: None,
            pending_reply: None,
            simulated_max_offset: None,
            selected_message_ids: Vec::new(),
        };
        stage.row_height_cache.rebuild_prefix_sums(&stage.messages);
        stage
    }

    /// Explicit fixture constructor for legacy card/layout tests. Never called by production code.
    pub fn new_demo() -> Self {
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
            chat_pinned_messages: std::collections::HashMap::new(),
            scroll_handle: ScrollHandle::new(),
            row_height_cache: RowHeightCache::new(),
            is_anchored_to_bottom: true,
            chat_scroll_positions: std::collections::HashMap::new(),
            unread_incoming_count: 0,
            active_context_menu: None,
            pending_reply: None,
            simulated_max_offset: None,
            selected_message_ids: Vec::new(),
        };

        stage.populate_demo_messages();
        stage.chat_histories.insert(1, stage.messages.clone());
        stage.chat_pinned_messages.insert(1, stage.pinned_message.clone());
        stage.populate_secondary_chat_histories();
        stage.row_height_cache.rebuild_prefix_sums(&stage.messages);
        stage
    }

    pub fn clear_selection(&mut self) {
        self.active_chat_id = None;
        self.chat_title = "Select a chat".to_string();
        self.chat_subtitle.clear();
        self.chat_avatar_initials.clear();
        self.chat_avatar_color = 0x242F3D;
        self.pinned_message = None;
        self.messages.clear();
        self.is_inspector_open = false;
        self.pending_call_request = false;
        self.pending_media_url = None;
        self.pending_bot_action = None;
        self.unread_incoming_count = 0;
        self.active_context_menu = None;
        self.pending_reply = None;
        self.simulated_max_offset = None;
        self.scroll_handle.set_offset(Point::default());
        self.is_anchored_to_bottom = true;
        self.row_height_cache.measured_heights.clear();
        self.row_height_cache.rebuild_prefix_sums(&self.messages);
    }

    pub fn reset(&mut self) {
        self.clear_selection();
        self.chat_histories.clear();
        self.chat_pinned_messages.clear();
        self.chat_scroll_positions.clear();
    }

    pub fn configure_for_cloud_workspace(&mut self, _server_url: &str, _username: Option<&str>) {
        self.reset();
    }

    pub fn set_chat(&mut self, chat_id: u64, title: &str, subtitle: &str, initials: &str, color: u32) {
        if let Some(prev_id) = self.active_chat_id {
            self.chat_histories.insert(prev_id, self.messages.clone());
            self.chat_pinned_messages.insert(prev_id, self.pinned_message.clone());
            self.chat_scroll_positions.insert(prev_id, (self.scroll_handle.offset(), self.is_at_bottom()));
        }
        if self.active_chat_id != Some(chat_id) {
            self.active_context_menu = None;
            self.pending_reply = None;
        }
        self.active_chat_id = Some(chat_id);
        self.chat_title = title.to_string();
        self.chat_subtitle = subtitle.to_string();
        self.chat_avatar_initials = initials.to_string();
        self.chat_avatar_color = color;

        // 1. Cleanly load history without cross-chat bleeding
        self.messages = self.chat_histories.get(&chat_id).cloned().unwrap_or_default();
        self.row_height_cache.mark_dirty();

        // 2. Cleanly load pinned banner without cross-chat bleeding
        self.pinned_message = self.chat_pinned_messages.get(&chat_id).cloned().unwrap_or_else(|| {
            self.messages.iter().find(|m| m.is_pinned).and_then(|m| match &m.content {
                MessageContent::Text { text, .. } => Some(text.lines().next().unwrap_or(text).to_string()),
                _ => None,
            })
        });

        // 3. Restore scroll position or scroll to bottom
        if let Some(&(saved_offset, was_at_bottom)) = self.chat_scroll_positions.get(&chat_id) {
            if !was_at_bottom {
                self.scroll_handle.set_offset(saved_offset);
                self.is_anchored_to_bottom = false;
            } else {
                self.scroll_to_bottom();
            }
        } else {
            self.scroll_to_bottom();
        }
    }

    pub fn toggle_inspector(&mut self) {
        self.is_inspector_open = !self.is_inspector_open;
    }

    pub fn add_message(&mut self, message: MessageItemModel) {
        if message.is_pinned {
            if let MessageContent::Text { ref text, .. } = message.content {
                let snippet = text.lines().next().unwrap_or(text).to_string();
                self.pinned_message = Some(snippet);
                if let Some(cid) = self.active_chat_id {
                    self.chat_pinned_messages.insert(cid, self.pinned_message.clone());
                }
            }
        }
        let is_outgoing = message.direction == MessageDirection::Outgoing;
        let was_at_bottom = self.is_at_bottom();

        self.messages.push(message.clone());
        if let Some(cid) = self.active_chat_id {
            self.chat_histories.entry(cid).or_default().push(message);
        }
        self.row_height_cache.mark_dirty();

        if is_outgoing || was_at_bottom {
            self.scroll_to_bottom();
        } else {
            self.unread_incoming_count += 1;
        }
    }

    pub fn add_outgoing_text_message(&mut self, text: &str) {
        let next_id = self.messages.iter().map(|m| m.id).max().unwrap_or(0) + 1;
        self.add_message(MessageItemModel {
            id: next_id,
            sender_id: 1,
            sender_name: "Me".to_string(),
            avatar_initials: "ME".to_string(),
            avatar_color: 0x2AABEE,
            direction: MessageDirection::Outgoing,
            timestamp: "12:00".to_string(),
            delivery_status: DeliveryStatus::Sent,
            reply_to: None,
            reactions: vec![],
            content: MessageContent::Text {
                text: text.to_string(),
                is_edited: false,
            },
            bot_keyboard: None,
            is_pinned: false,
        });
    }

    pub fn add_incoming_message(&mut self, sender_id: u64, text: &str) {
        let next_id = self.messages.iter().map(|m| m.id).max().unwrap_or(0) + 1;
        self.add_message(MessageItemModel {
            id: next_id,
            sender_id,
            sender_name: "Peer".to_string(),
            avatar_initials: "PR".to_string(),
            avatar_color: 0x9B59B6,
            direction: MessageDirection::Incoming,
            timestamp: "12:01".to_string(),
            delivery_status: DeliveryStatus::Read,
            reply_to: None,
            reactions: vec![],
            content: MessageContent::Text {
                text: text.to_string(),
                is_edited: false,
            },
            bot_keyboard: None,
            is_pinned: false,
        });
    }

    /// Merges server-fetched historical messages with existing local messages,
    /// deduplicating by message ID while preserving local pending/sending messages.
    pub fn merge_historical_messages(&mut self, server_messages: Vec<MessageItemModel>) {
        if self.messages.is_empty() {
            self.messages = server_messages;
            return;
        }

        // Keep local sending messages (id not yet confirmed by server)
        let sending_messages: Vec<MessageItemModel> = self
            .messages
            .iter()
            .filter(|m| m.delivery_status == DeliveryStatus::Sending)
            .cloned()
            .collect();

        let mut existing_ids: std::collections::HashSet<u64> =
            self.messages.iter().map(|m| m.id).collect();
        let mut merged = self.messages.clone();

        for s_msg in server_messages {
            if !existing_ids.contains(&s_msg.id) {
                existing_ids.insert(s_msg.id);
                merged.push(s_msg);
            }
        }

        // Re-append local sending items at the bottom
        for sending in sending_messages {
            if !existing_ids.contains(&sending.id) {
                merged.push(sending);
            }
        }

        self.messages = merged;
    }

    pub fn distance_from_bottom(&self) -> Pixels {
        let max_y = self.max_scroll_offset().height;
        let cur_y = -self.scroll_handle.offset().y;
        (max_y - cur_y).max(px(0.0))
    }

    pub fn is_at_bottom(&self) -> bool {
        let max_y = self.max_scroll_offset().height;
        if max_y <= px(0.0) {
            return true;
        }
        self.distance_from_bottom() <= px(48.0)
    }

    pub fn scroll_to_bottom(&mut self) {
        self.scroll_handle.scroll_to_bottom();
        if let Some(sim) = self.simulated_max_offset {
            self.scroll_handle.set_offset(Point::new(px(0.0), -sim.height));
        }
        self.is_anchored_to_bottom = true;
        self.unread_incoming_count = 0;
    }

    pub fn set_simulated_scroll_offset(&mut self, offset: Point<Pixels>, max_offset: Size<Pixels>) {
        self.scroll_handle.set_offset(offset);
        self.simulated_max_offset = Some(max_offset);
    }

    pub fn max_scroll_offset(&self) -> Size<Pixels> {
        if let Some(sim) = self.simulated_max_offset {
            sim
        } else {
            self.scroll_handle.max_offset()
        }
    }

    pub fn copy_message_text(&self, msg_id: u64) -> Option<String> {
        self.messages.iter().find(|m| m.id == msg_id).and_then(|m| match &m.content {
            MessageContent::Text { text, .. } => Some(text.clone()),
            MessageContent::Media { caption, .. } => caption.clone(),
            _ => None,
        })
    }

    pub fn toggle_pin_message(&mut self, msg_id: u64) {
        let mut new_pinned_state = false;
        let mut snippet_opt = None;
        if let Some(m) = self.messages.iter_mut().find(|m| m.id == msg_id) {
            m.is_pinned = !m.is_pinned;
            new_pinned_state = m.is_pinned;
            if new_pinned_state {
                snippet_opt = match &m.content {
                    MessageContent::Text { text, .. } => Some(text.lines().next().unwrap_or(text).to_string()),
                    _ => Some("Pinned message".to_string()),
                };
            }
        }
        if new_pinned_state {
            self.pinned_message = snippet_opt;
        } else {
            self.pinned_message = self.messages.iter().find(|m| m.is_pinned).and_then(|m| match &m.content {
                MessageContent::Text { text, .. } => Some(text.lines().next().unwrap_or(text).to_string()),
                _ => Some("Pinned message".to_string()),
            });
        }
        if let Some(cid) = self.active_chat_id {
            self.chat_pinned_messages.insert(cid, self.pinned_message.clone());
            if let Some(hist) = self.chat_histories.get_mut(&cid) {
                if let Some(m) = hist.iter_mut().find(|m| m.id == msg_id) {
                    m.is_pinned = new_pinned_state;
                }
            }
        }
    }

    pub fn delete_message(&mut self, msg_id: u64) {
        self.messages.retain(|m| m.id != msg_id);
        if let Some(cid) = self.active_chat_id {
            if let Some(hist) = self.chat_histories.get_mut(&cid) {
                hist.retain(|m| m.id != msg_id);
            }
        }
        self.row_height_cache.mark_dirty();
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

    pub fn compute_attachment(&self, idx: usize) -> (bool, bool) {
        compute_attachment(&self.messages, idx)
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
    pub fn edit_message_text(&mut self, message_id: u64, new_text: String) {
        if let Some(msg) = self.messages.iter_mut().find(|m| m.id == message_id) {
            msg.content = MessageContent::Text {
                text: new_text,
                is_edited: true,
            };
        }
        if let Some(chat_id) = self.active_chat_id {
            if let Some(hist) = self.chat_histories.get_mut(&chat_id) {
                if let Some(msg) = hist.iter_mut().find(|m| m.id == message_id) {
                    msg.content = MessageContent::Text {
                        text: match &self.messages.iter().find(|m| m.id == message_id).map(|m| &m.content) {
                            Some(MessageContent::Text { text, .. }) => text.clone(),
                            _ => String::new(),
                        },
                        is_edited: true,
                    };
                }
            }
        }
        self.row_height_cache.dirty = true;
    }

    pub fn toggle_select_message(&mut self, msg_id: u64) {
        if let Some(pos) = self.selected_message_ids.iter().position(|&x| x == msg_id) {
            self.selected_message_ids.remove(pos);
        } else {
            self.selected_message_ids.push(msg_id);
        }
    }

    pub fn delete_messages_by_ids(&mut self, ids: &[u64]) {
        self.messages.retain(|m| !ids.contains(&m.id));
        self.selected_message_ids.retain(|id| !ids.contains(id));
        if let Some(chat_id) = self.active_chat_id {
            if let Some(hist) = self.chat_histories.get_mut(&chat_id) {
                hist.retain(|m| !ids.contains(&m.id));
            }
        }
        self.row_height_cache.rebuild_prefix_sums(&self.messages);
    }
}

impl Default for ConversationStage {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for ConversationStage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let has_active_chat = self.active_chat_id.is_some();
        let title = self.chat_title.clone();
        let subtitle = self.chat_subtitle.clone();
        let initials = self.chat_avatar_initials.clone();
        let avatar_color = self.chat_avatar_color;
        let pinned = self.pinned_message.clone();
        let is_inspector_open = self.is_inspector_open;

        let total_msgs = self.messages.len();
        if self.row_height_cache.is_dirty() || self.row_height_cache.len() != total_msgs {
            self.row_height_cache.rebuild_prefix_sums(&self.messages);
        }

        let viewport_h = f32::from(self.scroll_handle.bounds().size.height).max(600.0);
        let scroll_top = f32::from(-self.scroll_handle.offset().y).max(0.0);
        let overscan_px = 300.0;

        let (start_idx, end_idx) = self.row_height_cache.visible_range(scroll_top, viewport_h, overscan_px, total_msgs);

        let top_spacer_height = if start_idx > 0 && !self.row_height_cache.prefix_sums.is_empty() {
            self.row_height_cache.prefix_sums[(start_idx - 1).min(self.row_height_cache.prefix_sums.len() - 1)]
        } else {
            0.0
        };

        let bottom_spacer_height = if end_idx < total_msgs.saturating_sub(1) && !self.row_height_cache.prefix_sums.is_empty() {
            (self.row_height_cache.total_height - self.row_height_cache.prefix_sums[end_idx.min(self.row_height_cache.prefix_sums.len() - 1)]).max(0.0)
        } else {
            0.0
        };

        let mut message_rows = Vec::with_capacity(if total_msgs == 0 { 0 } else { end_idx - start_idx + 1 });
        if total_msgs > 0 {
            for idx in start_idx..=end_idx.min(total_msgs.saturating_sub(1)) {
                let (attached_prev, attached_next) = compute_attachment(&self.messages, idx);
                message_rows.push(Self::render_interactive_message_row(
                    &self.messages[idx],
                    attached_prev,
                    attached_next,
                    cx,
                ));
            }
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
                    // Peer Info (Avatar + Title + Subtitle)
                    .child(
                        div()
                            .when(has_active_chat, |el| {
                                el.cursor_pointer().on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                    this.toggle_inspector();
                                    cx.notify();
                                }))
                            })
                            .flex()
                            .items_center()
                            .gap(px(12.0))
                            .when(has_active_chat || !initials.is_empty(), |el| {
                                el.child(
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
                            })
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(if has_active_chat { rgb(0xFFFFFF) } else { rgb(0x8A9AA8) })
                                            .child(title),
                                    )
                                    .when(!subtitle.is_empty(), |el| {
                                        el.child(
                                            div()
                                                .text_size(px(12.0))
                                                .text_color(rgb(0x8A9AA8))
                                                .child(subtitle),
                                        )
                                    }),
                            ),
                    )
                    // Action Icons (Audio Call, Video Call, Remote Computer, Drawer Toggle)
                    .when(has_active_chat, |el| {
                        el.child(
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
                        )
                    }),
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
                    .track_scroll(&self.scroll_handle)
                    .flex()
                    .flex_col()
                    .gap(px(0.0))
                    .when(!has_active_chat && total_msgs == 0, |el| {
                        el.child(
                            div()
                                .flex_1()
                                .w_full()
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(
                                    div()
                                        .px(px(14.0))
                                        .py(px(6.0))
                                        .rounded_full()
                                        .bg(rgb(0x182533))
                                        .text_size(px(13.0))
                                        .text_color(rgb(0x8A9AA8))
                                        .child("Select a chat"),
                                ),
                        )
                    })
                    .when(top_spacer_height > 0.0, |el| {
                        el.child(div().id("stage-top-spacer").w_full().h(px(top_spacer_height)).flex_shrink_0())
                    })
                    .children(message_rows)
                    .when(bottom_spacer_height > 0.0, |el| {
                        el.child(div().id("stage-bottom-spacer").w_full().h(px(bottom_spacer_height)).flex_shrink_0())
                    }),
            )
            .when_some(self.active_context_menu.clone(), |el, menu| {
                let msg_id = menu.message_id;
                let is_pinned = menu.is_pinned;
                let text = menu.text_snippet.clone();

                el.child(
                    div()
                        .absolute()
                        .inset_0()
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                            this.active_context_menu = None;
                            cx.notify();
                        }))
                        .child(
                            div()
                                .absolute()
                                .left(menu.position.x)
                                .top(menu.position.y)
                                .w(px(160.0))
                                .bg(rgb(0x212D3B))
                                .border_1()
                                .border_color(rgb(0x2B5278))
                                .rounded(px(8.0))
                                .shadow_lg()
                                .py(px(4.0))
                                .flex()
                                .flex_col()
                                .gap(px(1.0))
                                .when(text.is_some(), |m| {
                                    let t = text.unwrap();
                                    m.child(Self::render_menu_item("📋", "Copy Text", false, cx.listener(move |this, _, _, cx| {
                                        cx.write_to_clipboard(ClipboardItem::new_string(t.clone()));
                                        this.active_context_menu = None;
                                        cx.notify();
                                    })))
                                })
                                .child(Self::render_menu_item("↩️", "Reply", false, cx.listener(move |this, _, _, cx| {
                                    let sender_name = this.messages.iter().find(|m| m.id == msg_id).map(|m| m.sender_name.clone()).unwrap_or_else(|| "User".to_string());
                                    let snippet = this.messages.iter().find(|m| m.id == msg_id).map(|m| match &m.content {
                                        MessageContent::Text { text, .. } => text.lines().next().unwrap_or(text).to_string(),
                                        _ => "Message".to_string(),
                                    }).unwrap_or_default();
                                    this.pending_reply = Some(ReplyReference {
                                        message_id: msg_id,
                                        sender_name,
                                        snippet,
                                    });
                                    this.active_context_menu = None;
                                    cx.notify();
                                })))
                                .child(Self::render_menu_item("↪️", "Forward", false, cx.listener(move |this, _, _, cx| {
                                    this.active_context_menu = None;
                                    cx.notify();
                                })))
                                .child(Self::render_menu_item(
                                    "📌",
                                    if is_pinned { "Unpin" } else { "Pin" },
                                    false,
                                    cx.listener(move |this, _, _, cx| {
                                        this.toggle_pin_message(msg_id);
                                        this.active_context_menu = None;
                                        cx.notify();
                                    })
                                ))
                                .child(Self::render_menu_item("🗑️", "Delete", true, cx.listener(move |this, _, _, cx| {
                                    this.delete_message(msg_id);
                                    this.active_context_menu = None;
                                    cx.notify();
                                })))
                        )
                )
            })
    }
}

impl ConversationStage {
    fn render_menu_item(
        icon: &'static str,
        label: &'static str,
        is_danger: bool,
        handler: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    ) -> impl IntoElement {
        div()
            .cursor_pointer()
            .px(px(12.0))
            .py(px(6.0))
            .flex()
            .items_center()
            .gap(px(8.0))
            .hover(|s| s.bg(rgb(0x2B5278)))
            .on_mouse_down(MouseButton::Left, handler)
            .child(div().text_size(px(13.0)).child(icon))
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(if is_danger { rgb(0xE05D5D) } else { rgb(0xFFFFFF) })
                    .child(label),
            )
    }

    fn render_interactive_message_row(
        msg: &MessageItemModel,
        attached_prev: bool,
        attached_next: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
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
        let is_invoice_msg = matches!(msg.content, MessageContent::Invoice { .. });
        let rounding = BubbleRounding::compute(is_outgoing, attached_prev, attached_next);
        let bubble = Self::render_message_bubble(msg, rounding, cx);

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
            div()
                .w_full()
                .flex_shrink_0()
                .flex()
                .justify_end()
                .mt(px(if attached_prev { 0.0 } else { 6.0 }))
                .mb(px(2.0))
        } else {
            div()
                .w_full()
                .flex_shrink_0()
                .flex()
                .justify_start()
                .mt(px(if attached_prev { 0.0 } else { 6.0 }))
                .mb(px(2.0))
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
                                    || is_invoice_msg,
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
                                 }
                             }))
                            .on_mouse_down(MouseButton::Right, cx.listener(move |this, event: &MouseDownEvent, _window, cx| {
                                let is_pinned = this.messages.iter().find(|m| m.id == msg_id).map(|m| m.is_pinned).unwrap_or(false);
                                let text_snippet = this.messages.iter().find(|m| m.id == msg_id).and_then(|m| match &m.content {
                                    MessageContent::Text { text, .. } => Some(text.clone()),
                                    MessageContent::Media { caption, .. } => caption.clone(),
                                    _ => None,
                                });
                                this.active_context_menu = Some(MessageContextMenuState {
                                    message_id: msg_id,
                                    position: event.position,
                                    is_outgoing,
                                    is_pinned,
                                    text_snippet,
                                });
                                cx.notify();
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

    fn render_message_bubble(
        msg: &MessageItemModel,
        rounding: BubbleRounding,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        match &msg.content {
            MessageContent::Text { text, is_edited } => {
                TextMessageView::render(msg, text, *is_edited, rounding).into_any_element()
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
                rounding,
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
                rounding,
            ).into_any_element(),
            MessageContent::Album { items, caption } => {
                AlbumMessageView::render(msg, items, caption.as_deref(), rounding).into_any_element()
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
