//! Instant View Editor Box Dialog & Toolbar Host (`iv_editor_box.h/.cpp`)
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

/// Complete set of toolbar actions available in the Instant View article editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolbarActionId {
    Undo,
    Redo,
    Bold,
    Italic,
    Underline,
    StrikeOut,
    Spoiler,
    Subscript,
    Superscript,
    Marked,
    PlainText,
    Heading,
    Link,
    Math,
    Blockquote,
    Pullquote,
    CodeBlock,
    Details,
    OrderedList,
    BulletList,
    TaskList,
    Attach,
    Table,
    Location,
    Divider,
}

impl ToolbarActionId {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Undo => "Undo",
            Self::Redo => "Redo",
            Self::Bold => "Bold",
            Self::Italic => "Italic",
            Self::Underline => "Underline",
            Self::StrikeOut => "Strikethrough",
            Self::Spoiler => "Spoiler",
            Self::Subscript => "Subscript",
            Self::Superscript => "Superscript",
            Self::Marked => "Highlight",
            Self::PlainText => "Plain Text",
            Self::Heading => "Heading",
            Self::Link => "Link",
            Self::Math => "LaTeX Math",
            Self::Blockquote => "Quote",
            Self::Pullquote => "Pullquote",
            Self::CodeBlock => "Code Block",
            Self::Details => "Details",
            Self::OrderedList => "Numbered List",
            Self::BulletList => "Bullet List",
            Self::TaskList => "Task List",
            Self::Attach => "Attach Media",
            Self::Table => "Table",
            Self::Location => "Location Map",
            Self::Divider => "Horizontal Divider",
        }
    }
}

/// Visual activation state of toolbar icon buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToolbarButtonState {
    Disabled,
    #[default]
    Inactive,
    Active,
}

/// Item within the editor toolbar.
#[derive(Debug, Clone)]
pub struct ToolbarActionItem {
    pub id: ToolbarActionId,
    pub state: ToolbarButtonState,
    pub is_premium: bool,
}

/// Policy for committing changes from the editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SubmitType {
    #[default]
    Send,
    Save,
}

/// Configuration parameters for displaying an Instant View editor window.
#[derive(Debug, Clone)]
pub struct ShowWindowDescriptor {
    pub peer_id: u64,
    pub title: String,
    pub submit_label: String,
    pub submit_type: SubmitType,
    pub is_premium: bool,
}

impl Default for ShowWindowDescriptor {
    fn default() -> Self {
        Self {
            peer_id: 0,
            title: "Instant View Article Editor".to_string(),
            submit_label: "Publish".to_string(),
            submit_type: SubmitType::Send,
            is_premium: false,
        }
    }
}

/// Primary editor window controller and toolbar host.
#[derive(Debug, Clone)]
pub struct IvEditorBoxEditor {
    pub descriptor: ShowWindowDescriptor,
    pub is_open: bool,
    pub toolbar_actions: Vec<ToolbarActionItem>,
    pub active_action: Option<ToolbarActionId>,
    pub action_history: Vec<ToolbarActionId>,
}

impl Default for IvEditorBoxEditor {
    fn default() -> Self {
        let default_actions = vec![
            ToolbarActionItem { id: ToolbarActionId::Undo, state: ToolbarButtonState::Inactive, is_premium: false },
            ToolbarActionItem { id: ToolbarActionId::Redo, state: ToolbarButtonState::Inactive, is_premium: false },
            ToolbarActionItem { id: ToolbarActionId::Bold, state: ToolbarButtonState::Inactive, is_premium: false },
            ToolbarActionItem { id: ToolbarActionId::Italic, state: ToolbarButtonState::Inactive, is_premium: false },
            ToolbarActionItem { id: ToolbarActionId::Underline, state: ToolbarButtonState::Inactive, is_premium: false },
            ToolbarActionItem { id: ToolbarActionId::StrikeOut, state: ToolbarButtonState::Inactive, is_premium: false },
            ToolbarActionItem { id: ToolbarActionId::Heading, state: ToolbarButtonState::Inactive, is_premium: false },
            ToolbarActionItem { id: ToolbarActionId::Blockquote, state: ToolbarButtonState::Inactive, is_premium: false },
            ToolbarActionItem { id: ToolbarActionId::CodeBlock, state: ToolbarButtonState::Inactive, is_premium: false },
            ToolbarActionItem { id: ToolbarActionId::OrderedList, state: ToolbarButtonState::Inactive, is_premium: false },
            ToolbarActionItem { id: ToolbarActionId::BulletList, state: ToolbarButtonState::Inactive, is_premium: false },
            ToolbarActionItem { id: ToolbarActionId::Math, state: ToolbarButtonState::Inactive, is_premium: true },
            ToolbarActionItem { id: ToolbarActionId::Table, state: ToolbarButtonState::Inactive, is_premium: true },
        ];
        Self {
            descriptor: ShowWindowDescriptor::default(),
            is_open: true,
            toolbar_actions: default_actions,
            active_action: None,
            action_history: Vec::new(),
        }
    }
}

impl IvEditorBoxEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_descriptor(mut self, descriptor: ShowWindowDescriptor) -> Self {
        self.descriptor = descriptor;
        self
    }

    pub fn trigger_action(&mut self, id: ToolbarActionId) -> bool {
        if let Some(item) = self.toolbar_actions.iter_mut().find(|a| a.id == id) {
            if item.state == ToolbarButtonState::Disabled {
                return false;
            }
            item.state = match item.state {
                ToolbarButtonState::Active => ToolbarButtonState::Inactive,
                _ => ToolbarButtonState::Active,
            };
            self.active_action = Some(id);
            self.action_history.push(id);
            true
        } else {
            false
        }
    }

    pub fn set_button_state(&mut self, id: ToolbarActionId, state: ToolbarButtonState) {
        if let Some(item) = self.toolbar_actions.iter_mut().find(|a| a.id == id) {
            item.state = state;
        }
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }
}

impl Render for IvEditorBoxEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let active_count = self.toolbar_actions.iter().filter(|a| a.state == ToolbarButtonState::Active).count();
        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(16.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .font_weight(FontWeight::BOLD)
                            .text_size(px(16.0))
                            .child(self.descriptor.title.clone())
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(0x888888))
                            .child(format!("Submit: {:?}", self.descriptor.submit_type))
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(px(6.0))
                    .mt(px(12.0))
                    .p(px(8.0))
                    .bg(rgb(0xF5F5F5))
                    .rounded(px(6.0))
                    .children(self.toolbar_actions.iter().map(|item| {
                        let is_active = item.state == ToolbarButtonState::Active;
                        div()
                            .px(px(8.0))
                            .py(px(4.0))
                            .rounded(px(4.0))
                            .text_size(px(12.0))
                            .bg(if is_active { rgb(0x2AABEE) } else { rgb(0xFFFFFF) })
                            .text_color(if is_active { rgb(0xFFFFFF) } else { rgb(0x333333) })
                            .child(item.id.label())
                    }))
            )
            .child(
                div()
                    .mt(px(8.0))
                    .text_size(px(12.0))
                    .text_color(rgb(0x666666))
                    .child(format!(
                        "Open: {} | Active tools: {} | History entries: {}",
                        self.is_open, active_count, self.action_history.len()
                    ))
            )
    }
}
