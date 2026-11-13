//! Main chat list container coordinating pinned chats and chronological entries.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/dialogs_main_list.h:L1-88`
//! Subsystem: dialogs, Feature: TCG-N03

use super::dialogs_indexed_list::DialogsIndexedList;

#[derive(Debug, Clone, Default)]
pub struct DialogsMainList {
    pub indexed_list: DialogsIndexedList,
    pub filter_id: u32,
    pub is_frozen: bool,
}

impl DialogsMainList {
    pub fn new(filter_id: u32) -> Self {
        Self {
            indexed_list: DialogsIndexedList::new(),
            filter_id,
            is_frozen: false,
        }
    }
}
