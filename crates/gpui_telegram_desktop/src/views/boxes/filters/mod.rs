//! Telegram Desktop chat filter dialog boxes submodule.
//! Upstream reference: `Telegram/SourceFiles/boxes/filters/`
pub mod edit_filter_box;
pub use edit_filter_box::{EditFilterBox, EditFilterBoxFilters};

pub mod edit_filter_chats_list;
pub use edit_filter_chats_list::{EditFilterChatsList, EditFilterChatsListFilters};

pub mod edit_filter_chats_preview;
pub use edit_filter_chats_preview::{EditFilterChatsPreview, EditFilterChatsPreviewFilters};

pub mod edit_filter_links;
pub use edit_filter_links::{EditFilterLinks, EditFilterLinksFilters};
