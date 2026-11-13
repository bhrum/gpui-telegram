//! Authentic GPUI implementation of Telegram Desktop's TransferGiftBox.
//! Upstream reference: `Telegram/SourceFiles/boxes/transfer_gift_box.h:L30-L98`
//! and `Telegram/SourceFiles/boxes/transfer_gift_box.cpp:L60-L150`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_PLATE_GROUPS_PER_LINE: usize = 4;
pub const K_MOVED_TO_WALLET_TOAST_DURATION_MS: u64 = 5000;
pub const K_TMP_PASSWORD_PERIOD_SECS: u64 = 600;
pub const K_PASSWORD_STATE_TIMEOUT_MS: u64 = 10000;
pub const K_GIFT_MOVE_KEEP_ON_PROFILE_PREF: &str = "gift_move_keep_on_profile";

/// Target transfer destination.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransferDestination {
    TelegramUser { peer_id: u64, name: String },
    TonWallet { address: String },
}

pub type TransferTargetMode = TransferDestination;

/// View component for transferring unique star collectible gifts or exporting to TON blockchain.
#[derive(Clone)]
pub struct TransferGiftBox {
    pub gift_title: String,
    pub gift_number: u32,
    pub destination: TransferDestination,
    pub keep_on_profile: bool,
    pub transfer_fee_stars: u64,
    pub is_closed: bool,
}

impl Default for TransferGiftBox {
    fn default() -> Self {
        Self {
            gift_title: "Cyber Skull".into(),
            gift_number: 42,
            destination: TransferDestination::TonWallet {
                address: "EQBvW8Z5huBkMJYdn3PCDn57NuF64fzCUSn-9Ta4kG".into(),
            },
            keep_on_profile: true,
            transfer_fee_stars: 25,
            is_closed: false,
        }
    }
}

impl TransferGiftBox {
    pub fn new(gift_title: String, gift_number: u32, destination: TransferDestination) -> Self {
        Self {
            gift_title,
            gift_number,
            destination,
            keep_on_profile: true,
            transfer_fee_stars: 25,
            is_closed: false,
        }
    }

    pub fn set_destination(&mut self, destination: TransferDestination) {
        self.destination = destination;
    }

    pub fn toggle_keep_on_profile(&mut self) {
        self.keep_on_profile = !self.keep_on_profile;
    }

    pub fn close(&mut self) {
        self.is_closed = true;
    }
}

impl Render for TransferGiftBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        let mut card = div()
            .id("transfer_gift_box")
            .w(px(380.0))
            .p(px(24.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(14.0))
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(16.0));

        // Header Title
        card = card.child(
            div()
                .flex()
                .items_center()
                .gap(px(10.0))
                .child(
                    div()
                        .text_size(px(24.0))
                        .child("🎁"),
                )
                .child(
                    div()
                        .text_size(px(18.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x1F2937))
                        .child(format!("Transfer {} #{}", self.gift_title, self.gift_number)),
                ),
        );

        // Destination Display
        let dest_str = match &self.destination {
            TransferDestination::TelegramUser { name, .. } => format!("Recipient: {}", name),
            TransferDestination::TonWallet { address } => format!("TON Address: {:.8}...{:.6}", address, &address[address.len()-6..]),
        };

        card = card.child(
            div()
                .p(px(12.0))
                .bg(rgb(0xF8FAFC))
                .rounded(px(8.0))
                .flex()
                .flex_col()
                .gap(px(4.0))
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgb(0x64748B))
                        .child("Destination"),
                )
                .child(
                    div()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0x0F172A))
                        .child(dest_str),
                ),
        );

        // Keep on Profile Checkbox Option (`kGiftMoveKeepOnProfilePref`)
        card = card.child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .child(
                    div()
                        .w(px(18.0))
                        .h(px(18.0))
                        .rounded(px(4.0))
                        .bg(if self.keep_on_profile { rgb(0x2563EB) } else { rgb(0xE2E8F0) })
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(11.0))
                        .text_color(rgb(0xFFFFFF))
                        .child(if self.keep_on_profile { "✓" } else { "" }),
                )
                .child(
                    div()
                        .text_size(px(13.0))
                        .text_color(rgb(0x334155))
                        .child("Keep badge visible on profile"),
                ),
        );

        // Action Transfer Button
        card = card.child(
            div()
                .w_full()
                .py(px(10.0))
                .rounded(px(8.0))
                .bg(rgb(0x2563EB))
                .cursor_pointer()
                .flex()
                .justify_center()
                .text_size(px(14.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(0xFFFFFF))
                .child(format!("Confirm Transfer (Fee: ⭐ {})", self.transfer_fee_stars)),
        );

        card.into_any_element()
    }
}
