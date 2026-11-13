//! Module definitions for `crates/gpui_telegram_desktop/src/platform/mac/touchbar`

pub mod items;
pub mod mac_touchbar_audio;
pub mod mac_touchbar_common;
pub mod mac_touchbar_controls;
pub mod mac_touchbar_main;
pub mod mac_touchbar_manager;
pub mod mac_touchbar_media_view;

pub use mac_touchbar_audio::MacTouchbarAudioMacTouchbar;
pub use mac_touchbar_common::MacTouchbarCommonMacTouchbar;
pub use mac_touchbar_controls::MacTouchbarControlsMacTouchbar;
pub use mac_touchbar_main::MacTouchbarMainMacTouchbar;
pub use mac_touchbar_manager::MacTouchbarManagerMacTouchbar;
pub use mac_touchbar_media_view::MacTouchbarMediaViewMacTouchbar;
