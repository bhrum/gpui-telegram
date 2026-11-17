//! Authoritative PTS `UpdateEvent` application, sender identity resolution,
//! and incoming message/reaction/call state synchronization for `AppShell`.

use std::collections::HashSet;

use gpui::{App, AppContext, AsyncApp, Context, WeakEntity};
use gpui_telegram_core::types::{ChatId, MessageId};
use gpui_telegram_proto::contracts::CallSignalRequest as CallSignalPayload;
use gpui_telegram_proto::message::{MessageItem as Message, ReactionCount};
use gpui_telegram_proto::sync::{TypingAction, UpdateEvent};
use gpui_telegram_webrtc::SignalingFrame;

use crate::i18n::tr_str;
use crate::views::app_shell::shell_actions::peer_avatar_color;
use crate::views::app_shell::shell_settings_sync::apply_folders_to_shell;
use crate::views::app_shell::shell_state::reconcile_stage_message_list;
use crate::views::app_shell::shell_truth::{
    decode_inline_keyboard_json, derive_initials, overlay_rich_message_metadata,
    proto_keyboard_to_stage, SessionRequestFence,
};
use crate::views::call_overlay::TalkModeState;
use crate::views::conversation_stage::PinnedBannerEntry;
use crate::views::shell::AppShell;
use crate::views::stage::cards::{
    DeliveryStatus, InlineKeyboardButton, MessageContent, MessageDirection, MessageItemModel,
    ReactionItem, ServiceType,
};
use crate::views::{CallOverlay, ConversationStage};

fn with_initials(name: String, color: u32) -> (String, String, u32) {
    let init = derive_initials(&name);
    (name, if init.is_empty() { "TG".to_string() } else { init }, color)
}

fn format_typing_indicator(
    lang: &str,
    sender_name: &str,
    action: TypingAction,
) -> Option<String> {
    let verb = match action {
        TypingAction::Cancel => return None,
        TypingAction::Typing => tr_str(lang, "正在思考与输入...", "is typing..."),
        TypingAction::RecordAudio => tr_str(lang, "正在录制语音...", "is recording audio..."),
        TypingAction::UploadAudio => tr_str(lang, "正在发送语音...", "is sending audio..."),
        TypingAction::RecordVideo => tr_str(lang, "正在录制视频...", "is recording video..."),
        TypingAction::UploadVideo => tr_str(lang, "正在发送视频...", "is sending video..."),
        TypingAction::RecordRound => {
            tr_str(lang, "正在录制视频消息...", "is recording video message...")
        }
        TypingAction::UploadRound => {
            tr_str(lang, "正在发送视频消息...", "is sending video message...")
        }
        TypingAction::UploadPhoto => tr_str(lang, "正在发送图片...", "is sending photo..."),
        TypingAction::UploadDocument => tr_str(lang, "正在发送文件...", "is sending file..."),
        TypingAction::ChooseSticker => tr_str(lang, "正在选择贴纸...", "is choosing a sticker..."),
    };
    let name = sender_name.trim();
    if name.is_empty() {
        Some(verb.to_string())
    } else {
        Some(format!("{name} {verb}"))
    }
}

fn strip_state_patch_blocks(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut rest = input;
    while let Some(start) = rest.find("[STATE_PATCH]") {
        out.push_str(&rest[..start]);
        let after_open = &rest[start + "[STATE_PATCH]".len()..];
        if let Some(end) = after_open.find("[/STATE_PATCH]") {
            rest = &after_open[end + "[/STATE_PATCH]".len()..];
        } else {
            rest = "";
            break;
        }
    }
    out.push_str(rest);
    out
}

fn strip_think_blocks(input: &str) -> (String, String) {
    let mut outside = String::with_capacity(input.len());
    let mut inside = String::new();
    let mut rest = input;
    while let Some(start) = rest.find("<think>") {
        outside.push_str(&rest[..start]);
        let after_open = &rest[start + "<think>".len()..];
        if let Some(end) = after_open.find("</think>") {
            let chunk = after_open[..end].trim();
            if !chunk.is_empty() {
                if !inside.is_empty() {
                    inside.push(' ');
                }
                inside.push_str(chunk);
            }
            rest = &after_open[end + "</think>".len()..];
        } else {
            let chunk = after_open.trim();
            if !chunk.is_empty() {
                if !inside.is_empty() {
                    inside.push(' ');
                }
                inside.push_str(chunk);
            }
            rest = "";
            break;
        }
    }
    outside.push_str(rest);
    (outside, inside)
}

fn is_horizontal_rule(trimmed: &str) -> bool {
    let non_space: Vec<char> = trimmed.chars().filter(|c| !c.is_whitespace()).collect();
    if non_space.len() < 3 {
        return false;
    }
    let first = non_space[0];
    matches!(first, '-' | '*' | '_') && non_space.iter().all(|&c| c == first)
}

fn normalize_header_probe(trimmed: &str) -> String {
    let no_heading = trimmed.trim_start_matches('#').trim_start();
    no_heading.replace("**", "").replace("__", "").replace('`', "")
}

fn is_thinking_header(trimmed: &str) -> bool {
    let h = normalize_header_probe(trimmed);
    h.starts_with("💭 思考")
        || h.starts_with("💭思考")
        || h.starts_with("💭 正在思考")
        || h.starts_with("💭 Thinking")
        || h.starts_with("💭 Thought")
        || h == "💭"
}

fn is_steps_header(trimmed: &str) -> bool {
    let h = normalize_header_probe(trimmed);
    h.starts_with("🔧 执行步骤")
        || h.starts_with("🔧执行步骤")
        || h.starts_with("🔧 工具")
        || h.starts_with("🔧 Tool")
        || h.starts_with("🔧 Steps")
}

fn is_step_list_line(raw_line: &str, trimmed: &str) -> bool {
    if trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("• ") {
        return true;
    }
    let bytes = trimmed.as_bytes();
    let mut digits = 0usize;
    while digits < bytes.len() && bytes[digits].is_ascii_digit() {
        digits += 1;
    }
    if digits > 0
        && digits + 1 < bytes.len()
        && matches!(bytes[digits], b'.' | b')')
        && bytes[digits + 1] == b' '
    {
        return true;
    }
    !trimmed.is_empty() && (raw_line.starts_with("  ") || raw_line.starts_with('\t'))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum StreamHeaderState {
    Start,
    InThinkingImmediate,
    InThinkingQuoted,
    AfterThinkingBlank,
    InStepsHeader,
    InStepsList,
    AfterSeparator,
}

fn select_preview_source(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let mut state = StreamHeaderState::Start;
    let mut last_header_idx: Option<usize> = None;
    let mut answer_start_idx: Option<usize> = None;

    for (idx, &line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        match state {
            StreamHeaderState::Start => {
                if trimmed.is_empty() {
                    continue;
                }
                if is_thinking_header(trimmed) {
                    last_header_idx = Some(idx);
                    state = StreamHeaderState::InThinkingImmediate;
                } else if is_steps_header(trimmed) {
                    last_header_idx = Some(idx);
                    state = StreamHeaderState::InStepsHeader;
                } else {
                    answer_start_idx = Some(idx);
                    break;
                }
            }
            StreamHeaderState::InThinkingImmediate => {
                if trimmed.is_empty() {
                    state = StreamHeaderState::AfterThinkingBlank;
                } else if is_horizontal_rule(trimmed) {
                    state = StreamHeaderState::AfterSeparator;
                } else if is_steps_header(trimmed) {
                    last_header_idx = Some(idx);
                    state = StreamHeaderState::InStepsHeader;
                } else if is_thinking_header(trimmed) {
                    last_header_idx = Some(idx);
                    state = StreamHeaderState::InThinkingImmediate;
                } else if trimmed.starts_with('>') {
                    state = StreamHeaderState::InThinkingQuoted;
                }
            }
            StreamHeaderState::InThinkingQuoted => {
                if trimmed.is_empty() {
                    state = StreamHeaderState::AfterThinkingBlank;
                } else if is_horizontal_rule(trimmed) {
                    state = StreamHeaderState::AfterSeparator;
                } else if is_steps_header(trimmed) {
                    last_header_idx = Some(idx);
                    state = StreamHeaderState::InStepsHeader;
                } else if is_thinking_header(trimmed) {
                    last_header_idx = Some(idx);
                    state = StreamHeaderState::InThinkingImmediate;
                } else if trimmed.starts_with('>') {
                    state = StreamHeaderState::InThinkingQuoted;
                } else {
                    answer_start_idx = Some(idx);
                    break;
                }
            }
            StreamHeaderState::AfterThinkingBlank => {
                if trimmed.is_empty() {
                    continue;
                } else if is_horizontal_rule(trimmed) {
                    state = StreamHeaderState::AfterSeparator;
                } else if is_steps_header(trimmed) {
                    last_header_idx = Some(idx);
                    state = StreamHeaderState::InStepsHeader;
                } else if is_thinking_header(trimmed) {
                    last_header_idx = Some(idx);
                    state = StreamHeaderState::InThinkingImmediate;
                } else if trimmed.starts_with('>') {
                    state = StreamHeaderState::InThinkingQuoted;
                } else {
                    answer_start_idx = Some(idx);
                    break;
                }
            }
            StreamHeaderState::InStepsHeader => {
                if trimmed.is_empty() {
                    continue;
                } else if is_horizontal_rule(trimmed) {
                    state = StreamHeaderState::AfterSeparator;
                } else if is_steps_header(trimmed) {
                    last_header_idx = Some(idx);
                    state = StreamHeaderState::InStepsHeader;
                } else if is_thinking_header(trimmed) {
                    last_header_idx = Some(idx);
                    state = StreamHeaderState::InThinkingImmediate;
                } else if is_step_list_line(line, trimmed) {
                    state = StreamHeaderState::InStepsList;
                } else {
                    answer_start_idx = Some(idx);
                    break;
                }
            }
            StreamHeaderState::InStepsList => {
                if trimmed.is_empty() || is_horizontal_rule(trimmed) {
                    state = StreamHeaderState::AfterSeparator;
                } else if is_steps_header(trimmed) {
                    last_header_idx = Some(idx);
                    state = StreamHeaderState::InStepsHeader;
                } else if is_thinking_header(trimmed) {
                    last_header_idx = Some(idx);
                    state = StreamHeaderState::InThinkingImmediate;
                } else if is_step_list_line(line, trimmed) {
                    state = StreamHeaderState::InStepsList;
                } else {
                    answer_start_idx = Some(idx);
                    break;
                }
            }
            StreamHeaderState::AfterSeparator => {
                if trimmed.is_empty() || is_horizontal_rule(trimmed) {
                    continue;
                } else if is_steps_header(trimmed) {
                    last_header_idx = Some(idx);
                    state = StreamHeaderState::InStepsHeader;
                } else if is_thinking_header(trimmed) {
                    last_header_idx = Some(idx);
                    state = StreamHeaderState::InThinkingImmediate;
                } else {
                    answer_start_idx = Some(idx);
                    break;
                }
            }
        }
    }

    if let Some(start) = answer_start_idx {
        lines[start..].join("\n")
    } else if let Some(hdr) = last_header_idx {
        lines[hdr..].join("\n")
    } else {
        text.to_string()
    }
}

fn replace_markdown_links(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut rest = line;
    while let Some(open_bracket) = rest.find('[') {
        out.push_str(&rest[..open_bracket]);
        let after_bracket = &rest[open_bracket + 1..];
        if let Some(close_bracket) = after_bracket.find(']') {
            let after_close = &after_bracket[close_bracket + 1..];
            if after_close.starts_with('(') {
                let after_paren = &after_close[1..];
                if let Some(close_paren) = after_paren.find(')') {
                    let label = after_bracket[..close_bracket].trim();
                    let url = after_paren[..close_paren].trim();
                    if !label.is_empty() {
                        out.push_str(label);
                    } else {
                        out.push_str(url);
                    }
                    rest = &after_paren[close_paren + 1..];
                    continue;
                }
            }
        }
        out.push('[');
        rest = after_bracket;
    }
    out.push_str(rest);
    out
}

fn strip_line_markdown_noise(raw_line: &str) -> Option<String> {
    let mut s = raw_line.trim();
    if s.is_empty() || s.starts_with("```") || is_horizontal_rule(s) {
        return None;
    }
    let hashes = s.bytes().take_while(|&b| b == b'#').count();
    if (1..=6).contains(&hashes) && s[hashes..].starts_with(char::is_whitespace) {
        s = s[hashes..].trim_start();
    }
    while let Some(after_gt) = s.strip_prefix('>') {
        s = after_gt.trim_start();
    }
    if let Some(after_bullet) = s
        .strip_prefix("- ")
        .or_else(|| s.strip_prefix("* "))
        .or_else(|| s.strip_prefix("• "))
    {
        s = after_bullet.trim_start();
    }
    if s.is_empty() {
        return None;
    }
    let unlinked = replace_markdown_links(s);
    let cleaned = unlinked
        .replace("**", "")
        .replace("__", "")
        .replace("~~", "")
        .replace("||", "")
        .replace('`', "");
    let trimmed = cleaned.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn truncate_preview(text: &str, max_chars: usize) -> String {
    let mut iter = text.char_indices();
    if let Some((byte_idx, _)) = iter.nth(max_chars) {
        let truncated = text[..byte_idx].trim_end();
        format!("{truncated}...")
    } else {
        text.to_string()
    }
}

pub(crate) fn format_sidebar_preview(raw: &str) -> String {
    const MAX_PREVIEW_CHARS: usize = 160;
    let without_patch = strip_state_patch_blocks(raw);
    let (outside_think, inside_think) = strip_think_blocks(&without_patch);
    let base = if !outside_think.trim().is_empty() {
        outside_think
    } else if !inside_think.trim().is_empty() {
        inside_think
    } else {
        without_patch
    };

    let selected = select_preview_source(&base);
    let mut parts = Vec::new();
    for line in selected.lines() {
        if let Some(clean_line) = strip_line_markdown_noise(line) {
            parts.push(clean_line);
        }
    }
    let joined = parts.join(" ");
    let collapsed: String = joined.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.is_empty() {
        let fallback: String = base.split_whitespace().collect::<Vec<_>>().join(" ");
        return truncate_preview(&fallback, MAX_PREVIEW_CHARS);
    }
    truncate_preview(&collapsed, MAX_PREVIEW_CHARS)
}

fn extract_stage_message_snippet(msg: &MessageItemModel) -> Option<String> {
    match &msg.content {
        MessageContent::Text { text, .. } => Some(text.lines().next().unwrap_or(text).to_string()),
        MessageContent::Media { caption, .. } | MessageContent::Album { caption, .. } => {
            Some(caption.clone().unwrap_or_else(|| "Media".to_string()))
        }
        MessageContent::Poll { question, .. } => Some(question.clone()),
        MessageContent::Invoice { title, .. } => Some(title.clone()),
        MessageContent::Service { text, .. } => Some(text.clone()),
        _ => Some("Pinned message".to_string()),
    }
}

fn apply_text_edit_to_stage(
    st: &mut ConversationStage,
    chat_u64: u64,
    target_id: u64,
    text: &str,
    updated_kb: Option<Option<Vec<Vec<InlineKeyboardButton>>>>,
) -> bool {
    let mut is_latest_message = false;
    let is_active_chat = st.active_chat_id == Some(chat_u64);
    if is_active_chat {
        let was_anchored = st.is_anchored_to_bottom || st.is_at_bottom();
        if let Some(m) = st.messages.iter_mut().find(|m| m.id == target_id) {
            m.content = MessageContent::Text {
                text: text.to_string(),
                is_edited: true,
            };
            if let Some(ref kb) = updated_kb {
                m.bot_keyboard = kb.clone();
            }
            st.row_heights_probe.borrow_mut().remove(&target_id);
            st.row_height_cache.measured_heights.remove(&target_id);
            st.row_height_cache.mark_dirty();
        }
        if st.messages.last().is_some_and(|m| m.id == target_id) {
            is_latest_message = true;
            if was_anchored {
                st.scroll_to_bottom();
            }
        }
    }
    if let Some(history) = st.chat_histories.get_mut(&chat_u64) {
        if let Some(m) = history.iter_mut().find(|m| m.id == target_id) {
            m.content = MessageContent::Text {
                text: text.to_string(),
                is_edited: true,
            };
            if let Some(ref kb) = updated_kb {
                m.bot_keyboard = kb.clone();
            }
        }
        if (!is_active_chat || st.messages.is_empty())
            && history.last().is_some_and(|m| m.id == target_id)
        {
            is_latest_message = true;
        }
    }
    is_latest_message
}

/// Evaluates whether an incoming message should trigger a desktop notification
/// according to global preferences, chat category preferences, mute state, and window focus.
pub fn should_dispatch_message_notification(
    is_outgoing: bool,
    is_current_chat: bool,
    window_is_focused: bool,
    is_muted: bool,
    is_channel: bool,
    is_group: bool,
    prefs: &crate::app::preferences::NotificationPreferences,
) -> bool {
    if is_outgoing || !prefs.enabled || is_muted {
        return false;
    }
    let category_enabled = if is_channel {
        prefs.channels
    } else if is_group {
        prefs.groups
    } else {
        prefs.private_chats
    };
    if !category_enabled {
        return false;
    }
    !(is_current_chat && window_is_focused)
}

impl AppShell {
    fn apply_pinned_ids_update(
        &mut self,
        chat_u64: u64,
        ids: Vec<i64>,
        cx: &mut Context<Self>,
    ) {
        if chat_u64 == 0 {
            return;
        }
        let lang = self.active_lang_id().to_string();
        let latest_id_u64 = ids.first().and_then(|&id| u64::try_from(id).ok());
        let mut needs_http_hydrate = false;
        let mut sidebar_notice: Option<String> = None;

        self.stage.update(cx, |st, st_cx| {
            let is_active = st.active_chat_id == Some(chat_u64);
            let existing_target = latest_id_u64.and_then(|pid| {
                if is_active {
                    st.messages
                        .iter()
                        .find(|m| m.id == pid)
                        .or_else(|| {
                            st.chat_histories
                                .get(&chat_u64)
                                .and_then(|h| h.iter().find(|m| m.id == pid))
                        })
                } else {
                    st.chat_histories
                        .get(&chat_u64)
                        .and_then(|h| h.iter().find(|m| m.id == pid))
                }
            });
            let was_already_pinned = existing_target.is_some_and(|m| m.is_pinned);
            let pinned_info = existing_target.and_then(|m| {
                extract_stage_message_snippet(m).map(|s| (s, m.sender_name.clone()))
            });

            if let Some(history) = st.chat_histories.get_mut(&chat_u64) {
                for m in history.iter_mut() {
                    m.is_pinned = ids.contains(&(m.id as i64));
                }
            }
            if is_active {
                st.pinned_tracker.set_pinned_ids(ids.clone());
                for m in &mut st.messages {
                    m.is_pinned = ids.contains(&(m.id as i64));
                }
            }

            if ids.is_empty() {
                st.chat_pinned_messages.insert(chat_u64, None);
                if is_active {
                    st.pinned_message = None;
                }
            } else if let Some((ref snippet, ref raw_sender)) = pinned_info {
                st.chat_pinned_messages
                    .insert(chat_u64, Some(snippet.clone()));
                if is_active {
                    st.pinned_message = Some(snippet.clone());
                }
                if !was_already_pinned {
                    if let Some(new_pin_id) = latest_id_u64 {
                        let sender_label = if !raw_sender.trim().is_empty() {
                            raw_sender.clone()
                        } else if is_active && !st.chat_title.trim().is_empty() {
                            st.chat_title.clone()
                        } else {
                            "Bot".to_string()
                        };
                        let short_snippet = truncate_preview(snippet, 48);
                        let notice_text = if lang.starts_with("zh") {
                            format!("{sender_label} 置顶了「{short_snippet}」")
                        } else {
                            format!("{sender_label} pinned \"{short_snippet}\"")
                        };
                        sidebar_notice = Some(format!("📌 {notice_text}"));
                        let service_id = 0x7FFF_0000_0000_0000 | new_pin_id;
                        let service_msg = MessageItemModel {
                            id: service_id,
                            sender_id: 0,
                            sender_name: sender_label,
                            avatar_initials: String::new(),
                            avatar_color: 0,
                            direction: MessageDirection::Incoming,
                            timestamp: String::new(),
                            delivery_status: DeliveryStatus::Read,
                            reply_to: None,
                            reactions: Vec::new(),
                            content: MessageContent::Service {
                                text: notice_text,
                                service_type: ServiceType::PinnedMessage,
                            },
                            bot_keyboard: None,
                            is_pinned: false,
                        };
                        if is_active {
                            if !st.messages.iter().any(|m| m.id == service_id) {
                                st.add_message(service_msg);
                            }
                        } else {
                            let hist = st.chat_histories.entry(chat_u64).or_default();
                            if !hist.iter().any(|m| m.id == service_id) {
                                hist.push(service_msg);
                            }
                        }
                    }
                }
            } else {
                needs_http_hydrate = true;
            }
            st_cx.notify();
        });

        if let Some(preview) = sidebar_notice {
            self.sidebar.update(cx, |sb, sb_cx| {
                if let Some(dialog) = sb.dialogs.iter_mut().find(|d| d.id == chat_u64) {
                    dialog.last_message = preview;
                    sb_cx.notify();
                }
            });
        }

        if needs_http_hydrate {
            if let (Some(token), Some(uid)) = (
                self.session_sync.session_token.clone(),
                self.session_sync.user_id,
            ) {
                let fence = SessionRequestFence {
                    auth_generation: self.session_sync.auth_generation,
                    request_generation: 0,
                    token,
                    user_id: uid,
                };
                let client = self.client.clone();
                let ids_clone = ids;
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    if let Ok(pins) = client.list_pinned_messages(chat_u64 as i64).await {
                        let _ = weak.update(app, |shell, cx| {
                            if !fence.matches(
                                shell.is_authenticated,
                                shell.session_sync.auth_generation,
                                0,
                                shell.session_sync.session_token.as_deref(),
                                shell.session_sync.user_id,
                            ) {
                                return;
                            }
                            let entries: Vec<PinnedBannerEntry> = pins
                                .into_iter()
                                .filter_map(|p| {
                                    let msg_id = u64::try_from(p.message_id).ok()?;
                                    let preview = if p.preview_text.trim().is_empty() {
                                        format!("Pinned message #{msg_id}")
                                    } else {
                                        p.preview_text
                                    };
                                    Some(PinnedBannerEntry {
                                        msg_id,
                                        preview_text: preview,
                                        sender_name: String::new(),
                                        thumb_url: None,
                                    })
                                })
                                .collect();
                            shell.stage.update(cx, |st, st_cx| {
                                st.set_pinned_entries(chat_u64, entries);
                                if st.active_chat_id == Some(chat_u64) && !ids_clone.is_empty() {
                                    for id in &ids_clone {
                                        if !st.pinned_tracker.pinned_ids.contains(id) {
                                            st.pinned_tracker.pinned_ids.push(*id);
                                        }
                                    }
                                    st.pinned_tracker.reset_to_latest();
                                }
                                st_cx.notify();
                            });
                        });
                    }
                })
                .detach();
            }
        }
    }

    fn has_stage_or_history_message(&self, chat_id: u64, msg_id: u64, cx: &App) -> bool {
        let st = self.stage.read(cx);
        let in_active = st.active_chat_id == Some(chat_id)
            && st.messages.iter().any(|m| m.id == msg_id);
        in_active || st.chat_histories.get(&chat_id).is_some_and(|h| h.iter().any(|m| m.id == msg_id))
    }

    fn resolve_reaction_target_chat_id(
        &self,
        fallback_chat_id: ChatId,
        ev_chat_id: ChatId,
        msg_u64: u64,
        cx: &App,
    ) -> u64 {
        if ev_chat_id.0 > 0 {
            return ev_chat_id.0 as u64;
        }
        let fallback_u64 = u64::try_from(fallback_chat_id.0).unwrap_or(0);
        if fallback_u64 > 0 && self.has_stage_or_history_message(fallback_u64, msg_u64, cx) {
            return fallback_u64;
        }
        {
            let st = self.stage.read(cx);
            if let Some(active) = st.active_chat_id {
                if st.messages.iter().any(|m| m.id == msg_u64) {
                    return active;
                }
            }
            for (&cid, hist) in &st.chat_histories {
                if cid > 0 && hist.iter().any(|m| m.id == msg_u64) {
                    return cid;
                }
            }
        }
        if let Some(found) = self
            .client
            .local_store
            .find_chat_id_for_message(MessageId(msg_u64 as i64))
        {
            if found.0 > 0 {
                return found.0 as u64;
            }
        }
        if fallback_u64 > 0 {
            fallback_u64
        } else {
            self.stage.read(cx).active_chat_id.unwrap_or(0)
        }
    }

    /// Records the highest WebSocket PTS seen for a message's reaction updates.
    pub fn record_ws_reaction_pts(&mut self, fallback_chat_id: u64, pts: u64, event: &UpdateEvent) {
        if pts == 0 {
            return;
        }
        if let UpdateEvent::MessageReactions(ev) | UpdateEvent::ReactionUpdated(ev) = event {
            let cid = if ev.chat_id.0 > 0 { ev.chat_id.0 as u64 } else { fallback_chat_id };
            let mid = ev.message_id.0 as u64;
            let entry = self.session_sync.reaction_pts_by_msg.entry((cid, mid)).or_insert(0);
            if pts > *entry {
                *entry = pts;
            }
        }
    }

    /// Resolves `(display_name, initials, avatar_color)` for an incoming message sender from contacts, rich chats, sidebar, or stage.
    pub fn resolve_incoming_sender_identity(
        &self,
        chat_u64: u64,
        sender_id: u64,
        cx: &gpui::App,
    ) -> (String, String, u32) {
        let color = peer_avatar_color(sender_id);
        if let Some(c) = self.session_sync.contacts_list.iter().find(|c| c.user_id == sender_id) {
            let mut full = c.first_name.trim().to_string();
            if let Some(l) = c.last_name.as_deref().map(str::trim).filter(|l| !l.is_empty()) {
                if !full.is_empty() {
                    full.push(' ');
                }
                full.push_str(l);
            }
            if full.is_empty() {
                if let Some(u) = c.username.as_deref().map(str::trim) {
                    full = u.to_string();
                }
            }
            if !full.is_empty() {
                return with_initials(full, color);
            }
        }
        if let Some(rc) = self.session_sync.rich_chats_by_id.get(&chat_u64) {
            if rc.peer_user_id == Some(sender_id) || rc.is_bot || (!rc.is_group && !rc.is_channel) {
                let candidate = rc
                    .peer_display_name
                    .as_deref()
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .or_else(|| Some(rc.title.trim()).filter(|t| !t.is_empty()));
                if let Some(name) = candidate {
                    return with_initials(name.to_string(), color);
                }
            }
        }
        let sb = self.sidebar.read(cx);
        if let Some(d) = sb.dialogs.iter().find(|d| d.id == chat_u64) {
            let is_group_or_channel = d.folder == "Groups" || d.folder == "Channels";
            if (!is_group_or_channel || sender_id == chat_u64) && !d.peer_name.trim().is_empty() {
                return with_initials(d.peer_name.clone(), d.avatar_color);
            }
        }
        if sender_id > 0 {
            if let Some(bot) = self
                .miniapps
                .my_bots
                .iter()
                .chain(self.miniapps.attach_menu_bots.iter())
                .find(|b| b.bot_user_id > 0 && b.bot_user_id as u64 == sender_id)
            {
                let name = bot.display_name.trim();
                if !name.is_empty() {
                    return with_initials(name.to_string(), color);
                }
                let uname = bot.username.trim();
                if !uname.is_empty() {
                    return with_initials(uname.to_string(), color);
                }
            }
        }
        let st = self.stage.read(cx);
        if st.active_chat_id == Some(chat_u64) && !st.chat_title.trim().is_empty() {
            return with_initials(st.chat_title.clone(), st.chat_avatar_color);
        }
        with_initials(format!("User {}", sender_id), color)
    }

    /// Applies an authoritative `UpdateEvent` from WebSocket push or sync-diff to stage, sidebar, and calls.
    pub fn apply_update_event(
        &mut self,
        chat_id: ChatId,
        event: UpdateEvent,
        cx: &mut Context<Self>,
    ) {
        self.apply_update_event_with_pts(chat_id, None, event, cx);
    }

    /// Applies an authoritative `UpdateEvent` with an optional `pts` watermark so out-of-order
    /// WebSocket reaction frames never overwrite newer HTTP/WS reaction mutations.
    pub fn apply_update_event_with_pts(
        &mut self,
        chat_id: ChatId,
        pts: Option<u64>,
        event: UpdateEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            UpdateEvent::NewMessage(msg) => {
                self.apply_new_message_event(chat_id, msg, cx);
            }
            UpdateEvent::EditMessage(msg) => {
                let chat_u64 = if msg.chat_id.0 > 0 {
                    msg.chat_id.0 as u64
                } else if chat_id.0 > 0 {
                    chat_id.0 as u64
                } else {
                    0
                };
                let target_id = u64::try_from(msg.id.0).unwrap_or(0);
                let updated_kb = msg
                    .inline_keyboard
                    .as_deref()
                    .map(proto_keyboard_to_stage);
                if let gpui_telegram_proto::message::MessageContent::Text { text, .. } = msg.content
                {
                    let mut is_latest_message = false;
                    let mut has_history_for_chat = false;
                    self.stage.update(cx, |st, st_cx| {
                        has_history_for_chat = st.active_chat_id == Some(chat_u64)
                            || st.chat_histories.contains_key(&chat_u64);
                        is_latest_message =
                            apply_text_edit_to_stage(st, chat_u64, target_id, &text, updated_kb);
                        st_cx.notify();
                    });
                    if chat_u64 > 0 && target_id > 0 {
                        let preview_snippet = format_sidebar_preview(&text);
                        self.sidebar.update(cx, |sb, sb_cx| {
                            if let Some(dialog) = sb.dialogs.iter_mut().find(|d| d.id == chat_u64) {
                                let is_streaming_placeholder = dialog.last_message.is_empty()
                                    || dialog.last_message.trim_start().starts_with("💭")
                                    || dialog.last_message.trim_start().starts_with("🔧");
                                if is_latest_message
                                    || (!has_history_for_chat && is_streaming_placeholder)
                                {
                                    dialog.last_message = preview_snippet;
                                    sb_cx.notify();
                                }
                            }
                        });
                    }
                }
                cx.notify();
            }
            UpdateEvent::DeleteMessages(tombstone) => {
                let tomb_chat_i64 = match tombstone.chat_id {
                    gpui_telegram_core::types::PeerId::Chat(c) => c.0,
                    gpui_telegram_core::types::PeerId::User(u) => u.0 as i64,
                    gpui_telegram_core::types::PeerId::Channel(ch) => ch.0 as i64,
                    gpui_telegram_core::types::PeerId::Fake(f) => f as i64,
                };
                let mut chat_u64 = if tomb_chat_i64 > 0 {
                    tomb_chat_i64 as u64
                } else if chat_id.0 > 0 {
                    chat_id.0 as u64
                } else {
                    0
                };
                if chat_u64 == 0 {
                    if let Some(first_id) = tombstone.message_ids.first() {
                        if let Some(found) = self.client.local_store.find_chat_id_for_message(*first_id) {
                            chat_u64 = u64::try_from(found.0).unwrap_or(0);
                        }
                    }
                }
                let ids_to_delete: HashSet<u64> =
                    tombstone.message_ids.iter().filter_map(|id| u64::try_from(id.0).ok()).collect();
                if chat_u64 > 0 {
                    self.client
                        .local_store
                        .delete_messages(ChatId(chat_u64 as i64), &tombstone.message_ids);
                }

                self.stage.update(cx, |st, st_cx| {
                    if chat_u64 == 0 || st.active_chat_id == Some(chat_u64) {
                        st.messages.retain(|m| !ids_to_delete.contains(&m.id));
                    }
                    if chat_u64 > 0 {
                        if let Some(history) = st.chat_histories.get_mut(&chat_u64) {
                            history.retain(|m| !ids_to_delete.contains(&m.id));
                        }
                        if let Some(entries) = st.chat_pinned_entries.get_mut(&chat_u64) {
                            entries.retain(|e| !ids_to_delete.contains(&e.msg_id));
                        }
                    } else {
                        for history in st.chat_histories.values_mut() {
                            history.retain(|m| !ids_to_delete.contains(&m.id));
                        }
                        for entries in st.chat_pinned_entries.values_mut() {
                            entries.retain(|e| !ids_to_delete.contains(&e.msg_id));
                        }
                    }
                    for id in &ids_to_delete {
                        st.message_expirations.remove(id);
                    }
                    st.sync_pinned_tracker();
                    st.row_height_cache.mark_dirty();
                    st_cx.notify();
                });
                cx.notify();
            }
            UpdateEvent::UserTyping(typing_event) => {
                let target_chat_u64 = if typing_event.chat_id.0 > 0 {
                    typing_event.chat_id.0 as u64
                } else if chat_id.0 > 0 {
                    chat_id.0 as u64
                } else {
                    0
                };
                if target_chat_u64 > 0 {
                    let sender_u64 = typing_event.user_id.0;
                    match typing_event.action {
                        TypingAction::Cancel => {
                            self.sidebar.update(cx, |sb, sb_cx| {
                                sb.set_typing(target_chat_u64, None);
                                sb_cx.notify();
                            });
                        }
                        action => {
                            let is_self_echo =
                                sender_u64 > 0 && self.session_sync.user_id == Some(sender_u64);
                            if !is_self_echo {
                                let (sender_name, _, _) = self.resolve_incoming_sender_identity(
                                    target_chat_u64,
                                    sender_u64,
                                    cx,
                                );
                                let indicator = format_typing_indicator(
                                    self.active_lang_id(),
                                    &sender_name,
                                    action,
                                );
                                self.sidebar.update(cx, |sb, sb_cx| {
                                    sb.set_typing(target_chat_u64, indicator);
                                    sb_cx.notify();
                                });
                            }
                        }
                    }
                }
                cx.notify();
            }
            UpdateEvent::MessageReactions(ev) | UpdateEvent::ReactionUpdated(ev) => {
                let msg_u64 = ev.message_id.0 as u64;
                let chat_u64 =
                    self.resolve_reaction_target_chat_id(chat_id, ev.chat_id, msg_u64, cx);
                let watermark = self
                    .session_sync
                    .reaction_pts_by_msg
                    .get(&(chat_u64, msg_u64))
                    .copied()
                    .unwrap_or(0);
                if let Some(p) = pts.filter(|&p| p > 0) {
                    if watermark > 0 && p < watermark {
                        return;
                    }
                    if p > watermark {
                        self.session_sync
                            .reaction_pts_by_msg
                            .insert((chat_u64, msg_u64), p);
                    }
                }

                let is_self_actor = ev.user_id.0 > 0
                    && self.session_sync.user_id.is_some_and(|uid| uid == ev.user_id.0);
                let in_flight = self
                    .session_sync
                    .in_flight_reactions
                    .get(&(chat_u64, msg_u64))
                    .copied()
                    .unwrap_or(0);
                if is_self_actor && in_flight > 0 {
                    return;
                }

                let is_my_event = is_self_actor || ev.user_id.0 == 0;
                let raw_reactions = ev.reactions;
                let mut persisted_counts: Option<Vec<ReactionCount>> = None;
                self.stage.update(cx, |st, st_cx| {
                    let my_existing_chosen: HashSet<String> = if is_my_event {
                        HashSet::new()
                    } else {
                        st.messages
                            .iter()
                            .find(|m| m.id == msg_u64)
                            .or_else(|| {
                                st.chat_histories
                                    .get(&chat_u64)
                                    .and_then(|h| h.iter().find(|m| m.id == msg_u64))
                            })
                            .map(|m| {
                                m.reactions
                                    .iter()
                                    .filter(|r| r.user_reacted)
                                    .map(|r| r.emoji.clone())
                                    .collect()
                            })
                            .unwrap_or_default()
                    };
                    let mapped: Vec<ReactionItem> = raw_reactions
                        .into_iter()
                        .map(|r| {
                            let chosen = if is_my_event {
                                r.is_chosen_by_me
                            } else {
                                my_existing_chosen.contains(&r.emoji)
                            };
                            ReactionItem {
                                emoji: r.emoji,
                                count: r.count,
                                user_reacted: chosen,
                            }
                        })
                        .collect();
                    if chat_u64 > 0 && msg_u64 > 0 {
                        persisted_counts = Some(
                            mapped
                                .iter()
                                .map(|r| ReactionCount::new(&r.emoji, r.count, r.user_reacted))
                                .collect(),
                        );
                    }
                    st.set_message_reactions(chat_u64, msg_u64, mapped);
                    st_cx.notify();
                });
                if let Some(proto_counts) = persisted_counts {
                    self.client.local_store.update_reactions(
                        ChatId(chat_u64 as i64),
                        MessageId(msg_u64 as i64),
                        proto_counts,
                    );
                }
                cx.notify();
            }
            UpdateEvent::PinnedMessagesChanged {
                chat_id: ev_chat,
                pinned_ids,
            } => {
                let chat_u64 = if ev_chat.0 > 0 {
                    ev_chat.0 as u64
                } else {
                    u64::try_from(chat_id.0).unwrap_or(0)
                };
                let ids: Vec<i64> = pinned_ids.iter().map(|id| id.0).collect();
                self.apply_pinned_ids_update(chat_u64, ids, cx);
                cx.notify();
            }
            UpdateEvent::PinnedMessageChanged { message_id } => {
                let chat_u64 = self
                    .stage
                    .read(cx)
                    .active_chat_id
                    .unwrap_or_else(|| u64::try_from(chat_id.0).unwrap_or(0));
                let ids = message_id.map(|id| vec![id.0]).unwrap_or_default();
                self.apply_pinned_ids_update(chat_u64, ids, cx);
                cx.notify();
            }
            UpdateEvent::DraftUpdated {
                chat_id: ev_chat,
                draft,
                ..
            } => {
                let text = draft.map(|d| d.text).unwrap_or_default();
                self.sidebar.update(cx, |sb, sb_cx| {
                    sb.set_draft(ev_chat.0 as u64, text);
                    sb_cx.notify();
                });
                cx.notify();
            }
            UpdateEvent::ReadHistory(ev) => {
                let target_chat = if ev.chat_id.0 > 0 { ev.chat_id } else { chat_id };
                let target_u64 = u64::try_from(target_chat.0).unwrap_or(0);
                if target_u64 > 0 {
                    if ev.is_outbox {
                        self.client.local_store.mark_read_outbox(target_chat, ev.max_id);
                    } else {
                        self.client
                            .local_store
                            .mark_read_inbox(target_chat, ev.max_id, Some(ev.unread_count));
                        self.sidebar.update(cx, |sb, sb_cx| {
                            if let Some(d) = sb.dialogs.iter_mut().find(|d| d.id == target_u64) {
                                d.unread_count = ev.unread_count;
                            }
                            sb.recompute_folder_badges();
                            sb_cx.notify();
                        });
                    }
                }
                cx.notify();
            }
            UpdateEvent::FolderChanged { folder } => {
                self.client.local_store.upsert_folder(folder);
                let cached = self.client.local_store.get_folders();
                if !cached.is_empty() {
                    apply_folders_to_shell(self, &cached, cx);
                }
                self.refresh_folders(cx);
            }
            UpdateEvent::PollUpdated {
                chat_id: ev_chat, ..
            } => {
                if self.stage.read(cx).active_chat_id == Some(ev_chat.0 as u64) {
                    self.load_historical_messages(ev_chat.0 as u64, cx);
                }
            }
            UpdateEvent::CallUpdated {
                call_id,
                state,
                signal,
            } => {
                self.apply_call_updated_event(call_id, state, signal, cx);
            }
            UpdateEvent::StoryUpdated { .. } | UpdateEvent::ForumTopicChanged { .. } => {
                cx.notify();
            }
            _ => {}
        }
    }

    fn apply_new_message_event(
        &mut self,
        chat_id: ChatId,
        msg: Message,
        cx: &mut Context<Self>,
    ) {
        let is_outgoing = self
            .session_sync
            .user_id
            .is_some_and(|uid| uid == msg.sender_id.0);
        let current_user_id = self.session_sync.user_id.unwrap_or(0);
        let chat_u64 = if msg.chat_id.0 > 0 {
            msg.chat_id.0 as u64
        } else if chat_id.0 > 0 {
            chat_id.0 as u64
        } else {
            0
        };
        let server_id = u64::try_from(msg.id.0).unwrap_or(0);

        if is_outgoing {
            if let Some(nonce) = msg.client_nonce {
                let was_tracked = self
                    .session_sync
                    .pending_outgoing_nonces
                    .remove(&nonce)
                    .is_some()
                    || self.session_sync.resolved_outgoing_nonces.contains_key(&nonce)
                    || self.has_stage_or_history_message(chat_u64, nonce, cx)
                    || (server_id > 0
                        && self.has_stage_or_history_message(chat_u64, server_id, cx));
                if was_tracked {
                    if server_id > 0 {
                        self.session_sync
                            .resolved_outgoing_nonces
                            .insert(nonce, server_id);
                    }
                    self.reconcile_outgoing_message(
                        chat_u64,
                        nonce,
                        server_id,
                        DeliveryStatus::Read,
                        0,
                        cx,
                    );
                    return;
                }
            }
            if server_id > 0 && self.has_stage_or_history_message(chat_u64, server_id, cx) {
                self.reconcile_outgoing_message(
                    chat_u64,
                    server_id,
                    server_id,
                    DeliveryStatus::Read,
                    0,
                    cx,
                );
                return;
            }
        }

        let raw_text = match &msg.content {
            gpui_telegram_proto::message::MessageContent::Text { text, .. } => text.clone(),
            gpui_telegram_proto::message::MessageContent::Encrypted(_) => {
                "🔒 Encrypted message".to_string()
            }
            _ => "[Media attachment]".to_string(),
        };
        let sidebar_preview = match &msg.content {
            gpui_telegram_proto::message::MessageContent::Text { text, .. } => {
                format_sidebar_preview(text)
            }
            gpui_telegram_proto::message::MessageContent::Encrypted(_) => {
                "🔒 Encrypted message".to_string()
            }
            _ => "[Media attachment]".to_string(),
        };
        let inline_kb = msg
            .inline_keyboard
            .as_deref()
            .and_then(proto_keyboard_to_stage)
            .or_else(|| {
                serde_json::to_value(&msg.content)
                    .ok()
                    .and_then(|val| decode_inline_keyboard_json(&val))
            });

        let (sender_name, avatar_initials, avatar_color) = if is_outgoing {
            let my_name = self
                .session_sync
                .username
                .clone()
                .unwrap_or_else(|| "You".to_string());
            let init = derive_initials(&my_name);
            (
                my_name,
                if init.is_empty() { "ME".to_string() } else { init },
                peer_avatar_color(current_user_id),
            )
        } else {
            self.resolve_incoming_sender_identity(chat_u64, msg.sender_id.0, cx)
        };

        let model = MessageItemModel {
            id: server_id,
            sender_id: msg.sender_id.0,
            sender_name: sender_name.clone(),
            avatar_initials,
            avatar_color,
            direction: if is_outgoing {
                MessageDirection::Outgoing
            } else {
                MessageDirection::Incoming
            },
            timestamp: msg.date.format("%H:%M").to_string(),
            delivery_status: DeliveryStatus::Read,
            reply_to: None,
            reactions: vec![],
            content: MessageContent::Text {
                text: raw_text.clone(),
                is_edited: false,
            },
            bot_keyboard: inline_kb.clone(),
            is_pinned: false,
        };

        let expires_at_opt = msg.expires_at_ms.or_else(|| {
            msg.ttl_seconds
                .filter(|&ttl| ttl > 0)
                .map(|ttl| msg.date.timestamp_millis().saturating_add(i64::from(ttl) * 1000))
        });
        let is_current_chat = self.stage.read(cx).active_chat_id == Some(chat_u64);

        if is_current_chat {
            self.stage.update(cx, |st, st_cx| {
                if let Some(exp) = expires_at_opt {
                    if server_id > 0 {
                        st.message_expirations.insert(server_id, exp);
                    }
                }
                st.add_message(model.clone());
                st_cx.notify();
            });
        } else {
            self.stage.update(cx, |st, _| {
                if let Some(exp) = expires_at_opt {
                    if server_id > 0 {
                        st.message_expirations.insert(server_id, exp);
                    }
                }
                let hist = st.chat_histories.entry(chat_u64).or_default();
                if !reconcile_stage_message_list(hist, model.id, model.id, DeliveryStatus::Read) {
                    hist.push(model);
                }
            });
        }

        let is_bot_chat = self.is_chat_bot(chat_u64, cx)
            || self.sidebar.read(cx).bot_ids.contains(&msg.sender_id.0);

        if !is_outgoing
            && self.rtc_talk.talk_mode_overlay.is_some()
            && self.rtc_talk.active_talk_session_is_ai
            && self.rtc_talk.active_talk_bot_chat_id == Some(chat_u64)
            && !raw_text.trim().is_empty()
        {
            if let Some(ov) = self.rtc_talk.talk_mode_overlay.clone() {
                ov.update(cx, |o, _| {
                    o.add_transcript(&sender_name, &raw_text, false);
                });
            }
        }

        let sb_preview_for_dialog = sidebar_preview.clone();
        self.sidebar.update(cx, |sb, sb_cx| {
            if !is_outgoing {
                sb.set_typing(chat_u64, None);
            }
            if let Some(dialog) = sb.dialogs.iter_mut().find(|d| d.id == chat_u64) {
                dialog.last_message = sb_preview_for_dialog;
                dialog.timestamp = msg.date.format("%H:%M").to_string();
                if !is_current_chat && !is_outgoing {
                    dialog.unread_count += 1;
                }
            }
            sb.recompute_folder_badges();
            sb_cx.notify();
        });

        let dialog_info = self
            .sidebar
            .read(cx)
            .dialogs
            .iter()
            .find(|d| d.id == chat_u64)
            .map(|d| (d.is_muted, d.folder.clone()));
        let is_muted = dialog_info.as_ref().map(|(m, _)| *m).unwrap_or(false);
        let folder_name = dialog_info
            .as_ref()
            .map(|(_, f)| f.as_str())
            .unwrap_or("");
        let rich_chat = self.session_sync.rich_chats_by_id.get(&chat_u64);
        let is_channel = rich_chat.map(|rc| rc.is_channel).unwrap_or(false)
            || folder_name.eq_ignore_ascii_case("Channels");
        let is_group = rich_chat.map(|rc| rc.is_group).unwrap_or(false)
            || folder_name.eq_ignore_ascii_case("Groups");
        let window_is_focused = self.window_runtime.main_window.is_active()
            && !self.window_runtime.main_window.is_minimized;

        if should_dispatch_message_notification(
            is_outgoing,
            is_current_chat,
            window_is_focused,
            is_muted,
            is_channel,
            is_group,
            &self.preferences.notifications,
        ) {
            self.window_runtime.notifications.schedule_notification(
                chat_u64,
                &sender_name,
                &sidebar_preview,
                self.preferences.notifications.sound_enabled,
            );
            let parsed = crate::platform::mac::UNNotificationParsed::new(1, chat_u64, server_id as i64);
            let _ = self.window_runtime.mac_notifications.show_system_notification(
                &sender_name,
                "",
                &sidebar_preview,
                self.preferences.notifications.sound_enabled,
                parsed,
            );
        }

        if !is_outgoing && is_bot_chat && inline_kb.is_none() && server_id > 0 {
            if let (Some(token), Some(uid)) = (
                self.session_sync.session_token.clone(),
                self.session_sync.user_id,
            ) {
                let fence = SessionRequestFence {
                    auth_generation: self.session_sync.auth_generation,
                    request_generation: 0,
                    token,
                    user_id: uid,
                };
                let client = self.client.clone();
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    if let Ok(rich) = client
                        .hermes_messages_history_rich(chat_u64 as i64, Some(10))
                        .await
                    {
                        let _ = weak.update(app, |shell, cx| {
                            if !fence.matches(
                                shell.is_authenticated,
                                shell.session_sync.auth_generation,
                                0,
                                shell.session_sync.session_token.as_deref(),
                                shell.session_sync.user_id,
                            ) {
                                return;
                            }
                            shell.stage.update(cx, |st, st_cx| {
                                overlay_rich_message_metadata(&mut st.messages, &rich);
                                if let Some(hist) = st.chat_histories.get_mut(&chat_u64) {
                                    overlay_rich_message_metadata(hist, &rich);
                                }
                                st_cx.notify();
                            });
                            cx.notify();
                        });
                    }
                })
                .detach();
            }
        }
        cx.notify();
    }

    fn apply_call_updated_event(
        &mut self,
        call_id: String,
        state: String,
        signal: Option<CallSignalPayload>,
        cx: &mut Context<Self>,
    ) {
        if let Some(sig) = signal {
            let cid = call_id.clone();
            let frame_opt = match sig.signal_type.as_str() {
                "offer" => sig.sdp.map(|sdp| SignalingFrame::Offer { call_id: cid, sdp }),
                "answer" => sig.sdp.map(|sdp| SignalingFrame::Answer { call_id: cid, sdp }),
                "ice_candidate" => sig.candidate.map(|candidate| SignalingFrame::IceCandidate {
                    call_id: cid,
                    candidate,
                    sdp_mid: sig.sdp_mid,
                    sdp_mline_index: sig.sdp_mline_index.map(|v| v as u16),
                }),
                "ringing" => Some(SignalingFrame::Ringing { call_id: cid }),
                "accept" => Some(SignalingFrame::Accept { call_id: cid }),
                "hangup" => Some(SignalingFrame::Hangup {
                    call_id: cid,
                    reason: sig.reason.unwrap_or_else(|| "remote_hangup".to_string()),
                }),
                _ => None,
            };
            if let Some(frame) = frame_opt {
                self.rtc_talk.webrtc_engine.apply_signaling_frame(frame);
            }
        }
        let emojis = self
            .rtc_talk
            .webrtc_engine
            .emoji_fingerprint()
            .map(|f| f.emoji_chars.join(" "));
        match state.as_str() {
            "ended" | "declined" | "missed" | "cancelled" => {
                if self.rtc_talk.active_call_id.as_deref() == Some(call_id.as_str()) {
                    self.rtc_talk.active_call_id = None;
                    self.rtc_talk.talk_mode_overlay = None;
                }
                self.sync_calls_history(cx);
            }
            "accepted" | "active" | "connected" => {
                if let Some(ov) = self.rtc_talk.talk_mode_overlay.clone() {
                    ov.update(cx, |o, _| {
                        if let Some(ref e) = emojis {
                            o.e2ee_emojis = e.clone();
                        }
                        o.is_connected = true;
                        o.set_state(TalkModeState::Listening);
                    });
                }
            }
            "ringing" | "initiated" => {
                if self.rtc_talk.talk_mode_overlay.is_none() && !call_id.is_empty() {
                    self.rtc_talk.active_call_id = Some(call_id);
                    let pal = self.theme_state.palette;
                    let lang = self.active_lang_id();
                    let ov = cx.new(|_| {
                        let mut o = CallOverlay::new_live(
                            tr_str(lang, "来电通话", "Incoming Call").to_string(),
                            None,
                            None,
                            pal,
                            lang,
                        );
                        o.is_connected = false;
                        o.set_state(TalkModeState::Connecting);
                        if let Some(ref e) = emojis {
                            o.e2ee_emojis = e.clone();
                        }
                        o
                    });
                    self.rtc_talk.talk_mode_overlay = Some(ov);
                }
                self.sync_calls_history(cx);
            }
            _ => {}
        }
        cx.notify();
    }

    /// Resolves deep link URLs (such as `tg://chat?id={id}`) and switches to the destination chat.
    pub fn handle_deep_link(&mut self, url: &str, cx: &mut Context<Self>) -> bool {
        let link_info = crate::views::app_shell::window_session_controller_link_info::WindowSessionControllerLinkInfo::parse_deep_link(url);
        match link_info {
            crate::views::app_shell::window_session_controller_link_info::LinkInfoType::Peer(peer) => {
                if let Some(cid) = peer.channel_id {
                    let title_hint = peer
                        .username_or_id
                        .filter(|s| s.parse::<u64>().is_err());
                    self.open_or_select_chat(cid, title_hint, false, cx);
                    true
                } else if let Some(ref uid_str) = peer.username_or_id {
                    if let Ok(cid) = uid_str.parse::<u64>() {
                        self.open_or_select_chat(cid, None, false, cx);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_msg(id: u64, text: &str) -> MessageItemModel {
        MessageItemModel {
            id,
            sender_id: 600_001,
            sender_name: "Cue Cloud Agent".to_string(),
            avatar_initials: "CC".to_string(),
            avatar_color: 0x2AABEE,
            direction: MessageDirection::Incoming,
            timestamp: "12:00".to_string(),
            delivery_status: DeliveryStatus::Read,
            reply_to: None,
            reactions: vec![],
            content: MessageContent::Text {
                text: text.to_string(),
                is_edited: false,
            },
            bot_keyboard: None,
            is_pinned: false,
        }
    }

    #[test]
    fn test_format_typing_indicator_respects_cancel_and_resolved_name() {
        assert_eq!(
            format_typing_indicator("zh-CN", "Cue Cloud Agent", TypingAction::Cancel),
            None
        );
        assert_eq!(
            format_typing_indicator("en", "Cue Cloud Agent", TypingAction::Cancel),
            None
        );
        assert_eq!(
            format_typing_indicator("zh-CN", "Cue Cloud Agent", TypingAction::Typing),
            Some("Cue Cloud Agent 正在思考与输入...".to_string())
        );
        assert_eq!(
            format_typing_indicator("en", "Cue Cloud Agent", TypingAction::Typing),
            Some("Cue Cloud Agent is typing...".to_string())
        );
        assert_eq!(
            format_typing_indicator("zh-CN", "Alice", TypingAction::RecordAudio),
            Some("Alice 正在录制语音...".to_string())
        );
    }

    #[test]
    fn test_format_sidebar_preview_strips_streaming_telemetry_and_markdown() {
        assert_eq!(
            format_sidebar_preview("💭 正在思考..."),
            "💭 正在思考..."
        );
        assert_eq!(
            format_sidebar_preview(
                "💭 **思考过程**\n> 正在检查节点\n\n🔧 **执行步骤 (1/2)**\n- ⏳ `computer_exec` `uptime`"
            ),
            "🔧 执行步骤 (1/2) ⏳ computer_exec uptime"
        );
        let final_reply = concat!(
            "💭 **思考过程**\n> 用户需要查看负载。\n\n",
            "🔧 **执行步骤 (2/2)**\n- ✅ `computer_status` (12ms)\n- ✅ `computer_exec` (40ms)\n\n",
            "---\n\n",
            "### 服务器状态\n",
            "- 负载: **0.15**，请查看 [监控面板](https://status.example.com) 或 `shell_pts_updates.rs`。\n",
            "[STATE_PATCH]{\"ok\":true}[/STATE_PATCH]"
        );
        assert_eq!(
            format_sidebar_preview(final_reply),
            "服务器状态 负载: 0.15，请查看 监控面板 或 shell_pts_updates.rs。"
        );
        assert_eq!(format_sidebar_preview("[Media attachment]"), "[Media attachment]");
        assert_eq!(
            format_sidebar_preview("<think>hidden</think>最终结论：**通过**"),
            "最终结论：通过"
        );
    }

    #[test]
    fn test_apply_text_edit_to_stage_updates_active_and_background_and_checks_latest() {
        let mut stage = ConversationStage::new();
        stage.set_chat(23, "Cue Cloud Agent", "", "CC", 0x2AABEE);
        stage.add_message(sample_msg(1, "First message"));
        stage.add_message(sample_msg(2, "💭 正在思考..."));
        stage.row_height_cache.measured_heights.insert(2, 44.0);

        // Editing non-latest message updates text without claiming latest
        assert!(!apply_text_edit_to_stage(&mut stage, 23, 1, "First edited", None));
        assert!(matches!(
            stage.messages[0].content,
            MessageContent::Text { ref text, is_edited: true } if text == "First edited"
        ));

        // Editing latest message in active chat invalidates measured height & signals latest
        assert!(apply_text_edit_to_stage(&mut stage, 23, 2, "Final AI answer", None));
        assert!(!stage.row_height_cache.measured_heights.contains_key(&2));
        assert!(stage.row_height_cache.is_dirty());
        assert!(matches!(
            stage.messages[1].content,
            MessageContent::Text { ref text, is_edited: true } if text == "Final AI answer"
        ));
        assert!(matches!(
            stage.chat_histories.get(&23).and_then(|h| h.get(1)).map(|m| &m.content),
            Some(MessageContent::Text { text, is_edited: true }) if text == "Final AI answer"
        ));

        // Editing background chat with colliding per-chat message_id=2 must not touch active chat #99
        stage.set_chat(99, "Other Chat", "", "OC", 0x2AABEE);
        stage.add_message(sample_msg(2, "Chat 99 msg #2"));
        assert!(apply_text_edit_to_stage(&mut stage, 23, 2, "Updated chat 23", None));
        assert!(matches!(
            stage.messages[0].content,
            MessageContent::Text { ref text, is_edited: false } if text == "Chat 99 msg #2"
        ));
        assert!(matches!(
            stage.chat_histories.get(&23).and_then(|h| h.get(1)).map(|m| &m.content),
            Some(MessageContent::Text { text, is_edited: true }) if text == "Updated chat 23"
        ));
    }
}
