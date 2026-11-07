//! Authentic GPUI implementation of Telegram Desktop's Star Gift Craft Box.
//! Upstream reference: `Telegram/SourceFiles/boxes/star_gift_craft_box.h:L24-L51`
//! Upstream implementation: `Telegram/SourceFiles/boxes/star_gift_craft_box.cpp:L66-L100`, `L340-L420`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::sync::Arc;

pub const K_CRAFT_SLOTS: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CraftSlotItem {
    pub gift_id: u64,
    pub title: String,
    pub stars_value: u64,
    pub icon_emoji: String,
}

#[derive(Clone)]
pub struct StarGiftCraftBox {
    pub target_gift_id: u64,
    pub target_title: String,
    pub slots: [Option<CraftSlotItem>; K_CRAFT_SLOTS],
    pub success_permille: u32,
    pub is_closed: bool,
    pub on_start_craft: Option<Arc<dyn Fn(Vec<u64>) + Send + Sync>>,
    pub on_close: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl StarGiftCraftBox {
    pub fn new(target_gift_id: u64, target_title: impl Into<String>) -> Self {
        Self {
            target_gift_id,
            target_title: target_title.into(),
            slots: [None, None, None, None],
            success_permille: 0,
            is_closed: false,
            on_start_craft: None,
            on_close: None,
        }
    }

    pub fn recalculate_permille(&mut self) {
        let placed_count = self.slots.iter().filter(|s| s.is_some()).count() as u32;
        self.success_permille = placed_count * 250;
    }

    pub fn format_percent(permille: u32) -> String {
        let rounded = (permille + 5) / 10;
        format!("{}%", rounded)
    }

    pub fn put_gift_in_slot(&mut self, slot_idx: usize, item: CraftSlotItem) {
        if slot_idx < K_CRAFT_SLOTS {
            self.slots[slot_idx] = Some(item);
            self.recalculate_permille();
        }
    }

    pub fn remove_gift_from_slot(&mut self, slot_idx: usize) {
        if slot_idx < K_CRAFT_SLOTS {
            self.slots[slot_idx] = None;
            self.recalculate_permille();
        }
    }

    pub fn craft(&mut self) {
        let placed_ids: Vec<u64> = self.slots.iter().filter_map(|s| s.as_ref().map(|x| x.gift_id)).collect();
        if !placed_ids.is_empty() {
            if let Some(ref cb) = self.on_start_craft {
                cb(placed_ids);
            }
        }
    }

    pub fn close(&mut self) {
        self.is_closed = true;
        if let Some(ref cb) = self.on_close {
            cb();
        }
    }
}

impl Default for StarGiftCraftBox {
    fn default() -> Self {
        Self::new(777, "Cosmic Phoenix")
    }
}

impl Render for StarGiftCraftBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        let pct_str = Self::format_percent(self.success_permille);

        div()
            .id("star_gift_craft_box")
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
                            .child(format!("Forge {}", self.target_title)),
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
                    .h(px(140.0))
                    .w_full()
                    .rounded(px(10.0))
                    .bg(rgb(0x1A2536))
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap(px(8.0))
                    .child(div().text_size(px(40.0)).child("🔥"))
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x4FA3FF))
                            .child(format!("Success Chance: {}", pct_str)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .justify_between()
                    .gap(px(8.0))
                    .children((0..K_CRAFT_SLOTS).map(|idx| {
                        let slot = self.slots[idx].clone();
                        div()
                            .w(px(76.0))
                            .h(px(86.0))
                            .rounded(px(8.0))
                            .border_1()
                            .border_color(if slot.is_some() { rgb(0x2AABEE) } else { rgb(0xDDDDDD) })
                            .bg(if slot.is_some() { rgb(0xEEF8FF) } else { rgb(0xFAFAFA) })
                            .flex()
                            .flex_col()
                            .items_center()
                            .justify_center()
                            .gap(px(4.0))
                            .child(if let Some(ref item) = slot {
                                div().text_size(px(24.0)).child(item.icon_emoji.clone())
                            } else {
                                div().text_size(px(20.0)).text_color(rgb(0xCCCCCC)).child("+")
                            })
                            .child(if let Some(ref item) = slot {
                                div().text_size(px(10.0)).font_weight(FontWeight::MEDIUM).text_color(rgb(0x222222)).child(item.title.clone())
                            } else {
                                div().text_size(px(10.0)).text_color(rgb(0x999999)).child("Slot")
                            })
                    })),
            )
            .child(
                div()
                    .w_full()
                    .py(px(10.0))
                    .rounded(px(8.0))
                    .bg(if self.success_permille > 0 { rgb(0x2AABEE) } else { rgb(0xCCCCCC) })
                    .cursor_pointer()
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child("Start Crafting"),
                    ),
            )
            .into_any_element()
    }
}
