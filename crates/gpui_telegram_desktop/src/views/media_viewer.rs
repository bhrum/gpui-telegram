//! Media Viewer Lightbox Overlay (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.6).
//!
//! Sub-components:
//! 1. Fullscreen Dark Semi-Transparent Backdrop (rgba(0, 0, 0, 0.94)).
//! 2. Top Header (Counter e.g. "3 of 12", sender name, date, zoom buttons, download, close).
//! 3. Centered Media Viewport (Photo/Video with zoom level 100%-200% and video player controls).
//! 4. Navigation Chevrons (◀ Previous, ▶ Next).
//! 5. Bottom Thumbnail Strip.

use gpui::*;
use gpui::prelude::FluentBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MediaViewerItem {
    pub id: u64,
    pub url: String,
    pub is_video: bool,
    pub caption: Option<String>,
    pub sender_name: String,
    pub date: String,
    pub duration_secs: Option<u32>,
}

pub struct MediaViewer {
    pub items: Vec<MediaViewerItem>,
    pub current_index: usize,
    pub zoom_level: f32,
    pub is_playing: bool,
    pub is_open: bool,
    pub should_close: bool,
}

impl MediaViewer {
    pub fn new(media_url: String, is_video: bool) -> Self {
        let items = vec![
            MediaViewerItem {
                id: 1,
                url: media_url.clone(),
                is_video,
                caption: Some("High-resolution desktop view layout mockups".to_string()),
                sender_name: "Alice Smith".to_string(),
                date: "Today at 14:32".to_string(),
                duration_secs: if is_video { Some(124) } else { None },
            },
            MediaViewerItem {
                id: 2,
                url: "https://example.com/photo2.jpg".to_string(),
                is_video: false,
                caption: Some("Component hierarchy architecture diagram".to_string()),
                sender_name: "Alice Smith".to_string(),
                date: "Today at 14:20".to_string(),
                duration_secs: None,
            },
            MediaViewerItem {
                id: 3,
                url: "https://example.com/video1.mp4".to_string(),
                is_video: true,
                caption: Some("60fps Talk Mode and remote computer recording".to_string()),
                sender_name: "You".to_string(),
                date: "Yesterday".to_string(),
                duration_secs: Some(85),
            },
        ];

        Self {
            items,
            current_index: 0,
            zoom_level: 1.0,
            is_playing: false,
            is_open: true,
            should_close: false,
        }
    }

    pub fn next(&mut self) {
        if !self.items.is_empty() {
            self.current_index = (self.current_index + 1) % self.items.len();
            self.zoom_level = 1.0;
        }
    }

    pub fn prev(&mut self) {
        if !self.items.is_empty() {
            self.current_index = if self.current_index == 0 {
                self.items.len() - 1
            } else {
                self.current_index - 1
            };
            self.zoom_level = 1.0;
        }
    }

    pub fn zoom_in(&mut self) {
        self.zoom_level = (self.zoom_level + 0.25).min(3.0);
    }

    pub fn zoom_out(&mut self) {
        self.zoom_level = (self.zoom_level - 0.25).max(0.5);
    }

    pub fn reset_zoom(&mut self) {
        self.zoom_level = 1.0;
    }

    pub fn toggle_play(&mut self) {
        self.is_playing = !self.is_playing;
    }

    pub fn current_item(&self) -> Option<&MediaViewerItem> {
        self.items.get(self.current_index)
    }
}

impl Default for MediaViewer {
    fn default() -> Self {
        Self::new("https://example.com/demo.jpg".to_string(), false)
    }
}

impl Render for MediaViewer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let total = self.items.len();
        let current = self.current_index;
        let item = self.current_item().cloned();
        let items_snapshot = self.items.clone();

        div()
            .size_full()
            .bg(rgba(0x000000F0)) // 94% opacity dark lightbox backdrop
            .flex()
            .flex_col()
            .justify_between()
            // 1. Lightbox Top Header (height 52px)
            .child(
                div()
                    .h(px(52.0))
                    .w_full()
                    .px(px(20.0))
                    .bg(rgba(0x00000066))
                    .flex()
                    .items_center()
                    .justify_between()
                    // Counter & Metadata (e.g. "1 of 3 • Alice Smith • Today at 14:32")
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .child(format!("{} of {}", current + 1, total)),
                            )
                            .when_some(item.as_ref(), |el, it| {
                                el.child(
                                    div()
                                        .text_size(px(12.0))
                                        .text_color(rgb(0x8A9AA8))
                                        .child(format!("{} • {}", it.sender_name, it.date)),
                                )
                            }),
                    )
                    // Controls: Zoom Out, Zoom Reset, Zoom In, Download, Close
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .cursor_pointer()
                                    .px(px(8.0))
                                    .py(px(4.0))
                                    .rounded(px(4.0))
                                    .bg(rgba(0xFFFFFF11))
                                    .hover(|s| s.bg(rgba(0xFFFFFF33)))
                                    .text_size(px(12.0))
                                    .text_color(rgb(0xFFFFFF))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.reset_zoom();
                                        cx.notify();
                                    }))
                                    .child(format!("{:.0}%", self.zoom_level * 100.0)),
                            )
                            .child(
                                div()
                                    .cursor_pointer()
                                    .px(px(6.0))
                                    .py(px(2.0))
                                    .rounded(px(4.0))
                                    .hover(|s| s.bg(rgba(0xFFFFFF22)))
                                    .text_size(px(16.0))
                                    .text_color(rgb(0xFFFFFF))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.zoom_out();
                                        cx.notify();
                                    }))
                                    .child("🔍-"),
                            )
                            .child(
                                div()
                                    .cursor_pointer()
                                    .px(px(6.0))
                                    .py(px(2.0))
                                    .rounded(px(4.0))
                                    .hover(|s| s.bg(rgba(0xFFFFFF22)))
                                    .text_size(px(16.0))
                                    .text_color(rgb(0xFFFFFF))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.zoom_in();
                                        cx.notify();
                                    }))
                                    .child("🔍+"),
                            )
                            .child(
                                div()
                                    .cursor_pointer()
                                    .px(px(8.0))
                                    .py(px(4.0))
                                    .rounded(px(4.0))
                                    .bg(rgba(0xE5393544))
                                    .hover(|s| s.bg(rgb(0xE53935)))
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.should_close = true;
                                        cx.notify();
                                    }))
                                    .child("✕ Close"),
                            ),
                    ),
            )
            // 2. Center Media Viewport with Navigation Arrows
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px(px(24.0))
                    // Previous Chevron
                    .child(
                        div()
                            .cursor_pointer()
                            .w(px(48.0))
                            .h(px(48.0))
                            .rounded_full()
                            .bg(rgba(0xFFFFFF22))
                            .hover(|s| s.bg(rgb(0x2AABEE)))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(20.0))
                            .text_color(rgb(0xFFFFFF))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.prev();
                                cx.notify();
                            }))
                            .child("◀"),
                    )
                    // Centered Media Preview Canvas
                    .child(
                        div()
                            .cursor_pointer()
                            .relative()
                            .w(px(640.0 * self.zoom_level))
                            .h(px(400.0 * self.zoom_level))
                            .rounded(px(8.0))
                            .bg(rgb(0x18222D))
                            .flex()
                            .items_center()
                            .justify_center()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.toggle_play();
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .text_size(px(64.0))
                                    .child(if item.as_ref().map_or(false, |it| it.is_video) { "🎬" } else { "🖼️" }),
                            )
                            // Video Play Button Overlay
                            .when(item.as_ref().map_or(false, |it| it.is_video), |el| {
                                el.child(
                                    div()
                                        .w(px(56.0))
                                        .h(px(56.0))
                                        .rounded_full()
                                        .bg(rgba(0x00000088))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_size(px(24.0))
                                        .text_color(rgb(0xFFFFFF))
                                        .child(if self.is_playing { "❚❚" } else { "▶" }),
                                )
                            }),
                    )
                    // Next Chevron
                    .child(
                        div()
                            .cursor_pointer()
                            .w(px(48.0))
                            .h(px(48.0))
                            .rounded_full()
                            .bg(rgba(0xFFFFFF22))
                            .hover(|s| s.bg(rgb(0x2AABEE)))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(20.0))
                            .text_color(rgb(0xFFFFFF))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.next();
                                cx.notify();
                            }))
                            .child("▶"),
                    ),
            )
            // 3. Bottom Strip: Caption + Thumbnail Strip
            .child(
                div()
                    .h(px(80.0))
                    .w_full()
                    .bg(rgba(0x00000066))
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap(px(6.0))
                    // Caption
                    .when_some(item.as_ref().and_then(|it| it.caption.as_ref()), |el, cap| {
                        el.child(
                            div()
                                .text_size(px(13.0))
                                .text_color(rgb(0xFFFFFF))
                                .child(cap.clone()),
                        )
                    })
                    // Thumbnails Row
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .children(items_snapshot.into_iter().enumerate().map(|(idx, it)| {
                                let is_active = idx == current;
                                div()
                                    .cursor_pointer()
                                    .w(px(44.0))
                                    .h(px(32.0))
                                    .rounded(px(4.0))
                                    .bg(rgb(0x243447))
                                    .border_2()
                                    .border_color(if is_active { rgb(0x2AABEE) } else { rgba(0x00000000) })
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_size(px(12.0))
                                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                        this.current_index = idx;
                                        this.zoom_level = 1.0;
                                        cx.notify();
                                    }))
                                    .child(if it.is_video { "🎬" } else { "🖼️" })
                            })),
                    ),
            )
    }
}
