//! Info Profile Top Bar Action Button
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_top_bar_action_button.cpp:L1-L221`
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_top_bar_action_button.h:L1-L77`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopBarActionType {
    Call,
    VideoCall,
    Search,
    Mute,
    MoreMenu,
    Share,
}

/// Upstream `TopBarActionButtonStyle`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopBarActionButtonStyle {
    pub bg_color: u32,
    pub fg_color: Option<u32>,
    pub shadow_color: Option<u32>,
}

/// Action button placed on the profile drawer top bar.
#[derive(Debug, Clone)]
pub struct InfoProfileTopBarActionButtonProfile {
    pub action_type: TopBarActionType,
    pub is_hovered: bool,
    pub is_disabled: bool,
    pub badge: Option<String>,
    pub is_toggle: bool,
    pub toggle_state: bool,
}

impl InfoProfileTopBarActionButtonProfile {
    /// Upstream `kIconFadeStart = 0.4` (`info_profile_top_bar_action_button.cpp:L21`).
    pub const ICON_FADE_START: f32 = 0.4;
    /// Upstream `kIconFadeRange = 1.0 - kIconFadeStart = 0.6` (`info_profile_top_bar_action_button.cpp:L22`).
    pub const ICON_FADE_RANGE: f32 = 0.6;

    pub fn new(action_type: TopBarActionType) -> Self {
        Self {
            action_type,
            is_hovered: false,
            is_disabled: false,
            badge: None,
            is_toggle: false,
            toggle_state: false,
        }
    }

    pub fn set_hovered(&mut self, hovered: bool) {
        self.is_hovered = hovered;
    }

    pub fn set_disabled(&mut self, disabled: bool) {
        self.is_disabled = disabled;
    }

    pub fn toggle(&mut self, state: bool) {
        self.is_toggle = true;
        self.toggle_state = state;
    }

    pub fn icon_fade_opacity(&self, progress: f32) -> f32 {
        if progress < Self::ICON_FADE_START {
            1.0
        } else {
            let fade_progress = (progress - Self::ICON_FADE_START) / Self::ICON_FADE_RANGE;
            (1.0 - fade_progress).clamp(0.0, 1.0)
        }
    }

    pub fn icon_text(&self) -> &'static str {
        match self.action_type {
            TopBarActionType::Call => "📞",
            TopBarActionType::VideoCall => "📹",
            TopBarActionType::Search => "🔍",
            TopBarActionType::Mute => {
                if self.toggle_state {
                    "🔕"
                } else {
                    "🔔"
                }
            }
            TopBarActionType::MoreMenu => "⋮",
            TopBarActionType::Share => "↗",
        }
    }
}

impl Default for InfoProfileTopBarActionButtonProfile {
    fn default() -> Self {
        Self::new(TopBarActionType::Search)
    }
}

impl Render for InfoProfileTopBarActionButtonProfile {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let icon = self.icon_text();
        let is_hovered = self.is_hovered;

        div()
            .flex()
            .items_center()
            .justify_center()
            .w(px(32.0))
            .h(px(32.0))
            .rounded_full()
            .bg(if is_hovered { rgb(0xE6E8EB) } else { rgb(0xF1F2F4) })
            .text_sm()
            .child(icon)
    }
}
