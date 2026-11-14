use crate::cache::sqlite_cache::{CachedMediaEntry, SqliteClientCache};
use gpui_telegram_core::error::Result;
use gpui_telegram_core::types::{ChatId, MessageId};
use gpui_telegram_proto::contracts::{ChatDetailDto, DraftDto};
use gpui_telegram_proto::message::{MessageContent, MessageItem, ReactionCount};
use gpui_telegram_proto::{PollCard, TodoCard};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug, Default)]
struct LocalCacheState {
    messages: HashMap<ChatId, Vec<MessageItem>>,
    chats: HashMap<ChatId, ChatDetailDto>,
    drafts: HashMap<ChatId, DraftDto>,
    pinned_ids: HashMap<ChatId, Vec<MessageId>>,
    read_inbox_max: HashMap<ChatId, MessageId>,
    read_outbox_max: HashMap<ChatId, MessageId>,
    media_entries: HashMap<String, CachedMediaEntry>,
    mem_media_bytes: HashMap<String, Vec<u8>>,
}

#[derive(Debug, Default, Clone)]
pub struct LocalStore {
    state: Arc<RwLock<LocalCacheState>>,
    sqlite: Option<Arc<SqliteClientCache>>,
}

impl LocalStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open_with_dir(base_dir: impl AsRef<Path>) -> Result<Self> {
        let db = Arc::new(SqliteClientCache::open(base_dir.as_ref())?);
        let messages = db.load_messages()?;
        let chats = db.load_chats()?;
        let drafts = db.load_drafts()?;
        let (read_inbox_max, read_outbox_max) = db.load_read_states()?;
        let media_entries = db.load_media_index()?;
        let state = LocalCacheState {
            messages,
            chats,
            drafts,
            pinned_ids: HashMap::new(),
            read_inbox_max,
            read_outbox_max,
            media_entries,
            mem_media_bytes: HashMap::new(),
        };
        Ok(Self {
            state: Arc::new(RwLock::new(state)),
            sqlite: Some(db),
        })
    }

    pub fn insert_message(&self, message: MessageItem) {
        let mut stale_ids = Vec::new();
        {
            let mut guard = self.state.write();
            let list = guard.messages.entry(message.chat_id).or_default();
            list.retain(|existing| {
                let id_match = existing.id == message.id;
                let nonce_match = message.client_nonce.is_some()
                    && existing.client_nonce == message.client_nonce
                    && existing.id != message.id;
                if nonce_match {
                    stale_ids.push(existing.id);
                }
                !id_match && !nonce_match
            });
            list.push(message.clone());
            list.sort_by_key(|m| (m.date.timestamp_millis(), m.id.0));
        }
        if let Some(db) = &self.sqlite {
            for old_id in stale_ids {
                db.delete_message(message.chat_id, old_id);
            }
            db.upsert_message(&message);
        }
    }

    pub fn upsert_messages(&self, items: Vec<MessageItem>) {
        for item in items {
            self.insert_message(item);
        }
    }

    pub fn ack_client_nonce(&self, client_nonce: u64, assigned_id: MessageId) {
        if client_nonce == 0 || assigned_id.0 <= 0 {
            return;
        }
        let mut updated: Option<(ChatId, MessageId, MessageItem)> = None;
        {
            let mut guard = self.state.write();
            for (&chat_id, list) in guard.messages.iter_mut() {
                if let Some(item) = list.iter_mut().find(|m| m.client_nonce == Some(client_nonce)) {
                    let old_id = item.id;
                    item.id = assigned_id;
                    let clone = item.clone();
                    list.sort_by_key(|m| (m.date.timestamp_millis(), m.id.0));
                    updated = Some((chat_id, old_id, clone));
                    break;
                }
            }
        }
        if let (Some(db), Some((chat_id, old_id, msg))) = (&self.sqlite, updated) {
            if old_id != msg.id {
                db.delete_message(chat_id, old_id);
            }
            db.upsert_message(&msg);
        }
    }

    pub fn edit_message(&self, edited: MessageItem) {
        self.insert_message(edited);
    }

    pub fn delete_messages(&self, chat_id: ChatId, message_ids: &[MessageId]) {
        if message_ids.is_empty() {
            return;
        }
        {
            let mut guard = self.state.write();
            if let Some(list) = guard.messages.get_mut(&chat_id) {
                list.retain(|m| !message_ids.contains(&m.id));
            }
        }
        if let Some(db) = &self.sqlite {
            for &id in message_ids {
                db.delete_message(chat_id, id);
            }
        }
    }

    pub fn update_reactions(&self, chat_id: ChatId, message_id: MessageId, reactions: Vec<ReactionCount>) {
        let updated = {
            let mut guard = self.state.write();
            guard.messages.get_mut(&chat_id).and_then(|list| {
                list.iter_mut().find(|m| m.id == message_id).map(|m| {
                    m.reactions = reactions;
                    m.clone()
                })
            })
        };
        if let (Some(db), Some(msg)) = (&self.sqlite, updated) {
            db.upsert_message(&msg);
        }
    }

    pub fn update_poll(&self, chat_id: ChatId, message_id: MessageId, poll: PollCard) {
        let updated = {
            let mut guard = self.state.write();
            guard.messages.get_mut(&chat_id).and_then(|list| {
                list.iter_mut().find(|m| m.id == message_id).map(|m| {
                    m.content = MessageContent::Poll(poll);
                    m.clone()
                })
            })
        };
        if let (Some(db), Some(msg)) = (&self.sqlite, updated) {
            db.upsert_message(&msg);
        }
    }

    pub fn update_todo(&self, chat_id: ChatId, message_id: MessageId, todo: TodoCard) {
        let updated = {
            let mut guard = self.state.write();
            guard.messages.get_mut(&chat_id).and_then(|list| {
                list.iter_mut().find(|m| m.id == message_id).map(|m| {
                    m.content = MessageContent::Todo(todo);
                    m.clone()
                })
            })
        };
        if let (Some(db), Some(msg)) = (&self.sqlite, updated) {
            db.upsert_message(&msg);
        }
    }

    pub fn get_messages(&self, chat_id: ChatId) -> Result<Vec<MessageItem>> {
        Ok(self.state.read().messages.get(&chat_id).cloned().unwrap_or_default())
    }

    pub fn upsert_chat(&self, chat: ChatDetailDto) {
        let cid = ChatId(chat.chat_id);
        self.state.write().chats.insert(cid, chat.clone());
        if let Some(db) = &self.sqlite {
            db.upsert_chat(&chat);
        }
    }

    pub fn list_chats(&self) -> Vec<ChatDetailDto> {
        self.state.read().chats.values().cloned().collect()
    }

    pub fn get_chat(&self, chat_id: ChatId) -> Option<ChatDetailDto> {
        self.state.read().chats.get(&chat_id).cloned()
    }

    pub fn set_draft(&self, chat_id: ChatId, draft: Option<DraftDto>) {
        {
            let mut guard = self.state.write();
            if let Some(ref d) = draft {
                guard.drafts.insert(chat_id, d.clone());
            } else {
                guard.drafts.remove(&chat_id);
            }
        }
        if let Some(db) = &self.sqlite {
            db.set_draft(chat_id, draft.as_ref());
        }
    }

    pub fn get_draft(&self, chat_id: ChatId) -> Option<DraftDto> {
        self.state.read().drafts.get(&chat_id).cloned()
    }

    pub fn set_pinned_messages(&self, chat_id: ChatId, pinned_ids: Vec<MessageId>) {
        self.state.write().pinned_ids.insert(chat_id, pinned_ids);
    }

    pub fn get_pinned_messages(&self, chat_id: ChatId) -> Vec<MessageId> {
        self.state.read().pinned_ids.get(&chat_id).cloned().unwrap_or_default()
    }

    pub fn mark_read_inbox(&self, chat_id: ChatId, max_id: MessageId, unread_count: Option<u32>) {
        let (inbox, outbox, chat_opt) = {
            let mut guard = self.state.write();
            let entry = guard.read_inbox_max.entry(chat_id).or_insert(max_id);
            if max_id.0 > entry.0 {
                *entry = max_id;
            }
            let inbox = entry.0;
            let outbox = guard.read_outbox_max.get(&chat_id).map(|m| m.0).unwrap_or(0);
            let chat_opt = guard.chats.get_mut(&chat_id).map(|c| {
                c.read_inbox_max_id = c.read_inbox_max_id.max(max_id.0);
                if let Some(u) = unread_count {
                    c.unread_count = u;
                }
                c.clone()
            });
            (inbox, outbox, chat_opt)
        };
        if let Some(db) = &self.sqlite {
            db.upsert_read_state(chat_id, inbox, outbox);
            if let Some(c) = chat_opt {
                db.upsert_chat(&c);
            }
        }
    }

    pub fn mark_read_outbox(&self, chat_id: ChatId, max_id: MessageId) {
        let (inbox, outbox, chat_opt) = {
            let mut guard = self.state.write();
            let entry = guard.read_outbox_max.entry(chat_id).or_insert(max_id);
            if max_id.0 > entry.0 {
                *entry = max_id;
            }
            let outbox = entry.0;
            let inbox = guard.read_inbox_max.get(&chat_id).map(|m| m.0).unwrap_or(0);
            let chat_opt = guard.chats.get_mut(&chat_id).map(|c| {
                c.read_outbox_max_id = c.read_outbox_max_id.max(max_id.0);
                c.clone()
            });
            (inbox, outbox, chat_opt)
        };
        if let Some(db) = &self.sqlite {
            db.upsert_read_state(chat_id, inbox, outbox);
            if let Some(c) = chat_opt {
                db.upsert_chat(&c);
            }
        }
    }

    pub fn read_inbox_max_id(&self, chat_id: ChatId) -> Option<MessageId> {
        self.state.read().read_inbox_max.get(&chat_id).copied()
    }

    pub fn read_outbox_max_id(&self, chat_id: ChatId) -> Option<MessageId> {
        self.state.read().read_outbox_max.get(&chat_id).copied()
    }

    pub fn media_cas_path(&self, file_id: &str) -> Option<PathBuf> {
        self.state
            .read()
            .media_entries
            .get(file_id)
            .map(|e| e.local_path.clone())
            .filter(|p| p.exists())
    }

    pub fn put_media_cas(&self, file_id: &str, bytes: &[u8], mime_type: Option<&str>) -> Result<Option<PathBuf>> {
        if let Some(db) = &self.sqlite {
            let entry = db.write_media_cas(file_id, bytes, mime_type)?;
            let path = entry.local_path.clone();
            self.state.write().media_entries.insert(file_id.to_string(), entry);
            return Ok(Some(path));
        }
        self.state
            .write()
            .mem_media_bytes
            .insert(file_id.to_string(), bytes.to_vec());
        Ok(None)
    }

    pub fn get_media_cas(&self, file_id: &str) -> Option<Vec<u8>> {
        if let Some(path) = self.media_cas_path(file_id) {
            if let Ok(bytes) = std::fs::read(&path) {
                return Some(bytes);
            }
        }
        self.state.read().mem_media_bytes.get(file_id).cloned()
    }
}
