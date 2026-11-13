//! AppShell Modal Container: Orchestrates modal layer stack and backdrop effects.
//!
//! Upstream: `Telegram/SourceFiles/ui/layers/standalone_layer_stack.cpp:L88-L172`
//! Upstream: `Telegram/SourceFiles/ui/effects/layer_backdrop.cpp:L210-L286`

use crate::components::ui::effects::LayerBackdropEffects;
use crate::components::ui::layers::{LayerEntry, LayerStackOption, StandaloneLayerStackLayers};
use gpui::prelude::FluentBuilder;
use gpui::*;

#[derive(Debug, Clone)]
pub struct ModalContainer {
    pub stack: StandaloneLayerStackLayers,
    pub backdrop: LayerBackdropEffects,
}

impl ModalContainer {
    pub fn new() -> Self {
        Self {
            stack: StandaloneLayerStackLayers::new(),
            backdrop: LayerBackdropEffects::new(),
        }
    }

    pub fn show_modal(&mut self, title: impl Into<String>, width: f32, height: f32, option: LayerStackOption) -> u64 {
        self.backdrop.start(true);
        self.stack.show_box(title, width, height, option)
    }

    pub fn dismiss_top(&mut self) -> Option<LayerEntry> {
        let entry = self.stack.close_top();
        if !self.stack.has_active_layers() {
            self.backdrop.invalidate();
        }
        entry
    }

    pub fn hide_all(&mut self) {
        self.stack.hide_layers();
        self.backdrop.invalidate();
    }

    pub fn is_modal_active(&self) -> bool {
        self.stack.has_active_layers()
    }

    pub fn current_box(&self) -> Option<&LayerEntry> {
        self.stack.current_box()
    }
}

impl Default for ModalContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for ModalContainer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_modal_active() {
            return div()
                .absolute()
                .w(px(0.0))
                .h(px(0.0))
                .overflow_hidden()
                .into_any_element();
        }

        let box_info = self.current_box().unwrap().clone();
        let dim_alpha = self.backdrop.dim_opacity(1.0);
        let alpha_byte = (dim_alpha * 255.0).clamp(140.0, 220.0) as u8;
        let bg_color = rgba((0x101010 << 8) | (alpha_byte as u32));

        div()
            .id("modal-container-root")
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .occlude()
            .bg(bg_color)
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .w(px(box_info.width))
                    .min_h(px(box_info.height))
                    .bg(rgb(0x182533))
                    .rounded(px(14.0))
                    .shadow_lg()
                    .border_1()
                    .border_color(rgb(0x2B5278))
                    .p(px(20.0))
                    .flex()
                    .flex_col()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .items_center()
                            .pb(px(12.0))
                            .border_b_1()
                            .border_color(rgb(0x223344))
                            .child(
                                div()
                                    .font_weight(FontWeight::BOLD)
                                    .text_size(px(16.0))
                                    .text_color(rgb(0xF5F5F5))
                                    .child(box_info.title.clone()),
                            )
                            .child(
                                div()
                                    .cursor_pointer()
                                    .px(px(8.0))
                                    .py(px(2.0))
                                    .rounded(px(4.0))
                                    .hover(|s| s.bg(rgb(0x243447)))
                                    .text_size(px(14.0))
                                    .text_color(rgb(0x8A9AA8))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _event, _window, cx| {
                                            this.dismiss_top();
                                            cx.notify();
                                        }),
                                    )
                                    .child("✕"),
                            ),
                    )
                    .child(
                        div()
                            .py(px(16.0))
                            .text_size(px(13.0))
                            .text_color(rgb(0xB0C4DE))
                            .child(format!(
                                "{} is active and running with 120fps GPUI Metal acceleration.",
                                box_info.title
                            )),
                    )
                    .child(
                        div()
                            .flex()
                            .justify_end()
                            .pt(px(12.0))
                            .border_t_1()
                            .border_color(rgb(0x223344))
                            .child(
                                div()
                                    .cursor_pointer()
                                    .px(px(16.0))
                                    .py(px(6.0))
                                    .rounded(px(6.0))
                                    .bg(rgb(0x2AABEE))
                                    .hover(|s| s.bg(rgb(0x1E96D8)))
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _event, _window, cx| {
                                            this.dismiss_top();
                                            cx.notify();
                                        }),
                                    )
                                    .child("Done"),
                            ),
                    ),
            )
            .into_any_element()
    }
}
