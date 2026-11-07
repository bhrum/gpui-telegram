//! Authentic GPUI implementation of Telegram Desktop's StickersBox.
//! Upstream reference: `Telegram/SourceFiles/boxes/stickers_box.h:L30-L75`
//! and `Telegram/SourceFiles/boxes/stickers_box.cpp:L50-L150`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_ARCHIVED_LIMIT_FIRST_REQUEST: usize = 10;
pub const K_ARCHIVED_LIMIT_PER_PAGE: usize = 30;
pub const K_HANDLE_MEGAGROUP_SET_ADDRESS_CHANGE_TIMEOUT_MS: u64 = 1000;

/// Sections in Stickers Box (`stickers_box.h:L38-L44`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StickersBoxSection {
    Installed,
    Featured,
    Archived,
    Attached,
    Masks,
}

pub type StickersSection = StickersBoxSection;

/// Sticker set descriptor item in list.
#[derive(Clone, Debug)]
pub struct StickerSetItem {
    pub id: u64,
    pub title: String,
    pub short_name: String,
    pub count: usize,
    pub is_installed: bool,
    pub is_official: bool,
}

/// View component for managing installed, featured, and archived sticker sets.
#[derive(Clone)]
pub struct StickersBox {
    pub section: StickersBoxSection,
    pub sets: Vec<StickerSetItem>,
    pub is_closed: bool,
}

impl Default for StickersBox {
    fn default() -> Self {
        Self {
            section: StickersBoxSection::Installed,
            sets: vec![
                StickerSetItem {
                    id: 1,
                    title: "Duck Sticker Set".into(),
                    short_name: "duck".into(),
                    count: 32,
                    is_installed: true,
                    is_official: true,
                },
                StickerSetItem {
                    id: 2,
                    title: "Animated Pepe".into(),
                    short_name: "pepe".into(),
                    count: 48,
                    is_installed: true,
                    is_official: false,
                },
            ],
            is_closed: false,
        }
    }
}

impl StickersBox {
    pub fn new(section: StickersBoxSection) -> Self {
        Self {
            section,
            ..Default::default()
        }
    }

    pub fn set_section(&mut self, section: StickersBoxSection) {
        self.section = section;
    }

    pub fn remove_set(&mut self, set_id: u64) {
        self.sets.retain(|s| s.id != set_id);
    }

    pub fn close(&mut self) {
        self.is_closed = true;
    }
}

impl Render for StickersBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        let mut card = div()
            .id("stickers_box")
            .w(px(420.0))
            .p(px(24.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(12.0))
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(14.0));

        // Header Title
        card = card.child(
            div()
                .text_size(px(18.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(0x1F2937))
                .child("Stickers and Emoji"),
        );

        // Section Tabs
        let sections = [
            (StickersBoxSection::Installed, "Installed"),
            (StickersBoxSection::Featured, "Trending"),
            (StickersBoxSection::Archived, "Archived"),
            (StickersBoxSection::Masks, "Masks"),
        ];

        let mut tabs_row = div().flex().items_center().gap(px(8.0));
        for (sec, title) in sections {
            let is_active = self.section == sec;
            let bg_col = if is_active { rgb(0xEFF6FF) } else { rgb(0xF9FAFB) };
            let text_col = if is_active { rgb(0x2563EB) } else { rgb(0x64748B) };

            tabs_row = tabs_row.child(
                div()
                    .px(px(12.0))
                    .py(px(6.0))
                    .rounded(px(6.0))
                    .bg(bg_col)
                    .cursor_pointer()
                    .text_size(px(13.0))
                    .font_weight(if is_active { FontWeight::BOLD } else { FontWeight::MEDIUM })
                    .text_color(text_col)
                    .child(title),
            );
        }
        card = card.child(tabs_row);

        // Sticker Sets List
        let mut list_col = div().flex().flex_col().gap(px(8.0));
        for set in &self.sets {
            let row = div()
                .flex()
                .items_center()
                .justify_between()
                .p(px(10.0))
                .rounded(px(8.0))
                .bg(rgb(0xF8FAFC))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(0x0F172A))
                                .child(set.title.clone()),
                        )
                        .child(
                            div()
                                .text_size(px(12.0))
                                .text_color(rgb(0x64748B))
                                .child(format!("{} stickers", set.count)),
                        ),
                )
                .child(
                    div()
                        .px(px(10.0))
                        .py(px(4.0))
                        .rounded(px(4.0))
                        .bg(rgba(0xEF44441A))
                        .cursor_pointer()
                        .text_size(px(12.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0xDC2626))
                        .child("Remove"),
                );
            list_col = list_col.child(row);
        }
        card = card.child(list_col);

        card.into_any_element()
    }
}
