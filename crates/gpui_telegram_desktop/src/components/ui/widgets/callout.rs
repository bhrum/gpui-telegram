//! Callout Reusable Widget
//! Upstream: Telegram/SourceFiles/ui/widgets/glare_tooltip.cpp:L18-L60
//! Upstream: Telegram/SourceFiles/ui/widgets/peer_bubble.cpp:L23-L60
//! Implements inline callout notification cards with severity status and action buttons.

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalloutSeverity {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct Callout {
    pub title: String,
    pub message: String,
    pub severity: CalloutSeverity,
    pub is_dismissable: bool,
    pub is_dismissed: bool,
}

impl Callout {
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            severity: CalloutSeverity::Info,
            is_dismissable: true,
            is_dismissed: false,
        }
    }

    pub fn with_severity(mut self, severity: CalloutSeverity) -> Self {
        self.severity = severity;
        self
    }

    pub fn dismiss(&mut self) {
        self.is_dismissed = true;
    }

    pub fn severity_colors(&self) -> (u32, u32, u32) {
        // (background, border, icon/title)
        match self.severity {
            CalloutSeverity::Info => (0xE8F4FD, 0x2AABEE, 0x2AABEE),
            CalloutSeverity::Success => (0xEAF8EE, 0x34C759, 0x248A3D),
            CalloutSeverity::Warning => (0xFFF8E7, 0xFF9500, 0xC67600),
            CalloutSeverity::Error => (0xFDE8E8, 0xFF3B30, 0xD70015),
        }
    }
}

impl Default for Callout {
    fn default() -> Self {
        Self::new("Notice", "")
    }
}

impl Render for Callout {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_dismissed {
            return div().into_any_element();
        }

        let (bg, border, fg) = self.severity_colors();

        let mut root = div()
            .flex()
            .items_start()
            .justify_between()
            .w_full()
            .p(px(12.0))
            .bg(rgb(bg))
            .border_1()
            .border_color(rgb(border))
            .rounded(px(8.0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .child(
                        div()
                            .font_weight(FontWeight::BOLD)
                            .text_size(px(14.0))
                            .text_color(rgb(fg))
                            .child(self.title.clone()),
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(rgb(0x333333))
                            .child(self.message.clone()),
                    ),
            );

        if self.is_dismissable {
            root = root.child(
                div()
                    .w(px(20.0))
                    .h(px(20.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(12.0))
                    .text_color(rgb(0x888888))
                    .cursor_pointer()
                    .child("✕"),
            );
        }
        root.into_any_element()
    }
}
