//! MenuSend: Extended message send menu (Send without sound, Schedule message, Send when online)
//! Upstream: Telegram/SourceFiles/menu/menu_send.h, menu_send.cpp

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SendOption {
    SendSilently,
    ScheduleMessage,
    SendWhenOnline,
}

#[derive(Debug, Clone)]
pub struct MenuSend {
    pub is_visible: bool,
    pub peer_is_user: bool,
}

impl MenuSend {
    pub fn new() -> Self {
        Self {
            is_visible: false,
            peer_is_user: true,
        }
    }

    pub fn for_context(is_user: bool) -> Self {
        Self {
            is_visible: true,
            peer_is_user: is_user,
        }
    }

    pub fn available_options(&self) -> Vec<SendOption> {
        let mut opts = vec![SendOption::SendSilently, SendOption::ScheduleMessage];
        if self.peer_is_user {
            opts.push(SendOption::SendWhenOnline);
        }
        opts
    }

    pub fn dismiss(&mut self) {
        self.is_visible = false;
    }
}

impl Default for MenuSend {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for MenuSend {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }
        div()
            .flex()
            .flex_col()
            .p(px(4.0))
            .bg(rgb(0x17212B))
            .rounded(px(6.0))
            .shadow_md()
            .children(self.available_options().iter().map(|opt| {
                div()
                    .px(px(12.0))
                    .py(px(6.0))
                    .text_xs()
                    .text_color(rgb(0xFFFFFF))
                    .child(match opt {
                        SendOption::SendSilently => "Send Without Sound",
                        SendOption::ScheduleMessage => "Schedule Message...",
                        SendOption::SendWhenOnline => "Send When Online",
                    })
            }))
            .into_any_element()
    }
}
