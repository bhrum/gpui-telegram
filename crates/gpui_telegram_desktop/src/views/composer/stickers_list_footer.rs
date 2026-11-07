//! StickersListFooter: Tabbed footer bar for sticker pack navigation.
//!
//! Upstream reference:
//! - `Telegram/SourceFiles/chat_helpers/stickers_list_footer.h:L48-L100`
//! - `Telegram/SourceFiles/chat_helpers/stickers_list_footer.cpp:L37-L120`
#![allow(dead_code)]

use gpui::*;

/// Base magic number for virtual emoji pack set IDs (upstream: `kEmojiSectionSetIdBase = 0x77FF'FFFF'FFFF'FFF0ULL`).
pub const K_EMOJI_SECTION_SET_ID_BASE: u64 = 0x77FF_FFFF_FFFF_FFF0;

/// Maximum number of emoji search results returned in footer selector (upstream: `kEmojiSearchLimit = 32`).
pub const K_EMOJI_SEARCH_LIMIT: usize = 32;

/// Section ID calculation for emoji sections (upstream: `stickers_list_footer.cpp:L84-L89`).
pub fn emoji_section_set_id(section_index: u64) -> u64 {
    K_EMOJI_SECTION_SET_ID_BASE + section_index + 1
}

/// Set ID for recent emoji section (upstream: `stickers_list_footer.cpp:L91-L93`).
pub fn recent_emoji_section_set_id() -> u64 {
    emoji_section_set_id(0)
}

/// Set ID for all emoji section (upstream: `stickers_list_footer.cpp:L95-L97`).
pub fn all_emoji_section_set_id() -> u64 {
    K_EMOJI_SECTION_SET_ID_BASE
}

/// Set ID for search emoji section (upstream: `stickers_list_footer.cpp:L99-L103`).
pub fn search_emoji_section_set_id() -> u64 {
    K_EMOJI_SECTION_SET_ID_BASE + 8 + 2
}

/// Convert a set ID back to emoji section index if within valid range (upstream: `stickers_list_footer.cpp:L105-L114`).
pub fn set_id_emoji_section(id: u64) -> Option<u64> {
    let base = recent_emoji_section_set_id();
    if id < base {
        return None;
    }
    let index = id - base;
    if index <= 8 {
        Some(index)
    } else {
        None
    }
}

/// Validate icon animation policy (upstream: `stickers_list_footer.h:L48-L52`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ValidateIconAnimations {
    #[default]
    Full,
    Scroll,
    None,
}

/// Tab icon metadata (upstream: `stickers_list_footer.h:L76-L100`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StickerIconData {
    pub set_id: u64,
    pub title: String,
    pub is_megagroup: bool,
    pub has_lottie: bool,
    pub has_webm: bool,
    pub pixw: i32,
    pub pixh: i32,
}

/// StickersListFooter component state.
pub struct StickersListFooter {
    pub icons: Vec<StickerIconData>,
    pub active_set_id: Option<u64>,
    pub animations_mode: ValidateIconAnimations,
    pub scroll_offset: f32,
    pub updated_at: i64,
}

impl StickersListFooter {
    pub fn new() -> Self {
        Self {
            icons: Vec::new(),
            active_set_id: None,
            animations_mode: ValidateIconAnimations::Full,
            scroll_offset: 0.0,
            updated_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn add_icon(&mut self, icon: StickerIconData) {
        if self.active_set_id.is_none() {
            self.active_set_id = Some(icon.set_id);
        }
        self.icons.push(icon);
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn set_active_set(&mut self, set_id: u64) {
        self.active_set_id = Some(set_id);
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn select_next(&mut self) {
        if self.icons.is_empty() {
            return;
        }
        let current_idx = self
            .active_set_id
            .and_then(|id| self.icons.iter().position(|i| i.set_id == id))
            .unwrap_or(0);
        let next_idx = (current_idx + 1) % self.icons.len();
        self.active_set_id = Some(self.icons[next_idx].set_id);
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn select_prev(&mut self) {
        if self.icons.is_empty() {
            return;
        }
        let current_idx = self
            .active_set_id
            .and_then(|id| self.icons.iter().position(|i| i.set_id == id))
            .unwrap_or(0);
        let prev_idx = if current_idx == 0 {
            self.icons.len() - 1
        } else {
            current_idx - 1
        };
        self.active_set_id = Some(self.icons[prev_idx].set_id);
        self.updated_at = chrono::Utc::now().timestamp();
    }
}

impl Default for StickersListFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for StickersListFooter {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let active_id = self.active_set_id;

        div()
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .h(px(40.0))
            .bg(rgb(0xFAFAFA))
            .border_t_1()
            .border_color(rgb(0xE0E0E0))
            .px(px(8.0))
            .gap(px(4.0))
            .children(self.icons.iter().map(|icon| {
                let is_selected = active_id == Some(icon.set_id);
                div()
                    .w(px(32.0))
                    .h(px(32.0))
                    .rounded(px(4.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .bg(if is_selected {
                        rgb(0x2AABEE)
                    } else {
                        rgb(0xF0F0F0)
                    })
                    .text_color(if is_selected {
                        rgb(0xFFFFFF)
                    } else {
                        rgb(0x555555)
                    })
                    .text_size(px(14.0))
                    .child(icon.title.chars().next().unwrap_or('★').to_string())
            }))
    }
}
