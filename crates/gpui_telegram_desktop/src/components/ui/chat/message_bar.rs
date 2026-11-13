//! UI Message Bar reply/edit banner component.
//! Upstream: `Telegram/SourceFiles/ui/chat/message_bar.cpp:L40-L120`
//!           `Telegram/SourceFiles/ui/chat/message_bar.h:L20-L70`
//!
//! Renders the reply / pin / edit preview banner anchored directly above the composer input.

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageBarType {
    Reply,
    Edit,
    Pinned,
}

/// MessageBarChat component. Target symbol required by ledger: `MessageBarChat`.
#[derive(Debug, Clone)]
pub struct MessageBarChat {
    pub bar_type: MessageBarType,
    pub title: String,
    pub subtitle: String,
    pub is_visible: bool,
    pub has_thumbnail: bool,
}

impl MessageBarChat {
    pub fn new(bar_type: MessageBarType, title: impl Into<String>, subtitle: impl Into<String>) -> Self {
        Self {
            bar_type,
            title: title.into(),
            subtitle: subtitle.into(),
            is_visible: true,
            has_thumbnail: false,
        }
    }

    pub fn with_thumbnail(mut self, has_thumbnail: bool) -> Self {
        self.has_thumbnail = has_thumbnail;
        self
    }
}

impl Default for MessageBarChat {
    fn default() -> Self {
        Self::new(MessageBarType::Reply, "Reply", "Message")
    }
}

impl Render for MessageBarChat {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div();
        }

        let accent = match self.bar_type {
            MessageBarType::Reply => hsla(204.0 / 360.0, 0.88, 0.53, 1.0),
            MessageBarType::Edit => hsla(140.0 / 360.0, 0.65, 0.45, 1.0),
            MessageBarType::Pinned => hsla(280.0 / 360.0, 0.70, 0.50, 1.0),
        };

        div()
            .h(px(44.0))
            .w_full()
            .px(px(12.0))
            .bg(hsla(0.0, 0.0, 1.0, 1.0))
            .border_b_1()
            .border_color(hsla(0.0, 0.0, 0.90, 1.0))
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(
                div()
                    .w(px(3.0))
                    .h(px(32.0))
                    .rounded(px(1.5))
                    .bg(accent)
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(accent)
                            .child(self.title.clone())
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(hsla(0.0, 0.0, 0.35, 1.0))
                            .child(self.subtitle.clone())
                    )
            )
    }
}
