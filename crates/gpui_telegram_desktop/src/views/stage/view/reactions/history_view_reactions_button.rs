//! History View Reactions Button Overlay
//! Faithful upstream C++ extraction:
//! - `history_view_reactions_button.cpp:L30-L330`
//! - `history_view_reactions_button.h:L40-L130`

use gpui::*;
use crate::views::stage::reactions::button::{
    scale_for_state, opacity_for_scale, ButtonParameters, ExpandDirection, ReactionButtonState,
    K_ACTIVATE_DURATION_MS, K_COLLAPSE_DURATION_MS, K_EXPAND_DURATION_MS, K_TOGGLE_DURATION_MS,
    REACTION_CORNER_SIZE, REACTION_CORNER_SKIP,
};

#[derive(Debug, Clone)]
pub struct HistoryViewReactionsButtonViewReactions {
    pub state: ReactionButtonState,
    pub parameters: ButtonParameters,
    pub expand_direction: ExpandDirection,
    pub current_scale: f32,
    pub current_opacity: f32,
    pub expanded_height: f32,
    pub scroll: f32,
    pub is_hidden: bool,
}

impl HistoryViewReactionsButtonViewReactions {
    pub fn new() -> Self {
        let initial_state = ReactionButtonState::Shown;
        let scale = scale_for_state(initial_state);
        let opacity = opacity_for_scale(scale);

        Self {
            state: initial_state,
            parameters: ButtonParameters::default(),
            expand_direction: ExpandDirection::Up,
            current_scale: scale,
            current_opacity: opacity,
            expanded_height: REACTION_CORNER_SIZE,
            scroll: 0.0,
            is_hidden: false,
        }
    }

    pub fn apply_parameters(&mut self, parameters: ButtonParameters) {
        let count = parameters.reactions_count;
        let max_added = if count > 1 {
            (count as f32 - 1.0) * (REACTION_CORNER_SIZE + REACTION_CORNER_SKIP)
        } else {
            0.0
        };

        self.expanded_height = REACTION_CORNER_SIZE + max_added;
        let up = (parameters.center.y - px(max_added) >= parameters.visible_top)
            || (parameters.center.y + px(REACTION_CORNER_SIZE + max_added) > parameters.visible_bottom);
        self.expand_direction = if up { ExpandDirection::Up } else { ExpandDirection::Down };

        let state = if parameters.outside {
            ReactionButtonState::Hidden
        } else if parameters.cursor_left {
            ReactionButtonState::Active
        } else {
            ReactionButtonState::Shown
        };
        self.apply_state(state);
        self.parameters = parameters;
    }

    pub fn apply_state(&mut self, state: ReactionButtonState) {
        self.state = state;
        self.current_scale = scale_for_state(state);
        self.current_opacity = opacity_for_scale(self.current_scale);
        self.is_hidden = state == ReactionButtonState::Hidden;
    }

    pub fn expand_up(&self) -> bool {
        self.expand_direction == ExpandDirection::Up
    }
}

impl Default for HistoryViewReactionsButtonViewReactions {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewReactionsButtonViewReactions {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_hidden {
            return div().into_any_element();
        }

        let bg = match self.state {
            ReactionButtonState::Active | ReactionButtonState::Inside => rgb(0x2AABEE),
            _ => rgb(0xFFFFFF),
        };

        div()
            .flex()
            .items_center()
            .justify_center()
            .w(px(REACTION_CORNER_SIZE))
            .h(px(REACTION_CORNER_SIZE))
            .rounded_full()
            .bg(bg)
            .shadow_md()
            .text_size(px(14.0))
            .child("👍")
            .into_any_element()
    }
}
