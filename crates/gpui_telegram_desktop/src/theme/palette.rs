use gpui::{rgb, Rgba};

pub use super::palette_schemes::{
    AccentSwatch, TdesktopThemeMode, WallpaperPreset, ACCENT_SWATCHES, WALLPAPER_PRESETS,
};
pub use super::tdesktop_palette::TdesktopThemePalette;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PaletteTokens {
    pub window_bg: Rgba,
    pub window_fg: Rgba,
    pub active: Rgba,
    pub bubble_in_bg: Rgba,
    pub bubble_in_fg: Rgba,
    pub bubble_out_bg: Rgba,
    pub bubble_out_fg: Rgba,
    pub sidebar_bg: Rgba,
    pub sidebar_active: Rgba,
    pub sidebar_hover: Rgba,
    pub unread_badge_bg: Rgba,
    pub unread_badge_fg: Rgba,
    pub unread_badge_muted: Rgba,
}

pub struct ThemePalette;

impl ThemePalette {
    // ------------------------------------------------------------------------
    // Exact Upstream Light (Day Blue) Tokens
    // ------------------------------------------------------------------------
    pub const DAY_BLUE_WINDOW_BG: u32 = 0xFFFFFF;
    pub const DAY_BLUE_WINDOW_FG: u32 = 0x000000;
    pub const DAY_BLUE_ACTIVE: u32 = 0x40A7E3;
    pub const DAY_BLUE_MSG_OUT_BG: u32 = 0xDEF1FD;
    pub const DAY_BLUE_MSG_IN_BG: u32 = 0xFFFFFF;
    pub const DAY_BLUE_SIDEBAR_BG: u32 = 0xF1F1F1;
    pub const DAY_BLUE_SIDEBAR_ACTIVE: u32 = 0x40A7E3;
    pub const DAY_BLUE_SIDEBAR_HOVER: u32 = 0xF4F4F5;
    pub const DAY_BLUE_UNREAD_BG: u32 = 0x40A7E3;
    pub const DAY_BLUE_UNREAD_MUTED: u32 = 0xB0B0B0;

    // Backwards-compatible aliases
    pub const DAY_BLUE_PRIMARY: u32 = Self::DAY_BLUE_ACTIVE;
    pub const DAY_BLUE_BG: u32 = Self::DAY_BLUE_WINDOW_BG;
    pub const DAY_BLUE_SIDEBAR: u32 = Self::DAY_BLUE_SIDEBAR_BG;
    pub const DAY_BLUE_BUBBLE_OUT: u32 = Self::DAY_BLUE_MSG_OUT_BG;
    pub const DAY_BLUE_BUBBLE_IN: u32 = Self::DAY_BLUE_MSG_IN_BG;

    // ------------------------------------------------------------------------
    // Exact Upstream Dark (Night) Tokens
    // ------------------------------------------------------------------------
    pub const NIGHT_WINDOW_BG: u32 = 0x17212B;
    pub const NIGHT_WINDOW_FG: u32 = 0xF5F5F5;
    pub const NIGHT_ACTIVE: u32 = 0x40A7E3;
    pub const NIGHT_ACCENT: u32 = 0x5288C1;
    pub const NIGHT_MSG_OUT_BG: u32 = 0x2B5278;
    pub const NIGHT_MSG_IN_BG: u32 = 0x182533;
    pub const NIGHT_SIDEBAR_BG: u32 = 0x0E1621;
    pub const NIGHT_SIDEBAR_ACTIVE: u32 = 0x2B5278;
    pub const NIGHT_SIDEBAR_HOVER: u32 = 0x17212B;
    pub const NIGHT_UNREAD_BG: u32 = 0x5288C1;
    pub const NIGHT_UNREAD_MUTED: u32 = 0x6B808D;

    // Backwards-compatible aliases
    pub const NIGHT_PRIMARY: u32 = Self::NIGHT_ACTIVE;
    pub const NIGHT_BG: u32 = Self::NIGHT_WINDOW_BG;
    pub const NIGHT_SIDEBAR: u32 = Self::NIGHT_SIDEBAR_BG;
    pub const NIGHT_BUBBLE_OUT: u32 = Self::NIGHT_MSG_OUT_BG;
    pub const NIGHT_BUBBLE_IN: u32 = Self::NIGHT_MSG_IN_BG;

    pub fn primary_color() -> Rgba {
        rgb(Self::DAY_BLUE_PRIMARY)
    }

    pub fn day_blue() -> PaletteTokens {
        PaletteTokens {
            window_bg: rgb(Self::DAY_BLUE_WINDOW_BG),
            window_fg: rgb(Self::DAY_BLUE_WINDOW_FG),
            active: rgb(Self::DAY_BLUE_ACTIVE),
            bubble_in_bg: rgb(Self::DAY_BLUE_MSG_IN_BG),
            bubble_in_fg: rgb(Self::DAY_BLUE_WINDOW_FG),
            bubble_out_bg: rgb(Self::DAY_BLUE_MSG_OUT_BG),
            bubble_out_fg: rgb(Self::DAY_BLUE_WINDOW_FG),
            sidebar_bg: rgb(Self::DAY_BLUE_SIDEBAR_BG),
            sidebar_active: rgb(Self::DAY_BLUE_SIDEBAR_ACTIVE),
            sidebar_hover: rgb(Self::DAY_BLUE_SIDEBAR_HOVER),
            unread_badge_bg: rgb(Self::DAY_BLUE_UNREAD_BG),
            unread_badge_fg: rgb(0xFFFFFF),
            unread_badge_muted: rgb(Self::DAY_BLUE_UNREAD_MUTED),
        }
    }

    pub fn night() -> PaletteTokens {
        PaletteTokens {
            window_bg: rgb(Self::NIGHT_WINDOW_BG),
            window_fg: rgb(Self::NIGHT_WINDOW_FG),
            active: rgb(Self::NIGHT_ACTIVE),
            bubble_in_bg: rgb(Self::NIGHT_MSG_IN_BG),
            bubble_in_fg: rgb(Self::NIGHT_WINDOW_FG),
            bubble_out_bg: rgb(Self::NIGHT_MSG_OUT_BG),
            bubble_out_fg: rgb(Self::NIGHT_WINDOW_FG),
            sidebar_bg: rgb(Self::NIGHT_SIDEBAR_BG),
            sidebar_active: rgb(Self::NIGHT_SIDEBAR_ACTIVE),
            sidebar_hover: rgb(Self::NIGHT_SIDEBAR_HOVER),
            unread_badge_bg: rgb(Self::NIGHT_UNREAD_BG),
            unread_badge_fg: rgb(0xFFFFFF),
            unread_badge_muted: rgb(Self::NIGHT_UNREAD_MUTED),
        }
    }
}
