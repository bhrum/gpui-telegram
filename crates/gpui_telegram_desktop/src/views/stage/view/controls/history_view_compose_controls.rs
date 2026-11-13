//! Compose controls orchestrator component for the central history stage.
//! Upstream reference: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_compose_controls.cpp`
//! Upstream header: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_compose_controls.h`

use crate::views::composer::controls::{
    ComposeControlsMode, ComposeControlsModel, SendButtonType, WriteRestriction, WriteRestrictionType,
};
use gpui::*;
use serde::{Deserialize, Serialize};

pub struct HistoryViewComposeControlsViewControls {
    pub model: ComposeControlsModel,
    pub is_visible: bool,
}

impl HistoryViewComposeControlsViewControls {
    pub fn new() -> Self {
        Self {
            model: ComposeControlsModel::new(),
            is_visible: true,
        }
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.model.set_text(text);
    }

    pub fn set_mode(&mut self, mode: ComposeControlsMode) {
        self.model.mode = mode;
        self.model.update_send_button_type();
    }

    pub fn set_restriction(&mut self, restriction: WriteRestriction) {
        self.model.restriction = restriction;
    }
}

impl Default for HistoryViewComposeControlsViewControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewComposeControlsViewControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_restricted = self.model.restriction.is_restricted();
        let send_text = match self.model.send_button {
            SendButtonType::Send => "Send",
            SendButtonType::RecordVoice => "Record",
            SendButtonType::RecordRound => "Video",
            SendButtonType::Schedule => "Schedule",
            SendButtonType::SaveEdit => "Save",
        };

        let mut root = div()
            .flex()
            .flex_col()
            .w_full()
            .bg(rgb(0x18222d))
            .border_t_1()
            .border_color(rgb(0x2a394a));

        if is_restricted {
            let restriction_msg = self.model.restriction.text.clone();
            root = root.child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .p_3()
                    .text_xs()
                    .text_color(rgb(0xff5252))
                    .child(restriction_msg),
            );
        } else {
            let input_text = if self.model.text.is_empty() {
                "Write a message...".to_string()
            } else {
                self.model.text.clone()
            };
            root = root.child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_3()
                    .py_2()
                    .child(
                        div()
                            .flex_1()
                            .px_3()
                            .py_1p5()
                            .rounded_md()
                            .bg(rgb(0x242f3d))
                            .text_sm()
                            .text_color(rgb(0xffffff))
                            .child(input_text),
                    )
                    .child(
                        div()
                            .px_4()
                            .py_1p5()
                            .rounded_md()
                            .bg(rgb(0x2481cc))
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .child(send_text),
                    ),
            );
        }

        root
    }
}
