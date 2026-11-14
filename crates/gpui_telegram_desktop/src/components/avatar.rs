use crate::theme::TdesktopThemePalette;
use gpui::prelude::FluentBuilder;
use gpui::*;

pub struct Avatar {
    pub label: String,
    pub size: f32,
    pub color: u32,
    pub avatar_url: Option<String>,
    pub is_online: bool,
    pub palette: TdesktopThemePalette,
}

impl Avatar {
    pub fn new(label: impl Into<String>, size: f32) -> Self {
        Self {
            label: label.into(),
            size,
            color: 0x2AABEE,
            avatar_url: None,
            is_online: false,
            palette: TdesktopThemePalette::default(),
        }
    }

    pub fn with_color(mut self, color: u32) -> Self {
        self.color = color;
        self
    }

    pub fn with_image_url(mut self, url: Option<String>) -> Self {
        self.avatar_url = url.filter(|u| !u.trim().is_empty());
        self
    }

    pub fn with_online(mut self, is_online: bool) -> Self {
        self.is_online = is_online;
        self
    }

    pub fn with_palette(mut self, palette: TdesktopThemePalette) -> Self {
        self.palette = palette;
        self
    }

    fn initials(&self) -> String {
        let mut out = String::new();
        for part in self.label.split_whitespace().take(2) {
            if let Some(c) = part.chars().next() {
                out.push(c);
            }
        }
        if out.is_empty() {
            out.push('?');
        }
        out.to_uppercase()
    }
}

impl Render for Avatar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;
        let initials = self.initials();
        let dot_size = (self.size * 0.26).clamp(8.0, 14.0);

        div()
            .relative()
            .w(px(self.size))
            .h(px(self.size))
            .rounded_full()
            .bg(rgb(self.color))
            .flex()
            .items_center()
            .justify_center()
            .text_color(rgb(pal.active_button_fg))
            .text_size(px((self.size * 0.36).clamp(11.0, 26.0)))
            .font_weight(FontWeight::BOLD)
            .map(|el| {
                if let Some(ref url) = self.avatar_url {
                    el.overflow_hidden()
                        .child(img(url.clone()).w_full().h_full().object_fit(ObjectFit::Cover))
                } else {
                    el.child(initials)
                }
            })
            .when(self.is_online, |el| {
                el.child(
                    div()
                        .absolute()
                        .right(px(0.0))
                        .bottom(px(0.0))
                        .w(px(dot_size))
                        .h(px(dot_size))
                        .rounded_full()
                        .bg(rgb(pal.online_dot_fg))
                        .border_2()
                        .border_color(rgb(pal.dialogs_bg)),
                )
            })
    }
}
