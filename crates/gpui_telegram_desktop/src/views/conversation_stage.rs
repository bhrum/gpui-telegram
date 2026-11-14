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

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use gpui::*;
use gpui::prelude::FluentBuilder;

use crate::i18n::tr_key_or;
use crate::theme::TdesktopThemePalette;
use crate::views::stage::history_drag_area::{DragTargetZone, HistoryDragArea};
use crate::views::stage::history_view_highlight_manager::{
    HistoryViewHighlightManager, SelectedQuote,
};
use crate::views::stage::pinned_bar::PinnedTracker;
use crate::views::stage::rich_meta::MessageRichMeta;
use crate::views::stage::stage_bars::{
    render_pinned_banner_46px, render_scroll_to_bottom_fab, render_stage_top_bar, TopBarUiAction,
};
use crate::views::stage::stage_bubble::{render_message_row_1to1, BubbleGroupLayout, BubbleUiAction};
use crate::views::stage::stage_context_menu::{render_message_context_menu, ContextMenuAction};
use crate::views::stage::wallpaper::{
    render_empty_history_placeholder, render_empty_no_chat_pill, render_themed_wallpaper,
};
use crate::views::stage::cards_interactive::InteractiveCardAction;
use crate::views::stage::cards_media::MediaCardAction;
use crate::views::stage::entities_render::RichTextAction;
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
    pub chat_avatar_url: Option<String>,
    pub pinned_message: Option<String>,
    pub messages: Vec<MessageItemModel>,
    pub is_inspector_open: bool,
    pub pending_call_request: bool,
    pub pending_video_call: bool,
    pub pending_media_url: Option<(String, bool)>,
    pub pending_media_viewer: Option<(Vec<(String, bool, Option<String>)>, usize)>,
    pub pending_iv_url: Option<(String, String)>,
    pub pending_bot_action: Option<String>,
    pub pending_edit_message: Option<(u64, String)>,
    pub pending_forward_message_ids: Option<Vec<u64>>,
    pub pending_delete_message_ids: Option<Vec<u64>>,
    pub pending_pin_toggle: Option<(u64, bool)>,
    pub pending_translate_message_id: Option<u64>,
    pub pending_poll_vote: Option<(u64, u32)>,
    pub pending_reaction_toggle: Option<(u64, String)>,
    pub pending_voice_playback: Option<(u64, bool, f32, f32)>,
    pub pending_download_url: Option<(String, String)>,
    pub pending_dropped_paths: Option<Vec<PathBuf>>,
    pub pending_open_profile_peer_id: Option<u64>,
    pub pending_clear_history: bool,
    pub pending_export_chat: bool,
    pub pending_toggle_mute: bool,
    pub chat_histories: std::collections::HashMap<u64, Vec<MessageItemModel>>,
    pub chat_pinned_messages: std::collections::HashMap<u64, Option<String>>,
    pub scroll_handle: ScrollHandle,
    pub row_height_cache: RowHeightCache,
    pub is_anchored_to_bottom: bool,
    pub chat_scroll_positions: std::collections::HashMap<u64, (Point<Pixels>, bool)>,
    pub unread_incoming_count: usize,
    pub unread_anchor_msg_id: Option<u64>,
    pub active_context_menu: Option<MessageContextMenuState>,
    pub pending_reply: Option<ReplyReference>,
    pub simulated_max_offset: Option<Size<Pixels>>,
    pub selected_message_ids: Vec<u64>,
    pub palette: TdesktopThemePalette,
    pub lang_id: String,
    pub message_meta: HashMap<u64, MessageRichMeta>,
    pub pinned_tracker: PinnedTracker,
    pub highlight_manager: HistoryViewHighlightManager,
    pub drag_area: HistoryDragArea,
    pub focus_handle: Option<FocusHandle>,
    pub search_open: bool,
    pub search_query: String,
    pub search_match_ids: Vec<u64>,
    pub search_match_cursor: usize,
    pub hovered_message_id: Option<u64>,
    pub revealed_spoilers: HashSet<(u64, usize)>,
    pub expanded_quotes: HashSet<(u64, usize)>,
    pub copied_code_msg: Option<u64>,
    pub more_menu_open: bool,
    pub toast_banner: Option<String>,
}

fn delivery_status_rank(status: DeliveryStatus) -> u8 {
    match status {
        DeliveryStatus::Sending => 0,
        DeliveryStatus::Sent => 1,
        DeliveryStatus::Read => 2,
    }
}

fn max_delivery_status(a: DeliveryStatus, b: DeliveryStatus) -> DeliveryStatus {
    if delivery_status_rank(a) >= delivery_status_rank(b) {
        a
    } else {
        b
    }
}

fn merge_message_in_place(existing: &mut MessageItemModel, incoming: &MessageItemModel) {
    existing.delivery_status = max_delivery_status(existing.delivery_status, incoming.delivery_status);
    if !incoming.sender_name.is_empty() {
        existing.sender_name = incoming.sender_name.clone();
    }
    if !incoming.avatar_initials.is_empty() {
        existing.avatar_initials = incoming.avatar_initials.clone();
    }
    if !incoming.timestamp.is_empty() {
        existing.timestamp = incoming.timestamp.clone();
    }
    if incoming.reply_to.is_some() {
        existing.reply_to = incoming.reply_to.clone();
    }
    if !incoming.reactions.is_empty() {
        existing.reactions = incoming.reactions.clone();
    }
    existing.content = incoming.content.clone();
    if incoming.bot_keyboard.is_some() {
        existing.bot_keyboard = incoming.bot_keyboard.clone();
    }
    existing.is_pinned = existing.is_pinned || incoming.is_pinned;
}

impl ConversationStage {
    pub fn new() -> Self {
        let palette = TdesktopThemePalette::default();
        let mut stage = Self {
            active_chat_id: None,
            chat_title: "Select a chat".to_string(),
            chat_subtitle: String::new(),
            chat_avatar_initials: String::new(),
            chat_avatar_color: palette.input_bg,
            chat_avatar_url: None,
            pinned_message: None,
            messages: Vec::new(),
            is_inspector_open: false,
            pending_call_request: false,
            pending_video_call: false,
            pending_media_url: None,
            pending_media_viewer: None,
            pending_iv_url: None,
            pending_bot_action: None,
            pending_edit_message: None,
            pending_forward_message_ids: None,
            pending_delete_message_ids: None,
            pending_pin_toggle: None,
            pending_translate_message_id: None,
            pending_poll_vote: None,
            pending_reaction_toggle: None,
            pending_voice_playback: None,
            pending_download_url: None,
            pending_dropped_paths: None,
            pending_open_profile_peer_id: None,
            pending_clear_history: false,
            pending_export_chat: false,
            pending_toggle_mute: false,
            chat_histories: std::collections::HashMap::new(),
            chat_pinned_messages: std::collections::HashMap::new(),
            scroll_handle: ScrollHandle::new(),
            row_height_cache: RowHeightCache::new(),
            is_anchored_to_bottom: true,
            chat_scroll_positions: std::collections::HashMap::new(),
            unread_incoming_count: 0,
            unread_anchor_msg_id: None,
            active_context_menu: None,
            pending_reply: None,
            simulated_max_offset: None,
            selected_message_ids: Vec::new(),
            palette,
            lang_id: "en".to_string(),
            message_meta: HashMap::new(),
            pinned_tracker: PinnedTracker::new(false),
            highlight_manager: HistoryViewHighlightManager::new(),
            drag_area: HistoryDragArea::new(),
            focus_handle: None,
            search_open: false,
            search_query: String::new(),
            search_match_ids: Vec::new(),
            search_match_cursor: 0,
            hovered_message_id: None,
            revealed_spoilers: HashSet::new(),
            expanded_quotes: HashSet::new(),
            copied_code_msg: None,
            more_menu_open: false,
            toast_banner: None,
        };
        stage.row_height_cache.rebuild_prefix_sums(&stage.messages);
        stage
    }

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

    pub fn clear_selection(&mut self) {
        self.active_chat_id = None;
        self.chat_title = "Select a chat".to_string();
        self.chat_subtitle.clear();
        self.chat_avatar_initials.clear();
        self.chat_avatar_color = self.palette.input_bg;
        self.chat_avatar_url = None;
        self.pinned_message = None;
        self.messages.clear();
        self.is_inspector_open = false;
        self.pending_call_request = false;
        self.pending_video_call = false;
        self.pending_media_url = None;
        self.pending_media_viewer = None;
        self.pending_iv_url = None;
        self.pending_bot_action = None;
        self.pending_edit_message = None;
        self.pending_forward_message_ids = None;
        self.pending_delete_message_ids = None;
        self.pending_pin_toggle = None;
        self.pending_translate_message_id = None;
        self.pending_poll_vote = None;
        self.pending_reaction_toggle = None;
        self.pending_voice_playback = None;
        self.pending_download_url = None;
        self.pending_dropped_paths = None;
        self.pending_open_profile_peer_id = None;
        self.pending_clear_history = false;
        self.pending_export_chat = false;
        self.pending_toggle_mute = false;
        self.unread_incoming_count = 0;
        self.unread_anchor_msg_id = None;
        self.active_context_menu = None;
        self.pending_reply = None;
        self.simulated_max_offset = None;
        self.selected_message_ids.clear();
        self.pinned_tracker.set_pinned_ids(Vec::new());
        self.search_open = false;
        self.search_query.clear();
        self.search_match_ids.clear();
        self.search_match_cursor = 0;
        self.hovered_message_id = None;
        self.more_menu_open = false;
        self.toast_banner = None;
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
        self.message_meta.clear();
        self.revealed_spoilers.clear();
        self.expanded_quotes.clear();
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
        self.active_context_menu = None;
        self.selected_message_ids.clear();
        self.more_menu_open = false;
        self.search_open = false;
        self.search_query.clear();
        self.search_match_ids.clear();
        self.search_match_cursor = 0;

        // 1. Cleanly load history without cross-chat bleeding
        self.messages = self.chat_histories.get(&chat_id).cloned().unwrap_or_default();
        self.row_height_cache.mark_dirty();

        // 2. Cleanly load pinned banner without cross-chat bleeding + sync multi-pin tracker
        let pinned_ids: Vec<i64> = self
            .messages
            .iter()
            .filter(|m| m.is_pinned)
            .map(|m| m.id as i64)
            .collect();
        self.pinned_tracker.set_pinned_ids(pinned_ids);
        self.pinned_message = self.chat_pinned_messages.get(&chat_id).cloned().unwrap_or_else(|| {
            self.messages.iter().rev().find(|m| m.is_pinned).and_then(|m| match &m.content {
                MessageContent::Text { text, .. } => Some(text.lines().next().unwrap_or(text).to_string()),
                MessageContent::Media { caption, .. } | MessageContent::Album { caption, .. } => caption.clone(),
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
            let mut ids: Vec<i64> = self
                .messages
                .iter()
                .filter(|m| m.is_pinned)
                .map(|m| m.id as i64)
                .collect();
            if !ids.contains(&(message.id as i64)) {
                ids.push(message.id as i64);
            }
            self.pinned_tracker.set_pinned_ids(ids);
            if let MessageContent::Text { ref text, .. } = message.content {
                let snippet = text.lines().next().unwrap_or(text).to_string();
                self.pinned_message = Some(snippet);
                if let Some(cid) = self.active_chat_id {
                    self.chat_pinned_messages.insert(cid, self.pinned_message.clone());
                }
            }
        }
        if let Some(pos) = self.messages.iter().position(|m| m.id == message.id) {
            merge_message_in_place(&mut self.messages[pos], &message);
            if let Some(cid) = self.active_chat_id {
                let hist = self.chat_histories.entry(cid).or_default();
                if let Some(hpos) = hist.iter().position(|m| m.id == message.id) {
                    merge_message_in_place(&mut hist[hpos], &message);
                } else {
                    hist.push(self.messages[pos].clone());
                }
            }
            self.row_height_cache.mark_dirty();
            return;
        }

        let is_outgoing = message.direction == MessageDirection::Outgoing;
        let was_at_bottom = self.is_at_bottom();
        if !is_outgoing && !was_at_bottom && self.unread_anchor_msg_id.is_none() {
            self.unread_anchor_msg_id = Some(message.id);
        }

        self.messages.push(message.clone());
        if let Some(cid) = self.active_chat_id {
            self.chat_histories.entry(cid).or_default().push(message);
        }
        self.row_height_cache.mark_dirty();

        if is_outgoing || was_at_bottom {
            self.unread_anchor_msg_id = None;
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
        if self.focus_handle.is_none() {
            self.focus_handle = Some(cx.focus_handle());
        }
        let focus = self.focus_handle.clone().unwrap();

        let has_active_chat = self.active_chat_id.is_some();
        let total_msgs = self.messages.len();
        if self.row_height_cache.is_dirty() || self.row_height_cache.len() != total_msgs {
            self.row_height_cache.rebuild_prefix_sums(&self.messages);
        }
        if self.is_at_bottom() {
            self.unread_incoming_count = 0;
            self.unread_anchor_msg_id = None;
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

        let highlight_paint = self.highlight_manager.paint_state(stage_now_ms());
        let is_group_chat = self.chat_subtitle.contains("member")
            || self.chat_subtitle.contains("subscriber")
            || self.chat_subtitle == "group"
            || self.messages.iter().filter(|m| m.direction == MessageDirection::Incoming).map(|m| m.sender_id).collect::<HashSet<_>>().len() > 1;
        let selected_set: HashSet<u64> = self.selected_message_ids.iter().copied().collect();
        let has_any_selection = !selected_set.is_empty();

        let mut message_rows = Vec::with_capacity(if total_msgs == 0 { 0 } else { end_idx - start_idx + 1 });
        if total_msgs > 0 {
            for idx in start_idx..=end_idx.min(total_msgs.saturating_sub(1)) {
                let msg = &self.messages[idx];
                let (attached_prev, attached_next) = compute_attachment(&self.messages, idx);
                let is_outgoing = msg.direction == MessageDirection::Outgoing;
                let group = BubbleGroupLayout {
                    attached_to_prev: attached_prev,
                    attached_to_next: attached_next,
                    show_sender_name: !is_outgoing && !attached_prev && is_group_chat,
                    show_bottom_avatar: !is_outgoing && !attached_next,
                    show_tail: !attached_next,
                    reserve_avatar_column: !is_outgoing && is_group_chat,
                };
                let meta = self.message_meta.get(&msg.id);
                let prev_date = if idx > 0 {
                    self.message_meta.get(&self.messages[idx - 1].id).and_then(|m| m.date_group_label.as_deref())
                } else {
                    None
                };
                let cur_date = meta.and_then(|m| m.date_group_label.as_deref());
                let date_divider = match (prev_date, cur_date) {
                    (Some(p), Some(c)) if p != c => Some(c),
                    (None, Some(c)) if idx == 0 => Some(c),
                    _ => None,
                };
                let hl_alpha = highlight_paint
                    .as_ref()
                    .and_then(|(hid, p)| if *hid == msg.id { Some(p.opacity) } else { None })
                    .unwrap_or(0.0);

                message_rows.push(render_message_row_1to1(
                    msg,
                    meta,
                    group,
                    selected_set.contains(&msg.id),
                    has_any_selection,
                    self.hovered_message_id == Some(msg.id),
                    hl_alpha,
                    self.unread_anchor_msg_id == Some(msg.id),
                    date_divider,
                    &self.revealed_spoilers,
                    &self.expanded_quotes,
                    self.copied_code_msg,
                    &self.palette,
                    &self.lang_id,
                    cx,
                    |this, action, cx| this.handle_bubble_action(action, cx),
                ));
            }
        }

        let sticky_date: Option<String> = if total_msgs > 0 && has_active_chat {
            self.messages.get(start_idx).and_then(|m| {
                self.message_meta
                    .get(&m.id)
                    .and_then(|rm| rm.date_group_label.clone())
                    .or_else(|| if m.timestamp.contains(' ') { m.timestamp.split(' ').next().map(str::to_string) } else { None })
            })
        } else {
            None
        };

        let pinned_info: Option<(String, String, Option<String>, usize, usize)> = if has_active_chat {
            self.pinned_message.as_ref().map(|snippet| {
                let target_id = self.pinned_tracker.current_msg_id().map(|id| id as u64);
                let target_msg = target_id
                    .and_then(|id| self.messages.iter().find(|m| m.id == id))
                    .or_else(|| self.messages.iter().rev().find(|m| m.is_pinned));
                let sender = target_msg.map(|m| m.sender_name.clone()).unwrap_or_default();
                let thumb = target_msg.and_then(|m| match &m.content {
                    MessageContent::Media { url, .. } => Some(url.clone()),
                    MessageContent::Album { items, .. } => items.first().map(|i| i.url.clone()),
                    _ => None,
                });
                let count = self.pinned_tracker.count().max(1);
                let active_idx = self.pinned_tracker.current_index;
                (snippet.clone(), sender, thumb, count, active_idx)
            })
        } else {
            None
        };

        let show_scroll_fab = has_active_chat && total_msgs > 0 && (!self.is_at_bottom() || self.unread_incoming_count > 0);
        let accent_hex = self.palette.accent;

        let mut root = div()
            .id("conversation-stage-root")
            .track_focus(&focus)
            .key_context("ConversationStage")
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _window, cx| {
                this.handle_stage_key_down(event, cx);
            }))
            .drag_over::<gpui::ExternalPaths>(move |style, _paths: &gpui::ExternalPaths, _, _| {
                style.border_2().border_color(rgb(accent_hex))
            })
            .on_drop::<gpui::ExternalPaths>(cx.listener(|this, paths: &gpui::ExternalPaths, _, cx| {
                let dropped: Vec<PathBuf> = paths.paths().to_vec();
                if !dropped.is_empty() {
                    let strings: Vec<String> = dropped.iter().map(|p| p.display().to_string()).collect();
                    let (_zone, _) = this.drag_area.drop_files(strings, false);
                    this.pending_dropped_paths = Some(dropped);
                    this.toast_banner = Some("Dropped files ready to attach".to_string());
                    cx.notify();
                }
            }))
            .relative()
            .flex_1()
            .min_h(px(0.0))
            .w_full()
            .overflow_hidden()
            .bg(rgb(self.palette.history_bg))
            .flex()
            .flex_col();

        if has_active_chat || total_msgs > 0 {
            root = root.child(render_stage_top_bar(
                self.active_chat_id,
                &self.chat_title,
                &self.chat_subtitle,
                &self.chat_avatar_initials,
                self.chat_avatar_url.as_deref(),
                self.selected_message_ids.len(),
                self.search_open,
                &self.search_query,
                self.search_match_cursor,
                self.search_match_ids.len(),
                self.is_inspector_open,
                self.more_menu_open,
                &self.palette,
                &self.lang_id,
                cx,
                |this, action, cx| this.handle_topbar_action(action, cx),
            ));
        }

        if let Some((snippet, sender, thumb, count, active_idx)) = pinned_info {
            root = root.child(render_pinned_banner_46px(
                &snippet,
                &sender,
                thumb.as_deref(),
                count,
                active_idx,
                &self.palette,
                &self.lang_id,
                cx,
                |this, action, cx| this.handle_topbar_action(action, cx),
            ));
        }

        let transcript = div()
            .relative()
            .flex_1()
            .min_h(px(0.0))
            .w_full()
            .overflow_hidden()
            .child(render_themed_wallpaper(&self.palette))
            .child(
                div()
                    .id("stage-messages-scroll")
                    .relative()
                    .size_full()
                    .px(px(12.0))
                    .py(px(14.0))
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll_handle)
                    .flex()
                    .flex_col()
                    .when(!has_active_chat && total_msgs == 0, |el| {
                        el.child(render_empty_no_chat_pill(&self.palette, &self.lang_id))
                    })
                    .when(has_active_chat && total_msgs == 0, |el| {
                        el.child(render_empty_history_placeholder(&self.palette, &self.lang_id))
                    })
                    .when(top_spacer_height > 0.0, |el| {
                        el.child(div().id("stage-top-spacer").w_full().h(px(top_spacer_height)).flex_shrink_0())
                    })
                    .children(message_rows)
                    .when(bottom_spacer_height > 0.0, |el| {
                        el.child(div().id("stage-bottom-spacer").w_full().h(px(bottom_spacer_height)).flex_shrink_0())
                    }),
            )
            .when_some(sticky_date, |el, date_lbl| {
                el.child(
                    div()
                        .absolute()
                        .top(px(8.0))
                        .left_0()
                        .right_0()
                        .flex()
                        .justify_center()
                        .child(
                            div()
                                .px(px(12.0))
                                .py(px(3.0))
                                .rounded_full()
                                .bg(self.palette.service_pill_rgba())
                                .text_size(px(11.5))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(self.palette.service_pill_fg))
                                .child(date_lbl),
                        ),
                )
            })
            .when_some(self.toast_banner.clone(), |el, toast| {
                el.child(
                    div()
                        .id("stage-toast-pill")
                        .absolute()
                        .bottom(px(16.0))
                        .left_0()
                        .right_0()
                        .flex()
                        .justify_center()
                        .child(
                            div()
                                .px(px(14.0))
                                .py(px(6.0))
                                .rounded_full()
                                .bg(rgb(self.palette.tooltip_bg))
                                .text_size(px(12.0))
                                .text_color(rgb(self.palette.tooltip_fg))
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                    this.toast_banner = None;
                                    cx.notify();
                                }))
                                .child(toast),
                        ),
                )
            })
            .when(show_scroll_fab, |el| {
                let unread = self.unread_incoming_count as u32;
                el.child(render_scroll_to_bottom_fab(unread, &self.palette, cx, |this, cx| {
                    this.handle_topbar_action(TopBarUiAction::ScrollToBottom, cx);
                }))
            })
            .when(self.drag_area.is_active(), |el| {
                let target_files = matches!(self.drag_area.hovered_zone(), Some(DragTargetZone::Document));
                el.child(
                    div()
                        .absolute()
                        .inset(px(16.0))
                        .rounded(px(16.0))
                        .bg(TdesktopThemePalette::rgba_with_alpha(self.palette.card_bg, 0.90))
                        .border_2()
                        .border_color(rgb(self.palette.accent))
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .gap(px(8.0))
                        .child(div().text_size(px(15.0)).font_weight(FontWeight::BOLD).text_color(rgb(self.palette.text_primary)).child(
                            if target_files {
                                tr_key_or(&self.lang_id, "lng_drag_files_here", "Drop files to send uncompressed")
                            } else {
                                tr_key_or(&self.lang_id, "lng_drag_photos_here", "Drop photos or videos to send")
                            }
                        )),
                )
            });

        root = root.child(transcript);

        if let Some(menu) = self.active_context_menu.clone() {
            let can_edit = menu.is_outgoing
                && self
                    .messages
                    .iter()
                    .find(|m| m.id == menu.message_id)
                    .map(|m| matches!(m.content, MessageContent::Text { .. }))
                    .unwrap_or(false);
            let is_translated = self
                .message_meta
                .get(&menu.message_id)
                .map(|m| m.show_translation)
                .unwrap_or(false);
            root = root.child(render_message_context_menu(
                menu.message_id,
                menu.position,
                menu.is_outgoing,
                menu.is_pinned,
                can_edit,
                is_translated,
                &self.palette,
                &self.lang_id,
                cx,
                |this, action, cx| this.handle_context_action(action, cx),
            ));
        }

        root
    }
}

fn stage_now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

impl ConversationStage {
    pub fn apply_theme(&mut self, palette: TdesktopThemePalette, lang_id: impl Into<String>) {
        self.palette = palette;
        self.lang_id = lang_id.into();
    }

    pub fn set_chat_avatar_url(&mut self, url: Option<String>) {
        self.chat_avatar_url = url;
    }

    pub fn set_message_meta(&mut self, msg_id: u64, meta: MessageRichMeta) {
        self.message_meta.insert(msg_id, meta);
        self.row_height_cache.mark_dirty();
    }

    pub fn sync_pinned_tracker(&mut self) {
        let pinned_ids: Vec<i64> = self.messages.iter().filter(|m| m.is_pinned).map(|m| m.id as i64).collect();
        self.pinned_tracker.set_pinned_ids(pinned_ids);
    }

    pub fn jump_to_message(&mut self, msg_id: u64) {
        if let Some(idx) = self.messages.iter().position(|m| m.id == msg_id) {
            if self.row_height_cache.is_dirty() || self.row_height_cache.len() != self.messages.len() {
                self.row_height_cache.rebuild_prefix_sums(&self.messages);
            }
            let y_top = if idx > 0 {
                self.row_height_cache.prefix_sums.get(idx - 1).copied().unwrap_or(0.0)
            } else {
                0.0
            };
            self.scroll_handle.set_offset(point(px(0.0), px(-y_top)));
            self.is_anchored_to_bottom = false;
            self.highlight_manager.highlight(
                SelectedQuote {
                    message_id: msg_id,
                    text_range: None,
                    todo_list_id: None,
                    poll_option: None,
                    rich_page: None,
                },
                stage_now_ms(),
            );
        }
    }

    fn recompute_search_matches(&mut self) {
        let q = self.search_query.trim().to_lowercase();
        self.search_match_ids.clear();
        self.search_match_cursor = 0;
        if q.is_empty() {
            return;
        }
        for msg in self.messages.iter().rev() {
            let hay = match &msg.content {
                MessageContent::Text { text, .. } => text.clone(),
                MessageContent::Media { caption, .. } | MessageContent::Album { caption, .. } => {
                    caption.clone().unwrap_or_default()
                }
                MessageContent::Poll { question, .. } => question.clone(),
                MessageContent::Location { title, address, .. } => format!("{} {}", title, address),
                _ => msg.sender_name.clone(),
            };
            if hay.to_lowercase().contains(&q) || msg.sender_name.to_lowercase().contains(&q) {
                self.search_match_ids.push(msg.id);
            }
        }
        if let Some(&first_id) = self.search_match_ids.first() {
            self.jump_to_message(first_id);
        }
    }

    fn handle_stage_key_down(&mut self, event: &KeyDownEvent, cx: &mut Context<Self>) {
        let ks = &event.keystroke;
        if ks.modifiers.platform && ks.key == "f" && self.active_chat_id.is_some() {
            self.search_open = true;
            cx.notify();
            return;
        }
        if ks.key == "escape" {
            if self.active_context_menu.is_some() {
                self.active_context_menu = None;
            } else if self.more_menu_open {
                self.more_menu_open = false;
            } else if self.search_open {
                self.search_open = false;
                self.search_query.clear();
                self.search_match_ids.clear();
            } else if !self.selected_message_ids.is_empty() {
                self.selected_message_ids.clear();
            }
            cx.notify();
            return;
        }
        if self.search_open {
            if ks.key == "enter" {
                let delta = if ks.modifiers.shift { -1 } else { 1 };
                self.handle_topbar_action(TopBarUiAction::SearchStep(delta), cx);
            } else if ks.key == "backspace" {
                self.search_query.pop();
                self.recompute_search_matches();
                cx.notify();
            } else if !ks.modifiers.platform && !ks.modifiers.control {
                if let Some(ref ch) = ks.key_char {
                    self.search_query.push_str(ch);
                    self.recompute_search_matches();
                    cx.notify();
                } else if ks.key.len() == 1 {
                    self.search_query.push_str(&ks.key);
                    self.recompute_search_matches();
                    cx.notify();
                }
            }
        }
    }

    fn handle_topbar_action(&mut self, action: TopBarUiAction, cx: &mut Context<Self>) {
        match action {
            TopBarUiAction::ToggleInspector => self.toggle_inspector(),
            TopBarUiAction::ToggleSearch => {
                self.more_menu_open = false;
                self.search_open = !self.search_open;
                if !self.search_open {
                    self.search_query.clear();
                    self.search_match_ids.clear();
                }
            }
            TopBarUiAction::SearchStep(delta) => {
                let len = self.search_match_ids.len();
                if len > 0 {
                    if delta < 0 {
                        self.search_match_cursor = if self.search_match_cursor == 0 { len - 1 } else { self.search_match_cursor - 1 };
                    } else {
                        self.search_match_cursor = (self.search_match_cursor + 1) % len;
                    }
                    if let Some(&id) = self.search_match_ids.get(self.search_match_cursor) {
                        self.jump_to_message(id);
                    }
                }
            }
            TopBarUiAction::SearchClear => {
                self.search_open = false;
                self.search_query.clear();
                self.search_match_ids.clear();
                self.search_match_cursor = 0;
            }
            TopBarUiAction::StartCall { video } => {
                if video {
                    self.pending_video_call = true;
                } else {
                    self.pending_call_request = true;
                }
            }
            TopBarUiAction::ToggleTopMoreMenu => self.more_menu_open = !self.more_menu_open,
            TopBarUiAction::JumpPinned => {
                self.sync_pinned_tracker();
                if let Some(target_id) = self.pinned_tracker.next_pinned_id().or_else(|| self.pinned_tracker.current_msg_id()) {
                    self.jump_to_message(target_id as u64);
                } else if let Some(last_pin) = self.messages.iter().rev().find(|m| m.is_pinned) {
                    self.jump_to_message(last_pin.id);
                }
            }
            TopBarUiAction::UnpinCurrent => {
                let target_id = self
                    .pinned_tracker
                    .current_msg_id()
                    .map(|id| id as u64)
                    .or_else(|| self.messages.iter().rev().find(|m| m.is_pinned).map(|m| m.id));
                if let Some(id) = target_id {
                    self.toggle_pin_message(id);
                    self.sync_pinned_tracker();
                    self.pending_pin_toggle = Some((id, false));
                } else {
                    self.pinned_message = None;
                }
            }
            TopBarUiAction::ClearSelection => self.selected_message_ids.clear(),
            TopBarUiAction::ForwardSelected => {
                if !self.selected_message_ids.is_empty() {
                    self.pending_forward_message_ids = Some(self.selected_message_ids.clone());
                    self.selected_message_ids.clear();
                }
            }
            TopBarUiAction::CopySelected => {
                let mut parts = Vec::new();
                for &id in &self.selected_message_ids {
                    if let Some(t) = self.copy_message_text(id) {
                        parts.push(t);
                    }
                }
                if !parts.is_empty() {
                    cx.write_to_clipboard(ClipboardItem::new_string(parts.join("\n")));
                    self.toast_banner = Some(tr_key_or(&self.lang_id, "lng_copied_to_clipboard", "Copied to clipboard").to_string());
                }
                self.selected_message_ids.clear();
            }
            TopBarUiAction::DeleteSelected => {
                if !self.selected_message_ids.is_empty() {
                    let ids = self.selected_message_ids.clone();
                    self.delete_messages_by_ids(&ids);
                    self.sync_pinned_tracker();
                    self.pending_delete_message_ids = Some(ids);
                }
            }
            TopBarUiAction::ScrollToBottom => {
                self.scroll_to_bottom();
                self.unread_incoming_count = 0;
                self.unread_anchor_msg_id = None;
            }
            TopBarUiAction::ClearHistory => {
                self.more_menu_open = false;
                self.pending_clear_history = true;
            }
            TopBarUiAction::ExportChat => {
                self.more_menu_open = false;
                self.pending_export_chat = true;
            }
            TopBarUiAction::ToggleMuteChat => {
                self.more_menu_open = false;
                self.pending_toggle_mute = true;
            }
        }
        cx.notify();
    }

    fn handle_bubble_action(&mut self, action: BubbleUiAction, cx: &mut Context<Self>) {
        match action {
            BubbleUiAction::ClickReplyQuote(target_id) => self.jump_to_message(target_id),
            BubbleUiAction::ToggleReaction(msg_id, emoji) => {
                self.toggle_reaction(msg_id, &emoji);
                self.pending_reaction_toggle = Some((msg_id, emoji));
            }
            BubbleUiAction::ToggleSelect(msg_id) => self.toggle_select_message(msg_id),
            BubbleUiAction::OpenContextMenu(msg_id, pos) => {
                let (is_outgoing, is_pinned, text_snippet) = self
                    .messages
                    .iter()
                    .find(|m| m.id == msg_id)
                    .map(|m| {
                        let snippet = match &m.content {
                            MessageContent::Text { text, .. } => Some(text.clone()),
                            MessageContent::Media { caption, .. } | MessageContent::Album { caption, .. } => caption.clone(),
                            _ => None,
                        };
                        (m.is_outgoing(), m.is_pinned, snippet)
                    })
                    .unwrap_or((false, false, None));
                self.active_context_menu = Some(MessageContextMenuState {
                    message_id: msg_id,
                    position: pos,
                    is_outgoing,
                    is_pinned,
                    text_snippet,
                });
            }
            BubbleUiAction::SetHover(hovered) => {
                if self.hovered_message_id != hovered {
                    self.hovered_message_id = hovered;
                } else {
                    return;
                }
            }
            BubbleUiAction::Media(ma) => match ma {
                MediaCardAction::OpenViewer { msg_id, url, is_video, album_index } => {
                    let mut gallery = Vec::new();
                    let mut start_idx = 0usize;
                    for m in &self.messages {
                        match &m.content {
                            MessageContent::Media { media_type, url: murl, caption, .. }
                                if matches!(media_type, MediaType::Photo | MediaType::Video) =>
                            {
                                if m.id == msg_id && murl == &url {
                                    start_idx = gallery.len();
                                }
                                gallery.push((murl.clone(), *media_type == MediaType::Video, caption.clone()));
                            }
                            MessageContent::Album { items, caption } => {
                                for (idx, item) in items.iter().enumerate() {
                                    if m.id == msg_id && album_index == Some(idx) {
                                        start_idx = gallery.len();
                                    }
                                    gallery.push((item.url.clone(), item.media_type == MediaType::Video, caption.clone()));
                                }
                            }
                            _ => {}
                        }
                    }
                    if gallery.is_empty() {
                        gallery.push((url.clone(), is_video, None));
                    }
                    self.pending_media_url = Some((url, is_video));
                    self.pending_media_viewer = Some((gallery, start_idx));
                }
                MediaCardAction::ToggleSpoilerMedia(msg_id) => {
                    if let Some(meta) = self.message_meta.get_mut(&msg_id) {
                        meta.is_spoiler_media = !meta.is_spoiler_media;
                    }
                }
                MediaCardAction::ToggleVoicePlay(msg_id) => {
                    if let Some(m) = self.messages.iter_mut().find(|m| m.id == msg_id) {
                        if let MessageContent::Voice { ref mut is_playing, playback_position_secs, playback_speed, .. } = m.content {
                            *is_playing = !*is_playing;
                            self.pending_voice_playback = Some((msg_id, *is_playing, playback_position_secs, playback_speed));
                        }
                    }
                }
                MediaCardAction::CycleVoiceSpeed(msg_id) => {
                    if let Some(m) = self.messages.iter_mut().find(|m| m.id == msg_id) {
                        if let MessageContent::Voice { ref mut playback_speed, is_playing, playback_position_secs, .. } = m.content {
                            *playback_speed = if (*playback_speed - 1.0).abs() < 0.1 {
                                1.5
                            } else if (*playback_speed - 1.5).abs() < 0.1 {
                                2.0
                            } else {
                                1.0
                            };
                            self.pending_voice_playback = Some((msg_id, is_playing, playback_position_secs, *playback_speed));
                        }
                    }
                }
                MediaCardAction::SeekVoice(msg_id, frac) => {
                    if let Some(m) = self.messages.iter_mut().find(|m| m.id == msg_id) {
                        if let MessageContent::Voice { duration_secs, ref mut playback_position_secs, is_playing, playback_speed, .. } = m.content {
                            *playback_position_secs = (duration_secs as f32) * frac.clamp(0.0, 1.0);
                            self.pending_voice_playback = Some((msg_id, is_playing, *playback_position_secs, playback_speed));
                        }
                    }
                }
                MediaCardAction::RichText(rt) => self.handle_rich_text_action(rt, cx),
            },
            BubbleUiAction::Interactive(ia) => match ia {
                InteractiveCardAction::VotePoll(msg_id, option_id) => {
                    self.vote_poll(msg_id, option_id);
                    self.pending_poll_vote = Some((msg_id, option_id));
                }
                InteractiveCardAction::CallBack(_msg_id, video) => {
                    if video { self.pending_video_call = true; } else { self.pending_call_request = true; }
                }
                InteractiveCardAction::OpenStory(_msg_id, user_name) => {
                    self.pending_bot_action = Some(format!("Story: {}", user_name));
                }
                InteractiveCardAction::PayInvoice(msg_id, _title, _stars) => {
                    if let Some(m) = self.messages.iter_mut().find(|m| m.id == msg_id) {
                        if let MessageContent::Invoice { ref mut status, .. } = m.content {
                            *status = match *status {
                                InvoiceStatus::Pending => InvoiceStatus::Paid,
                                InvoiceStatus::Paid => InvoiceStatus::Paid,
                                InvoiceStatus::Refunded => InvoiceStatus::Paid,
                            };
                        }
                    }
                }
                InteractiveCardAction::BotButton(_msg_id, btn) => {
                    let label = match btn {
                        InlineButtonType::Callback(d) | InlineButtonType::Url(d) | InlineButtonType::WebApp(d) | InlineButtonType::SwitchInline(d) => d,
                    };
                    self.pending_bot_action = Some(label);
                }
            },
            BubbleUiAction::Context(ca) => {
                self.handle_context_action(ca, cx);
                return;
            }
            BubbleUiAction::RichText(rt) => self.handle_rich_text_action(rt, cx),
            BubbleUiAction::DownloadDoc(_msg_id, url, file_name) => {
                self.pending_download_url = Some((url, file_name));
            }
            BubbleUiAction::OpenContact(uid) => self.pending_open_profile_peer_id = Some(uid),
            BubbleUiAction::OpenWebUrl(url) => self.pending_media_url = Some((url, false)),
            BubbleUiAction::OpenInstantView(url, title) => self.pending_iv_url = Some((url, title)),
        }
        cx.notify();
    }

    fn handle_rich_text_action(&mut self, action: RichTextAction, cx: &mut Context<Self>) {
        match action {
            RichTextAction::ToggleSpoiler(msg_id, idx) => {
                if !self.revealed_spoilers.insert((msg_id, idx)) {
                    self.revealed_spoilers.remove(&(msg_id, idx));
                }
            }
            RichTextAction::ToggleBlockquote(msg_id, idx) => {
                if !self.expanded_quotes.insert((msg_id, idx)) {
                    self.expanded_quotes.remove(&(msg_id, idx));
                }
            }
            RichTextAction::CopyCode(code) => {
                cx.write_to_clipboard(ClipboardItem::new_string(code));
                self.toast_banner = Some(tr_key_or(&self.lang_id, "lng_copied_to_clipboard", "Code copied").to_string());
            }
            RichTextAction::OpenUrl(url) => self.pending_media_url = Some((url, false)),
            RichTextAction::ClickMention(m) | RichTextAction::ClickHashtag(m) | RichTextAction::ClickBotCommand(m) => {
                self.pending_bot_action = Some(m);
            }
        }
    }

    fn handle_context_action(&mut self, action: ContextMenuAction, cx: &mut Context<Self>) {
        match action {
            ContextMenuAction::React(msg_id, emoji) => {
                self.toggle_reaction(msg_id, &emoji);
                self.pending_reaction_toggle = Some((msg_id, emoji));
            }
            ContextMenuAction::Reply(msg_id) => {
                if let Some(m) = self.messages.iter().find(|m| m.id == msg_id) {
                    let snippet = match &m.content {
                        MessageContent::Text { text, .. } => text.lines().next().unwrap_or(text).to_string(),
                        MessageContent::Media { caption, .. } | MessageContent::Album { caption, .. } => {
                            caption.clone().unwrap_or_else(|| "Media".to_string())
                        }
                        _ => "Message".to_string(),
                    };
                    self.pending_reply = Some(ReplyReference {
                        message_id: msg_id,
                        sender_name: m.sender_name.clone(),
                        snippet,
                    });
                }
            }
            ContextMenuAction::Edit(msg_id) => {
                if let Some(text) = self.copy_message_text(msg_id) {
                    self.pending_edit_message = Some((msg_id, text));
                }
            }
            ContextMenuAction::TogglePin(msg_id) => {
                let will_pin = self.messages.iter().find(|m| m.id == msg_id).map(|m| !m.is_pinned).unwrap_or(true);
                self.toggle_pin_message(msg_id);
                self.sync_pinned_tracker();
                self.pending_pin_toggle = Some((msg_id, will_pin));
            }
            ContextMenuAction::CopyText(msg_id) => {
                if let Some(t) = self.copy_message_text(msg_id) {
                    cx.write_to_clipboard(ClipboardItem::new_string(t));
                    self.toast_banner = Some(tr_key_or(&self.lang_id, "lng_copied_to_clipboard", "Copied to clipboard").to_string());
                }
            }
            ContextMenuAction::Forward(msg_id) => {
                self.pending_forward_message_ids = Some(vec![msg_id]);
            }
            ContextMenuAction::Translate(msg_id) => {
                if let Some(meta) = self.message_meta.get_mut(&msg_id) {
                    meta.show_translation = !meta.show_translation;
                }
                self.pending_translate_message_id = Some(msg_id);
            }
            ContextMenuAction::Select(msg_id) => self.toggle_select_message(msg_id),
            ContextMenuAction::Delete(msg_id) => {
                self.delete_message(msg_id);
                self.sync_pinned_tracker();
                self.pending_delete_message_ids = Some(vec![msg_id]);
            }
            ContextMenuAction::CloseMenu => {}
        }
        self.active_context_menu = None;
        cx.notify();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[std::prelude::v1::test]
    fn test_add_message_deduplicates_by_id_and_preserves_highest_delivery_status() {
        let mut stage = ConversationStage::new();
        stage.set_chat(7, "Alice", "online", "AL", 0x2AABEE);

        stage.add_message(MessageItemModel {
            id: 42,
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
                text: "Initial".to_string(),
                is_edited: false,
            },
            bot_keyboard: None,
            is_pinned: false,
        });
        assert_eq!(stage.messages.len(), 1);

        // Stale Sending re-delivery refreshes content/reply/reactions without downgrading Sent or duplicating
        stage.add_message(MessageItemModel {
            id: 42,
            sender_id: 1,
            sender_name: "Me".to_string(),
            avatar_initials: "ME".to_string(),
            avatar_color: 0x2AABEE,
            direction: MessageDirection::Outgoing,
            timestamp: "12:00".to_string(),
            delivery_status: DeliveryStatus::Sending,
            reply_to: Some(ReplyReference {
                message_id: 41,
                sender_name: "Alice".to_string(),
                snippet: "Hi".to_string(),
            }),
            reactions: vec![ReactionItem {
                emoji: "👍".to_string(),
                count: 1,
                user_reacted: true,
            }],
            content: MessageContent::Text {
                text: "Updated".to_string(),
                is_edited: true,
            },
            bot_keyboard: None,
            is_pinned: false,
        });

        assert_eq!(stage.messages.len(), 1);
        assert_eq!(stage.chat_histories.get(&7).map(|h| h.len()), Some(1));
        assert_eq!(stage.messages[0].delivery_status, DeliveryStatus::Sent);
        assert_eq!(stage.messages[0].reactions.len(), 1);
        assert_eq!(stage.messages[0].reply_to.as_ref().map(|r| r.message_id), Some(41));
        assert!(matches!(
            stage.messages[0].content,
            MessageContent::Text { ref text, is_edited: true } if text == "Updated"
        ));

        // Higher Read status upgrades existing bubble cleanly
        stage.add_message(MessageItemModel {
            id: 42,
            sender_id: 1,
            sender_name: "Me".to_string(),
            avatar_initials: "ME".to_string(),
            avatar_color: 0x2AABEE,
            direction: MessageDirection::Outgoing,
            timestamp: "12:00".to_string(),
            delivery_status: DeliveryStatus::Read,
            reply_to: None,
            reactions: vec![],
            content: MessageContent::Text {
                text: "Updated".to_string(),
                is_edited: true,
            },
            bot_keyboard: None,
            is_pinned: false,
        });

        assert_eq!(stage.messages.len(), 1);
        assert_eq!(stage.messages[0].delivery_status, DeliveryStatus::Read);
    }
}
