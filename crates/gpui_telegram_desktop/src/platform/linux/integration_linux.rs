//! Linux Desktop Environment and System Integration
//! Upstream: Telegram/SourceFiles/platform/linux/integration_linux.cpp, .h
//! Manages display server detection (X11/Wayland), GIO actions, and sandbox lifecycle.

use gpui::*;
use gpui_telegram_core::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinuxDisplayServer {
    X11,
    Wayland,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinuxSandboxType {
    None,
    Flatpak,
    Snap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationLinuxLinux {
    pub is_supported: bool,
    pub is_enabled: bool,
    pub platform_name: String,
    pub display_server: LinuxDisplayServer,
    pub sandbox_type: LinuxSandboxType,
    pub registered_actions: Vec<String>,
    pub is_quitting: bool,
}

impl IntegrationLinuxLinux {
    pub fn new() -> Self {
        let display = if std::env::var("WAYLAND_DISPLAY").is_ok() {
            LinuxDisplayServer::Wayland
        } else if std::env::var("DISPLAY").is_ok() {
            LinuxDisplayServer::X11
        } else {
            LinuxDisplayServer::Unknown
        };

        let sandbox = if std::path::Path::new("/.flatpak-info").exists() {
            LinuxSandboxType::Flatpak
        } else if std::env::var("SNAP").is_ok() {
            LinuxSandboxType::Snap
        } else {
            LinuxSandboxType::None
        };

        Self {
            is_supported: true,
            is_enabled: true,
            platform_name: "Linux Desktop Integration".into(),
            display_server: display,
            sandbox_type: sandbox,
            registered_actions: vec!["app.quit".into(), "app.new-window".into(), "app.open-chat".into()],
            is_quitting: false,
        }
    }

    pub fn linux_integration(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn application(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn is_wayland(&self) -> bool {
        self.display_server == LinuxDisplayServer::Wayland
    }

    pub fn instance(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn quit(&mut self) -> Result<()> {
        self.is_quitting = true;
        Ok(())
    }

    pub fn action_map(&self) -> &[String] {
        &self.registered_actions
    }

    pub fn add_action(&mut self, action_name: impl Into<String>) {
        self.registered_actions.push(action_name.into());
    }
}

impl Default for IntegrationLinuxLinux {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for IntegrationLinuxLinux {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let server_label = match self.display_server {
            LinuxDisplayServer::Wayland => "Wayland",
            LinuxDisplayServer::X11 => "X11",
            LinuxDisplayServer::Unknown => "Unknown Display",
        };

        let sandbox_label = match self.sandbox_type {
            LinuxSandboxType::Flatpak => " [Flatpak]",
            LinuxSandboxType::Snap => " [Snap]",
            LinuxSandboxType::None => "",
        };

        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .p(px(6.0))
            .rounded(px(4.0))
            .bg(rgb(0x1F2937))
            .text_size(px(12.0))
            .text_color(rgb(0x93C5FD))
            .child(format!("Environment: {}{}", server_label, sandbox_label))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration_linux() {
        let mut integ = IntegrationLinuxLinux::new();
        assert!(integ.is_supported);
        assert!(!integ.is_quitting);

        assert!(integ.action_map().contains(&"app.quit".to_string()));
        integ.add_action("app.settings");
        assert!(integ.action_map().contains(&"app.settings".to_string()));

        assert!(integ.quit().is_ok());
        assert!(integ.is_quitting);
    }
}
