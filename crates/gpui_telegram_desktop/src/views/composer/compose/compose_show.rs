//! Compose Show Interface and Pause Reasons State
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/chat_helpers/compose/compose_show.h:L1-L70`
//! - `Telegram/SourceFiles/chat_helpers/compose/compose_show.cpp:L1-L58`
//!
//! Subsystem: chat_helpers, Feature: TCG-N11

/// Bitmask flags for pausing animated stickers and media in chat history.
/// Directly mirroring `compose_show.h:L32-L39`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PauseReasons(pub u32);

impl PauseReasons {
    pub const NONE: Self = Self(0);
    pub const INLINE_RESULTS: Self = Self(1 << 0);
    pub const TABBED_PANEL: Self = Self(1 << 1);
    pub const LAYER: Self = Self(1 << 2);
    pub const ROUND_PLAYING: Self = Self(1 << 3);
    pub const MEDIA_PREVIEW: Self = Self(1 << 4);

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    pub fn insert(&mut self, other: Self) {
        self.0 |= other.0;
    }

    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.0;
    }
}

/// Chosen sticker or document item for sending from tabbed panel
#[derive(Debug, Clone)]
pub struct ChosenFile {
    pub document_id: u64,
    pub mime_type: String,
    pub is_lottie: bool,
    pub is_webm: bool,
    pub emoji_association: Option<String>,
    pub caption: Option<String>,
}

/// Window resolution hierarchy tracker from `compose_show.cpp:L21-L48`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowResolveTarget {
    ActiveWindow,
    ActivePrimaryWindow,
    WindowForAccount,
    EnsureSeparateWindow,
}

/// Compose Show state controller managing pause reasons and media callbacks.
/// Target Symbol in ledger: `ComposeShowCompose`.
#[derive(Debug, Clone)]
pub struct ComposeShowCompose {
    pub pause_reasons: PauseReasons,
    pub adjust_shadow_left: bool,
    pub last_chosen_sticker: Option<ChosenFile>,
    pub media_preview_active: bool,
}

pub type ComposeShow = ComposeShowCompose;

impl Default for ComposeShowCompose {
    fn default() -> Self {
        Self {
            pause_reasons: PauseReasons::NONE,
            adjust_shadow_left: false,
            last_chosen_sticker: None,
            media_preview_active: false,
        }
    }
}

impl ComposeShowCompose {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pause(&mut self, reason: PauseReasons) {
        self.pause_reasons.insert(reason);
    }

    pub fn unpause(&mut self, reason: PauseReasons) {
        self.pause_reasons.remove(reason);
    }

    pub fn is_paused(&self, reason: PauseReasons) -> bool {
        if reason.is_empty() {
            !self.pause_reasons.is_empty()
        } else {
            self.pause_reasons.contains(reason)
        }
    }

    pub fn process_chosen_sticker(&mut self, file: ChosenFile) {
        self.last_chosen_sticker = Some(file);
    }

    pub fn set_media_preview(&mut self, active: bool) {
        self.media_preview_active = active;
        if active {
            self.pause(PauseReasons::MEDIA_PREVIEW);
        } else {
            self.unpause(PauseReasons::MEDIA_PREVIEW);
        }
    }
}
