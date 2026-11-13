use crate::cache::LocalStore;
use crate::net::{HttpClient, WsChannel};
use crate::sync::ClientSyncCoordinator;
use gpui_telegram_core::error::Result;
use gpui_telegram_core::types::ChatId;
use gpui_telegram_proto::message::MessageItem;

pub struct TelegramClient {
    pub local_store: LocalStore,
    pub sync_coordinator: ClientSyncCoordinator,
    pub http_client: HttpClient,
    pub ws_channel: WsChannel,
}

impl TelegramClient {
    pub fn new(server_url: String) -> Self {
        Self {
            local_store: LocalStore::new(),
            sync_coordinator: ClientSyncCoordinator::new(),
            http_client: HttpClient::new(server_url),
            ws_channel: WsChannel::new(),
        }
    }

    pub fn get_messages(&self, chat_id: ChatId) -> Result<Vec<MessageItem>> {
        self.local_store.get_messages(chat_id)
    }
}
