//! FUS-D003 regression tests: truthful initial and peer-selection state for `ContextInspector`.

use gpui_telegram_desktop::views::{
    ContextInspector, InspectorTab, MemberItem, SharedFileItem, SharedLinkItem, SharedMediaSubTab,
};

#[test]
fn new_and_default_inspector_start_closed_with_empty_facts() {
    for inspector in [ContextInspector::new(), ContextInspector::default()] {
        assert!(!inspector.is_open, "Inspector drawer must start closed");
        assert_eq!(inspector.active_tab, InspectorTab::Profile);

        assert!(inspector.profile.name.is_empty());
        assert!(inspector.profile.username.is_empty());
        assert!(inspector.profile.phone.is_empty());
        assert!(inspector.profile.bio.is_empty());
        assert!(inspector.profile.avatar_initials.is_empty());
        assert_eq!(inspector.profile.avatar_color, 0);
        assert!(inspector.profile.is_notifications_enabled);

        assert!(inspector.members.members.is_empty());
        assert!(inspector.members.search_query.is_empty());

        assert_eq!(inspector.shared_media.active_subtab, SharedMediaSubTab::Media);
        assert_eq!(inspector.shared_media.media_count, 0);
        assert!(inspector.shared_media.files.is_empty());
        assert!(inspector.shared_media.links.is_empty());

        assert!(inspector.settings.e2ee_fingerprint.is_empty());
        assert_eq!(inspector.settings.auto_delete_period, "Off");
        assert_eq!(inspector.settings.notification_sound, "Default (Aurora)");

        assert!(!inspector.is_secret_chat_active);
        assert!(!inspector.is_contact_blocked);
        assert!(inspector.pending_media_open.is_none());
    }
}

#[test]
fn cloud_workspace_reset_clears_populated_state_without_fabricating_bot_or_url_phone() {
    let mut inspector = ContextInspector::new_demo();
    inspector.open();
    inspector.set_tab(InspectorTab::Settings);
    inspector.profile.is_notifications_enabled = false;
    inspector.members.search_query = "alice".to_string();
    inspector.shared_media.active_subtab = SharedMediaSubTab::Links;
    inspector.settings.auto_delete_period = "24 Hours".to_string();
    inspector.settings.notification_sound = "Silent".to_string();
    inspector.is_secret_chat_active = true;
    inspector.is_contact_blocked = true;
    inspector.pending_media_open = Some(("https://example.com/photo.jpg".to_string(), false));

    let server_url = "http://34.46.77.133:8080";
    inspector.configure_for_cloud_workspace(server_url);

    assert!(!inspector.is_open, "Cloud reset must close the inspector drawer");
    assert_eq!(inspector.active_tab, InspectorTab::Profile);

    assert!(inspector.profile.name.is_empty(), "Cloud reset must not fabricate a bot peer name");
    assert!(inspector.profile.username.is_empty(), "Cloud reset must not fabricate a bot username");
    assert!(inspector.profile.phone.is_empty(), "Cloud reset must not store server URL as phone");
    assert!(inspector.profile.bio.is_empty(), "Cloud reset must not fabricate connectivity bio");
    assert!(inspector.profile.avatar_initials.is_empty());
    assert_eq!(inspector.profile.avatar_color, 0);
    assert!(inspector.profile.is_notifications_enabled);

    assert!(inspector.members.members.is_empty(), "Cloud reset must clear member list");
    assert!(inspector.members.search_query.is_empty());

    assert_eq!(inspector.shared_media.active_subtab, SharedMediaSubTab::Media);
    assert_eq!(inspector.shared_media.media_count, 0);
    assert!(inspector.shared_media.files.is_empty());
    assert!(inspector.shared_media.links.is_empty());

    assert!(inspector.settings.e2ee_fingerprint.is_empty());
    assert_eq!(inspector.settings.auto_delete_period, "Off");
    assert_eq!(inspector.settings.notification_sound, "Default (Aurora)");

    assert!(!inspector.is_secret_chat_active);
    assert!(!inspector.is_contact_blocked);
    assert!(inspector.pending_media_open.is_none());
}

#[test]
fn set_peer_uses_only_supplied_display_facts_and_never_synthesizes_username() {
    let mut inspector = ContextInspector::new();
    inspector.set_peer("Rust Core Team", "12 members", "RC", 0xE67E22);

    assert_eq!(inspector.profile.name, "Rust Core Team");
    assert_eq!(inspector.profile.bio, "12 members");
    assert_eq!(inspector.profile.avatar_initials, "RC");
    assert_eq!(inspector.profile.avatar_color, 0xE67E22);
    assert!(
        inspector.profile.username.is_empty(),
        "set_peer must never synthesize a @username from display name"
    );
    assert!(inspector.profile.phone.is_empty());
}

#[test]
fn set_peer_clears_prior_peer_facts_on_selection_change_and_preserves_same_peer_facts() {
    let mut inspector = ContextInspector::new();
    inspector.open();
    inspector.set_peer("Alice Smith", "online", "AS", 0x2AABEE);

    // Populate authoritative facts for Alice
    inspector.profile.username = "@alice_verified".to_string();
    inspector.profile.phone = "+15550192834".to_string();
    inspector.profile.is_notifications_enabled = false;
    inspector.members.members.push(MemberItem {
        id: 10,
        name: "Alice Smith".to_string(),
        initials: "AS".to_string(),
        color: 0x2AABEE,
        role: "Owner".to_string(),
        is_online: true,
    });
    inspector.members.search_query = "Alice".to_string();
    inspector.shared_media.active_subtab = SharedMediaSubTab::Files;
    inspector.shared_media.media_count = 7;
    inspector.shared_media.files.push(SharedFileItem {
        name: "notes.txt".to_string(),
        size_text: "12 KB".to_string(),
        extension: "TXT".to_string(),
        date: "Today".to_string(),
    });
    inspector.shared_media.links.push(SharedLinkItem {
        title: "Spec".to_string(),
        url: "https://example.org/spec".to_string(),
        domain: "example.org".to_string(),
    });
    inspector.settings.e2ee_fingerprint = "AA BB CC DD".to_string();
    inspector.settings.auto_delete_period = "7 Days".to_string();
    inspector.settings.notification_sound = "Note".to_string();
    inspector.is_secret_chat_active = true;
    inspector.is_contact_blocked = true;
    inspector.pending_media_open = Some(("notes.txt".to_string(), false));

    // Re-selecting the same peer with an updated status subtitle preserves same-peer facts
    inspector.set_peer("Alice Smith", "last seen recently", "AS", 0x2AABEE);
    assert_eq!(inspector.profile.name, "Alice Smith");
    assert_eq!(inspector.profile.bio, "last seen recently");
    assert_eq!(inspector.profile.username, "@alice_verified");
    assert_eq!(inspector.profile.phone, "+15550192834");
    assert!(!inspector.profile.is_notifications_enabled);
    assert_eq!(inspector.members.members.len(), 1);
    assert_eq!(inspector.members.search_query, "Alice");
    assert_eq!(inspector.shared_media.media_count, 7);
    assert_eq!(inspector.shared_media.files.len(), 1);
    assert_eq!(inspector.shared_media.links.len(), 1);
    assert_eq!(inspector.settings.e2ee_fingerprint, "AA BB CC DD");
    assert_eq!(inspector.settings.auto_delete_period, "7 Days");
    assert_eq!(inspector.settings.notification_sound, "Note");
    assert!(inspector.is_secret_chat_active);
    assert!(inspector.is_contact_blocked);
    assert_eq!(
        inspector.pending_media_open.as_ref().map(|(url, video)| (url.as_str(), *video)),
        Some(("notes.txt", false))
    );

    // Switching to a different peer clears all prior peer facts
    inspector.set_peer("Bob Builder", "online", "BB", 0x9B59B6);
    assert_eq!(inspector.profile.name, "Bob Builder");
    assert_eq!(inspector.profile.bio, "online");
    assert_eq!(inspector.profile.avatar_initials, "BB");
    assert_eq!(inspector.profile.avatar_color, 0x9B59B6);
    assert!(inspector.profile.username.is_empty());
    assert!(inspector.profile.phone.is_empty());
    assert!(inspector.profile.is_notifications_enabled);
    assert!(inspector.members.members.is_empty());
    assert!(inspector.members.search_query.is_empty());
    assert_eq!(inspector.shared_media.active_subtab, SharedMediaSubTab::Media);
    assert_eq!(inspector.shared_media.media_count, 0);
    assert!(inspector.shared_media.files.is_empty());
    assert!(inspector.shared_media.links.is_empty());
    assert!(inspector.settings.e2ee_fingerprint.is_empty());
    assert_eq!(inspector.settings.auto_delete_period, "Off");
    assert_eq!(inspector.settings.notification_sound, "Default (Aurora)");
    assert!(!inspector.is_secret_chat_active);
    assert!(!inspector.is_contact_blocked);
    assert!(inspector.pending_media_open.is_none());

    // Clearing peer selection empties profile and closes drawer
    inspector.set_peer("", "", "", 0);
    assert!(!inspector.is_open);
    assert!(inspector.profile.name.is_empty());
    assert!(inspector.profile.bio.is_empty());
    assert!(inspector.profile.avatar_initials.is_empty());
    assert_eq!(inspector.profile.avatar_color, 0);
}
