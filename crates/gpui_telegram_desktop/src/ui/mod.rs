//! Desktop UI primitives coordinator re-exporting and driving `crate::components::ui`.

pub use crate::components::ui::*;

use gpui::Rgba;

/// Runtime helper adapter bridging WCAG contrast, unread badge formatting, and frame palette checks.
#[derive(Debug, Clone, Copy, Default)]
pub struct UiShellToolkit;

impl UiShellToolkit {
    pub fn new() -> Self {
        Self
    }

    /// Formats unread counter badges for dialogs, folders, and rail pills.
    pub fn format_badge_count(&self, unread: i32, has_mention: bool, narrow: bool) -> String {
        UnreadCounterFormat::format(unread, has_mention, narrow)
    }

    /// Formats large subscriber/view counts (`1.2K`, `4M`) via Telegram Desktop metrics rules.
    pub fn format_compact_count(&self, total: i64) -> String {
        UnreadCounterFormat::format_short(total)
    }

    /// Returns WCAG AA high-contrast foreground color (black or white) for a 24-bit RGB hex surface.
    pub fn contrast_text_for_hex(&self, bg_hex: u32) -> Rgba {
        let r = ((bg_hex >> 16) & 0xFF) as u8;
        let g = ((bg_hex >> 8) & 0xFF) as u8;
        let b = (bg_hex & 0xFF) as u8;
        ColorContrast::optimal_text_color(r, g, b)
    }

    /// Selects active light or dark desktop titlebar palette tokens.
    pub fn window_frame_palette(&self, is_dark: bool) -> WindowPalette {
        if is_dark {
            WindowPalette::dark()
        } else {
            WindowPalette::light()
        }
    }
}
