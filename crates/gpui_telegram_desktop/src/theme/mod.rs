//! Module definitions for `crates/gpui_telegram_desktop/src/theme`

pub mod boxes;
pub mod chat_helpers;
pub mod dialogs;
pub mod history;
pub mod info;
pub mod intro;
pub mod palette;
pub mod settings;
pub mod ui;
pub mod wallet;
pub mod window;

pub use palette::ThemePalette;
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
