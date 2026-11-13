//! Instant View Article Page Blocks & Factory Engine (`iv_editor_page_blocks.h/.cpp`)
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

/// Complete enumeration of Instant View block types available in the editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum InsertBlockType {
    #[default]
    Paragraph,
    Heading,
    Blockquote,
    Pullquote,
    Code,
    Math,
    Footer,
    Divider,
    Anchor,
    OrderedList,
    BulletList,
    TaskList,
    Photo,
    Video,
    Audio,
    Details,
    Table,
    Map,
}

/// Action parameters for inserting a new block into an article document.
#[derive(Debug, Clone, Default)]
pub struct InsertAction {
    pub block_type: InsertBlockType,
    pub heading_level: i32,
    pub ordered_start: i32,
    pub code_language: String,
    pub latitude: f64,
    pub longitude: f64,
}

/// Rich article page block model maintaining hierarchy and formatting.
#[derive(Debug, Clone)]
pub struct PageBlock {
    pub kind: InsertBlockType,
    pub text: String,
    pub level: i32,
    pub caption: Option<String>,
    pub language: Option<String>,
    pub children: Vec<PageBlock>,
}

impl PageBlock {
    pub fn new(kind: InsertBlockType) -> Self {
        Self {
            kind,
            text: String::new(),
            level: 1,
            caption: None,
            language: None,
            children: Vec::new(),
        }
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }
}

pub fn make_paragraph_block() -> PageBlock {
    PageBlock::new(InsertBlockType::Paragraph)
}

pub fn make_heading_block(level: i32) -> PageBlock {
    let mut b = PageBlock::new(InsertBlockType::Heading);
    b.level = level.clamp(1, 6);
    b
}

pub fn make_quote_block(pullquote: bool) -> PageBlock {
    PageBlock::new(if pullquote {
        InsertBlockType::Pullquote
    } else {
        InsertBlockType::Blockquote
    })
}

pub fn make_code_block(language: Option<String>) -> PageBlock {
    let mut b = PageBlock::new(InsertBlockType::Code);
    b.language = language;
    b
}

pub fn make_math_block(formula: impl Into<String>) -> PageBlock {
    let mut b = PageBlock::new(InsertBlockType::Math);
    b.text = formula.into();
    b
}

pub fn make_divider_block() -> PageBlock {
    PageBlock::new(InsertBlockType::Divider)
}

pub fn make_anchor_block(anchor_id: impl Into<String>) -> PageBlock {
    let mut b = PageBlock::new(InsertBlockType::Anchor);
    b.text = anchor_id.into();
    b
}

pub fn make_list_block(kind: InsertBlockType) -> PageBlock {
    PageBlock::new(kind)
}

pub fn make_details_block() -> PageBlock {
    PageBlock::new(InsertBlockType::Details)
}

pub fn make_table_block() -> PageBlock {
    PageBlock::new(InsertBlockType::Table)
}

pub fn make_map_block(latitude: f64, longitude: f64) -> PageBlock {
    let mut b = PageBlock::new(InsertBlockType::Map);
    b.caption = Some(format!("geo:{:.6},{:.6}", latitude, longitude));
    b
}

pub fn block_can_own_child_container(kind: InsertBlockType) -> bool {
    matches!(
        kind,
        InsertBlockType::Blockquote
            | InsertBlockType::Pullquote
            | InsertBlockType::Details
            | InsertBlockType::OrderedList
            | InsertBlockType::BulletList
            | InsertBlockType::TaskList
    )
}

pub fn block_supports_text(kind: InsertBlockType) -> bool {
    !matches!(kind, InsertBlockType::Divider | InsertBlockType::Photo | InsertBlockType::Video)
}

pub fn append_paragraph_seam(destination: &mut PageBlock, source: PageBlock) -> usize {
    let seam_pos = destination.text.len();
    if !destination.text.is_empty() && !source.text.is_empty() {
        destination.text.push(' ');
    }
    destination.text.push_str(&source.text);
    destination.children.extend(source.children);
    seam_pos
}

/// Instant View article page blocks manager and tree builder.
#[derive(Debug, Clone)]
pub struct IvEditorPageBlocksEditor {
    pub blocks: Vec<PageBlock>,
    pub selected_index: Option<usize>,
    pub is_active: bool,
}

impl Default for IvEditorPageBlocksEditor {
    fn default() -> Self {
        Self {
            blocks: vec![make_paragraph_block()],
            selected_index: Some(0),
            is_active: false,
        }
    }
}

impl IvEditorPageBlocksEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_block(&mut self, action: InsertAction) -> usize {
        let block = match action.block_type {
            InsertBlockType::Paragraph => make_paragraph_block(),
            InsertBlockType::Heading => make_heading_block(action.heading_level),
            InsertBlockType::Blockquote => make_quote_block(false),
            InsertBlockType::Pullquote => make_quote_block(true),
            InsertBlockType::Code => make_code_block(Some(action.code_language)),
            InsertBlockType::Math => make_math_block(""),
            InsertBlockType::Divider => make_divider_block(),
            InsertBlockType::Anchor => make_anchor_block(""),
            InsertBlockType::OrderedList | InsertBlockType::BulletList | InsertBlockType::TaskList => {
                make_list_block(action.block_type)
            }
            InsertBlockType::Details => make_details_block(),
            InsertBlockType::Table => make_table_block(),
            InsertBlockType::Map => make_map_block(action.latitude, action.longitude),
            other => PageBlock::new(other),
        };

        let pos = self.selected_index.map(|idx| idx + 1).unwrap_or(self.blocks.len());
        self.blocks.insert(pos, block);
        self.selected_index = Some(pos);
        pos
    }

    pub fn remove_selected(&mut self) -> bool {
        if let Some(idx) = self.selected_index {
            if idx < self.blocks.len() {
                self.blocks.remove(idx);
                if self.blocks.is_empty() {
                    self.blocks.push(make_paragraph_block());
                    self.selected_index = Some(0);
                } else if idx >= self.blocks.len() {
                    self.selected_index = Some(self.blocks.len() - 1);
                }
                return true;
            }
        }
        false
    }
}

impl Render for IvEditorPageBlocksEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(12.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .child("Instant View Page Blocks Hierarchy")
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x666666))
                    .child(format!(
                        "Total blocks: {} | Selected: {:?}",
                        self.blocks.len(), self.selected_index
                    ))
            )
    }
}
