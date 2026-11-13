//! WindowFiltersFavorite: Pinned and favorite folder filter list manager
//! Upstream: Telegram/SourceFiles/window/window_filters_favorite.h, window_filters_favorite.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct WindowFiltersFavorite {
    pub favorite_ids: Vec<i32>,
}

impl WindowFiltersFavorite {
    pub fn new() -> Self {
        Self {
            favorite_ids: vec![0, 1],
        }
    }

    pub fn toggle_favorite(&mut self, folder_id: i32) -> bool {
        if let Some(pos) = self.favorite_ids.iter().position(|&id| id == folder_id) {
            self.favorite_ids.remove(pos);
            false
        } else {
            self.favorite_ids.push(folder_id);
            true
        }
    }

    pub fn is_favorite(&self, folder_id: i32) -> bool {
        self.favorite_ids.contains(&folder_id)
    }

    pub fn reorder(&mut self, from_idx: usize, to_idx: usize) {
        if from_idx < self.favorite_ids.len() && to_idx < self.favorite_ids.len() {
            let item = self.favorite_ids.remove(from_idx);
            self.favorite_ids.insert(to_idx, item);
        }
    }
}

impl Default for WindowFiltersFavorite {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowFiltersFavorite {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0x82868A))
            .child(format!("Favorite Filters: {:?}", self.favorite_ids))
    }
}
