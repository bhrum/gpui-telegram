//! Target Symbol: IvMarkdownDocumentMarkdown
//! Module 13: Instant View Document AST Model & Structural Node Taxonomy
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const FORMULA_EXACT_METRIC_SCALE: f32 = 1000.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ListType {
    #[default]
    Bullet,
    Numbered,
    Roman,
    Letter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TaskState {
    #[default]
    None,
    Unchecked,
    Checked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TableAlignment {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DocumentNodeKind {
    Heading(u8),
    Paragraph,
    Code(Option<String>),
    Blockquote,
    List(ListType),
    Table,
    Divider,
    Embed(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct DocumentNode {
    pub id: u64,
    pub kind: DocumentNodeKind,
    pub text_content: String,
    pub child_ids: Vec<u64>,
    pub task_state: TaskState,
}

#[derive(Debug, Clone)]
pub struct IvMarkdownDocumentMarkdown {
    pub nodes: Vec<DocumentNode>,
    pub root_ids: Vec<u64>,
    pub next_node_id: u64,
}

impl Default for IvMarkdownDocumentMarkdown {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            root_ids: Vec::new(),
            next_node_id: 1,
        }
    }
}

impl IvMarkdownDocumentMarkdown {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, kind: DocumentNodeKind, text: impl Into<String>) -> u64 {
        let id = self.next_node_id;
        self.next_node_id += 1;
        self.nodes.push(DocumentNode {
            id,
            kind,
            text_content: text.into(),
            child_ids: Vec::new(),
            task_state: TaskState::None,
        });
        self.root_ids.push(id);
        id
    }

    pub fn add_child(&mut self, parent_id: u64, child_id: u64) {
        if let Some(parent) = self.nodes.iter_mut().find(|n| n.id == parent_id) {
            parent.child_ids.push(child_id);
            self.root_ids.retain(|&id| id != child_id);
        }
    }

    pub fn find_node(&self, id: u64) -> Option<&DocumentNode> {
        self.nodes.iter().find(|n| n.id == id)
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

impl Render for IvMarkdownDocumentMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.node_count();
        div()
            .flex()
            .p_1()
            .text_xs()
            .text_color(rgb(0x6c7883))
            .child(format!("Document AST: {} nodes (roots: {})", count, self.root_ids.len()))
    }
}
