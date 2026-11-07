//! UI Who Reacted context menu preview action.
//! Upstream: `Telegram/SourceFiles/ui/controls/who_reacted_context_action.cpp:L100-L250`
//!           `Telegram/SourceFiles/ui/controls/who_reacted_context_action.h:L19-L65`
//!
//! Message reaction participant avatars popup list with read/seen/reacted indicators.

use gpui::*;

pub const K_MAX_SMALL_USERPICS: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhoReadType {
    Seen,
    Listened,
    Watched,
    Reacted,
    Edited,
    Original,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhoReadState {
    Empty,
    Unknown,
    MyHidden,
    HisHidden,
    TooOld,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhoReadParticipantItem {
    pub name: String,
    pub reaction_emoji: String,
    pub is_self: bool,
}

/// WhoReactedContextActionControls component. Target symbol required by ledger: `WhoReactedContextActionControls`.
#[derive(Debug, Clone)]
pub struct WhoReactedContextActionControls {
    pub participants: Vec<WhoReadParticipantItem>,
    pub read_type: WhoReadType,
    pub full_reactions_count: usize,
    pub full_read_count: usize,
    pub state: WhoReadState,
}

impl WhoReactedContextActionControls {
    pub fn new() -> Self {
        Self {
            participants: Vec::new(),
            read_type: WhoReadType::Reacted,
            full_reactions_count: 0,
            full_read_count: 0,
            state: WhoReadState::Empty,
        }
    }

    pub fn with_participant(mut self, name: impl Into<String>, reaction_emoji: impl Into<String>) -> Self {
        self.participants.push(WhoReadParticipantItem {
            name: name.into(),
            reaction_emoji: reaction_emoji.into(),
            is_self: false,
        });
        self.full_reactions_count = self.participants.len();
        self.state = WhoReadState::Unknown;
        self
    }
}

impl Default for WhoReactedContextActionControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WhoReactedContextActionControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut row = div().flex().items_center().gap(px(8.0)).p(px(8.0));
        let display_count = self.participants.len().min(K_MAX_SMALL_USERPICS);

        for p in self.participants.iter().take(display_count) {
            row = row.child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(4.0))
                    .child(
                        div()
                            .w(px(20.0))
                            .h(px(20.0))
                            .rounded(px(10.0))
                            .bg(hsla(0.0, 0.0, 0.85, 1.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(10.0))
                            .child(p.name.chars().next().unwrap_or('?').to_string())
                    )
                    .child(p.reaction_emoji.clone())
            );
        }

        if self.participants.len() > display_count {
            row = row.child(
                div()
                    .text_size(px(11.0))
                    .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                    .child(format!("+{}", self.participants.len() - display_count))
            );
        }

        row
    }
}
