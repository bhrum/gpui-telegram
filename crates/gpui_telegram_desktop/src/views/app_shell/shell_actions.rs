use gpui_telegram_client::net::http_client::{ChatDetails, ChatMemberItem};
use gpui_telegram_client::AccountSessionRecord;
use gpui_telegram_proto::contracts::auth::ContactDto;
use gpui_telegram_proto::contracts::features::{CallRecordDto, ChannelStatisticsDto};

use crate::i18n::tr_str;
use crate::views::app_shell::shell_panels::{ShellAcpTaskRow, ShellMcpServerRow, ShellMcpToolRow};
use crate::views::app_shell::shell_truth::{derive_initials, format_authoritative_timestamp};
use crate::views::boxes::modal_types::CallLogEntry;
use crate::views::chat_sidebar::DialogItemModel;
use crate::views::drawer::inspector_tabs::{
    MemberItem, SharedFileItem, SharedLinkItem, SharedMediaItem, SharedVoiceItem,
};
use crate::views::drawer::statistics::info_statistics_inner_widget::ChartDataPoint;
use crate::views::global_tasks_workspace::{AcpTaskEntry, McpServerEntry};
use crate::views::media_viewer::MediaViewerItem;
use crate::views::stage::cards::{MediaType, MessageContent, MessageItemModel};

const PEER_COLORS: [u32; 7] = [0xE17076, 0xFAA774, 0xA695E7, 0x7BC862, 0x6EC9CB, 0x65AADD, 0xEE7AAE];

pub fn peer_avatar_color(id: u64) -> u32 {
    PEER_COLORS[(id as usize) % PEER_COLORS.len()]
}

pub fn resolve_active_target_chat_id(stage_chat_id: Option<u64>, sidebar_selected: Option<u64>) -> Option<u64> {
    stage_chat_id.or(sidebar_selected).filter(|&id| id > 0)
}

pub fn chat_dialogs_to_forward_targets(dialogs: &[DialogItemModel]) -> Vec<(u64, String, String, u32)> {
    dialogs
        .iter()
        .map(|d| (d.id, d.peer_name.clone(), d.avatar_initials.clone(), d.avatar_color))
        .collect()
}

pub fn build_chat_profile_facts(details: &ChatDetails, lang_id: &str) -> Vec<(String, String)> {
    let mut facts = Vec::new();
    if let Some(about) = details.about.as_deref().filter(|s| !s.trim().is_empty()) {
        facts.push((tr_str(lang_id, "简介", "Bio").to_string(), about.to_string()));
    }
    let kind_label = match details.chat_type {
        3 => tr_str(lang_id, "频道", "Channel"),
        2 => tr_str(lang_id, "超级群组", "Supergroup"),
        1 => tr_str(lang_id, "群组", "Group"),
        _ => tr_str(lang_id, "私聊", "Private Chat"),
    };
    facts.push((tr_str(lang_id, "类型", "Type").to_string(), kind_label.to_string()));
    if details.members_count > 0 {
        facts.push((tr_str(lang_id, "成员", "Members").to_string(), details.members_count.to_string()));
    }
    if details.is_forum {
        facts.push((tr_str(lang_id, "话题论坛", "Topics").to_string(), tr_str(lang_id, "已启用", "Enabled").to_string()));
    }
    facts
}

pub fn map_chat_members_to_inspector(members: &[ChatMemberItem], contacts: &[ContactDto]) -> Vec<MemberItem> {
    members
        .iter()
        .map(|m| {
            let name = contacts
                .iter()
                .find(|c| c.user_id == m.user_id)
                .map(|c| {
                    let full = format!("{} {}", c.first_name, c.last_name.as_deref().unwrap_or("")).trim().to_string();
                    if full.is_empty() { format!("User {}", m.user_id) } else { full }
                })
                .unwrap_or_else(|| format!("User {}", m.user_id));
            MemberItem {
                id: m.user_id,
                initials: derive_initials(&name),
                name,
                color: peer_avatar_color(m.user_id),
                role: m.role.clone(),
                is_online: false,
            }
        })
        .collect()
}

pub fn map_channel_stats_to_chart(stats: &ChannelStatisticsDto) -> (u32, u32, u32, Vec<ChartDataPoint>) {
    let bars = stats
        .daily_series
        .iter()
        .map(|pt| ChartDataPoint { label: pt.day.clone(), value: pt.messages as usize })
        .collect();
    let target = stats.member_count.max(10) as u32;
    let current = stats.active_posters_7d as u32;
    let level = (current / 25).min(10);
    (level, current, target, bars)
}

pub fn map_stage_media_to_gallery(
    messages: &[MessageItemModel],
) -> (Vec<MediaViewerItem>, Vec<SharedMediaItem>, Vec<SharedFileItem>, Vec<SharedLinkItem>, Vec<SharedVoiceItem>) {
    let (mut viewer, mut media, mut files, mut links, mut voices) = (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for msg in messages {
        match &msg.content {
            MessageContent::Media { media_type, url, duration_secs, caption, .. } => {
                let is_video = matches!(media_type, MediaType::Video);
                viewer.push(MediaViewerItem {
                    id: msg.id,
                    url: url.clone(),
                    is_video,
                    caption: caption.clone(),
                    sender_name: msg.sender_name.clone(),
                    date: msg.timestamp.clone(),
                    duration_secs: *duration_secs,
                });
                media.push(SharedMediaItem {
                    id: msg.id,
                    url: url.clone(),
                    local_path: None,
                    caption: caption.clone().unwrap_or_default(),
                    is_video,
                    duration_sec: duration_secs.unwrap_or(0),
                });
            }
            MessageContent::Voice { duration_secs, .. } => {
                voices.push(SharedVoiceItem {
                    id: msg.id,
                    title: if msg.sender_name.is_empty() { format!("Voice #{}", msg.id) } else { msg.sender_name.clone() },
                    duration_sec: *duration_secs,
                    date: msg.timestamp.clone(),
                });
            }
            MessageContent::Text { text, .. } => {
                if let Some(url_start) = text.find("http://").or_else(|| text.find("https://")) {
                    let raw = text[url_start..].split_whitespace().next().unwrap_or("").to_string();
                    let domain = raw.trim_start_matches("https://").trim_start_matches("http://").split('/').next().unwrap_or("").to_string();
                    if !raw.is_empty() {
                        links.push(SharedLinkItem { title: msg.sender_name.clone(), url: raw, domain });
                    }
                }
                if text.starts_with("📎 ") {
                    let name = text.trim_start_matches("📎 ").to_string();
                    let ext = name.rsplit('.').next().unwrap_or("file").to_ascii_uppercase();
                    files.push(SharedFileItem { name, size_text: String::new(), extension: ext, date: msg.timestamp.clone() });
                }
            }
            _ => {}
        }
    }
    (viewer, media, files, links, voices)
}

pub fn shell_tasks_to_acp_entries(tasks: &[ShellAcpTaskRow]) -> Vec<AcpTaskEntry> {
    tasks
        .iter()
        .map(|t| AcpTaskEntry {
            id: t.task_id.clone(),
            title: t.title.clone(),
            status: t.status.clone(),
            schedule: None,
            last_output: None,
            requires_approval: t.requires_approval,
        })
        .collect()
}

pub fn shell_mcp_to_entries(servers: &[ShellMcpServerRow], tools: &[ShellMcpToolRow]) -> Vec<McpServerEntry> {
    servers
        .iter()
        .map(|srv| {
            let srv_tools = tools
                .iter()
                .filter(|t| t.server_name == srv.name || t.server_name == srv.server_id)
                .map(|t| t.name.clone())
                .collect();
            McpServerEntry {
                id: if srv.server_id.is_empty() { srv.name.clone() } else { srv.server_id.clone() },
                name: srv.name.clone(),
                endpoint: srv.status.clone(),
                is_connected: srv.status.eq_ignore_ascii_case("connected") || srv.status.eq_ignore_ascii_case("online"),
                tools: srv_tools,
            }
        })
        .collect()
}

pub fn contacts_to_modal_rows(contacts: &[ContactDto]) -> Vec<(String, String, String, u32, bool)> {
    contacts
        .iter()
        .map(|c| {
            let name = format!("{} {}", c.first_name, c.last_name.as_deref().unwrap_or("")).trim().to_string();
            let display = if name.is_empty() { c.username.clone().unwrap_or_else(|| format!("User {}", c.user_id)) } else { name };
            let initials = derive_initials(&display);
            let sub = c.username.as_deref().map(|u| format!("@{u}")).or_else(|| c.phone.clone()).unwrap_or_default();
            (display, sub, initials, peer_avatar_color(c.user_id), false)
        })
        .collect()
}

pub fn localize_call_state(state: &str, lang_id: &str) -> String {
    match state.trim().to_ascii_lowercase().as_str() {
        "active" | "connected" | "accepted" => tr_str(lang_id, "通话中", "Active").to_string(),
        "ringing" | "initiated" => tr_str(lang_id, "响铃中", "Ringing").to_string(),
        "ended" => tr_str(lang_id, "已结束", "Ended").to_string(),
        "missed" => tr_str(lang_id, "未接来电", "Missed").to_string(),
        "declined" => tr_str(lang_id, "已拒接", "Declined").to_string(),
        "cancelled" | "canceled" => tr_str(lang_id, "已取消", "Cancelled").to_string(),
        "" => tr_str(lang_id, "通话", "Call").to_string(),
        _ => state.to_string(),
    }
}

pub fn calls_to_modal_logs(calls: &[CallRecordDto], contacts: &[ContactDto], lang_id: &str) -> Vec<CallLogEntry> {
    calls
        .iter()
        .map(|c| {
            let peer_id = c.callee_id;
            let peer_name = contacts
                .iter()
                .find(|ct| ct.user_id == peer_id || ct.user_id == c.caller_id)
                .map(|ct| format!("{} {}", ct.first_name, ct.last_name.as_deref().unwrap_or("")).trim().to_string())
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| format!("User {}", peer_id));
            let is_missed = c.state.eq_ignore_ascii_case("missed") || c.state.eq_ignore_ascii_case("declined");
            let state_label = localize_call_state(&c.state, lang_id);
            let description = if c.duration_secs > 0 {
                format!("{state_label} · {}s", c.duration_secs)
            } else {
                state_label
            };
            CallLogEntry {
                peer_name,
                description,
                timestamp: format_authoritative_timestamp(Some(c.started_at_ms)),
                is_missed,
                is_video: c.is_video,
            }
        })
        .collect()
}

pub fn build_account_record(user_id: u64, full_name: &str, username: &str, phone: &str, bio: &str) -> AccountSessionRecord {
    let display = if full_name.trim().is_empty() { username.trim() } else { full_name.trim() };
    let safe_name = if display.is_empty() { format!("User {user_id}") } else { display.to_string() };
    AccountSessionRecord {
        account_id: user_id as u32,
        display_name: safe_name.clone(),
        username: username.to_string(),
        phone: phone.to_string(),
        bio: bio.to_string(),
        emoji_status: String::new(),
        is_premium: false,
        is_verified: false,
        avatar_initials: derive_initials(&safe_name),
        avatar_color: peer_avatar_color(user_id),
        unread_chats: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_chat_profile_facts_distinguishes_supergroup_and_channel() {
        let supergroup = ChatDetails {
            chat_id: 10,
            title: "Rust Guild".to_string(),
            chat_type: 2,
            creator_id: 1,
            about: None,
            members_count: 12,
            is_forum: false,
            photo_file_id: None,
            pts: 0,
        };
        let channel = ChatDetails {
            chat_type: 3,
            ..supergroup.clone()
        };

        let sg_zh = build_chat_profile_facts(&supergroup, "zh-CN");
        assert!(sg_zh.iter().any(|(k, v)| k == "类型" && v == "超级群组"));
        let sg_en = build_chat_profile_facts(&supergroup, "en");
        assert!(sg_en.iter().any(|(k, v)| k == "Type" && v == "Supergroup"));

        let ch_zh = build_chat_profile_facts(&channel, "zh-CN");
        assert!(ch_zh.iter().any(|(k, v)| k == "类型" && v == "频道"));
        let ch_en = build_chat_profile_facts(&channel, "en");
        assert!(ch_en.iter().any(|(k, v)| k == "Type" && v == "Channel"));
    }

    #[test]
    fn localize_call_state_translates_active_and_ended_states() {
        assert_eq!(localize_call_state("active", "zh-CN"), "通话中");
        assert_eq!(localize_call_state("active", "en"), "Active");
        assert_eq!(localize_call_state("missed", "zh-CN"), "未接来电");
    }
}
