//! Upstream: `Telegram/SourceFiles/ui/layers/standalone_layer_stack.cpp:L88-L172`, `L212-L231`
//! Upstream: `Telegram/SourceFiles/ui/layers/standalone_layer_stack.h:L36-L86`
//!
//! Autonomous GPUI Modal Layer Stack implementation. Manages a stack of modal
//! boxes/panels where pushing a new box dims/hides underlying layers and closing
//! the top box reactivates the previous entry.

use gpui::*;
use gpui::prelude::FluentBuilder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerStackOption {
    Normal,
    CloseOther,
}

#[derive(Debug, Clone)]
pub struct LayerEntry {
    pub id: u64,
    pub title: String,
    pub is_modal: bool,
    pub width: f32,
    pub height: f32,
    pub is_closing: bool,
}

#[derive(Debug, Clone)]
pub struct StandaloneLayerStackLayers {
    entries: Vec<LayerEntry>,
    closing: Vec<LayerEntry>,
    next_id: u64,
    anchor_size: Option<(f32, f32)>,
    box_added_count: usize,
    box_closed_count: usize,
}

impl StandaloneLayerStackLayers {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            closing: Vec::new(),
            next_id: 1,
            anchor_size: Some((800.0, 600.0)),
            box_added_count: 0,
            box_closed_count: 0,
        }
    }

    /// C++: `StandaloneLayerStack::showBox(box, options, animated)`
    pub fn show_box(&mut self, title: impl Into<String>, width: f32, height: f32, option: LayerStackOption) -> u64 {
        if option == LayerStackOption::CloseOther {
            self.hide_layers();
        }
        let id = self.next_id;
        self.next_id += 1;
        self.entries.push(LayerEntry {
            id,
            title: title.into(),
            is_modal: true,
            width: if width > 0.0 { width } else { 480.0 },
            height: if height > 0.0 { height } else { 320.0 },
            is_closing: false,
        });
        self.box_added_count += 1;
        id
    }

    /// C++: `StandaloneLayerStack::hideLayers(animated)`
    pub fn hide_layers(&mut self) {
        for mut entry in self.entries.drain(..) {
            entry.is_closing = true;
            self.closing.push(entry);
            self.box_closed_count += 1;
        }
    }

    /// C++: `StandaloneLayerStack::closeEntry(panel)`
    pub fn close_top(&mut self) -> Option<LayerEntry> {
        if let Some(mut entry) = self.entries.pop() {
            entry.is_closing = true;
            self.closing.push(entry.clone());
            self.box_closed_count += 1;
            Some(entry)
        } else {
            None
        }
    }

    pub fn current_box(&self) -> Option<&LayerEntry> {
        self.entries.last()
    }

    pub fn layer_count(&self) -> usize {
        self.entries.len()
    }

    pub fn has_active_layers(&self) -> bool {
        !self.entries.is_empty()
    }

    pub fn box_added_events(&self) -> usize {
        self.box_added_count
    }

    pub fn box_closed_events(&self) -> usize {
        self.box_closed_count
    }

    pub fn set_anchor_size(&mut self, width: f32, height: f32) {
        self.anchor_size = Some((width, height));
    }
}

impl Default for StandaloneLayerStackLayers {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for StandaloneLayerStackLayers {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let active = self.current_box();
        div()
            .relative()
            .size_full()
            .when(active.is_some(), |el| {
                let box_info = active.unwrap();
                el.child(
                    div()
                        .absolute()
                        .size_full()
                        .bg(rgba(0x10101088))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .w(px(box_info.width))
                                .min_h(px(box_info.height))
                                .bg(rgb(0x182533))
                                .rounded(px(12.0))
                                .shadow_lg()
                                .p(px(16.0))
                                .flex()
                                .flex_col()
                                .child(
                                    div()
                                        .text_size(px(16.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0xF5F5F5))
                                        .child(box_info.title.clone())
                                )
                        )
                )
            })
    }
}
