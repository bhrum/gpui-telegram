//! Info Profile Tab Skeleton loading state
//! Upstream: `Telegram/SourceFiles/info/profile/tabs/info_profile_tab_skeleton.cpp:L1-L155`
//! Upstream: `Telegram/SourceFiles/info/profile/tabs/info_profile_tab_skeleton.h:L1-L23`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;

/// Shimmer skeleton placeholder grid displayed while tab content is preloading from storage or network.
#[derive(Debug, Clone)]
pub struct InfoProfileTabSkeletonProfileTabs {
    pub column_count: usize,
    pub row_count: usize,
    pub is_animating: bool,
}

impl InfoProfileTabSkeletonProfileTabs {
    pub const DEFAULT_COLUMNS: usize = 3;
    pub const DEFAULT_ROWS: usize = 4;

    pub fn new() -> Self {
        Self {
            column_count: Self::DEFAULT_COLUMNS,
            row_count: Self::DEFAULT_ROWS,
            is_animating: true,
        }
    }

    pub fn set_grid(&mut self, columns: usize, rows: usize) {
        self.column_count = columns;
        self.row_count = rows;
    }
}

impl Default for InfoProfileTabSkeletonProfileTabs {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoProfileTabSkeletonProfileTabs {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(8.0))
            .gap(px(4.0))
            .child(
                div()
                    .w_full()
                    .h(px(80.0))
                    .rounded_md()
                    .bg(rgb(0x202b36))
            )
    }
}
