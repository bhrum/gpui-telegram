//! Module definitions for `crates/gpui_telegram_desktop/src/views/drawer/profile/tabs/adapters`

pub mod info_profile_tab_chats;
pub mod info_profile_tab_media;
pub mod info_profile_tab_members;
pub mod info_profile_tab_peer_lists;
pub mod info_profile_tab_polls;
pub mod info_profile_tab_saved;
pub mod info_profile_tab_stories;
pub mod info_profile_tab_sub_controller;

pub use info_profile_tab_chats::InfoProfileTabChatsProfileTabsAdapters;
pub use info_profile_tab_media::InfoProfileTabMediaProfileTabsAdapters;
pub use info_profile_tab_members::InfoProfileTabMembersProfileTabsAdapters;
pub use info_profile_tab_peer_lists::InfoProfileTabPeerListsProfileTabsAdapters;
pub use info_profile_tab_polls::InfoProfileTabPollsProfileTabsAdapters;
pub use info_profile_tab_saved::InfoProfileTabSavedProfileTabsAdapters;
pub use info_profile_tab_stories::InfoProfileTabStoriesProfileTabsAdapters;
pub use info_profile_tab_sub_controller::InfoProfileTabSubControllerProfileTabsAdapters;
