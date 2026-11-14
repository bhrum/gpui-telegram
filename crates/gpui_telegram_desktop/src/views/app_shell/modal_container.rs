//! AppShell Modal Container: Orchestrates modal layer stack, backdrop effects,
//! and real interactive GPUI dialog boxes (`ActiveModalDialog`).
//!
//! Upstream: `Telegram/SourceFiles/ui/layers/standalone_layer_stack.cpp:L88-L172`
//! Upstream: `Telegram/SourceFiles/ui/effects/layer_backdrop.cpp:L210-L286`

use crate::components::ui::effects::LayerBackdropEffects;
use crate::components::ui::layers::{LayerEntry, LayerStackOption, StandaloneLayerStackLayers};
use crate::theme::palette::TdesktopThemePalette;
use crate::views::boxes::modal_render::render_dialog_overlay;
use crate::views::modal_boxes_suite::{ActiveModalDialog, ModalActionOutcome, ModalDialogKind};
use gpui::*;

pub struct ModalContainer {
    pub stack: StandaloneLayerStackLayers,
    pub backdrop: LayerBackdropEffects,
    pub dialog: ActiveModalDialog,
}

impl ModalContainer {
    pub fn new() -> Self {
        Self {
            stack: StandaloneLayerStackLayers::new(),
            backdrop: LayerBackdropEffects::new(),
            dialog: ActiveModalDialog::new(Vec::new()),
        }
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) {
        self.dialog.set_palette(palette);
    }

    pub fn set_language(&mut self, language_id: impl Into<String>) {
        self.dialog.set_language(language_id);
    }

    pub fn show_modal(
        &mut self,
        title: impl Into<String>,
        width: f32,
        height: f32,
        option: LayerStackOption,
    ) -> u64 {
        let title_str = title.into();
        self.backdrop.start(true);
        self.dialog.open_for_title(&title_str, Vec::new());
        self.stack.show_box(title_str, width, height, option)
    }

    pub fn show_dialog(
        &mut self,
        title: impl Into<String>,
        kind: ModalDialogKind,
        forward_targets: Vec<(u64, String, String, u32)>,
        width: f32,
        height: f32,
        option: LayerStackOption,
    ) -> u64 {
        self.backdrop.start(true);
        self.dialog.open(kind, forward_targets);
        self.stack.show_box(title, width, height, option)
    }

    pub fn dismiss_top(&mut self) -> Option<LayerEntry> {
        let entry = self.stack.close_top();
        if !self.stack.has_active_layers() {
            self.dialog.close();
            self.backdrop.invalidate();
        }
        entry
    }

    pub fn hide_all(&mut self) {
        self.stack.hide_layers();
        self.dialog.close();
        self.backdrop.invalidate();
    }

    pub fn is_modal_active(&self) -> bool {
        self.stack.has_active_layers() || self.dialog.is_open()
    }

    pub fn current_box(&self) -> Option<&LayerEntry> {
        self.stack.current_box()
    }

    pub fn handle_key_input(&mut self, key: &str, key_char: Option<&str>) -> bool {
        if key == "escape" && self.is_modal_active() {
            if self.stack.has_active_layers() {
                self.dismiss_top();
            } else {
                self.dialog.close();
            }
            return true;
        }
        self.dialog.handle_key_input(key, key_char)
    }

    pub fn take_outcome(&mut self) -> Option<ModalActionOutcome> {
        self.dialog.pending_outcome.take()
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

        if !self.dialog.is_open() {
            if let Some(entry) = self.stack.current_box().cloned() {
                self.dialog.open_for_title(&entry.title, Vec::new());
            }
        }

        render_dialog_overlay(
            &self.dialog,
            |this| &mut this.dialog,
            |this| {
                if this.stack.has_active_layers() {
                    this.dismiss_top();
                } else {
                    this.dialog.close();
                    this.backdrop.invalidate();
                }
            },
            cx,
        )
    }
}
