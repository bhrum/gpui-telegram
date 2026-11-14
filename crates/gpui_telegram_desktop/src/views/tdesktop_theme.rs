//! Desktop view theme bridge binding `crate::theme` semantic palettes and `crate::i18n` helpers.

pub use crate::theme::{
    AccentSwatch, PaletteTokens, TdesktopThemeMode, TdesktopThemePalette, ThemePalette,
    WallpaperPreset, ACCENT_SWATCHES, WALLPAPER_PRESETS,
};

/// Inline bilingual selector (`zh` when `lang_id` starts with `"zh"`, else `en`).
pub fn tr_str<'a>(lang_id: &str, zh: &'a str, en: &'a str) -> &'a str {
    crate::i18n::tr_str(lang_id, zh, en)
}

/// Ordered list of the 4 official Telegram Desktop themes shown in Chat Settings.
pub const SUPPORTED_THEME_MODES: [TdesktopThemeMode; 4] = [
    TdesktopThemeMode::Night,
    TdesktopThemeMode::Tinted,
    TdesktopThemeMode::Day,
    TdesktopThemeMode::Classic,
];

/// Builds a complete desktop palette from persisted settings strings and optional wallpaper ID.
pub fn resolve_desktop_palette(
    theme_name: &str,
    accent_hex: u32,
    font_size_px: u32,
    bubble_radius_px: u32,
    wallpaper_id: Option<&str>,
) -> TdesktopThemePalette {
    let mode = TdesktopThemeMode::from_name(theme_name);
    let base = TdesktopThemePalette::build(mode, accent_hex, font_size_px, bubble_radius_px);
    match wallpaper_id {
        Some(id) if !id.trim().is_empty() => base.with_wallpaper(id),
        _ => base,
    }
}
