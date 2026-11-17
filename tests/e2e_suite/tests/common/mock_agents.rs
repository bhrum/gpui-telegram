#![allow(dead_code, unused_imports)]

use serde_json::json;
use std::collections::HashSet;

// ============================================================================
// Production Type Re-Exports (Migrated from M4 Mock Harnesses)
// ============================================================================

pub use gpui_telegram_agent::multi_agent::{
    FloorControlQueue, FloorPriority, LoopBudget, LoopBudgetExceededError, ScopedCapabilityToken,
    SharedGroupWorkspace, DEFAULT_MAX_LOOP_HOPS,
};
pub use gpui_telegram_proto::cards::{
    GenerativeCard, GenerativeCardLayout, GenerativeComponent,
};
pub use gpui_telegram_proto::miniapp_contracts::{AgentManifest, MiniAppActionSpec};
pub use gpui_telegram_proto::semantic_tree::{
    ActionExecutionStatus, SemanticAction, SemanticActionReceipt, SemanticComposerNode,
    SemanticInspectorNode, SemanticModalNode, SemanticNavRailNode, SemanticSidebarNode,
    SemanticStageNode, SemanticWindowNode, SemanticWorkspaceTree,
};
pub use gpui_telegram_server::search::hybrid_memory::{
    EntityRelationEdge, HybridMemoryEngine, HybridSearchHit, MemoryDocument,
};

// ============================================================================
// Headless SemanticWorkspaceTree Test Driver backed by Production Proto Types
// ============================================================================

pub struct SemanticWorkspaceTreeSimulator {
    state: SemanticWorkspaceTree,
    known_chats: HashSet<u64>,
}

impl SemanticWorkspaceTreeSimulator {
    pub fn new() -> Self {
        let mut known_chats = HashSet::new();
        known_chats.insert(100);
        known_chats.insert(200);

        let initial_hash = SemanticWorkspaceTree::compute_hash_parts("Normal", None, "", None, false);
        Self {
            state: SemanticWorkspaceTree {
                timestamp_ms: chrono::Utc::now().timestamp_millis(),
                window_layout: "Normal".into(),
                selected_chat_id: None,
                composer_text: String::new(),
                active_modal: None,
                is_call_active: false,
                window: SemanticWindowNode {
                    layout: "Normal".into(),
                    width_px: 1280.0,
                    height_px: 800.0,
                    active_panel: "Chats".into(),
                    is_authenticated: true,
                },
                nav_rail: SemanticNavRailNode::default(),
                sidebar: SemanticSidebarNode {
                    dialog_count: known_chats.len(),
                    selected_chat_id: None,
                    search_query: String::new(),
                },
                stage: SemanticStageNode::default(),
                composer: SemanticComposerNode::default(),
                inspector: SemanticInspectorNode::default(),
                active_modal_node: None,
                state_hash: initial_hash,
            },
            known_chats,
        }
    }

    pub fn register_chat(&mut self, chat_id: u64) {
        self.known_chats.insert(chat_id);
        self.state.sidebar.dialog_count = self.known_chats.len();
    }

    pub fn inspect_state(&self) -> SemanticWorkspaceTree {
        self.state.clone()
    }

    pub fn perform_action(&mut self, action: SemanticAction) -> SemanticActionReceipt {
        let action_id = uuid::Uuid::new_v4().to_string();
        let (action_type, success, error) = match &action {
            SemanticAction::SelectChat { chat_id } => {
                if self.known_chats.contains(chat_id) {
                    self.state.selected_chat_id = Some(*chat_id);
                    self.state.sidebar.selected_chat_id = Some(*chat_id);
                    self.state.stage.active_chat_id = Some(*chat_id);
                    ("SelectChat", true, None)
                } else {
                    (
                        "SelectChat",
                        false,
                        Some(format!("Unknown chat_id: {chat_id}")),
                    )
                }
            }
            SemanticAction::ComposeText { text } => {
                self.state.composer_text = text.clone();
                self.state.composer.text = text.clone();
                ("ComposeText", true, None)
            }
            SemanticAction::SendMessage => {
                if self.state.selected_chat_id.is_none() {
                    ("SendMessage", false, Some("No chat selected".into()))
                } else if self.state.composer_text.trim().is_empty() {
                    (
                        "SendMessage",
                        false,
                        Some("Cannot send empty message".into()),
                    )
                } else {
                    self.state.composer_text.clear();
                    self.state.composer.text.clear();
                    self.state.stage.message_count += 1;
                    ("SendMessage", true, None)
                }
            }
            SemanticAction::OpenModal { modal_id } => {
                self.state.active_modal = Some(modal_id.clone());
                self.state.active_modal_node = Some(SemanticModalNode {
                    modal_id: modal_id.clone(),
                    title: modal_id.clone(),
                });
                ("OpenModal", true, None)
            }
            SemanticAction::CloseModal => {
                self.state.active_modal = None;
                self.state.active_modal_node = None;
                ("CloseModal", true, None)
            }
            SemanticAction::ToggleCall { .. } => {
                self.state.is_call_active = !self.state.is_call_active;
                ("ToggleCall", true, None)
            }
        };

        let now_ms = chrono::Utc::now().timestamp_millis();
        self.state.timestamp_ms = now_ms;
        self.state.recompute_state_hash();

        SemanticActionReceipt {
            action_id,
            action_type: action_type.to_string(),
            success,
            status: if success {
                ActionExecutionStatus::Succeeded
            } else {
                ActionExecutionStatus::Failed
            },
            error,
            timestamp_ms: now_ms,
            mutation_summary: json!({ "action": action_type, "success": success }),
            resulting_state_hash: self.state.state_hash.clone(),
        }
    }
}
