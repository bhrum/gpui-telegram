//! Pinned dialogs sublist manager with fixed ordering.
//!
//! Upstream:
//! - `Telegram/SourceFiles/dialogs/dialogs_pinned_list.h:L1-L64`
//! - `Telegram/SourceFiles/dialogs/dialogs_pinned_list.cpp:L1-L179`
//!
//! Subsystem: dialogs, Feature: TCG-N03
//!
//! Key Behaviors:
//! - 1-indexed pin positions (`cachePinnedIndex(filterId, position + 1)`)
//! - Rotating newly pinned chat to index 0 (`std::rotate` L60-64)
//! - Capacity limiting on pin addition (`applyLimit` L79-85)
//! - Pairwise chat reordering (`reorder` L167-176)

pub use gpui_telegram_client::dialogs::pinned_list::DialogsPinnedList as ClientDialogsPinnedList;
use gpui_telegram_client::dialogs::key::DialogKey;
use serde::{Deserialize, Serialize};

/// Pinned dialogs sublist manager for sidebar views.
/// Upstream: `Dialogs::PinnedList` (`dialogs_pinned_list.h:L22-61`)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialogsPinnedList {
    pub filter_id: i32,
    pub limit: usize,
    pub data: Vec<DialogKey>,
}

impl DialogsPinnedList {
    pub const DEFAULT_MAX_PINNED: usize = 5;
    pub const PREMIUM_MAX_PINNED: usize = 10;

    pub fn new(filter_id: i32, limit: usize) -> Self {
        Self {
            filter_id,
            limit: limit.max(1),
            data: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn contains(&self, key: &DialogKey) -> bool {
        self.data.contains(key)
    }

    pub fn pin(&mut self, key: DialogKey) -> bool {
        if self.data.contains(&key) {
            return false;
        }
        if self.data.len() >= self.limit {
            self.data.pop();
        }
        self.data.insert(0, key);
        true
    }

    pub fn unpin(&mut self, key: &DialogKey) {
        self.data.retain(|k| k != key);
    }

    pub fn reorder(&mut self, key1: &DialogKey, key2: &DialogKey) -> bool {
        let idx1 = self.data.iter().position(|k| k == key1);
        let idx2 = self.data.iter().position(|k| k == key2);
        if let (Some(i1), Some(i2)) = (idx1, idx2) {
            if i1 != i2 {
                self.data.swap(i1, i2);
                return true;
            }
        }
        false
    }
}
