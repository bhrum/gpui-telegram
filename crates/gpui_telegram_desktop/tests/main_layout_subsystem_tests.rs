//! Main and Layout Subsystem Integration Tests (Desktop Crate)
//! Verifies:
//! 1. Matrix position shift math (kMatrixRowShift = 40000 encoding/decoding)
//! 2. Group selection bitmask math (IsSubGroupSelection, IsGroupItemSelection, FirstGroupItemIndex, AddGroupItemSelection)
//! 3. Responsive mosaic layout algorithm (5 items per row, proportional width distribution, point hit-testing)
//! 4. Document generic preview 4-color categorization (Blue=0, Green=1, Red=2, Yellow=3)
//! 5. MainDomain multi-account controller, passcode derivation lock, and active session streams
//! 6. MainWidget responsive layout modes (3-column >=1200, 2-column >=800, 1-column <800) and navigation stack
//! 7. MainWindow work mode state machine (PasscodeLock, SetupEmailLock, Intro, MainContent) and layer stack

use gpui_telegram_desktop::app::main_domain::{MainDomain, MAX_ACCOUNTS, PREMIUM_MAX_ACCOUNTS};
use gpui_telegram_desktop::app::main_session::{FreezeInfo, MainSession};
use gpui_telegram_desktop::app::mainwidget::{LayoutMode, Mainwidget};
use gpui_telegram_desktop::app::mainwindow::{Mainwindow, WindowWorkMode};
use gpui_telegram_desktop::app::session::send_as_peers::{SendAsKey, SendAsPeer};
use gpui_telegram_desktop::layout::{
    AbstractLayoutItem, DocumentColorIndex, LayoutDocumentGenericPreview, LayoutMosaic,
    LayoutPosition, LayoutSelection, Position, TextSelection, FULL_SELECTION,
    MATRIX_ROW_SHIFT, MAX_GROUP_SELECTION_ITEMS,
};

#[test]
fn test_matrix_position_encoding_invariants() {
    assert_eq!(MATRIX_ROW_SHIFT, 40000);

    // Roundtrip verification across various row/column pairs
    let cases = vec![
        (0, 0, 0),
        (0, 1, 1),
        (0, 39999, 39999),
        (1, 0, 40000),
        (1, 15, 40015),
        (10, 250, 400250),
        (100, 5, 4000005),
    ];

    for (row, col, expected_index) in cases {
        let index = LayoutPosition::position_to_index(row, col);
        assert_eq!(index, expected_index);

        let decoded = LayoutPosition::index_to_position(index);
        assert_eq!(decoded.row, row);
        assert_eq!(decoded.column, col);
        assert!(decoded.is_valid());
    }

    // Negative index represents invalid/removed position
    let invalid = LayoutPosition::index_to_position(-1);
    assert_eq!(invalid, Position::INVALID);
    assert!(!invalid.is_valid());
}

#[test]
fn test_grouped_album_selection_bitmask_math() {
    assert_eq!(MAX_GROUP_SELECTION_ITEMS, 15);
    assert_eq!(FULL_SELECTION, TextSelection::FULL);

    let mut sel = TextSelection::EMPTY;
    assert!(!LayoutSelection::is_sub_group_selection(sel));
    assert_eq!(LayoutSelection::first_group_item_index(sel), None);

    // Selecting item 0 and item 3 in an album
    sel = LayoutSelection::add_group_item_selection(sel, 0);
    assert!(LayoutSelection::is_sub_group_selection(sel));
    assert!(LayoutSelection::is_group_item_selection(sel, 0));
    assert!(!LayoutSelection::is_group_item_selection(sel, 1));
    assert_eq!(LayoutSelection::first_group_item_index(sel), Some(0));

    sel = LayoutSelection::add_group_item_selection(sel, 3);
    assert!(LayoutSelection::is_group_item_selection(sel, 0));
    assert!(LayoutSelection::is_group_item_selection(sel, 3));
    assert_eq!(LayoutSelection::first_group_item_index(sel), Some(0));

    // Removing item 0 leaves item 3
    sel = LayoutSelection::remove_group_item_selection(sel, 0);
    assert!(!LayoutSelection::is_group_item_selection(sel, 0));
    assert!(LayoutSelection::is_group_item_selection(sel, 3));
    assert_eq!(LayoutSelection::first_group_item_index(sel), Some(3));

    // Removing item 3 leaves empty selection
    sel = LayoutSelection::remove_group_item_selection(sel, 3);
    assert_eq!(sel, TextSelection::EMPTY);
    assert!(!LayoutSelection::is_sub_group_selection(sel));
}

#[test]
fn test_mosaic_layout_algorithm_and_hit_testing() {
    let mut mosaic = LayoutMosaic::new(320);
    mosaic.set_right_skip(4);
    mosaic.set_padding(8, 8, 8, 8);

    let items = vec![
        AbstractLayoutItem::new(100, 60),
        AbstractLayoutItem::new(100, 60),
        AbstractLayoutItem::new(100, 60),
        AbstractLayoutItem::new(100, 60),
    ];

    mosaic.add_items(items);
    assert_eq!(mosaic.rows_count(), 1); // 4 items fit in one row (<5 items per row and ~320px)

    let total_h = mosaic.count_desired_height(320);
    assert!(total_h >= 60 + 16);

    // Hit test first item
    let hit0 = mosaic.find_by_point(15, 15);
    assert!(hit0.exact);
    assert_eq!(hit0.index, 0);

    // Bounding rectangle check
    let rect0 = mosaic.find_rect(0);
    assert!(rect0.is_some());
    let (rx, ry, rw, rh) = rect0.unwrap();
    assert_eq!((rx, ry), (8, 8)); // padding left and top
    assert!(rw > 0);
    assert_eq!(rh, 60);
}

#[test]
fn test_document_generic_preview_four_color_categorization() {
    // 0: Blue (Documents, Text, PSD)
    let docx = LayoutDocumentGenericPreview::create("Contract.DOCX");
    assert_eq!(docx.color_palette(), DocumentColorIndex::Blue);
    assert_eq!(docx.ext, "docx");

    let txt = LayoutDocumentGenericPreview::create_with_mime("readme.md", Some("text/plain"));
    assert_eq!(txt.color_palette(), DocumentColorIndex::Blue);

    // 1: Green (Spreadsheets)
    let xlsx = LayoutDocumentGenericPreview::create("Finance_Q3.XLSX");
    assert_eq!(xlsx.color_palette(), DocumentColorIndex::Green);

    let csv = LayoutDocumentGenericPreview::create("data.csv");
    assert_eq!(csv.color_palette(), DocumentColorIndex::Green);

    // 2: Red (PDF, Presentation, Keynote)
    let pdf = LayoutDocumentGenericPreview::create("Whitepaper.pdf");
    assert_eq!(pdf.color_palette(), DocumentColorIndex::Red);

    let ppt = LayoutDocumentGenericPreview::create("PitchDeck.pptx");
    assert_eq!(ppt.color_palette(), DocumentColorIndex::Red);

    // 3: Yellow (Archives, Media)
    let zip = LayoutDocumentGenericPreview::create("Sources.zip");
    assert_eq!(zip.color_palette(), DocumentColorIndex::Yellow);

    let mp3 = LayoutDocumentGenericPreview::create("AudioTrack.mp3");
    assert_eq!(mp3.color_palette(), DocumentColorIndex::Yellow);

    // Fallback modulo classification
    let unknown = LayoutDocumentGenericPreview::create("file.xyz");
    assert!(unknown.index < 4);
}

#[test]
fn test_main_domain_multi_account_lifecycle() {
    let mut domain = MainDomain::new("data_dir".into());
    assert_eq!(MAX_ACCOUNTS, 3);
    assert_eq!(PREMIUM_MAX_ACCOUNTS, 6);

    domain.set_passcode(b"pass123");

    // Single-flight passcode check
    assert!(!domain.try_passcode(b"wrong"));
    assert!(!domain.started());

    assert!(domain.try_passcode(b"pass123"));
    assert!(domain.started());

    // Add 3 standard accounts
    domain.add_account(false).unwrap();
    domain.add_account(false).unwrap();
    domain.add_account(false).unwrap();
    assert_eq!(domain.accounts.len(), 3);
    assert!(domain.add_account(false).is_err());

    // Unread badge updates
    domain.notify_unread_badge_changed(15, false);
    assert_eq!(domain.unread_badge(), (15, false));
}

#[test]
fn test_main_session_and_send_as_peers() {
    let mut session = MainSession::new(424242, false);
    assert_eq!(session.user_id(), 424242);
    assert_eq!(session.unique_id(), 424242);

    // Freeze info
    let freeze = FreezeInfo {
        since: 1700000000,
        until: 1700100000,
        appeal_url: "https://telegram.org/appeal".into(),
    };
    session.set_freeze_info(freeze);
    assert!(session.is_frozen());

    // Send As Peers
    let key = SendAsKey::message(999);
    session.send_as_peers.set_list(
        key,
        vec![
            SendAsPeer::new(424242, "Me".into(), false),
            SendAsPeer::new(888888, "Channel Admin".into(), false),
        ],
    );
    assert!(session.send_as_peers.should_choose(&key));
    session.send_as_peers.set_chosen(999, 888888);
    assert_eq!(session.send_as_peers.resolve_chosen(&key), 888888);
}

#[test]
fn test_mainwidget_responsive_layout_modes() {
    let mut widget = Mainwidget::new(1440, 900);
    assert_eq!(widget.layout_mode, LayoutMode::ThreeColumn);

    let (dx, dy, dw, dh) = widget.dialogs_rect();
    assert_eq!((dx, dy, dw, dh), (0, 0, 360, 900));

    // Two column
    widget.resize(1000, 800);
    assert_eq!(widget.layout_mode, LayoutMode::TwoColumn);
    assert_eq!(widget.dialogs_width, 320);

    // One column
    widget.resize(600, 800);
    assert_eq!(widget.layout_mode, LayoutMode::OneColumn);
    assert_eq!(widget.dialogs_rect().2, 600);

    widget.show_history(777, None);
    assert_eq!(widget.dialogs_rect().2, 0);
    assert_eq!(widget.history_rect().2, 600);

    assert!(widget.show_back_from_stack());
    assert_eq!(widget.current_peer_id, None);
}

#[test]
fn test_mainwindow_work_modes_and_layer_stack() {
    let mut window = Mainwindow::new(1280, 800);
    assert_eq!(window.work_mode(), WindowWorkMode::MainContent);

    window.setup_passcode_lock();
    assert_eq!(window.work_mode(), WindowWorkMode::PasscodeLock);

    window.clear_passcode_lock();
    assert_eq!(window.work_mode(), WindowWorkMode::MainContent);

    window.setup_intro();
    assert_eq!(window.work_mode(), WindowWorkMode::Intro);

    window.setup_main();
    assert_eq!(window.work_mode(), WindowWorkMode::MainContent);

    window.push_layer("MediaViewer".into());
    assert!(window.is_layer_shown());
    assert!(window.pop_layer());
    assert!(!window.is_layer_shown());
}

#[test]
fn test_authentic_send_details_options() {
    use gpui_telegram_desktop::views::app_shell::{
        CaptionState, MenuSendDetails, SendDetails, SendMenuType,
    };

    let mut details = SendDetails::new();
    assert_eq!(details.send_type, SendMenuType::Disabled);
    assert!(!details.is_silent());
    assert!(!details.is_scheduled());

    // Silent send configuration
    details.set_silent(true);
    assert!(details.is_silent());
    assert_eq!(details.send_type, SendMenuType::SilentOnly);

    // Scheduled send configuration
    details.set_scheduled(Some(1730000000));
    assert!(details.is_scheduled());
    assert_eq!(details.send_type, SendMenuType::Scheduled);

    // Send As Peer selection
    details.set_send_as(Some(123456789));
    assert_eq!(details.send_as_peer_id, Some(123456789));

    // Caption inversion
    assert!(!details.invert_caption);
    details.caption = CaptionState::Below;
    details.toggle_invert_caption();
    assert!(details.invert_caption);
    assert_eq!(details.caption, CaptionState::Above);

    // View component wrapping
    let mut menu = MenuSendDetails::with_details(details);
    assert!(menu.is_open);
    assert!(menu.details().is_scheduled());
    menu.close();
    assert!(!menu.is_open);
}

#[test]
fn test_authentic_support_preload_cache() {
    use gpui_telegram_desktop::views::app_shell::SupportPreload;

    let mut support = SupportPreload::new();
    assert!(!support.cached_snippets.is_empty());

    // Preload history simulation
    let count = support.preload_history(1001, 500);
    assert_eq!(count, 50);
    assert!(support.is_peer_preloaded(1001));
    assert!(!support.is_peer_preloaded(1002));

    // Fast reply snippet lookup & search
    let welcome = support.lookup_snippet("/welcome");
    assert!(welcome.is_some());
    assert!(welcome.unwrap().tags.contains(&"greeting".to_string()));

    support.add_snippet("/custom", "Custom FAQ", "Please check settings.", &["faq"]);
    let found = support.search_snippets("settings");
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].shortcut, "/custom");
}

#[test]
fn test_authentic_window_section_transitions_and_gestures() {
    use gpui_telegram_desktop::views::app_shell::window_section_common::SlideDirection;
    use gpui_telegram_desktop::views::app_shell::{
        SectionActionResult, SectionSlideParams, SectionTransition, WindowSectionCommon,
    };

    let params = SectionSlideParams::default_forward().with_duration(200.0);
    assert_eq!(params.direction, SlideDirection::FromRight);
    assert_eq!(params.duration_ms, 200.0);

    let mut section = WindowSectionCommon::new();
    assert!(!section.is_in_transition());

    section.start_transition(params);
    assert!(section.is_in_transition());

    // Step animation
    section.tick(100.0);
    assert!(section.is_in_transition());
    section.tick(150.0);
    assert!(!section.is_in_transition());

    // Gesture back handling
    let back_params = SectionSlideParams::default_back();
    let mut transition = SectionTransition::new(back_params);
    transition.start_gesture_back();
    assert!(transition.gesture_back_active);

    // Drag 100px on 400px width = 25% (< 35% threshold -> should not commit)
    transition.update_gesture_back(100.0, 400.0);
    assert_eq!(transition.current_progress, 0.25);
    assert!(!transition.commit_gesture_back());

    // Drag 200px on 400px width = 50% (>= 35% threshold -> commits back)
    transition.start_gesture_back();
    transition.update_gesture_back(200.0, 400.0);
    assert!(transition.commit_gesture_back());
    assert_eq!(transition.current_progress, 1.0);

    // SectionActionResult handling
    section.handle_action(SectionActionResult::Fallback);
    assert_eq!(section.last_action_result, SectionActionResult::Fallback);
}

#[test]
fn test_authentic_link_info_deep_link_resolution() {
    use gpui_telegram_desktop::views::app_shell::{
        LinkInfoType, ResolveType, WindowSessionControllerLinkInfo,
    };

    // 1. Resolve tg://resolve?domain=telegram&start=token123
    let res1 = WindowSessionControllerLinkInfo::parse_deep_link("tg://resolve?domain=telegram&start=token123");
    match res1 {
        LinkInfoType::Peer(peer) => {
            assert_eq!(peer.username_or_id, Some("telegram".to_string()));
            assert_eq!(peer.start_token, Some("token123".to_string()));
            assert_eq!(peer.resolve_type, ResolveType::BotStart);
        }
        _ => panic!("Expected Peer link"),
    }

    // 2. Resolve https://t.me/durov/123
    let res2 = WindowSessionControllerLinkInfo::parse_deep_link("https://t.me/durov/123");
    match res2 {
        LinkInfoType::Peer(peer) => {
            assert_eq!(peer.username_or_id, Some("durov".to_string()));
            assert_eq!(peer.message_id, Some(123));
        }
        _ => panic!("Expected Peer link"),
    }

    // 3. Resolve join invite link tg://join?invite=AbCdEfGh
    let res3 = WindowSessionControllerLinkInfo::parse_deep_link("tg://join?invite=AbCdEfGh");
    match res3 {
        LinkInfoType::Peer(peer) => {
            assert_eq!(peer.chat_link_slug, Some("AbCdEfGh".to_string()));
            assert!(peer.join_channel);
        }
        _ => panic!("Expected Peer link"),
    }

    // 4. Controller history tracking
    let mut controller = WindowSessionControllerLinkInfo::new();
    controller.open_link("https://t.me/telegram");
    assert_eq!(controller.history.len(), 1);
    assert!(controller.current_link().is_some());
    controller.clear();
    assert!(controller.current_link().is_none());
    assert_eq!(controller.history.len(), 0);
}
