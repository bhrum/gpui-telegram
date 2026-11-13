//! NotificationsUtilities: Text sanitization, privacy masking, and formatting for desktop alerts
//! Upstream: Telegram/SourceFiles/window/notifications_utilities.h, notifications_utilities.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct NotificationsUtilities;

impl NotificationsUtilities {
    pub fn new() -> Self {
        Self
    }

    pub fn format_body(body: &str, max_len: usize) -> String {
        let trimmed = body.trim();
        if trimmed.chars().count() > max_len {
            let truncated: String = trimmed.chars().take(max_len).collect();
            format!("{}...", truncated)
        } else {
            trimmed.to_string()
        }
    }

    pub fn sanitize(sender: &str, body: &str, show_preview: bool) -> (String, String) {
        if show_preview {
            (sender.to_string(), Self::format_body(body, 120))
        } else {
            (sender.to_string(), "New message".to_string())
        }
    }
}

impl Default for NotificationsUtilities {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for NotificationsUtilities {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0x82868A))
            .child("NotificationsUtilities")
    }
}
