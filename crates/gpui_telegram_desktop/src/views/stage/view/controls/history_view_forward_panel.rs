//! Forward panel view component for message composer stage.
//! Upstream reference: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_forward_panel.cpp`
//! Upstream header: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_forward_panel.h`

use crate::views::composer::controls::{
    ForwardOptions, ForwardPanelModel, ForwardPanelView, ForwardedItem, NAME_NO_CAPTIONS_VERSION,
    NAME_WITH_CAPTIONS_VERSION,
};
use gpui::*;
use serde::{Deserialize, Serialize};

pub struct HistoryViewForwardPanelViewControls {
    pub view: ForwardPanelView,
}

impl HistoryViewForwardPanelViewControls {
    pub fn new() -> Self {
        Self {
            view: ForwardPanelView::new(),
        }
    }

    pub fn set_items(&mut self, items: Vec<ForwardedItem>) {
        self.view.set_items(items);
    }

    pub fn cycle_options(&mut self) -> ForwardOptions {
        self.view.model.edit_to_next_option()
    }
}

impl Default for HistoryViewForwardPanelViewControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewForwardPanelViewControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_empty = self.view.model.is_empty();
        let header = self.view.model.header_text();

        div()
            .flex()
            .items_center()
            .justify_between()
            .px_3()
            .py_1()
            .bg(rgb(0x18222d))
            .border_l_2()
            .border_color(rgb(0x2481cc))
            .text_xs()
            .child(
                div()
                    .text_color(rgb(0xffffff))
                    .child(if is_empty { "No forwards".to_string() } else { header }),
            )
    }
}
