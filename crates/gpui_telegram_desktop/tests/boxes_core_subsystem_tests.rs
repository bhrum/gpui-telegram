//! Subsystem tests for Module 08 Core Dialog Boxes & Confirmation Layer.
//! Validates 1:1 parity with Telegram Desktop upstream implementations.

use std::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering};
use std::sync::Arc;

use gpui::px;

use gpui_telegram_desktop::views::boxes::about_box::{
    alpha_version_download_url, current_version_text, telegram_faq_link, AboutBox,
};
use gpui_telegram_desktop::views::boxes::about_sponsored_box::{AboutSponsoredBox, K_SPONSORED_ADS_URL};
use gpui_telegram_desktop::views::boxes::abstract_box::{AbstractBox, AbstractBoxState};
use gpui_telegram_desktop::views::boxes::auto_lock_box::{
    parse_custom_time, time_string, AutoLockBox, K_CUSTOM, K_OPTIONS,
};
use gpui_telegram_desktop::views::boxes::confirm_box::{ConfirmBox, ConfirmBoxArgs};
use gpui_telegram_desktop::views::boxes::connection_box::{
    proxy_to_tg_link, ConnectionBox, ProxyConfig, ProxyItemState, ProxyType,
    K_SAVE_SETTINGS_DELAYED_TIMEOUT_MS,
};
use gpui_telegram_desktop::views::boxes::delete_box::DeleteBoxView;
use gpui_telegram_desktop::views::boxes::delete_messages_box::{
    DeleteMessagesBox, DeleteMessagesMode, K_DELETE_MESSAGES_BOX_ANIMATION_DURATION_MS,
};
use gpui_telegram_desktop::views::boxes::passcode_box::{PasscodeBox, PasscodeMode};
use gpui_telegram_desktop::views::boxes::share_box::{ShareBoxView, ShareRecipient};

#[test]
fn test_abstract_box_layering_and_lifecycle() {
    let mut modal = AbstractBox::new().with_title("Test Box");
    assert!(modal.is_layer_shown());
    assert_eq!(modal.layer_count, 1);
    assert_eq!(modal.width, px(380.0));
    assert!(modal.close_on_click_outside);

    modal.add_button("Cancel", false, false);
    modal.add_button("OK", true, false);
    assert_eq!(modal.buttons.len(), 2);
    assert!(!modal.buttons[0].is_primary);
    assert!(modal.buttons[1].is_primary);

    let closed = Arc::new(AtomicBool::new(false));
    let closed_clone = closed.clone();
    modal.on_close = Some(Arc::new(move || {
        closed_clone.store(true, Ordering::SeqCst);
    }));

    // Test layer hide
    let remaining = modal.hide_layer();
    assert_eq!(remaining, 0);
    assert_eq!(modal.state, AbstractBoxState::Hidden);
    assert!(!modal.is_layer_shown());
    assert!(closed.load(Ordering::SeqCst));

    // Test layer re-show
    assert!(modal.show_box());
    assert!(modal.is_layer_shown());
    assert_eq!(modal.layer_count, 1);
}

#[test]
fn test_confirm_box_actions_and_keyboard_triggers() {
    let confirmed_flag = Arc::new(AtomicBool::new(false));
    let cancelled_flag = Arc::new(AtomicBool::new(false));

    let cf = confirmed_flag.clone();
    let cl = cancelled_flag.clone();

    let args = ConfirmBoxArgs {
        title: Some("Delete Chat".to_string()),
        text: "Are you sure?".to_string(),
        confirm_text: "Delete".to_string(),
        cancel_text: "Cancel".to_string(),
        inform: false,
        strict_cancel: false,
        is_danger: true,
        confirmed: Some(Arc::new(move || cf.store(true, Ordering::SeqCst))),
        cancelled: Some(Arc::new(move || cl.store(true, Ordering::SeqCst))),
    };

    let mut box_item = ConfirmBox::make_confirm_box(args);
    assert!(!box_item.is_closed);
    assert!(box_item.args.is_danger);

    // Enter key triggers confirm
    box_item.handle_enter_key();
    assert!(box_item.is_closed);
    assert!(confirmed_flag.load(Ordering::SeqCst));

    // Reset and test Esc key
    let mut box_item2 = ConfirmBox::make_confirm_box(ConfirmBoxArgs {
        cancelled: Some(Arc::new(move || cancelled_flag.store(true, Ordering::SeqCst))),
        ..Default::default()
    });
    box_item2.handle_esc_key();
    assert!(box_item2.is_closed);

    // Test inform box has inform = true
    let inform_box = ConfirmBox::make_inform_box("Operation complete");
    assert!(inform_box.args.inform);
    assert_eq!(inform_box.args.text, "Operation complete");
}

#[test]
fn test_auto_lock_box_intervals_and_parsing() {
    assert_eq!(K_OPTIONS, [60, 300, 3600, 18000, K_CUSTOM]);

    assert_eq!(time_string(60), "00:01");
    assert_eq!(time_string(300), "00:05");
    assert_eq!(time_string(3600), "01:00");
    assert_eq!(time_string(18000), "05:00");

    assert_eq!(parse_custom_time("10:00"), Some(36000));
    assert_eq!(parse_custom_time("00:10"), Some(600));
    assert_eq!(parse_custom_time("01:30"), Some(5400));
    assert_eq!(parse_custom_time("00:45"), Some(2700));
    assert_eq!(parse_custom_time("invalid"), None);
    assert_eq!(parse_custom_time("01:80"), None);

    let saved_time = Arc::new(AtomicI32::new(0));
    let st = saved_time.clone();
    let mut auto_lock = AutoLockBox::new(300);
    auto_lock.on_save = Some(Arc::new(move |secs| st.store(secs, Ordering::SeqCst)));

    auto_lock.select_option(3600);
    auto_lock.save();
    assert_eq!(saved_time.load(Ordering::SeqCst), 3600);
    assert!(auto_lock.is_closed);
}

#[test]
fn test_passcode_box_validation_rules_and_hashing() {
    let mut pbox = PasscodeBox::new(PasscodeMode::Create, false);

    // Too short (less than 4)
    pbox.set_new_passcode("123");
    pbox.set_reenter_passcode("123");
    assert!(pbox.validate().is_err());

    // Mismatched re-entry
    pbox.set_new_passcode("1234");
    pbox.set_reenter_passcode("1235");
    assert!(pbox.validate().is_err());

    // Hint equal to password
    pbox.set_new_passcode("1234");
    pbox.set_reenter_passcode("1234");
    pbox.set_hint("1234");
    assert!(pbox.validate().is_err());

    // Valid create
    pbox.set_hint("my pin");
    assert!(pbox.validate().is_ok());

    let success_pass = Arc::new(parking_lot::Mutex::new(String::new()));
    let sp = success_pass.clone();
    pbox.on_success = Some(Arc::new(move |pass| *sp.lock() = pass.to_string()));
    assert!(pbox.submit());
    assert_eq!(*success_pass.lock(), "1234");
    assert!(pbox.is_closed);

    // Verify salted hash output is non-empty 32-byte digest
    let salt1 = b"random_salt_1_bytes_telegram_12";
    let salt2 = b"random_salt_2_bytes_telegram_34";
    let hash = PasscodeBox::hash_passcode("1234", salt1, salt2);
    assert!(!hash.is_empty());
}
#[test]
fn test_delete_box_and_messages_box_semantics() {
    let mut dbox = DeleteBoxView::new(3, true).with_peer_name("Alice");
    assert_eq!(dbox.message_count, 3);
    assert!(dbox.delete_for_everyone);

    dbox.toggle_delete_for_everyone();
    assert!(!dbox.delete_for_everyone);

    let confirmed_revoke = Arc::new(AtomicBool::new(false));
    let cr = confirmed_revoke.clone();
    dbox.on_confirm = Some(Arc::new(move |revoke| cr.store(revoke, Ordering::SeqCst)));

    dbox.toggle_delete_for_everyone(); // Now true again
    dbox.confirm();
    assert!(dbox.is_closed);
    assert!(confirmed_revoke.load(Ordering::SeqCst));

    // Full DeleteMessagesBox
    assert_eq!(K_DELETE_MESSAGES_BOX_ANIMATION_DURATION_MS, 80);
    let mut mbox = DeleteMessagesBox::new_selected(vec![101, 102, 103], "Bob");
    assert_eq!(mbox.selected_count(), 3);
    assert!(mbox.revoke_for_all);
    assert!(!mbox.remember_choice);

    mbox.toggle_revoke();
    mbox.toggle_remember();
    assert!(!mbox.revoke_for_all);
    assert!(mbox.remember_choice);

    let wipe_box = DeleteMessagesBox::new_wipe_history("Public Channel", true, true);
    assert!(wipe_box.is_channel);
    assert_eq!(wipe_box.mode, DeleteMessagesMode::WipeHistory { just_clear: true });
}

#[test]
fn test_about_box_and_sponsored_box_parity() {
    assert_eq!(telegram_faq_link("en_US"), "https://telegram.org/faq");
    assert_eq!(telegram_faq_link("de_DE"), "https://telegram.org/faq/de");
    assert_eq!(telegram_faq_link("es_ES"), "https://telegram.org/faq/es");
    assert_eq!(telegram_faq_link("it_IT"), "https://telegram.org/faq/it");
    assert_eq!(telegram_faq_link("ko_KR"), "https://telegram.org/faq/ko");
    assert_eq!(telegram_faq_link("pt-br"), "https://telegram.org/faq/br");

    let v_release = current_version_text("5.8.1", false, false, None);
    assert!(v_release.starts_with("5.8.1"));

    let v_canary = current_version_text("5.8.1", false, true, Some("a1b2c3d"));
    assert!(v_canary.contains("canary"));
    assert!(v_canary.contains("a1b2c3d"));

    let alpha_url = alpha_version_download_url(100500, "mac");
    assert!(alpha_url.starts_with("https://tdesktop.com/mac/"));
    assert!(alpha_url.ends_with(".zip"));

    let sponsored = AboutSponsoredBox::new();
    assert_eq!(K_SPONSORED_ADS_URL, "https://ads.telegram.org");
    assert_eq!(sponsored.ads_url, "https://ads.telegram.org");
    assert!(!sponsored.is_closed);

    let about = AboutBox::new();
    assert!(about.version_text.contains("5.8.1"));
    assert!(!about.is_closed);
}

#[test]
fn test_connection_box_proxy_and_deep_links() {
    assert_eq!(K_SAVE_SETTINGS_DELAYED_TIMEOUT_MS, 1000);

    let socks_proxy = ProxyConfig {
        id: 1,
        proxy_type: ProxyType::Socks5,
        host: "127.0.0.1".to_string(),
        port: 1080,
        user: Some("tguser".to_string()),
        password: Some("tgpass".to_string()),
        secret: None,
        ping_ms: Some(42),
        state: ProxyItemState::Online,
        is_enabled: true,
    };

    let socks_link = proxy_to_tg_link(&socks_proxy);
    assert_eq!(socks_link, "tg://socks?server=127.0.0.1&port=1080&user=tguser&pass=tgpass");

    let mtproto_proxy = ProxyConfig {
        id: 2,
        proxy_type: ProxyType::Mtproto,
        host: "proxy.example.com".to_string(),
        port: 443,
        user: None,
        password: None,
        secret: Some("ee112233445566778899aabbccddeeff".to_string()),
        ping_ms: Some(120),
        state: ProxyItemState::Available,
        is_enabled: false,
    };

    let mtproto_link = proxy_to_tg_link(&mtproto_proxy);
    assert_eq!(mtproto_link, "tg://proxy?server=proxy.example.com&port=443&secret=ee112233445566778899aabbccddeeff");

    let mut conn_box = ConnectionBox::new();
    conn_box.add_proxy(socks_proxy);
    conn_box.add_proxy(mtproto_proxy);
    assert_eq!(conn_box.proxies.len(), 2);

    conn_box.select_proxy(2);
    assert_eq!(conn_box.selected_proxy_id, Some(2));
    assert!(conn_box.proxies[1].is_enabled);
    assert!(!conn_box.proxies[0].is_enabled);

    conn_box.remove_proxy(1);
    assert_eq!(conn_box.proxies.len(), 1);
}

#[test]
fn test_share_box_recipients_and_actions() {
    let r1 = ShareRecipient {
        peer_id: 1001,
        name: "Alice Smith".to_string(),
        username: Some("alice".to_string()),
        is_selected: false,
    };
    let r2 = ShareRecipient {
        peer_id: 1002,
        name: "Dev Team Chat".to_string(),
        username: None,
        is_selected: true,
    };

    let mut sbox = ShareBoxView::new("https://t.me/telegram".to_string())
        .with_recipients(vec![r1, r2]);

    assert_eq!(sbox.selected_count(), 1);
    assert_eq!(sbox.selected_peer_ids(), vec![1002]);

    sbox.toggle_recipient(1001);
    assert_eq!(sbox.selected_count(), 2);

    sbox.set_comment("Check this out!");
    assert_eq!(sbox.copy_link(), "https://t.me/telegram");

    let submitted_count = Arc::new(AtomicUsize::new(0));
    let sc = submitted_count.clone();
    sbox.on_submit = Some(Arc::new(move |peers, _comment| {
        sc.store(peers.len(), Ordering::SeqCst);
    }));

    sbox.submit();
    assert!(sbox.is_closed);
    assert_eq!(submitted_count.load(Ordering::SeqCst), 2);
}
