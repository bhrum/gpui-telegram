//! Reaction Button and Inline Reaction Pill List
//! Faithful upstream C++ extraction:
//! - `history_view_reactions_button.cpp:L30-L330`
//! - `history_view_reactions.cpp:L35-L280`

use gpui::*;

pub const K_TOGGLE_DURATION_MS: u64 = 120;
pub const K_ACTIVATE_DURATION_MS: u64 = 150;
pub const K_EXPAND_DURATION_MS: u64 = 300;
pub const K_COLLAPSE_DURATION_MS: u64 = 250;
pub const K_BUTTON_SHOW_DELAY_MS: u64 = 300;
pub const K_BUTTON_EXPAND_DELAY_MS: u64 = 25;
pub const K_BUTTON_HIDE_DELAY_MS: u64 = 300;
pub const K_BUTTON_EXPANDED_HIDE_DELAY_MS: u64 = 0;
pub const K_MAX_REACTIONS_SCROLL_AT_ONCE: i32 = 2;
pub const K_REFRESH_LIST_DELAY_MS: u64 = 100;

pub const K_IN_NON_CHOSEN_OPACITY: f32 = 0.12;
pub const K_OUT_NON_CHOSEN_OPACITY: f32 = 0.18;
pub const K_MAX_RECENT_USERPICS: usize = 3;
pub const K_MAX_NICE_PER_ROW: usize = 5;

pub const REACTION_CORNER_SIZE: f32 = 32.0;
pub const REACTION_CORNER_SKIP: f32 = 4.0;
pub const REACTION_CORNER_ADDED_HEIGHT_MAX: f32 = 200.0;
pub const REACTION_TAG_ARROW: f32 = 5.0;
pub const REACTION_TAG_RIGHT_RADIUS: f32 = 4.0;
pub const REACTION_TAG_LEFT_RADIUS: f32 = 14.0;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReactionKey {
    Emoji(String),
    Custom(u64),
    Paid,
}

impl ReactionKey {
    pub fn is_paid(&self) -> bool {
        matches!(self, ReactionKey::Paid)
    }

    pub fn display_label(&self) -> String {
        match self {
            ReactionKey::Emoji(e) => e.clone(),
            ReactionKey::Custom(id) => format!("custom:{id}"),
            ReactionKey::Paid => "⭐".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReactionButtonState {
    #[default]
    Hidden,
    Shown,
    Active,
    Inside,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExpandDirection {
    #[default]
    Up,
    Down,
}

/// Upstream `Button::ScaleForState` (`history_view_reactions_button.cpp:L290-L298`)
pub fn scale_for_state(state: ReactionButtonState) -> f32 {
    match state {
        ReactionButtonState::Hidden => 1.0 / 3.0,
        ReactionButtonState::Shown => 2.0 / 3.0,
        ReactionButtonState::Active | ReactionButtonState::Inside => 1.0,
    }
}

/// Upstream `Button::OpacityForScale` (`history_view_reactions_button.cpp:L300-L305`)
pub fn opacity_for_scale(scale: f32) -> f32 {
    let min_scale = 1.0 / 3.0;
    let max_scale = 2.0 / 3.0;
    ((scale - min_scale) / (max_scale - min_scale)).clamp(0.0, 1.0)
}

#[derive(Debug, Clone)]
pub struct ButtonParameters {
    pub center: Point<Pixels>,
    pub pointer: Point<Pixels>,
    pub global_pointer: Point<Pixels>,
    pub reactions_count: usize,
    pub visible_top: Pixels,
    pub visible_bottom: Pixels,
    pub outside: bool,
    pub cursor_left: bool,
}

impl Default for ButtonParameters {
    fn default() -> Self {
        Self {
            center: Point::default(),
            pointer: Point::default(),
            global_pointer: Point::default(),
            reactions_count: 1,
            visible_top: px(0.0),
            visible_bottom: px(1000.0),
            outside: false,
            cursor_left: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReactionPill {
    pub id: ReactionKey,
    pub count: i32,
    pub chosen: bool,
    pub paid: bool,
    pub tag: bool,
    pub tag_title: Option<String>,
    pub recent_avatars: Vec<String>,
    pub catalog_index: usize,
}

impl ReactionPill {
    pub fn new(id: ReactionKey, count: i32) -> Self {
        let is_paid = id.is_paid();
        Self {
            id,
            count,
            chosen: false,
            paid: is_paid,
            tag: false,
            tag_title: None,
            recent_avatars: Vec::new(),
            catalog_index: 0,
        }
    }
}

/// Upstream `InlineList` layout & sorting logic (`history_view_reactions.cpp:L177-L232`)
pub fn sort_reaction_pills(mut pills: Vec<ReactionPill>) -> Vec<ReactionPill> {
    pills.sort_by(|a, b| {
        // Paid reaction always sorts first
        if a.paid && !b.paid {
            return std::cmp::Ordering::Less;
        } else if !a.paid && b.paid {
            return std::cmp::Ordering::Greater;
        }

        // Count descending (subtracting current user's vote)
        let acount = a.count - if a.chosen { 1 } else { 0 };
        let bcount = b.count - if b.chosen { 1 } else { 0 };
        if acount != bcount {
            return bcount.cmp(&acount);
        }

        // Tie-breaker: original catalog index
        a.catalog_index.cmp(&b.catalog_index)
    });
    pills
}

#[derive(Debug, Clone)]
pub struct ReactionPillButton {
    pub pill: ReactionPill,
    pub state: ReactionButtonState,
    pub scale: f32,
    pub opacity: f32,
}

impl ReactionPillButton {
    pub fn new(pill: ReactionPill) -> Self {
        let scale = scale_for_state(ReactionButtonState::Shown);
        let opacity = opacity_for_scale(scale);
        Self {
            pill,
            state: ReactionButtonState::Shown,
            scale,
            opacity,
        }
    }

    pub fn set_state(&mut self, state: ReactionButtonState) {
        self.state = state;
        self.scale = scale_for_state(state);
        self.opacity = opacity_for_scale(self.scale);
    }
}

impl Render for ReactionPillButton {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_chosen = self.pill.chosen;
        let is_paid = self.pill.paid;
        
        let bg_color = if is_paid {
            rgba(0xFFA50033)
        } else if is_chosen {
            rgba(0x2AABEE33)
        } else {
            rgba(0x00000014)
        };

        let border_color = if is_chosen {
            rgb(0x2AABEE)
        } else if is_paid {
            rgb(0xFFA500)
        } else {
            rgba(0x00000020)
        };

        let label = self.pill.id.display_label();
        let count_str = if self.pill.count > 0 {
            format!(" {}", self.pill.count)
        } else {
            String::new()
        };

        div()
            .flex()
            .items_center()
            .h(px(26.0))
            .px(px(8.0))
            .rounded(px(13.0))
            .bg(bg_color)
            .border_1()
            .border_color(border_color)
            .text_size(px(12.0))
            .text_color(rgb(0x222222))
            .child(
                div()
                    .flex()
                    .items_center()
                    .child(label)
                    .child(
                        div()
                            .font_weight(FontWeight::BOLD)
                            .text_color(if is_chosen { rgb(0x2AABEE) } else { rgb(0x555555) })
                            .child(count_str)
                    )
            )
    }
}
