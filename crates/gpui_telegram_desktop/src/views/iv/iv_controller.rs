//! Target Symbol: IvController
//! Module 13: Instant View & TON Site Window Controller
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const ZOOM_STEP: i32 = 10;
pub const MIN_ZOOM: i32 = 50;
pub const MAX_ZOOM: i32 = 300;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControllerEventType {
    Close,
    Quit,
    OpenChannel(String),
    JoinChannel(String),
    OpenPage(String),
    OpenLink(String),
    OpenLinkExternal(String),
    OpenMedia(String),
    Report(String),
}

#[derive(Debug, Clone)]
pub struct ControllerEvent {
    pub event_type: ControllerEventType,
    pub url: String,
    pub webpage_id: u64,
}

#[derive(Debug, Clone)]
pub struct IvController {
    pub current_url: String,
    pub page_title: String,
    pub subtitle: String,
    pub zoom: i32,
    pub history: Vec<String>,
    pub history_index: usize,
    pub events_log: Vec<ControllerEvent>,
    pub is_active: bool,
}

impl Default for IvController {
    fn default() -> Self {
        Self {
            current_url: String::new(),
            page_title: "Instant View".to_string(),
            subtitle: String::new(),
            zoom: 100,
            history: Vec::new(),
            history_index: 0,
            events_log: Vec::new(),
            is_active: false,
        }
    }
}

impl IvController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_good_ton_site_url(uri: &str) -> bool {
        let lower = uri.to_lowercase();
        if lower.starts_with("ton://") {
            return true;
        }
        if let Some(host) = lower
            .strip_prefix("http://")
            .or_else(|| lower.strip_prefix("https://"))
        {
            let host_part = host.split('/').next().unwrap_or("");
            return host_part.ends_with(".ton") || host_part.contains(".ton:");
        }
        false
    }

    pub fn navigate_to(&mut self, url: impl Into<String>) {
        let u = url.into();
        self.history.truncate(self.history_index.saturating_add(1));
        self.history.push(u.clone());
        self.history_index = self.history.len().saturating_sub(1);
        self.current_url = u.clone();
        self.is_active = true;
        self.emit_event(ControllerEventType::OpenPage(u.clone()), u, 0);
    }

    pub fn can_go_back(&self) -> bool {
        self.history_index > 0
    }

    pub fn can_go_forward(&self) -> bool {
        self.history_index + 1 < self.history.len()
    }

    pub fn go_back(&mut self) -> bool {
        if self.can_go_back() {
            self.history_index -= 1;
            self.current_url = self.history[self.history_index].clone();
            true
        } else {
            false
        }
    }

    pub fn go_forward(&mut self) -> bool {
        if self.can_go_forward() {
            self.history_index += 1;
            self.current_url = self.history[self.history_index].clone();
            true
        } else {
            false
        }
    }

    pub fn zoom_in(&mut self) {
        self.zoom = (self.zoom + ZOOM_STEP).min(MAX_ZOOM);
    }

    pub fn zoom_out(&mut self) {
        self.zoom = (self.zoom - ZOOM_STEP).max(MIN_ZOOM);
    }

    pub fn reset_zoom(&mut self) {
        self.zoom = 100;
    }

    pub fn emit_event(&mut self, event_type: ControllerEventType, url: impl Into<String>, webpage_id: u64) {
        self.events_log.push(ControllerEvent {
            event_type,
            url: url.into(),
            webpage_id,
        });
    }
}

impl Render for IvController {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let url_display = if self.current_url.is_empty() {
            "No page loaded".to_string()
        } else {
            self.current_url.clone()
        };

        div()
            .flex()
            .flex_col()
            .w_full()
            .bg(rgb(0x1E222A))
            .border_b_1()
            .border_color(rgb(0x2E3440))
            .p(px(8.0))
            .gap(px(6.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(px(6.0))
                            .items_center()
                            .child(
                                div()
                                    .px(px(8.0))
                                    .py(px(4.0))
                                    .rounded(px(4.0))
                                    .bg(if self.can_go_back() { rgb(0x2E3440) } else { rgb(0x1A1D24) })
                                    .text_color(if self.can_go_back() { rgb(0xD8DEE9) } else { rgb(0x4C566A) })
                                    .child("◀"),
                            )
                            .child(
                                div()
                                    .px(px(8.0))
                                    .py(px(4.0))
                                    .rounded(px(4.0))
                                    .bg(if self.can_go_forward() { rgb(0x2E3440) } else { rgb(0x1A1D24) })
                                    .text_color(if self.can_go_forward() { rgb(0xD8DEE9) } else { rgb(0x4C566A) })
                                    .child("▶"),
                            ),
                    )
                    // URL display
                    .child(
                        div()
                            .flex_1()
                            .mx(px(12.0))
                            .px(px(10.0))
                            .py(px(4.0))
                            .rounded(px(4.0))
                            .bg(rgb(0x14161C))
                            .text_size(px(12.0))
                            .text_color(rgb(0x88C0D0))
                            .child(url_display),
                    )
                    // Zoom indicator
                    .child(
                        div()
                            .px(px(8.0))
                            .py(px(4.0))
                            .rounded(px(4.0))
                            .bg(rgb(0x2E3440))
                            .text_size(px(12.0))
                            .text_color(rgb(0xECEFF4))
                            .child(format!("{}%", self.zoom)),
                    ),
            )
    }
}
