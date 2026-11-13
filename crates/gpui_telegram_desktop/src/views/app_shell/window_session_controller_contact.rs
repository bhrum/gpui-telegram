//! WindowSessionControllerContact: Deep link and contact token resolution state machine
//! Upstream: Telegram/SourceFiles/window/window_session_controller_contact.cpp

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContactResolveStatus {
    Idle,
    Resolving { token: String },
    Resolved { token: String, peer_id: u64 },
    Failed { error: String, is_flood: bool },
}

#[derive(Debug, Clone)]
pub struct WindowSessionControllerContact {
    pub status: ContactResolveStatus,
    pub pending_requests: u32,
}

impl WindowSessionControllerContact {
    pub fn new() -> Self {
        Self {
            status: ContactResolveStatus::Idle,
            pending_requests: 0,
        }
    }

    pub fn resolve_contact_token(&mut self, token: impl Into<String>) {
        let t = token.into();
        self.pending_requests += 1;
        self.status = ContactResolveStatus::Resolving { token: t };
    }

    pub fn cancel_contact_token_resolve(&mut self) {
        if self.pending_requests > 0 {
            self.pending_requests -= 1;
        }
        self.status = ContactResolveStatus::Idle;
    }

    pub fn complete_resolve(&mut self, token: String, peer_id: u64) {
        if self.pending_requests > 0 {
            self.pending_requests -= 1;
        }
        self.status = ContactResolveStatus::Resolved { token, peer_id };
    }

    pub fn fail_resolve(&mut self, error: impl Into<String>, is_flood: bool) {
        if self.pending_requests > 0 {
            self.pending_requests -= 1;
        }
        self.status = ContactResolveStatus::Failed {
            error: error.into(),
            is_flood,
        };
    }
}

impl Default for WindowSessionControllerContact {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowSessionControllerContact {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .p(px(8.0))
            .child(format!("Contact Token Resolver: {:?}", self.status))
    }
}
