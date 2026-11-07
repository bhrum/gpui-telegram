//! Upstream:
//! - Telegram/SourceFiles/dialogs/dialogs_key.h:L1-L170
//! - Telegram/SourceFiles/dialogs/dialogs_key.cpp:L1-L116
//!
//! Upstream Classes & Responsibilities:
//! - `Dialogs::Key` (`dialogs_key.h:L30-65`): Tagged union representing a History, Folder, Topic, Thread, or SavedSublist.
//! - `Dialogs::RowDescriptor` (`dialogs_key.h:L67-106`): Equality comparator matching both key and full message ID.
//! - `Dialogs::SearchState` (`dialogs_key.h:L143-167`, `dialogs_key.cpp:L89-114`): Search scope with `defaultTabForMe()` and `filterChatsList()`.

pub use gpui_telegram_client::dialogs::key::{
    fix_hashtag_search_query, is_hash_or_cashtag_search_query, ChatSearchPeerTabType,
    ChatSearchTab, ChatTypeFilter, DialogKey, DialogSearchState, EntrySection, EntryState,
    FixedHashtagSearchQuery, HashOrCashtag, PostsSearchIntroState, RestoreWindowsChoice,
    RowDescriptor, SearchEmptyIcon,
};
use serde::{Deserialize, Serialize};

/// Dialog Key component for the GPUI sidebar view layer.
/// Upstream: `Dialogs::Key` (`dialogs_key.h:L30-65`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DialogsKey {
    pub inner: DialogKey,
}

impl DialogsKey {
    pub fn history(peer_id: i64) -> Self {
        Self {
            inner: DialogKey::History(peer_id),
        }
    }

    pub fn folder(folder_id: i32) -> Self {
        Self {
            inner: DialogKey::Folder(folder_id),
        }
    }

    pub fn topic(peer_id: i64, topic_root_id: i32) -> Self {
        Self {
            inner: DialogKey::ForumTopic {
                peer_id,
                topic_root_id,
            },
        }
    }

    pub fn thread(peer_id: i64, thread_root_id: i32) -> Self {
        Self {
            inner: DialogKey::Thread {
                peer_id,
                thread_root_id,
            },
        }
    }

    pub fn saved_sublist(peer_id: i64) -> Self {
        Self {
            inner: DialogKey::SavedSublist(peer_id),
        }
    }

    pub fn peer_id(&self) -> Option<i64> {
        self.inner.peer_id()
    }

    pub fn folder_id(&self) -> Option<i32> {
        self.inner.folder_id()
    }

    pub fn topic_id(&self) -> Option<i32> {
        self.inner.topic_id()
    }
}

impl Default for DialogsKey {
    fn default() -> Self {
        Self::history(0)
    }
}

impl From<DialogKey> for DialogsKey {
    fn from(inner: DialogKey) -> Self {
        Self { inner }
    }
}

impl From<DialogsKey> for DialogKey {
    fn from(key: DialogsKey) -> Self {
        key.inner
    }
}
