//! Upstream: Telegram/SourceFiles/info/global_media/info_global_media_provider.h
//! Upstream: Telegram/SourceFiles/info/global_media/info_global_media_provider.cpp
//! Subsystem: info | Feature ID: TCG-N23 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use super::info_global_media_inner_widget::SharedMediaType;

pub const K_MINIMAL_IDS_LIMIT: i32 = 16;
pub const K_PRELOADED_SCREENS_COUNT: i32 = 4;
pub const K_PRELOADED_SCREENS_COUNT_FULL: i32 = 9; // 4 + 1 + 4
pub const K_PRELOAD_IF_LESS_THAN_SCREENS: i32 = 2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalMediaKey {
    pub around_id: u64,
}

#[derive(Debug, Clone)]
pub struct GlobalMediaSlice {
    pub key: GlobalMediaKey,
    pub items: Vec<u64>,
    pub full_count: Option<i32>,
    pub skipped_after: i32,
}

impl GlobalMediaSlice {
    pub fn new(key: GlobalMediaKey, items: Vec<u64>, full_count: Option<i32>, skipped_after: i32) -> Self {
        Self {
            key,
            items,
            full_count,
            skipped_after,
        }
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn index_of(&self, id: u64) -> Option<usize> {
        self.items.iter().position(|&x| x == id)
    }

    pub fn skipped_before(&self) -> Option<i32> {
        self.full_count.map(|fc| fc - self.skipped_after - (self.items.len() as i32))
    }

    pub fn distance(&self, a: &GlobalMediaKey, b: &GlobalMediaKey) -> Option<i32> {
        let i = self.index_of(a.around_id)?;
        let j = self.index_of(b.around_id)?;
        Some((j as i32) - (i as i32))
    }

    pub fn nearest(&self, id: u64) -> Option<u64> {
        if self.items.is_empty() {
            return None;
        }
        // Items sorted descending by message position
        for &item in &self.items {
            if item <= id {
                return Some(item);
            }
        }
        self.items.last().copied()
    }
}

#[derive(Debug, Clone)]
pub struct InfoGlobalMediaProviderGlobalMedia {
    pub media_type: SharedMediaType,
    pub current_query: String,
    pub generation: u64,
    pub around_id: u64,
    pub ids_limit: i32,
    pub slice: GlobalMediaSlice,
    pub only_forwardable: bool,
}

impl InfoGlobalMediaProviderGlobalMedia {
    pub fn new(media_type: SharedMediaType) -> Self {
        Self {
            media_type,
            current_query: String::new(),
            generation: 1,
            around_id: u64::MAX,
            ids_limit: K_MINIMAL_IDS_LIMIT,
            slice: GlobalMediaSlice::new(GlobalMediaKey { around_id: u64::MAX }, Vec::new(), None, 0),
            only_forwardable: false,
        }
    }

    pub fn min_item_height(&self, width: i32) -> i32 {
        match self.media_type {
            SharedMediaType::Photo | SharedMediaType::Video => (width / 3).max(80),
            SharedMediaType::File => 56,
            SharedMediaType::Audio | SharedMediaType::RoundVoiceFile => 48,
            SharedMediaType::Link => 64,
        }
    }

    /// Upstream logic: `sectionHasFloatingHeader`
    pub fn section_has_floating_header(&self) -> bool {
        match self.media_type {
            SharedMediaType::Photo
            | SharedMediaType::Video
            | SharedMediaType::Audio
            | SharedMediaType::RoundVoiceFile => false,
            SharedMediaType::File | SharedMediaType::Link => true,
        }
    }

    pub fn set_search_query(&mut self, query: impl Into<String>) {
        self.current_query = query.into();
        self.generation += 1;
        self.around_id = u64::MAX;
        self.ids_limit = K_MINIMAL_IDS_LIMIT;
    }

    /// Upstream preloading logic: `checkPreload`
    pub fn compute_preload_limits(&self, viewport_height: i32, viewport_width: i32) -> (i32, i32) {
        let min_height = self.min_item_height(viewport_width).max(1);
        let preloaded_height = K_PRELOADED_SCREENS_COUNT_FULL * viewport_height;
        let preloaded_count = preloaded_height / min_height;
        let preload_ids_limit_min = (preloaded_count / 2) + 1;
        let preload_ids_limit = preload_ids_limit_min + (viewport_height / min_height);
        (preload_ids_limit_min, preload_ids_limit)
    }

    pub fn check_preload_required(&self, viewport_height: i32, viewport_width: i32, item_position: u64) -> bool {
        let (min_limit, _) = self.compute_preload_limits(viewport_height, viewport_width);
        if self.ids_limit < min_limit {
            return true;
        }
        let key_current = GlobalMediaKey { around_id: self.around_id };
        let key_target = GlobalMediaKey { around_id: item_position };
        if let Some(delta) = self.slice.distance(&key_current, &key_target) {
            let min_height = self.min_item_height(viewport_width).max(1);
            let min_screen_delta = K_PRELOADED_SCREENS_COUNT - K_PRELOAD_IF_LESS_THAN_SCREENS;
            let min_delta = (min_screen_delta * viewport_height) / min_height;
            delta.abs() >= min_delta
        } else {
            true
        }
    }
}

impl Default for InfoGlobalMediaProviderGlobalMedia {
    fn default() -> Self {
        Self::new(SharedMediaType::Photo)
    }
}

impl Render for InfoGlobalMediaProviderGlobalMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let type_name = match self.media_type {
            SharedMediaType::Photo => "Photos",
            SharedMediaType::Video => "Videos",
            SharedMediaType::File => "Files",
            SharedMediaType::Audio => "Audio",
            SharedMediaType::Link => "Links",
            SharedMediaType::RoundVoiceFile => "Voice Messages",
        };

        div()
            .flex()
            .flex_row()
            .justify_between()
            .items_center()
            .p(px(8.0))
            .bg(rgb(0x17212b))
            .rounded(px(4.0))
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x707579))
                    .child(format!("Category: {}", type_name)),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x5288c1))
                    .child(format!("Loaded: {} items", self.slice.size())),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_media_slice_distance_and_nearest() {
        let items = vec![100, 90, 80, 70, 60, 50];
        let slice = GlobalMediaSlice::new(
            GlobalMediaKey { around_id: 100 },
            items,
            Some(10),
            2,
        );

        assert_eq!(slice.size(), 6);
        assert_eq!(slice.skipped_before(), Some(2)); // 10 - 2 - 6 = 2

        let k1 = GlobalMediaKey { around_id: 90 };
        let k2 = GlobalMediaKey { around_id: 70 };
        assert_eq!(slice.distance(&k1, &k2), Some(2)); // index 3 - index 1 = 2

        assert_eq!(slice.nearest(85), Some(80));
        assert_eq!(slice.nearest(120), Some(100));
        assert_eq!(slice.nearest(40), Some(50));
    }

    #[test]
    fn test_section_floating_header_and_preload_math() {
        let photo_provider = InfoGlobalMediaProviderGlobalMedia::new(SharedMediaType::Photo);
        assert!(!photo_provider.section_has_floating_header());

        let file_provider = InfoGlobalMediaProviderGlobalMedia::new(SharedMediaType::File);
        assert!(file_provider.section_has_floating_header());

        let (min_lim, full_lim) = file_provider.compute_preload_limits(600, 360);
        assert!(min_lim > 0);
        assert!(full_lim >= min_lim);
    }
}
