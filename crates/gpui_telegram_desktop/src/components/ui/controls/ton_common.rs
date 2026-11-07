//! UI TON cryptocurrency helpers and amount formatters.
//! Upstream: `Telegram/SourceFiles/ui/controls/ton_common.cpp:L40-L140`
//!           `Telegram/SourceFiles/ui/controls/ton_common.h:L21-L43, L64-L71`
//!
//! Nanoton conversion (1 TON = 10^9 nanotons), canonical number formatting, and wallet badge styling.

use gpui::*;

pub const K_NANOS_IN_ONE: i64 = 1_000_000_000;
pub const K_MAX_DIGITS_COUNT: usize = 9;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormattedTonAmount {
    pub whole_string: String,
    pub separator: String,
    pub nano_string: String,
    pub full: String,
}

/// Formats nanotons into decimal TON string representation.
/// Mirrors upstream `FormatTonAmount` in `ui/controls/ton_common.cpp:L100-L140`.
pub fn format_ton_amount(amount: i64, signed: bool, rounded: bool) -> FormattedTonAmount {
    let sign_prefix = if amount < 0 {
        "−"
    } else if signed && amount > 0 {
        "+"
    } else {
        ""
    };

    let abs_amount = amount.abs();
    let whole = abs_amount / K_NANOS_IN_ONE;
    let nanos = abs_amount % K_NANOS_IN_ONE;

    let sep = ".".to_string();
    let nano_str = if nanos == 0 {
        String::new()
    } else {
        let raw = format!("{:09}", nanos);
        if rounded {
            raw.chars().take(2).collect()
        } else {
            raw.trim_end_matches('0').to_string()
        }
    };

    let whole_str = format!("{}{}", sign_prefix, whole);
    let full = if nano_str.is_empty() {
        whole_str.clone()
    } else {
        format!("{}{}{}", whole_str, sep, nano_str)
    };

    FormattedTonAmount {
        whole_string: whole_str,
        separator: sep,
        nano_string: nano_str,
        full,
    }
}

/// Parse decimal TON string into nanotons.
pub fn parse_ton_amount_string(amount: &str) -> Option<i64> {
    let s = amount.trim().replace(',', ".");
    if s.is_empty() {
        return None;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() > 2 {
        return None;
    }
    let whole: i64 = parts[0].parse().ok()?;
    let nano: i64 = if parts.len() == 2 {
        let frac = parts[1];
        if frac.len() > 9 {
            return None;
        }
        let padded = format!("{:0<9}", frac);
        padded.parse().ok()?
    } else {
        0
    };
    Some(whole * K_NANOS_IN_ONE + nano)
}

/// TonCommonControls component. Target symbol required by ledger: `TonCommonControls`.
#[derive(Debug, Clone)]
pub struct TonCommonControls {
    pub amount_nanos: i64,
    pub is_signed: bool,
    pub is_rounded: bool,
}

impl TonCommonControls {
    pub fn new(amount_nanos: i64) -> Self {
        Self {
            amount_nanos,
            is_signed: false,
            is_rounded: false,
        }
    }

    pub fn formatted(&self) -> FormattedTonAmount {
        format_ton_amount(self.amount_nanos, self.is_signed, self.is_rounded)
    }
}

impl Default for TonCommonControls {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Render for TonCommonControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let fmt = self.formatted();
        div()
            .flex()
            .items_center()
            .gap(px(4.0))
            .text_size(px(14.0))
            .font_weight(FontWeight::SEMIBOLD)
            .child("💎")
            .child(fmt.full)
    }
}
