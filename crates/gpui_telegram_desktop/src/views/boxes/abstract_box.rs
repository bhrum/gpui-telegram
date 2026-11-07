//! Authentic GPUI implementation of Telegram Desktop's Abstract Box & Layering Container.
//! Upstream reference: `Telegram/SourceFiles/boxes/abstract_box.h:L28-L51` and `abstract_box.cpp:L15-L48`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Box layer presentation states matching upstream `Ui::LayerOptions` and `Ui::LayerWidget`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AbstractBoxState {
    #[default]
    Hidden,
    Opening,
    Shown,
    Closing,
}

/// Generic action button definition for modal footer bars.
#[derive(Clone)]
pub struct BoxActionButton {
    pub label: String,
    pub is_danger: bool,
    pub is_primary: bool,
    pub on_click: Option<std::sync::Arc<dyn Fn(&mut Window, &mut App) + Send + Sync>>,
}

impl std::fmt::Debug for BoxActionButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoxActionButton")
            .field("label", &self.label)
            .field("is_danger", &self.is_danger)
            .field("is_primary", &self.is_primary)
            .finish()
    }
}

/// AbstractBox represents the top-level modal backdrop layer and dialog window container.
/// Cites upstream:
/// - `Telegram/SourceFiles/boxes/abstract_box.h:L39-L47`: `Ui::show(content, options, animated)`
/// - `Telegram/SourceFiles/boxes/abstract_box.cpp:L32-L46`: `Ui::hideLayer`, `Ui::isLayerShown`
#[derive(Clone)]
pub struct AbstractBox {
    pub title: String,
    pub subtitle: Option<String>,
    pub width: Pixels,
    pub close_on_click_outside: bool,
    pub state: AbstractBoxState,
    pub layer_count: usize,
    pub buttons: Vec<BoxActionButton>,
    pub on_close: Option<std::sync::Arc<dyn Fn() + Send + Sync>>,
}

impl AbstractBox {
    /// Upstream metric tokens: `st::boxWidth` (380px), `st::boxRadius` (10px).
    pub fn new() -> Self {
        Self {
            title: String::new(),
            subtitle: None,
            width: px(380.0),
            close_on_click_outside: true,
            state: AbstractBoxState::Shown,
            layer_count: 1,
            buttons: Vec::new(),
            on_close: None,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_width(mut self, width: Pixels) -> Self {
        self.width = width;
        self
    }

    pub fn with_close_on_click_outside(mut self, close: bool) -> Self {
        self.close_on_click_outside = close;
        self
    }

    pub fn add_button(&mut self, label: impl Into<String>, is_primary: bool, is_danger: bool) {
        self.buttons.push(BoxActionButton {
            label: label.into(),
            is_danger,
            is_primary,
            on_click: None,
        });
    }

    /// Mirrors `Ui::isLayerShown()` from `abstract_box.cpp:L41-L46`.
    pub fn is_layer_shown(&self) -> bool {
        self.state == AbstractBoxState::Shown || self.state == AbstractBoxState::Opening
    }

    /// Mirrors `Ui::hideLayer()` from `abstract_box.cpp:L32-L39`.
    pub fn hide_layer(&mut self) -> usize {
        self.state = AbstractBoxState::Hidden;
        if self.layer_count > 0 {
            self.layer_count -= 1;
        }
        if let Some(ref cb) = self.on_close {
            cb();
        }
        self.layer_count
    }

    /// Mirrors `Ui::internal::showBox()` from `abstract_box.cpp:L18-L28`.
    pub fn show_box(&mut self) -> bool {
        self.state = AbstractBoxState::Shown;
        self.layer_count += 1;
        true
    }
}

impl Default for AbstractBox {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for AbstractBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_layer_shown() {
            return div().into_any_element();
        }

        // Upstream styling: st::layerBg = rgba(0, 0, 0, 0.45)
        let backdrop = div()
            .id("box_backdrop")
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgba(0x00000073));

        // Dialog container: st::boxRadius = 10px, st::boxBg = #FFFFFF
        let mut dialog = div()
            .id("box_container")
            .w(self.width)
            .max_w(px(560.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(10.0))
            .shadow_lg()
            .p(px(24.0))
            .flex()
            .flex_col()
            .gap(px(16.0));

        // Title row: st::boxTitleStyle
        if !self.title.is_empty() {
            dialog = dialog.child(
                div()
                    .text_size(px(18.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0x222222))
                    .child(self.title.clone()),
            );
        }

        // Action buttons row (aligned right per tdesktop button layout)
        if !self.buttons.is_empty() {
            let mut btn_row = div().flex().justify_end().items_center().gap(px(8.0));
            for btn in &self.buttons {
                let text_color = if btn.is_danger {
                    rgb(0xE53935)
                } else if btn.is_primary {
                    rgb(0x5288C1)
                } else {
                    rgb(0x707579)
                };

                btn_row = btn_row.child(
                    div()
                        .px(px(16.0))
                        .py(px(8.0))
                        .rounded(px(6.0))
                        .cursor_pointer()
                        .hover(|s| s.bg(rgba(0x5288C11A)))
                        .text_size(px(14.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(text_color)
                        .child(btn.label.clone()),
                );
            }
            dialog = dialog.child(btn_row);
        }

        backdrop.child(dialog).into_any_element()
    }
}
