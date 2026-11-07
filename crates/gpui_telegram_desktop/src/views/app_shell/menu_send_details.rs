//! SendDetails: Extended message dispatch options and metadata payload
//! Upstream: Telegram/SourceFiles/menu/menu_send_details.h, menu_send_details.cpp
//! Subsystem: window | Feature ID: TCG-N46 | Disposition: gpui_ui_1to1

use gpui::*;
use serde::{Deserialize, Serialize};

/// Type of extended send action matching upstream SendMenu::Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SendMenuType {
    Disabled,
    SilentOnly,
    Scheduled,
    ScheduledToUser, // "Send when online"
    Reminder,
    EditCommentPrice,
}

impl Default for SendMenuType {
    fn default() -> Self {
        Self::Disabled
    }
}

/// Spoiler state for attached media
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpoilerState {
    None,
    Enabled,
    Possible,
}

impl Default for SpoilerState {
    fn default() -> Self {
        Self::None
    }
}

/// Caption positioning state (above or below media)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CaptionState {
    None,
    Below,
    Above,
}

impl Default for CaptionState {
    fn default() -> Self {
        Self::None
    }
}

/// Photo upload quality preference
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhotoQualityState {
    None,
    Standard,
    High,
}

impl Default for PhotoQualityState {
    fn default() -> Self {
        Self::None
    }
}

/// Video/audio cover state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoverState {
    None,
    Add,
    Has,
}

impl Default for CoverState {
    fn default() -> Self {
        Self::None
    }
}

/// Authentic send details configuration for scheduled/silent delivery and media formatting
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SendDetails {
    pub send_type: SendMenuType,
    pub silent: bool,
    pub scheduled_at: Option<i64>,
    pub send_as_peer_id: Option<u64>,
    pub bare_topic_root_id: i64,
    pub spoiler: SpoilerState,
    pub caption: CaptionState,
    pub invert_caption: bool,
    pub photo_quality: PhotoQualityState,
    pub cover: CoverState,
    pub comment_preview: String,
    pub comment_streamer_name: String,
    pub price: Option<u64>,
    pub comment_price_min: Option<u64>,
    pub effect_allowed: bool,
}

impl SendDetails {
    pub fn new() -> Self {
        Self {
            send_type: SendMenuType::Disabled,
            silent: false,
            scheduled_at: None,
            send_as_peer_id: None,
            bare_topic_root_id: 0,
            spoiler: SpoilerState::None,
            caption: CaptionState::None,
            invert_caption: false,
            photo_quality: PhotoQualityState::None,
            cover: CoverState::None,
            comment_preview: String::new(),
            comment_streamer_name: String::new(),
            price: None,
            comment_price_min: None,
            effect_allowed: false,
        }
    }

    pub fn silent() -> Self {
        let mut details = Self::new();
        details.silent = true;
        details.send_type = SendMenuType::SilentOnly;
        details
    }

    pub fn scheduled(timestamp: i64) -> Self {
        let mut details = Self::new();
        details.scheduled_at = Some(timestamp);
        details.send_type = SendMenuType::Scheduled;
        details
    }

    pub fn send_when_online() -> Self {
        let mut details = Self::new();
        details.send_type = SendMenuType::ScheduledToUser;
        details
    }

    pub fn with_send_as(mut self, peer_id: u64) -> Self {
        self.send_as_peer_id = Some(peer_id);
        self
    }

    pub fn set_silent(&mut self, silent: bool) {
        self.silent = silent;
        if silent && self.send_type == SendMenuType::Disabled {
            self.send_type = SendMenuType::SilentOnly;
        }
    }

    pub fn set_scheduled(&mut self, timestamp: Option<i64>) {
        self.scheduled_at = timestamp;
        if timestamp.is_some() {
            self.send_type = SendMenuType::Scheduled;
        } else if self.send_type == SendMenuType::Scheduled {
            self.send_type = SendMenuType::Disabled;
        }
    }

    pub fn set_send_as(&mut self, peer_id: Option<u64>) {
        self.send_as_peer_id = peer_id;
    }

    pub fn toggle_invert_caption(&mut self) {
        self.invert_caption = !self.invert_caption;
        if self.caption == CaptionState::Below {
            self.caption = CaptionState::Above;
        } else if self.caption == CaptionState::Above {
            self.caption = CaptionState::Below;
        }
    }

    pub fn is_scheduled(&self) -> bool {
        self.scheduled_at.is_some()
            || self.send_type == SendMenuType::Scheduled
            || self.send_type == SendMenuType::ScheduledToUser
    }

    pub fn is_silent(&self) -> bool {
        self.silent || self.send_type == SendMenuType::SilentOnly
    }
}

impl Default for SendDetails {
    fn default() -> Self {
        Self::new()
    }
}

/// GPUI View component for managing and rendering extended send options
#[derive(Debug, Clone)]
pub struct MenuSendDetails {
    pub details: SendDetails,
    pub is_open: bool,
}

impl MenuSendDetails {
    pub fn new() -> Self {
        Self {
            details: SendDetails::new(),
            is_open: false,
        }
    }

    pub fn with_details(details: SendDetails) -> Self {
        Self {
            details,
            is_open: true,
        }
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }

    pub fn details(&self) -> &SendDetails {
        &self.details
    }

    pub fn details_mut(&mut self) -> &mut SendDetails {
        &mut self.details
    }
}

impl Default for MenuSendDetails {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for MenuSendDetails {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_open {
            return div().into_any_element();
        }

        let send_type_desc = match self.details.send_type {
            SendMenuType::Disabled => "Immediate Send",
            SendMenuType::SilentOnly => "Send Silently",
            SendMenuType::Scheduled => "Scheduled Delivery",
            SendMenuType::ScheduledToUser => "Send When Online",
            SendMenuType::Reminder => "Set Reminder",
            SendMenuType::EditCommentPrice => "Paid Message",
        };

        div()
            .flex()
            .flex_col()
            .p(px(8.0))
            .bg(rgb(0x17212B))
            .rounded(px(6.0))
            .shadow_md()
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_sm()
                    .text_color(rgb(0xFFFFFF))
                    .child("Send Options")
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x82868A))
                    .child(format!("Mode: {}", send_type_desc))
            )
            .children(self.details.scheduled_at.map(|ts| {
                div()
                    .text_xs()
                    .text_color(rgb(0x5288C1))
                    .child(format!("Scheduled: {}", ts))
            }))
            .children(self.details.send_as_peer_id.map(|peer| {
                div()
                    .text_xs()
                    .text_color(rgb(0x82868A))
                    .child(format!("Send As Peer: {}", peer))
            }))
            .into_any_element()
    }
}
