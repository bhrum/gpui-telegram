//! Authentic GPUI theme stylesheet for ModerateMessagesBox.
//! Upstream reference: `Telegram/SourceFiles/boxes/moderate_messages_box.style:L1-L46`.

use gpui::{px, rgb, Pixels, Rgba};

#[derive(Debug, Clone)]
pub struct ModerateMessagesBoxStyleSheet {
    pub checkbox_size: Pixels,
    pub reason_padding: (Pixels, Pixels, Pixels, Pixels),
    pub options_spacing: Pixels,
    pub ban_option_fg: Rgba,
    pub report_spam_fg: Rgba,
    pub delete_all_fg: Rgba,
    pub warning_fg: Rgba,
    pub border_color: Rgba,
}

impl Default for ModerateMessagesBoxStyleSheet {
    fn default() -> Self {
        Self {
            checkbox_size: px(20.0),
            reason_padding: (px(12.0), px(8.0), px(12.0), px(8.0)),
            options_spacing: px(14.0),
            ban_option_fg: rgb(0xDF3F40),
            report_spam_fg: rgb(0xF59E0B),
            delete_all_fg: rgb(0xDC2626),
            warning_fg: rgb(0xEF4444),
            border_color: rgb(0xE2E8F0),
        }
    }
}

impl ModerateMessagesBoxStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spacing(&self) -> Pixels {
        self.options_spacing
    }
}
