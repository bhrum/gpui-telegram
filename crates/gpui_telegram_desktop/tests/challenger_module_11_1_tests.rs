//! Empirical Challenger 1 Adversarial Stress Suite for Module 11 Iteration 2 Re-Gate
//! Subsystem: Info Drawer Tab Adapters, State Transitions & Profile Back Button

use std::sync::{Arc, RwLock};
use std::thread;

use gpui_telegram_desktop::views::drawer::profile_back_button::ProfileBackButton;
use gpui_telegram_desktop::views::drawer::profile::tabs::adapters::{
    info_profile_tab_media::MediaGridTile,
    info_profile_tab_members::MemberRosterItem,
    info_profile_tab_peer_lists::PeerRowItem,
    info_profile_tab_polls::{PollCard, PollOptionItem},
    info_profile_tab_saved::{SavedSublistItem, SavedTagFilter},
    info_profile_tab_stories::StoryItem,
    InfoProfileTabMediaProfileTabsAdapters, InfoProfileTabMembersProfileTabsAdapters,
    InfoProfileTabPeerListsProfileTabsAdapters, InfoProfileTabPollsProfileTabsAdapters,
    InfoProfileTabSavedProfileTabsAdapters, InfoProfileTabStoriesProfileTabsAdapters,
    InfoProfileTabSubControllerProfileTabsAdapters,
};
use gpui_telegram_desktop::views::drawer::profile::tabs::info_profile_tab_content::ProfileTabKind;
use gpui_telegram_desktop::views::drawer::profile::tabs::info_profile_tab_top_bar_bindings::InfoProfileTabTopBarBindingsProfileTabs;
use gpui_telegram_desktop::views::drawer::profile::tabs::info_profile_tabs_host::InfoProfileTabsHostProfileTabs;


// ============================================================================
// 1. Profile Back Button Empirical Adversarial Testing
// ============================================================================

#[test]
fn test_empirical_profile_back_button_text_elision_and_extremes() {
    let mut btn = ProfileBackButton::new();
    assert_eq!(btn.circle_radius, 18.0);
    assert_eq!(btn.opacity, 1.0);
    assert_eq!(btn.cached_width, -1.0);
    assert_eq!(btn.elision_width, 240.0);

    // 1. Text elision boundary conditions
    btn.set_text("Short");
    assert_eq!(btn.elided_text(10), "Short"); // text <= max_chars
    assert_eq!(btn.elided_text(5), "Short");   // exactly max_chars
    assert_eq!(btn.elided_text(4), "S...");    // 1 char over -> take(1) + "..."
    assert_eq!(btn.elided_text(3), "Short");   // max_chars <= 3 fallback as-is
    assert_eq!(btn.elided_text(2), "Short");   // max_chars <= 3 fallback as-is
    assert_eq!(btn.elided_text(0), "Short");   // 0 chars fallback as-is

    // 2. Extremely long string (10,000 characters)
    let long_str = "A".repeat(10_000);
    btn.set_text(long_str);
    let elided_50 = btn.elided_text(50);
    assert_eq!(elided_50.chars().count(), 50);
    assert!(elided_50.ends_with("..."));
    assert_eq!(&elided_50[..47], &"A".repeat(47));

    // 3. Multi-byte UTF-8, CJK, and Emojis
    btn.set_text("电报桌面客户端 🚀 Telegram Desktop Client");
    // Verify character count rather than byte count
    let _char_count = btn.text.chars().count();
    let elided_unicode = btn.elided_text(12);
    assert_eq!(elided_unicode.chars().count(), 12);
    assert!(elided_unicode.ends_with("..."));

    // Pure emoji string
    btn.set_text("🔥🎉🦀💻🌟✨🎯📦");
    assert_eq!(btn.text.chars().count(), 8);
    let elided_emoji = btn.elided_text(6);
    assert_eq!(elided_emoji.chars().count(), 6);
    assert!(elided_emoji.ends_with("..."));

    // 4. Available width and elision width calculations
    // Default circle_radius = 18.0 -> icon_reserved = 18 * 2 + 16 = 52.0
    btn.update_elision(300.0);
    assert_eq!(btn.cached_width, 300.0);
    assert_eq!(btn.elision_width, 248.0); // 300 - 52 = 248

    // Width less than reserved space: clamped to safety floor 40.0
    btn.update_elision(60.0);
    assert_eq!(btn.elision_width, 40.0); // (60 - 52 = 8) -> clamped to 40.0

    // Extreme negative / zero widths
    btn.update_elision(0.0);
    assert_eq!(btn.elision_width, 40.0);
    btn.update_elision(-500.0);
    assert_eq!(btn.elision_width, 40.0);

    // Custom circle radius clamping
    btn.set_circle_radius(25.0);
    assert_eq!(btn.circle_radius, 25.0);
    btn.update_elision(300.0);
    assert_eq!(btn.elision_width, 234.0); // 300 - (50 + 16) = 234

    // Radius floor safety (min 8.0)
    btn.set_circle_radius(2.0);
    assert_eq!(btn.circle_radius, 8.0);
    btn.set_circle_radius(-10.0);
    assert_eq!(btn.circle_radius, 8.0);
}

#[test]
fn test_empirical_profile_back_button_fast_clicks_and_hover_transitions() {
    let mut btn = ProfileBackButton::with_text("Channel Details");
    btn.set_subtext("9,999 members");

    assert_eq!(btn.text, "Channel Details");
    assert_eq!(btn.subtext.as_deref(), Some("9,999 members"));

    // Cache invalidation on text/subtext changes
    btn.update_elision(250.0);
    assert_eq!(btn.cached_width, 250.0);
    btn.set_text("Updated Channel");
    assert_eq!(btn.cached_width, -1.0);

    btn.update_elision(250.0);
    btn.set_subtext("10,000 members");
    assert_eq!(btn.cached_width, -1.0);

    btn.update_elision(250.0);
    btn.clear_subtext();
    assert_eq!(btn.cached_width, -1.0);
    assert!(btn.subtext.is_none());

    // Rapid click simulation (10,000 rapid mouse-down / mouse-up cycles)
    for _ in 0..10_000 {
        btn.set_pressed(true);
        assert!(btn.is_pressed);
        assert_eq!(btn.hover_opacity, 0.15);

        btn.set_pressed(false);
        assert!(!btn.is_pressed);
    }

    // Hover + Press state matrix
    // Case 1: unhovered, unpressed
    btn.set_hovered(false);
    btn.set_pressed(false);
    assert_eq!(btn.hover_opacity, 0.0);

    // Case 2: hovered, unpressed
    btn.set_hovered(true);
    assert_eq!(btn.hover_opacity, 0.08);

    // Case 3: hovered, pressed
    btn.set_pressed(true);
    assert_eq!(btn.hover_opacity, 0.15);

    // Case 4: release while still hovered
    btn.set_pressed(false);
    assert_eq!(btn.hover_opacity, 0.08);

    // Case 5: unhover
    btn.set_hovered(false);
    assert_eq!(btn.hover_opacity, 0.0);

    // Case 6: press without hover (touch / mouse drag)
    btn.set_pressed(true);
    assert_eq!(btn.hover_opacity, 0.15);
    btn.set_pressed(false);
    assert_eq!(btn.hover_opacity, 0.0);

    // Opacity bounds clamping
    btn.set_opacity(1.0);
    assert_eq!(btn.opacity, 1.0);
    btn.set_opacity(1.5);
    assert_eq!(btn.opacity, 1.0);
    btn.set_opacity(-0.2);
    assert_eq!(btn.opacity, 0.0);
    btn.set_opacity(0.42);
    assert_eq!(btn.opacity, 0.42);
}


// ============================================================================
// 2. Drawer Tab State Transitions & Sub-Controller Coordination
// ============================================================================

#[test]
fn test_empirical_drawer_tab_state_transitions_with_sub_controller() {
    let all_kinds = vec![
        ProfileTabKind::Media,
        ProfileTabKind::Files,
        ProfileTabKind::Links,
        ProfileTabKind::Audio,
        ProfileTabKind::Voice,
        ProfileTabKind::Gifs,
        ProfileTabKind::Stories,
        ProfileTabKind::Members,
        ProfileTabKind::SimilarChannels,
        ProfileTabKind::Polls,
        ProfileTabKind::SavedSublists,
        ProfileTabKind::Gifts,
    ];

    let mut host = InfoProfileTabsHostProfileTabs::new(all_kinds.clone());
    assert_eq!(host.active_tab, ProfileTabKind::Media);

    let mut sub_ctrl = InfoProfileTabSubControllerProfileTabsAdapters::new();
    assert_eq!(sub_ctrl.current_scroll_top, 0.0);
    assert!(sub_ctrl.is_at_top());
    assert_eq!(sub_ctrl.collapse_ratio(), 0.0);

    // Permutation walk across all 12 drawer tabs with scroll state memoization
    let tab_scroll_positions = [
        (ProfileTabKind::Media, "media", 350.0),
        (ProfileTabKind::Members, "members", 120.0),
        (ProfileTabKind::Files, "files", 600.0),
        (ProfileTabKind::Stories, "stories", 850.0),
        (ProfileTabKind::Polls, "polls", 40.0),
        (ProfileTabKind::SavedSublists, "saved", 210.0),
        (ProfileTabKind::Gifts, "gifts", 0.0),
    ];

    for &(tab, tab_id, target_scroll) in &tab_scroll_positions {
        if host.active_tab != tab {
            assert!(host.switch_to_tab(tab));
            assert_eq!(host.active_tab, tab);
        }
        // Idempotency: switching to already active tab returns false
        assert!(!host.switch_to_tab(tab));

        // Sub-controller scroll simulation for this tab
        sub_ctrl.set_scroll_top(target_scroll);
        assert_eq!(sub_ctrl.current_scroll_top, target_scroll);
        sub_ctrl.save_tab_scroll(tab_id, target_scroll);
    }

    // Return to earlier tabs and verify scroll restoration
    for &(tab, tab_id, expected_scroll) in &tab_scroll_positions {
        if host.active_tab != tab {
            assert!(host.switch_to_tab(tab));
        }
        let restored = sub_ctrl.restore_tab_scroll(tab_id);
        assert_eq!(restored, expected_scroll, "Scroll mismatch for tab {:?}", tab);
        sub_ctrl.set_scroll_top(restored);
        assert_eq!(sub_ctrl.current_scroll_top, expected_scroll);
    }

    // Header collapse ratio stress boundaries
    // header_max_height = 240, header_min_height = 56 -> max_collapse = 184
    sub_ctrl.set_scroll_top(0.0);
    assert_eq!(sub_ctrl.collapse_ratio(), 0.0);
    assert!(sub_ctrl.is_at_top());

    sub_ctrl.set_scroll_top(92.0); // 92 / 184 = 0.5
    assert!((sub_ctrl.collapse_ratio() - 0.5).abs() < 1e-4);
    assert!(!sub_ctrl.is_at_top());

    sub_ctrl.set_scroll_top(184.0); // Exactly at max collapse
    assert_eq!(sub_ctrl.collapse_ratio(), 1.0);

    sub_ctrl.set_scroll_top(5000.0); // Past max collapse -> clamped to 1.0
    assert_eq!(sub_ctrl.collapse_ratio(), 1.0);
    assert_eq!(sub_ctrl.header_collapse_offset, 184.0);

    // Negative scroll clamped to 0.0
    sub_ctrl.set_scroll_top(-100.0);
    assert_eq!(sub_ctrl.current_scroll_top, 0.0);
    assert_eq!(sub_ctrl.collapse_ratio(), 0.0);

    // Viewport and bottom detection
    sub_ctrl.update_scroll_state(600.0, 500.0, 1200.0); // remaining = 1200 - (600 + 500) = 100
    assert!(sub_ctrl.is_at_bottom(100.0));
    assert!(sub_ctrl.is_at_bottom(150.0));
    assert!(!sub_ctrl.is_at_bottom(50.0));
}


// ============================================================================
// 3. Tab Adapters Specialized Domain State Transitions
// ============================================================================

#[test]
fn test_empirical_tab_adapters_specialized_domain_state_transitions() {
    // A. Media Adapter Transitions
    let mut media = InfoProfileTabMediaProfileTabsAdapters::new(1001);
    assert_eq!(media.peer_id, 1001);
    assert_eq!(media.total_media_count, 0);
    assert!(!media.is_active);

    media.set_active(true);
    assert!(media.is_active);

    // Grid columns boundary clamping [1, 6]
    media.set_grid_columns(0);
    assert_eq!(media.grid_columns, 1);
    media.set_grid_columns(10);
    assert_eq!(media.grid_columns, 6);
    media.set_grid_columns(3);
    assert_eq!(media.grid_columns, 3);

    // Adding media tiles
    media.add_tile(MediaGridTile {
        id: 101,
        media_type: "photo".into(),
        duration_text: None,
        caption: Some("Vacation snap".into()),
        size_text: Some("2.1 MB".into()),
    });
    media.add_tile(MediaGridTile {
        id: 102,
        media_type: "video".into(),
        duration_text: Some("02:15".into()),
        caption: Some("Conference talk".into()),
        size_text: Some("45 MB".into()),
    });
    assert_eq!(media.total_media_count, 2);
    assert_eq!(media.tiles.len(), 2);

    // Preload calculation
    assert!(media.check_preload(1500.0, 500.0, 2300.0)); // 2300 - 2000 = 300 <= 400
    assert!(!media.check_preload(1000.0, 500.0, 2300.0)); // 2300 - 1500 = 800 > 400

    // Pagination
    assert_eq!(media.current_page, 0);
    assert!(media.load_next_page());
    assert_eq!(media.current_page, 1);

    // B. Polls Adapter Transitions & Voting Dynamic Calculations
    let mut polls = InfoProfileTabPollsProfileTabsAdapters::new(2002);
    assert_eq!(polls.polls_count, 0);

    let card = PollCard {
        poll_id: 555,
        question: "Preferred desktop UI framework?".into(),
        total_votes: 0,
        is_closed: false,
        is_anonymous: false,
        options: vec![
            PollOptionItem {
                text: "GPUI".into(),
                vote_count: 0,
                vote_percentage: 0.0,
                is_chosen: false,
            },
            PollOptionItem {
                text: "Qt C++".into(),
                vote_count: 0,
                vote_percentage: 0.0,
                is_chosen: false,
            },
        ],
    };
    polls.add_poll(card);
    assert_eq!(polls.polls_count, 1);

    // Vote for option 0 (GPUI)
    assert!(polls.vote(555, 0));
    assert_eq!(polls.polls[0].total_votes, 1);
    assert_eq!(polls.polls[0].options[0].vote_count, 1);
    assert_eq!(polls.polls[0].options[0].vote_percentage, 100.0);
    assert_eq!(polls.polls[0].options[1].vote_percentage, 0.0);

    // Vote for option 1 (Qt C++)
    assert!(polls.vote(555, 1));
    assert_eq!(polls.polls[0].total_votes, 2);
    assert_eq!(polls.polls[0].options[0].vote_percentage, 50.0);
    assert_eq!(polls.polls[0].options[1].vote_percentage, 50.0);

    // Invalid option vote
    assert!(!polls.vote(555, 99));

    // Close poll: further votes rejected
    assert!(polls.close_poll(555));
    assert!(polls.polls[0].is_closed);
    assert!(!polls.vote(555, 0)); // Rejected on closed poll

    // C. Saved Sublists & Tag Filtering
    let mut saved = InfoProfileTabSavedProfileTabsAdapters::new(3003);
    saved.add_sublist(SavedSublistItem {
        sublist_id: 1,
        title: "Finance & Tax Receipts".into(),
        messages_count: 14,
        pinned: true,
        tag: Some("Finance".into()),
    });
    saved.add_sublist(SavedSublistItem {
        sublist_id: 2,
        title: "Architecture Ideas".into(),
        messages_count: 8,
        pinned: false,
        tag: Some("Work".into()),
    });
    saved.add_sublist(SavedSublistItem {
        sublist_id: 3,
        title: "Personal Notes".into(),
        messages_count: 32,
        pinned: false,
        tag: None,
    });
    saved.add_tag(SavedTagFilter {
        tag: "Finance".into(),
        count: 1,
    });
    assert_eq!(saved.tags.len(), 1);
    assert_eq!(saved.sublists_count, 3);
    assert_eq!(saved.filtered_sublists().len(), 3);

    // Tag filtering
    saved.set_active_tag(Some("Finance".into()));
    assert_eq!(saved.filtered_sublists().len(), 1);
    assert_eq!(saved.filtered_sublists()[0].sublist_id, 1);

    // Combined tag and query filtering
    saved.set_search_query("Receipts");
    assert_eq!(saved.filtered_sublists().len(), 1);
    saved.set_search_query("Unmatched");
    assert_eq!(saved.filtered_sublists().len(), 0);

    // Clear tag: search across all tags
    saved.set_active_tag(None);
    saved.set_search_query("Ideas");
    assert_eq!(saved.filtered_sublists().len(), 1);
    assert_eq!(saved.filtered_sublists()[0].sublist_id, 2);

    // Peer Lists Adapter Transitions
    let mut peer_lists = InfoProfileTabPeerListsProfileTabsAdapters::new(3004, "Similar Channels");
    peer_lists.add_row(PeerRowItem {
        peer_id: 1,
        name: "Rust Developers".into(),
        role: "Channel".into(),
        badge: Some("Verified".into()),
        action_text: "Join".into(),
        is_action_active: true,
    });
    peer_lists.add_row(PeerRowItem {
        peer_id: 2,
        name: "GPUI Explorers".into(),
        role: "Supergroup".into(),
        badge: None,
        action_text: "Join".into(),
        is_action_active: false,
    });
    assert_eq!(peer_lists.items_count, 2);
    assert_eq!(peer_lists.filtered_rows().len(), 2);
    peer_lists.set_search_filter("Rust");
    assert_eq!(peer_lists.filtered_rows().len(), 1);
    assert_eq!(peer_lists.filtered_rows()[0].peer_id, 1);
    assert!(peer_lists.remove_row(1));
    assert_eq!(peer_lists.items_count, 1);

    // D. Stories Adapter Date Grouping & Totals
    let mut stories = InfoProfileTabStoriesProfileTabsAdapters::new(4004, false);
    stories.add_story(
        "Today",
        StoryItem {
            story_id: 101,
            is_viewed: false,
            views_count: 85,
            caption: Some("Morning coffee".into()),
        },
    );
    stories.add_story(
        "Today",
        StoryItem {
            story_id: 102,
            is_viewed: false,
            views_count: 115,
            caption: None,
        },
    );
    stories.add_story(
        "Yesterday",
        StoryItem {
            story_id: 99,
            is_viewed: true,
            views_count: 300,
            caption: Some("Sunset".into()),
        },
    );
    assert_eq!(stories.stories_count, 3);
    assert_eq!(stories.groups.len(), 2);
    assert_eq!(stories.total_views(), 500);

    assert!(stories.mark_story_viewed(101));
    assert!(stories.groups[0].stories[0].is_viewed);
    assert!(!stories.mark_story_viewed(9999)); // Unknown story

    // Archive toggle
    assert!(stories.toggle_archive());
    assert!(stories.is_archive);
    assert!(!stories.toggle_archive());
    assert!(!stories.is_archive);

    // E. Members Adapter Search Threshold & Operations
    let mut members = InfoProfileTabMembersProfileTabsAdapters::new(5005);
    assert!(!members.search_enabled);

    // Add 19 members: search must remain disabled (< 20)
    for i in 1..20 {
        members.add_member(MemberRosterItem {
            user_id: i,
            name: format!("Member {:02}", i),
            is_online: i % 2 == 0,
            status_text: "last seen recently".into(),
            is_admin: false,
            is_owner: false,
            rank_custom: None,
            can_promote: true,
            can_kick: true,
        });
    }
    assert_eq!(members.members_count, 19);
    assert!(!members.search_enabled);

    // Add 20th member: search threshold reached (>= 20)
    members.add_member(MemberRosterItem {
        user_id: 20,
        name: "Member 20".into(),
        is_online: true,
        status_text: "online".into(),
        is_admin: false,
        is_owner: false,
        rank_custom: None,
        can_promote: true,
        can_kick: true,
    });
    assert_eq!(members.members_count, 20);
    assert!(members.search_enabled);

    // Promote member 5 to Admin
    assert!(members.promote_member(5, Some("Community Lead".into())));
    let m5 = members.members.iter().find(|m| m.user_id == 5).unwrap();
    assert!(m5.is_admin);
    assert_eq!(m5.rank_custom.as_deref(), Some("Community Lead"));

    // Kick member 10
    assert!(members.kick_member(10));
    assert_eq!(members.members_count, 19);
    assert!(!members.search_enabled); // Back below threshold
    assert!(!members.kick_member(999)); // Unknown user

    // F. Top Bar Bindings State Transitions
    let mut bindings = InfoProfileTabTopBarBindingsProfileTabs::new("Photos");
    bindings.set_tab_count("Photos", 1);
    bindings.format_subtitle_for_tab("Photos", "photo", "photos");
    assert_eq!(bindings.current_tab_subtitle.as_deref(), Some("1 photo"));

    bindings.set_tab_count("Photos", 42);
    bindings.format_subtitle_for_tab("Photos", "photo", "photos");
    assert_eq!(bindings.current_tab_subtitle.as_deref(), Some("42 photos"));

    // Search toggle lifecycle
    assert!(bindings.toggle_search());
    assert!(bindings.is_search_field_active);
    bindings.set_search_query("Sunset");
    assert_eq!(bindings.search_query, "Sunset");

    assert!(!bindings.toggle_search());
    assert!(!bindings.is_search_field_active);
    assert!(bindings.search_query.is_empty());

    // Selection mode lifecycle
    bindings.set_selection_mode(true, 7);
    assert!(bindings.is_selection_mode);
    assert_eq!(bindings.selected_count, 7);
    bindings.set_selection_mode(false, 0);
    assert!(!bindings.is_selection_mode);
    assert_eq!(bindings.selected_count, 0);
}


// ============================================================================
// 4. Concurrent Multi-Threaded Tab Adapters Stress Harness
// ============================================================================

#[test]
fn test_empirical_concurrent_tab_adapters_stress() {
    let sub_ctrl = Arc::new(RwLock::new(
        InfoProfileTabSubControllerProfileTabsAdapters::new(),
    ));
    let polls = Arc::new(RwLock::new(
        InfoProfileTabPollsProfileTabsAdapters::new(888),
    ));
    let members = Arc::new(RwLock::new(
        InfoProfileTabMembersProfileTabsAdapters::new(999),
    ));

    // Pre-populate poll with 4 options
    {
        let mut p = polls.write().unwrap();
        p.add_poll(PollCard {
            poll_id: 1,
            question: "Concurrent Poll Stress".into(),
            total_votes: 0,
            is_closed: false,
            is_anonymous: true,
            options: vec![
                PollOptionItem { text: "Opt 0".into(), vote_count: 0, vote_percentage: 0.0, is_chosen: false },
                PollOptionItem { text: "Opt 1".into(), vote_count: 0, vote_percentage: 0.0, is_chosen: false },
                PollOptionItem { text: "Opt 2".into(), vote_count: 0, vote_percentage: 0.0, is_chosen: false },
                PollOptionItem { text: "Opt 3".into(), vote_count: 0, vote_percentage: 0.0, is_chosen: false },
            ],
        });
    }

    let mut handles = Vec::new();

    // 8 concurrent threads executing 500 iterations
    for t_idx in 0..8 {
        let sub_ctrl_clone = Arc::clone(&sub_ctrl);
        let polls_clone = Arc::clone(&polls);
        let members_clone = Arc::clone(&members);

        handles.push(thread::spawn(move || {
            let tab_keys = ["media", "members", "files", "stories", "polls"];
            for iter in 0..500 {
                // 1. Sub-controller scroll save and restore
                {
                    let tab = tab_keys[(t_idx + iter) % tab_keys.len()];
                    let scroll = (iter as f32) * 2.5;
                    let mut ctrl = sub_ctrl_clone.write().unwrap();
                    ctrl.save_tab_scroll(tab, scroll);
                    let restored = ctrl.restore_tab_scroll(tab);
                    assert!(restored >= 0.0);
                }

                // 2. Poll voting contention
                {
                    let opt = (t_idx + iter) % 4;
                    let mut p = polls_clone.write().unwrap();
                    p.vote(1, opt);
                }

                // 3. Member roster concurrent additions
                {
                    let uid = ((t_idx * 10_000) + iter) as i64;
                    let mut m = members_clone.write().unwrap();
                    m.add_member(MemberRosterItem {
                        user_id: uid,
                        name: format!("Worker {} Item {}", t_idx, iter),
                        is_online: iter % 2 == 0,
                        status_text: "active".into(),
                        is_admin: false,
                        is_owner: false,
                        rank_custom: None,
                        can_promote: false,
                        can_kick: true,
                    });
                }
            }
        }));
    }

    for handle in handles {
        handle.join().expect("Concurrent stress worker panicked");
    }

    // Verify invariants after concurrent execution
    let final_ctrl = sub_ctrl.read().unwrap();
    assert_eq!(final_ctrl.tab_scroll_cache.len(), 5);

    let final_polls = polls.read().unwrap();
    assert_eq!(final_polls.polls[0].total_votes, 8 * 500);
    let sum_percentages: f32 = final_polls.polls[0].options.iter().map(|o| o.vote_percentage).sum();
    assert!((sum_percentages - 100.0).abs() < 0.1);

    let final_members = members.read().unwrap();
    assert_eq!(final_members.members_count, 8 * 500);
    assert!(final_members.search_enabled);
}
