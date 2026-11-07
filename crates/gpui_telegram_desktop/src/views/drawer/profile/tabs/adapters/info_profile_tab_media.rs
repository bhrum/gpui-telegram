//! Info Profile Tab Media Adapter
//! Upstream: `Telegram/SourceFiles/info/profile/tabs/adapters/info_profile_tab_media.cpp:L1-L334`
//! Upstream: `Telegram/SourceFiles/info/profile/tabs/adapters/info_profile_tab_media.h:L1-L25`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;
use serde::{Deserialize, Serialize};

/// Media item rendered inside the media grid tab.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaGridTile {
    pub id: u64,
    pub media_type: String,
    pub duration_text: Option<String>,
    pub caption: Option<String>,
    pub size_text: Option<String>,
}

/// Tab adapter bridging the profile tabs host with shared media browser views.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfoProfileTabMediaProfileTabsAdapters {
    pub peer_id: i64,
    pub total_media_count: usize,
    pub is_active: bool,
    pub is_split_mode: bool,
    pub grid_columns: usize,
    pub page_size: usize,
    pub current_page: usize,
    pub is_loading: bool,
    pub has_more: bool,
    pub scroll_top: f32,
    pub tiles: Vec<MediaGridTile>,
}

impl Default for InfoProfileTabMediaProfileTabsAdapters {
    fn default() -> Self {
        Self::new(0)
    }
}

impl InfoProfileTabMediaProfileTabsAdapters {
    pub const PRELOAD_THRESHOLD_PX: f32 = 400.0;

    pub fn new(peer_id: i64) -> Self {
        Self {
            peer_id,
            total_media_count: 0,
            is_active: false,
            is_split_mode: false,
            grid_columns: 3,
            page_size: 40,
            current_page: 0,
            is_loading: false,
            has_more: true,
            scroll_top: 0.0,
            tiles: Vec::new(),
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn set_count(&mut self, count: usize) {
        self.total_media_count = count;
    }

    pub fn set_split_mode(&mut self, split: bool) {
        self.is_split_mode = split;
    }

    pub fn set_grid_columns(&mut self, columns: usize) {
        self.grid_columns = columns.clamp(1, 6);
    }

    pub fn add_tile(&mut self, tile: MediaGridTile) {
        self.tiles.push(tile);
        self.total_media_count = self.tiles.len();
    }

    pub fn load_next_page(&mut self) -> bool {
        if self.is_loading || !self.has_more {
            return false;
        }
        self.is_loading = true;
        self.current_page += 1;
        self.is_loading = false;
        true
    }

    pub fn check_preload(&self, scroll_top: f32, viewport_height: f32, content_height: f32) -> bool {
        if !self.has_more || self.is_loading {
            return false;
        }
        let remaining_scroll = content_height - (scroll_top + viewport_height);
        remaining_scroll <= Self::PRELOAD_THRESHOLD_PX
    }
}

impl Render for InfoProfileTabMediaProfileTabsAdapters {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_empty = self.tiles.is_empty();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xFFFFFF))
            .p(px(8.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .px(px(4.0))
                    .py(px(6.0))
                    .border_b_1()
                    .border_color(rgb(0xEBEEF2))
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x707579))
                            .child(format!("{} items", self.total_media_count))
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x2AABEE))
                            .child(if self.is_split_mode { "Split Tabs" } else { "Combined Grid" })
                    )
            )
            .child(
                if is_empty {
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .h(px(180.0))
                        .text_sm()
                        .text_color(rgb(0x707579))
                        .child("No shared media yet")
                } else {
                    div()
                        .flex()
                        .flex_wrap()
                        .gap(px(4.0))
                        .py(px(8.0))
                        .children(self.tiles.iter().map(|tile| {
                            div()
                                .size(px(80.0))
                                .rounded_sm()
                                .bg(rgb(0xDFE3E8))
                                .flex()
                                .flex_col()
                                .justify_between()
                                .p(px(4.0))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(0x555555))
                                        .child(tile.media_type.clone())
                                )
                                .children(tile.duration_text.as_ref().map(|dur| {
                                    div()
                                        .text_xs()
                                        .text_color(rgb(0xFFFFFF))
                                        .bg(rgb(0x333333))
                                        .px(px(2.0))
                                        .rounded_xs()
                                        .child(dur.clone())
                                }))
                        }))
                }
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_adapter_pagination_and_grid() {
        let mut adapter = InfoProfileTabMediaProfileTabsAdapters::new(1001);
        assert_eq!(adapter.peer_id, 1001);
        assert_eq!(adapter.total_media_count, 0);
        assert!(!adapter.is_active);

        adapter.set_active(true);
        assert!(adapter.is_active);

        adapter.add_tile(MediaGridTile {
            id: 1,
            media_type: "photo".into(),
            duration_text: None,
            caption: Some("First photo".into()),
            size_text: Some("1.2 MB".into()),
        });
        adapter.add_tile(MediaGridTile {
            id: 2,
            media_type: "video".into(),
            duration_text: Some("01:23".into()),
            caption: None,
            size_text: Some("15.4 MB".into()),
        });

        assert_eq!(adapter.total_media_count, 2);
        assert_eq!(adapter.tiles.len(), 2);

        adapter.set_split_mode(true);
        assert!(adapter.is_split_mode);

        adapter.set_grid_columns(4);
        assert_eq!(adapter.grid_columns, 4);

        // Preload checks
        assert!(adapter.check_preload(1600.0, 500.0, 2400.0)); // 2400 - 2100 = 300 <= 400
        assert!(!adapter.check_preload(1000.0, 500.0, 2400.0)); // 2400 - 1500 = 900 > 400

        assert!(adapter.load_next_page());
        assert_eq!(adapter.current_page, 1);
    }
}
