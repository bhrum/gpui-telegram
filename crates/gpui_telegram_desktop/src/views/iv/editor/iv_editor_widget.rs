//! Target Symbol: IvEditorWidgetEditor
//! Module 13: Instant View Article Editor Canvas Component
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditorBlockKind {
    Title,
    Author,
    Heading { level: u8 },
    Paragraph,
    Blockquote,
    Code { lang: Option<String> },
    ListItem { is_ordered: bool, index: usize },
    Divider,
}

#[derive(Debug, Clone)]
pub struct EditorBlock {
    pub id: usize,
    pub kind: EditorBlockKind,
    pub text: String,
    pub is_selected: bool,
}

#[derive(Debug, Clone, Default)]
pub struct SelectionCapture {
    pub block_id: usize,
    pub cursor_offset: usize,
    pub selection_length: usize,
}

#[derive(Debug, Clone)]
pub struct IvEditorWidgetEditor {
    pub blocks: Vec<EditorBlock>,
    pub active_block_id: Option<usize>,
    pub selection: SelectionCapture,
    pub undo_stack: Vec<Vec<EditorBlock>>,
    pub redo_stack: Vec<Vec<EditorBlock>>,
    pub next_block_id: usize,
    pub is_dirty: bool,
}

impl Default for IvEditorWidgetEditor {
    fn default() -> Self {
        let initial_blocks = vec![
            EditorBlock {
                id: 1,
                kind: EditorBlockKind::Title,
                text: "Instant View Article Title".to_string(),
                is_selected: false,
            },
            EditorBlock {
                id: 2,
                kind: EditorBlockKind::Author,
                text: "By Editorial Team".to_string(),
                is_selected: false,
            },
            EditorBlock {
                id: 3,
                kind: EditorBlockKind::Paragraph,
                text: "Start typing article content here or paste markdown...".to_string(),
                is_selected: true,
            },
        ];

        Self {
            blocks: initial_blocks,
            active_block_id: Some(3),
            selection: SelectionCapture {
                block_id: 3,
                cursor_offset: 0,
                selection_length: 0,
            },
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            next_block_id: 4,
            is_dirty: false,
        }
    }
}

impl IvEditorWidgetEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_block(&mut self, kind: EditorBlockKind, text: impl Into<String>) -> usize {
        self.save_undo_state();
        let id = self.next_block_id;
        self.next_block_id += 1;
        self.blocks.push(EditorBlock {
            id,
            kind,
            text: text.into(),
            is_selected: false,
        });
        self.active_block_id = Some(id);
        self.is_dirty = true;
        id
    }

    pub fn delete_block(&mut self, id: usize) -> bool {
        self.save_undo_state();
        if let Some(pos) = self.blocks.iter().position(|b| b.id == id) {
            self.blocks.remove(pos);
            if self.active_block_id == Some(id) {
                self.active_block_id = self.blocks.last().map(|b| b.id);
            }
            self.is_dirty = true;
            true
        } else {
            false
        }
    }

    pub fn set_active_block(&mut self, id: usize) {
        self.active_block_id = Some(id);
        for b in &mut self.blocks {
            b.is_selected = b.id == id;
        }
    }

    pub fn apply_heading(&mut self, level: u8) {
        if let Some(id) = self.active_block_id {
            if self.blocks.iter().any(|b| b.id == id) {
                self.save_undo_state();
                if let Some(block) = self.blocks.iter_mut().find(|b| b.id == id) {
                    block.kind = EditorBlockKind::Heading { level };
                }
                self.is_dirty = true;
            }
        }
    }

    pub fn handle_shortcut(&mut self, key: &str, ctrl: bool, shift: bool) -> bool {
        if ctrl && !shift && key.eq_ignore_ascii_case("t") {
            self.apply_heading(1);
            return true;
        }
        if ctrl && shift && key.eq_ignore_ascii_case("h") {
            self.apply_heading(2);
            return true;
        }
        if ctrl && !shift && key.eq_ignore_ascii_case("z") {
            return self.undo();
        }
        if ctrl && shift && key.eq_ignore_ascii_case("z") {
            return self.redo();
        }
        false
    }

    fn save_undo_state(&mut self) {
        if self.undo_stack.len() >= 64 {
            self.undo_stack.remove(0);
        }
        self.undo_stack.push(self.blocks.clone());
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) -> bool {
        if let Some(prev) = self.undo_stack.pop() {
            self.redo_stack.push(self.blocks.clone());
            self.blocks = prev;
            self.is_dirty = true;
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(self.blocks.clone());
            self.blocks = next;
            self.is_dirty = true;
            true
        } else {
            false
        }
    }

    pub fn word_count(&self) -> usize {
        self.blocks
            .iter()
            .map(|b| b.text.split_whitespace().count())
            .sum()
    }
}

impl Render for IvEditorWidgetEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut container = div()
            .flex()
            .flex_col()
            .w_full()
            .max_w(px(720.0))
            .bg(rgb(0x181B20))
            .rounded(px(12.0))
            .p(px(24.0))
            .gap(px(16.0))
            .shadow_lg();

        for block in &self.blocks {
            let (font_size, font_weight, text_color) = match &block.kind {
                EditorBlockKind::Title => (px(28.0), FontWeight::BOLD, rgb(0xFFFFFF)),
                EditorBlockKind::Author => (px(14.0), FontWeight::NORMAL, rgb(0x8D95A3)),
                EditorBlockKind::Heading { level: 1 } => (px(22.0), FontWeight::BOLD, rgb(0xF0F3F8)),
                EditorBlockKind::Heading { level: 2 } => (px(18.0), FontWeight::BOLD, rgb(0xE0E4EC)),
                EditorBlockKind::Heading { .. } => (px(16.0), FontWeight::BOLD, rgb(0xD0D5E0)),
                EditorBlockKind::Blockquote => (px(15.0), FontWeight::NORMAL, rgb(0x6AB2F2)),
                EditorBlockKind::Code { .. } => (px(13.0), FontWeight::NORMAL, rgb(0x68D391)),
                EditorBlockKind::ListItem { .. } => (px(15.0), FontWeight::NORMAL, rgb(0xE4E7EB)),
                EditorBlockKind::Divider => (px(12.0), FontWeight::NORMAL, rgb(0x404854)),
                EditorBlockKind::Paragraph => (px(15.0), FontWeight::NORMAL, rgb(0xD8DCE4)),
            };

            let is_active = self.active_block_id == Some(block.id);
            let border_color = if is_active { rgb(0x2AABEE) } else { rgb(0x282E38) };

            container = container.child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .p(px(8.0))
                    .rounded(px(6.0))
                    .border_1()
                    .border_color(border_color)
                    .bg(if is_active { rgb(0x202630) } else { rgb(0x181B20) })
                    .child(
                        div()
                            .text_size(font_size)
                            .font_weight(font_weight)
                            .text_color(text_color)
                            .child(block.text.clone()),
                    ),
            );
        }

        // Status bar
        container.child(
            div()
                .flex()
                .flex_row()
                .justify_between()
                .pt(px(12.0))
                .border_t_1()
                .border_color(rgb(0x282E38))
                .text_size(px(12.0))
                .text_color(rgb(0x6E7684))
                .child(format!("Blocks: {}", self.blocks.len()))
                .child(format!("Words: {}", self.word_count())),
        )
    }
}
