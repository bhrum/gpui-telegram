//! Full semantic `TdesktopThemePalette` struct and helpers.

use std::collections::HashMap;
use gpui::{rgb, Rgba};

use super::palette::PaletteTokens;
use super::palette_presets::preset_palette;
use super::palette_schemes::{
    find_wallpaper_preset, parse_tdesktop_theme_colors, peer_color_index, rgba_from_hex_alpha,
    TdesktopThemeMode,
};

/// Complete semantic color, typography, bubble geometry, and peer palette derived
/// from Telegram Desktop `.tdesktop-theme` tokens across `Night`, `Tinted`, `Day`, and `Classic`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TdesktopThemePalette {
    pub mode: TdesktopThemeMode,
    pub window_bg: u32,
    pub hover_bg: u32,
    pub nav_rail_bg: u32,
    pub rail_bg: u32,
    pub dialogs_bg: u32,
    pub dialogs_hover_bg: u32,
    pub dialogs_active_bg: u32,
    pub history_bg: u32,
    pub stage_bg: u32,
    pub top_bar_bg: u32,
    pub topbar_bg: u32,
    pub compose_bg: u32,
    pub composer_bg: u32,
    pub input_bg: u32,
    pub card_bg: u32,
    pub modal_bg: u32,
    pub tooltip_bg: u32,
    pub tooltip_fg: u32,
    pub menu_bg: u32,
    pub menu_hover_bg: u32,
    pub scrollbar_bg: u32,
    pub scrollbar_hover_bg: u32,
    pub msg_in_bg: u32,
    pub msg_out_bg: u32,
    pub msg_in_selected_bg: u32,
    pub msg_out_selected_bg: u32,
    pub msg_in_date_fg: u32,
    pub msg_out_date_fg: u32,
    pub reply_bar_fg: u32,
    pub reply_bar_bg: u32,
    pub code_bg: u32,
    pub code_fg: u32,
    pub service_pill_bg: u32,
    pub service_pill_fg: u32,
    pub unread_badge_bg: u32,
    pub unread_badge_fg: u32,
    pub muted_badge_bg: u32,
    pub online_dot_fg: u32,
    pub check_read_fg: u32,
    pub check_sent_fg: u32,
    pub icon_fg: u32,
    pub icon_active: u32,
    pub border_fg: u32,
    pub shadow_fg: u32,
    pub text_primary: u32,
    pub text_secondary: u32,
    pub text_link: u32,
    pub accent: u32,
    pub accent_hover: u32,
    pub active_button_fg: u32,
    pub reaction_bg: u32,
    pub reaction_active_bg: u32,
    pub danger: u32,
    pub success: u32,
    pub wallpaper_top: u32,
    pub wallpaper_bottom: u32,
    pub wallpaper_pattern: u32,
    pub peer_name_colors: [u32; 7],
    pub peer_avatar_colors: [u32; 7],
    pub peer_avatar_colors_bottom: [u32; 7],
    pub font_size_px: u32,
    pub bubble_radius_px: u32,
    pub ui_scale_pct: u32,
    pub blur_wallpaper: bool,
}

impl TdesktopThemePalette {
    /// Constructs a full semantic palette for the requested theme mode, custom accent color,
    /// message font scale (`12..=20`), and bubble corner radius (`6..=18`).
    pub fn build(
        mode: TdesktopThemeMode,
        accent: u32,
        font_size_px: u32,
        bubble_radius_px: u32,
    ) -> Self {
        preset_palette(mode, accent, font_size_px, bubble_radius_px)
    }

    /// Applies live interface zoom percentage (`100..=300`) to scale fixed-pixel UI tokens.
    pub fn with_ui_scale(mut self, scale_pct: u32) -> Self {
        self.ui_scale_pct = scale_pct.clamp(50, 300);
        self
    }

    /// Applies wallpaper frosted-blur state to stage rendering.
    pub fn with_blur_wallpaper(mut self, blur: bool) -> Self {
        self.blur_wallpaper = blur;
        self
    }

    /// Multiplier (`1.0` at `100%`, `1.25` at `125%`, `1.5` at `150%`, `2.0` at `200%`).
    pub fn scale_factor(&self) -> f32 {
        let pct = if self.ui_scale_pct == 0 { 100 } else { self.ui_scale_pct.clamp(50, 300) };
        (pct as f32) / 100.0
    }

    /// Scales a base pixel metric by the active interface scale factor.
    pub fn scaled_px(&self, base_px: f32) -> f32 {
        base_px * self.scale_factor()
    }

    /// Applies a named built-in wallpaper preset (`default_night`, `aurora_dusk`, etc.).
    pub fn with_wallpaper(mut self, preset_id: &str) -> Self {
        if let Some(preset) = find_wallpaper_preset(preset_id) {
            self.wallpaper_top = preset.top_hex;
            self.wallpaper_bottom = preset.bottom_hex;
            self.wallpaper_pattern = preset.pattern_hex;
        }
        self
    }

    /// Deterministic Telegram 7-slot sender name color for `peer_id`.
    pub const fn peer_name_color(&self, peer_id: i64) -> u32 {
        self.peer_name_colors[peer_color_index(peer_id)]
    }

    /// Deterministic Telegram 7-slot avatar base color for `peer_id`.
    pub const fn peer_avatar_color(&self, peer_id: i64) -> u32 {
        self.peer_avatar_colors[peer_color_index(peer_id)]
    }

    /// Deterministic Telegram 7-slot `(top, bottom)` avatar gradient colors for `peer_id`.
    pub const fn peer_avatar_gradient(&self, peer_id: i64) -> (u32, u32) {
        let idx = peer_color_index(peer_id);
        (self.peer_avatar_colors[idx], self.peer_avatar_colors_bottom[idx])
    }

    /// Converts an `0xRRGGBB` hex integer plus float opacity (`0.0..=1.0`) to GPUI `Rgba`.
    pub fn rgba_with_alpha(hex: u32, alpha: f32) -> Rgba {
        rgba_from_hex_alpha(hex, alpha)
    }

    /// Translucent service pill / date badge background matching Fabushi / Telegram Desktop stage overlays.
    pub fn service_pill_rgba(&self) -> Rgba {
        let alpha = if self.mode.is_fabushi_style() {
            0.92
        } else if self.mode.is_dark() {
            0.80
        } else {
            0.52
        };
        Self::rgba_with_alpha(self.service_pill_bg, alpha)
    }

    /// Translucent idle scrollbar thumb color.
    pub fn scrollbar_rgba(&self) -> Rgba {
        let alpha = if self.mode.is_dark() { 0.28 } else { 0.34 };
        Self::rgba_with_alpha(self.scrollbar_bg, alpha)
    }

    /// Translucent hovered scrollbar thumb color.
    pub fn scrollbar_hover_rgba(&self) -> Rgba {
        let alpha = if self.mode.is_dark() { 0.48 } else { 0.55 };
        Self::rgba_with_alpha(self.scrollbar_hover_bg, alpha)
    }

    /// Subtle drop-shadow color for floating menus, popovers, and modal boxes.
    pub fn shadow_rgba(&self) -> Rgba {
        let alpha = if self.mode.is_dark() { 0.42 } else { 0.16 };
        Self::rgba_with_alpha(self.shadow_fg, alpha)
    }

    /// Converts this semantic palette into legacy [`PaletteTokens`] for existing widgets/tests.
    pub fn to_palette_tokens(&self) -> PaletteTokens {
        PaletteTokens {
            window_bg: rgb(self.window_bg),
            window_fg: rgb(self.text_primary),
            active: rgb(self.accent),
            bubble_in_bg: rgb(self.msg_in_bg),
            bubble_in_fg: rgb(self.text_primary),
            bubble_out_bg: rgb(self.msg_out_bg),
            bubble_out_fg: rgb(self.text_primary),
            sidebar_bg: rgb(self.dialogs_bg),
            sidebar_active: rgb(self.dialogs_active_bg),
            sidebar_hover: rgb(self.dialogs_hover_bg),
            unread_badge_bg: rgb(self.unread_badge_bg),
            unread_badge_fg: rgb(self.unread_badge_fg),
            unread_badge_muted: rgb(self.muted_badge_bg),
        }
    }

    /// Parses `.tdesktop-theme` color definitions (`key: #rrggbb;` / aliases) into `0xRRGGBB`.
    pub fn parse_tdesktop_theme_colors(content: &str) -> HashMap<String, u32> {
        parse_tdesktop_theme_colors(content)
    }
}

impl Default for TdesktopThemePalette {
    fn default() -> Self {
        Self::build(TdesktopThemeMode::Night, 0x1084FE, 14, 17)
    }
}
