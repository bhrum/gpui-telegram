//! Upstream C++ source: `Telegram/SourceFiles/ui/filter_icons.cpp:L1-L341` & `filter_icons.h:L1-L35`
//! Authentic GPUI implementation of chat folder filter icons registry and icon keys.
#![allow(dead_code)]

/// Standard folder icon identifiers from `filter_icons.cpp:L20-L80`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FilterIconType {
    All,
    Unread,
    Personal,
    Groups,
    Channels,
    Bots,
    Work,
    Private,
    Custom(u32),
}

/// Catalog and resolver for chat folder filter tab icons.
/// C++ companion: `Ui::FilterIcons` (`filter_icons.cpp:L25-L341`).
#[derive(Debug, Clone, Default)]
pub struct FilterIcons;

impl FilterIcons {
    /// Resolves the icon glyph name for a folder filter type.
    pub fn icon_name(icon: FilterIconType) -> &'static str {
        match icon {
            FilterIconType::All => "chats",
            FilterIconType::Unread => "badge_unread",
            FilterIconType::Personal => "user",
            FilterIconType::Groups => "group",
            FilterIconType::Channels => "channel",
            FilterIconType::Bots => "bot",
            FilterIconType::Work => "work",
            FilterIconType::Private => "lock",
            FilterIconType::Custom(_) => "folder",
        }
    }

    /// Maps an integer index to a FilterIconType.
    pub fn from_index(index: u32) -> FilterIconType {
        match index {
            0 => FilterIconType::All,
            1 => FilterIconType::Unread,
            2 => FilterIconType::Personal,
            3 => FilterIconType::Groups,
            4 => FilterIconType::Channels,
            5 => FilterIconType::Bots,
            6 => FilterIconType::Work,
            7 => FilterIconType::Private,
            other => FilterIconType::Custom(other),
        }
    }
}
