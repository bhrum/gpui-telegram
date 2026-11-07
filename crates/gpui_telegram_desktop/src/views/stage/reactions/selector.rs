//! Reaction Selector Flyout Popover & Custom Emoji Categories
//! Faithful upstream C++ extraction:
//! - `history_view_reactions_selector.cpp:L45-L160`
//! - `history_view_reactions_tabs.cpp:L20-L80`

use gpui::*;
use super::button::ReactionKey;

pub const K_EXPAND_DURATION_MS: u64 = 300;
pub const K_SCALE_DURATION_MS: u64 = 120;
pub const K_FULL_DURATION_MS: u64 = K_EXPAND_DURATION_MS + K_SCALE_DURATION_MS;
pub const K_EXPAND_DELAY_MS: u64 = 40;
pub const K_ACCEPT_CLICKS_AFTER_MS: u64 = 300;
pub const K_DEFAULT_COLUMNS: usize = 8;
pub const K_MIN_NON_TRANSPARENT_COLUMNS: usize = 7;
pub const K_FRAMES_COUNT: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectorState {
    #[default]
    Hidden,
    Appearing,
    Showing,
    Collapsed,
    Expanded,
    Disappearing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReactionCategory {
    All,
    Top,
    Recent,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct ReactionOption {
    pub key: ReactionKey,
    pub is_premium: bool,
    pub category: ReactionCategory,
}

#[derive(Debug, Clone)]
pub struct ReactionsSelector {
    pub state: SelectorState,
    pub active_category: ReactionCategory,
    pub reactions: Vec<ReactionOption>,
    pub search_query: String,
    pub is_expanded: bool,
    pub columns: usize,
    pub progress: f32,
    pub opacity: f32,
}

impl ReactionsSelector {
    pub fn new() -> Self {
        let default_reactions = vec![
            ReactionOption { key: ReactionKey::Emoji("👍".into()), is_premium: false, category: ReactionCategory::Top },
            ReactionOption { key: ReactionKey::Emoji("❤️".into()), is_premium: false, category: ReactionCategory::Top },
            ReactionOption { key: ReactionKey::Emoji("🔥".into()), is_premium: false, category: ReactionCategory::Top },
            ReactionOption { key: ReactionKey::Emoji("🎉".into()), is_premium: false, category: ReactionCategory::Top },
            ReactionOption { key: ReactionKey::Emoji("👏".into()), is_premium: false, category: ReactionCategory::Top },
            ReactionOption { key: ReactionKey::Emoji("😁".into()), is_premium: false, category: ReactionCategory::Top },
            ReactionOption { key: ReactionKey::Emoji("🤔".into()), is_premium: false, category: ReactionCategory::Top },
            ReactionOption { key: ReactionKey::Emoji("🤯".into()), is_premium: false, category: ReactionCategory::Top },
            ReactionOption { key: ReactionKey::Emoji("😱".into()), is_premium: false, category: ReactionCategory::Top },
            ReactionOption { key: ReactionKey::Emoji("😢".into()), is_premium: false, category: ReactionCategory::Top },
            ReactionOption { key: ReactionKey::Paid, is_premium: false, category: ReactionCategory::Top },
        ];

        Self {
            state: SelectorState::Showing,
            active_category: ReactionCategory::All,
            reactions: default_reactions,
            search_query: String::new(),
            is_expanded: false,
            columns: K_DEFAULT_COLUMNS,
            progress: 1.0,
            opacity: 1.0,
        }
    }

    pub fn set_expanded(&mut self, expanded: bool) {
        self.is_expanded = expanded;
        self.state = if expanded {
            SelectorState::Expanded
        } else {
            SelectorState::Collapsed
        };
    }

    pub fn update_show_state(&mut self, progress: f32, opacity: f32, appearing: bool) {
        self.progress = progress.clamp(0.0, 1.0);
        self.opacity = opacity.clamp(0.0, 1.0);
        self.state = if appearing {
            SelectorState::Appearing
        } else if self.progress >= 1.0 {
            if self.is_expanded { SelectorState::Expanded } else { SelectorState::Showing }
        } else if self.progress <= 0.0 {
            SelectorState::Hidden
        } else {
            SelectorState::Disappearing
        };
    }

    pub fn filtered_reactions(&self) -> Vec<&ReactionOption> {
        self.reactions.iter().filter(|r| {
            if !self.search_query.is_empty() {
                r.key.display_label().to_lowercase().contains(&self.search_query.to_lowercase())
            } else {
                match &self.active_category {
                    ReactionCategory::All => true,
                    cat => &r.category == cat,
                }
            }
        }).collect()
    }
}

impl Default for ReactionsSelector {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for ReactionsSelector {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let items = self.filtered_reactions();

        div()
            .flex()
            .flex_col()
            .p(px(8.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(16.0))
            .shadow_lg()
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .w(px(280.0))
            // Category header tabs
            .child(
                div()
                    .flex()
                    .gap(px(8.0))
                    .pb(px(6.0))
                    .border_b_1()
                    .border_color(rgb(0xF0F0F0))
                    .text_size(px(12.0))
                    .text_color(rgb(0x777777))
                    .child(
                        div()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x2AABEE))
                            .child("Top")
                    )
                    .child(div().child("Recent"))
                    .child(div().child("Custom"))
            )
            // Reaction grid
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(px(6.0))
                    .pt(px(6.0))
                    .children(items.into_iter().map(|item| {
                        let label = item.key.display_label();
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(28.0))
                            .h(px(28.0))
                            .rounded(px(6.0))
                            .hover(|s| s.bg(rgba(0x00000014)))
                            .child(label)
                    }))
            )
    }
}
