//! Optimistic reaction dispatch, nonce queueing, and HTTP/WS PTS reconciliation for `AppShell`.

use gpui::{AsyncApp, Context, WeakEntity};
use gpui_telegram_client::net::HttpClientError;
use gpui_telegram_core::types::{ChatId, MessageId};
use gpui_telegram_proto::message::ReactionCount;
use gpui_telegram_proto::ws::{ReactionRequest, WsClientFrame};

use crate::views::app_shell::shell_truth::SessionRequestFence;
use crate::views::shell::AppShell;
use crate::views::stage::cards::ReactionItem;

fn reaction_items_to_proto(items: &[ReactionItem]) -> Vec<ReactionCount> {
    items
        .iter()
        .map(|r| ReactionCount::new(&r.emoji, r.count, r.user_reacted))
        .collect()
}

fn fence_matches(fence: &SessionRequestFence, shell: &AppShell) -> bool {
    fence.matches(
        shell.is_authenticated,
        shell.session_sync.auth_generation,
        0,
        shell.session_sync.session_token.as_deref(),
        shell.session_sync.user_id,
    )
}

impl AppShell {
    /// Decrements the in-flight reaction counter for `(cid, server_mid)` and returns remaining count.
    pub fn decrement_in_flight_reaction(&mut self, cid: u64, server_mid: u64) -> usize {
        let key = (cid, server_mid);
        let next = self
            .session_sync
            .in_flight_reactions
            .get(&key)
            .copied()
            .unwrap_or(0)
            .saturating_sub(1);
        if next == 0 {
            self.session_sync.in_flight_reactions.remove(&key);
        } else {
            self.session_sync.in_flight_reactions.insert(key, next);
        }
        next
    }

    /// Reconciles an authoritative HTTP reaction response with local PTS watermarks and updates the stage view.
    pub fn apply_http_reaction_response(
        &mut self,
        cid: u64,
        mid: u64,
        server_mid: u64,
        pts: u64,
        mapped: Vec<ReactionItem>,
        cx: &mut Context<Self>,
    ) {
        self.apply_http_reaction_response_with_authority(
            cid,
            mid,
            server_mid,
            pts,
            Some(mapped),
            cx,
        );
    }

    /// Reconciles an HTTP reaction response with local PTS watermarks.
    ///
    /// When `authoritative_reactions` is `Some(snapshot)` (modern server returning `"reactions": [...]`,
    /// including `"reactions": []` when the last reaction is removed), replaces the stage & SQLite
    /// reaction snapshot. When `None` (legacy server ack `"status": "reaction_updated"` omitting
    /// `"reactions"`), preserves optimistic/stage reactions instead of wiping them to `vec![]` and
    /// persists the reconciled stage reaction snapshot into `LocalStore`.
    pub fn apply_http_reaction_response_with_authority(
        &mut self,
        cid: u64,
        mid: u64,
        server_mid: u64,
        pts: u64,
        authoritative_reactions: Option<Vec<ReactionItem>>,
        cx: &mut Context<Self>,
    ) {
        let remaining = self.decrement_in_flight_reaction(cid, server_mid);
        if pts > 0 {
            let cid_typed = ChatId(cid as i64);
            let cur_pts = self.client.sync_coordinator.last_pts(cid_typed);
            if cur_pts == 0 || cur_pts + 1 == pts {
                self.client.sync_coordinator.set_pts(cid_typed, pts);
                let drained = self.client.sync_coordinator.drain_contiguous_buffer(cid_typed);
                for buffered in drained {
                    self.apply_update_event_with_pts(
                        cid_typed,
                        Some(buffered.pts),
                        buffered.event,
                        cx,
                    );
                }
            }
        }

        let key = (cid, server_mid);
        let prev_pts = self
            .session_sync
            .reaction_pts_by_msg
            .get(&key)
            .copied()
            .unwrap_or(0);
        if pts > 0 {
            if pts >= prev_pts {
                self.session_sync.reaction_pts_by_msg.insert(key, pts);
                self.session_sync
                    .latest_http_reaction_by_msg
                    .insert(key, (pts, authoritative_reactions));
            }
        } else if prev_pts == 0 {
            self.session_sync
                .latest_http_reaction_by_msg
                .insert(key, (0, authoritative_reactions));
        }

        if remaining == 0 {
            if let Some((cand_pts, cand_reactions)) =
                self.session_sync.latest_http_reaction_by_msg.remove(&key)
            {
                let watermark = self
                    .session_sync
                    .reaction_pts_by_msg
                    .get(&key)
                    .copied()
                    .unwrap_or(0);
                if watermark == 0 || cand_pts >= watermark {
                    match cand_reactions {
                        Some(authoritative_list) => {
                            self.client.local_store.update_reactions(
                                ChatId(cid as i64),
                                MessageId(server_mid as i64),
                                reaction_items_to_proto(&authoritative_list),
                            );
                            self.stage.update(cx, |st, st_cx| {
                                st.set_message_reactions(cid, mid, authoritative_list.clone());
                                if server_mid != mid {
                                    st.set_message_reactions(cid, server_mid, authoritative_list);
                                }
                                st_cx.notify();
                            });
                            cx.notify();
                        }
                        None => {
                            let stage_snapshot: Option<Vec<ReactionCount>> = {
                                let st = self.stage.read(cx);
                                let from_active = (st.active_chat_id == Some(cid))
                                    .then(|| st.messages.iter().find(|m| m.id == server_mid || m.id == mid))
                                    .flatten();
                                from_active
                                    .or_else(|| {
                                        st.chat_histories
                                            .get(&cid)
                                            .and_then(|h| h.iter().find(|m| m.id == server_mid || m.id == mid))
                                    })
                                    .map(|m| reaction_items_to_proto(&m.reactions))
                            };
                            if let Some(proto_counts) = stage_snapshot {
                                self.client.local_store.update_reactions(
                                    ChatId(cid as i64),
                                    MessageId(server_mid as i64),
                                    proto_counts,
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    /// Dispatches a message reaction mutation, queuing optimistic nonces until reconciled.
    pub fn dispatch_reaction_mutation(
        &mut self,
        cid: u64,
        mid: u64,
        emoji: String,
        is_added: bool,
        cx: &mut Context<Self>,
    ) {
        if !self.is_authenticated {
            return;
        }
        let (Some(token), Some(uid)) = (
            self.session_sync
                .session_token
                .clone()
                .filter(|t| !t.trim().is_empty()),
            self.session_sync.user_id,
        ) else {
            return;
        };

        // If `mid` is an optimistic `Sending` client nonce whose `send_message_rich` has not yet
        // returned a `server_id`, queue the desired reaction state until `reconcile_outgoing_message`.
        if self.session_sync.pending_outgoing_nonces.contains_key(&mid) {
            let queue = self
                .session_sync
                .pending_nonce_reactions
                .entry(mid)
                .or_default();
            if let Some(existing) = queue.iter_mut().find(|(em, _)| *em == emoji) {
                existing.1 = is_added;
            } else {
                queue.push((emoji, is_added));
            }
            return;
        }

        if self.client.token().is_none() {
            self.client.set_token(Some(token.clone()));
        }
        let server_mid = self
            .session_sync
            .resolved_outgoing_nonces
            .get(&mid)
            .copied()
            .unwrap_or(mid);
        *self
            .session_sync
            .in_flight_reactions
            .entry((cid, server_mid))
            .or_insert(0) += 1;

        let fence = SessionRequestFence {
            auth_generation: self.session_sync.auth_generation,
            request_generation: 0,
            token,
            user_id: uid,
        };
        let c = self.client.clone();
        let emoji_for_task = emoji.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let action = if is_added { "add" } else { "remove" };
            let result = c
                .react_to_message(
                    cid as i64,
                    server_mid as i64,
                    emoji_for_task.clone(),
                    Some(action),
                )
                .await;
            match result {
                Ok(resp) => {
                    let resp_pts = resp.pts;
                    let has_authoritative = resp.has_authoritative_reactions();
                    let authoritative_reactions: Option<Vec<ReactionItem>> =
                        has_authoritative.then(|| {
                            resp.reactions
                                .into_iter()
                                .map(|r| ReactionItem {
                                    emoji: r.emoji,
                                    count: r.count,
                                    user_reacted: r.is_chosen_by_me,
                                })
                                .collect()
                        });
                    let _ = weak.update(app, |shell, cx| {
                        if !fence_matches(&fence, shell) {
                            return;
                        }
                        shell.apply_http_reaction_response_with_authority(
                            cid,
                            mid,
                            server_mid,
                            resp_pts,
                            authoritative_reactions,
                            cx,
                        );
                    });
                }
                Err(err) => {
                    let should_rollback = matches!(
                        &err,
                        HttpClientError::Unauthorized(_)
                            | HttpClientError::Api {
                                status: 400 | 401 | 403 | 404,
                                ..
                            }
                    );
                    let err_text = err.to_string();
                    if !should_rollback && c.is_connected() {
                        let _ = c.send_ws_frame(WsClientFrame::SendReaction(
                            ReactionRequest::new(
                                ChatId(cid as i64),
                                MessageId(server_mid as i64),
                                emoji_for_task.clone(),
                                is_added,
                            ),
                        ));
                    }
                    let _ = weak.update(app, |shell, cx| {
                        if !fence_matches(&fence, shell) {
                            return;
                        }
                        let remaining = shell.decrement_in_flight_reaction(cid, server_mid);
                        if should_rollback && remaining == 0 {
                            let mut rolled_back_snapshot: Option<Vec<ReactionCount>> = None;
                            shell.stage.update(cx, |st, st_cx| {
                                let target_id = if st.messages.iter().any(|m| m.id == server_mid) {
                                    server_mid
                                } else {
                                    mid
                                };
                                let matches_failed = st
                                    .messages
                                    .iter()
                                    .find(|m| m.id == target_id)
                                    .map(|m| {
                                        m.reactions
                                            .iter()
                                            .any(|r| r.emoji == emoji_for_task && r.user_reacted)
                                            == is_added
                                    })
                                    .unwrap_or(false);
                                if matches_failed {
                                    st.toggle_reaction(target_id, &emoji_for_task);
                                    rolled_back_snapshot = st
                                        .messages
                                        .iter()
                                        .find(|m| m.id == target_id)
                                        .map(|m| reaction_items_to_proto(&m.reactions));
                                    st_cx.notify();
                                }
                            });
                            if let Some(proto_counts) = rolled_back_snapshot {
                                shell.client.local_store.update_reactions(
                                    ChatId(cid as i64),
                                    MessageId(server_mid as i64),
                                    proto_counts,
                                );
                            }
                            shell.window_runtime.enqueue_toast(err_text, 2500);
                            cx.notify();
                        }
                    });
                }
            }
        })
        .detach();
    }
}
