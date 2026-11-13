//! Module definitions for `crates/gpui_telegram_desktop/src/platform/linux`

pub mod current_geo_location_linux;
pub mod file_utilities_linux;
pub mod integration_linux;
pub mod launcher_linux;
pub mod main_window_linux;
pub mod notifications_manager_linux;
pub mod overlay_widget_linux;
pub mod specific_linux;
pub mod text_recognition_linux;
pub mod translate_provider_linux;
pub mod tray_linux;
pub mod update_install_linux;
pub mod webauthn_linux;

pub use current_geo_location_linux::CurrentGeoLocationLinuxLinux;
pub use file_utilities_linux::FileUtilitiesLinuxLinux;
pub use integration_linux::IntegrationLinuxLinux;
pub use launcher_linux::LauncherLinuxLinux;
pub use main_window_linux::MainWindowLinuxLinux;
pub use notifications_manager_linux::NotificationsManagerLinuxLinux;
pub use overlay_widget_linux::OverlayWidgetLinuxLinux;
pub use specific_linux::SpecificLinuxLinux;
pub use text_recognition_linux::TextRecognitionLinuxLinux;
pub use translate_provider_linux::TranslateProviderLinuxLinux;
pub use tray_linux::TrayLinuxLinux;
pub use update_install_linux::UpdateInstallLinuxLinux;
pub use webauthn_linux::WebauthnLinuxLinux;
