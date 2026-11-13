//! Authentic GPUI implementation of Telegram Desktop's Sticker Set Preview Box.
//! Upstream reference: `Telegram/SourceFiles/boxes/sticker_set_box.h:L51-L95`
//! Upstream implementation: `Telegram/SourceFiles/boxes/sticker_set_box.cpp:L84-L95`, `L320-L460`, `L1200-L1350`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::sync::Arc;

pub const K_STICKERS_PER_ROW: usize = 5;
pub const K_EMOJI_PER_ROW: usize = 8;
pub const K_MIN_REPAINT_DELAY_MS: u64 = 33;
pub const K_MIN_AFTER_SCROLL_DELAY_MS: u64 = 33;
pub const K_GRAY_LOCK_OPACITY: f32 = 0.3;
pub const K_STICKER_MOVE_DURATION_MS: u64 = 200;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StickerInstallStatus {
    NotInstalled,
    Installing,
    Installed,
    Uninstalling,
    Archived,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StickerPreviewItem {
    pub document_id: u64,
    pub alt_emoji: String,
    pub preview_emoji: String,
}

#[derive(Clone)]
pub struct StickerSetBox {
    pub set_id: u64,
    pub title: String,
    pub short_name: String,
    pub is_emoji_set: bool,
    pub status: StickerInstallStatus,
    pub stickers: Vec<StickerPreviewItem>,
    pub is_closed: bool,
    pub on_install: Option<Arc<dyn Fn(u64) + Send + Sync>>,
    pub on_uninstall: Option<Arc<dyn Fn(u64) + Send + Sync>>,
    pub on_close: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl StickerSetBox {
    pub fn new(set_id: u64, title: impl Into<String>, short_name: impl Into<String>) -> Self {
        Self {
            set_id,
            title: title.into(),
            short_name: short_name.into(),
            is_emoji_set: false,
            status: StickerInstallStatus::NotInstalled,
            stickers: vec![
                StickerPreviewItem { document_id: 1, alt_emoji: "😀".into(), preview_emoji: "🐱".into() },
                StickerPreviewItem { document_id: 2, alt_emoji: "😍".into(), preview_emoji: "😻".into() },
                StickerPreviewItem { document_id: 3, alt_emoji: "😂".into(), preview_emoji: "😹".into() },
                StickerPreviewItem { document_id: 4, alt_emoji: "😎".into(), preview_emoji: "😼".into() },
                StickerPreviewItem { document_id: 5, alt_emoji: "😭".into(), preview_emoji: "😿".into() },
                StickerPreviewItem { document_id: 6, alt_emoji: "😡".into(), preview_emoji: "😾".into() },
                StickerPreviewItem { document_id: 7, alt_emoji: "😴".into(), preview_emoji: "😽".into() },
                StickerPreviewItem { document_id: 8, alt_emoji: "🎉".into(), preview_emoji: "🙀".into() },
                StickerPreviewItem { document_id: 9, alt_emoji: "🔥".into(), preview_emoji: "🐾".into() },
                StickerPreviewItem { document_id: 10, alt_emoji: "👍".into(), preview_emoji: "😸".into() },
            ],
            is_closed: false,
            on_install: None,
            on_uninstall: None,
            on_close: None,
        }
    }

    pub fn toggle_install(&mut self) {
        match self.status {
            StickerInstallStatus::NotInstalled | StickerInstallStatus::Archived => {
                self.status = StickerInstallStatus::Installing;
                if let Some(ref cb) = self.on_install {
                    cb(self.set_id);
                }
                self.status = StickerInstallStatus::Installed;
            }
            StickerInstallStatus::Installed => {
                self.status = StickerInstallStatus::Uninstalling;
                if let Some(ref cb) = self.on_uninstall {
                    cb(self.set_id);
                }
                self.status = StickerInstallStatus::NotInstalled;
            }
            _ => {}
        }
    }

    pub fn close(&mut self) {
        self.is_closed = true;
        if let Some(ref cb) = self.on_close {
            cb();
        }
    }
}

impl Default for StickerSetBox {
    fn default() -> Self {
        Self::new(555, "Cat Memes Official", "cat_memes")
    }
}

impl Render for StickerSetBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        let per_row = if self.is_emoji_set { K_EMOJI_PER_ROW } else { K_STICKERS_PER_ROW };

        let (btn_text, btn_bg, btn_fg) = match self.status {
            StickerInstallStatus::NotInstalled => (format!("Add {} Stickers", self.stickers.len()), rgb(0x2AABEE), rgb(0xFFFFFF)),
            StickerInstallStatus::Installing => ("Installing...".to_string(), rgb(0xCCCCCC), rgb(0xFFFFFF)),
            StickerInstallStatus::Installed => ("Remove Stickers".to_string(), rgb(0xFEEBEB), rgb(0xE53935)),
            StickerInstallStatus::Uninstalling => ("Removing...".to_string(), rgb(0xCCCCCC), rgb(0xFFFFFF)),
            StickerInstallStatus::Archived => ("Restore Stickers".to_string(), rgb(0x2AABEE), rgb(0xFFFFFF)),
        };

        div()
            .id("sticker_set_box")
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
                            .flex()
                            .flex_col()
                            .child(div().text_size(px(16.0)).font_weight(FontWeight::BOLD).text_color(rgb(0x222222)).child(self.title.clone()))
                            .child(div().text_size(px(12.0)).text_color(rgb(0x888888)).child(format!("t.me/addstickers/{}", self.short_name))),
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
                    .flex()
                    .flex_wrap()
                    .gap(px(8.0))
                    .children(self.stickers.iter().map(|s| {
                        div()
                            .size(px(58.0))
                            .rounded(px(8.0))
                            .bg(rgb(0xF8F9FA))
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(div().text_size(px(32.0)).child(s.preview_emoji.clone()))
                    })),
            )
            .child(
                div()
                    .w_full()
                    .py(px(10.0))
                    .rounded(px(8.0))
                    .bg(btn_bg)
                    .cursor_pointer()
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(btn_fg)
                            .child(btn_text),
                    ),
            )
            .into_any_element()
    }
}
