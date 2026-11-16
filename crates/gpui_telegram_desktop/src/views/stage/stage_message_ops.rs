//! Message CRUD, history merging, reaction, poll, scroll, and row-building operations for `ConversationStage`.

use std::collections::HashSet;

use gpui::*;

use crate::views::conversation_stage::{compute_attachment, ConversationStage, MSG_MAX_WIDTH, MSG_MIN_WIDTH};
use crate::views::stage::cards::*;
use crate::views::stage::history_view_highlight_manager::ChatPaintHighlight;
use crate::views::stage::rich_meta::MessageRichMeta;
use crate::views::stage::stage_bubble::{render_message_row_1to1, BubbleGroupLayout};
use crate::views::stage::stage_context_menu::{
    should_open_selector_below, ReactionOverlayLayout, ReactionOverlayMode,
};

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
        self.pending_bot_button_action = None;
        self.pending_edit_message = None;
        self.pending_forward_message_ids = None;
        self.pending_delete_message_ids = None;
        self.pending_pin_toggle = None;
        self.pending_translate_message_id = None;
        self.pending_poll_vote = None;
        self.pending_reaction_toggle = None;
        self.pending_reaction_actions.clear();
        self.pending_voice_playback = None;
        self.pending_download_url = None;
        self.pending_dropped_paths = None;
        self.pending_open_profile_peer_id = None;
        self.pending_clear_history = false;
        self.pending_export_chat = false;
        self.pending_toggle_mute = false;
        self.pending_change_wallpaper = false;
        self.pending_toggle_pin_chat = false;
        self.pending_set_ttl_seconds = None;
        self.pending_toggle_block_peer = false;
        self.pending_delete_or_leave_chat = false;
        self.is_muted_chat = false;
        self.is_pinned_chat = false;
        self.is_peer_blocked = false;
        self.is_group_chat = false;
        self.is_saved_messages = false;
        self.message_ttl_seconds = 0;
        self.ttl_submenu_open = false;
        self.unread_incoming_count = 0;
        self.unread_anchor_msg_id = None;
        self.active_context_menu = None;
        self.context_menu_reactions_expanded = false;
        self.pending_reply = None;
        self.simulated_max_offset = None;
        self.simulated_viewport_height = None;
        self.selected_message_ids.clear();
        self.pinned_tracker.set_pinned_ids(Vec::new());
        self.search_open = false;
        self.search_query.clear();
        self.search_match_ids.clear();
        self.search_match_cursor = 0;
        self.hovered_message_id = None;
        self.hovered_gutter_id = None;
        self.hovered_reaction_bar_id = None;
        self.hovered_reaction_bridge_id = None;
        self.quick_strip_msg_id = None;
        self.hover_grace = None;
        self.hover_grace_generation = self.hover_grace_generation.wrapping_add(1);
        self.reaction_picker_msg_id = None;
        self.transcript_bounds_probe.set(None);
        self.bubble_widths_probe.borrow_mut().clear();
        self.bubble_bounds_probe.borrow_mut().clear();
        self.row_heights_probe.borrow_mut().clear();
        self.more_menu_open = false;
        self.toast_banner = None;
        self.scroll_handle.set_offset(Point::default());
        self.is_anchored_to_bottom = true;
        self.row_height_cache.measured_heights.clear();
        self.row_height_cache.rebuild_prefix_sums(&self.messages);
    }

    pub fn toggle_inspector(&mut self) {
        self.is_inspector_open = !self.is_inspector_open;
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
        let sender_name = if self.chat_title.is_empty() || self.chat_title == "Select a chat" {
            "Peer".to_string()
        } else {
            self.chat_title.clone()
        };
        let avatar_initials = if !self.chat_avatar_initials.is_empty() {
            self.chat_avatar_initials.clone()
        } else {
            sender_name
                .chars()
                .next()
                .map(|c| c.to_uppercase().to_string())
                .unwrap_or_else(|| "?".to_string())
        };
        self.add_message(MessageItemModel {
            id: next_id,
            sender_id,
            sender_name,
            avatar_initials,
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
    /// deduplicating by message ID while preserving local pending/sending messages
    /// and reconciling server-side reaction/delivery updates on existing messages.
    pub fn merge_historical_messages(&mut self, server_messages: Vec<MessageItemModel>) {
        self.merge_historical_messages_preserving(server_messages, &HashSet::new());
    }

    /// Merges server-fetched historical messages while preserving optimistic reactions on
    /// messages whose IDs are in `protected_reaction_msg_ids` (e.g. in-flight reaction requests).
    pub fn merge_historical_messages_preserving(
        &mut self,
        server_messages: Vec<MessageItemModel>,
        protected_reaction_msg_ids: &HashSet<u64>,
    ) {
        if self.messages.is_empty() {
            self.messages = server_messages;
            self.row_height_cache.mark_dirty();
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
            } else if let Some(existing) = merged.iter_mut().find(|m| m.id == s_msg.id) {
                if existing.delivery_status != DeliveryStatus::Sending {
                    if !protected_reaction_msg_ids.contains(&s_msg.id) {
                        existing.reactions = s_msg.reactions;
                    }
                    existing.delivery_status =
                        max_delivery_status(existing.delivery_status, s_msg.delivery_status);
                    existing.is_pinned = existing.is_pinned || s_msg.is_pinned;
                }
            }
        }

        // Re-append local sending items at the bottom
        for sending in sending_messages {
            if !existing_ids.contains(&sending.id) {
                merged.push(sending);
            }
        }

        self.messages = merged;
        self.row_height_cache.mark_dirty();
    }

    /// Remaps active hover, picker, context-menu, and selection IDs when an optimistic
    /// `local_id` (`client_nonce`) is promoted to an authoritative `server_id`.
    pub fn remap_message_id(&mut self, local_id: u64, server_id: u64) {
        if server_id == 0 || local_id == server_id {
            return;
        }
        if self.hovered_message_id == Some(local_id) {
            self.hovered_message_id = Some(server_id);
        }
        if self.hovered_gutter_id == Some(local_id) {
            self.hovered_gutter_id = Some(server_id);
        }
        if self.hovered_reaction_bar_id == Some(local_id) {
            self.hovered_reaction_bar_id = Some(server_id);
        }
        if self.hovered_reaction_bridge_id == Some(local_id) {
            self.hovered_reaction_bridge_id = Some(server_id);
        }
        if self.quick_strip_msg_id == Some(local_id) {
            self.quick_strip_msg_id = Some(server_id);
        }
        if let Some((ref mut gid, _)) = self.hover_grace {
            if *gid == local_id {
                *gid = server_id;
            }
        }
        if self.reaction_picker_msg_id == Some(local_id) {
            self.reaction_picker_msg_id = Some(server_id);
        }
        {
            let mut widths = self.bubble_widths_probe.borrow_mut();
            if let Some(w) = widths.remove(&local_id) {
                widths.insert(server_id, w);
            }
        }
        {
            let mut bounds = self.bubble_bounds_probe.borrow_mut();
            if let Some(b) = bounds.remove(&local_id) {
                bounds.insert(server_id, b);
            }
        }
        {
            let mut row_heights = self.row_heights_probe.borrow_mut();
            if let Some(h) = row_heights.remove(&local_id) {
                row_heights.insert(server_id, h);
            }
        }
        if let Some(h) = self.row_height_cache.measured_heights.remove(&local_id) {
            self.row_height_cache.measured_heights.insert(server_id, h);
        }
        if let Some(ref mut menu) = self.active_context_menu {
            if menu.message_id == local_id {
                menu.message_id = server_id;
            }
        }
        for sel in &mut self.selected_message_ids {
            if *sel == local_id {
                *sel = server_id;
            }
        }
    }

    pub fn enqueue_reaction_toggle(&mut self, msg_id: u64, emoji: String) {
        let is_added = self
            .messages
            .iter()
            .find(|m| m.id == msg_id)
            .map(|m| m.reactions.iter().any(|r| r.emoji == emoji && r.user_reacted))
            .unwrap_or(true);
        self.pending_reaction_toggle = Some((msg_id, emoji.clone()));
        self.pending_reaction_actions
            .push_back((msg_id, emoji, is_added));
    }

    pub fn drain_pending_reaction_actions(&mut self) -> Vec<(u64, String, bool)> {
        if !self.pending_reaction_actions.is_empty() {
            self.pending_reaction_toggle = None;
            return self.pending_reaction_actions.drain(..).collect();
        }
        if let Some((mid, emoji)) = self.pending_reaction_toggle.take() {
            let is_added = self
                .messages
                .iter()
                .find(|m| m.id == mid)
                .map(|m| m.reactions.iter().any(|r| r.emoji == emoji && r.user_reacted))
                .unwrap_or(true);
            return vec![(mid, emoji, is_added)];
        }
        Vec::new()
    }

    pub fn resolved_stage_bounds(&self) -> Option<Bounds<Pixels>> {
        self.stage_bounds_probe
            .get()
            .filter(|b| b.size.width > px(1.0) && b.size.height > px(1.0))
    }

    pub fn set_simulated_stage_bounds(&mut self, bounds: Option<Bounds<Pixels>>) {
        self.stage_bounds_probe.set(bounds);
    }

    pub fn effective_viewport_height(&self) -> f32 {
        self.simulated_viewport_height
            .filter(|&h| h > 10.0)
            .or_else(|| {
                self.transcript_bounds_probe
                    .get()
                    .map(|b| f32::from(b.size.height))
                    .filter(|&h| h > 10.0)
            })
            .or_else(|| {
                let h = f32::from(self.scroll_handle.bounds().size.height);
                (h > 10.0).then_some(h)
            })
            .unwrap_or(600.0)
    }

    pub fn effective_stage_width(&self) -> f32 {
        self.resolved_stage_bounds()
            .map(|b| f32::from(b.size.width))
            .filter(|&w| w > 10.0)
            .or_else(|| {
                self.transcript_bounds_probe
                    .get()
                    .map(|b| f32::from(b.size.width))
                    .filter(|&w| w > 10.0)
            })
            .or_else(|| {
                let w = f32::from(self.scroll_handle.bounds().size.width);
                (w > 10.0).then_some(w)
            })
            .unwrap_or(760.0)
    }

    pub fn resolved_or_estimated_bubble_width(
        &self,
        msg: &MessageItemModel,
        stage_w: f32,
        reserve_avatar_column: bool,
    ) -> f32 {
        let avatar_skip = if reserve_avatar_column { 40.0 } else { 0.0 };
        let max_avail = (stage_w - 56.0 - 56.0 - avatar_skip).clamp(MSG_MIN_WIDTH, MSG_MAX_WIDTH);
        if let Some(&measured) = self.bubble_widths_probe.borrow().get(&msg.id) {
            return measured.clamp(MSG_MIN_WIDTH, max_avail);
        }
        let estimated = match &msg.content {
            MessageContent::Text { text, .. } => {
                let max_line_px = text
                    .lines()
                    .map(|line| {
                        line.chars()
                            .map(|ch| if ch.is_ascii() { 7.6_f32 } else { 14.5_f32 })
                            .sum::<f32>()
                    })
                    .fold(0.0_f32, f32::max);
                max_line_px + 72.0
            }
            MessageContent::Voice { .. } => 260.0,
            MessageContent::Service { .. } => MSG_MIN_WIDTH,
            _ => 320.0,
        };
        estimated.clamp(MSG_MIN_WIDTH, max_avail)
    }

    fn row_viewport_vertical_span(&self, idx: usize) -> (f32, f32, f32) {
        let viewport_h = self.effective_viewport_height();
        if self.simulated_viewport_height.is_none() {
            if let (Some(vp), Some(msg)) = (self.transcript_bounds_probe.get(), self.messages.get(idx))
            {
                let vp_h = f32::from(vp.size.height);
                if vp_h > 10.0 {
                    if let Some(&b_bounds) = self.bubble_bounds_probe.borrow().get(&msg.id) {
                        let b_h = f32::from(b_bounds.size.height);
                        if b_h > 1.0 {
                            let rel_top = f32::from(b_bounds.origin.y - vp.origin.y);
                            return (rel_top, rel_top + b_h, vp_h);
                        }
                    }
                }
            }
        }
        let scroll_top = f32::from(-self.scroll_handle.offset().y).max(0.0);
        let pad_top = 14.0_f32;
        let row_top = if idx == 0 {
            0.0
        } else {
            self.row_height_cache
                .prefix_sums
                .get(idx.saturating_sub(1))
                .copied()
                .unwrap_or(0.0)
        } + pad_top;
        let row_bottom = self
            .row_height_cache
            .prefix_sums
            .get(idx)
            .copied()
            .map(|v| v + pad_top)
            .unwrap_or(row_top + 42.0);
        (row_top - scroll_top, row_bottom - scroll_top, viewport_h)
    }

    /// Computes `(is_within_viewport, anchor_below)` in scroll-relative pixel coordinates
    /// so scrolled chats never clip `ExpandedSelector` behind the top header or paint
    /// `deferred` overlays from overscanned rows over stage chrome.
    pub fn compute_reaction_overlay_placement(&self, idx: usize) -> (bool, bool) {
        let (rel_top, rel_bottom, viewport_h) = self.row_viewport_vertical_span(idx);
        let is_within_viewport = rel_bottom > 12.0 && rel_top < viewport_h - 12.0;
        let anchor_below = should_open_selector_below(rel_top, rel_bottom, viewport_h);
        (is_within_viewport, anchor_below)
    }

    pub fn compute_reaction_overlay_layout(
        &self,
        idx: usize,
        msg: &MessageItemModel,
        reserve_avatar_column: bool,
    ) -> (bool, ReactionOverlayLayout) {
        let (rel_top, rel_bottom, viewport_h) = self.row_viewport_vertical_span(idx);
        let is_within_viewport = rel_bottom > 12.0 && rel_top < viewport_h - 12.0;
        let selector_below = should_open_selector_below(rel_top, rel_bottom, viewport_h);
        let stage_w = self.effective_stage_width();
        let bubble_w = self.resolved_or_estimated_bubble_width(msg, stage_w, reserve_avatar_column);
        let is_outgoing = msg.direction == MessageDirection::Outgoing;
        let layout = ReactionOverlayLayout::compute(
            selector_below,
            rel_top,
            rel_bottom,
            viewport_h,
            bubble_w,
            stage_w,
            is_outgoing,
            reserve_avatar_column,
        );
        (is_within_viewport, layout)
    }

    pub fn reaction_overlay_mode(&self, msg_id: u64) -> Option<ReactionOverlayMode> {
        if !self.selected_message_ids.is_empty() || self.active_context_menu.is_some() {
            return None;
        }
        let directly_hovering = self.hovered_message_id == Some(msg_id)
            || self.hovered_gutter_id == Some(msg_id)
            || self.hovered_reaction_bar_id == Some(msg_id)
            || self.hovered_reaction_bridge_id == Some(msg_id);
        if self.reaction_picker_msg_id == Some(msg_id) {
            Some(ReactionOverlayMode::ExpandedSelector)
        } else if self.hovered_reaction_bar_id == Some(msg_id)
            || self.hovered_reaction_bridge_id == Some(msg_id)
            || (directly_hovering && self.quick_strip_msg_id == Some(msg_id))
        {
            Some(ReactionOverlayMode::QuickStrip)
        } else if self.hovered_message_id == Some(msg_id)
            || self.hovered_gutter_id == Some(msg_id)
        {
            Some(ReactionOverlayMode::CornerButton)
        } else if let Some((grace_id, grace_mode)) = self.hover_grace {
            (grace_id == msg_id).then_some(grace_mode)
        } else {
            None
        }
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
        if self.hovered_message_id == Some(msg_id) {
            self.hovered_message_id = None;
        }
        if self.hovered_gutter_id == Some(msg_id) {
            self.hovered_gutter_id = None;
        }
        if self.hovered_reaction_bar_id == Some(msg_id) {
            self.hovered_reaction_bar_id = None;
        }
        if self.hovered_reaction_bridge_id == Some(msg_id) {
            self.hovered_reaction_bridge_id = None;
        }
        if self.quick_strip_msg_id == Some(msg_id) {
            self.quick_strip_msg_id = None;
        }
        if self.reaction_picker_msg_id == Some(msg_id) {
            self.reaction_picker_msg_id = None;
        }
        if self.hover_grace.is_some_and(|(gid, _)| gid == msg_id) {
            self.hover_grace = None;
            self.hover_grace_generation = self.hover_grace_generation.wrapping_add(1);
        }
        self.bubble_widths_probe.borrow_mut().remove(&msg_id);
        self.bubble_bounds_probe.borrow_mut().remove(&msg_id);
        self.row_heights_probe.borrow_mut().remove(&msg_id);
        self.row_height_cache.measured_heights.remove(&msg_id);
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
        let mut updated = false;
        if let Some(msg) = self.messages.iter_mut().find(|m| m.id == message_id) {
            msg.toggle_reaction(emoji);
            updated = true;
        }
        if let Some(cid) = self.active_chat_id {
            if let Some(hist) = self.chat_histories.get_mut(&cid) {
                if let Some(hmsg) = hist.iter_mut().find(|m| m.id == message_id) {
                    hmsg.toggle_reaction(emoji);
                    updated = true;
                }
            }
        }
        if updated {
            self.row_height_cache.mark_dirty();
        }
    }

    pub fn set_message_reactions(
        &mut self,
        chat_id: u64,
        message_id: u64,
        reactions: Vec<ReactionItem>,
    ) {
        let mut updated = false;
        if self.active_chat_id == Some(chat_id) || self.active_chat_id.is_none() {
            if let Some(msg) = self.messages.iter_mut().find(|m| m.id == message_id) {
                msg.reactions = reactions.clone();
                updated = true;
            }
        }
        if let Some(hist) = self.chat_histories.get_mut(&chat_id) {
            if let Some(hmsg) = hist.iter_mut().find(|m| m.id == message_id) {
                hmsg.reactions = reactions;
                updated = true;
            }
        }
        if updated {
            self.row_height_cache.mark_dirty();
        }
    }

    pub fn message_count(&self) -> usize {
        self.messages.len()
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

    pub fn is_selecting_messages(&self) -> bool {
        self.select_mode_active || !self.selected_message_ids.is_empty()
    }

    pub fn clear_selected_messages(&mut self) {
        self.select_mode_active = false;
        self.selected_message_ids.clear();
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
        {
            let mut row_heights = self.row_heights_probe.borrow_mut();
            for id in ids {
                row_heights.remove(id);
                self.row_height_cache.measured_heights.remove(id);
            }
        }
        self.row_height_cache.mark_dirty();
        self.row_height_cache.rebuild_prefix_sums(&self.messages);
    }

    pub(crate) fn build_visible_message_rows(
        &self,
        start_idx: usize,
        end_idx: usize,
        is_group_chat: bool,
        stage_w: f32,
        highlight_paint: Option<(u64, ChatPaintHighlight)>,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        let total_msgs = self.messages.len();
        let selected_set: HashSet<u64> = self.selected_message_ids.iter().copied().collect();
        let has_any_selection = self.select_mode_active || !selected_set.is_empty();
        let mut message_rows =
            Vec::with_capacity(if total_msgs == 0 { 0 } else { end_idx - start_idx + 1 });
        if total_msgs == 0 {
            return message_rows;
        }
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
                self.message_meta
                    .get(&self.messages[idx - 1].id)
                    .and_then(|m| m.date_group_label.as_deref())
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

            let (is_within_viewport, reaction_overlay_layout) =
                self.compute_reaction_overlay_layout(idx, msg, group.reserve_avatar_column);
            let overlay_mode = if is_within_viewport {
                self.reaction_overlay_mode(msg.id)
            } else {
                None
            };

            message_rows.push(render_message_row_1to1(
                msg,
                meta,
                self.active_fabushi_bot.as_ref(),
                group,
                stage_w,
                selected_set.contains(&msg.id),
                has_any_selection,
                overlay_mode,
                reaction_overlay_layout,
                Some(self.bubble_widths_probe.clone()),
                Some(self.bubble_bounds_probe.clone()),
                Some(self.row_heights_probe.clone()),
                hl_alpha,
                self.unread_anchor_msg_id == Some(msg.id),
                date_divider,
                &self.revealed_spoilers,
                &self.expanded_quotes,
                self.copied_code_msg,
                &self.palette,
                &self.lang_id,
                window,
                cx,
                |this, action, cx| this.handle_bubble_action(action, cx),
            ));
        }
        message_rows
    }

    pub(crate) fn sticky_date_label(&self, start_idx: usize) -> Option<String> {
        if self.messages.is_empty() || self.active_chat_id.is_none() {
            return None;
        }
        self.messages.get(start_idx).and_then(|m| {
            self.message_meta
                .get(&m.id)
                .and_then(|rm| rm.date_group_label.clone())
                .or_else(|| {
                    if m.timestamp.contains(' ') {
                        m.timestamp.split(' ').next().map(str::to_string)
                    } else {
                        None
                    }
                })
        })
    }

    pub(crate) fn pinned_banner_info(&self) -> Option<(String, String, Option<String>, usize, usize)> {
        self.active_chat_id?;
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
    }
}
