use crate::cache::LocalStore;
use crate::client_dispatch::{apply_ws_frame, ClientDomainEvent};
use crate::config::ClientConfig;
use crate::net::{
    AuthResponse, ConnectionStatus, HermesApprovalDecisionRequest, HermesApprovalDto,
    HermesCallInviteRequest, HermesCallInviteResponse, HermesClient, HermesCommandDescriptor,
    HermesCommandRequest, HermesCommandResponse, HermesCreateKanbanBoardRequest,
    HermesCreateKanbanCardRequest, HermesCronJobDto, HermesHeartbeatConfigDto,
    HermesHonchoRequest, HermesHonchoStateDto, HermesKanbanBoardDto, HermesKanbanCardDto,
    HermesMemoryMutationRequest, HermesMemoryStateDto, HermesMoveKanbanCardRequest,
    HermesProviderConfigDto, HermesRichChatDto, HermesRichMessageDto, HermesRollbackRequest,
    HermesRollbackResponse, HermesSaveCronRequest, HermesSaveProviderRequest,
    HermesSaveSkillRequest, HermesSessionBranchRequest, HermesSessionDetailDto,
    HermesSessionDto, HermesSessionExportResponse, HermesSessionSearchRequest, HermesSkillDto,
    HermesSoulDto, HermesStatusDto, HermesTalkTurnRequest, HermesTalkTurnResponse,
    HermesTerminalProcessDto, HermesToolsetDto, HermesTurnRequest, HermesTurnResponse,
    HermesUpdateToolsetsRequest, HermesUsageSummaryDto, HttpClient, HttpClientError,
    OpenAiChatCompletionsRequest, WsChannel, WsTransport,
};
use crate::sync::ClientSyncCoordinator;
use gpui_telegram_core::error::Result;
use gpui_telegram_core::types::{ChatId, MessageId};
use gpui_telegram_proto::message::MessageItem;
use gpui_telegram_proto::ws::{WsClientFrame, WsServerFrame};
use parking_lot::{Mutex, RwLock};
use std::collections::HashSet;
use std::path::Path;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Runtime;

pub struct TelegramClient {
    pub config: ClientConfig,
    pub local_store: LocalStore,
    pub sync_coordinator: ClientSyncCoordinator,
    pub http_client: HttpClient,
    pub hermes: HermesClient,
    pub ws_channel: WsChannel,
    pub ws_transport: WsTransport,
    runtime: Arc<Runtime>,
    auth_session: Arc<RwLock<Option<AuthResponse>>>,
    auth_token: Arc<RwLock<Option<String>>>,
    pending_events: Arc<Mutex<Vec<ClientDomainEvent>>>,
    gap_chats: Arc<Mutex<HashSet<ChatId>>>,
    next_nonce: Arc<AtomicU64>,
    next_client_msg_offset: Arc<AtomicU32>,
}

impl TelegramClient {
    pub fn new(config: ClientConfig) -> Self {
        Self::with_store(config, LocalStore::new())
    }

    pub fn open_with_dir(config: ClientConfig, cache_dir: impl AsRef<Path>) -> Result<Self> {
        let store = LocalStore::open_with_dir(cache_dir)?;
        Ok(Self::with_store(config, store))
    }

    fn with_store(config: ClientConfig, local_store: LocalStore) -> Self {
        let runtime = Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2)
                .thread_name("gpui-tokio-client")
                .enable_all()
                .build()
                .expect("Failed to initialize client Tokio runtime"),
        );

        let handle = runtime.handle().clone();
        let http_client = HttpClient::new_with_runtime(
            config.server_url.clone(),
            Duration::from_secs(config.http_timeout_secs),
            Some(handle.clone()),
        );
        let hermes = HermesClient::new(http_client.clone());
        let ws_channel = WsChannel::new();
        let ws_transport = WsTransport::new_with_runtime(
            config.server_url.clone(),
            ws_channel.clone(),
            Some(handle),
        );

        let seed_nonce = chrono::Utc::now().timestamp_millis().max(1) as u64;
        Self {
            config,
            local_store,
            sync_coordinator: ClientSyncCoordinator::new(),
            http_client,
            hermes,
            ws_channel,
            ws_transport,
            runtime,
            auth_session: Arc::new(RwLock::new(None)),
            auth_token: Arc::new(RwLock::new(None)),
            pending_events: Arc::new(Mutex::new(Vec::new())),
            gap_chats: Arc::new(Mutex::new(HashSet::new())),
            next_nonce: Arc::new(AtomicU64::new(seed_nonce)),
            next_client_msg_offset: Arc::new(AtomicU32::new(1)),
        }
    }

    pub fn new_with_url(server_url: String) -> Self {
        let config = ClientConfig {
            server_url,
            ..Default::default()
        };
        Self::new(config)
    }

    pub fn runtime(&self) -> &Arc<Runtime> {
        &self.runtime
    }

    pub fn status(&self) -> ConnectionStatus {
        self.ws_transport.status()
    }

    pub fn is_connected(&self) -> bool {
        self.ws_transport.is_connected()
    }

    pub fn set_auth_session(&self, session: AuthResponse) {
        self.set_token(Some(session.token.clone()));
        *self.auth_session.write() = Some(session);
    }

    pub fn set_token(&self, token: Option<String>) {
        self.ws_transport.set_token(token.clone());
        *self.auth_token.write() = token;
    }

    pub fn token(&self) -> Option<String> {
        self.auth_token
            .read()
            .clone()
            .or_else(|| self.auth_session.read().as_ref().map(|s| s.token.clone()))
    }

    pub fn current_auth_session(&self) -> Option<AuthResponse> {
        self.auth_session.read().clone()
    }

    pub fn current_user_id(&self) -> Option<u64> {
        self.auth_session.read().as_ref().map(|s| s.user_id)
    }

    pub fn clear_session(&self) {
        self.ws_transport.stop();
        self.ws_transport.set_token(None);
        *self.auth_session.write() = None;
        *self.auth_token.write() = None;
    }

    pub(crate) fn require_token(&self) -> std::result::Result<String, HttpClientError> {
        self.token()
            .filter(|t| !t.trim().is_empty())
            .ok_or_else(|| HttpClientError::Unauthorized("No active session token".to_string()))
    }

    pub fn connect_ws(&self) {
        self.ws_transport.start();
    }

    pub fn disconnect_ws(&self) {
        self.ws_transport.stop();
    }

    pub fn send_ws_frame(&self, frame: WsClientFrame) -> bool {
        self.ws_channel.send(frame)
    }

    pub fn next_client_nonce(&self) -> u64 {
        self.next_nonce.fetch_add(1, Ordering::Relaxed)
    }

    pub(crate) fn next_provisional_msg_id(&self) -> MessageId {
        let offset = self.next_client_msg_offset.fetch_add(1, Ordering::Relaxed);
        gpui_telegram_core::types::MsgId::generate_client(offset)
    }

    pub fn get_messages(&self, chat_id: ChatId) -> Result<Vec<MessageItem>> {
        self.local_store.get_messages(chat_id)
    }

    pub fn process_ws_frame(&self, frame: WsServerFrame) -> Vec<ClientDomainEvent> {
        let mut events = Vec::new();
        let gaps = apply_ws_frame(&self.local_store, &self.sync_coordinator, frame, &mut events);
        if !gaps.is_empty() {
            self.gap_chats.lock().extend(gaps);
        }
        if !events.is_empty() {
            self.pending_events.lock().extend(events.iter().cloned());
        }
        events
    }

    pub fn poll_events(&self) -> Vec<ClientDomainEvent> {
        while let Some(frame) = self.ws_channel.try_recv() {
            let mut batch = Vec::new();
            let gaps =
                apply_ws_frame(&self.local_store, &self.sync_coordinator, frame, &mut batch);
            if !gaps.is_empty() {
                self.gap_chats.lock().extend(gaps);
            }
            if !batch.is_empty() {
                self.pending_events.lock().extend(batch);
            }
        }
        std::mem::take(&mut *self.pending_events.lock())
    }

    pub fn pending_gap_chats(&self) -> Vec<ChatId> {
        self.gap_chats.lock().iter().copied().collect()
    }

    pub(crate) fn clear_gap_chat(&self, chat_id: ChatId) {
        self.gap_chats.lock().remove(&chat_id);
    }

    pub fn hermes(&self) -> &HermesClient {
        &self.hermes
    }

    pub async fn hermes_status(&self) -> std::result::Result<HermesStatusDto, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.status(&token).await
    }

    pub async fn hermes_run_turn(
        &self,
        req: &HermesTurnRequest,
    ) -> std::result::Result<HermesTurnResponse, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.run_turn(&token, req).await
    }

    pub async fn hermes_run_command(
        &self,
        req: &HermesCommandRequest,
    ) -> std::result::Result<HermesCommandResponse, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.run_command(&token, req).await
    }

    pub async fn hermes_list_commands(
        &self,
        prefix: Option<&str>,
    ) -> std::result::Result<Vec<HermesCommandDescriptor>, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.list_commands(&token, prefix).await
    }

    pub async fn hermes_list_sessions(
        &self,
    ) -> std::result::Result<Vec<HermesSessionDto>, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.list_sessions(&token).await
    }

    pub async fn hermes_get_session(
        &self,
        session_id: &str,
    ) -> std::result::Result<HermesSessionDetailDto, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.get_session(&token, session_id).await
    }

    pub async fn hermes_search_sessions(
        &self,
        req: &HermesSessionSearchRequest,
    ) -> std::result::Result<Vec<serde_json::Value>, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.search_sessions(&token, req).await
    }

    pub async fn hermes_branch_session(
        &self,
        session_id: &str,
        req: &HermesSessionBranchRequest,
    ) -> std::result::Result<HermesSessionDto, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.branch_session(&token, session_id, req).await
    }

    pub async fn hermes_export_session(
        &self,
        session_id: &str,
        format: Option<&str>,
    ) -> std::result::Result<HermesSessionExportResponse, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.export_session(&token, session_id, format).await
    }

    pub async fn hermes_get_memory(
        &self,
    ) -> std::result::Result<HermesMemoryStateDto, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.get_memory(&token).await
    }

    pub async fn hermes_mutate_memory(
        &self,
        req: &HermesMemoryMutationRequest,
    ) -> std::result::Result<HermesMemoryStateDto, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.mutate_memory(&token, req).await
    }

    pub async fn hermes_get_soul(
        &self,
        bot_id: Option<u64>,
    ) -> std::result::Result<HermesSoulDto, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.get_soul(&token, bot_id).await
    }

    pub async fn hermes_save_soul(
        &self,
        soul: &HermesSoulDto,
    ) -> std::result::Result<HermesSoulDto, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.save_soul(&token, soul).await
    }

    pub async fn hermes_get_honcho(
        &self,
    ) -> std::result::Result<HermesHonchoStateDto, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.get_honcho(&token).await
    }

    pub async fn hermes_post_honcho(
        &self,
        req: &HermesHonchoRequest,
    ) -> std::result::Result<HermesHonchoStateDto, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.post_honcho(&token, req).await
    }

    pub async fn hermes_list_skills(
        &self,
    ) -> std::result::Result<Vec<HermesSkillDto>, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.list_skills(&token).await
    }

    pub async fn hermes_save_skill(
        &self,
        req: &HermesSaveSkillRequest,
    ) -> std::result::Result<HermesSkillDto, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.save_skill(&token, req).await
    }

    pub async fn hermes_delete_skill(
        &self,
        name: &str,
    ) -> std::result::Result<(), HttpClientError> {
        let token = self.require_token()?;
        self.hermes.delete_skill(&token, name).await
    }

    pub async fn hermes_list_toolsets(
        &self,
    ) -> std::result::Result<Vec<HermesToolsetDto>, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.list_toolsets(&token).await
    }

    pub async fn hermes_update_toolsets(
        &self,
        req: &HermesUpdateToolsetsRequest,
    ) -> std::result::Result<Vec<HermesToolsetDto>, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.update_toolsets(&token, req).await
    }

    pub async fn hermes_list_approvals(
        &self,
    ) -> std::result::Result<Vec<HermesApprovalDto>, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.list_approvals(&token).await
    }

    pub async fn hermes_decide_approval(
        &self,
        approval_id: &str,
        req: &HermesApprovalDecisionRequest,
    ) -> std::result::Result<HermesApprovalDto, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.decide_approval(&token, approval_id, req).await
    }

    pub async fn hermes_list_cron(
        &self,
    ) -> std::result::Result<Vec<HermesCronJobDto>, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.list_cron(&token).await
    }

    pub async fn hermes_save_cron(
        &self,
        req: &HermesSaveCronRequest,
    ) -> std::result::Result<HermesCronJobDto, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.save_cron(&token, req).await
    }

    pub async fn hermes_delete_cron(
        &self,
        job_id: &str,
    ) -> std::result::Result<(), HttpClientError> {
        let token = self.require_token()?;
        self.hermes.delete_cron(&token, job_id).await
    }

    pub async fn hermes_run_cron(
        &self,
        job_id: &str,
    ) -> std::result::Result<serde_json::Value, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.run_cron(&token, job_id).await
    }

    pub async fn hermes_get_heartbeat(
        &self,
    ) -> std::result::Result<HermesHeartbeatConfigDto, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.get_heartbeat(&token).await
    }

    pub async fn hermes_save_heartbeat(
        &self,
        cfg: &HermesHeartbeatConfigDto,
    ) -> std::result::Result<HermesHeartbeatConfigDto, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.save_heartbeat(&token, cfg).await
    }

    pub async fn hermes_kanban_boards(
        &self,
    ) -> std::result::Result<Vec<HermesKanbanBoardDto>, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.kanban_boards(&token).await
    }

    pub async fn hermes_kanban_create_board(
        &self,
        req: &HermesCreateKanbanBoardRequest,
    ) -> std::result::Result<HermesKanbanBoardDto, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.kanban_create_board(&token, req).await
    }

    pub async fn hermes_kanban_create_card(
        &self,
        req: &HermesCreateKanbanCardRequest,
    ) -> std::result::Result<HermesKanbanCardDto, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.kanban_create_card(&token, req).await
    }

    pub async fn hermes_kanban_move_card(
        &self,
        card_id: &str,
        req: &HermesMoveKanbanCardRequest,
    ) -> std::result::Result<HermesKanbanCardDto, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.kanban_move_card(&token, card_id, req).await
    }

    pub async fn hermes_usage_stats(
        &self,
    ) -> std::result::Result<HermesUsageSummaryDto, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.usage_stats(&token).await
    }

    pub async fn hermes_list_providers(
        &self,
    ) -> std::result::Result<Vec<HermesProviderConfigDto>, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.list_providers(&token).await
    }

    pub async fn hermes_save_provider(
        &self,
        req: &HermesSaveProviderRequest,
    ) -> std::result::Result<HermesProviderConfigDto, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.save_provider(&token, req).await
    }

    pub async fn hermes_rollback_turn(
        &self,
        req: &HermesRollbackRequest,
    ) -> std::result::Result<HermesRollbackResponse, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.rollback_turn(&token, req).await
    }

    pub async fn hermes_terminal_processes(
        &self,
    ) -> std::result::Result<Vec<HermesTerminalProcessDto>, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.terminal_processes(&token).await
    }

    pub async fn hermes_chat_completions(
        &self,
        req: &OpenAiChatCompletionsRequest,
    ) -> std::result::Result<serde_json::Value, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.chat_completions(&token, req).await
    }

    pub async fn hermes_talk_turn(
        &self,
        req: &HermesTalkTurnRequest,
    ) -> std::result::Result<HermesTalkTurnResponse, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.talk_turn(&token, req).await
    }

    pub async fn hermes_call_invite(
        &self,
        req: &HermesCallInviteRequest,
    ) -> std::result::Result<HermesCallInviteResponse, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.call_invite(&token, req).await
    }

    pub async fn hermes_list_chats_rich(
        &self,
    ) -> std::result::Result<Vec<HermesRichChatDto>, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.list_chats_rich(&token).await
    }

    pub async fn hermes_get_chat_rich(
        &self,
        chat_id: i64,
    ) -> std::result::Result<HermesRichChatDto, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.get_chat_rich(&token, chat_id).await
    }

    pub async fn hermes_messages_history_rich(
        &self,
        chat_id: i64,
        limit: Option<usize>,
    ) -> std::result::Result<Vec<HermesRichMessageDto>, HttpClientError> {
        let token = self.require_token()?;
        self.hermes.messages_history_rich(&token, chat_id, limit).await
    }
}
