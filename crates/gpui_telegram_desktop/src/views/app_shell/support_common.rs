//! SupportCommon: Shared data structures and ticket state models for customer support
//! Upstream: Telegram/SourceFiles/window/support_common.h, support_common.cpp

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TicketStatus {
    Open,
    PendingUserResponse,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportTicket {
    pub ticket_id: u64,
    pub user_id: u64,
    pub subject: String,
    pub status: TicketStatus,
    pub assigned_agent: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SupportCommon {
    pub active_tickets: Vec<SupportTicket>,
}

impl SupportCommon {
    pub fn new() -> Self {
        Self {
            active_tickets: Vec::new(),
        }
    }

    pub fn create_ticket(&mut self, user_id: u64, subject: impl Into<String>) -> u64 {
        let id = self.active_tickets.len() as u64 + 1;
        self.active_tickets.push(SupportTicket {
            ticket_id: id,
            user_id,
            subject: subject.into(),
            status: TicketStatus::Open,
            assigned_agent: None,
        });
        id
    }

    pub fn assign_agent(&mut self, ticket_id: u64, agent: impl Into<String>) {
        if let Some(t) = self.active_tickets.iter_mut().find(|t| t.ticket_id == ticket_id) {
            t.assigned_agent = Some(agent.into());
            t.status = TicketStatus::PendingUserResponse;
        }
    }

    pub fn resolve_ticket(&mut self, ticket_id: u64) {
        if let Some(t) = self.active_tickets.iter_mut().find(|t| t.ticket_id == ticket_id) {
            t.status = TicketStatus::Resolved;
        }
    }
}

impl Default for SupportCommon {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SupportCommon {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0x82868A))
            .child(format!("SupportCommon: {} tickets", self.active_tickets.len()))
    }
}
