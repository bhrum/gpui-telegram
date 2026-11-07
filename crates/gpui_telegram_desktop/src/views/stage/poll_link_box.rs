//! Poll Option Link Attachment Dialog Box.
//!
//! Citations:
//! - Upstream: `Telegram/SourceFiles/poll/poll_link_box.h:L14-L22`
//! - Upstream: `Telegram/SourceFiles/poll/poll_link_box.cpp:L19-L65`

use gpui::*;

/// State and modal prompt for attaching external hyperlinks to poll options.
/// C++ citation: `Poll::AddPollOptionLinkBox` (poll_link_box.cpp:L21-L60).
pub struct PollLinkBox {
    pub title: String,
    pub description: String,
    pub url_input: String,
    pub initial_url: String,
    pub is_open: bool,
    pub has_error: bool,
    pub error_message: Option<String>,
}

impl PollLinkBox {
    pub fn new(initial: impl Into<String>) -> Self {
        let init = initial.into();
        Self {
            title: "Add Link to Option".to_string(), // lng_polls_create_option_link_title
            description: "Enter a valid URL for this poll option:".to_string(), // lng_polls_create_option_link_description
            url_input: init.clone(),
            initial_url: init,
            is_open: true,
            has_error: false,
            error_message: None,
        }
    }

    pub fn set_input(&mut self, text: impl Into<String>) {
        self.url_input = text.into();
        self.has_error = false;
        self.error_message = None;
    }

    /// Validates and submits the link.
    /// C++ citation: `poll_link_box.cpp:L49-L60`.
    pub fn submit(&mut self) -> Result<String, &'static str> {
        let trimmed = self.url_input.trim();
        if trimmed.is_empty() {
            self.has_error = true;
            self.error_message = Some("URL cannot be empty".to_string());
            return Err("URL cannot be empty");
        }
        if !trimmed.starts_with("http://") && !trimmed.starts_with("https://") && !trimmed.starts_with("tg://") {
            self.has_error = true;
            self.error_message = Some("URL must begin with http://, https://, or tg://".to_string());
            return Err("Invalid URL protocol");
        }
        self.is_open = false;
        Ok(trimmed.to_string())
    }

    pub fn cancel(&mut self) {
        self.is_open = false;
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }
}

impl Default for PollLinkBox {
    fn default() -> Self {
        Self::new("")
    }
}

impl Render for PollLinkBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_open {
            return div().into_any_element();
        }

        div()
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgba(0x00000066))
            .child(
                div()
                    .w(px(380.0))
                    .p(px(20.0))
                    .bg(rgb(0xFFFFFF))
                    .rounded(px(12.0))
                    .shadow_xl()
                    .child(
                        div()
                            .font_weight(FontWeight::BOLD)
                            .text_size(px(16.0))
                            .text_color(rgb(0x18181B))
                            .child(self.title.clone())
                    )
                    .child(
                        div()
                            .mt(px(8.0))
                            .text_size(px(13.0))
                            .text_color(rgb(0x71717A))
                            .child(self.description.clone())
                    )
                    .child(
                        div()
                            .mt(px(16.0))
                            .p(px(8.0))
                            .border_1()
                            .border_color(if self.has_error { rgb(0xEF4444) } else { rgb(0xD4D4D8) })
                            .rounded(px(6.0))
                            .child(if self.url_input.is_empty() {
                                "https://example.com".to_string()
                            } else {
                                self.url_input.clone()
                            })
                    )
            )
            .into_any_element()
    }
}
