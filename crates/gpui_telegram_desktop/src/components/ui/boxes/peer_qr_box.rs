//! Authentic GPUI Peer QR Code & Share Modal Component.
//! Reference: Telegram/SourceFiles/ui/boxes/peer_qr_box.cpp:L50-L1891
//! Reference: Telegram/SourceFiles/ui/boxes/peer_qr_box.h:L18-L28

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Toast display duration for copied link: 2800ms.
/// Reference: `peer_qr_box.cpp:L55`
pub const K_TOAST_DURATION_MS: u64 = 2800;

/// Interaction cooldown for share actions: 3000ms.
/// Reference: `peer_qr_box.cpp:L56`
pub const K_SHARE_CUTOFF_MS: u64 = 3000;

/// Maximum decorative emoji center badge choices: 6.
/// Reference: `peer_qr_box.cpp:L57`
pub const K_MAX_EMOJI_PICKS: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QrThemeColor {
    Blue,
    Purple,
    Green,
    Orange,
    Dark,
}

/// Target symbol for peer QR code modal box.
/// Reference: `peer_qr_box.h:L18-L28`
#[derive(Debug, Clone)]
pub struct PeerQrBoxBoxes {
    pub link: String,
    pub title: String,
    pub about: String,
    pub theme: QrThemeColor,
    pub copied_to_clipboard: bool,
    pub toast_active: bool,
}

impl Default for PeerQrBoxBoxes {
    fn default() -> Self {
        Self::new("https://t.me/durov", "Pavel Durov")
    }
}

impl PeerQrBoxBoxes {
    pub fn new(link: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            link: link.into(),
            title: title.into(),
            about: "Scan this QR code with your phone camera to join".to_string(),
            theme: QrThemeColor::Blue,
            copied_to_clipboard: false,
            toast_active: false,
        }
    }

    /// Triggers copy link to clipboard and activates confirmation toast.
    /// Reference: `peer_qr_box.cpp:L320-L335`
    pub fn copy_link(&mut self) -> String {
        self.copied_to_clipboard = true;
        self.toast_active = true;
        self.link.clone()
    }

    pub fn set_theme(&mut self, theme: QrThemeColor) {
        self.theme = theme;
    }
}
