//! Platform Specific Integration Service
//! Upstream: Telegram/SourceFiles/platform/platform_specific.h
//! Subsystem: platform | Feature ID: TCG-N46 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui_telegram_core::error::Result;
use std::collections::HashMap;

pub const K_UNTRANSLOCATED_ARGUMENT: &str = "-untranslocated";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuitReason {
    UserAction,
    AutoRestart,
    UpdateInstalled,
    Crash,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PermissionType {
    Camera,
    Microphone,
    Notifications,
    Location,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionStatus {
    Granted,
    Denied,
    Undetermined,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemSettingsType {
    General,
    Audio,
    Notifications,
    Privacy,
}

#[derive(Debug, Clone)]
pub struct LocationPoint {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: f64,
}

#[derive(Debug, Clone)]
pub struct PlatformSpecific {
    pub is_started: bool,
    pub screenshot_protection: bool,
    pub application_icon: Option<String>,
    pub permissions: HashMap<PermissionType, PermissionStatus>,
    pub is_supported: bool,
    pub is_enabled: bool,
    pub platform_name: String,
}

impl PlatformSpecific {
    pub fn new() -> Self {
        let mut permissions = HashMap::new();
        permissions.insert(PermissionType::Notifications, PermissionStatus::Granted);
        permissions.insert(PermissionType::Microphone, PermissionStatus::Undetermined);
        permissions.insert(PermissionType::Camera, PermissionStatus::Undetermined);
        permissions.insert(PermissionType::Location, PermissionStatus::Undetermined);

        Self {
            is_started: false,
            screenshot_protection: false,
            application_icon: None,
            permissions,
            is_supported: true,
            is_enabled: true,
            platform_name: std::env::consts::OS.to_string(),
        }
    }

    /// Upstream method: `start`
    pub fn start(&mut self) -> Result<()> {
        self.is_started = true;
        Ok(())
    }

    /// Upstream method: `finish`
    pub fn finish(&mut self) -> Result<()> {
        self.is_started = false;
        Ok(())
    }

    /// Upstream method: `SetApplicationIcon`
    pub fn set_application_icon(&mut self) -> Result<()> {
        self.application_icon = Some("telegram_app_icon".into());
        Ok(())
    }

    /// Upstream method: `NewVersionLaunched`
    pub fn new_version_launched(&mut self) -> Result<()> {
        Ok(())
    }

    /// Upstream method: `OpenSystemSettingsForPermission`
    pub fn open_system_settings_for_permission(&mut self) -> Result<()> {
        Ok(())
    }

    /// Upstream method: `SetWindowScreenshotProtection`
    pub fn set_window_screenshot_protection(&mut self) -> Result<()> {
        self.screenshot_protection = true;
        Ok(())
    }

    pub fn set_screenshot_protection(&mut self, enabled: bool) {
        self.screenshot_protection = enabled;
    }

    pub fn check_permission(&self, perm: PermissionType) -> PermissionStatus {
        self.permissions.get(&perm).copied().unwrap_or(PermissionStatus::Undetermined)
    }

    pub fn update_permission(&mut self, perm: PermissionType, status: PermissionStatus) {
        self.permissions.insert(perm, status);
    }
}

impl Default for PlatformSpecific {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for PlatformSpecific {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .p(px(8.0))
            .gap(px(4.0))
            .bg(rgb(0xFFFFFF))
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .rounded_md()
            .child(
                div()
                    .text_size(px(12.0))
                    .font_weight(FontWeight::BOLD)
                    .child(format!("Security & OS Specific ({})", self.platform_name)),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_size(px(11.0))
                            .child(format!("Screenshot Protection: {}", if self.screenshot_protection { "Active" } else { "Disabled" })),
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .child(format!("State: {}", if self.is_started { "Running" } else { "Stopped" })),
                    ),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_specific_lifecycle_and_permissions() {
        let mut specific = PlatformSpecific::new();
        assert!(!specific.is_started);
        assert!(specific.start().is_ok());
        assert!(specific.is_started);

        assert!(specific.set_window_screenshot_protection().is_ok());
        assert!(specific.screenshot_protection);

        assert_eq!(specific.check_permission(PermissionType::Notifications), PermissionStatus::Granted);
        specific.update_permission(PermissionType::Camera, PermissionStatus::Granted);
        assert_eq!(specific.check_permission(PermissionType::Camera), PermissionStatus::Granted);

        assert!(specific.finish().is_ok());
        assert!(!specific.is_started);
    }
}
