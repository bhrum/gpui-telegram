//! Subsystem Integration Tests for Module 08 Slice 7: Peer List, Contacts, Privacy & Management Boxes
//!
//! Verifies:
//! - Mandate #15: Genuine implementations without fake/dummy facades or { label, value } boilerplate.
//! - Mandate #18: Exact C++ logic, constant values, fisheye math, debounced controllers, and privacy evaluation.

use gpui::px;
use gpui_telegram_desktop::views::boxes::add_contact_box::{
    AddContactBox, ContactValidationError, K_MAX_USER_FIRST_LAST_NAME,
};
use gpui_telegram_desktop::views::boxes::edit_privacy_box::{
    EditPrivacyBox, PrivacyCategory, PrivacyRuleOption, K_DEFAULT_PRIVATE_MESSAGES_PRICE,
    K_MINI_APPS_ROW_ID, K_PREMIUMS_ROW_ID,
};
use gpui_telegram_desktop::views::boxes::peer_list_box::PeerListBox;
use gpui_telegram_desktop::views::boxes::peer_list_controllers::{
    ContactSortMode, PeerListControllers, K_SEARCH_PER_PAGE, K_SORT_BY_ONLINE_THROTTLE,
};
use gpui_telegram_desktop::views::boxes::peer_list_section_headers::PeerListSectionHeaders;
use gpui_telegram_desktop::views::boxes::peer_list_section_index::{
    PeerListSectionIndex, SectionIndexLetter, K_BULGE_RESERVE_SLOTS, K_EASE,
    K_FISHEYE_REACH_SLOTS, K_MAX_BOOST,
};
use gpui_telegram_desktop::views::boxes::peer_list_widgets::{
    PeerListRow, PeerListWidgets, PeerRowStatus,
};
use gpui_telegram_desktop::views::boxes::peer_lists_box::{PeerListsBox, PeerSubList};
use gpui_telegram_desktop::views::boxes::peers::edit_peer_permissions_box::{
    EditPeerPermissionsBoxPeers, K_SLOWMODE_SECONDS,
};
use gpui_telegram_desktop::views::boxes::peers::*;
use gpui_telegram_desktop::views::boxes::report_messages_box::{
    ReportMessagesBox, ReportReason,
};
use gpui_telegram_desktop::views::boxes::select_future_owner_box::{
    OwnerCandidate, SelectFutureOwnerBox,
};

#[test]
fn test_section_index_fisheye_math() {
    let mut index = PeerListSectionIndex::new();
    assert_eq!(K_MAX_BOOST, 1.8);
    assert_eq!(K_EASE, 0.6);
    assert_eq!(K_FISHEYE_REACH_SLOTS, 2.2);
    assert_eq!(K_BULGE_RESERVE_SLOTS, 2.5);

    let letters = vec![
        SectionIndexLetter { letter: 'A', content_top: px(0.0) },
        SectionIndexLetter { letter: 'B', content_top: px(100.0) },
        SectionIndexLetter { letter: 'C', content_top: px(200.0) },
        SectionIndexLetter { letter: 'D', content_top: px(300.0) },
    ];
    index.set_letters(letters);
    assert_eq!(index.slots.len(), 4);

    // Mouse over slot 1 ('B') at pitch 16px -> center_y = 24px
    index.update_fisheye(px(24.0));
    let slot_b = &index.slots[1];
    assert!(slot_b.target_scale > 1.0);
    assert!(slot_b.current_scale > 1.0);
    assert!(slot_b.target_scale <= K_MAX_BOOST);

    // Jump callback returns top position
    let jumped = index.jump_at_y(px(24.0));
    assert_eq!(jumped, Some(px(100.0)));
}

#[test]
fn test_section_headers_sticky_positioning() {
    let mut headers = PeerListSectionHeaders::new();
    let entries = vec![
        ("A".to_string(), 5, px(0.0)),
        ("B".to_string(), 12, px(150.0)),
        ("C".to_string(), 8, px(350.0)),
    ];
    headers.rebuild(entries);
    assert!(headers.has_header());

    assert_eq!(headers.update_active(px(50.0)).unwrap().title, "A");
    assert_eq!(headers.update_active(px(150.0)).unwrap().title, "B");
    assert_eq!(headers.update_active(px(200.0)).unwrap().title, "B");
    assert_eq!(headers.update_active(px(400.0)).unwrap().title, "C");
}

#[test]
fn test_peer_list_widgets_virtual_rows() {
    let mut widgets = PeerListWidgets::new();
    let row1 = PeerListRow::new(101, "Alice".to_string(), PeerRowStatus::Online);
    let row2 = PeerListRow::new(102, "Bob".to_string(), PeerRowStatus::Offline);

    widgets.append_row(row1);
    widgets.append_row(row2);
    assert_eq!(widgets.row_count(), 2);

    assert_eq!(widgets.find_row(101).unwrap().title, "Alice");
    assert!(widgets.set_row_checked(101, true));
    assert_eq!(widgets.find_row(101).unwrap().checked_ratio, 1.0);
    assert_eq!(widgets.collect_checked_ids(), vec![101]);
}


#[test]
fn test_peer_list_controllers_sorting_and_debounce() {
    let mut ctrl = PeerListControllers::new();
    assert_eq!(K_SORT_BY_ONLINE_THROTTLE.as_millis(), 3000);
    assert_eq!(K_SEARCH_PER_PAGE, 50);

    let mut rows = vec![
        PeerListRow::new(1, "Charlie".to_string(), PeerRowStatus::Offline),
        PeerListRow::new(2, "Alice".to_string(), PeerRowStatus::Online),
        PeerListRow::new(3, "Bob".to_string(), PeerRowStatus::LastSeenRecently),
    ];

    // Alphabetical sort
    ctrl.set_sort_mode(ContactSortMode::Name, 1000);
    ctrl.sort_rows(&mut rows);
    assert_eq!(rows[0].title, "Alice");
    assert_eq!(rows[1].title, "Bob");
    assert_eq!(rows[2].title, "Charlie");

    // Online status sort: Online -> LastSeenRecently -> Offline
    ctrl.set_sort_mode(ContactSortMode::Online, 2000);
    ctrl.sort_rows(&mut rows);
    assert_eq!(rows[0].title, "Alice");
    assert_eq!(rows[1].title, "Bob");
    assert_eq!(rows[2].title, "Charlie");

    // Search cache handling
    assert!(ctrl.handle_search_query("ali").is_none());
    ctrl.cache_search_results("ali".to_string(), vec![2]);
    assert_eq!(ctrl.handle_search_query("ali"), Some(&vec![2]));
}

#[test]
fn test_peer_list_box_multi_select_metrics() {
    let mut box_view = PeerListBox::new("Select Friends".to_string(), true);
    box_view.add_row(PeerListRow::new(10, "Dev1".to_string(), PeerRowStatus::Online));
    box_view.add_row(PeerListRow::new(20, "Dev2".to_string(), PeerRowStatus::Online));

    assert_eq!(box_view.multi_select_height, px(0.0));
    assert_eq!(box_view.top_scroll_skip(), px(0.0));

    // Toggle row selection
    assert!(box_view.toggle_row_selection(10));
    assert_eq!(box_view.selected_count(), 1);
    assert!(box_view.multi_select_height > px(0.0));
    assert_eq!(box_view.selected_ids(), vec![10]);

    // Above search mode adds search height
    box_view.above_search = true;
    assert_eq!(box_view.top_scroll_skip(), box_view.search_height + box_view.multi_select_height);
}

#[test]
fn test_peer_lists_box_tabbed_composite() {
    let mut multi = PeerListsBox::new("Choose Chats".to_string());
    let mut list1 = PeerSubList::new("Contacts".to_string());
    list1.widgets.append_row(PeerListRow::new(100, "Contact1".to_string(), PeerRowStatus::Online));
    let mut list2 = PeerSubList::new("Recent".to_string());
    list2.widgets.append_row(PeerListRow::new(200, "Recent1".to_string(), PeerRowStatus::Offline));

    multi.add_sub_list(list1);
    multi.add_sub_list(list2);

    assert_eq!(multi.active_sub_list().unwrap().title, "Contacts");
    multi.set_active_tab(1);
    assert_eq!(multi.active_sub_list().unwrap().title, "Recent");

    assert!(multi.toggle_row_selection(0, 100));
    assert_eq!(multi.collect_selected_ids(), vec![100]);
}

#[test]
fn test_add_contact_validation_boundaries() {
    let mut box_view = AddContactBox::new();
    assert_eq!(K_MAX_USER_FIRST_LAST_NAME, 64);

    // Empty first name error
    box_view.set_fields("", "Smith", "+1234567890");
    assert_eq!(box_view.error, Some(ContactValidationError::EmptyFirstName));
    assert!(!box_view.can_submit());

    // Invalid short phone
    box_view.set_fields("John", "Doe", "123");
    assert_eq!(box_view.error, Some(ContactValidationError::InvalidPhoneNumber));

    // Valid contact
    box_view.set_fields("John", "Doe", "+1 (234) 567-8901");
    assert_eq!(box_view.error, None);
    assert!(box_view.can_submit());
}

#[test]
fn test_edit_privacy_box_rule_eval() {
    let mut priv_box = EditPrivacyBox::new(PrivacyCategory::LastSeen);
    assert_eq!(K_PREMIUMS_ROW_ID, 0x7fff_ffff_0000_0001);
    assert_eq!(K_MINI_APPS_ROW_ID, 0x7fff_ffff_0000_0002);
    assert_eq!(K_DEFAULT_PRIVATE_MESSAGES_PRICE, 10);

    // Default Contacts rule
    assert!(priv_box.is_allowed_for(42, true, false));
    assert!(!priv_box.is_allowed_for(43, false, false));

    // Whitelist override
    priv_box.add_always_allow(43);
    assert!(priv_box.is_allowed_for(43, false, false));

    // Blacklist override takes precedence
    priv_box.add_never_allow(42);
    assert!(!priv_box.is_allowed_for(42, true, true));

    // Premium only rule
    priv_box.set_rule(PrivacyRuleOption::Premium);
    assert!(priv_box.is_allowed_for(99, false, true));
    assert!(!priv_box.is_allowed_for(98, false, false));
}

#[test]
fn test_report_messages_and_future_owner_boxes() {
    let mut report = ReportMessagesBox::new(555, vec![101, 102]);
    assert!(!report.can_submit());

    report.select_reason(ReportReason::Spam);
    assert!(report.can_submit());

    report.select_reason(ReportReason::Other);
    assert!(!report.can_submit());
    report.set_comment("Violates terms");
    assert!(report.can_submit());

    let mut owner_box = SelectFutureOwnerBox::new(777);
    owner_box.set_candidates(vec![
        OwnerCandidate {
            user_id: 1,
            name: "Admin Alice".to_string(),
            username: Some("alice".to_string()),
            is_admin: true,
            is_bot: false,
        },
        OwnerCandidate {
            user_id: 2,
            name: "Moderator Bot".to_string(),
            username: Some("bot".to_string()),
            is_admin: true,
            is_bot: true, // Should be filtered out
        },
    ]);
    assert_eq!(owner_box.candidates.len(), 1);
    assert!(owner_box.select_candidate(1));
    assert!(owner_box.check_security_requirements().is_ok());

    owner_box.has_2fa_for_7_days = false;
    assert!(owner_box.check_security_requirements().is_err());
}

#[test]
fn test_peers_specialized_boxes() {
    let mut perm = EditPeerPermissionsBoxPeers::new(888);
    assert_eq!(K_SLOWMODE_SECONDS.len(), 7);
    perm.set_slowmode(60);
    assert_eq!(perm.slowmode_seconds, 60);

    let mut part = AddParticipantsBoxPeers::new(888, false);
    assert!(part.toggle_selection(11));
    assert_eq!(part.selected_count(), 1);
    assert!(part.can_submit());

    let mut color = EditPeerColorBoxPeers::new(888);
    color.set_color_index(8);
    assert_eq!(color.required_boost_level, 2);
    assert!(!color.can_apply());
    color.current_boost_level = 3;
    assert!(color.can_apply());
}
