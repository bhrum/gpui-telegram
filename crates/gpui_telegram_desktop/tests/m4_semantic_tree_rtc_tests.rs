//! Headless GPUI Integration Tests for Milestone 4 Features 18 & 23:
//! - Feature 18: `AppShell::inspect_state` & `AppShell::perform_action` (`SemanticWorkspaceTree`)
//! - Feature 23: `DefaultAiMediaTap` + `AppShell::process_incoming_ai_audio_pcm` -> `TalkCoordinator::handle_barge_in`

use gpui::{App, AppContext, Application, Entity};
use gpui_telegram_agent::talk::TalkSessionState;
use gpui_telegram_client::{ClientConfig, TelegramClient};
use gpui_telegram_desktop::theme::TelegramAssetSource;
use gpui_telegram_desktop::views::chat_sidebar::{DialogDeliveryStatus, DialogItemModel};
use gpui_telegram_desktop::views::AppShell;
use gpui_telegram_proto::semantic_tree::{ActionExecutionStatus, SemanticAction};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

const CHILD_ENV: &str = "GPUI_M4_SEMANTIC_RTC_CHILD";
const CHILD_TARGET_ENV: &str = "GPUI_M4_SEMANTIC_RTC_TARGET";
static TEMP_COUNTER: AtomicU64 = AtomicU64::new(1);

struct TempConfigDirGuard {
    path: PathBuf,
}

impl TempConfigDirGuard {
    fn new(prefix: &str) -> Self {
        let seq = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "gpui_tg_m4_sem_rtc_{prefix}_{}_{}",
            std::process::id(),
            seq
        ));
        std::fs::create_dir_all(&dir).expect("create isolated temp config dir");
        Self { path: dir }
    }
}

impl Drop for TempConfigDirGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

fn dispatch_headless_target(target: &str) -> ! {
    let body: fn(&mut App) = match target {
        "test_gpui_appshell_semantic_tree_inspect_and_perform_action" => {
            body_appshell_semantic_tree_inspect_and_perform_action
        }
        "test_gpui_appshell_ai_media_tap_barge_in_wiring" => {
            body_appshell_ai_media_tap_barge_in_wiring
        }
        other => {
            eprintln!("unknown headless gpui test target: {other}");
            std::process::exit(101);
        }
    };
    let target_owned = target.to_string();
    let outer = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        Application::headless()
            .with_assets(TelegramAssetSource::new())
            .run(move |cx: &mut App| {
                let inner = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    body(cx);
                }));
                match inner {
                    Ok(()) => std::process::exit(0),
                    Err(payload) => {
                        let msg = payload
                            .downcast_ref::<&str>()
                            .copied()
                            .or_else(|| payload.downcast_ref::<String>().map(|s| s.as_str()))
                            .unwrap_or("panic");
                        eprintln!("[headless child panic] {target_owned}: {msg}");
                        std::process::exit(101);
                    }
                }
            });
    }));
    if let Err(payload) = outer {
        let msg = payload
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| payload.downcast_ref::<String>().map(|s| s.as_str()))
            .unwrap_or("panic");
        eprintln!("[headless outer panic] {target}: {msg}");
        std::process::exit(101);
    }
    std::process::exit(0);
}

#[cfg(target_os = "macos")]
#[used]
#[unsafe(link_section = "__DATA,__mod_init_func")]
static MACOS_HEADLESS_MAIN_THREAD_ENTRY: extern "C" fn() = macos_mod_init_headless_entry;

#[cfg(target_os = "macos")]
extern "C" fn macos_mod_init_headless_entry() {
    if std::env::var(CHILD_ENV).as_deref() == Ok("1") {
        if let Ok(target) = std::env::var(CHILD_TARGET_ENV) {
            dispatch_headless_target(&target);
        }
    }
}

fn run_in_headless_subprocess(test_fn_name: &str) {
    if std::env::var(CHILD_ENV).as_deref() == Ok("1") {
        dispatch_headless_target(test_fn_name);
    }

    let cfg_guard = TempConfigDirGuard::new(test_fn_name);
    let exe = std::env::current_exe().expect("current_exe");
    let output = std::process::Command::new(exe)
        .env(CHILD_ENV, "1")
        .env(CHILD_TARGET_ENV, test_fn_name)
        .env("GPUI_TELEGRAM_CONFIG_DIR", &cfg_guard.path)
        .env("GPUI_OFFLINE_TEST", "1")
        .arg("--exact")
        .arg(test_fn_name)
        .arg("--nocapture")
        .output()
        .expect("spawn headless gpui child");
    assert!(
        output.status.success(),
        "headless gpui child failed for {test_fn_name}:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn new_isolated_client() -> Arc<TelegramClient> {
    let mut cfg = ClientConfig::default();
    cfg.server_url = "http://127.0.0.1:9".to_string();
    Arc::new(TelegramClient::new(cfg))
}

#[test]
fn test_gpui_appshell_semantic_tree_inspect_and_perform_action() {
    run_in_headless_subprocess("test_gpui_appshell_semantic_tree_inspect_and_perform_action");
}

fn body_appshell_semantic_tree_inspect_and_perform_action(cx: &mut App) {
    let client = new_isolated_client();
    let _rt = client.runtime().enter();

    let shell: Entity<AppShell> = cx.new(|cx| {
        let s = AppShell::new_with_client(client.clone(), cx);
        s.sidebar.update(cx, |sb, _| {
            sb.dialogs.push(DialogItemModel {
                id: 7001,
                peer_name: "Semantic Workspace Chat".to_string(),
                avatar_initials: "SW".to_string(),
                avatar_color: 0x3A7AFE,
                last_message: String::new(),
                timestamp: "now".to_string(),
                unread_count: 0,
                is_muted: false,
                is_pinned: false,
                is_online: true,
                typing_indicator: None,
                delivery_status: DialogDeliveryStatus::Read,
                folder: "All".to_string(),
            });
        });
        s
    });

    shell.update(cx, |s, cx| {
        let initial_tree = s.inspect_state(cx);
        assert!(!initial_tree.state_hash.is_empty());

        // 1. SelectChat
        let r_sel = s.perform_action(SemanticAction::SelectChat { chat_id: 7001 }, cx);
        assert_eq!(r_sel.status, ActionExecutionStatus::Succeeded);
        let t1 = s.inspect_state(cx);
        assert_eq!(t1.stage.active_chat_id, Some(7001));
        assert_eq!(r_sel.resulting_state_hash, t1.state_hash);
        assert_ne!(t1.state_hash, initial_tree.state_hash);

        // 2. ComposeText & SendMessage
        let r_comp = s.perform_action(
            SemanticAction::ComposeText {
                text: "Hello via SemanticAction".to_string(),
            },
            cx,
        );
        assert_eq!(r_comp.status, ActionExecutionStatus::Succeeded);
        let t2 = s.inspect_state(cx);
        assert_eq!(t2.composer.text, "Hello via SemanticAction");

        let r_send = s.perform_action(SemanticAction::SendMessage, cx);
        assert_eq!(r_send.status, ActionExecutionStatus::Succeeded);
        let t3 = s.inspect_state(cx);
        assert!(t3.composer.text.is_empty());
        assert!(t3.stage.message_count >= 1);
        assert_ne!(t3.state_hash, t1.state_hash);

        // 3. OpenModal rejects empty modal_id, distinguishes "none" from None, & CloseModal
        let r_empty_modal = s.perform_action(
            SemanticAction::OpenModal {
                modal_id: "   ".to_string(),
            },
            cx,
        );
        assert_eq!(r_empty_modal.status, ActionExecutionStatus::Failed);
        assert!(s.inspect_state(cx).active_modal.is_none());

        let r_modal_none = s.perform_action(
            SemanticAction::OpenModal {
                modal_id: "none".to_string(),
            },
            cx,
        );
        assert_eq!(r_modal_none.status, ActionExecutionStatus::Succeeded);
        assert_ne!(s.inspect_state(cx).state_hash, t3.state_hash);
        let _ = s.perform_action(SemanticAction::CloseModal, cx);

        let r_open = s.perform_action(
            SemanticAction::OpenModal {
                modal_id: "new_group".to_string(),
            },
            cx,
        );
        assert_eq!(r_open.status, ActionExecutionStatus::Succeeded);
        assert!(s.inspect_state(cx).active_modal.is_some());

        let r_close = s.perform_action(SemanticAction::CloseModal, cx);
        assert_eq!(r_close.status, ActionExecutionStatus::Succeeded);
        assert!(s.inspect_state(cx).active_modal.is_none());

        // 4. ToggleCall
        let r_call = s.perform_action(
            SemanticAction::ToggleCall {
                peer_id: 7001,
                is_video: false,
            },
            cx,
        );
        assert_eq!(r_call.status, ActionExecutionStatus::Succeeded);
        assert!(s.inspect_state(cx).is_call_active);
    });
}

#[test]
fn test_gpui_appshell_ai_media_tap_barge_in_wiring() {
    run_in_headless_subprocess("test_gpui_appshell_ai_media_tap_barge_in_wiring");
}

fn body_appshell_ai_media_tap_barge_in_wiring(cx: &mut App) {
    let client = new_isolated_client();
    let _rt = client.runtime().enter();

    let shell: Entity<AppShell> = cx.new(|cx| AppShell::new_with_client(client.clone(), cx));

    shell.update(cx, |s, cx| {
        s.rtc_talk.talk_coordinator.state = TalkSessionState::Speaking;
        s.rtc_talk
            .talk_coordinator
            .enqueue_audio(vec![1, 2, 3, 4])
            .expect("enqueue audio");

        // Silent 20ms frame should not trigger barge-in
        let silence = vec![0i16; 320];
        let triggered_silence = s.process_incoming_ai_audio_pcm(&silence, 16_000, 1, cx);
        assert!(!triggered_silence);
        assert_eq!(s.rtc_talk.talk_coordinator.state, TalkSessionState::Speaking);

        // Three consecutive active 20ms frames (60ms <= 60ms VAD latency) trigger barge-in
        let speech = vec![12_000i16; 320];
        let mut triggered_speech = false;
        for _ in 0..3 {
            if s.process_incoming_ai_audio_pcm(&speech, 16_000, 1, cx) {
                triggered_speech = true;
                break;
            }
        }
        assert!(triggered_speech);
        assert_eq!(s.rtc_talk.talk_coordinator.state, TalkSessionState::Interrupted);
        assert!(s.rtc_talk.talk_coordinator.audio_buffer_queue.is_empty());
        assert!(s.rtc_talk.talk_coordinator.total_flushed_chunks >= 1);

        // Next AI Speaking turn must NOT inherit sticky barge-in on silent frames
        s.rtc_talk.talk_coordinator.state = TalkSessionState::Speaking;
        let trig_next_silence = s.process_incoming_ai_audio_pcm(&silence, 16_000, 1, cx);
        assert!(!trig_next_silence);
        assert_eq!(s.rtc_talk.talk_coordinator.state, TalkSessionState::Speaking);
    });
}
