//! Target Symbol: IvEditorToolbarPillEditor
//! Module 13: Instant View Floating Editor Toolbar Pill
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolbarButtonId {
    Bold,
    Italic,
    Underline,
    Strike,
    Code,
    Link,
    Spoiler,
    Heading,
    List,
    Quote,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToolbarButtonState {
    #[default]
    Inactive,
    Active,
    Disabled,
}

#[derive(Debug, Clone)]
pub struct PillButton {
    pub id: ToolbarButtonId,
    pub symbol: &'static str,
    pub tooltip: &'static str,
    pub state: ToolbarButtonState,
}

#[derive(Debug, Clone)]
pub struct IvEditorToolbarPillEditor {
    pub buttons: Vec<PillButton>,
    pub is_visible: bool,
    pub x: f32,
    pub y: f32,
    pub ripple_size: f32,
    pub button_pad: f32,
    pub selected_button: Option<ToolbarButtonId>,
}

impl Default for IvEditorToolbarPillEditor {
    fn default() -> Self {
        Self {
            buttons: vec![
                PillButton {
                    id: ToolbarButtonId::Bold,
                    symbol: "B",
                    tooltip: "Bold (Ctrl+B)",
                    state: ToolbarButtonState::Inactive,
                },
                PillButton {
                    id: ToolbarButtonId::Italic,
                    symbol: "I",
                    tooltip: "Italic (Ctrl+I)",
                    state: ToolbarButtonState::Inactive,
                },
                PillButton {
                    id: ToolbarButtonId::Underline,
                    symbol: "U",
                    tooltip: "Underline (Ctrl+U)",
                    state: ToolbarButtonState::Inactive,
                },
                PillButton {
                    id: ToolbarButtonId::Strike,
                    symbol: "S",
                    tooltip: "Strikethrough (Ctrl+Shift+X)",
                    state: ToolbarButtonState::Inactive,
                },
                PillButton {
                    id: ToolbarButtonId::Code,
                    symbol: "<>",
                    tooltip: "Monospace (Ctrl+Shift+M)",
                    state: ToolbarButtonState::Inactive,
                },
                PillButton {
                    id: ToolbarButtonId::Link,
                    symbol: "🔗",
                    tooltip: "Create Link (Ctrl+K)",
                    state: ToolbarButtonState::Inactive,
                },
                PillButton {
                    id: ToolbarButtonId::Spoiler,
                    symbol: "👁",
                    tooltip: "Spoiler (Ctrl+Shift+P)",
                    state: ToolbarButtonState::Inactive,
                },
            ],
            is_visible: true,
            x: 0.0,
            y: 0.0,
            ripple_size: 32.0,
            button_pad: 4.0,
            selected_button: None,
        }
    }
}

impl IvEditorToolbarPillEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn natural_size(&self) -> (f32, f32) {
        let count = self.buttons.len() as f32;
        let width = self.button_pad * 2.0 + count * self.ripple_size + (count - 1.0).max(0.0) * self.button_pad;
        let height = self.ripple_size + self.button_pad * 2.0;
        (width, height)
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_button_state(&mut self, id: ToolbarButtonId, state: ToolbarButtonState) {
        if let Some(btn) = self.buttons.iter_mut().find(|b| b.id == id) {
            btn.state = state;
        }
    }

    pub fn toggle_button(&mut self, id: ToolbarButtonId) {
        if let Some(btn) = self.buttons.iter_mut().find(|b| b.id == id) {
            btn.state = match btn.state {
                ToolbarButtonState::Inactive => ToolbarButtonState::Active,
                ToolbarButtonState::Active => ToolbarButtonState::Inactive,
                ToolbarButtonState::Disabled => ToolbarButtonState::Disabled,
            };
            self.selected_button = Some(id);
        }
    }
}

impl Render for IvEditorToolbarPillEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }

        let mut row = div()
            .flex()
            .flex_row()
            .items_center()
            .bg(rgb(0x1F242D))
            .rounded(px(18.0))
            .shadow_lg()
            .border_1()
            .border_color(rgb(0x353B48))
            .p(px(self.button_pad))
            .gap(px(self.button_pad));

        for btn in &self.buttons {
            let (bg, text_color) = match btn.state {
                ToolbarButtonState::Active => (rgb(0x2AABEE), rgb(0xFFFFFF)),
                ToolbarButtonState::Inactive => (rgb(0x1F242D), rgb(0xCCD2DE)),
                ToolbarButtonState::Disabled => (rgb(0x1F242D), rgb(0x5A606E)),
            };

            row = row.child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(self.ripple_size))
                    .h(px(self.ripple_size))
                    .rounded(px(14.0))
                    .bg(bg)
                    .text_color(text_color)
                    .font_weight(if btn.id == ToolbarButtonId::Bold {
                        FontWeight::BOLD
                    } else {
                        FontWeight::NORMAL
                    })
                    .text_size(px(13.0))
                    .hover(|s| {
                        if btn.state == ToolbarButtonState::Inactive {
                            s.bg(rgb(0x2C3340))
                        } else {
                            s
                        }
                    })
                    .child(btn.symbol),
            );
        }

        row.into_any_element()
    }
}
