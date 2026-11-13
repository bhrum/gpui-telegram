//! Chat Filters Tabs Mode definitions and mode resolvers.
//! Reference: Telegram/SourceFiles/ui/widgets/chat_filters_tabs_mode.h:L13-L33

#![allow(dead_code, unused_imports, unused_variables)]

use serde::{Deserialize, Serialize};

/// Values are serialized in Core::Settings, don't change their numerical values.
/// Reference: `chat_filters_tabs_mode.h:L13-L18`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[repr(i32)]
pub enum ChatsFiltersTabsMode {
    #[default]
    Default = 0,
    TextAndIcons = 1,
    IconsOnly = 2,
    TextOnly = 3,
}

/// Resolves effective mode for horizontal tab layouts.
/// Reference: `chat_filters_tabs_mode.h:L20-L25`
pub fn horizontal_chats_filters_tabs_mode(mode: ChatsFiltersTabsMode) -> ChatsFiltersTabsMode {
    if mode == ChatsFiltersTabsMode::Default {
        ChatsFiltersTabsMode::TextOnly
    } else {
        mode
    }
}

/// Resolves effective mode for vertical tab strip layouts.
/// Reference: `chat_filters_tabs_mode.h:L27-L32`
pub fn vertical_chats_filters_tabs_mode(mode: ChatsFiltersTabsMode) -> ChatsFiltersTabsMode {
    if mode == ChatsFiltersTabsMode::Default {
        ChatsFiltersTabsMode::TextAndIcons
    } else {
        mode
    }
}

/// Target symbol representing chat filter tabs mode configuration and state.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ChatFiltersTabsModeWidgets {
    pub mode: ChatsFiltersTabsMode,
    pub is_vertical: bool,
}

impl ChatFiltersTabsModeWidgets {
    pub fn new(mode: ChatsFiltersTabsMode, is_vertical: bool) -> Self {
        Self { mode, is_vertical }
    }

    pub fn effective_mode(&self) -> ChatsFiltersTabsMode {
        if self.is_vertical {
            vertical_chats_filters_tabs_mode(self.mode)
        } else {
            horizontal_chats_filters_tabs_mode(self.mode)
        }
    }

    pub fn set_mode(&mut self, mode: ChatsFiltersTabsMode) {
        self.mode = mode;
    }

    pub fn set_vertical(&mut self, is_vertical: bool) {
        self.is_vertical = is_vertical;
    }

    pub fn shows_icons(&self) -> bool {
        matches!(
            self.effective_mode(),
            ChatsFiltersTabsMode::TextAndIcons | ChatsFiltersTabsMode::IconsOnly
        )
    }

    pub fn shows_text(&self) -> bool {
        matches!(
            self.effective_mode(),
            ChatsFiltersTabsMode::TextAndIcons | ChatsFiltersTabsMode::TextOnly
        )
    }
}
