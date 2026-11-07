//! StickersListWidget: Telegram sticker grid picker view component.
//!
//! Upstream reference:
//! - `Telegram/SourceFiles/chat_helpers/stickers_list_widget.h:L70-L146`
//! - `Telegram/SourceFiles/chat_helpers/stickers_list_widget.cpp:L75-L160`
#![allow(dead_code)]

use gpui::*;

/// Search request debounce in milliseconds (upstream: `kSearchRequestDelay = 400`).
pub const K_SEARCH_REQUEST_DELAY_MS: u64 = 400;

/// Default limit of recent stickers to show (upstream: `kRecentDisplayLimit = 20`).
pub const K_RECENT_DISPLAY_LIMIT: usize = 20;

/// Official pack preload count (upstream: `kPreloadOfficialPages = 4`).
pub const K_PRELOAD_OFFICIAL_PAGES: usize = 4;

/// Official pack page load limit (upstream: `kOfficialLoadLimit = 40`).
pub const K_OFFICIAL_LOAD_LIMIT: usize = 40;

/// Minimum repaint interval for sticker animation (upstream: `kMinRepaintDelay = 33ms`).
pub const K_MIN_REPAINT_DELAY_MS: u64 = 33;

/// Stickers list mode (upstream: `stickers_list_widget.h:L70-L76`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StickersListMode {
    #[default]
    Full,
    Masks,
    UserpicBuilder,
    ChatIntro,
    MessageEffects,
}

/// Sticker item description (upstream: `stickers_list_widget.cpp:L103-L113`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StickerItem {
    pub id: u64,
    pub document_id: u64,
    pub alt_emoticon: String,
    pub is_premium: bool,
    pub is_lottie: bool,
    pub is_webm: bool,
    pub pack_id: u64,
    pub thumbnail_url: Option<String>,
}

/// Sticker pack representation (upstream: `stickers_list_widget.cpp:L115-L144`).
#[derive(Debug, Clone)]
pub struct StickerSet {
    pub id: u64,
    pub title: String,
    pub short_name: String,
    pub count: usize,
    pub installed: bool,
    pub archived: bool,
    pub stickers: Vec<StickerItem>,
}

impl StickerSet {
    pub fn new(id: u64, title: impl Into<String>, short_name: impl Into<String>) -> Self {
        Self {
            id,
            title: title.into(),
            short_name: short_name.into(),
            count: 0,
            installed: true,
            archived: false,
            stickers: Vec::new(),
        }
    }

    pub fn in_my_list(&self) -> bool {
        self.installed && !self.archived
    }
}

/// StickersListWidget authentic GPUI state.
pub struct StickersListWidget {
    pub mode: StickersListMode,
    pub sets: Vec<StickerSet>,
    pub recent_stickers: Vec<StickerItem>,
    pub favorite_stickers: Vec<StickerItem>,
    pub search_query: String,
    pub selected_set_id: Option<u64>,
    pub active_index: Option<usize>,
    pub is_active: bool,
    pub scroll_top: f32,
    pub updated_at: i64,
}

impl StickersListWidget {
    pub fn new() -> Self {
        Self {
            mode: StickersListMode::Full,
            sets: Vec::new(),
            recent_stickers: Vec::new(),
            favorite_stickers: Vec::new(),
            search_query: String::new(),
            selected_set_id: None,
            active_index: None,
            is_active: false,
            scroll_top: 0.0,
            updated_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn with_mode(mut self, mode: StickersListMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn set_mode(&mut self, mode: StickersListMode) {
        self.mode = mode;
        self.touch();
    }

    pub fn add_set(&mut self, set: StickerSet) {
        self.sets.push(set);
        self.touch();
    }

    pub fn show_sticker_set(&mut self, set_id: u64) {
        self.selected_set_id = Some(set_id);
        self.touch();
    }

    pub fn add_recent(&mut self, sticker: StickerItem) {
        self.recent_stickers.retain(|s| s.id != sticker.id);
        self.recent_stickers.insert(0, sticker);
        if self.recent_stickers.len() > K_RECENT_DISPLAY_LIMIT {
            self.recent_stickers.truncate(K_RECENT_DISPLAY_LIMIT);
        }
        self.touch();
    }

    pub fn set_search_query(&mut self, query: impl Into<String>) {
        self.search_query = query.into();
        self.touch();
    }

    pub fn prepare_stickers(&self, skip_premium: bool) -> Vec<StickerItem> {
        let mut results = Vec::new();
        for set in &self.sets {
            if !set.in_my_list() {
                continue;
            }
            for sticker in &set.stickers {
                if skip_premium && sticker.is_premium {
                    continue;
                }
                if self.search_query.is_empty()
                    || sticker.alt_emoticon.contains(&self.search_query)
                    || set.title.to_lowercase().contains(&self.search_query.to_lowercase())
                {
                    results.push(sticker.clone());
                }
            }
        }
        results
    }

    pub fn active_stickers_count(&self) -> usize {
        self.sets.iter().filter(|s| s.in_my_list()).map(|s| s.stickers.len()).sum()
    }

    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now().timestamp();
    }
}

impl Default for StickersListWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for StickersListWidget {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let active_stickers = self.prepare_stickers(false);
        let count = active_stickers.len();

        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(rgb(0xFFFFFF))
            .p(px(8.0))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .pb(px(6.0))
                    .border_b_1()
                    .border_color(rgb(0xEEEEEE))
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x333333))
                            .child(format!("Stickers ({} items)", count))
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(0x999999))
                            .child(format!("{:?}", self.mode))
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(px(6.0))
                    .pt(px(8.0))
                    .children(active_stickers.into_iter().take(48).map(|s| {
                        div()
                            .w(px(52.0))
                            .h(px(52.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(px(6.0))
                            .bg(rgb(0xF5F5F5))
                            .hover(|st| st.bg(rgb(0xEAEAEA)))
                            .child(
                                div()
                                    .text_size(px(24.0))
                                    .child(if s.alt_emoticon.is_empty() {
                                        "🎨".to_string()
                                    } else {
                                        s.alt_emoticon
                                    })
                            )
                    }))
            )
    }
}
