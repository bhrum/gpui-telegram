//! macOS Specific Private Platform Utilities (Dock, Media Keys, Security Bookmarks)
//! Upstream Source: Telegram/SourceFiles/platform/mac/specific_mac_p.h, Telegram/SourceFiles/platform/mac/specific_mac_p.mm
//! Subsystem: platform | Feature ID: TCG-N46 | Disposition: rust_domain_or_server_1to1

use gpui_telegram_core::error::Result;
use serde::{Deserialize, Serialize};

/// Action items for macOS Dock menu.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacDockMenuItem {
    pub title: String,
    pub unread_badge: u32,
    pub is_separator: bool,
}

/// Private helper implementation for macOS platform specifics and Dock menu.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecificMacPMac {
    pub appkit_version: f64,
    pub documents_path: String,
    pub app_data_path: String,
    pub dock_menu_items: Vec<MacDockMenuItem>,
    pub ignore_activation_timeout_ms: u64,
}

impl SpecificMacPMac {
    pub const IGNORE_ACTIVATION_TIMEOUT_MS: u64 = 500;

    pub fn new() -> Self {
        Self {
            appkit_version: 2200.0, // macOS 13+ AppKit milestone
            documents_path: "~/Documents".into(),
            app_data_path: "~/Library/Application Support/Telegram Desktop".into(),
            dock_menu_items: Vec::new(),
            ignore_activation_timeout_ms: Self::IGNORE_ACTIVATION_TIMEOUT_MS,
        }
    }

    pub fn setup_dock_menu(&mut self) -> Result<()> {
        self.dock_menu_items.clear();
        self.dock_menu_items.push(MacDockMenuItem {
            title: "Open Telegram".into(),
            unread_badge: 0,
            is_separator: false,
        });
        self.dock_menu_items.push(MacDockMenuItem {
            title: "".into(),
            unread_badge: 0,
            is_separator: true,
        });
        self.dock_menu_items.push(MacDockMenuItem {
            title: "Mute for 1 hour".into(),
            unread_badge: 0,
            is_separator: false,
        });
        Ok(())
    }

    pub fn handle_media_key_event(&mut self, key_code: u32) -> Result<bool> {
        // NX_KEYTYPE_PLAY = 16, NX_KEYTYPE_NEXT = 17, NX_KEYTYPE_PREVIOUS = 18
        match key_code {
            16 | 17 | 18 | 19 | 20 => Ok(true),
            _ => Ok(false),
        }
    }

    pub fn create_download_path_bookmark(&self, path: &str) -> Vec<u8> {
        let mut bookmark = b"BOOKMARK_SCOPE:".to_vec();
        bookmark.extend_from_slice(path.as_bytes());
        bookmark
    }

    pub fn enable_download_path_access(&self, bookmark: &[u8]) -> bool {
        bookmark.starts_with(b"BOOKMARK_SCOPE:")
    }
}

impl Default for SpecificMacPMac {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dock_menu_and_bookmarks() {
        let mut p = SpecificMacPMac::new();
        p.setup_dock_menu().unwrap();
        assert_eq!(p.dock_menu_items.len(), 3);
        assert_eq!(p.dock_menu_items[0].title, "Open Telegram");

        assert!(p.handle_media_key_event(16).unwrap());
        assert!(!p.handle_media_key_event(99).unwrap());

        let bm = p.create_download_path_bookmark("~/Downloads");
        assert!(p.enable_download_path_access(&bm));
    }
}
