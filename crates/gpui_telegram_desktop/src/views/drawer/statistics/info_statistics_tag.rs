//! Statistics Entity Tag
//! Upstream: `Telegram/SourceFiles/info/statistics/info_statistics_tag.h:L1-L31`
//! Subsystem: info | Feature ID: TCG-N12 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatisticsTargetKind {
    Channel,
    Supergroup,
    Message,
    Story,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InfoStatisticsTagStatistics {
    pub peer_id: u64,
    pub context_msg_id: Option<i32>,
    pub story_id: Option<i32>,
    pub is_visible: bool,
}

impl InfoStatisticsTagStatistics {
    pub fn new(peer_id: u64) -> Self {
        Self {
            peer_id,
            context_msg_id: None,
            story_id: None,
            is_visible: true,
        }
    }

    pub fn for_message(peer_id: u64, msg_id: i32) -> Self {
        Self {
            peer_id,
            context_msg_id: Some(msg_id),
            story_id: None,
            is_visible: true,
        }
    }

    pub fn for_story(peer_id: u64, story_id: i32) -> Self {
        Self {
            peer_id,
            context_msg_id: None,
            story_id: Some(story_id),
            is_visible: true,
        }
    }

    pub fn target_kind(&self) -> StatisticsTargetKind {
        if self.story_id.is_some() {
            StatisticsTargetKind::Story
        } else if self.context_msg_id.is_some() {
            StatisticsTargetKind::Message
        } else {
            StatisticsTargetKind::Channel
        }
    }
}

impl Default for InfoStatisticsTagStatistics {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Render for InfoStatisticsTagStatistics {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let label = match self.target_kind() {
            StatisticsTargetKind::Channel => format!("Channel Stats ({})", self.peer_id),
            StatisticsTargetKind::Supergroup => format!("Group Stats ({})", self.peer_id),
            StatisticsTargetKind::Message => format!("Message #{} Stats", self.context_msg_id.unwrap_or(0)),
            StatisticsTargetKind::Story => format!("Story #{} Stats", self.story_id.unwrap_or(0)),
        };

        div()
            .flex()
            .items_center()
            .px(px(8.0))
            .py(px(4.0))
            .rounded_md()
            .bg(rgb(0x1e2c3a))
            .text_xs()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(rgb(0x0088cc))
            .child(label)
    }
}
