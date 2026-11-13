//! History View Bottom Info Layout & Rendering (`history_view_bottom_info.h/cpp`).
//! Upstream reference: `Telegram/SourceFiles/history/view/history_view_bottom_info.cpp:L40-L774`
//! Mandate #18 Citations:
//! - Schedule period text map: 60s, 300s, daily, weekly, biweekly, monthly (`history_view_bottom_info.cpp:L42-L70`)
//! - Flags bitfield (`history_view_bottom_info.h:L35-L54`)
//! - Layout geometry: `msgDateDelta = point(2px, 5px)`, `historyViewsSpace = 4px` (`history_view_bottom_info.cpp:L341`)

use gpui::*;
use serde::{Deserialize, Serialize};

/// Flags matching C++ `HistoryView::BottomInfo::Data::Flag` (`history_view_bottom_info.h:L35-L54`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct BottomInfoFlags(pub u16);

impl BottomInfoFlags {
    pub const EDITED: u16 = 0x0001;
    pub const OUT_LAYOUT: u16 = 0x0002;
    pub const SENDING: u16 = 0x0004;
    pub const REPLIES_CONTEXT: u16 = 0x0008;
    pub const SPONSORED: u16 = 0x0010;
    pub const PINNED: u16 = 0x0020;
    pub const IMPORTED: u16 = 0x0040;
    pub const SHORTCUT: u16 = 0x0080;
    pub const ESTIMATE_DATE: u16 = 0x0100;
    pub const FORWARDED_DATE: u16 = 0x0200;
    pub const SILENT: u16 = 0x0400;
    pub const EDITED_PRIMARY: u16 = 0x0800;
    pub const EPHEMERAL: u16 = 0x1000;
    pub const UPDATED: u16 = 0x2000;
    pub const FULL_DATE: u16 = 0x4000;

    pub fn new() -> Self {
        Self(0)
    }

    pub fn contains(&self, flag: u16) -> bool {
        (self.0 & flag) != 0
    }

    pub fn insert(&mut self, flag: u16) {
        self.0 |= flag;
    }

    pub fn remove(&mut self, flag: u16) {
        self.0 &= !flag;
    }

    pub fn set(&mut self, flag: u16, val: bool) {
        if val {
            self.insert(flag);
        } else {
            self.remove(flag);
        }
    }
}

/// Recurring schedule interval text resolver (`history_view_bottom_info.cpp:L42-L70`).
pub fn schedule_period_text(period: i64) -> &'static str {
    if period <= 60 {
        "minutely"
    } else if period <= 300 {
        "5-minutely"
    } else if period <= 86400 {
        "daily"
    } else if period <= 7 * 86400 {
        "weekly"
    } else if period <= 14 * 86400 {
        "biweekly"
    } else if period <= 30 * 86400 {
        "monthly"
    } else if period <= 91 * 86400 {
        "every 3 months"
    } else if period <= 182 * 86400 {
        "every 6 months"
    } else {
        "yearly"
    }
}

/// Compact format for view counts (e.g. 1200 -> "1.2K", 1500000 -> "1.5M").
pub fn format_views_count(count: i32) -> String {
    if count < 1000 {
        count.to_string()
    } else if count < 1_000_000 {
        let k = (count as f64) / 1000.0;
        format!("{:.1}K", k).replace(".0K", "K")
    } else {
        let m = (count as f64) / 1_000_000.0;
        format!("{:.1}M", m).replace(".0M", "M")
    }
}

/// Data structure matching C++ `HistoryView::BottomInfo::Data` (`history_view_bottom_info.h:L34-L67`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BottomInfoData {
    pub date: i64,
    pub edited_date: Option<i64>,
    pub author: Option<String>,
    pub flags: BottomInfoFlags,
    pub views: Option<i32>,
    pub replies: Option<i32>,
    pub forwards_count: Option<i32>,
}

impl Default for BottomInfoData {
    fn default() -> Self {
        Self {
            date: 0,
            edited_date: None,
            author: None,
            flags: BottomInfoFlags::new(),
            views: None,
            replies: None,
            forwards_count: None,
        }
    }
}

impl BottomInfoData {
    pub fn format_time_string(&self) -> String {
        let hours = (self.date % 86400) / 3600;
        let minutes = (self.date % 3600) / 60;
        format!("{:02}:{:02}", hours, minutes)
    }

    pub fn render(&self) -> impl IntoElement {
        let is_outgoing = self.flags.contains(BottomInfoFlags::OUT_LAYOUT);
        let is_sending = self.flags.contains(BottomInfoFlags::SENDING);
        let is_edited = self.flags.contains(BottomInfoFlags::EDITED);
        let is_silent = self.flags.contains(BottomInfoFlags::SILENT);
        let is_pinned = self.flags.contains(BottomInfoFlags::PINNED);

        let date_fg = if is_outgoing {
            rgb(0x5EA852) // Outgoing green tint
        } else {
            rgb(0x888888) // Incoming neutral gray
        };

        let mut row = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(3.0))
            .text_size(px(11.0))
            .text_color(date_fg);

        if let Some(views) = self.views {
            row = row.child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(2.0))
                    .child(div().text_size(px(10.0)).child("👁"))
                    .child(div().child(format_views_count(views))),
            );
        }

        if is_silent {
            row = row.child(div().text_size(px(10.0)).child("🔕"));
        }

        if is_pinned {
            row = row.child(div().text_size(px(10.0)).child("📌"));
        }

        if let Some(ref author) = self.author {
            row = row.child(
                div()
                    .font_weight(FontWeight::MEDIUM)
                    .child(author.clone()),
            );
        }

        if is_edited {
            row = row.child(div().child("edited"));
        }

        row = row.child(div().child(self.format_time_string()));

        if is_outgoing {
            let ticks = if is_sending {
                "🕒"
            } else if self.flags.contains(BottomInfoFlags::UPDATED) {
                "✓✓"
            } else {
                "✓"
            };

            row = row.child(
                div()
                    .text_size(px(10.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(date_fg)
                    .child(ticks),
            );
        }

        row
    }
}

/// GPUI View Element for message bottom information strip.
pub struct BottomInfoView {
    pub data: BottomInfoData,
}

impl BottomInfoView {
    pub fn new(data: BottomInfoData) -> Self {
        Self { data }
    }

    pub fn render(&self) -> impl IntoElement {
        self.data.render()
    }
}

impl Render for BottomInfoView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.data.render()
    }
}
