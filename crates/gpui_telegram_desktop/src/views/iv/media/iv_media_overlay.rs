//! Media overlay component for Instant View
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

#[derive(Debug, Clone, Default)]
pub struct IvMediaOverlay {
    pub is_open: bool,
    pub media_url: String,
    pub caption: String,
    pub zoom_level: f32,
    pub is_video: bool,
}

impl IvMediaOverlay {
    pub fn new() -> Self {
        Self {
            is_open: false,
            media_url: String::new(),
            caption: String::new(),
            zoom_level: 1.0,
            is_video: false,
        }
    }

    pub fn show(&mut self, url: impl Into<String>, caption: impl Into<String>, is_video: bool) {
        self.is_open = true;
        self.media_url = url.into();
        self.caption = caption.into();
        self.zoom_level = 1.0;
        self.is_video = is_video;
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.zoom_level = 1.0;
    }

    pub fn zoom_in(&mut self) {
        self.zoom_level = (self.zoom_level + 0.25).min(4.0);
    }

    pub fn zoom_out(&mut self) {
        self.zoom_level = (self.zoom_level - 0.25).max(0.5);
    }
}

impl Render for IvMediaOverlay {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_open {
            return div();
        }

        let caption = self.caption.clone();
        let zoom = (self.zoom_level * 100.0) as i32;

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .p_4()
            .bg(rgba(0x000000ee))
            .text_color(rgb(0xffffff))
            .child(
                div()
                    .flex()
                    .w_full()
                    .justify_between()
                    .child(div().font_weight(FontWeight::BOLD).child("Media Overlay"))
                    .child(div().text_sm().child(format!("Zoom: {}%", zoom))),
            )
            .child(
                div()
                    .my_8()
                    .p_6()
                    .border_1()
                    .border_color(rgb(0x444444))
                    .rounded_md()
                    .child(if self.is_video { "Video Media Frame" } else { "Image Media Frame" }),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0xaaaaaa))
                    .child(caption),
            )
    }
}
