//! Upstream C++ source: `Telegram/SourceFiles/ui/empty_userpic.cpp:L1-L728` & `empty_userpic.h:L1-L158`
//! Authentic GPUI implementation of fallback userpic gradients and initials rendering.
#![allow(dead_code)]

use gpui::{hsla, px, Hsla, Pixels};

/// Predefined userpic gradient color stops (`empty_userpic.h:L16-L19`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BgColors {
    pub color1: Hsla,
    pub color2: Hsla,
}

/// Fallback avatar generator using initials and deterministic gradient backgrounds.
/// C++ companion: `Ui::EmptyUserpic` (`empty_userpic.cpp:L24-L728`).
#[derive(Debug, Clone)]
pub struct EmptyUserpic {
    colors: BgColors,
    initials: String,
    size: Pixels,
    is_saved_messages: bool,
    is_replies: bool,
}

impl EmptyUserpic {
    /// Resolves one of the 8 standard Telegram empty avatar gradient presets.
    pub fn resolve_colors(index: usize) -> BgColors {
        match index % 8 {
            0 => BgColors { color1: hsla(0.0, 0.70, 0.66, 1.0), color2: hsla(5.0, 0.85, 0.76, 1.0) },       // Red
            1 => BgColors { color1: hsla(24.0, 0.85, 0.72, 1.0), color2: hsla(30.0, 0.90, 0.78, 1.0) },     // Orange
            2 => BgColors { color1: hsla(260.0, 0.65, 0.75, 1.0), color2: hsla(270.0, 0.80, 0.82, 1.0) },   // Violet
            3 => BgColors { color1: hsla(105.0, 0.55, 0.58, 1.0), color2: hsla(115.0, 0.68, 0.66, 1.0) },   // Green
            4 => BgColors { color1: hsla(180.0, 0.60, 0.61, 1.0), color2: hsla(185.0, 0.75, 0.66, 1.0) },   // Cyan
            5 => BgColors { color1: hsla(205.0, 0.70, 0.63, 1.0), color2: hsla(210.0, 0.85, 0.72, 1.0) },   // Blue
            6 => BgColors { color1: hsla(330.0, 0.75, 0.71, 1.0), color2: hsla(335.0, 0.85, 0.76, 1.0) },   // Pink
            _ => BgColors { color1: hsla(225.0, 0.68, 0.68, 1.0), color2: hsla(230.0, 0.80, 0.75, 1.0) },   // Dark Blue
        }
    }

    /// Creates an empty userpic from name string and peer ID.
    pub fn for_peer(title: &str, peer_id: u64, size: f32) -> Self {
        let colors = Self::resolve_colors((peer_id & 0x07) as usize);
        let initials = Self::extract_initials(title);
        Self {
            colors,
            initials,
            size: px(size),
            is_saved_messages: false,
            is_replies: false,
        }
    }

    /// Creates the special Saved Messages userpic (blue ribbon bookmark).
    pub fn for_saved_messages(size: f32) -> Self {
        Self {
            colors: Self::resolve_colors(5),
            initials: "🔖".to_string(),
            size: px(size),
            is_saved_messages: true,
            is_replies: false,
        }
    }

    /// Extracts 1 or 2 uppercase initials from a peer's display name.
    pub fn extract_initials(title: &str) -> String {
        let words: Vec<&str> = title.split_whitespace().collect();
        match words.len() {
            0 => String::new(),
            1 => words[0].chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_default(),
            _ => {
                let first = words[0].chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_default();
                let second = words[1].chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_default();
                format!("{}{}", first, second)
            }
        }
    }

    pub fn colors(&self) -> BgColors {
        self.colors
    }

    pub fn initials(&self) -> &str {
        &self.initials
    }

    pub fn size(&self) -> Pixels {
        self.size
    }
}

impl Default for EmptyUserpic {
    fn default() -> Self {
        Self::for_peer("Telegram", 0, 48.0)
    }
}
