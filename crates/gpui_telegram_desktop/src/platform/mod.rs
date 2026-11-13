//! Module definitions for `crates/gpui_telegram_desktop/src/platform`

pub mod linux;
pub mod mac;
pub mod platform_current_geo_location;
pub mod platform_file_bookmark;
pub mod platform_file_utilities;
pub mod platform_integration;
pub mod platform_launcher;
pub mod platform_main_window;
pub mod platform_notifications_manager;
pub mod platform_overlay_widget;
pub mod platform_specific;
pub mod platform_text_recognition;
pub mod platform_translate_provider;
pub mod platform_tray;
pub mod platform_webauthn;
pub mod platform_window_title;
pub mod win;

pub use platform_current_geo_location::PlatformCurrentGeoLocation;
pub use platform_file_bookmark::PlatformFileBookmark;
pub use platform_file_utilities::PlatformFileUtilities;
pub use platform_integration::PlatformIntegration;
pub use platform_launcher::PlatformLauncher;
pub use platform_main_window::PlatformMainWindow;
pub use platform_notifications_manager::PlatformNotificationsManager;
pub use platform_overlay_widget::PlatformOverlayWidget;
pub use platform_specific::PlatformSpecific;
pub use platform_text_recognition::PlatformTextRecognition;
pub use platform_translate_provider::PlatformTranslateProvider;
pub use platform_tray::PlatformTray;
pub use platform_webauthn::PlatformWebauthn;
pub use platform_window_title::PlatformWindowTitle;
