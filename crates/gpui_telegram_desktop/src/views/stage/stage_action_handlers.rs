//! Theme, jump/search navigation, keyboard, top-bar, bubble, rich-text, context-menu, and overlay handlers for `ConversationStage`.

use std::time::Duration;

use gpui::*;

use crate::i18n::tr_key_or;
use crate::theme::TdesktopThemePalette;
use crate::views::conversation_stage::{stage_now_ms, ConversationStage, MessageContextMenuState};
use crate::views::stage::cards::*;
use crate::views::stage::cards_interactive::InteractiveCardAction;
use crate::views::stage::cards_media::MediaCardAction;
use crate::views::stage::entities_render::RichTextAction;
use crate::views::stage::history_drag_area::DragTargetZone;
use crate::views::stage::history_view_highlight_manager::SelectedQuote;
use crate::views::stage::stage_bars::TopBarUiAction;
use crate::views::stage::stage_bubble::BubbleUiAction;
use crate::views::stage::stage_context_menu::{
    render_message_context_menu, ContextMenuAction, ReactionOverlayMode,
};

pub const K_REACTION_HOVER_GRACE_MS: u64 = 180;

impl ConversationStage {
    pub fn apply_theme(&mut self, palette: TdesktopThemePalette, lang_id: impl Into<String>) {
        if self.palette.font_size_px != palette.font_size_px
            || self.palette.ui_scale_pct != palette.ui_scale_pct
            || self.palette.bubble_radius_px != palette.bubble_radius_px
        {
            self.row_height_cache.mark_dirty();
        }
        self.blur_wallpaper = palette.blur_wallpaper;
        self.palette = palette;
        self.lang_id = lang_id.into();
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

    pub fn recompute_search_matches(&mut self) {
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

    pub fn step_search_match(&mut self, delta: i32) {
        let len = self.search_match_ids.len();
        if len > 0 {
            if delta < 0 {
                self.search_match_cursor = if self.search_match_cursor == 0 {
                    len - 1
                } else {
                    self.search_match_cursor - 1
                };
            } else {
                self.search_match_cursor = (self.search_match_cursor + 1) % len;
            }
            if let Some(&id) = self.search_match_ids.get(self.search_match_cursor) {
                self.jump_to_message(id);
            }
        }
    }

    pub(crate) fn handle_stage_key_down(&mut self, event: &KeyDownEvent, cx: &mut Context<Self>) {
        let ks = &event.keystroke;
        if ks.modifiers.platform && ks.key == "f" && self.active_chat_id.is_some() {
            self.search_open = true;
            cx.notify();
            return;
        }
        if ks.key == "escape" {
            let mut handled = true;
            if self.active_context_menu.is_some() {
                self.active_context_menu = None;
                self.context_menu_reactions_expanded = false;
            } else if self.reaction_picker_msg_id.is_some() {
                self.reaction_picker_msg_id = None;
                self.hover_grace = None;
                self.hover_grace_generation = self.hover_grace_generation.wrapping_add(1);
            } else if self.more_menu_open {
                self.more_menu_open = false;
                self.ttl_submenu_open = false;
            } else if self.search_open {
                self.search_open = false;
                self.search_query.clear();
                self.search_match_ids.clear();
            } else if self.select_mode_active || !self.selected_message_ids.is_empty() {
                self.select_mode_active = false;
                self.selected_message_ids.clear();
            } else {
                handled = false;
            }
            if handled {
                cx.stop_propagation();
                cx.notify();
            }
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

    pub fn apply_topbar_action(&mut self, action: TopBarUiAction) {
        match action {
            TopBarUiAction::ToggleInspector => self.toggle_inspector(),
            TopBarUiAction::ToggleSearch => {
                self.more_menu_open = false;
                self.ttl_submenu_open = false;
                self.search_open = !self.search_open;
                if !self.search_open {
                    self.search_query.clear();
                    self.search_match_ids.clear();
                }
            }
            TopBarUiAction::SearchStep(delta) => self.step_search_match(delta),
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
            TopBarUiAction::ToggleTopMoreMenu => {
                self.more_menu_open = !self.more_menu_open;
                if !self.more_menu_open {
                    self.ttl_submenu_open = false;
                }
            }
            TopBarUiAction::DismissTopMoreMenu => {
                self.more_menu_open = false;
                self.ttl_submenu_open = false;
            }
            TopBarUiAction::ToggleTtlSubmenu => {
                self.ttl_submenu_open = !self.ttl_submenu_open;
            }
            TopBarUiAction::SetMessageTtl(secs) => {
                self.message_ttl_seconds = secs;
                self.ttl_submenu_open = false;
                self.more_menu_open = false;
                self.pending_set_ttl_seconds = Some(secs);
            }
            TopBarUiAction::ViewProfile => {
                self.more_menu_open = false;
                self.ttl_submenu_open = false;
                self.is_inspector_open = true;
                if let Some(cid) = self.active_chat_id {
                    self.pending_open_profile_peer_id = Some(cid);
                }
            }
            TopBarUiAction::ChangeWallpaper => {
                self.more_menu_open = false;
                self.ttl_submenu_open = false;
                self.pending_change_wallpaper = true;
            }
            TopBarUiAction::TogglePinChat => {
                self.more_menu_open = false;
                self.ttl_submenu_open = false;
                self.is_pinned_chat = !self.is_pinned_chat;
                self.pending_toggle_pin_chat = true;
            }
            TopBarUiAction::ToggleBlockPeer => {
                self.more_menu_open = false;
                self.ttl_submenu_open = false;
                if !self.is_group_chat && !self.is_saved_messages {
                    self.pending_toggle_block_peer = true;
                }
            }
            TopBarUiAction::DeleteOrLeaveChat => {
                self.more_menu_open = false;
                self.ttl_submenu_open = false;
                self.pending_delete_or_leave_chat = true;
            }
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
            TopBarUiAction::EnterSelectMode => {
                self.more_menu_open = false;
                self.ttl_submenu_open = false;
                self.select_mode_active = true;
            }
            TopBarUiAction::ClearSelection => {
                self.select_mode_active = false;
                self.selected_message_ids.clear();
            }
            TopBarUiAction::ForwardSelected => {
                if !self.selected_message_ids.is_empty() {
                    self.pending_forward_message_ids = Some(self.selected_message_ids.clone());
                    self.selected_message_ids.clear();
                }
                self.select_mode_active = false;
            }
            TopBarUiAction::CopySelected => {
                self.selected_message_ids.clear();
                self.select_mode_active = false;
            }
            TopBarUiAction::DeleteSelected => {
                if !self.selected_message_ids.is_empty() {
                    let ids = self.selected_message_ids.clone();
                    self.delete_messages_by_ids(&ids);
                    self.sync_pinned_tracker();
                    self.pending_delete_message_ids = Some(ids);
                }
                self.select_mode_active = false;
            }
            TopBarUiAction::ScrollToBottom => {
                self.scroll_to_bottom();
                self.unread_incoming_count = 0;
                self.unread_anchor_msg_id = None;
            }
            TopBarUiAction::ClearHistory => {
                self.more_menu_open = false;
                self.ttl_submenu_open = false;
                self.pending_clear_history = true;
            }
            TopBarUiAction::ExportChat => {
                self.more_menu_open = false;
                self.ttl_submenu_open = false;
                self.pending_export_chat = true;
            }
            TopBarUiAction::ToggleMuteChat => {
                self.more_menu_open = false;
                self.ttl_submenu_open = false;
                self.is_muted_chat = !self.is_muted_chat;
                self.pending_toggle_mute = true;
            }
        }
    }

    pub fn handle_topbar_action(&mut self, action: TopBarUiAction, cx: &mut Context<Self>) {
        if matches!(action, TopBarUiAction::CopySelected) {
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
            self.select_mode_active = false;
            cx.notify();
            return;
        }
        self.apply_topbar_action(action);
        cx.notify();
    }

    fn clear_hover_grace(&mut self) {
        if self.hover_grace.is_some() {
            self.hover_grace = None;
            self.hover_grace_generation = self.hover_grace_generation.wrapping_add(1);
        }
    }

    fn has_any_direct_hover(&self) -> bool {
        self.hovered_message_id.is_some()
            || self.hovered_gutter_id.is_some()
            || self.hovered_reaction_bar_id.is_some()
            || self.hovered_reaction_bridge_id.is_some()
            || self.reaction_picker_msg_id.is_some()
    }

    /// Applies hover, reaction-picker, or reaction-toggle bubble actions without requiring a GPUI window context.
    /// Returns `Some(should_notify)` when handled here, or `None` for other `BubbleUiAction` variants.
    pub fn apply_reaction_ui_action(&mut self, action: &BubbleUiAction) -> Option<bool> {
        match action {
            BubbleUiAction::ToggleReaction(msg_id, emoji) => {
                self.toggle_reaction(*msg_id, emoji);
                self.reaction_picker_msg_id = None;
                self.hovered_reaction_bar_id = None;
                self.hovered_reaction_bridge_id = None;
                self.quick_strip_msg_id = None;
                self.clear_hover_grace();
                self.enqueue_reaction_toggle(*msg_id, emoji.clone());
                Some(true)
            }
            BubbleUiAction::ToggleReactionPicker(msg_id) => {
                self.clear_hover_grace();
                self.quick_strip_msg_id = None;
                if self.reaction_picker_msg_id == Some(*msg_id) {
                    self.reaction_picker_msg_id = None;
                } else {
                    self.reaction_picker_msg_id = Some(*msg_id);
                }
                Some(true)
            }
            BubbleUiAction::SetHover(hovered) => {
                self.clear_hover_grace();
                let prev_qs = self.quick_strip_msg_id;
                if *hovered != self.quick_strip_msg_id {
                    self.quick_strip_msg_id = None;
                }
                if self.hovered_message_id != *hovered || self.quick_strip_msg_id != prev_qs {
                    self.hovered_message_id = *hovered;
                    Some(true)
                } else {
                    Some(false)
                }
            }
            BubbleUiAction::SetBubbleHover(msg_id, hovered) => {
                let prev_qs = self.quick_strip_msg_id;
                if *hovered {
                    let keep_strip =
                        self.reaction_overlay_mode(*msg_id) == Some(ReactionOverlayMode::QuickStrip);
                    self.clear_hover_grace();
                    if !keep_strip {
                        self.quick_strip_msg_id = None;
                    }
                }
                let next = if *hovered {
                    Some(*msg_id)
                } else if self.hovered_message_id == Some(*msg_id) {
                    None
                } else {
                    self.hovered_message_id
                };
                if self.hovered_message_id != next || self.quick_strip_msg_id != prev_qs {
                    self.hovered_message_id = next;
                    Some(true)
                } else {
                    Some(false)
                }
            }
            BubbleUiAction::SetBubbleGutterHover(msg_id, hovered) => {
                let prev_qs = self.quick_strip_msg_id;
                if *hovered {
                    let keep_strip =
                        self.reaction_overlay_mode(*msg_id) == Some(ReactionOverlayMode::QuickStrip);
                    self.clear_hover_grace();
                    if !keep_strip {
                        self.quick_strip_msg_id = None;
                    }
                }
                let next = if *hovered {
                    Some(*msg_id)
                } else if self.hovered_gutter_id == Some(*msg_id) {
                    None
                } else {
                    self.hovered_gutter_id
                };
                if self.hovered_gutter_id != next || self.quick_strip_msg_id != prev_qs {
                    self.hovered_gutter_id = next;
                    Some(true)
                } else {
                    Some(false)
                }
            }
            BubbleUiAction::SetReactionBarHover(msg_id, hovered) => {
                let prev_qs = self.quick_strip_msg_id;
                if *hovered {
                    self.clear_hover_grace();
                    self.quick_strip_msg_id = Some(*msg_id);
                }
                let next = if *hovered {
                    Some(*msg_id)
                } else if self.hovered_reaction_bar_id == Some(*msg_id) {
                    None
                } else {
                    self.hovered_reaction_bar_id
                };
                if self.hovered_reaction_bar_id != next || self.quick_strip_msg_id != prev_qs {
                    self.hovered_reaction_bar_id = next;
                    Some(true)
                } else {
                    Some(false)
                }
            }
            BubbleUiAction::SetReactionBridgeHover(msg_id, hovered) => {
                let prev_qs = self.quick_strip_msg_id;
                if *hovered {
                    self.clear_hover_grace();
                    self.quick_strip_msg_id = Some(*msg_id);
                }
                let next = if *hovered {
                    Some(*msg_id)
                } else if self.hovered_reaction_bridge_id == Some(*msg_id) {
                    None
                } else {
                    self.hovered_reaction_bridge_id
                };
                if self.hovered_reaction_bridge_id != next || self.quick_strip_msg_id != prev_qs {
                    self.hovered_reaction_bridge_id = next;
                    Some(true)
                } else {
                    Some(false)
                }
            }
            _ => None,
        }
    }

    pub fn is_reaction_overlay_visible(&self, msg_id: u64) -> bool {
        self.reaction_overlay_mode(msg_id).is_some()
    }

    pub fn handle_bubble_action(&mut self, action: BubbleUiAction, cx: &mut Context<Self>) {
        let hover_leave_candidate = match &action {
            BubbleUiAction::SetBubbleHover(mid, false)
            | BubbleUiAction::SetBubbleGutterHover(mid, false)
            | BubbleUiAction::SetReactionBarHover(mid, false)
            | BubbleUiAction::SetReactionBridgeHover(mid, false) => {
                self.reaction_overlay_mode(*mid).map(|mode| (*mid, mode))
            }
            _ => None,
        };

        if let Some(should_notify) = self.apply_reaction_ui_action(&action) {
            if let Some((leave_mid, prev_mode)) = hover_leave_candidate {
                if !self.has_any_direct_hover()
                    && self.selected_message_ids.is_empty()
                    && self.active_context_menu.is_none()
                    && matches!(
                        prev_mode,
                        ReactionOverlayMode::CornerButton | ReactionOverlayMode::QuickStrip
                    )
                {
                    self.hover_grace = Some((leave_mid, prev_mode));
                    self.hover_grace_generation = self.hover_grace_generation.wrapping_add(1);
                    let grace_gen = self.hover_grace_generation;
                    cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                        app.background_executor()
                            .timer(Duration::from_millis(K_REACTION_HOVER_GRACE_MS))
                            .await;
                        let _ = weak.update(app, |this, cx| {
                            if this.hover_grace_generation == grace_gen {
                                this.hover_grace = None;
                                if !this.has_any_direct_hover() {
                                    this.quick_strip_msg_id = None;
                                }
                                cx.notify();
                            }
                        });
                    })
                    .detach();
                }
            }
            if should_notify {
                cx.notify();
            }
            return;
        }
        match action {
            BubbleUiAction::ClickReplyQuote(target_id) => self.jump_to_message(target_id),
            BubbleUiAction::ToggleReaction(..)
            | BubbleUiAction::ToggleReactionPicker(..)
            | BubbleUiAction::SetHover(..)
            | BubbleUiAction::SetBubbleHover(..)
            | BubbleUiAction::SetBubbleGutterHover(..)
            | BubbleUiAction::SetReactionBarHover(..)
            | BubbleUiAction::SetReactionBridgeHover(..) => unreachable!(),
            BubbleUiAction::ToggleSelect(msg_id) => {
                self.reaction_picker_msg_id = None;
                self.quick_strip_msg_id = None;
                self.clear_hover_grace();
                self.toggle_select_message(msg_id);
            }
            BubbleUiAction::OpenContextMenu(msg_id, pos) => {
                self.reaction_picker_msg_id = None;
                self.hovered_reaction_bar_id = None;
                self.hovered_reaction_bridge_id = None;
                self.quick_strip_msg_id = None;
                self.clear_hover_grace();
                let local_pos = if let Some(b) = self.resolved_stage_bounds() {
                    Point::new(pos.x - b.origin.x, pos.y - b.origin.y)
                } else {
                    pos
                };
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
                self.context_menu_reactions_expanded = false;
                self.active_context_menu = Some(MessageContextMenuState {
                    message_id: msg_id,
                    position: local_pos,
                    is_outgoing,
                    is_pinned,
                    text_snippet,
                });
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
                InteractiveCardAction::BotButton(msg_id, btn) => {
                    self.pending_bot_button_action = Some((msg_id, btn));
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

    pub fn handle_context_action(&mut self, action: ContextMenuAction, cx: &mut Context<Self>) {
        match action {
            ContextMenuAction::ToggleExpandReactions => {
                if self.active_context_menu.is_some() {
                    self.context_menu_reactions_expanded = !self.context_menu_reactions_expanded;
                }
                cx.notify();
                return;
            }
            ContextMenuAction::React(msg_id, emoji) => {
                self.toggle_reaction(msg_id, &emoji);
                self.reaction_picker_msg_id = None;
                self.hovered_reaction_bar_id = None;
                self.enqueue_reaction_toggle(msg_id, emoji);
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
        self.context_menu_reactions_expanded = false;
        cx.notify();
    }

    pub(crate) fn render_sticky_date_pill(&self, date_lbl: String) -> AnyElement {
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
            )
            .into_any_element()
    }

    pub(crate) fn render_toast_banner_pill(
        &self,
        toast: String,
        cx: &mut Context<Self>,
    ) -> AnyElement {
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
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            this.toast_banner = None;
                            cx.notify();
                        }),
                    )
                    .child(toast),
            )
            .into_any_element()
    }

    pub(crate) fn render_drag_area_overlay(&self) -> AnyElement {
        let target_files = matches!(self.drag_area.hovered_zone(), Some(DragTargetZone::Document));
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
            .child(
                div()
                    .text_size(px(15.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(self.palette.text_primary))
                    .child(if target_files {
                        tr_key_or(&self.lang_id, "lng_drag_files_here", "Drop files to send uncompressed")
                    } else {
                        tr_key_or(&self.lang_id, "lng_drag_photos_here", "Drop photos or videos to send")
                    }),
            )
            .into_any_element()
    }

    pub(crate) fn render_active_context_menu_overlay(
        &self,
        menu: MessageContextMenuState,
        cx: &mut Context<Self>,
    ) -> AnyElement {
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
        let chosen_emojis: Vec<String> = self
            .messages
            .iter()
            .find(|m| m.id == menu.message_id)
            .map(|m| {
                m.reactions
                    .iter()
                    .filter(|r| r.user_reacted)
                    .map(|r| r.emoji.clone())
                    .collect()
            })
            .unwrap_or_default();
        let stage_size = self.resolved_stage_bounds().map(|b| b.size);
        render_message_context_menu(
            menu.message_id,
            menu.position,
            stage_size,
            menu.is_outgoing,
            menu.is_pinned,
            can_edit,
            is_translated,
            self.context_menu_reactions_expanded,
            &chosen_emojis,
            &self.palette,
            &self.lang_id,
            cx,
            |this, action, cx| this.handle_context_action(action, cx),
        )
    }
}
