//! Authentic GPUI implementation of Telegram Desktop's Background Wallpapers Box.
//! Upstream reference: `Telegram/SourceFiles/boxes/background_box.h:L22-L51`
//! Upstream implementation: `Telegram/SourceFiles/boxes/background_box.cpp:L38-L60`, `L350-L460`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::sync::Arc;

pub const K_BACKGROUNDS_IN_ROW: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallpaperKind {
    Pattern,
    Gradient,
    SolidColor,
    CustomFile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WallpaperItem {
    pub id: u64,
    pub title: String,
    pub kind: WallpaperKind,
    pub preview_color_hex: String,
    pub slug: String,
}

#[derive(Clone)]
pub struct BackgroundBox {
    pub peer_id: Option<u64>,
    pub peer_name: Option<String>,
    pub wallpapers: Vec<WallpaperItem>,
    pub selected_wallpaper_id: Option<u64>,
    pub is_closed: bool,
    pub on_select: Option<Arc<dyn Fn(u64) + Send + Sync>>,
    pub on_choose_file: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_reset_peer: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_close: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl BackgroundBox {
    pub fn new(peer_id: Option<u64>, peer_name: Option<String>) -> Self {
        Self {
            peer_id,
            peer_name,
            wallpapers: Self::default_wallpapers(),
            selected_wallpaper_id: Some(1),
            is_closed: false,
            on_select: None,
            on_choose_file: None,
            on_reset_peer: None,
            on_close: None,
        }
    }

    pub fn default_wallpapers() -> Vec<WallpaperItem> {
        vec![
            WallpaperItem { id: 1, title: "Classic Doodles".into(), kind: WallpaperKind::Pattern, preview_color_hex: "#B5D6EC".into(), slug: "classic_pattern".into() },
            WallpaperItem { id: 2, title: "Cosmic Indigo".into(), kind: WallpaperKind::Gradient, preview_color_hex: "#1B2A4A".into(), slug: "cosmic_indigo".into() },
            WallpaperItem { id: 3, title: "Emerald Serenity".into(), kind: WallpaperKind::Gradient, preview_color_hex: "#1B4D3E".into(), slug: "emerald_serenity".into() },
            WallpaperItem { id: 4, title: "Sunset Orange".into(), kind: WallpaperKind::Gradient, preview_color_hex: "#E06D53".into(), slug: "sunset_orange".into() },
            WallpaperItem { id: 5, title: "Graphite Noir".into(), kind: WallpaperKind::SolidColor, preview_color_hex: "#181818".into(), slug: "graphite_noir".into() },
            WallpaperItem { id: 6, title: "Pure Lavender".into(), kind: WallpaperKind::SolidColor, preview_color_hex: "#E6E6FA".into(), slug: "pure_lavender".into() },
        ]
    }

    pub fn select_wallpaper(&mut self, id: u64) {
        self.selected_wallpaper_id = Some(id);
        if let Some(ref cb) = self.on_select {
            cb(id);
        }
    }

    pub fn choose_from_file(&mut self) {
        if let Some(ref cb) = self.on_choose_file {
            cb();
        }
    }

    pub fn close(&mut self) {
        self.is_closed = true;
        if let Some(ref cb) = self.on_close {
            cb();
        }
    }
}

impl Default for BackgroundBox {
    fn default() -> Self {
        Self::new(None, None)
    }
}

impl Render for BackgroundBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        let title_text = if let Some(ref name) = self.peer_name {
            format!("Set Wallpaper for {}", name)
        } else {
            "Chat Wallpaper".to_string()
        };

        div()
            .id("background_box")
            .w(px(380.0))
            .p(px(20.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(12.0))
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(14.0))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x222222))
                            .child(title_text),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .text_size(px(14.0))
                            .text_color(rgb(0x888888))
                            .child("✕"),
                    ),
            )
            .child(
                div()
                    .w_full()
                    .py(px(9.0))
                    .rounded(px(8.0))
                    .bg(rgb(0xF1F3F5))
                    .cursor_pointer()
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(0x2AABEE))
                            .child("Choose from file"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(px(10.0))
                    .children(self.wallpapers.iter().map(|wp| {
                        let is_sel = Some(wp.id) == self.selected_wallpaper_id;
                        div()
                            .w(px(106.0))
                            .h(px(130.0))
                            .rounded(px(8.0))
                            .border_2()
                            .border_color(if is_sel { rgb(0x2AABEE) } else { rgb(0xEEEEEE) })
                            .bg(rgb(0x1B2A4A))
                            .cursor_pointer()
                            .flex()
                            .flex_col()
                            .justify_between()
                            .p(px(8.0))
                            .child(
                                if is_sel {
                                    div().size(px(18.0)).rounded_full().bg(rgb(0x2AABEE)).text_size(px(11.0)).text_color(rgb(0xFFFFFF)).flex().items_center().justify_center().child("✓")
                                } else {
                                    div()
                                },
                            )
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(0xFFFFFF))
                                    .child(wp.title.clone()),
                            )
                    })),
            )
            .into_any_element()
    }
}
