//! Fullscreen & Inline Animated Emoji Interactions
//! Faithful upstream C++ extraction:
//! - `history_view_emoji_interactions.cpp:L30-L75`
//! - `history_view_emoji_interactions.h:L40-L70`

use gpui::*;

pub const K_PREMIUM_SHIFT: f32 = 21.0 / 240.0;
pub const K_MAX_PLAYS: usize = 5;
pub const K_MAX_PLAYS_WITH_SMALL_DELAY: usize = 3;
pub const K_SMALL_DELAY_MS: u64 = 200;
pub const K_DROP_DELAYED_AFTER_DELAY_MS: u64 = 2000;

#[derive(Debug, Clone)]
pub struct EmojiPlayState {
    pub emoji: String,
    pub plays_count: usize,
    pub is_active: bool,
    pub shift_x: f32,
    pub shift_y: f32,
}

#[derive(Debug, Clone)]
pub struct HistoryViewEmojiInteractionsView {
    pub current_interaction: Option<EmojiPlayState>,
    pub can_play: bool,
}

impl HistoryViewEmojiInteractionsView {
    pub fn new() -> Self {
        Self {
            current_interaction: None,
            can_play: true,
        }
    }

    pub fn play_emoji(&mut self, emoji: String) -> bool {
        if !self.can_play {
            return false;
        }

        let mut count = 1;
        if let Some(existing) = &mut self.current_interaction {
            if existing.emoji == emoji && existing.plays_count < K_MAX_PLAYS {
                existing.plays_count += 1;
                count = existing.plays_count;
            } else if existing.plays_count >= K_MAX_PLAYS {
                return false;
            }
        }

        self.current_interaction = Some(EmojiPlayState {
            emoji,
            plays_count: count,
            is_active: true,
            shift_x: 0.0,
            shift_y: 0.0,
        });
        true
    }
}

impl Default for HistoryViewEmojiInteractionsView {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewEmojiInteractionsView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(interaction) = &self.current_interaction {
            let label = &interaction.emoji;
            div()
                .absolute()
                .inset_0()
                .flex()
                .items_center()
                .justify_center()
                .text_size(px(72.0))
                .child(label.clone())
                .into_any_element()
        } else {
            div().into_any_element()
        }
    }
}
