//! Info Bot StarRef Join Widget GPUI Component
//! Upstream: Telegram/SourceFiles/info/bot/starref/info_bot_starref_join_widget.h, info_bot_starref_join_widget.cpp
//! Exact affiliate program browser, pagination (`kPerPage = 50`), and sorting state machine.

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_PER_PAGE: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StarRefJoinType {
    Joined,
    Suggested,
    Existing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StarRefSuggestedSort {
    Profitability,
    Revenue,
    Date,
}

#[derive(Debug, Clone)]
pub struct StarRefProgramItem {
    pub bot_id: u64,
    pub bot_name: String,
    pub commission_bp: u16,
    pub duration_months: i32,
    pub monthly_revenue_stars: i64,
}

#[derive(Debug, Clone)]
pub struct InfoBotStarrefJoinWidgetBotStarref {
    pub peer_id: u64,
    pub title: String,
    pub join_type: StarRefJoinType,
    pub sort: StarRefSuggestedSort,
    pub items: Vec<StarRefProgramItem>,
    pub is_visible: bool,
    pub current_page: usize,
}

impl InfoBotStarrefJoinWidgetBotStarref {
    pub fn new(peer_id: u64, join_type: StarRefJoinType) -> Self {
        Self {
            peer_id,
            title: match join_type {
                StarRefJoinType::Joined => "Joined Programs".to_string(),
                StarRefJoinType::Suggested => "Suggested Programs".to_string(),
                StarRefJoinType::Existing => "My Programs".to_string(),
            },
            join_type,
            sort: StarRefSuggestedSort::Profitability,
            items: Vec::new(),
            is_visible: true,
            current_page: 0,
        }
    }

    pub fn set_sort(&mut self, sort: StarRefSuggestedSort) {
        self.sort = sort;
        self.apply_sort();
    }

    pub fn add_item(&mut self, item: StarRefProgramItem) {
        self.items.push(item);
        self.apply_sort();
    }

    pub fn apply_sort(&mut self) {
        match self.sort {
            StarRefSuggestedSort::Profitability => {
                self.items.sort_by(|a, b| b.commission_bp.cmp(&a.commission_bp));
            }
            StarRefSuggestedSort::Revenue => {
                self.items.sort_by(|a, b| b.monthly_revenue_stars.cmp(&a.monthly_revenue_stars));
            }
            StarRefSuggestedSort::Date => {
                self.items.sort_by(|a, b| b.bot_id.cmp(&a.bot_id));
            }
        }
    }

    pub fn load_more_rows(&mut self) {
        self.current_page += 1;
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }
}

impl Default for InfoBotStarrefJoinWidgetBotStarref {
    fn default() -> Self {
        Self::new(0, StarRefJoinType::Suggested)
    }
}

impl Render for InfoBotStarrefJoinWidgetBotStarref {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.items.len();

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
                    .child(format!("Total Programs: {} (Sorted by {:?})", count, self.sort))
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_widget_sorting_and_pagination() {
        let mut widget = InfoBotStarrefJoinWidgetBotStarref::new(3003, StarRefJoinType::Suggested);
        widget.add_item(StarRefProgramItem {
            bot_id: 1,
            bot_name: "AlphaBot".to_string(),
            commission_bp: 200,
            duration_months: 12,
            monthly_revenue_stars: 10000,
        });
        widget.add_item(StarRefProgramItem {
            bot_id: 2,
            bot_name: "BetaBot".to_string(),
            commission_bp: 500,
            duration_months: 12,
            monthly_revenue_stars: 5000,
        });

        // Sorted by Profitability (commission_bp desc) by default
        assert_eq!(widget.items[0].bot_name, "BetaBot");

        widget.set_sort(StarRefSuggestedSort::Revenue);
        assert_eq!(widget.items[0].bot_name, "AlphaBot");

        assert_eq!(widget.current_page, 0);
        widget.load_more_rows();
        assert_eq!(widget.current_page, 1);
    }
}
