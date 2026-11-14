//! Domain module integrating and re-exporting core messaging domain models from `fabushi-messaging-core`.

pub use fabushi_messaging_core::{
    actor, conversation, dialog, engine_tdrp_dialogs, engine_tdrp_media_stories,
    engine_tdrp_moderation, engine_tdrp_topics_channels, message, miniapp, payment, realtime,
    secret_chat, story, wallet, MessagingState,
};

/// Desktop-side domain state wrapper bridging `fabushi-messaging-core` snapshots into `TelegramApp`.
#[derive(Debug, Clone, Default)]
pub struct DesktopDomainBridge {
    pub messaging_state: MessagingState,
    pub active_actor_id: Option<actor::ActorId>,
    pub active_conversation_id: Option<conversation::ConversationId>,
}

impl DesktopDomainBridge {
    /// Constructs a clean, unpopulated desktop domain bridge with zero fabricated conversations.
    pub fn new() -> Self {
        Self::default()
    }

    /// Binds an authenticated desktop user identity to the domain coordinator.
    pub fn bind_actor(&mut self, user_id: u64) {
        self.active_actor_id = Some(actor::ActorId(format!("tg:{user_id}")));
    }

    /// Clears bound actor and active conversation upon logout or account switch.
    pub fn reset_session(&mut self) {
        self.active_actor_id = None;
        self.active_conversation_id = None;
    }

    /// Updates the currently focused conversation identifier.
    pub fn select_conversation(&mut self, chat_id: u64) {
        self.active_conversation_id = Some(conversation::ConversationId(format!("chat:{chat_id}")));
    }

    /// Human-readable status text mapped from `fabushi_messaging_core::realtime::CallState`.
    pub fn call_state_label(state: realtime::CallState) -> &'static str {
        match state {
            realtime::CallState::Ringing => "Ringing...",
            realtime::CallState::Connecting => "Connecting...",
            realtime::CallState::Active => "Call in progress",
            realtime::CallState::Held => "Call on hold",
            realtime::CallState::Ended => "Call ended",
            realtime::CallState::Failed => "Call failed",
        }
    }
}
