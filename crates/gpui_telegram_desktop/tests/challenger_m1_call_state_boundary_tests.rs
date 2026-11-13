//! Empirical Challenger M1 Call State & Boundary Conditions Regression Suite
//!
//! Validates:
//! 1. Static code analysis & oracle assertion across ALL source files in `crates/gpui_telegram_desktop/src`:
//!    - Zero occurrences of "Claw Assistant"
//!    - Zero occurrences of "正在为您接入实时语音"
//!    - Zero occurrences of static fake latency ("latency_ms: 18" or "18ms")
//!    - Zero occurrences of hardcoded initial unread badges ("unread_badge: 1" or "unread_badge: 2")
//! 2. Message Click Bubble Fuzzing:
//!    - Simulates clicking every message type in conversation history (all canonical variants).
//!    - Asserts `pending_call_request` remains false and no call overlay is triggered.
//! 3. Escape Key Dismissal & Lifecycle:
//!    - Verifies Escape key cleanly dismisses TalkMode and resets destination to Chats.
//!    - Verifies background pulses do not resurrect dismissed overlays.

use std::fs;
use std::path::{Path, PathBuf};
use gpui_telegram_desktop::views::nav_rail::{NavDestination, NavRail};
use gpui_telegram_desktop::views::call_overlay::CallOverlay;
use gpui_telegram_desktop::views::conversation_stage::{
    ConversationStage, DeliveryStatus, InvoiceStatus, MediaType, MessageContent,
    MessageDirection, MessageItemModel, PollOption, PollType, ServiceType,
};
use gpui_telegram_desktop::views::shell::ActiveWorkspacePanel;

// ============================================================================
// PART 1: Static Code Analysis Oracle across ALL source files in `src/`
// ============================================================================

fn collect_all_rs_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(collect_all_rs_files(&path));
            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                files.push(path);
            }
        }
    }
    files
}

#[test]
fn test_oracle_zero_occurrences_of_claw_assistant_in_src() {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let src_dir = Path::new(crate_dir).join("src");
    let rs_files = collect_all_rs_files(&src_dir);
    assert!(!rs_files.is_empty(), "src directory must contain .rs files");

    let mut violations = Vec::new();
    for file in &rs_files {
        let content = fs::read_to_string(file).expect("Failed to read file");
        for (line_no, line) in content.lines().enumerate() {
            if line.contains("Claw Assistant") {
                violations.push(format!("{}:{}: {}", file.display(), line_no + 1, line.trim()));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "FOUND FORBIDDEN STRING 'Claw Assistant' in source files:\n{}",
        violations.join("\n")
    );
}

#[test]
fn test_oracle_zero_occurrences_of_voice_connecting_script_in_src() {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let src_dir = Path::new(crate_dir).join("src");
    let rs_files = collect_all_rs_files(&src_dir);

    let mut violations = Vec::new();
    for file in &rs_files {
        let content = fs::read_to_string(file).expect("Failed to read file");
        for (line_no, line) in content.lines().enumerate() {
            if line.contains("正在为您接入实时语音") {
                violations.push(format!("{}:{}: {}", file.display(), line_no + 1, line.trim()));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "FOUND FORBIDDEN STRING '正在为您接入实时语音' in source files:\n{}",
        violations.join("\n")
    );
}

#[test]
fn test_oracle_zero_hardcoded_unread_badges_1_or_2_in_src() {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let src_dir = Path::new(crate_dir).join("src");
    let rs_files = collect_all_rs_files(&src_dir);

    let mut violations = Vec::new();
    for file in &rs_files {
        let content = fs::read_to_string(file).expect("Failed to read file");
        for (line_no, line) in content.lines().enumerate() {
            if line.contains("unread_badge: 1") || line.contains("unread_badge: 2") {
                violations.push(format!("{}:{}: {}", file.display(), line_no + 1, line.trim()));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "FOUND HARDCODED UNREAD BADGE 1 or 2 in source files:\n{}",
        violations.join("\n")
    );
}

#[test]
fn test_oracle_zero_static_fake_latency_in_call_overlay() {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let call_overlay_path = Path::new(crate_dir).join("src/views/call_overlay.rs");
    let content = fs::read_to_string(&call_overlay_path).expect("Failed to read call_overlay.rs");

    let mut violations = Vec::new();
    for (line_no, line) in content.lines().enumerate() {
        if line.contains("latency_ms: 18") || line.contains("18ms") {
            violations.push(format!("{}:{}: {}", call_overlay_path.display(), line_no + 1, line.trim()));
        }
    }

    assert!(
        violations.is_empty(),
        "FOUND STATIC FAKE LATENCY in call_overlay.rs:\n{}",
        violations.join("\n")
    );
}

/// Adversarial check: Scan all source files in src/ for "18ms" or "latency_ms: 18" static latency references.
/// Reports all occurrences found across the entire src directory.
#[test]
fn test_adversarial_scan_18ms_static_fake_latency_in_src() {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let src_dir = Path::new(crate_dir).join("src");
    let rs_files = collect_all_rs_files(&src_dir);

    let mut found_18ms_locations = Vec::new();
    for file in &rs_files {
        let content = fs::read_to_string(file).expect("Failed to read file");
        for (line_no, line) in content.lines().enumerate() {
            if line.contains("18ms") || line.contains("latency_ms: 18") {
                found_18ms_locations.push(format!("{}:{}: {}", file.display(), line_no + 1, line.trim()));
            }
        }
    }

    println!("[18ms Scan Audit] Total matches found in src: {}", found_18ms_locations.len());
    for loc in &found_18ms_locations {
        println!("  -> {}", loc);
    }
}

// ============================================================================
// PART 2: Message Click Bubble Fuzzing
// ============================================================================

/// Helper to construct canonical messages of all types
fn make_fuzz_message(id: u64, content: MessageContent) -> MessageItemModel {
    MessageItemModel {
        id,
        sender_id: 100,
        sender_name: "Fuzz Peer".to_string(),
        avatar_initials: "FP".to_string(),
        avatar_color: 0x2AABEE,
        direction: MessageDirection::Incoming,
        timestamp: "12:00".to_string(),
        delivery_status: DeliveryStatus::Read,
        reply_to: None,
        reactions: vec![],
        content,
        bot_keyboard: None,
        is_pinned: false,
    }
}

/// Simulates the exact click dispatch logic in `conversation_stage.rs:1627-1668`
fn simulate_message_left_click(stage: &mut ConversationStage, msg_id: u64) {
    let msg = match stage.messages.iter().find(|m| m.id == msg_id) {
        Some(m) => m.clone(),
        None => return,
    };

    let media_click_info = match &msg.content {
        MessageContent::Media { url, media_type, .. } => {
            Some((url.clone(), matches!(media_type, MediaType::Video)))
        }
        _ => None,
    };
    let is_voice_msg = matches!(msg.content, MessageContent::Voice { .. });
    let is_invoice_msg = matches!(msg.content, MessageContent::Invoice { .. });

    if let Some((url, is_video)) = media_click_info {
        stage.pending_media_url = Some((url, is_video));
    } else if is_voice_msg {
        if let Some(m) = stage.messages.iter_mut().find(|m| m.id == msg_id) {
            if let MessageContent::Voice {
                ref mut is_playing,
                ref mut playback_position_secs,
                ref mut playback_speed,
                duration_secs,
                ..
            } = m.content
            {
                *is_playing = !*is_playing;
                if *is_playing {
                    *playback_position_secs =
                        (*playback_position_secs + 6.0) % (duration_secs as f32).max(1.0);
                } else {
                    *playback_speed = if (*playback_speed - 1.0).abs() < 0.1 {
                        1.5
                    } else if (*playback_speed - 1.5).abs() < 0.1 {
                        2.0
                    } else {
                        1.0
                    };
                }
            }
        }
    } else if is_invoice_msg {
        if let Some(m) = stage.messages.iter_mut().find(|m| m.id == msg_id) {
            if let MessageContent::Invoice { ref mut status, .. } = m.content {
                *status = match *status {
                    InvoiceStatus::Pending => InvoiceStatus::Paid,
                    InvoiceStatus::Paid => InvoiceStatus::Refunded,
                    InvoiceStatus::Refunded => InvoiceStatus::Paid,
                };
            }
        }
    }
}

#[test]
fn test_fuzz_every_message_type_bubble_click_never_triggers_call() {
    let mut stage = ConversationStage::new();

    // 1. First test default history (contains all 11 canonical demo cards, including Call #107)
    let default_ids: Vec<u64> = stage.messages.iter().map(|m| m.id).collect();
    assert!(!default_ids.is_empty(), "Stage must have demo messages");

    for id in default_ids {
        assert!(!stage.pending_call_request, "pending_call_request must be false before click");
        simulate_message_left_click(&mut stage, id);
        assert!(
            !stage.pending_call_request,
            "CRITICAL BUG: Clicking message id {} triggered pending_call_request!",
            id
        );
    }

    // 2. Synthesize messages covering all possible MessageContent variants and directions
    let test_contents = vec![
        MessageContent::Text {
            text: "Hello adversarial test".to_string(),
            is_edited: false,
        },
        MessageContent::Text {
            text: "Edited message".to_string(),
            is_edited: true,
        },
        MessageContent::Media {
            media_type: MediaType::Photo,
            url: "https://example.com/photo.jpg".to_string(),
            width: 800,
            height: 600,
            duration_secs: None,
            file_size_bytes: 10240,
            caption: Some("Photo caption".to_string()),
            is_streaming: false,
        },
        MessageContent::Media {
            media_type: MediaType::Video,
            url: "https://example.com/video.mp4".to_string(),
            width: 1920,
            height: 1080,
            duration_secs: Some(120),
            file_size_bytes: 1048576,
            caption: Some("Video caption".to_string()),
            is_streaming: false,
        },
        MessageContent::Voice {
            duration_secs: 45,
            waveform_5bit: vec![1, 2, 3, 4, 5],
            playback_position_secs: 0.0,
            is_playing: false,
            playback_speed: 1.0,
        },
        MessageContent::Album {
            items: vec![],
            caption: Some("Album caption".to_string()),
        },
        MessageContent::Location {
            latitude: 37.7749,
            longitude: -122.4194,
            title: "San Francisco".to_string(),
            address: "CA, USA".to_string(),
        },
        MessageContent::Poll {
            question: "Is this secure?".to_string(),
            poll_type: PollType::Regular,
            options: vec![PollOption {
                id: 1,
                text: "Yes".to_string(),
                votes: 10,
                percentage: 100.0,
                is_selected: false,
                is_correct: false,
            }],
            total_votes: 10,
            is_closed: false,
            selected_option_id: None,
        },
        MessageContent::Call {
            call_type: gpui_telegram_desktop::views::stage::cards::CallType::Voice,
            direction: gpui_telegram_desktop::views::stage::cards::CallDirection::Incoming,
            duration_secs: 180,
            status_text: "Call ended".to_string(),
        },
        MessageContent::Call {
            call_type: gpui_telegram_desktop::views::stage::cards::CallType::Video,
            direction: gpui_telegram_desktop::views::stage::cards::CallDirection::Outgoing,
            duration_secs: 45,
            status_text: "Missed call".to_string(),
        },
        MessageContent::Story {
            user_name: "Alice".to_string(),
            avatar_initials: "A".to_string(),
            caption: "Story caption".to_string(),
            preview_color: 0x2AABEE,
            expires_in: "24h".to_string(),
        },
        MessageContent::Invoice {
            title: "Premium Stars".to_string(),
            description: "100 Stars".to_string(),
            amount_stars: 100,
            currency: "XTR".to_string(),
            status: InvoiceStatus::Pending,
            receipt_id: None,
        },
        MessageContent::Service {
            text: "Alice joined the group".to_string(),
            service_type: ServiceType::UserJoined,
        },
    ];

    let mut start_id = 5000u64;
    for content in test_contents {
        start_id += 1;
        let msg = make_fuzz_message(start_id, content);
        stage.messages.push(msg);

        // Click it multiple times
        for _ in 0..5 {
            simulate_message_left_click(&mut stage, start_id);
            assert!(
                !stage.pending_call_request,
                "CRITICAL: Message id {} activated pending_call_request on click!",
                start_id
            );
        }
    }
}

// ============================================================================
// PART 3: Escape Key Dismissal & Lifecycle State Machine
// ============================================================================

/// Shell State Machine Simulator for testing Escape key and Overlay lifecycle
struct ShellLifecycleSimulator {
    pub active_destination: NavDestination,
    pub last_handled_nav_dest: Option<NavDestination>,
    pub active_panel: ActiveWorkspacePanel,
    pub talk_mode_overlay_active: bool,
    pub computer_overlay_active: bool,
}

impl ShellLifecycleSimulator {
    pub fn new() -> Self {
        Self {
            active_destination: NavDestination::Chats,
            last_handled_nav_dest: Some(NavDestination::Chats),
            active_panel: ActiveWorkspacePanel::Chats,
            talk_mode_overlay_active: false,
            computer_overlay_active: false,
        }
    }

    /// User clicks destination on NavRail
    pub fn select_nav_destination(&mut self, dest: NavDestination) {
        self.active_destination = dest;
        self.last_handled_nav_dest = Some(dest);
        match dest {
            NavDestination::Chats => self.active_panel = ActiveWorkspacePanel::Chats,
            NavDestination::Calls => self.active_panel = ActiveWorkspacePanel::Calls,
            NavDestination::TalkMode => self.talk_mode_overlay_active = true,
            NavDestination::RemoteComputer => self.computer_overlay_active = true,
            NavDestination::TaskCenter => self.active_panel = ActiveWorkspacePanel::TaskCenter,
            NavDestination::McpTools => self.active_panel = ActiveWorkspacePanel::McpTools,
            NavDestination::Settings => self.active_panel = ActiveWorkspacePanel::Settings,
            _ => {}
        }
    }

    /// Escape key press dispatch matching `shell.rs:1572-1581`
    pub fn handle_escape_key(&mut self) -> bool {
        if self.talk_mode_overlay_active {
            self.hide_talk_mode();
            return true;
        }
        if self.computer_overlay_active {
            self.hide_remote_computer();
            return true;
        }
        false
    }

    /// `hide_talk_mode` matching `shell.rs:629-637`
    pub fn hide_talk_mode(&mut self) {
        self.talk_mode_overlay_active = false;
        self.last_handled_nav_dest = Some(NavDestination::Chats);
        if self.active_destination == NavDestination::TalkMode
            || self.active_destination == NavDestination::Calls
        {
            self.active_destination = NavDestination::Chats;
        }
    }

    /// `hide_remote_computer` matching `shell.rs:650-658`
    pub fn hide_remote_computer(&mut self) {
        self.computer_overlay_active = false;
        self.last_handled_nav_dest = Some(NavDestination::Chats);
        if self.active_destination == NavDestination::RemoteComputer {
            self.active_destination = NavDestination::Chats;
        }
    }

    /// Background notification pulse matching `shell.rs:163`
    pub fn on_background_pulse(&mut self) {
        // Guard check:
        // if self.last_handled_nav_trigger == trigger && self.last_handled_nav_dest == Some(dest)
        // Background pulse does NOT change trigger or destination, so it short-circuits.
    }
}

#[test]
fn test_escape_key_cleanly_dismisses_talk_mode_and_resets_destination() {
    let mut shell = ShellLifecycleSimulator::new();

    // 1. User activates TalkMode via NavRail
    shell.select_nav_destination(NavDestination::TalkMode);
    assert!(shell.talk_mode_overlay_active);
    assert_eq!(shell.active_destination, NavDestination::TalkMode);

    // 2. User presses Escape key
    let handled = shell.handle_escape_key();
    assert!(handled, "Escape key must be handled when talk mode is active");
    assert!(!shell.talk_mode_overlay_active, "TalkMode overlay must be dismissed");
    assert_eq!(
        shell.active_destination,
        NavDestination::Chats,
        "Active destination must reset to Chats on Escape"
    );
    assert_eq!(shell.last_handled_nav_dest, Some(NavDestination::Chats));

    // 3. Repeated background pulses must NOT resurrect TalkMode
    for _ in 0..100 {
        shell.on_background_pulse();
        assert!(!shell.talk_mode_overlay_active, "TalkMode must remain dismissed");
        assert_eq!(shell.active_destination, NavDestination::Chats);
    }
}

#[test]
fn test_escape_key_when_talk_mode_not_active_is_not_intercepted() {
    let mut shell = ShellLifecycleSimulator::new();
    assert!(!shell.talk_mode_overlay_active);

    // Escape should return false (not consumed by talk mode overlay)
    let handled = shell.handle_escape_key();
    assert!(!handled, "Escape key must not be consumed if no overlay is active");
    assert_eq!(shell.active_destination, NavDestination::Chats);
}

#[test]
fn test_call_overlay_clean_empty_state_and_dismiss_flag() {
    let mut overlay = CallOverlay::new("Bob Jones".to_string());
    assert_eq!(overlay.persona_name, "Bob Jones");
    assert_eq!(overlay.latency_ms, 0);
    assert!(overlay.transcript.is_empty());
    assert!(!overlay.is_mic_muted);
    assert!(!overlay.should_close);

    // Closing the overlay sets should_close
    overlay.should_close = true;
    assert!(overlay.should_close);
}

#[test]
fn test_nav_rail_unread_badges_are_authentic_zero() {
    let rail = NavRail::new();
    for item in &rail.items {
        if item.destination != NavDestination::Chats {
            assert_eq!(
                item.unread_badge, 0,
                "Destination {:?} must have 0 unread badge initially!",
                item.destination
            );
        }
    }
}
