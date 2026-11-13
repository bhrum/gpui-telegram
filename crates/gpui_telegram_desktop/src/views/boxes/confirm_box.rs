//! Authentic GPUI implementation of Telegram Desktop's Confirm & Inform Boxes.
//! Upstream reference: `Telegram/SourceFiles/ui/boxes/confirm_box.h:L13-L71` and `confirm_box.cpp:L17-L160`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::sync::Arc;

/// Arguments for configuring a ConfirmBox or InformBox.
/// Cites upstream `Ui::ConfirmBoxArgs` (`confirm_box.h:L15-L42`).
#[derive(Clone)]
pub struct ConfirmBoxArgs {
    pub title: Option<String>,
    pub text: String,
    pub confirm_text: String,
    pub cancel_text: String,
    pub inform: bool,
    pub strict_cancel: bool,
    pub is_danger: bool,
    pub confirmed: Option<Arc<dyn Fn() + Send + Sync>>,
    pub cancelled: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl Default for ConfirmBoxArgs {
    fn default() -> Self {
        Self {
            title: None,
            text: String::new(),
            confirm_text: "OK".to_string(),
            cancel_text: "Cancel".to_string(),
            inform: false,
            strict_cancel: false,
            is_danger: false,
            confirmed: None,
            cancelled: None,
        }
    }
}

/// ConfirmBox view component.
/// Cites upstream `Ui::ConfirmBox` and `Ui::MakeConfirmBox` (`confirm_box.cpp:L17-L116`).
#[derive(Clone)]
pub struct ConfirmBox {
    pub args: ConfirmBoxArgs,
    pub is_closed: bool,
}

impl ConfirmBox {
    pub fn new(args: ConfirmBoxArgs) -> Self {
        Self {
            args,
            is_closed: false,
        }
    }

    /// Factory method mirroring `Ui::MakeConfirmBox` (`confirm_box.h:L51`, `confirm_box.cpp:L114-L116`).
    pub fn make_confirm_box(args: ConfirmBoxArgs) -> Self {
        Self::new(args)
    }

    /// Factory method mirroring `Ui::MakeInformBox` (`confirm_box.h:L53-L62`).
    pub fn make_inform_box(text: impl Into<String>) -> Self {
        let args = ConfirmBoxArgs {
            text: text.into(),
            inform: true,
            ..Default::default()
        };
        Self::new(args)
    }

    /// Triggers confirmation action and executes callback.
    pub fn confirm(&mut self) {
        if self.is_closed {
            return;
        }
        self.is_closed = true;
        if let Some(ref cb) = self.args.confirmed {
            cb();
        }
    }

    /// Triggers cancellation action and executes callback.
    pub fn cancel(&mut self) {
        if self.is_closed {
            return;
        }
        self.is_closed = true;
        if let Some(ref cb) = self.args.cancelled {
            cb();
        }
    }

    /// Handles keyboard events: Enter/Return activates confirm per `confirm_box.cpp:L82-L92`.
    pub fn handle_enter_key(&mut self) {
        self.confirm();
    }

    /// Handles Escape key: cancels if not strictCancel per `confirm_box.cpp:L103-L111`.
    pub fn handle_esc_key(&mut self) {
        if !self.args.strict_cancel {
            self.cancel();
        } else {
            self.is_closed = true;
        }
    }
}

impl Render for ConfirmBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        let mut card = div()
            .id("confirm_box_card")
            .w(px(380.0))
            .p(px(24.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(10.0))
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(16.0));

        // Optional title: st::boxTitle
        if let Some(ref title) = self.args.title {
            card = card.child(
                div()
                    .text_size(px(18.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0x222222))
                    .child(title.clone()),
            );
        }

        // Body message text: st::boxLabel (`confirm_box.cpp:L33-L40`)
        card = card.child(
            div()
                .text_size(px(14.0))
                .line_height(relative(1.4))
                .text_color(rgb(0x555555))
                .child(self.args.text.clone()),
        );

        // Buttons row
        let mut btn_row = div().flex().justify_end().items_center().gap(px(8.0));

        if !self.args.inform {
            // Cancel button
            btn_row = btn_row.child(
                div()
                    .id("cancel_button")
                    .px(px(16.0))
                    .py(px(8.0))
                    .rounded(px(6.0))
                    .cursor_pointer()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgb(0x707579))
                    .hover(|s| s.bg(rgba(0x7075791A)))
                    .child(self.args.cancel_text.clone()),
            );
        }

        // Confirm button: st::attentionBoxButton or st::defaultBoxButton
        let confirm_color = if self.args.is_danger {
            rgb(0xE53935)
        } else {
            rgb(0x5288C1)
        };

        btn_row = btn_row.child(
            div()
                .id("confirm_button")
                .px(px(16.0))
                .py(px(8.0))
                .rounded(px(6.0))
                .cursor_pointer()
                .text_size(px(14.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(confirm_color)
                .hover(|s| s.bg(rgba(0x5288C11A)))
                .child(self.args.confirm_text.clone()),
        );

        card.child(btn_row).into_any_element()
    }
}
