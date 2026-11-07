//! Instant View Editor State Machine & Mutation Engine (`iv_editor_state.h/.cpp`)
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

/// Operation mode of the article editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FieldMode {
    #[default]
    Compose,
    Edit,
}

/// Category of structural or formatting document mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreparedMutationKind {
    InsertBlock,
    RemoveBlock,
    SplitBlock,
    JoinBlocks,
    FormatText,
    ChangeBlockType,
}

/// Text formatting style applied to selected ranges.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextFormattingAction {
    Bold,
    Italic,
    Underline,
    StrikeOut,
    Code,
    Spoiler,
}

/// Undo/redo snapshot capturing the state of an article draft.
#[derive(Debug, Clone)]
pub struct Snapshot {
    pub blocks: Vec<String>,
    pub active_block_index: usize,
    pub cursor_position: usize,
    pub timestamp: i64,
}

/// Core Instant View editor state machine maintaining blocks and transactional history.
#[derive(Debug, Clone)]
pub struct IvEditorStateEditor {
    pub mode: FieldMode,
    pub blocks: Vec<String>,
    pub active_block_index: usize,
    pub cursor_position: usize,
    pub selection: Option<(usize, usize)>,
    pub undo_stack: Vec<Snapshot>,
    pub redo_stack: Vec<Snapshot>,
    pub is_dirty: bool,
    pub is_active: bool,
}

impl Default for IvEditorStateEditor {
    fn default() -> Self {
        Self {
            mode: FieldMode::Compose,
            blocks: vec!["Welcome to Instant View Editor.".to_string()],
            active_block_index: 0,
            cursor_position: 0,
            selection: None,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            is_dirty: false,
            is_active: false,
        }
    }
}

impl IvEditorStateEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_mode(mut self, mode: FieldMode) -> Self {
        self.mode = mode;
        self
    }

    fn record_snapshot(&mut self) {
        let snapshot = Snapshot {
            blocks: self.blocks.clone(),
            active_block_index: self.active_block_index,
            cursor_position: self.cursor_position,
            timestamp: chrono::Utc::now().timestamp(),
        };
        self.undo_stack.push(snapshot);
        self.redo_stack.clear();
        self.is_dirty = true;
    }

    pub fn insert_text(&mut self, text: &str) -> bool {
        self.record_snapshot();
        if let Some(block) = self.blocks.get_mut(self.active_block_index) {
            let pos = self.cursor_position.min(block.len());
            block.insert_str(pos, text);
            self.cursor_position = pos + text.len();
            true
        } else {
            false
        }
    }

    pub fn delete_backwards(&mut self) -> bool {
        let can_delete = if let Some(block) = self.blocks.get(self.active_block_index) {
            self.cursor_position > 0 && self.cursor_position <= block.len()
        } else {
            false
        };
        if can_delete {
            self.record_snapshot();
            if let Some(block) = self.blocks.get_mut(self.active_block_index) {
                block.remove(self.cursor_position - 1);
                self.cursor_position -= 1;
                return true;
            }
        }
        false
    }

    pub fn split_active_block(&mut self) -> bool {
        if let Some(block) = self.blocks.get(self.active_block_index).cloned() {
            self.record_snapshot();
            let pos = self.cursor_position.min(block.len());
            let first_half = block[..pos].to_string();
            let second_half = block[pos..].to_string();

            self.blocks[self.active_block_index] = first_half;
            self.blocks.insert(self.active_block_index + 1, second_half);
            self.active_block_index += 1;
            self.cursor_position = 0;
            true
        } else {
            false
        }
    }

    pub fn apply_formatting(&mut self, action: TextFormattingAction) -> bool {
        if let Some((start, end)) = self.selection {
            self.record_snapshot();
            if let Some(block) = self.blocks.get_mut(self.active_block_index) {
                let tag = match action {
                    TextFormattingAction::Bold => "**",
                    TextFormattingAction::Italic => "*",
                    TextFormattingAction::Underline => "__",
                    TextFormattingAction::StrikeOut => "~~",
                    TextFormattingAction::Code => "`",
                    TextFormattingAction::Spoiler => "||",
                };
                let s = start.min(end).min(block.len());
                let e = start.max(end).min(block.len());
                let inner = &block[s..e];
                let formatted = format!("{}{}{}", tag, inner, tag);
                block.replace_range(s..e, &formatted);
                self.selection = None;
                return true;
            }
        }
        false
    }

    pub fn undo(&mut self) -> bool {
        if let Some(prev) = self.undo_stack.pop() {
            let current = Snapshot {
                blocks: self.blocks.clone(),
                active_block_index: self.active_block_index,
                cursor_position: self.cursor_position,
                timestamp: chrono::Utc::now().timestamp(),
            };
            self.redo_stack.push(current);

            self.blocks = prev.blocks;
            self.active_block_index = prev.active_block_index;
            self.cursor_position = prev.cursor_position;
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        if let Some(next) = self.redo_stack.pop() {
            let current = Snapshot {
                blocks: self.blocks.clone(),
                active_block_index: self.active_block_index,
                cursor_position: self.cursor_position,
                timestamp: chrono::Utc::now().timestamp(),
            };
            self.undo_stack.push(current);

            self.blocks = next.blocks;
            self.active_block_index = next.active_block_index;
            self.cursor_position = next.cursor_position;
            true
        } else {
            false
        }
    }
}

impl Render for IvEditorStateEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
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
                    .font_weight(FontWeight::BOLD)
                    .child(format!("Instant View State Engine ({:?})", self.mode))
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x666666))
                    .child(format!(
                        "Blocks: {} | Active block: #{} | Cursor: {} | Undo steps: {} | Redo steps: {}",
                        self.blocks.len(), self.active_block_index + 1, self.cursor_position, self.undo_stack.len(), self.redo_stack.len()
                    ))
            )
            .child(
                div()
                    .mt(px(8.0))
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .children(self.blocks.iter().enumerate().map(|(idx, b)| {
                        let is_active = idx == self.active_block_index;
                        div()
                            .p(px(6.0))
                            .rounded(px(4.0))
                            .bg(if is_active { rgb(0xF0F7FF) } else { rgb(0xFFFFFF) })
                            .border_1()
                            .border_color(if is_active { rgb(0x2AABEE) } else { rgb(0xEEEEEE) })
                            .text_size(px(13.0))
                            .child(b.clone())
                    }))
            )
    }
}
