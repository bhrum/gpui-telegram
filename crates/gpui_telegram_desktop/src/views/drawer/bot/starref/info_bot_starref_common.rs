//! Info Bot StarRef Common GPUI Component
//! Upstream: Telegram/SourceFiles/info/bot/starref/info_bot_starref_common.h, info_bot_starref_common.cpp
//! Exact affiliate program commission formatting, duration calculation, and connected bot cards.

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StarRefProgram {
    pub commission_basis_points: u16, // e.g. 200 = 2.00%
    pub duration_months: i32,         // 12 = 1 year, 999 = forever
}

impl Default for StarRefProgram {
    fn default() -> Self {
        Self {
            commission_basis_points: 200,
            duration_months: 12,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConnectedBotState {
    pub program: StarRefProgram,
    pub link: String,
    pub date: i64,
    pub users_count: i32,
    pub revoked: bool,
}

#[derive(Debug, Clone)]
pub struct InfoBotStarrefCommonBotStarref {
    pub title: String,
    pub connected_bots: Vec<(u64, ConnectedBotState)>, // peer_id -> state
    pub is_visible: bool,
}

impl InfoBotStarrefCommonBotStarref {
    pub fn new() -> Self {
        Self {
            title: "Affiliate Program".to_string(),
            connected_bots: Vec::new(),
            is_visible: true,
        }
    }

    pub fn format_commission(commission_bp: u16) -> String {
        let percent = (commission_bp as f64) / 100.0;
        if percent.fract() == 0.0 {
            format!("{:.0}%", percent)
        } else {
            format!("{:.1}%", percent)
        }
    }

    pub fn format_program_duration(duration_months: i32) -> String {
        if duration_months <= 0 || duration_months >= 999 {
            "Forever".to_string()
        } else if duration_months % 12 == 0 {
            let years = duration_months / 12;
            if years == 1 {
                "1 year".to_string()
            } else {
                format!("{} years", years)
            }
        } else if duration_months == 1 {
            "1 month".to_string()
        } else {
            format!("{} months", duration_months)
        }
    }

    pub fn add_connected_bot(&mut self, bot_id: u64, state: ConnectedBotState) {
        self.connected_bots.retain(|(id, _)| *id != bot_id);
        self.connected_bots.push((bot_id, state));
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }
}

impl Default for InfoBotStarrefCommonBotStarref {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoBotStarrefCommonBotStarref {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.connected_bots.len();

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .gap(px(8.0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .child(self.title.clone())
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x71717A))
                    .child(format!("Connected Affiliate Bots: {}", count))
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_commission_and_duration() {
        assert_eq!(InfoBotStarrefCommonBotStarref::format_commission(200), "2%");
        assert_eq!(InfoBotStarrefCommonBotStarref::format_commission(250), "2.5%");
        assert_eq!(InfoBotStarrefCommonBotStarref::format_commission(1500), "15%");

        assert_eq!(InfoBotStarrefCommonBotStarref::format_program_duration(999), "Forever");
        assert_eq!(InfoBotStarrefCommonBotStarref::format_program_duration(0), "Forever");
        assert_eq!(InfoBotStarrefCommonBotStarref::format_program_duration(1), "1 month");
        assert_eq!(InfoBotStarrefCommonBotStarref::format_program_duration(6), "6 months");
        assert_eq!(InfoBotStarrefCommonBotStarref::format_program_duration(12), "1 year");
        assert_eq!(InfoBotStarrefCommonBotStarref::format_program_duration(24), "2 years");
    }

    #[test]
    fn test_add_connected_bot() {
        let mut common = InfoBotStarrefCommonBotStarref::new();
        let state = ConnectedBotState {
            program: StarRefProgram {
                commission_basis_points: 500,
                duration_months: 12,
            },
            link: "https://t.me/bot?start=ref123".to_string(),
            date: 1700000000,
            users_count: 42,
            revoked: false,
        };
        common.add_connected_bot(5001, state);
        assert_eq!(common.connected_bots.len(), 1);
        assert_eq!(common.connected_bots[0].1.users_count, 42);
    }
}
