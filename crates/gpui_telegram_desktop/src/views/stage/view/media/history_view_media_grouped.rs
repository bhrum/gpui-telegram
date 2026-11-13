//! Authentic GPUI History View Media Grouped Component.
//!
//! Upstream:
//! - `Telegram/SourceFiles/history/view/media/history_view_media_grouped.h:L1-L167`
//! - `Telegram/SourceFiles/history/view/media/history_view_media_grouped.cpp:L1-L962`
//!
//! Constants & Mosaic Algorithm:
//! - `kMaxSize = 10` (`history_view_media_grouped.h:L22`)
//! - `DetectMode`: Document without video/animation -> `Mode::Column`, else `Mode::Grid` (`history_view_media_grouped.cpp:L133-L140`)
//! - Proportional scaling math: `factor = newWidth / maxWidth`, `scale(v) = round(v * factor)` (`history_view_media_grouped.cpp:L227-L257`)

use crate::views::stage::media::{GroupedMediaItem, GroupedMediaLayoutGeometry, GroupedMediaMode, GroupedMediaView, MAX_GROUP_SIZE};

pub const K_MAX_GROUPED_MEDIA_ITEMS: usize = 10;

/// Container component for grouped albums and file playlists in history transcript.
/// Upstream: `HistoryView::GroupedMedia` (`history_view_media_grouped.h:L20-L164`)
#[derive(Debug, Clone)]
pub struct HistoryViewMediaGroupedViewMedia {
    pub mode: GroupedMediaMode,
    pub items: Vec<GroupedMediaItem>,
    pub layout_geometry: Vec<GroupedMediaLayoutGeometry>,
    pub total_width: f32,
    pub total_height: f32,
}

impl HistoryViewMediaGroupedViewMedia {
    pub fn new(mode: GroupedMediaMode) -> Self {
        Self {
            mode,
            items: Vec::new(),
            layout_geometry: Vec::new(),
            total_width: 0.0,
            total_height: 0.0,
        }
    }

    pub fn add_item(&mut self, item: GroupedMediaItem) -> bool {
        if self.items.len() >= MAX_GROUP_SIZE {
            return false;
        }
        self.items.push(item);
        true
    }

    pub fn recount_layout(&mut self, target_width: f32) {
        self.layout_geometry = GroupedMediaView::compute_layout(&self.items, self.mode, target_width);
        self.total_width = target_width;
        self.total_height = self.layout_geometry.iter().map(|g| g.y + g.height).fold(0.0f32, f32::max);
    }
}
