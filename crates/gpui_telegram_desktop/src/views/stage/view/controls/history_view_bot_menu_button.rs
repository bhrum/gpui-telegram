//! Bot menu button with dynamic text size animation and custom emoji support.
//! Upstream reference: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_bot_menu_button.cpp`
//! Upstream header: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_bot_menu_button.h`

use gpui::*;

/// Authentic bot menu button model and GPUI component
pub struct HistoryViewBotMenuButtonViewControls {
    pub text: String,
    pub is_small: bool,
    pub is_active: bool,
    pub width_px: f32,
    pub emoji_id: Option<u64>,
    pub bot_id: u64,
    pub bot_username: String,
    pub menu_url: String,
    pub accent_color: u32,
    pub pending_launch: Option<(u64, String, String, String)>,
}

impl HistoryViewBotMenuButtonViewControls {
    pub fn new() -> Self {
        Self {
            text: "Menu".to_string(),
            is_small: false,
            is_active: false,
            width_px: 80.0,
            emoji_id: None,
            bot_id: 0,
            bot_username: String::new(),
            menu_url: String::new(),
            accent_color: 0x2481cc,
            pending_launch: None,
        }
    }

    /// Refresh button text and small flag, recalculating animated width
    /// Upstream: `BotMenuButton::refresh` (`history_view_bot_menu_button.cpp:L70-85`)
    pub fn refresh(&mut self, text: impl Into<String>, small: bool) -> bool {
        let new_text = text.into();
        let changed = self.text != new_text || self.is_small != small;
        self.text = new_text;
        self.is_small = small;
        self.width_px = if small { 36.0 } else { 24.0 + (self.text.len() as f32 * 8.0).max(48.0) };
        changed
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    /// Binds this control to a concrete bot's WebApp menu entry (`[☰ 打开小程序 / Open App]`).
    pub fn set_bot_menu(
        &mut self,
        bot_id: u64,
        bot_username: impl Into<String>,
        text: impl Into<String>,
        menu_url: impl Into<String>,
    ) {
        self.bot_id = bot_id;
        self.bot_username = bot_username.into();
        self.menu_url = menu_url.into();
        self.is_active = !self.menu_url.trim().is_empty();
        let label = text.into();
        self.refresh(if label.trim().is_empty() { "Open App".to_string() } else { label }, false);
    }

    pub fn clear_bot_menu(&mut self) {
        self.bot_id = 0;
        self.bot_username.clear();
        self.menu_url.clear();
        self.is_active = false;
        self.pending_launch = None;
    }

    pub fn take_launch(&mut self) -> Option<(u64, String, String, String)> {
        self.pending_launch.take()
    }
}

impl Default for HistoryViewBotMenuButtonViewControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewBotMenuButtonViewControls {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_small = self.is_small;
        let text = self.text.clone();
        let active_bg = if self.is_active { self.accent_color } else { 0x242f3d };
        let bid = self.bot_id;
        let buser = self.bot_username.clone();
        let btitle = self.text.clone();
        let burl = self.menu_url.clone();

        let mut inner = div()
            .flex()
            .items_center()
            .gap_1()
            .child("☰");

        if !is_small {
            inner = inner.child(text);
        }

        div()
            .id("composer-bot-menu-btn")
            .cursor_pointer()
            .flex_shrink_0()
            .flex()
            .items_center()
            .justify_center()
            .h(px(34.0))
            .px(px(12.0))
            .rounded_full()
            .bg(rgb(active_bg))
            .text_xs()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(rgb(0xffffff))
            .hover(|s| s.bg(rgb(0x2b8be0)))
            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _ev, _w, cx| {
                if !burl.trim().is_empty() || !buser.trim().is_empty() {
                    this.pending_launch = Some((bid, buser.clone(), btitle.clone(), burl.clone()));
                    cx.notify();
                }
            }))
            .child(inner)
    }
}
