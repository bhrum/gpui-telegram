//! History Inner Widget Accessibility Representation.
//!
//! Citations:
//! - Upstream: `Telegram/SourceFiles/history/history_inner_widget_accessibility.h:L16-L65`
//! - Upstream: `Telegram/SourceFiles/history/history_inner_widget_accessibility.cpp:L50-L180`
//! - MessageSubItem enumeration: 44 discrete accessible semantic parts (history_inner_widget_accessibility.h:L16-L65)

use gpui::*;

/// Accessible sub-elements within a rendered message item.
/// C++ citation: `HistoryView::MessageSubItem` (history_inner_widget_accessibility.h:L16-L65).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageSubItem {
    Seen,
    Sender,
    ViaBot,
    Reply,
    Forward,
    MediaType,
    Download,
    Played,
    Artist,
    Title,
    Filename,
    Duration,
    Dimensions,
    FileSize,
    Message,
    Delivery,
    Edited,
    Time,
    Reactions,
    Views,
    Signature,
    Pinned,
    WebSite,
    WebTitle,
    WebDescription,
    PollQuestion,
    PollOptions,
    PollStatus,
    ContactName,
    ContactPhone,
    Location,
    StickerEmoji,
    GameTitle,
    GameDescription,
    InvoiceTitle,
    InvoiceAmount,
    Spoiler,
    Dice,
    Giveaway,
    Gift,
    TodoTitle,
    TodoItems,
    Factcheck,
    ForwardDate,
}

/// Accessible node model for screen readers and keyboard navigation.
#[derive(Debug, Clone)]
pub struct AccessibleMessageNode {
    pub message_id: u64,
    pub sender_name: String,
    pub date_text: String,
    pub body_text: String,
    pub is_outgoing: bool,
    pub delivery_status: String,
    pub subitems: Vec<(MessageSubItem, String)>,
}

impl AccessibleMessageNode {
    pub fn new(
        message_id: u64,
        sender_name: impl Into<String>,
        date_text: impl Into<String>,
        body_text: impl Into<String>,
        is_outgoing: bool,
    ) -> Self {
        Self {
            message_id,
            sender_name: sender_name.into(),
            date_text: date_text.into(),
            body_text: body_text.into(),
            is_outgoing,
            delivery_status: if is_outgoing { "Sent".to_string() } else { "Received".to_string() },
            subitems: Vec::new(),
        }
    }

    pub fn add_subitem(&mut self, item: MessageSubItem, label: impl Into<String>) {
        self.subitems.push((item, label.into()));
    }

    /// Formulates full spoken announcement string for assistive technology.
    /// C++ citation: `history_inner_widget_accessibility.cpp:L80-L150`.
    pub fn build_accessible_announcement(&self) -> String {
        let mut parts = Vec::new();
        if self.is_outgoing {
            parts.push("Outgoing message".to_string());
        } else {
            parts.push(format!("Message from {}", self.sender_name));
        }
        parts.push(self.date_text.clone());
        parts.push(self.delivery_status.clone());
        if !self.body_text.is_empty() {
            parts.push(self.body_text.clone());
        }
        for (kind, label) in &self.subitems {
            parts.push(format!("{:?}: {}", kind, label));
        }
        parts.join(", ")
    }
}

/// Accessibility tree controller for `HistoryInnerWidget`.
/// C++ citation: `HistoryInnerWidgetAccessibility` (history_inner_widget_accessibility.h:L10-L80).
pub struct HistoryInnerWidgetAccessibility {
    nodes: Vec<AccessibleMessageNode>,
    focused_message_index: Option<usize>,
    focused_subitem_index: Option<usize>,
}

impl HistoryInnerWidgetAccessibility {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            focused_message_index: None,
            focused_subitem_index: None,
        }
    }

    pub fn set_nodes(&mut self, nodes: Vec<AccessibleMessageNode>) {
        self.nodes = nodes;
        if let Some(idx) = self.focused_message_index {
            if idx >= self.nodes.len() {
                self.focused_message_index = if self.nodes.is_empty() { None } else { Some(self.nodes.len() - 1) };
            }
        }
    }

    pub fn focus_next_message(&mut self) -> Option<&AccessibleMessageNode> {
        if self.nodes.is_empty() {
            return None;
        }
        let next = match self.focused_message_index {
            Some(i) => (i + 1).min(self.nodes.len() - 1),
            None => 0,
        };
        self.focused_message_index = Some(next);
        self.focused_subitem_index = None;
        self.current_focused_node()
    }

    pub fn focus_prev_message(&mut self) -> Option<&AccessibleMessageNode> {
        if self.nodes.is_empty() {
            return None;
        }
        let prev = match self.focused_message_index {
            Some(i) => i.saturating_sub(1),
            None => 0,
        };
        self.focused_message_index = Some(prev);
        self.focused_subitem_index = None;
        self.current_focused_node()
    }

    pub fn current_focused_node(&self) -> Option<&AccessibleMessageNode> {
        self.focused_message_index.and_then(|i| self.nodes.get(i))
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

impl Default for HistoryInnerWidgetAccessibility {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryInnerWidgetAccessibility {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().into_any_element()
    }
}
