//! Info Profile Icon component
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_icon.cpp:L1-L40`
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_icon.h:L1-L40`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;

/// Small icon tag rendered in profile cards for specialized roles or status.
#[derive(Debug, Clone)]
pub struct InfoProfileIconProfile {
    pub icon_name: String,
    pub is_active: bool,
    pub tint_color: u32,
}

impl InfoProfileIconProfile {
    pub fn new(icon_name: impl Into<String>) -> Self {
        Self {
            icon_name: icon_name.into(),
            is_active: true,
            tint_color: 0x5288c1,
        }
    }

    pub fn set_tint(&mut self, color: u32) {
        self.tint_color = color;
    }
}

impl Default for InfoProfileIconProfile {
    fn default() -> Self {
        Self::new("info")
    }
}

impl Render for InfoProfileIconProfile {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_center()
            .w(px(20.0))
            .h(px(20.0))
            .text_color(rgb(self.tint_color))
            .child(self.icon_name.clone())
    }
}
