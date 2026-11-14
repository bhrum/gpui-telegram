use chrono::Utc;
use gpui_telegram_core::types::{ChatId, MessageFlags, MessageId, MsgId, PeerId, UserId, UserStatus};
use gpui_telegram_proto::contracts::CallSignalRequest;
use gpui_telegram_proto::message::{DeleteTombstone, MessageContent, MessageItem, ReactionCount, ReplyHeader};
use gpui_telegram_proto::sync::{
    PtsUpdate, ReadHistoryEvent, ReactionEvent, SyncGetDifferenceResponse, UpdateEvent,
};
use gpui_telegram_proto::ws::{WsClientFrame, WsServerFrame};
use gpui_telegram_webrtc::signaling::SignalingFrame;
use parking_lot::RwLock;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

/// GatewayHub manages real-time WebSocket connections across multiple devices per user,
/// chat event fan-out, and bidirectional frame handling.
#[derive(Debug, Clone)]
pub struct GatewayHub {
    /// Multi-device socket registry: UserId -> list of active socket senders
    subscribers: Arc<RwLock<HashMap<UserId, Vec<mpsc::UnboundedSender<WsServerFrame>>>>>,
    /// Chat subscriptions: ChatId -> Set of subscribed UserIds
    chat_subscribers: Arc<RwLock<HashMap<i64, HashSet<UserId>>>>,
    /// Next allocated sequential message ID
    next_msg_id: Arc<AtomicI64>,
    /// Server presentation sequence counter (PTS)
    next_pts: Arc<AtomicU64>,
    /// Optional attached message pipeline for real persistence and deduplication
    pipeline: Arc<RwLock<Option<Arc<crate::messaging::pipeline::MessagePipeline>>>>,
    /// Optional attached server database for membership queries
    db: Arc<RwLock<Option<Arc<crate::db::ServerDatabase>>>>,
    /// Optional Tantivy search indexer for real-time WS message indexing
    search_indexer: Arc<RwLock<Option<Arc<crate::search::tantivy_indexer::TantivyIndexer>>>>,
    /// Optional WebRTC signaling hub for call session state & relay
    signaling_hub: Arc<RwLock<Option<Arc<crate::webrtc::signaling_hub::SignalingHub>>>>,
}

impl Default for GatewayHub {
    fn default() -> Self {
        Self {
            subscribers: Arc::new(RwLock::new(HashMap::new())),
            chat_subscribers: Arc::new(RwLock::new(HashMap::new())),
            next_msg_id: Arc::new(AtomicI64::new(1001)),
            next_pts: Arc::new(AtomicU64::new(1)),
            pipeline: Arc::new(RwLock::new(None)),
            db: Arc::new(RwLock::new(None)),
            search_indexer: Arc::new(RwLock::new(None)),
            signaling_hub: Arc::new(RwLock::new(None)),
        }
    }
}

impl GatewayHub {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_pipeline(
        pipeline: Arc<crate::messaging::pipeline::MessagePipeline>,
        db: Arc<crate::db::ServerDatabase>,
    ) -> Self {
        let hub = Self::default();
        *hub.pipeline.write() = Some(pipeline);
        *hub.db.write() = Some(db);
        hub
    }

    pub fn set_pipeline(
        &self,
        pipeline: Arc<crate::messaging::pipeline::MessagePipeline>,
        db: Arc<crate::db::ServerDatabase>,
    ) {
        *self.pipeline.write() = Some(pipeline);
        *self.db.write() = Some(db);
    }

    pub fn set_search_indexer(&self, indexer: Arc<crate::search::tantivy_indexer::TantivyIndexer>) {
        *self.search_indexer.write() = Some(indexer);
    }

    pub fn set_signaling_hub(&self, sh: Arc<crate::webrtc::signaling_hub::SignalingHub>) {
        *self.signaling_hub.write() = Some(sh);
    }

    pub fn signaling_hub(&self) -> Option<Arc<crate::webrtc::signaling_hub::SignalingHub>> {
        self.signaling_hub.read().clone()
    }

    /// Register an active WebSocket connection sender for a user
    pub fn register(&self, user_id: UserId, sender: mpsc::UnboundedSender<WsServerFrame>) {
        let mut subs = self.subscribers.write();
        subs.entry(user_id).or_default().push(sender);
        info!("Registered new WebSocket connection for {}", user_id);
    }

    /// Unregister a specific connection sender for a user
    pub fn unregister_sender(&self, user_id: UserId, sender: &mpsc::UnboundedSender<WsServerFrame>) {
        let mut subs = self.subscribers.write();
        if let Some(senders) = subs.get_mut(&user_id) {
            senders.retain(|s| !s.is_closed() && !s.same_channel(sender));
            if senders.is_empty() {
                subs.remove(&user_id);
            }
        }
        info!("Unregistered WebSocket connection for {}", user_id);
    }

    /// Unregister all connections for a user
    pub fn unregister(&self, user_id: UserId) {
        self.subscribers.write().remove(&user_id);
    }

    /// Disconnect all active sockets for a user (called on logout/revocation)
    pub fn disconnect_user(&self, user_id: UserId) {
        let senders = self.subscribers.write().remove(&user_id);
        if let Some(list) = senders {
            for sender in list {
                let _ = sender.send(WsServerFrame::Error {
                    code: 401,
                    message: "Session has been revoked".into(),
                    details: None,
                });
            }
        }
    }

    /// Get active connection count for a specific user
    pub fn user_connection_count(&self, user_id: UserId) -> usize {
        self.subscribers
            .read()
            .get(&user_id)
            .map(|list| list.iter().filter(|s| !s.is_closed()).count())
            .unwrap_or(0)
    }

    /// Total count of all active WebSocket connections across all users
    pub fn total_connections(&self) -> usize {
        self.subscribers
            .read()
            .values()
            .map(|list| list.iter().filter(|s| !s.is_closed()).count())
            .sum()
    }

    /// Broadcast frame to all active device sockets belonging to a user
    pub fn broadcast_to_user(&self, user_id: UserId, frame: WsServerFrame) {
        let mut subs = self.subscribers.write();
        if let Some(senders) = subs.get_mut(&user_id) {
            senders.retain(|tx| tx.send(frame.clone()).is_ok());
            if senders.is_empty() {
                subs.remove(&user_id);
            }
        }
    }

    /// Broadcast frame to all subscribers of a chat
    pub fn broadcast_to_chat(&self, chat_id: i64, frame: WsServerFrame) {
        let targets: Vec<UserId> = self
            .chat_subscribers
            .read()
            .get(&chat_id)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .collect();

        for target in targets {
            self.broadcast_to_user(target, frame.clone());
        }
    }

    /// Subscribe user to chat events
    pub fn subscribe_chat(&self, chat_id: i64, user_id: UserId) {
        self.chat_subscribers
            .write()
            .entry(chat_id)
            .or_default()
            .insert(user_id);
    }

    /// Returns a set of user IDs that are subscribed to a chat
    pub fn get_chat_subscribers(&self, chat_id: i64) -> HashSet<UserId> {
        self.chat_subscribers
            .read()
            .get(&chat_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Unsubscribe user from chat events
    pub fn unsubscribe_chat(&self, chat_id: i64, user_id: UserId) {
        let mut chats = self.chat_subscribers.write();
        if let Some(subs) = chats.get_mut(&chat_id) {
            subs.remove(&user_id);
            if subs.is_empty() {
                chats.remove(&chat_id);
            }
        }
    }

    /// Current PTS sequence number
    pub fn current_pts(&self) -> u64 {
        self.next_pts.load(Ordering::Acquire)
    }

    /// Allocate the next PTS sequence number
    pub fn next_pts(&self) -> u64 {
        self.next_pts.fetch_add(1, Ordering::SeqCst) + 1
    }

    /// Collect all DB members + explicit subscribers of a chat and push frame to each.
    pub fn broadcast_to_chat_and_members(
        &self,
        chat_id: ChatId,
        ensure_user: Option<UserId>,
        frame: WsServerFrame,
    ) {
        let mut targets = HashSet::new();
        if let Some(uid) = ensure_user {
            targets.insert(uid);
        }
        if let Some(ref db) = *self.db.read() {
            if let Ok(members) = db.get_chat_members(chat_id) {
                targets.extend(members);
            }
        }
        if let Some(subs) = self.chat_subscribers.read().get(&chat_id.0) {
            targets.extend(subs.iter().copied());
        }
        for uid in targets {
            self.broadcast_to_user(uid, frame.clone());
        }
    }

    fn is_member_in_db(db: &crate::db::ServerDatabase, chat_id: ChatId, user_id: UserId) -> Result<bool, u16> {
        match db.get_chat(chat_id) {
            Ok(Some(chat)) => {
                if chat.creator_id == user_id {
                    return Ok(true);
                }
                Ok(db.get_chat_member(chat_id, user_id).ok().flatten().is_some())
            }
            Ok(None) => Err(404),
            Err(_) => Err(500),
        }
    }

    fn is_admin_in_db(db: &crate::db::ServerDatabase, chat_id: ChatId, user_id: UserId) -> bool {
        if let Ok(Some(chat)) = db.get_chat(chat_id) {
            if chat.creator_id == user_id {
                return true;
            }
        }
        if let Ok(Some(member)) = db.get_chat_member(chat_id, user_id) {
            return member.role.eq_ignore_ascii_case("owner")
                || member.role.eq_ignore_ascii_case("creator")
                || member.role.eq_ignore_ascii_case("admin")
                || member.role.eq_ignore_ascii_case("administrator");
        }
        false
    }

    pub fn broadcast_user_status(&self, user_id: UserId, is_online: bool) {
        let peers: HashSet<UserId> = if let Some(ref db) = *self.db.read() {
            let mut set = HashSet::new();
            if let Ok(chats) = db.list_user_chats(user_id) {
                for chat in chats {
                    if let Ok(members) = db.get_chat_members(chat.chat_id) {
                        for m in members {
                            if m != user_id {
                                set.insert(m);
                            }
                        }
                    }
                }
            }
            set
        } else {
            return;
        };
        if peers.is_empty() {
            return;
        }
        let now = Utc::now();
        let status = if is_online {
            UserStatus::Online {
                expires_at: now + chrono::Duration::minutes(5),
            }
        } else {
            UserStatus::Offline {
                was_online: Some(now),
            }
        };
        let frame = WsServerFrame::Update(UpdateEvent::UserStatusChanged { user_id, status });
        for peer in peers {
            self.broadcast_to_user(peer, frame.clone());
        }
    }

    fn index_ws_message(&self, chat_id: ChatId, msg_id: i64, sender_id: UserId, date_ms: i64, content: &MessageContent) {
        if let Some(ref indexer) = *self.search_indexer.read() {
            let text = match content {
                MessageContent::Text { text, .. } => text.clone(),
                MessageContent::Photo { caption, .. }
                | MessageContent::Video { caption, .. } => caption.clone().unwrap_or_default(),
                MessageContent::Document { file_name, .. } => file_name.clone(),
                MessageContent::Poll(p) => p.question.clone(),
                _ => String::new(),
            };
            if !text.is_empty() {
                indexer.index_message_nonblocking(crate::search::schema::SearchDocument::new(
                    msg_id.to_string(),
                    chat_id.0.to_string(),
                    sender_id.0.to_string(),
                    date_ms,
                    text,
                    None,
                ));
                indexer.commit_nonblocking();
            }
        }
    }

    /// Handle incoming client WebSocket frame
    pub fn handle_client_frame(
        &self,
        user_id: UserId,
        frame: WsClientFrame,
        reply_tx: &mpsc::UnboundedSender<WsServerFrame>,
    ) {
        match frame {
            WsClientFrame::Ping { nonce } => {
                let _ = reply_tx.send(WsServerFrame::Pong { nonce });
            }
            WsClientFrame::Sync(req) => {
                let maybe_pipeline = self.pipeline.read().clone();
                if let Some(pipeline) = maybe_pipeline {
                    match pipeline.pts_engine().get_difference(&req) {
                        Ok(resp) => {
                            let _ = reply_tx.send(WsServerFrame::SyncDiff(resp));
                        }
                        Err(e) => {
                            let _ = reply_tx.send(WsServerFrame::Error {
                                code: 500,
                                message: format!("Sync error: {e}"),
                                details: None,
                            });
                        }
                    }
                } else {
                    let current_pts = self.current_pts();
                    let resp = SyncGetDifferenceResponse::new(
                        req.chat_id,
                        Vec::new(),
                        Vec::new(),
                        current_pts,
                        false,
                    );
                    let _ = reply_tx.send(WsServerFrame::SyncDiff(resp));
                }
            }
            WsClientFrame::SendMessage(req) => {
                let maybe_pipeline = self.pipeline.read().clone();
                let maybe_db = self.db.read().clone();

                if let Some(ref db) = maybe_db {
                    if let Err(code) = Self::is_member_in_db(db, req.chat_id, user_id) {
                        let _ = reply_tx.send(WsServerFrame::Error {
                            code,
                            message: if code == 404 { "Chat not found" } else { "Not a member of this chat" }.into(),
                            details: None,
                        });
                        return;
                    }
                }

                if let Some(pipeline) = maybe_pipeline {
                    let msg = MessageItem {
                        id: MessageId(0),
                        chat_id: req.chat_id,
                        peer_id: PeerId::User(user_id),
                        to_peer: PeerId::Chat(req.chat_id),
                        sender_id: user_id,
                        topic_id: req.topic_id,
                        flags: MessageFlags {
                            is_out: true,
                            is_silent: req.silent,
                            ..Default::default()
                        },
                        date: Utc::now(),
                        edit_date: None,
                        content: req.content.clone(),
                        reply_to: req.reply_to.map(ReplyHeader::new),
                        forward_info: None,
                        reactions: Vec::new(),
                        client_nonce: Some(req.client_nonce),
                    };

                    match pipeline.process_incoming_message_strict(msg) {
                        Ok(pts_update) => {
                            let (msg_id, date_ms) = match &pts_update.event {
                                UpdateEvent::NewMessage(item) => (item.id.0, item.date.timestamp_millis()),
                                _ => (0, pts_update.timestamp_ms),
                            };
                            if let Some(ref db) = maybe_db {
                                let kb_json = req.inline_keyboard.as_ref().and_then(|k| serde_json::to_string(k).ok());
                                if kb_json.is_some() || req.scheduled_at_ms.is_some() {
                                    let _ = db.save_message_extras(req.chat_id, MsgId(msg_id), kb_json.as_deref(), req.scheduled_at_ms);
                                }
                            }
                            self.index_ws_message(req.chat_id, msg_id, user_id, date_ms, &req.content);
                            let _ = reply_tx.send(WsServerFrame::Ack {
                                correlation_id: 0,
                                client_nonce: req.client_nonce,
                                message_id: msg_id,
                                pts: pts_update.pts,
                            });
                            self.broadcast_to_chat_and_members(req.chat_id, Some(user_id), WsServerFrame::PtsPush(pts_update));
                        }
                        Err(e) => {
                            let _ = reply_tx.send(WsServerFrame::Error {
                                code: 500,
                                message: format!("Failed to process message: {e}"),
                                details: None,
                            });
                        }
                    }
                } else {
                    let msg_id = MessageId(self.next_msg_id.fetch_add(1, Ordering::SeqCst));
                    let pts = self.next_pts();
                    let now = Utc::now();
                    let message = MessageItem {
                        id: msg_id,
                        chat_id: req.chat_id,
                        peer_id: PeerId::User(user_id),
                        to_peer: PeerId::Chat(req.chat_id),
                        sender_id: user_id,
                        topic_id: req.topic_id,
                        flags: MessageFlags { is_out: true, ..Default::default() },
                        date: now,
                        edit_date: None,
                        content: req.content,
                        reply_to: req.reply_to.map(ReplyHeader::new),
                        forward_info: None,
                        reactions: Vec::new(),
                        client_nonce: Some(req.client_nonce),
                    };
                    let _ = reply_tx.send(WsServerFrame::Ack {
                        correlation_id: 0,
                        client_nonce: req.client_nonce,
                        message_id: msg_id.0,
                        pts,
                    });
                    let pts_update = PtsUpdate::new(req.chat_id, pts, 1, UpdateEvent::NewMessage(message), now.timestamp_millis());
                    self.broadcast_to_user(user_id, WsServerFrame::PtsPush(pts_update.clone()));
                    self.broadcast_to_chat(req.chat_id.0, WsServerFrame::PtsPush(pts_update));
                }
            }
            WsClientFrame::EditMessage(req) => {
                let maybe_pipeline = self.pipeline.read().clone();
                let maybe_db = self.db.read().clone();
                if let (Some(pipeline), Some(db)) = (maybe_pipeline, maybe_db) {
                    if let Err(code) = Self::is_member_in_db(&db, req.chat_id, user_id) {
                        let _ = reply_tx.send(WsServerFrame::Error { code, message: "Not authorized for chat".into(), details: None });
                        return;
                    }
                    match db.get_message(req.chat_id, MsgId(req.message_id.0)) {
                        Ok(Some(rec)) if rec.deleted_at_ms.is_none() => {
                            if rec.sender_id != user_id && !Self::is_admin_in_db(&db, req.chat_id, user_id) {
                                let _ = reply_tx.send(WsServerFrame::Error { code: 403, message: "Cannot edit another user's message".into(), details: None });
                                return;
                            }
                        }
                        Ok(_) => {
                            let _ = reply_tx.send(WsServerFrame::Error { code: 404, message: "Message not found".into(), details: None });
                            return;
                        }
                        Err(e) => {
                            let _ = reply_tx.send(WsServerFrame::Error { code: 500, message: e.to_string(), details: None });
                            return;
                        }
                    }
                    match pipeline.edit_message(req.chat_id, MsgId(req.message_id.0), req.content.clone()) {
                        Ok(pts_update) => {
                            self.index_ws_message(req.chat_id, req.message_id.0, user_id, pts_update.timestamp_ms, &req.content);
                            self.broadcast_to_chat_and_members(req.chat_id, Some(user_id), WsServerFrame::PtsPush(pts_update));
                        }
                        Err(e) => {
                            let _ = reply_tx.send(WsServerFrame::Error { code: 500, message: e.to_string(), details: None });
                        }
                    }
                } else {
                    let pts = self.next_pts();
                    let now = Utc::now();
                    let message = MessageItem {
                        id: req.message_id, chat_id: req.chat_id, peer_id: PeerId::User(user_id),
                        to_peer: PeerId::Chat(req.chat_id), sender_id: user_id, topic_id: None,
                        flags: MessageFlags { is_out: true, is_edited: true, ..Default::default() },
                        date: now, edit_date: Some(now), content: req.content, reply_to: None,
                        forward_info: None, reactions: Vec::new(), client_nonce: None,
                    };
                    let pts_update = PtsUpdate::new(req.chat_id, pts, 1, UpdateEvent::EditMessage(message), now.timestamp_millis());
                    self.broadcast_to_user(user_id, WsServerFrame::PtsPush(pts_update.clone()));
                    self.broadcast_to_chat(req.chat_id.0, WsServerFrame::PtsPush(pts_update));
                }
            }
            WsClientFrame::DeleteMessages(req) => {
                let maybe_pipeline = self.pipeline.read().clone();
                let maybe_db = self.db.read().clone();
                if let (Some(pipeline), Some(db)) = (maybe_pipeline, maybe_db) {
                    if let Err(code) = Self::is_member_in_db(&db, req.chat_id, user_id) {
                        let _ = reply_tx.send(WsServerFrame::Error { code, message: "Not authorized for chat".into(), details: None });
                        return;
                    }
                    let is_admin = Self::is_admin_in_db(&db, req.chat_id, user_id);
                    if !is_admin {
                        for mid in &req.message_ids {
                            if let Ok(Some(rec)) = db.get_message(req.chat_id, MsgId(mid.0)) {
                                if rec.sender_id != user_id {
                                    let _ = reply_tx.send(WsServerFrame::Error { code: 403, message: "Cannot delete another user's message".into(), details: None });
                                    return;
                                }
                            }
                        }
                    }
                    let msg_ids: Vec<MsgId> = req.message_ids.iter().map(|m| MsgId(m.0)).collect();
                    match pipeline.delete_messages(req.chat_id, msg_ids, req.for_everyone) {
                        Ok(pts_update) => {
                            if let Some(ref indexer) = *self.search_indexer.read() {
                                for mid in &req.message_ids {
                                    indexer.delete_message_nonblocking(mid.0.to_string());
                                }
                                indexer.commit_nonblocking();
                            }
                            self.broadcast_to_chat_and_members(req.chat_id, Some(user_id), WsServerFrame::PtsPush(pts_update));
                        }
                        Err(e) => {
                            let _ = reply_tx.send(WsServerFrame::Error { code: 500, message: e.to_string(), details: None });
                        }
                    }
                } else {
                    let pts = self.next_pts();
                    let tombstone = DeleteTombstone::new(PeerId::Chat(req.chat_id), req.message_ids, req.for_everyone);
                    let pts_update = PtsUpdate::new(req.chat_id, pts, 1, UpdateEvent::DeleteMessages(tombstone), Utc::now().timestamp_millis());
                    self.broadcast_to_user(user_id, WsServerFrame::PtsPush(pts_update.clone()));
                    self.broadcast_to_chat(req.chat_id.0, WsServerFrame::PtsPush(pts_update));
                }
            }
            WsClientFrame::SendReaction(req) => {
                if let Some(db) = self.db.read().clone() {
                    if let Err(code) = Self::is_member_in_db(&db, req.chat_id, user_id) {
                        let _ = reply_tx.send(WsServerFrame::Error { code, message: "Not authorized for chat".into(), details: None });
                        return;
                    }
                    let action = if req.add { "add" } else { "remove" };
                    match db.toggle_reaction_atomic(req.chat_id, MsgId(req.message_id.0), user_id, &req.emoji, action) {
                        Ok((pts, _details, counts)) => {
                            let ev = ReactionEvent::new(req.chat_id, req.message_id, user_id, counts);
                            let pts_update = PtsUpdate::new(req.chat_id, pts, 1, UpdateEvent::ReactionUpdated(ev), Utc::now().timestamp_millis());
                            self.broadcast_to_chat_and_members(req.chat_id, Some(user_id), WsServerFrame::PtsPush(pts_update));
                        }
                        Err(e) => {
                            let _ = reply_tx.send(WsServerFrame::Error { code: 400, message: e.to_string(), details: None });
                        }
                    }
                } else {
                    let pts = self.next_pts();
                    let rx_count = ReactionCount::new(req.emoji, if req.add { 1 } else { 0 }, req.add);
                    let ev = ReactionEvent::new(req.chat_id, req.message_id, user_id, vec![rx_count]);
                    let pts_update = PtsUpdate::new(req.chat_id, pts, 1, UpdateEvent::ReactionUpdated(ev), Utc::now().timestamp_millis());
                    self.broadcast_to_user(user_id, WsServerFrame::PtsPush(pts_update.clone()));
                    self.broadcast_to_chat(req.chat_id.0, WsServerFrame::PtsPush(pts_update));
                }
            }
            WsClientFrame::ReadHistory(req) => {
                if let Some(db) = self.db.read().clone() {
                    if let Err(code) = Self::is_member_in_db(&db, req.chat_id, user_id) {
                        let _ = reply_tx.send(WsServerFrame::Error { code, message: "Not authorized for chat".into(), details: None });
                        return;
                    }
                    match db.mark_chat_read_atomic(req.chat_id, user_id, MsgId(req.max_id.0), 0) {
                        Ok((inbox_max, _outbox_max, unread, pts, senders)) => {
                            let now_ms = Utc::now().timestamp_millis();
                            let reader_ev = ReadHistoryEvent::new(req.chat_id, MessageId(inbox_max.0), false, unread);
                            let pts_update = PtsUpdate::new(req.chat_id, pts, 1, UpdateEvent::ReadHistory(reader_ev), now_ms);
                            self.broadcast_to_user(user_id, WsServerFrame::PtsPush(pts_update));
                            let outbox_update = PtsUpdate::new(
                                req.chat_id,
                                pts,
                                0,
                                UpdateEvent::ReadHistoryOut { max_id: MessageId(inbox_max.0) },
                                now_ms,
                            );
                            for sender in senders {
                                self.broadcast_to_user(sender, WsServerFrame::PtsPush(outbox_update.clone()));
                            }
                        }
                        Err(e) => {
                            let _ = reply_tx.send(WsServerFrame::Error { code: 500, message: e.to_string(), details: None });
                        }
                    }
                } else {
                    let pts = self.next_pts();
                    let ev = ReadHistoryEvent::new(req.chat_id, req.max_id, false, 0);
                    let pts_update = PtsUpdate::new(req.chat_id, pts, 1, UpdateEvent::ReadHistory(ev), Utc::now().timestamp_millis());
                    self.broadcast_to_user(user_id, WsServerFrame::PtsPush(pts_update.clone()));
                    self.broadcast_to_chat(req.chat_id.0, WsServerFrame::PtsPush(pts_update));
                }
            }
            WsClientFrame::SendTyping(mut event) => {
                event.user_id = user_id;
                let chat_id = event.chat_id;
                if let Some(ref db) = *self.db.read() {
                    if Self::is_member_in_db(db, chat_id, user_id).is_err() {
                        return;
                    }
                }
                let frame = WsServerFrame::Update(UpdateEvent::UserTyping(event));
                self.broadcast_to_chat_and_members(chat_id, None, frame);
            }
            WsClientFrame::SubscribeChat { chat_id } => {
                self.subscribe_chat(chat_id, user_id);
            }
            WsClientFrame::UnsubscribeChat { chat_id } => {
                self.unsubscribe_chat(chat_id, user_id);
            }
            WsClientFrame::VotePoll { chat_id, message_id: _, poll_id, option_ids } => {
                if let Some(db) = self.db.read().clone() {
                    if let Err(code) = Self::is_member_in_db(&db, chat_id, user_id) {
                        let _ = reply_tx.send(WsServerFrame::Error { code, message: "Not authorized for chat".into(), details: None });
                        return;
                    }
                    match db.vote_poll_atomic(poll_id as i64, user_id, &option_ids) {
                        Ok((cid, mid, pts, poll)) => {
                            let ev = UpdateEvent::PollUpdated { chat_id: cid, message_id: MessageId(mid.0), poll };
                            let pts_update = PtsUpdate::new(cid, pts, 1, ev, Utc::now().timestamp_millis());
                            self.broadcast_to_chat_and_members(cid, Some(user_id), WsServerFrame::PtsPush(pts_update));
                        }
                        Err(e) => {
                            let _ = reply_tx.send(WsServerFrame::Error { code: 400, message: e.to_string(), details: None });
                        }
                    }
                }
            }
            WsClientFrame::CallSignal {
                call_id,
                signal_type,
                target_user_id,
                sdp,
                candidate,
                sdp_mid,
                sdp_mline_index,
            } => {
                let db = self.db.read().clone();
                let sh = self.signaling_hub.read().clone();
                let resolved_peer = target_user_id.map(UserId).or_else(|| {
                    sh.as_ref().and_then(|s| s.get_session(&call_id).and_then(|sess| sess.peer_of(user_id).ok()))
                });
                let new_state = match signal_type.as_str() {
                    "ringing" => Some("ringing"),
                    "accept" | "answer" => Some("connected"),
                    "hangup" => Some("ended"),
                    _ => None,
                };
                if let (Some(d), Some(st)) = (&db, new_state) {
                    let _ = d.update_call_state_and_rating(&call_id, Some(st), None, None, None);
                }
                let sig_req = CallSignalRequest {
                    call_id: call_id.clone(),
                    signal_type: signal_type.clone(),
                    sdp: sdp.clone(),
                    candidate: candidate.clone(),
                    sdp_mid: sdp_mid.clone(),
                    sdp_mline_index,
                    reason: None,
                };
                if let Some(sh_arc) = sh {
                    let sig_frame = match signal_type.as_str() {
                        "invite" => resolved_peer.map(|to_user| SignalingFrame::Invite {
                            call_id: call_id.clone(),
                            from_user: user_id,
                            to_user,
                            is_video: false,
                        }),
                        "ringing" => Some(SignalingFrame::Ringing { call_id: call_id.clone() }),
                        "accept" => Some(SignalingFrame::Accept { call_id: call_id.clone() }),
                        "offer" => sdp.map(|s| SignalingFrame::Offer { call_id: call_id.clone(), sdp: s }),
                        "answer" => sdp.map(|s| SignalingFrame::Answer { call_id: call_id.clone(), sdp: s }),
                        "ice_candidate" => candidate.map(|c| SignalingFrame::IceCandidate {
                            call_id: call_id.clone(),
                            candidate: c,
                            sdp_mid,
                            sdp_mline_index: sdp_mline_index.map(|v| v as u16),
                        }),
                        "hangup" => Some(SignalingFrame::Hangup {
                            call_id: call_id.clone(),
                            reason: "user_hangup".into(),
                        }),
                        _ => None,
                    };
                    if let Some(frame) = sig_frame {
                        if let Ok(handle) = tokio::runtime::Handle::try_current() {
                            handle.spawn(async move {
                                let _ = sh_arc.dispatch_frame(user_id, frame).await;
                            });
                        }
                    }
                }
                if let Some(peer) = resolved_peer {
                    let frame = WsServerFrame::Update(UpdateEvent::CallUpdated {
                        call_id,
                        state: signal_type,
                        signal: Some(sig_req),
                    });
                    self.broadcast_to_user(peer, frame);
                }
            }
            WsClientFrame::AckUpdates { chat_id: _, pts } => {
                let _ = reply_tx.send(WsServerFrame::Ack {
                    correlation_id: 0,
                    client_nonce: 0,
                    message_id: 0,
                    pts,
                });
            }
        }
    }
}

/// Upgraded WebSocket connection handler with 30s ping / 60s inactivity timeout
pub async fn handle_ws_socket(
    socket: axum::extract::ws::WebSocket,
    user_id: UserId,
    hub: Arc<GatewayHub>,
    heartbeat_interval_secs: u64,
    inactivity_timeout_secs: u64,
) {
    use axum::extract::ws::Message;
    use futures::{SinkExt, StreamExt};
    use std::time::Duration;

    let (mut ws_sender, mut ws_receiver) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<WsServerFrame>();

    hub.register(user_id, tx.clone());
    hub.broadcast_user_status(user_id, true);
    if let Some(sh) = hub.signaling_hub() {
        let (sig_tx, mut sig_rx) = mpsc::channel::<SignalingFrame>(64);
        sh.register_client(user_id, sig_tx);
        let tx_sig = tx.clone();
        tokio::spawn(async move {
            while let Some(sf) = sig_rx.recv().await {
                let (call_id, state, signal) = match &sf {
                    SignalingFrame::Invite { call_id, .. } => (call_id.clone(), "ringing".into(), None),
                    SignalingFrame::Ringing { call_id } => (call_id.clone(), "ringing".into(), None),
                    SignalingFrame::Accept { call_id } => (call_id.clone(), "connected".into(), None),
                    SignalingFrame::Offer { call_id, sdp } => (
                        call_id.clone(),
                        "offer".into(),
                        Some(CallSignalRequest {
                            call_id: call_id.clone(), signal_type: "offer".into(),
                            sdp: Some(sdp.clone()), candidate: None, sdp_mid: None, sdp_mline_index: None, reason: None,
                        }),
                    ),
                    SignalingFrame::Answer { call_id, sdp } => (
                        call_id.clone(),
                        "answer".into(),
                        Some(CallSignalRequest {
                            call_id: call_id.clone(), signal_type: "answer".into(),
                            sdp: Some(sdp.clone()), candidate: None, sdp_mid: None, sdp_mline_index: None, reason: None,
                        }),
                    ),
                    SignalingFrame::IceCandidate { call_id, candidate, sdp_mid, sdp_mline_index } => (
                        call_id.clone(),
                        "ice_candidate".into(),
                        Some(CallSignalRequest {
                            call_id: call_id.clone(), signal_type: "ice_candidate".into(),
                            sdp: None, candidate: Some(candidate.clone()), sdp_mid: sdp_mid.clone(),
                            sdp_mline_index: sdp_mline_index.map(|v| v as u32), reason: None,
                        }),
                    ),
                    SignalingFrame::Hangup { call_id, reason } => (
                        call_id.clone(),
                        "ended".into(),
                        Some(CallSignalRequest {
                            call_id: call_id.clone(), signal_type: "hangup".into(),
                            sdp: None, candidate: None, sdp_mid: None, sdp_mline_index: None, reason: Some(reason.clone()),
                        }),
                    ),
                };
                if tx_sig.send(WsServerFrame::Update(UpdateEvent::CallUpdated { call_id, state, signal })).is_err() {
                    break;
                }
            }
        });
    }

    // Task forwarding outgoing frames to the WebSocket client
    let forward_task = tokio::spawn(async move {
        while let Some(frame) = rx.recv().await {
            match serde_json::to_string(&frame) {
                Ok(json) => {
                    if ws_sender.send(Message::Text(json.into())).await.is_err() {
                        break;
                    }
                }
                Err(e) => {
                    error!("Failed to serialize WsServerFrame: {e}");
                }
            }
        }
    });

    let tx_clone = tx.clone();
    let hub_clone = hub.clone();
    let mut last_activity = tokio::time::Instant::now();
    let inactivity_limit = Duration::from_secs(inactivity_timeout_secs);
    let ping_interval = Duration::from_secs(heartbeat_interval_secs);
    let mut heartbeat_timer = tokio::time::interval(ping_interval);

    loop {
        tokio::select! {
            _ = heartbeat_timer.tick() => {
                if last_activity.elapsed() >= inactivity_limit {
                    warn!("WebSocket client {} timed out after {}s inactivity", user_id, inactivity_timeout_secs);
                    break;
                }
            }
            msg = ws_receiver.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        last_activity = tokio::time::Instant::now();
                        match serde_json::from_str::<WsClientFrame>(&text) {
                            Ok(client_frame) => {
                                hub_clone.handle_client_frame(user_id, client_frame, &tx_clone);
                            }
                            Err(e) => {
                                let _ = tx_clone.send(WsServerFrame::Error {
                                    code: 400,
                                    message: format!("Malformed client frame: {e}"),
                                    details: None,
                                });
                            }
                        }
                    }
                    Some(Ok(Message::Ping(_payload))) => {
                        last_activity = tokio::time::Instant::now();
                    }
                    Some(Ok(Message::Pong(_))) => {
                        last_activity = tokio::time::Instant::now();
                    }
                    Some(Ok(Message::Close(_))) | None => {
                        break;
                    }
                    Some(Err(e)) => {
                        debug!("WebSocket error for {}: {e}", user_id);
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    forward_task.abort();
    hub.unregister_sender(user_id, &tx);
    if hub.user_connection_count(user_id) == 0 {
        hub.broadcast_user_status(user_id, false);
        if let Some(sh) = hub.signaling_hub() {
            sh.unregister_client(user_id);
        }
    }
}
