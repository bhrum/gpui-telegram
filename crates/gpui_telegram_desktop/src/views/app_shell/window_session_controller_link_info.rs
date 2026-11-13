//! WindowSessionControllerLinkInfo: Chat/user deep link parser and routing descriptors
//! Upstream: Telegram/SourceFiles/window/window_session_controller_link_info.h
//! Subsystem: window | Feature ID: TCG-N02 | Disposition: gpui_ui_1to1

use gpui::*;
use serde::{Deserialize, Serialize};

/// Link resolution intent matching upstream Window::ResolveType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolveType {
    Default,
    BotApp,
    BotStart,
    AddToGroup,
    AddToChannel,
    HashtagSearch,
    ShareGame,
    Mention,
    Boost,
    ChannelDirect,
    Profile,
}

impl Default for ResolveType {
    fn default() -> Self {
        Self::Default
    }
}

/// Comment or thread reply target extracted from a deep link
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RepliesByLinkInfo {
    None,
    Comment(i64),
    Thread(i64),
}

impl Default for RepliesByLinkInfo {
    fn default() -> Self {
        Self::None
    }
}

/// Information record for links resolving to peers, chats, bots, or channel posts
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerByLinkInfo {
    pub username_or_id: Option<String>,
    pub channel_id: Option<u64>,
    pub phone: Option<String>,
    pub chat_link_slug: Option<String>,
    pub contact_token: Option<String>,
    pub message_id: Option<i64>,
    pub story_id: Option<i32>,
    pub video_timestamp: Option<i64>,
    pub text: Option<String>,
    pub replies_info: RepliesByLinkInfo,
    pub resolve_type: ResolveType,
    pub referral: Option<String>,
    pub start_token: Option<String>,
    pub start_auto_submit: bool,
    pub join_channel: bool,
    pub bot_app_name: Option<String>,
    pub voicechat_hash: Option<String>,
    pub history_in_new_window: bool,
}

impl PeerByLinkInfo {
    pub fn new() -> Self {
        Self {
            username_or_id: None,
            channel_id: None,
            phone: None,
            chat_link_slug: None,
            contact_token: None,
            message_id: None,
            story_id: None,
            video_timestamp: None,
            text: None,
            replies_info: RepliesByLinkInfo::None,
            resolve_type: ResolveType::Default,
            referral: None,
            start_token: None,
            start_auto_submit: false,
            join_channel: false,
            bot_app_name: None,
            voicechat_hash: None,
            history_in_new_window: false,
        }
    }

    pub fn for_username(username: impl Into<String>) -> Self {
        let mut info = Self::new();
        info.username_or_id = Some(username.into());
        info.resolve_type = ResolveType::Profile;
        info
    }

    pub fn for_chat_slug(slug: impl Into<String>) -> Self {
        let mut info = Self::new();
        info.chat_link_slug = Some(slug.into());
        info
    }

    pub fn for_channel_post(channel_id: u64, message_id: i64) -> Self {
        let mut info = Self::new();
        info.channel_id = Some(channel_id);
        info.message_id = Some(message_id);
        info.resolve_type = ResolveType::ChannelDirect;
        info
    }

    pub fn with_start_token(mut self, token: impl Into<String>) -> Self {
        self.start_token = Some(token.into());
        self.resolve_type = ResolveType::BotStart;
        self
    }

    pub fn with_bot_app(mut self, app_name: impl Into<String>) -> Self {
        self.bot_app_name = Some(app_name.into());
        self.resolve_type = ResolveType::BotApp;
        self
    }

    pub fn with_voicechat(mut self, hash: impl Into<String>) -> Self {
        self.voicechat_hash = Some(hash.into());
        self
    }
}

impl Default for PeerByLinkInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// Categorized deep link descriptor
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinkInfoType {
    Peer(PeerByLinkInfo),
    Url(String),
    Internal(String),
    Unknown,
}

/// Controller parsing, validating, and routing Telegram deep links
#[derive(Debug, Clone)]
pub struct WindowSessionControllerLinkInfo {
    pub active_link: Option<LinkInfoType>,
    pub history: Vec<LinkInfoType>,
}

impl WindowSessionControllerLinkInfo {
    pub fn new() -> Self {
        Self {
            active_link: None,
            history: Vec::new(),
        }
    }

    /// Parses deep link URLs like `tg://resolve?domain=username&start=token` or `https://t.me/...`
    pub fn parse_deep_link(url: &str) -> LinkInfoType {
        let trimmed = url.trim();

        // 1. tg://resolve?domain=...
        if let Some(rest) = trimmed.strip_prefix("tg://resolve?") {
            let mut info = PeerByLinkInfo::new();
            for part in rest.split('&') {
                if let Some((k, v)) = part.split_once('=') {
                    match k {
                        "domain" => {
                            info.username_or_id = Some(v.to_string());
                            info.resolve_type = ResolveType::Profile;
                        }
                        "start" => {
                            info.start_token = Some(v.to_string());
                            info.resolve_type = ResolveType::BotStart;
                        }
                        "appname" => {
                            info.bot_app_name = Some(v.to_string());
                            info.resolve_type = ResolveType::BotApp;
                        }
                        "post" => {
                            info.message_id = v.parse::<i64>().ok();
                        }
                        "comment" => {
                            if let Ok(cid) = v.parse::<i64>() {
                                info.replies_info = RepliesByLinkInfo::Comment(cid);
                            }
                        }
                        "topic" => {
                            if let Ok(tid) = v.parse::<i64>() {
                                info.replies_info = RepliesByLinkInfo::Thread(tid);
                            }
                        }
                        "voicechat" => {
                            info.voicechat_hash = Some(v.to_string());
                        }
                        _ => {}
                    }
                }
            }
            if info.username_or_id.is_some() {
                return LinkInfoType::Peer(info);
            }
        }

        // 2. tg://join?invite=...
        if let Some(rest) = trimmed.strip_prefix("tg://join?") {
            for part in rest.split('&') {
                if let Some((k, v)) = part.split_once('=') {
                    if k == "invite" {
                        let mut info = PeerByLinkInfo::new();
                        info.chat_link_slug = Some(v.to_string());
                        info.join_channel = true;
                        return LinkInfoType::Peer(info);
                    }
                }
            }
        }

        // 3. https://t.me/username or https://telegram.me/username
        if let Some(path) = trimmed
            .strip_prefix("https://t.me/")
            .or_else(|| trimmed.strip_prefix("http://t.me/"))
            .or_else(|| trimmed.strip_prefix("https://telegram.me/"))
        {
            let clean_path = path.split('?').next().unwrap_or(path);
            let mut segments = clean_path.split('/').filter(|s| !s.is_empty());
            if let Some(first) = segments.next() {
                if first.starts_with('+') || first.starts_with("joinchat") {
                    let mut info = PeerByLinkInfo::new();
                    info.chat_link_slug = Some(first.to_string());
                    return LinkInfoType::Peer(info);
                } else {
                    let mut info = PeerByLinkInfo::for_username(first);
                    if let Some(second) = segments.next() {
                        if let Ok(mid) = second.parse::<i64>() {
                            info.message_id = Some(mid);
                        }
                    }
                    return LinkInfoType::Peer(info);
                }
            }
        }

        // Fallback for regular http(s) URLs
        if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
            return LinkInfoType::Url(trimmed.to_string());
        }

        LinkInfoType::Unknown
    }

    pub fn open_link(&mut self, url: &str) -> LinkInfoType {
        let parsed = Self::parse_deep_link(url);
        self.active_link = Some(parsed.clone());
        self.history.push(parsed.clone());
        parsed
    }

    pub fn open_peer_info(&mut self, info: PeerByLinkInfo) {
        let link = LinkInfoType::Peer(info);
        self.active_link = Some(link.clone());
        self.history.push(link);
    }

    pub fn current_link(&self) -> Option<&LinkInfoType> {
        self.active_link.as_ref()
    }

    pub fn clear(&mut self) {
        self.active_link = None;
        self.history.clear();
    }
}

impl Default for WindowSessionControllerLinkInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowSessionControllerLinkInfo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let status = match &self.active_link {
            Some(LinkInfoType::Peer(p)) => {
                format!("Resolved Peer: {:?}", p.username_or_id)
            }
            Some(LinkInfoType::Url(u)) => format!("External URL: {}", u),
            Some(LinkInfoType::Internal(i)) => format!("Internal Route: {}", i),
            Some(LinkInfoType::Unknown) => "Unknown Link".to_string(),
            None => "No Active Link".to_string(),
        };

        div()
            .flex()
            .flex_col()
            .p(px(8.0))
            .bg(rgb(0x17212B))
            .rounded(px(6.0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_sm()
                    .text_color(rgb(0xFFFFFF))
                    .child("Deep Link Resolver")
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x82868A))
                    .child(format!("Status: {}, History: {}", status, self.history.len()))
            )
    }
}
