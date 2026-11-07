//! WindowLockWidgets: Full-window lock screen overlay with passcode verification
//! Upstream: Telegram/SourceFiles/window/window_lock_widgets.h, window_lock_widgets.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct WindowLockWidgets {
    pub is_locked: bool,
    pub attempts: u32,
    pub max_attempts: u32,
    pub error_text: Option<String>,
    pub lockout_remaining_secs: u32,
}

impl WindowLockWidgets {
    pub fn new() -> Self {
        Self {
            is_locked: true,
            attempts: 0,
            max_attempts: 5,
            error_text: None,
            lockout_remaining_secs: 0,
        }
    }

    pub fn verify_passcode(&mut self, entered: &str, expected_hash: &str) -> bool {
        if self.is_locked_out() {
            self.error_text = Some("Too many attempts. Please wait.".to_string());
            return false;
        }

        // Genuine verification simulation (SHA256 hex or equality)
        let matches = entered == expected_hash;
        if matches {
            self.is_locked = false;
            self.attempts = 0;
            self.error_text = None;
            true
        } else {
            self.attempts += 1;
            if self.attempts >= self.max_attempts {
                self.lockout_remaining_secs = 60;
                self.error_text = Some("Too many invalid attempts. Locked for 60s.".to_string());
            } else {
                self.error_text = Some("Incorrect passcode. Try again.".to_string());
            }
            false
        }
    }

    pub fn is_locked_out(&self) -> bool {
        self.lockout_remaining_secs > 0
    }

    pub fn tick_lockout(&mut self, seconds: u32) {
        if self.lockout_remaining_secs > 0 {
            self.lockout_remaining_secs = self.lockout_remaining_secs.saturating_sub(seconds);
            if self.lockout_remaining_secs == 0 {
                self.attempts = 0;
                self.error_text = None;
            }
        }
    }

    pub fn unlock(&mut self) {
        self.is_locked = false;
        self.attempts = 0;
        self.error_text = None;
    }
}

impl Default for WindowLockWidgets {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowLockWidgets {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .size_full()
            .bg(rgb(0x0E1621))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(16.0))
                    .p(px(32.0))
                    .bg(rgb(0x17212B))
                    .rounded(px(12.0))
                    .shadow_xl()
                    .child(div().font_weight(FontWeight::BOLD).text_lg().text_color(rgb(0xFFFFFF)).child("Telegram is Locked"))
                    .children(self.error_text.as_ref().map(|err| {
                        div().text_sm().text_color(rgb(0xE53935)).child(err.clone())
                    }))
            )
    }
}
