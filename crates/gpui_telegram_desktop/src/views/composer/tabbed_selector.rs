//! Tabbed Selector (Emoji, Stickers, GIFs, Masks) Controller
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/chat_helpers/tabbed_selector.h:L1-L488`
//! - `Telegram/SourceFiles/chat_helpers/tabbed_selector.cpp:L1-L1685`
//! - `Telegram/SourceFiles/chat_helpers/tabbed_selector.h:L55-L60` (`SelectorTab`)
//! - `Telegram/SourceFiles/chat_helpers/tabbed_selector.h:L83-L98` (`TabbedSelectorMode`)
//!
//! Subsystem: chat_helpers, Feature: TCG-N11

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectorTab {
    Emoji,
    Stickers,
    Gifs,
    Masks,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabbedSelectorMode {
    Full,
    EmojiOnly,
    CustomEmojiOnly,
    CustomEmojiAndGifs,
    StickersOnly,
    StickersAndGifs,
    MediaEditor,
    EmojiStatus,
    ChannelStatus,
    BackgroundEmoji,
    FullReactions,
    RecentReactions,
    PeerTitle,
    ChatIntro,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlideDirection {
    LeftToRight,
    RightToLeft,
}

/// Tabbed Selector controlling active tab views (Emoji, Stickers, GIFs), search states,
/// slide animations, and footer synchronization.
/// Target symbol in ledger: `TabbedSelector`.
#[derive(Debug, Clone)]
pub struct TabbedSelector {
    pub current_tab: SelectorTab,
    pub mode: TabbedSelectorMode,
    pub is_searching: bool,
    pub search_query: String,
    pub last_slide_direction: Option<SlideDirection>,
    pub footer_scroll_top: f32,
    pub round_radius: f32,
}

impl Default for TabbedSelector {
    fn default() -> Self {
        Self {
            current_tab: SelectorTab::Emoji,
            mode: TabbedSelectorMode::Full,
            is_searching: false,
            search_query: String::new(),
            last_slide_direction: None,
            footer_scroll_top: 0.0,
            round_radius: 8.0,
        }
    }
}

impl TabbedSelector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_mode(mut self, mode: TabbedSelectorMode) -> Self {
        self.mode = mode;
        self
    }

    /// Switch active tab with directional slide animation.
    /// Directly mirrors `tabbed_selector.cpp:L277-L299`.
    pub fn switch_tab(&mut self, new_tab: SelectorTab) -> Option<SlideDirection> {
        if self.current_tab == new_tab {
            return None;
        }

        let direction = if (new_tab as usize) > (self.current_tab as usize) {
            SlideDirection::RightToLeft
        } else {
            SlideDirection::LeftToRight
        };

        self.last_slide_direction = Some(direction);
        self.current_tab = new_tab;
        self.search_query.clear();
        self.is_searching = false;

        Some(direction)
    }

    pub fn is_tab_supported(&self, tab: SelectorTab) -> bool {
        match (self.mode, tab) {
            (TabbedSelectorMode::EmojiOnly | TabbedSelectorMode::CustomEmojiOnly, SelectorTab::Emoji) => true,
            (TabbedSelectorMode::EmojiOnly | TabbedSelectorMode::CustomEmojiOnly, _) => false,
            (TabbedSelectorMode::StickersOnly, SelectorTab::Stickers) => true,
            (TabbedSelectorMode::StickersOnly, _) => false,
            (TabbedSelectorMode::StickersAndGifs, SelectorTab::Stickers | SelectorTab::Gifs) => true,
            (TabbedSelectorMode::StickersAndGifs, _) => false,
            _ => true,
        }
    }

    pub fn set_search_query(&mut self, query: &str) {
        self.search_query = query.trim().to_string();
        self.is_searching = !self.search_query.is_empty();
    }
}
