//! Authentic GPUI theme stylesheet for EditPeerMembers (Channel & Group Admin/Banned Rights).
//! Upstream reference: `Telegram/SourceFiles/boxes/peers/edit_peer_members.style:L1-L127`.

use gpui::{px, rgb, Pixels, Rgba};

#[derive(Debug, Clone)]
pub struct EditPeerMembersPeersStyleSheet {
    pub members_padding: (Pixels, Pixels, Pixels, Pixels),
    pub rights_row_height: Pixels,
    pub rights_checkbox_size: Pixels,
    pub slider_height: Pixels,
    pub section_header_height: Pixels,
    pub title_fg: Rgba,
    pub username_default_fg: Rgba,
    pub restricted_fg: Rgba,
    pub admin_accent_fg: Rgba,
    pub divider_color: Rgba,
}

impl Default for EditPeerMembersPeersStyleSheet {
    fn default() -> Self {
        Self {
            members_padding: (px(24.0), px(16.0), px(24.0), px(16.0)),
            rights_row_height: px(44.0),
            rights_checkbox_size: px(20.0),
            slider_height: px(32.0),
            section_header_height: px(36.0),
            title_fg: rgb(0x1E293B),
            username_default_fg: rgb(0x64748B),
            restricted_fg: rgb(0xEF4444),
            admin_accent_fg: rgb(0x3B82F6),
            divider_color: rgb(0xE2E8F0),
        }
    }
}

impl EditPeerMembersPeersStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn row_height(&self) -> Pixels {
        self.rights_row_height
    }
}
