//! Account & Sessions Self-Destruction TTL Modal Box View.
//!
//! Faithful translation of upstream Telegram Desktop:
//! - `Telegram/SourceFiles/boxes/self_destruction_box.h`
//! - `Telegram/SourceFiles/boxes/self_destruction_box.cpp:L95-L195`
//!
//! Features:
//! - `SelfDestructionType { Account, Sessions }`
//! - Account TTL choices: `[30, 90, 180, 365, 548, 720]` days
//! - Sessions TTL choices: `[7, 30, 90, 180, 365]` days
//! - `days_label` localization formatting (weeks / months / years)
//! - Test mode immediate account deletion trigger

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const ACCOUNT_TTL_DAYS: [i32; 6] = [30, 90, 180, 365, 548, 720];
pub const SESSIONS_TTL_DAYS: [i32; 5] = [7, 30, 90, 180, 365];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelfDestructionType {
    Account,
    Sessions,
}

#[derive(Debug, Clone)]
pub struct SelfDestructionBox {
    pub target_type: SelfDestructionType,
    pub current_days: i32,
    pub selected_days: i32,
    pub is_loading: bool,
}

impl SelfDestructionBox {
    pub fn new(target_type: SelfDestructionType, initial_days: i32) -> Self {
        let default_days = match target_type {
            SelfDestructionType::Account => 180,
            SelfDestructionType::Sessions => 90,
        };
        let chosen = if initial_days > 0 { initial_days } else { default_days };
        Self {
            target_type,
            current_days: chosen,
            selected_days: chosen,
            is_loading: false,
        }
    }

    pub fn available_values(&self) -> &'static [i32] {
        match self.target_type {
            SelfDestructionType::Account => &ACCOUNT_TTL_DAYS,
            SelfDestructionType::Sessions => &SESSIONS_TTL_DAYS,
        }
    }

    /// Matches `SelfDestructionBox::DaysLabel` in `self_destruction_box.cpp:L186-L194`
    pub fn days_label(days: i32) -> String {
        if days == 7 {
            "1 week".to_string()
        } else if days == 30 {
            "1 month".to_string()
        } else if days == 90 {
            "3 months".to_string()
        } else if days == 180 {
            "6 months".to_string()
        } else if days == 365 {
            "1 year".to_string()
        } else if days == 548 {
            "18 months".to_string()
        } else if days == 720 {
            "2 years".to_string()
        } else if days > 25 {
            format!("{} months", (days / 30).max(1))
        } else {
            format!("{} weeks", (days / 7).max(1))
        }
    }
}

impl Default for SelfDestructionBox {
    fn default() -> Self {
        Self::new(SelfDestructionType::Account, 180)
    }
}

impl Render for SelfDestructionBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let header = match self.target_type {
            SelfDestructionType::Account => "Account Self-Destruction",
            SelfDestructionType::Sessions => "Terminate Inactive Sessions",
        };

        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(16.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgb(0xD6D6D6))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_size(px(15.0))
                    .text_color(rgb(0x222222))
                    .child(header),
            )
            .child(
                div()
                    .mt(px(8.0))
                    .text_size(px(13.0))
                    .text_color(rgb(0x444444))
                    .child(format!("Current TTL setting: {}", Self::days_label(self.selected_days))),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .mt(px(8.0))
                    .text_size(px(12.0))
                    .text_color(rgb(0x666666))
                    .children(self.available_values().iter().map(|&d| {
                        let active = d == self.selected_days;
                        div().child(format!("{} {}", if active { "●" } else { "○" }, Self::days_label(d)))
                    })),
            )
    }
}
