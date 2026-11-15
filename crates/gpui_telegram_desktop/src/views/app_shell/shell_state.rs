//! Grouped domain state structs and pure reconciliation helpers for `AppShell`.
//!
//! Splits session/sync tracking, Mini App registry state, WebRTC/AI talk state,
//! floating overlay handles, and theme/locale visual state into cohesive sub-structs
//! so `AppShell` stays concise and modular.

use std::collections::HashMap;

use gpui::Entity;
use gpui_telegram_agent::TalkCoordinator;
use gpui_telegram_client::net::hermes_client::HermesRichChatDto;
use gpui_telegram_media::{AudioPlaybackEngine, VoiceRecorder};
use gpui_telegram_proto::computer_contracts::{
    BotComputerProfileDto, ComputerNodeDto, PendingComputerCommandDto,
};
use gpui_telegram_proto::contracts::auth::ContactDto;
use gpui_telegram_proto::miniapp_contracts::{BotRegistryEntry, MiniAppEntry};
use gpui_telegram_webrtc::WebRtcCallEngine;

use crate::components::bots::bot_attach_web_view::BotAttachWebView;
use crate::theme::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::boxes::peers::create_managed_bot_box::CreateManagedBotBoxPeers;
use crate::views::iv::IvViewer;
use crate::views::stage::cards::{DeliveryStatus, MessageItemModel, ReactionItem};
use crate::views::{CallOverlay, ComputerWorkbench, MediaViewer};

/// Session identity, generation fencing counters, contact/chat caches, and optimistic send/reaction maps.
pub struct ShellSessionSyncState {
    pub session_token: Option<String>,
    pub session_id: Option<String>,
    pub user_id: Option<u64>,
    pub username: Option<String>,
    pub ws_stream_started: bool,
    pub auth_generation: u64,
    pub chat_list_request_generation: u64,
    pub history_request_generation: u64,
    pub last_dispatched_search_seq: u64,
    pub next_message_id: u64,
    pub contacts_list: Vec<ContactDto>,
    pub rich_chats_by_id: HashMap<u64, HermesRichChatDto>,
    pub chat_peer_user_ids: HashMap<u64, u64>,
    pub pending_outgoing_nonces: HashMap<u64, u64>,
    pub resolved_outgoing_nonces: HashMap<u64, u64>,
    pub pending_nonce_reactions: HashMap<u64, Vec<(String, bool)>>,
    pub in_flight_reactions: HashMap<(u64, u64), usize>,
    pub reaction_pts_by_msg: HashMap<(u64, u64), u64>,
    pub latest_http_reaction_by_msg: HashMap<(u64, u64), (u64, Option<Vec<ReactionItem>>)>,
}

impl Default for ShellSessionSyncState {
    fn default() -> Self {
        Self::new()
    }
}

impl ShellSessionSyncState {
    /// Creates an unauthenticated session/sync state with `next_message_id` starting at `900`.
    pub fn new() -> Self {
        Self {
            session_token: None,
            session_id: None,
            user_id: None,
            username: None,
            ws_stream_started: false,
            auth_generation: 0,
            chat_list_request_generation: 0,
            history_request_generation: 0,
            last_dispatched_search_seq: 0,
            next_message_id: 900,
            contacts_list: Vec::new(),
            rich_chats_by_id: HashMap::new(),
            chat_peer_user_ids: HashMap::new(),
            pending_outgoing_nonces: HashMap::new(),
            resolved_outgoing_nonces: HashMap::new(),
            pending_nonce_reactions: HashMap::new(),
            in_flight_reactions: HashMap::new(),
            reaction_pts_by_msg: HashMap::new(),
            latest_http_reaction_by_msg: HashMap::new(),
        }
    }

    /// Advances auth, chat-list, and history request generation fences to invalidate stale async responses.
    pub fn bump_generations(&mut self) {
        self.auth_generation = self.auth_generation.wrapping_add(1);
        self.chat_list_request_generation = self.chat_list_request_generation.wrapping_add(1);
        self.history_request_generation = self.history_request_generation.wrapping_add(1);
    }

    /// Binds an authenticated session identity and advances request generation fences.
    pub fn begin_session(
        &mut self,
        token: String,
        session_id: String,
        user_id: u64,
        username: String,
    ) {
        self.bump_generations();
        self.session_token = Some(token);
        self.session_id = Some(session_id);
        self.user_id = Some(user_id);
        self.username = Some(username);
    }

    /// Clears per-message optimistic nonce and reaction tracking maps.
    pub fn clear_message_tracking(&mut self) {
        self.pending_outgoing_nonces.clear();
        self.resolved_outgoing_nonces.clear();
        self.pending_nonce_reactions.clear();
        self.in_flight_reactions.clear();
        self.reaction_pts_by_msg.clear();
        self.latest_http_reaction_by_msg.clear();
    }

    /// Resets account-bound session identity, cached peer maps, and optimistic tracking on logout.
    pub fn reset_on_logout(&mut self) {
        self.bump_generations();
        self.session_token = None;
        self.session_id = None;
        self.user_id = None;
        self.username = None;
        self.ws_stream_started = false;
        self.last_dispatched_search_seq = 0;
        self.contacts_list.clear();
        self.rich_chats_by_id.clear();
        self.chat_peer_user_ids.clear();
        self.clear_message_tracking();
    }
}

/// Active Mini App WebView sheets, NL bot creator modal, and catalog/bot registry caches.
#[derive(Default)]
pub struct ShellMiniAppState {
    pub miniapp_overlay: Option<Entity<BotAttachWebView>>,
    pub is_miniapp_minimized: bool,
    pub create_bot_box: Option<Entity<CreateManagedBotBoxPeers>>,
    pub catalog_miniapps: Vec<MiniAppEntry>,
    pub installed_miniapps: Vec<MiniAppEntry>,
    pub attach_menu_bots: Vec<BotRegistryEntry>,
    pub my_bots: Vec<BotRegistryEntry>,
}

impl ShellMiniAppState {
    /// Creates an empty Mini App and bot registry state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Clears active Mini App overlays and account-scoped bot/app registries.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// Voice/video call engines, active call state, and AI Talk Mode session coordination.
pub struct ShellRtcTalkState {
    pub talk_mode_overlay: Option<Entity<CallOverlay>>,
    pub audio_player: AudioPlaybackEngine,
    pub voice_recorder: VoiceRecorder,
    pub webrtc_engine: WebRtcCallEngine,
    pub active_call_id: Option<String>,
    pub talk_coordinator: TalkCoordinator,
    pub active_talk_session_is_ai: bool,
    pub active_talk_bot_chat_id: Option<u64>,
    pub active_talk_session_id: Option<String>,
}

impl Default for ShellRtcTalkState {
    fn default() -> Self {
        Self::new()
    }
}

impl ShellRtcTalkState {
    /// Initializes audio playback, 16kHz voice recorder, WebRTC engine, and Talk Mode coordinator.
    pub fn new() -> Self {
        Self {
            talk_mode_overlay: None,
            audio_player: AudioPlaybackEngine::new(),
            voice_recorder: VoiceRecorder::new(16_000),
            webrtc_engine: WebRtcCallEngine::default(),
            active_call_id: None,
            talk_coordinator: TalkCoordinator::new(),
            active_talk_session_is_ai: false,
            active_talk_bot_chat_id: None,
            active_talk_session_id: None,
        }
    }

    /// Tears down active WebRTC/AI talk sessions, stops media capture/playback, and closes the call overlay.
    pub fn reset_on_logout(&mut self) {
        self.active_call_id = None;
        self.webrtc_engine.hangup_local("logout");
        let _ = self.webrtc_engine.drain_outbound_frames();
        let _ = self.voice_recorder.stop();
        self.audio_player.stop();
        self.talk_mode_overlay = None;
        self.active_talk_session_is_ai = false;
        self.active_talk_bot_chat_id = None;
        self.active_talk_session_id = None;
    }
}

/// Floating fullscreen/lightbox overlays, passcode lock state, and sidebar/viewport revision watermarks.
pub struct ShellOverlayState {
    pub computer_overlay: Option<Entity<ComputerWorkbench>>,
    pub media_viewer_overlay: Option<Entity<MediaViewer>>,
    pub iv_viewer_overlay: Option<Entity<IvViewer>>,
    pub is_passcode_locked: bool,
    pub passcode_unlock_input: String,
    pub passcode_unlock_error: Option<String>,
    pub last_user_activity: std::time::Instant,
    pub mic_test_loop_gen: u64,
    pub last_viewport_width: f32,
    pub last_sidebar_selection_revision: u64,
    pub last_sidebar_collapse_revision: u64,
}

impl Default for ShellOverlayState {
    fn default() -> Self {
        Self::new()
    }
}

impl ShellOverlayState {
    /// Creates overlay state with an initial `1280.0` viewport width watermark.
    pub fn new() -> Self {
        Self {
            computer_overlay: None,
            media_viewer_overlay: None,
            iv_viewer_overlay: None,
            is_passcode_locked: false,
            passcode_unlock_input: String::new(),
            passcode_unlock_error: None,
            last_user_activity: std::time::Instant::now(),
            mic_test_loop_gen: 0,
            last_viewport_width: 1280.0,
            last_sidebar_selection_revision: 0,
            last_sidebar_collapse_revision: 0,
        }
    }

    /// Dismisses remote computer, media lightbox, instant-view, and passcode lock overlays.
    pub fn clear_overlays(&mut self) {
        self.computer_overlay = None;
        self.media_viewer_overlay = None;
        self.iv_viewer_overlay = None;
        self.is_passcode_locked = false;
        self.passcode_unlock_input.clear();
        self.passcode_unlock_error = None;
        self.mic_test_loop_gen = self.mic_test_loop_gen.wrapping_add(1);
    }
}

/// Resolved desktop theme palette, accent/wallpaper overrides, bubble radius, and active locale flag.
#[derive(Debug, Clone)]
pub struct ShellThemeVisualState {
    pub palette: TdesktopThemePalette,
    pub theme_mode: TdesktopThemeMode,
    pub accent_color: Option<u32>,
    pub bubble_radius_px: u32,
    pub wallpaper_id: Option<String>,
    pub is_chinese_locale: bool,
}

impl Default for ShellThemeVisualState {
    fn default() -> Self {
        Self::new(
            TdesktopThemePalette::default(),
            TdesktopThemeMode::Night,
            false,
        )
    }
}

impl ShellThemeVisualState {
    /// Creates theme visual state with default `12px` bubble radius and no explicit accent/wallpaper overrides.
    pub fn new(
        palette: TdesktopThemePalette,
        theme_mode: TdesktopThemeMode,
        is_chinese_locale: bool,
    ) -> Self {
        Self {
            palette,
            theme_mode,
            accent_color: None,
            bubble_radius_px: 12,
            wallpaper_id: None,
            is_chinese_locale,
        }
    }

    /// Returns `"zh-CN"` when Chinese locale is active, otherwise `"en"`.
    pub fn active_lang_id(&self) -> &'static str {
        if self.is_chinese_locale {
            "zh-CN"
        } else {
            "en"
        }
    }
}

/// Multi-device & cloud computer registry, Bot computer profiles, and pending approval queue.
#[derive(Debug, Clone, Default)]
pub struct ShellComputersState {
    pub computers: Vec<ComputerNodeDto>,
    pub bot_profiles: HashMap<u64, BotComputerProfileDto>,
    pub pending_approvals: Vec<PendingComputerCommandDto>,
    pub current_host_node_id: Option<String>,
    pub relay_loop_started: bool,
    pub relay_generation: u64,
}

impl ShellComputersState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset_on_logout(&mut self) {
        self.computers.clear();
        self.bot_profiles.clear();
        self.pending_approvals.clear();
        self.current_host_node_id = None;
        self.relay_loop_started = false;
        self.relay_generation = self.relay_generation.wrapping_add(1);
    }
}

/// Returns `true` when `sidebar.selection_revision` advances with a selected dialog,
/// preventing hover or background notifications at an unchanged revision from switching panels.
pub fn should_switch_to_chats_on_sidebar_selection(
    current_revision: u64,
    last_seen_revision: &mut u64,
    has_selected_dialog: bool,
) -> bool {
    if current_revision == *last_seen_revision {
        return false;
    }
    *last_seen_revision = current_revision;
    has_selected_dialog
}

fn delivery_status_rank(status: DeliveryStatus) -> u8 {
    match status {
        DeliveryStatus::Sending => 0,
        DeliveryStatus::Sent => 1,
        DeliveryStatus::Read => 2,
    }
}

fn max_delivery_status(a: DeliveryStatus, b: DeliveryStatus) -> DeliveryStatus {
    if delivery_status_rank(a) >= delivery_status_rank(b) {
        a
    } else {
        b
    }
}

/// Promotes an optimistic message (`local_id`) to authoritative `server_id` in place,
/// preserving the highest delivery status and collapsing any duplicate with `server_id`.
pub fn reconcile_stage_message_list(
    list: &mut Vec<MessageItemModel>,
    local_id: u64,
    server_id: u64,
    min_status: DeliveryStatus,
) -> bool {
    let existing_server_idx = (server_id > 0)
        .then(|| list.iter().position(|m| m.id == server_id))
        .flatten();
    let local_idx = list.iter().position(|m| m.id == local_id);

    match (local_idx, existing_server_idx) {
        (Some(l_idx), Some(s_idx)) if l_idx != s_idx => {
            let local_status = list[l_idx].delivery_status;
            let local_reactions = list[l_idx].reactions.clone();
            list[s_idx].delivery_status = max_delivery_status(
                max_delivery_status(list[s_idx].delivery_status, local_status),
                min_status,
            );
            if list[s_idx].reactions.is_empty() && !local_reactions.is_empty() {
                list[s_idx].reactions = local_reactions;
            }
            list.remove(l_idx);
            true
        }
        (Some(l_idx), _) => {
            if server_id > 0 {
                list[l_idx].id = server_id;
            }
            list[l_idx].delivery_status =
                max_delivery_status(list[l_idx].delivery_status, min_status);
            true
        }
        (None, Some(s_idx)) => {
            list[s_idx].delivery_status =
                max_delivery_status(list[s_idx].delivery_status, min_status);
            true
        }
        (None, None) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_state_defaults_and_resets() {
        let mut sync = ShellSessionSyncState::default();
        assert_eq!(sync.next_message_id, 900);
        sync.user_id = Some(42);
        sync.session_token = Some("tok".to_string());
        sync.pending_outgoing_nonces.insert(1, 2);
        sync.reset_on_logout();
        assert_eq!(sync.auth_generation, 1);
        assert_eq!(sync.user_id, None);
        assert!(sync.session_token.is_none());
        assert!(sync.pending_outgoing_nonces.is_empty());

        let mut miniapps = ShellMiniAppState::new();
        miniapps.is_miniapp_minimized = true;
        miniapps.reset();
        assert!(!miniapps.is_miniapp_minimized);

        let mut overlays = ShellOverlayState::default();
        assert_eq!(overlays.last_viewport_width, 1280.0);
        overlays.clear_overlays();
        assert!(overlays.computer_overlay.is_none());

        let theme = ShellThemeVisualState::default();
        assert_eq!(theme.bubble_radius_px, 12);
        assert_eq!(theme.active_lang_id(), "en");
    }
}
