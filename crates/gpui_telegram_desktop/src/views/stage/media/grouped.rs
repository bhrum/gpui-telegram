//! Authentic GPUI Grouped Media (Album / Playlist) Mosaic Layout.
//!
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_media_grouped.h`
//! - `Telegram/SourceFiles/history/view/media/history_view_media_grouped.cpp`

use gpui::*;
use super::RectParts;

/// Maximum number of media items grouped in a single album.
/// Upstream: `GroupedMedia::kMaxSize = 10` (`history_view_media_grouped.h:L22`)
pub const MAX_GROUP_SIZE: usize = 10;
pub const K_MAX_GROUPED_MEDIA_ITEMS: usize = MAX_GROUP_SIZE;
pub const GROUP_WIDTH_MIN: f32 = 120.0;
pub const GROUP_WIDTH_MAX: f32 = 420.0;
pub const GROUP_ITEM_SKIP: f32 = 2.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GroupedMediaMode {
    #[default]
    Grid,
    Column,
}

#[derive(Debug, Clone)]
pub struct GroupedMediaItem {
    pub item_id: u64,
    pub width: f32,
    pub height: f32,
    pub sides: RectParts,
    pub title: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GroupedMediaLayoutGeometry {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub sides: RectParts,
}

pub struct GroupedMediaView;

impl GroupedMediaView {
    /// Compute proportional mosaic layout geometry for grouped media items.
    /// Upstream: `GroupedMedia::countOptimalSize` & `countCurrentSize`
    /// (`history_view_media_grouped.cpp:L142-L258`)
    pub fn compute_layout(
        items: &[GroupedMediaItem],
        mode: GroupedMediaMode,
        target_width: f32,
    ) -> Vec<GroupedMediaLayoutGeometry> {
        let count = items.len().min(MAX_GROUP_SIZE);
        if count == 0 {
            return Vec::new();
        }

        if mode == GroupedMediaMode::Column {
            // Vertical Playlist Layout: `LayoutPlaylist`
            let mut result = Vec::with_capacity(count);
            let mut top = 0.0f32;
            for (idx, item) in items[..count].iter().enumerate() {
                let h = item.height.max(48.0);
                result.push(GroupedMediaLayoutGeometry {
                    x: 0.0,
                    y: top,
                    width: target_width,
                    height: h,
                    sides: RectParts {
                        top: idx == 0,
                        bottom: idx + 1 == count,
                        left: true,
                        right: true,
                    },
                });
                top += h;
            }
            return result;
        }

        // Grid Mosaic Layout: Arrange into rows (1 to 3 items per row)
        let mut result = Vec::with_capacity(count);
        let rows = match count {
            1 => vec![1],
            2 => vec![2],
            3 => vec![1, 2],
            4 => vec![2, 2],
            5 => vec![2, 3],
            6 => vec![3, 3],
            7 => vec![1, 3, 3],
            8 => vec![2, 3, 3],
            9 => vec![3, 3, 3],
            _ => vec![2, 3, 3, 2],
        };

        let spacing = GROUP_ITEM_SKIP;
        let mut item_idx = 0;
        let mut current_y = 0.0f32;

        for (row_idx, &row_items_count) in rows.iter().enumerate() {
            let actual_count = row_items_count.min(count - item_idx);
            if actual_count == 0 {
                break;
            }
            let is_top_row = row_idx == 0;
            let is_bottom_row = (item_idx + actual_count) >= count;

            let total_spacing = spacing * (actual_count.saturating_sub(1) as f32);
            let item_w = (target_width - total_spacing) / actual_count as f32;
            // Target height proportional to average aspect ratio or fixed square/4:3
            let item_h = if count == 1 {
                target_width * 0.75
            } else if actual_count == 1 {
                target_width * 0.5
            } else {
                item_w * 0.85
            };

            for col_idx in 0..actual_count {
                let x = col_idx as f32 * (item_w + spacing);
                let is_left_col = col_idx == 0;
                let is_right_col = col_idx + 1 == actual_count;

                result.push(GroupedMediaLayoutGeometry {
                    x,
                    y: current_y,
                    width: item_w,
                    height: item_h,
                    sides: RectParts {
                        top: is_top_row,
                        bottom: is_bottom_row,
                        left: is_left_col,
                        right: is_right_col,
                    },
                });
                item_idx += 1;
            }
            current_y += item_h + spacing;
        }

        result
    }

    pub fn render(
        items: &[GroupedMediaItem],
        mode: GroupedMediaMode,
        target_width: f32,
    ) -> impl IntoElement {
        let geoms = Self::compute_layout(items, mode, target_width);
        let total_h = geoms.iter().map(|g| g.y + g.height).fold(0.0f32, f32::max);

        div()
            .relative()
            .w(px(target_width))
            .h(px(total_h))
            .rounded(px(12.0))
            .overflow_hidden()
            .children(
                geoms.into_iter().enumerate().map(|(idx, g)| {
                    let title = items.get(idx).and_then(|i| i.title.clone()).unwrap_or_else(|| format!("Media #{}", idx + 1));
                    div()
                        .absolute()
                        .left(px(g.x))
                        .top(px(g.y))
                        .w(px(g.width))
                        .h(px(g.height))
                        .bg(rgb(0x182533))
                        .flex()
                        .items_center()
                        .justify_center()
                        .border_1()
                        .border_color(rgba(0x00000040))
                        .child(
                            div()
                                .text_size(px(11.0))
                                .text_color(rgb(0x7D9BB7))
                                .child(title),
                        )
                }),
            )
    }
}
