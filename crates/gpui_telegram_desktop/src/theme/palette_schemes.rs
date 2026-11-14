//! Upstream Telegram Desktop theme modes, accent swatches, wallpaper presets,
//! peer identity color tables, and `.tdesktop-theme` color parser.

use std::collections::HashMap;
use gpui::Rgba;

/// Supported Telegram Desktop color themes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TdesktopThemeMode {
    #[default]
    Night,
    Tinted,
    Day,
    Classic,
}

impl TdesktopThemeMode {
    /// Parses a persisted or user-entered theme name (defaults to `Night`).
    pub fn from_name(name: &str) -> Self {
        match name.trim().to_ascii_lowercase().as_str() {
            "night" | "night-default" => Self::Night,
            "tinted" | "night tinted" | "night_tinted" | "night-green" => Self::Tinted,
            "day" | "day blue" | "day_blue" | "day-blue" => Self::Day,
            "classic" | "day classic" | "day_classic" => Self::Classic,
            _ => Self::Night,
        }
    }

    /// Canonical display/storage name matching Telegram Settings & integration tests.
    pub const fn as_name(&self) -> &'static str {
        match self {
            Self::Night => "Night",
            Self::Tinted => "Tinted",
            Self::Day => "Day",
            Self::Classic => "Classic",
        }
    }

    /// Returns `true` for dark themes (`Night` and `Tinted`).
    pub const fn is_dark(&self) -> bool {
        matches!(self, Self::Night | Self::Tinted)
    }

    /// Returns the default upstream accent hex (`0xRRGGBB`) for this mode.
    pub const fn default_accent(&self) -> u32 {
        match self {
            Self::Night => 0x5288C1,
            Self::Tinted => 0x3FC1B0,
            Self::Day | Self::Classic => 0x40A7E3,
        }
    }
}

/// A selectable accent swatch shown in Chat Settings -> Accent Color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccentSwatch {
    pub name: &'static str,
    pub hex: u32,
}

/// Curated Telegram Desktop accent swatches.
pub const ACCENT_SWATCHES: &[AccentSwatch] = &[
    AccentSwatch { name: "Sky Blue", hex: 0x40A7E3 },
    AccentSwatch { name: "Classic Blue", hex: 0x5288C1 },
    AccentSwatch { name: "Teal Mint", hex: 0x3FC1B0 },
    AccentSwatch { name: "Emerald", hex: 0x4FAD2D },
    AccentSwatch { name: "Amber Gold", hex: 0xE5A030 },
    AccentSwatch { name: "Coral Orange", hex: 0xE86C52 },
    AccentSwatch { name: "Crimson Rose", hex: 0xE16794 },
    AccentSwatch { name: "Lavender", hex: 0x8544D6 },
];

/// A built-in stage chat wallpaper gradient + vector doodle tint preset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WallpaperPreset {
    pub id: &'static str,
    pub name: &'static str,
    pub top_hex: u32,
    pub bottom_hex: u32,
    pub pattern_hex: u32,
    pub is_dark: bool,
}

/// Curated built-in wallpapers covering Night, Tinted, Day, and Classic themes.
pub const WALLPAPER_PRESETS: &[WallpaperPreset] = &[
    WallpaperPreset {
        id: "default_night",
        name: "Midnight Ocean",
        top_hex: 0x0E1621,
        bottom_hex: 0x16222E,
        pattern_hex: 0x223244,
        is_dark: true,
    },
    WallpaperPreset {
        id: "default_tinted",
        name: "Deep Forest",
        top_hex: 0x1C2227,
        bottom_hex: 0x232B30,
        pattern_hex: 0x313E46,
        is_dark: true,
    },
    WallpaperPreset {
        id: "default_day",
        name: "Daylight Sky",
        top_hex: 0xE5EFF7,
        bottom_hex: 0xDCEAF4,
        pattern_hex: 0xC2D7E8,
        is_dark: false,
    },
    WallpaperPreset {
        id: "default_classic",
        name: "Telegram Classic",
        top_hex: 0xD5E8C8,
        bottom_hex: 0xB8D8A3,
        pattern_hex: 0x93BD7E,
        is_dark: false,
    },
    WallpaperPreset {
        id: "aurora_dusk",
        name: "Aurora Dusk",
        top_hex: 0x181B2C,
        bottom_hex: 0x231E36,
        pattern_hex: 0x363058,
        is_dark: true,
    },
    WallpaperPreset {
        id: "desert_sand",
        name: "Warm Sand",
        top_hex: 0xF2E9DB,
        bottom_hex: 0xE7D7C1,
        pattern_hex: 0xCCB698,
        is_dark: false,
    },
    WallpaperPreset {
        id: "monochrome_slate",
        name: "Graphite Slate",
        top_hex: 0x171A1F,
        bottom_hex: 0x1F242B,
        pattern_hex: 0x2D343D,
        is_dark: true,
    },
];

pub fn find_wallpaper_preset(preset_id: &str) -> Option<&'static WallpaperPreset> {
    let needle = preset_id.trim();
    WALLPAPER_PRESETS
        .iter()
        .find(|p| p.id.eq_ignore_ascii_case(needle) || p.name.eq_ignore_ascii_case(needle))
}

/// Upstream 7 Telegram peer sender name colors (Day / Classic).
pub const PEER_NAME_COLORS_DAY: [u32; 7] =
    [0xC03D33, 0x4FAD2D, 0xD09306, 0x168ACD, 0x8544D6, 0xCD4073, 0x2996AD];
/// Upstream 7 Telegram peer sender name colors (Night).
pub const PEER_NAME_COLORS_NIGHT: [u32; 7] =
    [0xFB6169, 0x85DE85, 0xF3BC5C, 0x65BDF3, 0xB48BF2, 0xFF5694, 0x62D4E3];
/// Upstream 7 Telegram peer sender name colors (Tinted / Night Green).
pub const PEER_NAME_COLORS_TINTED: [u32; 7] =
    [0xEC7577, 0x86D67F, 0xE4C054, 0x68C7F3, 0xB383F3, 0xE16794, 0x57C9E0];
/// Upstream 7 Telegram peer avatar gradient top stops.
pub const PEER_AVATAR_TOP: [u32; 7] =
    [0xFF845E, 0x9AD164, 0xECC65B, 0x5CAFFA, 0xB694F9, 0xFF8AAC, 0x5BCBE3];
/// Upstream 7 Telegram peer avatar gradient bottom stops.
pub const PEER_AVATAR_BOTTOM: [u32; 7] =
    [0xD45246, 0x46BA43, 0xD59F36, 0x408ACF, 0x6C61DF, 0xD95574, 0x359AD4];

pub const fn peer_color_index(peer_id: i64) -> usize {
    (peer_id.unsigned_abs() as usize) % 7
}

pub fn mix_rgb(a: u32, b: u32, t: f32) -> u32 {
    let t = t.clamp(0.0, 1.0);
    let mix_ch = |shift: u32| -> u32 {
        let ca = ((a >> shift) & 0xFF) as f32;
        let cb = ((b >> shift) & 0xFF) as f32;
        (ca + (cb - ca) * t).round().clamp(0.0, 255.0) as u32
    };
    (mix_ch(16) << 16) | (mix_ch(8) << 8) | mix_ch(0)
}

pub fn adjust_rgb(hex: u32, delta: i32) -> u32 {
    let shift_ch = |shift: u32| -> u32 {
        (((hex >> shift) & 0xFF) as i32 + delta).clamp(0, 255) as u32
    };
    (shift_ch(16) << 16) | (shift_ch(8) << 8) | shift_ch(0)
}

pub fn rgba_from_hex_alpha(hex: u32, alpha: f32) -> Rgba {
    Rgba {
        r: ((hex >> 16) & 0xFF) as f32 / 255.0,
        g: ((hex >> 8) & 0xFF) as f32 / 255.0,
        b: (hex & 0xFF) as f32 / 255.0,
        a: alpha.clamp(0.0, 1.0),
    }
}

/// Parses upstream `.tdesktop-theme` palette declarations (`key: #rrggbb;` / `#rrggbbaa;` / aliases).
pub fn parse_tdesktop_theme_colors(content: &str) -> HashMap<String, u32> {
    let mut cleaned = String::with_capacity(content.len());
    let mut in_block = false;
    let mut chars = content.chars().peekable();
    while let Some(c) = chars.next() {
        if in_block {
            if c == '*' && chars.peek() == Some(&'/') {
                chars.next();
                in_block = false;
            }
            continue;
        }
        if c == '/' && chars.peek() == Some(&'*') {
            chars.next();
            in_block = true;
        } else if c == '/' && chars.peek() == Some(&'/') {
            for nc in chars.by_ref() {
                if nc == '\n' {
                    cleaned.push('\n');
                    break;
                }
            }
        } else {
            cleaned.push(c);
        }
    }

    let mut colors = HashMap::new();
    let mut aliases: Vec<(String, String)> = Vec::new();
    for stmt in cleaned.split(';') {
        let Some((k, v)) = stmt.split_once(':') else {
            continue;
        };
        let key = k.trim();
        let val = v.trim();
        if key.is_empty() || val.is_empty() {
            continue;
        }
        if let Some(hex_str) = val.strip_prefix('#') {
            if hex_str.len() == 6 {
                if let Ok(rgb) = u32::from_str_radix(hex_str, 16) {
                    colors.insert(key.to_owned(), rgb);
                }
            } else if hex_str.len() == 8 {
                if let Ok(rgba) = u32::from_str_radix(hex_str, 16) {
                    colors.insert(key.to_owned(), (rgba >> 8) & 0xFF_FFFF);
                }
            }
        } else if !val.contains(' ') {
            aliases.push((key.to_owned(), val.to_owned()));
        }
    }
    for _ in 0..4 {
        for (key, target) in &aliases {
            if !colors.contains_key(key) {
                if let Some(&rgb) = colors.get(target) {
                    colors.insert(key.clone(), rgb);
                }
            }
        }
    }
    colors
}
