//! Authentic GPUI theme stylesheet for ComposeAiBox.
//! Upstream reference: `Telegram/SourceFiles/boxes/compose_ai_box.style:L1-L190`.

use gpui::{px, rgb, Pixels, Rgba};

#[derive(Debug, Clone)]
pub struct ComposeAiBoxStyleSheet {
    pub tabs_skip: Pixels,
    pub tabs_height: Pixels,
    pub tabs_radius: Pixels,
    pub prompt_height: Pixels,
    pub button_height: Pixels,
    pub tone_pill_padding: (Pixels, Pixels),
    pub tabs_bg: Rgba,
    pub ai_accent: Rgba,
    pub prompt_bg: Rgba,
    pub text_fg: Rgba,
    pub subtext_fg: Rgba,
}

impl Default for ComposeAiBoxStyleSheet {
    fn default() -> Self {
        Self {
            tabs_skip: px(8.0),
            tabs_height: px(58.0),
            tabs_radius: px(29.0),
            prompt_height: px(96.0),
            button_height: px(40.0),
            tone_pill_padding: (px(12.0), px(6.0)),
            tabs_bg: rgb(0xF4F4F5),
            ai_accent: rgb(0x7158E8),
            prompt_bg: rgb(0xFFFFFF),
            text_fg: rgb(0x1F2937),
            subtext_fg: rgb(0x6B7280),
        }
    }
}

impl ComposeAiBoxStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tabs_height(&self) -> Pixels {
        self.tabs_height
    }

    pub fn tabs_radius(&self) -> Pixels {
        self.tabs_radius
    }
}
