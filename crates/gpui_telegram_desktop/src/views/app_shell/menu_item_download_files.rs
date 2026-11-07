//! MenuItemDownloadFiles: Batch download action for selected messages containing files
//! Upstream: Telegram/SourceFiles/menu/menu_item_download_files.h, menu_item_download_files.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct MenuItemDownloadFiles {
    pub selected_item_ids: Vec<i64>,
    pub destination_directory: Option<String>,
}

impl MenuItemDownloadFiles {
    pub fn new() -> Self {
        Self {
            selected_item_ids: Vec::new(),
            destination_directory: None,
        }
    }

    pub fn set_selection(&mut self, ids: Vec<i64>) {
        self.selected_item_ids = ids;
    }

    pub fn set_destination(&mut self, path: impl Into<String>) {
        self.destination_directory = Some(path.into());
    }

    pub fn total_selected(&self) -> usize {
        self.selected_item_ids.len()
    }
}

impl Default for MenuItemDownloadFiles {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for MenuItemDownloadFiles {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0x82868A))
            .child(format!("DownloadFiles: {} items selected", self.selected_item_ids.len()))
    }
}
