use crate::cache::LocalStore;
use crate::sync::{ClientSyncCoordinator, SyncOutcome};
use gpui_telegram_core::types::{ChatId, MessageId, PeerId, UserId, UserStatus};
use gpui_telegram_proto::contracts::{
    CallSignalRequest, ChatFolderDto, DraftDto, ForumTopicDto, StoryDto,
};
use gpui_telegram_proto::message::{MessageItem, ReactionCount};
use gpui_telegram_proto::sync::{ChatActionEvent, TypingEvent, UpdateEvent};
use gpui_telegram_proto::ws::WsServerFrame;
use gpui_telegram_proto::{PollCard, TodoCard};

#[derive(Debug, Clone)]
pub enum ClientDomainEvent {
    NewMessage(MessageItem),
    EditMessage(MessageItem),
    DeleteMessages {
        chat_id: ChatId,
        message_ids: Vec<MessageId>,
        for_everyone: bool,
    },
    ReactionsUpdated {
        chat_id: ChatId,
        message_id: MessageId,
        user_id: UserId,
        reactions: Vec<ReactionCount>,
    },
    PollUpdated {
        chat_id: ChatId,
        message_id: MessageId,
        poll: PollCard,
    },
    TodoUpdated {
        chat_id: ChatId,
        message_id: MessageId,
        todo: TodoCard,
    },
    ReadHistory {
        chat_id: ChatId,
        max_id: MessageId,
        is_outbox: bool,
        unread_count: Option<u32>,
    },
    Typing(TypingEvent),
    UserStatusChanged {
        user_id: UserId,
        status: UserStatus,
    },
    PinnedMessagesChanged {
        chat_id: ChatId,
        pinned_ids: Vec<MessageId>,
    },
    DraftUpdated {
        chat_id: ChatId,
        topic_id: Option<i64>,
        draft: Option<DraftDto>,
    },
    ForumTopicChanged {
        chat_id: ChatId,
        topic: ForumTopicDto,
    },
    FolderChanged {
        folder: ChatFolderDto,
    },
    CallUpdated {
        call_id: String,
        state: String,
        signal: Option<CallSignalRequest>,
    },
    StoryUpdated {
        story: StoryDto,
    },
    ChatAction {
        chat_id: Option<ChatId>,
        action: ChatActionEvent,
    },
    MessageAck {
        correlation_id: u64,
        client_nonce: u64,
        message_id: MessageId,
        pts: u64,
    },
    SyncGapDetected {
        chat_id: ChatId,
        from_pts: u64,
    },
    ServerError {
        code: u16,
        message: String,
        details: Option<String>,
    },
    Pong {
        nonce: u64,
    },
}

pub(crate) fn peer_id_to_chat_id(peer: PeerId) -> ChatId {
    match peer {
        PeerId::Chat(c) => c,
        PeerId::Channel(ch) => ChatId(ch.0 as i64),
        PeerId::User(u) => ChatId(u.0 as i64),
        PeerId::Fake(f) => ChatId(f as i64),
    }
}

pub(crate) fn apply_update_event(
    store: &LocalStore,
    fallback_chat_id: Option<ChatId>,
    event: UpdateEvent,
    out_events: &mut Vec<ClientDomainEvent>,
) {
    match event {
        UpdateEvent::NewMessage(msg) => {
            store.insert_message(msg.clone());
            out_events.push(ClientDomainEvent::NewMessage(msg));
        }
        UpdateEvent::EditMessage(msg) => {
            store.edit_message(msg.clone());
            out_events.push(ClientDomainEvent::EditMessage(msg));
        }
        UpdateEvent::DeleteMessages(tomb) => {
            let cid = fallback_chat_id.unwrap_or_else(|| peer_id_to_chat_id(tomb.chat_id));
            store.delete_messages(cid, &tomb.message_ids);
            out_events.push(ClientDomainEvent::DeleteMessages {
                chat_id: cid,
                message_ids: tomb.message_ids,
                for_everyone: tomb.for_everyone,
            });
        }
        UpdateEvent::UserTyping(ev) => out_events.push(ClientDomainEvent::Typing(ev)),
        UpdateEvent::ChatAction(action) => out_events.push(ClientDomainEvent::ChatAction {
            chat_id: fallback_chat_id,
            action,
        }),
        UpdateEvent::ReadHistory(ev) => {
            if ev.is_outbox {
                store.mark_read_outbox(ev.chat_id, ev.max_id);
            } else {
                store.mark_read_inbox(ev.chat_id, ev.max_id, Some(ev.unread_count));
            }
            out_events.push(ClientDomainEvent::ReadHistory {
                chat_id: ev.chat_id,
                max_id: ev.max_id,
                is_outbox: ev.is_outbox,
                unread_count: Some(ev.unread_count),
            });
        }
        UpdateEvent::ReadHistoryIn { max_id } => {
            if let Some(cid) = fallback_chat_id {
                store.mark_read_inbox(cid, max_id, Some(0));
                out_events.push(ClientDomainEvent::ReadHistory {
                    chat_id: cid,
                    max_id,
                    is_outbox: false,
                    unread_count: Some(0),
                });
            }
        }
        UpdateEvent::ReadHistoryOut { max_id } => {
            if let Some(cid) = fallback_chat_id {
                store.mark_read_outbox(cid, max_id);
                out_events.push(ClientDomainEvent::ReadHistory {
                    chat_id: cid,
                    max_id,
                    is_outbox: true,
                    unread_count: None,
                });
            }
        }
        UpdateEvent::MessageReactions(ev) | UpdateEvent::ReactionUpdated(ev) => {
            store.update_reactions(ev.chat_id, ev.message_id, ev.reactions.clone());
            out_events.push(ClientDomainEvent::ReactionsUpdated {
                chat_id: ev.chat_id,
                message_id: ev.message_id,
                user_id: ev.user_id,
                reactions: ev.reactions,
            });
        }
        UpdateEvent::UserStatusChanged { user_id, status } => {
            out_events.push(ClientDomainEvent::UserStatusChanged { user_id, status });
        }
        UpdateEvent::PinnedMessageChanged { message_id } => {
            if let Some(cid) = fallback_chat_id {
                let ids = message_id.into_iter().collect::<Vec<_>>();
                store.set_pinned_messages(cid, ids.clone());
                out_events.push(ClientDomainEvent::PinnedMessagesChanged {
                    chat_id: cid,
                    pinned_ids: ids,
                });
            }
        }
        UpdateEvent::PinnedMessagesChanged { chat_id, pinned_ids } => {
            store.set_pinned_messages(chat_id, pinned_ids.clone());
            out_events.push(ClientDomainEvent::PinnedMessagesChanged { chat_id, pinned_ids });
        }
        UpdateEvent::PollUpdated { chat_id, message_id, poll } => {
            store.update_poll(chat_id, message_id, poll.clone());
            out_events.push(ClientDomainEvent::PollUpdated { chat_id, message_id, poll });
        }
        UpdateEvent::TodoUpdated { chat_id, message_id, todo } => {
            store.update_todo(chat_id, message_id, todo.clone());
            out_events.push(ClientDomainEvent::TodoUpdated { chat_id, message_id, todo });
        }
        UpdateEvent::DraftUpdated { chat_id, topic_id, draft } => {
            store.set_draft(chat_id, draft.clone());
            out_events.push(ClientDomainEvent::DraftUpdated { chat_id, topic_id, draft });
        }
        UpdateEvent::ForumTopicChanged { chat_id, topic } => {
            out_events.push(ClientDomainEvent::ForumTopicChanged { chat_id, topic });
        }
        UpdateEvent::FolderChanged { folder } => {
            out_events.push(ClientDomainEvent::FolderChanged { folder });
        }
        UpdateEvent::CallUpdated { call_id, state, signal } => {
            out_events.push(ClientDomainEvent::CallUpdated { call_id, state, signal });
        }
        UpdateEvent::StoryUpdated { story } => {
            out_events.push(ClientDomainEvent::StoryUpdated { story });
        }
    }
}

pub(crate) fn apply_ws_frame(
    store: &LocalStore,
    sync: &ClientSyncCoordinator,
    frame: WsServerFrame,
    out_events: &mut Vec<ClientDomainEvent>,
) -> Vec<ChatId> {
    let mut gaps = Vec::new();
    match frame {
        WsServerFrame::Pong { nonce } => out_events.push(ClientDomainEvent::Pong { nonce }),
        WsServerFrame::Update(ev) => apply_update_event(store, None, ev, out_events),
        WsServerFrame::PtsPush(update) => match sync.evaluate_update(&update) {
            SyncOutcome::Contiguous => {
                let cid = update.chat_id;
                apply_update_event(store, Some(cid), update.event, out_events);
                for buffered in sync.drain_contiguous_buffer(cid) {
                    let bcid = buffered.chat_id;
                    apply_update_event(store, Some(bcid), buffered.event, out_events);
                }
            }
            SyncOutcome::Duplicate => {}
            SyncOutcome::GapDetected { last_seen, .. } => {
                let cid = update.chat_id;
                sync.buffer_gap_update(update);
                out_events.push(ClientDomainEvent::SyncGapDetected {
                    chat_id: cid,
                    from_pts: last_seen,
                });
                gaps.push(cid);
            }
        },
        WsServerFrame::SyncDiff(diff) => {
            store.upsert_messages(diff.new_messages);
            for ev in diff.other_updates {
                apply_update_event(store, Some(diff.chat_id), ev, out_events);
            }
            for pts_up in diff.updates {
                let cid = pts_up.chat_id;
                let _ = sync.apply_update(pts_up.clone());
                apply_update_event(store, Some(cid), pts_up.event, out_events);
            }
            let target_pts = diff.pts.max(diff.current_pts);
            if target_pts > sync.last_pts(diff.chat_id) {
                sync.set_pts(diff.chat_id, target_pts);
            }
            for buffered in sync.drain_contiguous_buffer(diff.chat_id) {
                let bcid = buffered.chat_id;
                apply_update_event(store, Some(bcid), buffered.event, out_events);
            }
        }
        WsServerFrame::Ack { correlation_id, client_nonce, message_id, pts } => {
            let msg_id = MessageId(message_id);
            store.ack_client_nonce(client_nonce, msg_id);
            out_events.push(ClientDomainEvent::MessageAck {
                correlation_id,
                client_nonce,
                message_id: msg_id,
                pts,
            });
        }
        WsServerFrame::Error { code, message, details } => {
            out_events.push(ClientDomainEvent::ServerError { code, message, details });
        }
    }
    gaps
}
