//! Genuine History View Slot Machine Component (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_slot_machine.h:L20-L79`
//! - `Telegram/SourceFiles/history/view/media/history_view_slot_machine.cpp:L21-L68`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use crate::views::stage::media::poll::{decode_slot_machine_reels, SlotReelSymbol, K_SLOT_WIN_VALUE};

#[derive(Debug, Clone)]
pub struct HistoryViewSlotMachineViewMedia {
    pub value: u8,
}

impl HistoryViewSlotMachineViewMedia {
    pub fn new(value: u8) -> Self {
        Self {
            value: value.clamp(1, 64),
        }
    }

    pub fn reels(&self) -> [SlotReelSymbol; 3] {
        decode_slot_machine_reels(self.value)
    }

    pub fn is_jackpot(&self) -> bool {
        self.value == K_SLOT_WIN_VALUE
    }
}

impl Default for HistoryViewSlotMachineViewMedia {
    fn default() -> Self {
        Self::new(64) // Jackpot 777 default
    }
}

impl Render for HistoryViewSlotMachineViewMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let reels = self.reels();
        let reel_symbols = reels.map(|r| match r {
            SlotReelSymbol::Bar => "BAR",
            SlotReelSymbol::Berries => "🍇",
            SlotReelSymbol::Lemon => "🍋",
            SlotReelSymbol::Seven => "7️⃣",
        });

        div()
            .flex()
            .flex_col()
            .items_center()
            .p(px(10.0))
            .rounded(px(10.0))
            .bg(rgb(0x182533))
            .gap(px(6.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .p(px(8.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x101921))
                    .border_1()
                    .border_color(if self.is_jackpot() { rgb(0xFFD700) } else { rgb(0x233140) })
                    .children(reel_symbols.iter().map(|s| {
                        div()
                            .w(px(32.0))
                            .h(px(32.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(18.0))
                            .child(*s)
                    })),
            )
            .when(self.is_jackpot(), |el| {
                el.child(
                    div()
                        .text_size(px(12.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xFFD700))
                        .child("🎉 777 JACKPOT!"),
                )
            })
    }
}
