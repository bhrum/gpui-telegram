//! Module definitions for `crates/gpui_telegram_desktop/src/views/drawer/profile/tabs`

pub mod adapters;
pub mod info_profile_tab_content;
pub mod info_profile_tab_skeleton;
pub mod info_profile_tab_top_bar_bindings;
pub mod info_profile_tabs_host;
pub mod info_profile_tabs_strip;

pub use info_profile_tab_content::InfoProfileTabContentProfileTabs;
pub use info_profile_tab_skeleton::InfoProfileTabSkeletonProfileTabs;
pub use info_profile_tab_top_bar_bindings::InfoProfileTabTopBarBindingsProfileTabs;
pub use info_profile_tabs_host::InfoProfileTabsHostProfileTabs;
pub use info_profile_tabs_strip::InfoProfileTabsStripProfileTabs;
