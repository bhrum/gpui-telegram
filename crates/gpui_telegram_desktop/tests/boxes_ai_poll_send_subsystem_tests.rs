//! Subsystem unit and integration tests for Module 08 Slice 11:
//! AI boxes, Poll/Quiz box, Send Files boxes, and Utility Dialogs.

use gpui_telegram_desktop::views::boxes::auto_download_box::*;
use gpui_telegram_desktop::views::boxes::compose_ai_box::*;
use gpui_telegram_desktop::views::boxes::create_ai_box::*;
use gpui_telegram_desktop::views::boxes::create_ai_tone_box::*;
use gpui_telegram_desktop::views::boxes::create_poll_box::*;
use gpui_telegram_desktop::views::boxes::dictionaries_manager::*;
use gpui_telegram_desktop::views::boxes::download_path_box::*;
use gpui_telegram_desktop::views::boxes::edit_caption_box::*;
use gpui_telegram_desktop::views::boxes::edit_todo_list_box::*;
use gpui_telegram_desktop::views::boxes::keys_legend::*;
use gpui_telegram_desktop::views::boxes::max_invite_box::*;
use gpui_telegram_desktop::views::boxes::moderate_messages_box::*;
use gpui_telegram_desktop::views::boxes::music_attach_box::*;
use gpui_telegram_desktop::views::boxes::phone_banned_box::*;
use gpui_telegram_desktop::views::boxes::pin_messages_box::*;
use gpui_telegram_desktop::views::boxes::preview_ai_tone_box::*;
use gpui_telegram_desktop::views::boxes::ringtones_box::*;
use gpui_telegram_desktop::views::boxes::self_destruction_box::*;
use gpui_telegram_desktop::views::boxes::send_files_box::*;
use gpui_telegram_desktop::views::boxes::send_files_box_reply_header::*;
use gpui_telegram_desktop::views::boxes::send_gif_with_caption_box::*;

#[test]
fn test_auto_download_box_rules_and_buckets() {
    let mut b = AutoDownloadBox::new(AutoDownloadSource::Group);
    assert_eq!(b.source_title(), "Groups");
    assert_eq!(b.video_size_limit, K_DEFAULT_DOWNLOAD_LIMIT);
    assert_eq!(K_DEFAULT_DOWNLOAD_LIMIT, 10 * 1024 * 1024);
    assert_eq!(K_DEFAULT_AUTOPLAY_LIMIT, 50 * 1024 * 1024);

    b.add_always_exception(12345);
    b.add_never_exception(67890);
    assert_eq!(b.exceptions_count(), 2);
    assert!(b.buckets.always.contains(&12345));
    assert!(b.buckets.never.contains(&67890));

    // Switching exception bucket
    b.add_always_exception(67890);
    assert!(!b.buckets.never.contains(&67890));
    assert!(b.buckets.always.contains(&67890));
}

#[test]
fn test_compose_ai_box_mode_and_diffs() {
    let mut ai = ComposeAiBox::new("Draft text to refine");
    assert_eq!(ai.state, CardState::Waiting);
    assert_eq!(gpui_telegram_desktop::views::boxes::compose_ai_box::K_PROMPT_REMAINING_THRESHOLD, 100);

    ai.set_mode(ComposeAiMode::Fix);
    ai.start_generation();
    assert_eq!(ai.state, CardState::Loading);

    let diff = DiffEntity {
        diff_type: DiffType::Replace,
        offset: 0,
        length: 5,
        old_text: "Draft".to_string(),
        new_text: "Final".to_string(),
    };
    ai.complete_generation("Final text to refine".to_string(), vec![diff]);
    assert_eq!(ai.state, CardState::Ready);
    assert_eq!(ai.diff_entities.len(), 1);
}

#[test]
fn test_create_ai_language_fallback() {
    assert_eq!(AiLanguage::default_for("en"), AiLanguage::Spanish);
    assert_eq!(AiLanguage::default_for("English"), AiLanguage::Spanish);
    assert_eq!(AiLanguage::default_for("de"), AiLanguage::English);
    assert_eq!(AiLanguage::default_for("ru"), AiLanguage::English);

    let mut box_ai = CreateAiBox::new("Write an article about Rust GPUI");
    assert!(!box_ai.is_prompt_near_limit());
    box_ai.prompt = "a".repeat(950);
    assert!(box_ai.is_prompt_near_limit());
}

#[test]
fn test_create_poll_box_limits_and_validation() {
    assert_eq!(K_QUESTION_LIMIT, 255);
    assert_eq!(K_MAX_OPTIONS_COUNT, 10);
    assert_eq!(K_OPTION_LIMIT, 100);

    let mut poll = CreatePollBox::new();
    assert!(!poll.is_valid());

    poll.question = "What is your favorite language?".to_string();
    poll.options[0].text = "Rust".to_string();
    poll.options[1].text = "C++".to_string();
    assert!(poll.is_valid());

    // Option limit check
    for i in 2..K_MAX_OPTIONS_COUNT {
        assert!(poll.add_option(format!("Option {}", i)));
    }
    assert_eq!(poll.options.len(), 10);
    assert!(!poll.add_option("11th Option"));

    // Quiz mode requires marked correct answer
    poll.is_quiz = true;
    assert!(!poll.is_valid());
    poll.set_correct_option(0);
    assert!(poll.is_valid());
}

#[test]
fn test_edit_todo_list_tasks_and_validation() {
    let mut todo = EditTodoListBox::new().with_title("Sprint 8 Tasks");
    assert!(!todo.is_valid());

    todo.tasks[0].text = "Read upstream C++".to_string();
    todo.add_task("Implement GPUI views");
    assert!(todo.is_valid());
    assert_eq!(todo.tasks.len(), 2);

    todo.toggle_completed(1);
    assert!(todo.tasks[0].is_completed);

    assert!(todo.remove_task(2));
    assert_eq!(todo.tasks.len(), 1);
}

#[test]
fn test_send_files_box_limits_and_caption() {
    assert_eq!(K_MAX_MESSAGE_LENGTH, 4096);
    assert_eq!(K_STASH_FILES_LIMIT, 100);
    assert_eq!(K_STASH_FILES_MEMORY_LIMIT_BYTES, 256 * 1024 * 1024);

    let mut send = SendFilesBox::new(vec![
        PreparedMediaFile {
            path: "/path/to/photo.jpg".to_string(),
            display_name: "photo.jpg".to_string(),
            size_bytes: 2 * 1024 * 1024,
            is_image: true,
            is_video: false,
        },
    ]);
    assert_eq!(send.total_bytes(), 2 * 1024 * 1024);
    assert_eq!(send.send_way, SendFilesWay::Album);

    send.set_caption("My caption");
    assert_eq!(send.caption, "My caption");
    send.toggle_compress();
    assert!(!send.compress_images);
}

#[test]
fn test_phone_banned_and_pin_boxes() {
    let banned = PhoneBannedBox::new("+1234567890");
    let mailto = banned.build_support_mailto();
    assert!(mailto.starts_with("mailto:login@stel.com?subject="));
    assert!(mailto.contains("%2B1234567890"));

    let pin_private = PinMessagesBox::new(42, PinChatType::PrivateUser, "Alice");
    assert!(pin_private.confirmation_text().contains("Pin this message in your chat with Alice?"));
    assert!(!pin_private.is_silent_pin());

    let pin_group = PinMessagesBox::new(43, PinChatType::Group, "Dev Group");
    assert!(pin_group.confirmation_text().contains("pin this message in this group?"));
}

#[test]
fn test_self_destruction_ttl_values() {
    assert_eq!(ACCOUNT_TTL_DAYS, [30, 90, 180, 365, 548, 720]);
    assert_eq!(SESSIONS_TTL_DAYS, [7, 30, 90, 180, 365]);

    assert_eq!(SelfDestructionBox::days_label(7), "1 week");
    assert_eq!(SelfDestructionBox::days_label(30), "1 month");
    assert_eq!(SelfDestructionBox::days_label(180), "6 months");
    assert_eq!(SelfDestructionBox::days_label(365), "1 year");
    assert_eq!(SelfDestructionBox::days_label(720), "2 years");

    let account_box = SelfDestructionBox::new(SelfDestructionType::Account, 365);
    assert_eq!(account_box.selected_days, 365);
    assert_eq!(account_box.available_values().len(), 6);
}

#[test]
fn test_ringtones_and_keys_legend() {
    assert_eq!(K_DEFAULT_SOUND_VALUE, -1);
    assert_eq!(K_NO_SOUND_VALUE, -2);
    assert_eq!(K_NO_DETACH_TIMEOUT_MS, 250);

    let mut ring = RingtonesBox::new();
    assert_eq!(ring.selected_sound_id, -1);
    ring.set_sound(101);
    assert_eq!(ring.selected_sound_id, 101);
    ring.set_volume(0.5);
    assert!((ring.volume_level - 0.5).abs() < 1e-4);

    assert!(is_modifier_glyph(GLYPH_CMD));
    assert!(is_modifier_glyph(GLYPH_CTRL));
    assert!(is_modifier_glyph(GLYPH_OPT));
    assert!(is_modifier_glyph(GLYPH_SHIFT));
    assert!(!is_modifier_glyph('A'));

    let legend = KeysLegend::new();
    assert!(legend.total_shortcuts() >= 6);
}

#[test]
fn test_utility_boxes() {
    let mut dl = DownloadPathBox::new();
    assert_eq!(dl.selected_type, DownloadDirectoryType::Downloads);
    dl.set_type(DownloadDirectoryType::Temp);
    assert_eq!(dl.effective_path_display(), "Temporary folder (cleared on exit)");

    let mut dict = DictionariesManager::new();
    assert_eq!(dict.filtered_dictionaries().len(), 4);
    dict.set_query("span");
    assert_eq!(dict.filtered_dictionaries().len(), 1);

    let max_inv = MaxInviteBox::new(1001, 200_000).with_invite_link("https://t.me/+join");
    assert_eq!(max_inv.copy_link_to_clipboard().as_deref(), Some("https://t.me/+join"));

    let mod_box = ModerateMessagesBox::new(999, "Spammer", vec![1, 2, 3]).with_common_groups(5);
    assert_eq!(mod_box.common_groups_count, 5);

    let mut rep = SendFilesBoxReplyHeader::new(101, "Bob", "Hello");
    assert!(rep.is_visible);
    rep.dismiss();
    assert!(!rep.is_visible);

    let mut gif = SendGifWithCaptionBox::new(555, 666, "Dancing cat");
    assert_eq!(gif.caption, "Dancing cat");
    gif.toggle_silent();
    assert!(gif.is_silent);

    let tone = CreateAiToneBox::new();
    assert!(!tone.is_valid());

    let mut prev = PreviewAiToneBox::new(1, "Formal", "Use formal prose");
    prev.trigger_spin();
    assert_eq!(prev.spin_phase, RefreshSpinPhase::Spinning);

    let mut edit_cap = EditCaptionBox::new(777, "Old caption");
    edit_cap.toggle_invert_caption();
    assert!(edit_cap.invert_caption);

    let mut music = MusicAttachBox::new();
    assert_eq!(music.selected_count(), 0);
    music.toggle_track_selected(10);
    assert_eq!(music.selected_count(), 1);
}
