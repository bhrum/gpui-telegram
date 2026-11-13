//! History View Reactions Manager
//! Faithful upstream C++ extraction:
//! - `history_view_reactions.cpp:L35-L280`
//! - `history_view_reactions.h:L35-L60`

use gpui::*;
use crate::views::stage::reactions::button::{
    sort_reaction_pills, ReactionKey, ReactionPill,
    K_IN_NON_CHOSEN_OPACITY, K_OUT_NON_CHOSEN_OPACITY, K_MAX_RECENT_USERPICS, K_MAX_NICE_PER_ROW,
};

#[derive(Debug, Clone)]
pub struct HistoryViewReactionsViewReactions {
    pub message_id: u64,
    pub pills: Vec<ReactionPill>,
    pub in_bubble: bool,
    pub flipped: bool,
    pub are_tags: bool,
    pub centered: bool,
    pub available_width: f32,
}

impl HistoryViewReactionsViewReactions {
    pub fn new(message_id: u64) -> Self {
        Self {
            message_id,
            pills: Vec::new(),
            in_bubble: true,
            flipped: false,
            are_tags: false,
            centered: false,
            available_width: 480.0,
        }
    }

    pub fn set_reactions(&mut self, reactions: Vec<ReactionPill>) {
        self.pills = sort_reaction_pills(reactions);
    }

    pub fn add_reaction(&mut self, key: ReactionKey, is_my: bool) {
        if let Some(existing) = self.pills.iter_mut().find(|p| p.id == key) {
            existing.count += 1;
            if is_my {
                existing.chosen = true;
            }
        } else {
            let mut pill = ReactionPill::new(key, 1);
            pill.chosen = is_my;
            self.pills.push(pill);
        }
        self.pills = sort_reaction_pills(std::mem::take(&mut self.pills));
    }

    pub fn remove_reaction(&mut self, key: &ReactionKey) {
        if let Some(pos) = self.pills.iter().position(|p| &p.id == key) {
            if self.pills[pos].count <= 1 {
                self.pills.remove(pos);
            } else {
                self.pills[pos].count -= 1;
                self.pills[pos].chosen = false;
            }
        }
        self.pills = sort_reaction_pills(std::mem::take(&mut self.pills));
    }

    pub fn opacity_for_button(&self, chosen: bool) -> f32 {
        if chosen {
            1.0
        } else if self.in_bubble {
            K_IN_NON_CHOSEN_OPACITY
        } else {
            K_OUT_NON_CHOSEN_OPACITY
        }
    }
}

impl Default for HistoryViewReactionsViewReactions {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Render for HistoryViewReactionsViewReactions {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let pills = &self.pills;
        div()
            .flex()
            .flex_wrap()
            .gap(px(4.0))
            .children(pills.iter().map(|pill| {
                let is_chosen = pill.chosen;
                let is_paid = pill.paid;
                let bg_color = if is_paid {
                    rgba(0xFFA5002A)
                } else if is_chosen {
                    rgba(0x2AABEE2A)
                } else {
                    rgba(0x0000000D)
                };

                let border_color = if is_chosen {
                    rgb(0x2AABEE)
                } else if is_paid {
                    rgb(0xFFA500)
                } else {
                    rgba(0x00000015)
                };

                let mut pill_div = div()
                    .flex()
                    .items_center()
                    .h(px(24.0))
                    .px(px(6.0))
                    .rounded(px(12.0))
                    .bg(bg_color)
                    .border_1()
                    .border_color(border_color)
                    .text_size(px(12.0))
                    .child(pill.id.display_label());

                if pill.count > 0 {
                    pill_div = pill_div.child(
                        div()
                            .ml(px(3.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(if is_chosen { rgb(0x2AABEE) } else { rgb(0x444444) })
                            .child(format!("{}", pill.count)),
                    );
                }

                pill_div
            }))
    }
}
