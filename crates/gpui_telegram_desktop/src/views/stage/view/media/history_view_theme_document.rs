//! Authentic GPUI History View Theme Document Component.
//!
//! Upstream:
//! - `Telegram/SourceFiles/history/view/media/history_view_theme_document.h:L1-L189`
//! - `Telegram/SourceFiles/history/view/media/history_view_theme_document.cpp:L1-L894`
//!
//! Wallpaper and Theme Rendering Parameters (`history_view_theme_document.h:L84-L93`):
//! - Pattern opacity float64
//! - Gradient rotation angle
//! - Blurred wallpaper background preparation

/// Component for wallpaper documents and theme preview cards in history transcript.
/// Upstream: `HistoryView::ThemeDocument` (`history_view_theme_document.h:L26-L93`)
#[derive(Debug, Clone)]
pub struct HistoryViewThemeDocumentViewMedia {
    pub document_id: u64,
    pub pattern_opacity: f32,
    pub gradient_rotation: i32,
    pub is_dark: bool,
    pub is_blurred: bool,
    pub dimming_intensity: i32,
}

impl HistoryViewThemeDocumentViewMedia {
    pub fn new(document_id: u64) -> Self {
        Self {
            document_id,
            pattern_opacity: 0.5,
            gradient_rotation: 0,
            is_dark: false,
            is_blurred: false,
            dimming_intensity: 0,
        }
    }

    pub fn with_wallpaper_params(mut self, opacity: f32, rotation: i32, is_dark: bool) -> Self {
        self.pattern_opacity = opacity.clamp(0.0, 1.0);
        self.gradient_rotation = rotation % 360;
        self.is_dark = is_dark;
        self
    }
}
