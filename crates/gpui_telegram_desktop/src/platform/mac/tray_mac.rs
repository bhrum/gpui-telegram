//! macOS Menu Bar Status Item Tray
//! Upstream Source: Telegram/SourceFiles/platform/mac/tray_mac.h, Telegram/SourceFiles/platform/mac/tray_mac.mm
//! Subsystem: platform | Feature ID: TCG-N46 | Disposition: rust_domain_or_server_1to1

use gpui_telegram_core::error::Result;
use serde::{Deserialize, Serialize};

/// Action item on macOS system menu bar tray.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacTrayAction {
    pub id: String,
    pub title: String,
    pub is_separator: bool,
    pub is_enabled: bool,
}

/// macOS Status Item system tray implementation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrayMacMac {
    pub has_icon: bool,
    pub is_dark_appearance: bool,
    pub unread_count: u32,
    pub actions: Vec<MacTrayAction>,
    pub last_action_triggered: Option<String>,
}

impl TrayMacMac {
    pub fn new() -> Self {
        Self {
            has_icon: false,
            is_dark_appearance: true,
            unread_count: 0,
            actions: Vec::new(),
            last_action_triggered: None,
        }
    }

    pub fn create_icon(&mut self) -> Result<()> {
        self.has_icon = true;
        Ok(())
    }

    pub fn destroy_icon(&mut self) -> Result<()> {
        self.has_icon = false;
        Ok(())
    }

    pub fn update_appearance(&mut self, is_dark: bool) -> Result<()> {
        self.is_dark_appearance = is_dark;
        Ok(())
    }

    pub fn set_unread_count(&mut self, count: u32) -> Result<()> {
        self.unread_count = count;
        Ok(())
    }

    pub fn add_action(&mut self, id: impl Into<String>, title: impl Into<String>) -> Result<()> {
        self.actions.push(MacTrayAction {
            id: id.into(),
            title: title.into(),
            is_separator: false,
            is_enabled: true,
        });
        Ok(())
    }

    pub fn add_separator(&mut self) -> Result<()> {
        self.actions.push(MacTrayAction {
            id: String::new(),
            title: String::new(),
            is_separator: true,
            is_enabled: false,
        });
        Ok(())
    }

    pub fn trigger_action(&mut self, id: &str) -> Result<bool> {
        if let Some(pos) = self.actions.iter().position(|a| a.id == id && a.is_enabled) {
            let act_id = self.actions[pos].id.clone();
            self.last_action_triggered = Some(act_id);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl Default for TrayMacMac {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tray_menu_actions() {
        let mut tray = TrayMacMac::new();
        tray.create_icon().unwrap();
        assert!(tray.has_icon);

        tray.add_action("open", "Show Telegram").unwrap();
        tray.add_separator().unwrap();
        tray.add_action("quit", "Quit Telegram").unwrap();
        assert_eq!(tray.actions.len(), 3);

        assert!(tray.trigger_action("open").unwrap());
        assert_eq!(tray.last_action_triggered, Some("open".into()));

        tray.set_unread_count(5).unwrap();
        assert_eq!(tray.unread_count, 5);
    }
}
