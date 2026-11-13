//! Cloud Emoji Sets Manager
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/chat_helpers/emoji_sets_manager.h:L1-L33`
//! - `Telegram/SourceFiles/chat_helpers/emoji_sets_manager.cpp:L1-L570`
//! - `Telegram/SourceFiles/chat_helpers/emoji_sets_manager.cpp:L42-L47` (`kSets` definition)
//!
//! Subsystem: chat_helpers, Feature: TCG-N11

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EmojiSetState {
    Builtin,
    Available,
    Downloading { progress: f32 },
    Installed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EmojiSetDescriptor {
    pub id: i32,
    pub document_id: u64,
    pub size_bytes: usize,
    pub name: String,
    pub state: EmojiSetState,
}

/// Emoji Sets Manager controlling downloaded emoji pack selection and cloud downloads.
/// Target symbol in ledger: `EmojiSetsManager`.
#[derive(Debug, Clone)]
pub struct EmojiSetsManager {
    pub sets: Vec<EmojiSetDescriptor>,
    pub active_set_id: i32,
}

impl Default for EmojiSetsManager {
    fn default() -> Self {
        Self {
            // Mirroring kSets from emoji_sets_manager.cpp:L42-L47
            sets: vec![
                EmojiSetDescriptor {
                    id: 0,
                    document_id: 0,
                    size_bytes: 0,
                    name: "Apple".to_string(),
                    state: EmojiSetState::Builtin,
                },
                EmojiSetDescriptor {
                    id: 1,
                    document_id: 3223,
                    size_bytes: 7_854_643,
                    name: "Android".to_string(),
                    state: EmojiSetState::Available,
                },
                EmojiSetDescriptor {
                    id: 2,
                    document_id: 3224,
                    size_bytes: 6_072_532,
                    name: "Twemoji".to_string(),
                    state: EmojiSetState::Available,
                },
                EmojiSetDescriptor {
                    id: 3,
                    document_id: 3225,
                    size_bytes: 7_805_861,
                    name: "JoyPixels".to_string(),
                    state: EmojiSetState::Available,
                },
            ],
            active_set_id: 0,
        }
    }
}

impl EmojiSetsManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn active_set(&self) -> Option<&EmojiSetDescriptor> {
        self.sets.iter().find(|s| s.id == self.active_set_id)
    }

    pub fn set_active(&mut self, id: i32) -> bool {
        if let Some(set) = self.sets.iter().find(|s| s.id == id) {
            if matches!(set.state, EmojiSetState::Builtin | EmojiSetState::Installed) {
                self.active_set_id = id;
                return true;
            }
        }
        false
    }

    pub fn update_progress(&mut self, id: i32, progress: f32) {
        if let Some(set) = self.sets.iter_mut().find(|s| s.id == id) {
            if progress >= 1.0 {
                set.state = EmojiSetState::Installed;
            } else {
                set.state = EmojiSetState::Downloading { progress };
            }
        }
    }
}
