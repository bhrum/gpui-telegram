//! Target Symbol: IvEditorTextEntitiesEditor
//! Module 13: Instant View Rich Text Entity Translation Subsystem
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const MAX_RICH_TEXT_NODE_LENGTH: usize = 16000;
pub const MAX_COMMITTED_FIELD_LENGTH: usize = 256 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TextRange {
    pub offset: usize,
    pub length: usize,
}

impl TextRange {
    pub fn new(offset: usize, length: usize) -> Self {
        Self { offset, length }
    }

    pub fn end(&self) -> usize {
        self.offset.saturating_add(self.length)
    }

    pub fn contains(&self, index: usize) -> bool {
        index >= self.offset && index < self.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityKind {
    Bold,
    Italic,
    Underline,
    Strike,
    Code,
    Pre(Option<String>),
    Spoiler,
    CustomUrl(String),
    CustomEmoji(u64),
    Formula { is_block: bool },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorEntity {
    pub kind: EntityKind,
    pub range: TextRange,
}

#[derive(Debug, Clone, Default)]
pub struct RichTextEditorOffsetReplacement {
    pub rich_offset: usize,
    pub rich_length: usize,
    pub editor_length: usize,
}

#[derive(Debug, Clone, Default)]
pub struct FormulaReplacement {
    pub offset: usize,
    pub length: usize,
    pub source: String,
    pub is_block: bool,
}

#[derive(Debug, Clone, Default)]
pub struct RichTextEditorConversion {
    pub text: String,
    pub entities: Vec<EditorEntity>,
    pub replacements: Vec<RichTextEditorOffsetReplacement>,
}

#[derive(Debug, Clone)]
pub struct IvEditorTextEntitiesEditor {
    pub conversion: RichTextEditorConversion,
    pub formula_replacements: Vec<FormulaReplacement>,
    pub preview_text: String,
}

impl Default for IvEditorTextEntitiesEditor {
    fn default() -> Self {
        Self {
            conversion: RichTextEditorConversion::default(),
            formula_replacements: Vec::new(),
            preview_text: String::new(),
        }
    }
}

impl IvEditorTextEntitiesEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn range_inside_text(text: &str, offset: usize, length: usize) -> bool {
        let total = text.chars().count();
        offset.saturating_add(length) <= total
    }

    pub fn map_rich_offset_to_editor_offset(
        replacements: &[RichTextEditorOffsetReplacement],
        offset: usize,
    ) -> usize {
        let mut delta: isize = 0;
        for rep in replacements {
            if rep.rich_offset + rep.rich_length <= offset {
                delta += (rep.editor_length as isize) - (rep.rich_length as isize);
            } else if rep.rich_offset < offset {
                delta += (rep.editor_length as isize) - ((offset - rep.rich_offset) as isize);
                break;
            }
        }
        (offset as isize + delta).max(0) as usize
    }

    pub fn parse_formulas(input: &str) -> (String, Vec<FormulaReplacement>) {
        let mut result = String::with_capacity(input.len());
        let mut formulas = Vec::new();
        let mut chars = input.chars().peekable();
        let mut current_offset = 0;

        while let Some(ch) = chars.next() {
            if ch == '$' {
                let is_block = chars.peek() == Some(&'$');
                if is_block {
                    chars.next();
                }

                let mut formula_str = String::new();
                let mut closed = false;
                while let Some(c) = chars.next() {
                    if c == '$' {
                        if is_block {
                            if chars.peek() == Some(&'$') {
                                chars.next();
                                closed = true;
                                break;
                            } else {
                                formula_str.push(c);
                            }
                        } else {
                            closed = true;
                            break;
                        }
                    } else {
                        formula_str.push(c);
                    }
                }

                if closed {
                    let placeholder = "[formula]";
                    let form_offset = current_offset;
                    let form_len = placeholder.len();
                    result.push_str(placeholder);
                    current_offset += form_len;
                    formulas.push(FormulaReplacement {
                        offset: form_offset,
                        length: form_len,
                        source: formula_str,
                        is_block,
                    });
                } else {
                    result.push('$');
                    current_offset += 1;
                    if is_block {
                        result.push('$');
                        current_offset += 1;
                    }
                    result.push_str(&formula_str);
                    current_offset += formula_str.len();
                }
            } else {
                result.push(ch);
                current_offset += ch.len_utf8();
            }
        }

        (result, formulas)
    }

    pub fn set_text_with_entities(&mut self, text: String, entities: Vec<EditorEntity>) {
        let (processed_text, formulas) = Self::parse_formulas(&text);
        self.preview_text = text.clone();
        self.formula_replacements = formulas;
        self.conversion = RichTextEditorConversion {
            text: processed_text,
            entities,
            replacements: Vec::new(),
        };
    }
}

impl Render for IvEditorTextEntitiesEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let entity_count = self.conversion.entities.len();
        let formula_count = self.formula_replacements.len();

        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(10.0))
            .bg(rgb(0x1E232A))
            .rounded(px(6.0))
            .gap(px(4.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(8.0))
                    .text_size(px(12.0))
                    .text_color(rgb(0xA0A5AD))
                    .child(format!("Entities: {}", entity_count))
                    .child(format!("Formulas: {}", formula_count)),
            )
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(rgb(0xD1D5DB))
                    .child(if self.preview_text.is_empty() {
                        "No text loaded".to_string()
                    } else {
                        self.preview_text.clone()
                    }),
            )
    }
}
