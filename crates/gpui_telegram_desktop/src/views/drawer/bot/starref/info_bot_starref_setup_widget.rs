//! Info Bot StarRef Setup Widget GPUI Component
//! Upstream: Telegram/SourceFiles/info/bot/starref/info_bot_starref_setup_widget.h, info_bot_starref_setup_widget.cpp
//! Exact constants (`kDurationForeverValue = 999`, `kCommissionDefault = 200`, `kDurationDefault = 12`), and commission editor.

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_DURATION_FOREVER_VALUE: i32 = 999;
pub const K_COMMISSION_DEFAULT: u16 = 200;
pub const K_DURATION_DEFAULT: i32 = 12;
pub const K_DISABLED_FADE: f32 = 0.3;

#[derive(Debug, Clone)]
pub struct InfoBotStarrefSetupWidgetBotStarref {
    pub peer_id: u64,
    pub title: String,
    pub commission_bp: u16,
    pub duration_months: i32,
    pub program_exists: bool,
    pub is_visible: bool,
}

impl InfoBotStarrefSetupWidgetBotStarref {
    pub fn new(peer_id: u64, exists: bool) -> Self {
        Self {
            peer_id,
            title: if exists { "Edit Affiliate Program".to_string() } else { "Create Affiliate Program".to_string() },
            commission_bp: 0,
            duration_months: 0,
            program_exists: exists,
            is_visible: true,
        }
    }

    pub fn value_for_commission(&self) -> u16 {
        if self.commission_bp > 0 {
            self.commission_bp
        } else {
            K_COMMISSION_DEFAULT
        }
    }

    pub fn value_for_duration_months(&self) -> i32 {
        if self.duration_months > 0 {
            self.duration_months
        } else if self.program_exists {
            K_DURATION_FOREVER_VALUE
        } else {
            K_DURATION_DEFAULT
        }
    }

    pub fn set_commission(&mut self, commission_bp: u16) {
        self.commission_bp = commission_bp;
    }

    pub fn set_duration_months(&mut self, months: i32) {
        self.duration_months = months;
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }
}

impl Default for InfoBotStarrefSetupWidgetBotStarref {
    fn default() -> Self {
        Self::new(0, false)
    }
}

impl Render for InfoBotStarrefSetupWidgetBotStarref {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let comm = self.value_for_commission();
        let dur = self.value_for_duration_months();

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .gap(px(12.0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .child(self.title.clone())
            )
            .child(
                div()
                    .flex()
                    .justify_between()
                    .child("Commission Rate:")
                    .child(format!("{:.1}% ({} bp)", (comm as f64) / 100.0, comm))
            )
            .child(
                div()
                    .flex()
                    .justify_between()
                    .child("Program Duration:")
                    .child(if dur == K_DURATION_FOREVER_VALUE { "Forever".to_string() } else { format!("{} months", dur) })
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_widget_defaults() {
        let new_prog = InfoBotStarrefSetupWidgetBotStarref::new(4004, false);
        assert_eq!(new_prog.value_for_commission(), K_COMMISSION_DEFAULT);
        assert_eq!(new_prog.value_for_duration_months(), K_DURATION_DEFAULT);

        let existing_prog = InfoBotStarrefSetupWidgetBotStarref::new(4004, true);
        assert_eq!(existing_prog.value_for_commission(), K_COMMISSION_DEFAULT);
        assert_eq!(existing_prog.value_for_duration_months(), K_DURATION_FOREVER_VALUE);

        let mut custom = InfoBotStarrefSetupWidgetBotStarref::new(4004, false);
        custom.set_commission(500);
        custom.set_duration_months(6);
        assert_eq!(custom.value_for_commission(), 500);
        assert_eq!(custom.value_for_duration_months(), 6);
    }
}
