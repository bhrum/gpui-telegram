//! Truthful initial state, authoritative sync, and session/generation fencing (`FUS-D004`).

use std::collections::{HashMap, HashSet};

use crate::views::chat_sidebar::{DialogDeliveryStatus, DialogItemModel};
use crate::views::stage::cards::{
    DeliveryStatus, InlineButtonType, InlineKeyboardButton, MediaType, MessageContent,
    MessageDirection, MessageItemModel, PollOption, PollType, ReactionItem, ReplyReference,
};
use crate::views::{CanonicalComposer, ChatSidebar, ContextInspector, ConversationStage};
use gpui_telegram_client::net::hermes_client::{HermesInlineButtonDto, HermesRichMessageDto};
use gpui_telegram_client::net::http_client::HistoricalMessage;
use gpui_telegram_client::ChatSummary;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionRequestFence {
    pub auth_generation: u64,
    pub request_generation: u64,
    pub token: String,
    pub user_id: u64,
}

impl SessionRequestFence {
    pub fn matches(
        &self,
        is_authenticated: bool,
        current_auth_gen: u64,
        current_req_gen: u64,
        current_token: Option<&str>,
        current_user_id: Option<u64>,
    ) -> bool {
        is_authenticated
            && !self.token.is_empty()
            && self.user_id > 0
            && self.auth_generation == current_auth_gen
            && self.request_generation == current_req_gen
            && current_token == Some(self.token.as_str())
            && current_user_id == Some(self.user_id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedSendTarget {
    pub chat_id: u64,
    pub user_id: u64,
    pub peer_name: String,
}

pub fn peer_subtitle_for_dialog(dialog: &DialogItemModel) -> String {
    if dialog.is_online {
        "online".to_string()
    } else {
        String::new()
    }
}

pub fn format_authoritative_timestamp(raw_ts: Option<i64>) -> String {
    let Some(ts) = raw_ts.filter(|&v| v > 0) else {
        return String::new();
    };
    let dt = if ts >= 10_000_000_000 {
        chrono::DateTime::from_timestamp_millis(ts)
    } else {
        chrono::DateTime::from_timestamp(ts, 0)
    };
    dt.map(|d| d.format("%H:%M").to_string()).unwrap_or_default()
}

pub fn derive_initials(title: &str) -> String {
    let trimmed = title.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    let words: Vec<&str> = trimmed.split_whitespace().collect();
    if words.len() >= 2 {
        let two: String = words
            .iter()
            .filter_map(|w| {
                w.chars()
                    .find(|c| c.is_alphanumeric())
                    .or_else(|| w.chars().next())
            })
            .take(2)
            .collect();
        if !two.is_empty() {
            return two.to_uppercase();
        }
    }
    let single = words.first().copied().unwrap_or(trimmed);
    let alnum: Vec<char> = single.chars().filter(|c| c.is_alphanumeric()).collect();
    if !alnum.is_empty() {
        if alnum.iter().all(|c| c.is_ascii_digit()) {
            return alnum.into_iter().take(2).collect();
        }
        return alnum[0].to_uppercase().collect();
    }
    single
        .chars()
        .next()
        .map(|c| c.to_uppercase().collect())
        .unwrap_or_default()
}

pub fn proto_keyboard_to_stage(
    rows: &[Vec<gpui_telegram_proto::message::InlineKeyboardButton>],
) -> Option<Vec<Vec<InlineKeyboardButton>>> {
    let converted: Vec<Vec<InlineKeyboardButton>> = rows
        .iter()
        .filter_map(|row| {
            let btns: Vec<InlineKeyboardButton> = row
                .iter()
                .filter(|b| !b.text.trim().is_empty())
                .map(|b| {
                    let button_type = if let Some(ref url) = b.url {
                        InlineButtonType::Url(url.clone())
                    } else if let Some(ref web) = b.web_app_url {
                        InlineButtonType::WebApp(web.clone())
                    } else if let Some(ref cb) = b.callback_data {
                        InlineButtonType::Callback(cb.clone())
                    } else {
                        InlineButtonType::Callback(b.text.clone())
                    };
                    InlineKeyboardButton {
                        text: b.text.clone(),
                        button_type,
                    }
                })
                .collect();
            if btns.is_empty() { None } else { Some(btns) }
        })
        .collect();
    if converted.is_empty() { None } else { Some(converted) }
}

pub fn decode_inline_keyboard_json(
    val: &serde_json::Value,
) -> Option<Vec<Vec<InlineKeyboardButton>>> {
    let kb_val = val
        .get("inline_keyboard")
        .or_else(|| val.get("bot_keyboard"))
        .unwrap_or(val);
    let rows = serde_json::from_value::<Vec<Vec<gpui_telegram_proto::message::InlineKeyboardButton>>>(
        kb_val.clone(),
    )
    .ok()?;
    proto_keyboard_to_stage(&rows)
}

pub fn hermes_keyboard_to_stage(
    rows: &[Vec<HermesInlineButtonDto>],
) -> Option<Vec<Vec<InlineKeyboardButton>>> {
    let converted: Vec<Vec<InlineKeyboardButton>> = rows
        .iter()
        .filter_map(|row| {
            let btns: Vec<InlineKeyboardButton> = row
                .iter()
                .filter(|b| !b.text.trim().is_empty())
                .map(|b| {
                    let button_type = if let Some(ref url) = b.url {
                        InlineButtonType::Url(url.clone())
                    } else if let Some(ref web) = b.web_app_url {
                        InlineButtonType::WebApp(web.clone())
                    } else if let Some(ref cb) = b.callback_data {
                        InlineButtonType::Callback(cb.clone())
                    } else {
                        InlineButtonType::Callback(b.text.clone())
                    };
                    InlineKeyboardButton {
                        text: b.text.clone(),
                        button_type,
                    }
                })
                .collect();
            if btns.is_empty() { None } else { Some(btns) }
        })
        .collect();
    if converted.is_empty() { None } else { Some(converted) }
}

pub fn overlay_rich_message_metadata(
    models: &mut [MessageItemModel],
    rich: &[HermesRichMessageDto],
) {
    let by_id: HashMap<u64, &HermesRichMessageDto> = rich
        .iter()
        .filter_map(|r| u64::try_from(r.message_id).ok().map(|id| (id, r)))
        .collect();
    for model in models.iter_mut() {
        let Some(r) = by_id.get(&model.id) else {
            continue;
        };
        if model.direction != MessageDirection::Outgoing {
            if let Some(ref name) = r.sender_name {
                let trimmed = name.trim();
                if !trimmed.is_empty() {
                    model.sender_name = trimmed.to_string();
                }
            }
            let derived = r
                .sender_initials
                .as_deref()
                .map(str::trim)
                .filter(|s| !s.is_empty() && *s != "?")
                .map(ToString::to_string)
                .unwrap_or_else(|| derive_initials(&model.sender_name));
            if !derived.is_empty() {
                model.avatar_initials = derived;
            }
        }
        let kb = r
            .inline_keyboard
            .as_deref()
            .or(r.bot_keyboard.as_deref())
            .and_then(hermes_keyboard_to_stage);
        if kb.is_some() {
            model.bot_keyboard = kb;
        }
    }
}

pub fn reset_account_conversation_views(
    sidebar: &mut ChatSidebar,
    stage: &mut ConversationStage,
    inspector: &mut ContextInspector,
    server_url: &str,
    username: Option<&str>,
) {
    sidebar.configure_for_cloud_workspace(server_url);
    sidebar.dialogs.clear();
    sidebar.custom_folders.clear();
    sidebar.bot_ids.clear();
    sidebar.contact_chat_ids.clear();
    sidebar.group_chat_ids.clear();
    sidebar.channel_chat_ids.clear();
    sidebar.clear_selection();
    sidebar.inner_widget.total_rows = 0;
    sidebar.folders = vec![("All".to_string(), 0)];

    stage.configure_for_cloud_workspace(server_url, username);

    inspector.configure_for_cloud_workspace(server_url);
    clear_inspector_peer(inspector);
    inspector.is_open = false;
}

fn clear_inspector_peer(inspector: &mut ContextInspector) {
    inspector.profile.name.clear();
    inspector.profile.status.clear();
    inspector.profile.avatar_initials.clear();
    inspector.profile.avatar_color = 0x2AABEE;
    inspector.profile.facts.clear();
    inspector.media_items.clear();
}

pub fn select_authoritative_dialog(
    sidebar: &mut ChatSidebar,
    stage: &mut ConversationStage,
    inspector: &mut ContextInspector,
    chat_id: u64,
) -> bool {
    let Some((idx, dialog)) = sidebar
        .dialogs
        .iter()
        .enumerate()
        .find(|(_, d)| d.id == chat_id)
        .map(|(i, d)| (i, d.clone()))
    else {
        return false;
    };

    sidebar.select_dialog(chat_id);
    sidebar.inner_widget.selected_index = Some(idx);

    let subtitle = peer_subtitle_for_dialog(&dialog);
    stage.set_chat(
        dialog.id,
        &dialog.peer_name,
        &subtitle,
        &dialog.avatar_initials,
        dialog.avatar_color,
    );
    stage.is_group_chat = dialog.folder == "Groups" || dialog.folder == "Channels";
    stage.is_saved_messages = dialog.peer_name == "Saved Messages" || dialog.peer_name == "收藏夹";

    let avatar_initials = if dialog.avatar_initials.trim().is_empty() {
        derive_initials(&dialog.peer_name)
    } else {
        dialog.avatar_initials
    };
    inspector.profile.name = dialog.peer_name;
    inspector.profile.status = subtitle;
    inspector.profile.avatar_initials = avatar_initials;
    inspector.profile.avatar_color = dialog.avatar_color;
    inspector.profile.facts.clear();
    inspector.media_items.clear();
    true
}

pub fn apply_authoritative_chat_list(
    sidebar: &mut ChatSidebar,
    stage: &mut ConversationStage,
    inspector: &mut ContextInspector,
    chats: &[ChatSummary],
) -> Option<u64> {
    let previous_selected = sidebar.active_chat_id;
    let prev_flags: std::collections::HashMap<u64, (bool, bool)> = sidebar
        .dialogs
        .iter()
        .map(|d| (d.id, (d.is_muted, d.is_pinned)))
        .collect();
    sidebar.bot_ids.clear();
    sidebar.group_chat_ids.clear();
    sidebar.channel_chat_ids.clear();
    sidebar.dialogs = chats
        .iter()
        .map(|chat| {
            let is_bot = chat.is_bot.unwrap_or(false) || chat.chat_type == 4;
            let is_channel = chat.chat_type == 3;
            let is_group = !is_channel
                && (chat.is_group.unwrap_or(false) || matches!(chat.chat_type, 1 | 2));
            if is_bot {
                sidebar.bot_ids.insert(chat.chat_id);
            }
            if is_channel {
                sidebar.channel_chat_ids.insert(chat.chat_id);
            }
            if is_group {
                sidebar.group_chat_ids.insert(chat.chat_id);
            }
            if chat.chat_type == 5 {
                sidebar.saved_messages_id = Some(chat.chat_id);
            }
            let folder = if is_bot {
                "Bots".to_string()
            } else if is_channel {
                "Channels".to_string()
            } else if is_group {
                "Groups".to_string()
            } else {
                "All".to_string()
            };
            let (was_muted, was_pinned) = prev_flags
                .get(&chat.chat_id)
                .copied()
                .unwrap_or((false, false));
            DialogItemModel {
                id: chat.chat_id,
                peer_name: chat.title.clone(),
                avatar_initials: derive_initials(&chat.title),
                avatar_color: chat.avatar_color.unwrap_or(0x2AABEE),
                last_message: chat
                    .last_message
                    .as_deref()
                    .map(super::shell_pts_updates::format_sidebar_preview)
                    .unwrap_or_default(),
                timestamp: format_authoritative_timestamp(chat.updated_at),
                unread_count: chat.unread_count,
                is_muted: was_muted,
                is_pinned: was_pinned,
                is_online: false,
                typing_indicator: None,
                delivery_status: DialogDeliveryStatus::Read,
                folder,
            }
        })
        .collect();
    sidebar.inner_widget.total_rows = sidebar.dialogs.len();
    sidebar.recompute_folder_badges();

    let valid_ids: HashSet<u64> = sidebar.dialogs.iter().map(|d| d.id).collect();
    stage.chat_histories.retain(|id, _| valid_ids.contains(id));
    stage.chat_pinned_messages.retain(|id, _| valid_ids.contains(id));
    stage.chat_scroll_positions.retain(|id, _| valid_ids.contains(id));

    if let Some(prev_id) = previous_selected {
        if select_authoritative_dialog(sidebar, stage, inspector, prev_id) {
            return Some(prev_id);
        }
    }

    sidebar.clear_selection();
    stage.clear_selection();
    clear_inspector_peer(inspector);
    None
}

pub fn historical_message_to_model(
    hm: &HistoricalMessage,
    self_user_id: u64,
    peer_name: &str,
    peer_initials: &str,
    peer_color: u32,
) -> MessageItemModel {
    let is_outgoing = hm.is_outgoing.unwrap_or(hm.sender_id == self_user_id);
    let sender_name = if is_outgoing {
        "Me".to_string()
    } else if !peer_name.is_empty() {
        peer_name.to_string()
    } else {
        format!("User {}", hm.sender_id)
    };
    let avatar_initials = if is_outgoing {
        "ME".to_string()
    } else if !peer_initials.is_empty() {
        peer_initials.to_string()
    } else {
        derive_initials(&sender_name)
    };
    let avatar_color = if is_outgoing { 0x3498DB } else { peer_color };
    let content = decode_historical_content(hm);

    let reply_to = hm
        .reply_to
        .or_else(|| hm.reply_to_info.as_ref().and_then(|r| r.resolved_message_id()))
        .map(|id| {
            let reply_sender = hm
                .reply_to_info
                .as_ref()
                .and_then(|r| {
                    r.sender_name
                        .as_deref()
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(str::to_string)
                        .or_else(|| {
                            r.sender_id.map(|sid| {
                                if sid == self_user_id {
                                    "Me".to_string()
                                } else if !peer_name.is_empty() {
                                    peer_name.to_string()
                                } else {
                                    format!("User {sid}")
                                }
                            })
                        })
                })
                .unwrap_or_default();
            let reply_snippet = hm
                .reply_to_info
                .as_ref()
                .and_then(|r| r.resolved_snippet())
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .unwrap_or_default();
            ReplyReference {
                message_id: id,
                sender_name: reply_sender,
                snippet: reply_snippet,
            }
        });

    let reactions = hm
        .reactions
        .iter()
        .map(|r| ReactionItem {
            emoji: r.emoji.clone(),
            count: r.count,
            user_reacted: r.chosen,
        })
        .collect();

    MessageItemModel {
        id: hm.message_id,
        sender_id: hm.sender_id,
        sender_name,
        avatar_initials,
        avatar_color,
        direction: if is_outgoing {
            MessageDirection::Outgoing
        } else {
            MessageDirection::Incoming
        },
        timestamp: format_authoritative_timestamp(hm.date),
        delivery_status: DeliveryStatus::Read,
        reply_to,
        reactions,
        content,
        bot_keyboard: hm.content.as_ref().and_then(decode_inline_keyboard_json),
        is_pinned: false,
    }
}

fn decode_poll_content(payload: &serde_json::Value, fallback_text: &str) -> MessageContent {
    let question = payload
        .get("question")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or(fallback_text)
        .to_string();
    let is_quiz = payload
        .get("is_quiz")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let is_anonymous = payload
        .get("is_anonymous")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let poll_type = if is_quiz {
        PollType::Quiz
    } else if is_anonymous {
        PollType::Anonymous
    } else {
        PollType::Regular
    };
    let correct_option_id = payload
        .get("correct_option_id")
        .and_then(|v| v.as_u64())
        .map(|n| n as u32);
    let declared_total = payload
        .get("total_voters")
        .or_else(|| payload.get("total_votes"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;

    let raw_opts = payload
        .get("options")
        .and_then(|v| v.as_array())
        .filter(|arr| !arr.is_empty())
        .or_else(|| payload.get("answers").and_then(|v| v.as_array()));

    let mut options: Vec<PollOption> = raw_opts
        .map(|arr| {
            arr.iter()
                .enumerate()
                .map(|(idx, opt)| {
                    let id = opt
                        .get("option_id")
                        .or_else(|| opt.get("id"))
                        .and_then(|v| v.as_u64())
                        .unwrap_or(idx as u64) as u32;
                    let text = opt
                        .get("text")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let votes = opt
                        .get("vote_count")
                        .or_else(|| opt.get("voters"))
                        .or_else(|| opt.get("votes"))
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0) as u32;
                    let is_selected = opt
                        .get("is_chosen")
                        .or_else(|| opt.get("chosen"))
                        .or_else(|| opt.get("is_selected"))
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);
                    let is_correct = opt
                        .get("is_correct")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(correct_option_id == Some(id));
                    let percentage = opt
                        .get("percentage")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0) as f32;
                    PollOption {
                        id,
                        text,
                        votes,
                        percentage,
                        is_selected,
                        is_correct,
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    let sum_votes: u32 = options.iter().map(|o| o.votes).sum();
    let total_votes = declared_total.max(sum_votes);
    if total_votes > 0 {
        for opt in &mut options {
            if opt.percentage <= 0.0 && opt.votes > 0 {
                opt.percentage = (opt.votes as f32 * 100.0) / (total_votes as f32);
            }
        }
    }
    let is_closed = payload
        .get("is_closed")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let selected_option_id = options.iter().find(|o| o.is_selected).map(|o| o.id);

    MessageContent::Poll {
        question,
        poll_type,
        options,
        total_votes,
        is_closed,
        selected_option_id,
    }
}

fn decode_historical_content(hm: &HistoricalMessage) -> MessageContent {
    let Some(val) = hm.content.as_ref() else {
        return MessageContent::Text {
            text: hm.text.clone(),
            is_edited: false,
        };
    };
    let kind_lower = val
        .get("kind")
        .or_else(|| val.get("type"))
        .and_then(|v| v.as_str())
        .map(str::to_ascii_lowercase);
    let payload = val.get("data").unwrap_or(val);
    match kind_lower.as_deref() {
        Some("poll") => decode_poll_content(payload, &hm.text),
        Some("voice") => MessageContent::Voice {
            duration_secs: payload
                .get("duration_secs")
                .or_else(|| payload.get("duration_seconds"))
                .and_then(|v| v.as_u64())
                .unwrap_or(1) as u32,
            waveform_5bit: payload
                .get("waveform_5bit")
                .or_else(|| payload.get("waveform"))
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|n| n.as_u64().map(|x| x.min(31) as u8))
                        .collect()
                })
                .filter(|w: &Vec<u8>| !w.is_empty())
                .unwrap_or_else(|| vec![6, 12, 18, 24, 16, 10]),
            playback_position_secs: 0.0,
            is_playing: false,
            playback_speed: 1.0,
        },
        Some("photo") | Some("image") | Some("video") | Some("media") => MessageContent::Media {
            media_type: if kind_lower.as_deref() == Some("video") {
                MediaType::Video
            } else {
                MediaType::Photo
            },
            url: payload
                .get("url")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            width: payload.get("width").and_then(|v| v.as_u64()).unwrap_or(640) as u32,
            height: payload.get("height").and_then(|v| v.as_u64()).unwrap_or(480) as u32,
            duration_secs: payload
                .get("duration_secs")
                .or_else(|| payload.get("duration_seconds"))
                .and_then(|v| v.as_u64())
                .map(|n| n as u32),
            file_size_bytes: payload
                .get("file_size_bytes")
                .or_else(|| payload.get("size_bytes"))
                .and_then(|v| v.as_u64())
                .unwrap_or(0),
            caption: payload
                .get("caption")
                .and_then(|v| v.as_str())
                .map(str::to_string)
                .or_else(|| (!hm.text.is_empty()).then(|| hm.text.clone())),
            is_streaming: false,
        },
        Some("location") => MessageContent::Location {
            latitude: payload.get("latitude").and_then(|v| v.as_f64()).unwrap_or(0.0),
            longitude: payload.get("longitude").and_then(|v| v.as_f64()).unwrap_or(0.0),
            title: payload
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or(&hm.text)
                .to_string(),
            address: payload
                .get("address")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        },
        Some("encrypted") => MessageContent::Text {
            text: if hm.text.trim().is_empty() {
                "🔒 Encrypted message".to_string()
            } else {
                hm.text.clone()
            },
            is_edited: false,
        },
        Some("generativecard") | Some("generative_card") => {
            if let Ok(card) = serde_json::from_value::<gpui_telegram_proto::cards::GenerativeCard>(payload.clone()) {
                MessageContent::GenerativeCard(card)
            } else {
                MessageContent::Text {
                    text: hm.text.clone(),
                    is_edited: false,
                }
            }
        }
        _ => MessageContent::Text {
            text: hm.text.clone(),
            is_edited: payload
                .get("is_edited")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
        },
    }
}

pub fn apply_fenced_chat_list_response(
    fence: &SessionRequestFence,
    is_authenticated: bool,
    current_auth_gen: u64,
    current_req_gen: u64,
    current_token: Option<&str>,
    current_user_id: Option<u64>,
    sidebar: &mut ChatSidebar,
    stage: &mut ConversationStage,
    inspector: &mut ContextInspector,
    chats: &[ChatSummary],
) -> bool {
    if !fence.matches(
        is_authenticated,
        current_auth_gen,
        current_req_gen,
        current_token,
        current_user_id,
    ) {
        return false;
    }
    apply_authoritative_chat_list(sidebar, stage, inspector, chats);
    true
}

fn message_content_preview(content: &MessageContent) -> String {
    match content {
        MessageContent::Text { text, .. } => text.clone(),
        MessageContent::Media { caption, .. } => {
            caption.clone().unwrap_or_else(|| "Media".to_string())
        }
        MessageContent::Voice { duration_secs, .. } => format!("Voice ({duration_secs}s)"),
        MessageContent::Album { caption, .. } => {
            caption.clone().unwrap_or_else(|| "Album".to_string())
        }
        MessageContent::Location { title, .. } => title.clone(),
        MessageContent::Poll { question, .. } => question.clone(),
        MessageContent::Service { text, .. } => text.clone(),
        MessageContent::Call { status_text, .. } => status_text.clone(),
        MessageContent::Story { caption, .. } => caption.clone(),
        MessageContent::Invoice { title, .. } => title.clone(),
        MessageContent::BotKeyboard { .. } => "Keyboard".to_string(),
        MessageContent::GenerativeCard(card) => card.title.clone(),
    }
}

fn resolve_reply_references(models: &mut [MessageItemModel], existing: &[MessageItemModel]) {
    let mut index: HashMap<u64, (String, String)> = existing
        .iter()
        .map(|m| (m.id, (m.sender_name.clone(), message_content_preview(&m.content))))
        .collect();
    for m in models.iter() {
        index.insert(m.id, (m.sender_name.clone(), message_content_preview(&m.content)));
    }
    for m in models.iter_mut() {
        if let Some(ref mut reply) = m.reply_to {
            if let Some((orig_sender, orig_preview)) = index.get(&reply.message_id) {
                if reply.sender_name.is_empty() {
                    reply.sender_name = orig_sender.clone();
                }
                if reply.snippet.is_empty() {
                    reply.snippet = orig_preview.clone();
                }
            }
            if reply.sender_name.is_empty() {
                reply.sender_name = "Reply".to_string();
            }
            if reply.snippet.is_empty() {
                reply.snippet = format!("Message #{}", reply.message_id);
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn apply_fenced_history_response(
    fence: &SessionRequestFence,
    is_authenticated: bool,
    current_auth_gen: u64,
    current_history_req_gen: u64,
    current_token: Option<&str>,
    current_user_id: Option<u64>,
    sidebar: &ChatSidebar,
    stage: &mut ConversationStage,
    chat_id: u64,
    messages: Vec<HistoricalMessage>,
) -> bool {
    apply_fenced_history_response_preserving(
        fence,
        is_authenticated,
        current_auth_gen,
        current_history_req_gen,
        current_token,
        current_user_id,
        sidebar,
        stage,
        chat_id,
        messages,
        &std::collections::HashSet::new(),
    )
}

#[allow(clippy::too_many_arguments)]
pub fn apply_fenced_history_response_preserving(
    fence: &SessionRequestFence,
    is_authenticated: bool,
    current_auth_gen: u64,
    current_history_req_gen: u64,
    current_token: Option<&str>,
    current_user_id: Option<u64>,
    sidebar: &ChatSidebar,
    stage: &mut ConversationStage,
    chat_id: u64,
    messages: Vec<HistoricalMessage>,
    protected_reaction_msg_ids: &std::collections::HashSet<u64>,
) -> bool {
    if !fence.matches(
        is_authenticated,
        current_auth_gen,
        current_history_req_gen,
        current_token,
        current_user_id,
    ) {
        return false;
    }
    let Some(dialog) = sidebar.dialogs.iter().find(|d| d.id == chat_id) else {
        return false;
    };

    let mut models: Vec<MessageItemModel> = messages
        .iter()
        .map(|hm| {
            historical_message_to_model(
                hm,
                fence.user_id,
                &dialog.peer_name,
                &dialog.avatar_initials,
                dialog.avatar_color,
            )
        })
        .collect();

    let existing_refs: Vec<MessageItemModel> = if stage.active_chat_id == Some(chat_id) {
        stage.messages.clone()
    } else {
        stage
            .chat_histories
            .get(&chat_id)
            .cloned()
            .unwrap_or_default()
    };
    resolve_reply_references(&mut models, &existing_refs);

    let mut effective_protected = protected_reaction_msg_ids.clone();
    for hm in &messages {
        if !hm.has_authoritative_reactions() {
            effective_protected.insert(hm.message_id);
        }
    }

    if stage.active_chat_id == Some(chat_id) {
        let should_scroll_to_bottom =
            stage.messages.is_empty() || stage.is_anchored_to_bottom || stage.is_at_bottom();
        stage.merge_historical_messages_preserving(models, &effective_protected);
        stage.row_height_cache.rebuild_prefix_sums(&stage.messages);
        if should_scroll_to_bottom {
            stage.scroll_to_bottom();
        }
        stage.chat_histories.insert(chat_id, stage.messages.clone());
    } else {
        if !effective_protected.is_empty() {
            for m in &mut models {
                if effective_protected.contains(&m.id) {
                    if let Some(prev) = existing_refs.iter().find(|e| e.id == m.id) {
                        m.reactions = prev.reactions.clone();
                    }
                }
            }
        }
        for prev in &existing_refs {
            if prev.delivery_status == DeliveryStatus::Sending
                && !models.iter().any(|m| m.id == prev.id)
            {
                models.push(prev.clone());
            }
        }
        stage.chat_histories.insert(chat_id, models);
    }
    true
}

pub fn validate_selected_send_target(
    is_authenticated: bool,
    token: Option<&str>,
    user_id: Option<u64>,
    sidebar: &ChatSidebar,
    stage: &ConversationStage,
) -> Option<VerifiedSendTarget> {
    if !is_authenticated {
        return None;
    }
    let has_token = token.is_some_and(|t| !t.trim().is_empty());
    let uid = user_id.filter(|&id| id > 0)?;
    if !has_token {
        return None;
    }
    let chat_id = sidebar.active_chat_id?;
    if stage.active_chat_id != Some(chat_id) {
        return None;
    }
    let dialog = sidebar.dialogs.iter().find(|d| d.id == chat_id)?;
    Some(VerifiedSendTarget {
        chat_id,
        user_id: uid,
        peer_name: dialog.peer_name.clone(),
    })
}

pub fn try_consume_composer_for_send(
    is_authenticated: bool,
    token: Option<&str>,
    user_id: Option<u64>,
    sidebar: &ChatSidebar,
    stage: &ConversationStage,
    composer: &mut CanonicalComposer,
) -> Option<(VerifiedSendTarget, String)> {
    let candidate = composer.submitted_message.take()?;
    if candidate.trim().is_empty() {
        return None;
    }
    let Some(target) =
        validate_selected_send_target(is_authenticated, token, user_id, sidebar, stage)
    else {
        if composer.text().trim().is_empty() {
            composer.set_text(candidate);
        }
        return None;
    };

    composer.clear();
    Some((target, candidate))
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui_telegram_client::net::http_client::{HistoricalReaction, HistoricalReplyToInfo};

    fn selected_workspace(chat_id: u64) -> (ChatSidebar, ConversationStage, ContextInspector) {
        let mut sidebar = ChatSidebar::new();
        let mut stage = ConversationStage::new();
        let mut inspector = ContextInspector::new();
        let chats = vec![ChatSummary {
            chat_id,
            title: "Project Engineering".to_string(),
            chat_type: 2,
            last_message: Some("Hello".to_string()),
            unread_count: 0,
            is_group: Some(true),
            is_bot: Some(false),
            avatar_color: Some(0x65AADD),
            updated_at: Some(1_700_000_000_000),
        }];
        apply_authoritative_chat_list(&mut sidebar, &mut stage, &mut inspector, &chats);
        assert!(select_authoritative_dialog(
            &mut sidebar,
            &mut stage,
            &mut inspector,
            chat_id,
        ));
        (sidebar, stage, inspector)
    }

    #[test]
    fn try_consume_composer_for_send_returns_none_while_typing_and_sends_only_when_submitted() {
        let (sidebar, stage, _inspector) = selected_workspace(101);
        let mut composer = CanonicalComposer::new();

        composer.set_text("Typing draft in composer");
        assert!(composer.submitted_message.is_none());
        assert!(
            try_consume_composer_for_send(
                true,
                Some("tok"),
                Some(10),
                &sidebar,
                &stage,
                &mut composer,
            )
            .is_none(),
            "keystroke notifications must never consume composer.text() without explicit submission"
        );
        assert_eq!(composer.text(), "Typing draft in composer");

        composer.submitted_message = Some("Typing draft in composer".to_string());
        let (target, sent) = try_consume_composer_for_send(
            true,
            Some("tok"),
            Some(10),
            &sidebar,
            &stage,
            &mut composer,
        )
        .expect("explicitly submitted message should send");
        assert_eq!(target.chat_id, 101);
        assert_eq!(sent, "Typing draft in composer");
        assert_eq!(composer.text(), "");
        assert_eq!(composer.submitted_message, None);
    }

    #[test]
    fn select_authoritative_dialog_populates_inspector_avatar() {
        let (_sidebar, _stage, inspector) = selected_workspace(101);
        assert_eq!(inspector.profile.name, "Project Engineering");
        assert_eq!(inspector.profile.avatar_initials, "PE");
        assert_eq!(inspector.profile.avatar_color, 0x65AADD);
    }

    #[test]
    fn decode_historical_content_decodes_tagged_poll_into_poll_card_model() {
        let hm = HistoricalMessage {
            message_id: 55,
            sender_id: 20,
            text: "[Poll: Favorite Rust UI?]".to_string(),
            content: Some(serde_json::json!({
                "type": "Poll",
                "data": {
                    "id": "poll_55",
                    "poll_id": 55,
                    "question": "Favorite Rust UI?",
                    "options": [
                        { "option_id": 0, "text": "GPUI", "vote_count": 3, "is_chosen": true, "percentage": 75.0 },
                        { "option_id": 1, "text": "Other", "vote_count": 1, "is_chosen": false, "percentage": 25.0 }
                    ],
                    "total_voters": 4,
                    "is_closed": false,
                    "is_anonymous": false,
                    "is_quiz": false
                }
            })),
            date: Some(1_700_000_000_000),
            reply_to: None,
            is_outgoing: Some(false),
            ..Default::default()
        };

        let model = historical_message_to_model(&hm, 10, "Project Engineering", "PE", 0x65AADD);
        match model.content {
            MessageContent::Poll {
                question,
                poll_type,
                options,
                total_votes,
                is_closed,
                selected_option_id,
            } => {
                assert_eq!(question, "Favorite Rust UI?");
                assert_eq!(poll_type, PollType::Regular);
                assert_eq!(total_votes, 4);
                assert!(!is_closed);
                assert_eq!(selected_option_id, Some(0));
                assert_eq!(options.len(), 2);
                assert_eq!(options[0].text, "GPUI");
                assert_eq!(options[0].votes, 3);
                assert!(options[0].is_selected);
            }
            other => panic!("expected MessageContent::Poll, got {:?}", other),
        }
    }

    #[test]
    fn history_populates_reactions_and_reply_references_from_info_and_history_lookup() {
        let (sidebar, mut stage, _inspector) = selected_workspace(101);
        let fence = SessionRequestFence {
            auth_generation: 1,
            request_generation: 2,
            token: "tok".to_string(),
            user_id: 10,
        };
        let msgs = vec![
            HistoricalMessage {
                message_id: 1,
                sender_id: 20,
                text: "Original announcement".to_string(),
                content: None,
                date: Some(1_700_000_000_000),
                reply_to: None,
                is_outgoing: Some(false),
                reactions: vec![HistoricalReaction {
                    emoji: "🔥".to_string(),
                    count: 2,
                    chosen: true,
                    user_ids: vec![10, 20],
                }],
                ..Default::default()
            },
            HistoricalMessage {
                message_id: 2,
                sender_id: 10,
                text: "Looks great!".to_string(),
                content: None,
                date: Some(1_700_000_005_000),
                reply_to: Some(1),
                is_outgoing: Some(true),
                reply_to_info: Some(HistoricalReplyToInfo {
                    reply_to_msg_id: Some(1),
                    quote_text: None,
                    ..Default::default()
                }),
                ..Default::default()
            },
        ];

        assert!(apply_fenced_history_response(
            &fence,
            true,
            1,
            2,
            Some("tok"),
            Some(10),
            &sidebar,
            &mut stage,
            101,
            msgs,
        ));
        assert_eq!(stage.messages.len(), 2);
        assert_eq!(
            stage.messages[0].reactions,
            vec![ReactionItem {
                emoji: "🔥".to_string(),
                count: 2,
                user_reacted: true,
            }]
        );
        let reply = stage.messages[1].reply_to.as_ref().expect("reply ref");
        assert_eq!(reply.message_id, 1);
        assert_eq!(reply.sender_name, "Project Engineering");
        assert_eq!(reply.snippet, "Original announcement");
    }
}
