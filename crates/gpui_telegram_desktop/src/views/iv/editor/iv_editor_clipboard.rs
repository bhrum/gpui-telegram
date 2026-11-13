//! Instant View Editor Clipboard MIME Protocol (`iv_editor_clipboard.h/.cpp`)
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const CLIPBOARD_MIME_TYPE: &str = "application/x-tdesktop-iv-editor";

/// Provenance metadata tracking which session and serial revision originated the clipboard data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ClipboardOrigin {
    pub session_id: u64,
    pub serial: u64,
}

/// Simplified serializable representation of an IV article page block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RichPageBlockCopy {
    pub kind: String,
    pub text: String,
    pub language: Option<String>,
    pub heading_level: Option<i32>,
}

/// Serializable representation of an IV list item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RichPageListItemCopy {
    pub text: String,
    pub checked: Option<bool>,
    pub number: Option<i32>,
}

/// Clipboard payload when copying a sequence of article blocks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClipboardBlockData {
    pub origin: ClipboardOrigin,
    pub blocks: Vec<RichPageBlockCopy>,
}

/// Clipboard payload when copying nested list items.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClipboardListItemsData {
    pub origin: ClipboardOrigin,
    pub list_kind: String,
    pub is_task_list: bool,
    pub items: Vec<RichPageListItemCopy>,
}

/// Union of clipboard formats supported by the Instant View article editor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClipboardData {
    Blocks(ClipboardBlockData),
    ListItems(ClipboardListItemsData),
}

impl ClipboardData {
    pub fn origin(&self) -> ClipboardOrigin {
        match self {
            Self::Blocks(b) => b.origin,
            Self::ListItems(l) => l.origin,
        }
    }

    pub fn block_count(&self) -> usize {
        match self {
            Self::Blocks(b) => b.blocks.len(),
            Self::ListItems(l) => l.items.len(),
        }
    }
}

/// Serializes ClipboardData to a JSON payload suitable for clipboard storage.
pub fn serialize_clipboard_data(data: &ClipboardData) -> Vec<u8> {
    match data {
        ClipboardData::Blocks(b) => {
            let mut buf = format!("BLOCKS:{}:{}:", b.origin.session_id, b.origin.serial);
            for block in &b.blocks {
                let lang = block.language.as_deref().unwrap_or("");
                let hlevel = block.heading_level.map(|l| l.to_string()).unwrap_or_default();
                buf.push_str(&format!("{}^{}^{}^{};", block.kind, block.text.replace(';', "\\;"), lang, hlevel));
            }
            buf.into_bytes()
        }
        ClipboardData::ListItems(l) => {
            let mut buf = format!("LIST:{}:{}:{}:{}:", l.origin.session_id, l.origin.serial, l.list_kind, l.is_task_list);
            for item in &l.items {
                buf.push_str(&format!("{}^{};", item.text.replace(';', "\\;"), item.checked.unwrap_or(false)));
            }
            buf.into_bytes()
        }
    }
}

/// Deserializes ClipboardData from bytes.
pub fn deserialize_clipboard_data(bytes: &[u8]) -> Option<ClipboardData> {
    let s = std::str::from_utf8(bytes).ok()?;
    if let Some(rest) = s.strip_prefix("BLOCKS:") {
        let mut parts = rest.splitn(3, ':');
        let session_id = parts.next()?.parse().ok()?;
        let serial = parts.next()?.parse().ok()?;
        let items_str = parts.next()?;
        let mut blocks = Vec::new();
        for item in items_str.split(';') {
            if item.is_empty() {
                continue;
            }
            let mut kv = item.splitn(4, '^');
            let kind = kv.next()?.to_string();
            let text = kv.next().unwrap_or("").to_string();
            let lang_str = kv.next().unwrap_or("");
            let language = if lang_str.is_empty() { None } else { Some(lang_str.to_string()) };
            let hlevel_str = kv.next().unwrap_or("");
            let heading_level = hlevel_str.parse().ok();
            blocks.push(RichPageBlockCopy {
                kind,
                text,
                language,
                heading_level,
            });
        }
        Some(ClipboardData::Blocks(ClipboardBlockData {
            origin: ClipboardOrigin { session_id, serial },
            blocks,
        }))
    } else if let Some(rest) = s.strip_prefix("LIST:") {
        let mut parts = rest.splitn(5, ':');
        let session_id = parts.next()?.parse().ok()?;
        let serial = parts.next()?.parse().ok()?;
        let list_kind = parts.next()?.to_string();
        let is_task = parts.next()?.parse().unwrap_or(false);
        let items_str = parts.next()?;
        let mut items = Vec::new();
        for item in items_str.split(';') {
            if item.is_empty() {
                continue;
            }
            let mut kv = item.splitn(2, '^');
            let text = kv.next()?.to_string();
            let checked = kv.next().and_then(|c| c.parse().ok());
            items.push(RichPageListItemCopy {
                text,
                checked,
                number: None,
            });
        }
        Some(ClipboardData::ListItems(ClipboardListItemsData {
            origin: ClipboardOrigin { session_id, serial },
            list_kind,
            is_task_list: is_task,
            items,
        }))
    } else {
        None
    }
}

/// Instant View editor clipboard controller.
#[derive(Debug, Clone)]
pub struct IvEditorClipboardEditor {
    pub origin: ClipboardOrigin,
    pub clipboard_data: Option<ClipboardData>,
    pub copy_count: usize,
    pub paste_count: usize,
    pub is_active: bool,
}

impl Default for IvEditorClipboardEditor {
    fn default() -> Self {
        Self {
            origin: ClipboardOrigin { session_id: 1, serial: 1 },
            clipboard_data: None,
            copy_count: 0,
            paste_count: 0,
            is_active: false,
        }
    }
}

impl IvEditorClipboardEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn copy_blocks(&mut self, blocks: Vec<RichPageBlockCopy>) {
        self.origin.serial += 1;
        self.clipboard_data = Some(ClipboardData::Blocks(ClipboardBlockData {
            origin: self.origin,
            blocks,
        }));
        self.copy_count += 1;
    }

    pub fn copy_list_items(&mut self, list_kind: impl Into<String>, items: Vec<RichPageListItemCopy>, is_task: bool) {
        self.origin.serial += 1;
        self.clipboard_data = Some(ClipboardData::ListItems(ClipboardListItemsData {
            origin: self.origin,
            list_kind: list_kind.into(),
            is_task_list: is_task,
            items,
        }));
        self.copy_count += 1;
    }

    pub fn paste(&mut self) -> Option<ClipboardData> {
        if self.clipboard_data.is_some() {
            self.paste_count += 1;
        }
        self.clipboard_data.clone()
    }
}

impl Render for IvEditorClipboardEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let (desc, count) = match &self.clipboard_data {
            Some(ClipboardData::Blocks(b)) => ("Blocks", b.blocks.len()),
            Some(ClipboardData::ListItems(l)) => ("List Items", l.items.len()),
            None => ("Empty", 0),
        };
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
                    .child("Instant View Clipboard Manager")
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x666666))
                    .child(format!(
                        "MIME: {} | Type: {} ({} items) | Copies: {} | Pastes: {}",
                        CLIPBOARD_MIME_TYPE, desc, count, self.copy_count, self.paste_count
                    ))
            )
    }
}
