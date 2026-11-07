//! Instant View Clipboard Importer for Markdown & HTML (`iv_editor_clipboard_import.h/.cpp`)
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const IMPORTED_MEDIA_PLACEHOLDER_LIMIT: usize = 4096;
const MEDIA_PLACEHOLDER_BASE: u64 = 0xF000_0000_0000_0000;

/// Generates a reserved synthetic ID for media imported from clipboard.
pub const fn imported_media_placeholder_id(index: usize) -> u64 {
    MEDIA_PLACEHOLDER_BASE + (index as u64)
}

/// Extracts placeholder index if the ID falls into the reserved clipboard media range.
pub const fn imported_media_placeholder_index(id: u64) -> Option<usize> {
    if id >= MEDIA_PLACEHOLDER_BASE && id < MEDIA_PLACEHOLDER_BASE + (IMPORTED_MEDIA_PLACEHOLDER_LIMIT as u64) {
        Some((id - MEDIA_PLACEHOLDER_BASE) as usize)
    } else {
        None
    }
}

/// Limits enforced when importing HTML or TSV tables into article blocks.
#[derive(Debug, Clone)]
pub struct TableImportLimits {
    pub max_rows: usize,
    pub max_columns: usize,
    pub max_cells: usize,
    pub max_cell_length: usize,
}

impl Default for TableImportLimits {
    fn default() -> Self {
        Self {
            max_rows: 100,
            max_columns: 20,
            max_cells: 500,
            max_cell_length: 1024,
        }
    }
}

/// Attached local media resource from clipboard.
#[derive(Debug, Clone, Default)]
pub struct ImportedLocalMedia {
    pub path: String,
    pub content: Vec<u8>,
}

/// Generic representation of an imported article block.
#[derive(Debug, Clone)]
pub struct ImportedBlock {
    pub kind: String,
    pub content: String,
    pub level: Option<usize>,
}

/// Resulting blocks and media extracted during clipboard import.
#[derive(Debug, Clone, Default)]
pub struct BlocksImportResult {
    pub blocks: Vec<ImportedBlock>,
    pub local_media: Vec<ImportedLocalMedia>,
    pub truncated: bool,
}

/// Parses plain markdown text into distinct IV article page blocks.
pub fn blocks_from_markdown(text: &str, max_blocks: usize) -> BlocksImportResult {
    let mut blocks = Vec::new();
    let mut lines = text.lines().peekable();
    let mut truncated = false;

    while let Some(line) = lines.next() {
        if blocks.len() >= max_blocks {
            truncated = true;
            break;
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if let Some(heading) = trimmed.strip_prefix("### ") {
            blocks.push(ImportedBlock { kind: "heading".to_string(), content: heading.to_string(), level: Some(3) });
        } else if let Some(heading) = trimmed.strip_prefix("## ") {
            blocks.push(ImportedBlock { kind: "heading".to_string(), content: heading.to_string(), level: Some(2) });
        } else if let Some(heading) = trimmed.strip_prefix("# ") {
            blocks.push(ImportedBlock { kind: "heading".to_string(), content: heading.to_string(), level: Some(1) });
        } else if let Some(quote) = trimmed.strip_prefix("> ") {
            blocks.push(ImportedBlock { kind: "blockquote".to_string(), content: quote.to_string(), level: None });
        } else if let Some(item) = trimmed.strip_prefix("- ").or_else(|| trimmed.strip_prefix("* ")) {
            blocks.push(ImportedBlock { kind: "bullet_list_item".to_string(), content: item.to_string(), level: None });
        } else if trimmed.starts_with("```") {
            let mut code_lines = Vec::new();
            while let Some(code_line) = lines.next() {
                if code_line.trim().starts_with("```") {
                    break;
                }
                code_lines.push(code_line);
            }
            blocks.push(ImportedBlock {
                kind: "code".to_string(),
                content: code_lines.join("\n"),
                level: None,
            });
        } else {
            blocks.push(ImportedBlock {
                kind: "paragraph".to_string(),
                content: trimmed.to_string(),
                level: None,
            });
        }
    }

    BlocksImportResult {
        blocks,
        local_media: Vec::new(),
        truncated,
    }
}

/// Primary clipboard import controller for the Instant View editor.
#[derive(Debug, Clone)]
pub struct IvEditorClipboardImportEditor {
    pub limits: TableImportLimits,
    pub last_result: Option<BlocksImportResult>,
    pub total_imports: usize,
    pub is_active: bool,
}

impl Default for IvEditorClipboardImportEditor {
    fn default() -> Self {
        Self {
            limits: TableImportLimits::default(),
            last_result: None,
            total_imports: 0,
            is_active: false,
        }
    }
}

impl IvEditorClipboardImportEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn import_markdown(&mut self, text: &str) -> &BlocksImportResult {
        let res = blocks_from_markdown(text, self.limits.max_cells);
        self.last_result = Some(res);
        self.total_imports += 1;
        self.last_result.as_ref().unwrap()
    }

    pub fn import_table_tsv(&mut self, tsv: &str) -> (Vec<Vec<String>>, bool) {
        let mut rows = Vec::new();
        let mut truncated = false;

        for line in tsv.lines() {
            if rows.len() >= self.limits.max_rows {
                truncated = true;
                break;
            }
            let mut cells: Vec<String> = line.split('\t').map(|s| s.to_string()).collect();
            if cells.len() > self.limits.max_columns {
                cells.truncate(self.limits.max_columns);
                truncated = true;
            }
            rows.push(cells);
        }
        self.total_imports += 1;
        (rows, truncated)
    }
}

impl Render for IvEditorClipboardImportEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let block_count = self.last_result.as_ref().map(|r| r.blocks.len()).unwrap_or(0);
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
                    .child("Instant View Clipboard Importer")
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x666666))
                    .child(format!(
                        "Imports performed: {} | Last imported blocks: {} | Max cells limit: {}",
                        self.total_imports, block_count, self.limits.max_cells
                    ))
            )
    }
}
