//! Module definitions for `crates/gpui_telegram_desktop/src/theme`

pub mod assets;
mod assets_embedded;
pub mod boxes;
pub mod chat_helpers;
pub mod dialogs;
pub mod history;
pub mod info;
pub mod intro;
pub mod palette;
mod palette_presets;
mod palette_schemes;
pub mod settings;
mod tdesktop_palette;
pub mod ui;
pub mod wallet;
pub mod window;

pub use assets::{IconName, TelegramAssetSource, tg_icon, tg_icon_path, tg_icon_rgba};
pub use palette::{
    AccentSwatch, PaletteTokens, TdesktopThemeMode, TdesktopThemePalette, ThemePalette,
    WallpaperPreset, ACCENT_SWATCHES, WALLPAPER_PRESETS,
};
pub mod api;
pub use api::*;
pub mod calls;
pub use calls::*;
pub mod editor;
pub use editor::*;
pub mod export;
pub use export::*;
pub mod iv;
pub use iv::*;
pub mod media;
pub use media::*;
pub mod passport;
pub use passport::*;
pub mod payments;
pub use payments::*;
pub mod statistics;
pub use statistics::*;
