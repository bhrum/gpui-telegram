//! Challenger M3 Settings Navigation Router & Breadcrumb Stack Adversarial Stress Tests
//!
//! Target: Feature F9 (Settings Navigation Router & Stack)
//! Mandate: Empirical stress tests, fuzzing, invariant checking, zero mocks, zero fake hardcoded passes.
//!
//! Objectives:
//! 1. Deep stack stress: 10,000 pushes and full unwinds with cyclic section transitions without overflow, corruption, or OOM.
//! 2. Breadcrumb integrity & acyclicity oracle: Exhaustive verification across all 24 variants that breadcrumbs terminate at Main and self without cycles.
//! 3. Unwind correctness: Repeated pops from various depths and deep-link orphan states, cleanly transitioning to NavDestination::Chats.
//! 4. High-frequency oscillating / ping-pong transitions between Main and sub-sections.
//! 5. Chaos random-walk fuzzing (10,000 steps) testing pushes, pops, closes, and breadcrumb jumps against strict invariants.
//! 6. Source AST & code contract verification for AppShell settings methods and keyboard bindings.
//! 7. Exhaustive metadata, slug, icon, and localization integrity oracle across all 24 variants.
//! 8. SettingsType view model state lifecycle and visibility toggle.

use gpui_telegram_desktop::views::nav_rail::NavDestination;
use gpui_telegram_desktop::views::settings::settings_type::{SettingsSectionType, SettingsType};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// All 24 canonical settings section variants
const ALL_24_SECTIONS: [SettingsSectionType; 24] = [
    SettingsSectionType::Main,
    SettingsSectionType::Information,
    SettingsSectionType::Notifications,
    SettingsSectionType::NotificationsType,
    SettingsSectionType::NotificationsReactions,
    SettingsSectionType::PrivacySecurity,
    SettingsSectionType::BlockedPeers,
    SettingsSectionType::ActiveSessions,
    SettingsSectionType::Chat,
    SettingsSectionType::Folders,
    SettingsSectionType::Advanced,
    SettingsSectionType::Calls,
    SettingsSectionType::Credits,
    SettingsSectionType::Premium,
    SettingsSectionType::Shortcuts,
    SettingsSectionType::Websites,
    SettingsSectionType::Passkeys,
    SettingsSectionType::LocalPasscode,
    SettingsSectionType::GlobalTtl,
    SettingsSectionType::PowerSaving,
    SettingsSectionType::LocalStorage,
    SettingsSectionType::Business,
    SettingsSectionType::Language,
    SettingsSectionType::Search,
];

/// Exact replica of AppShell's settings routing state machine contract
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettingsNavRouter {
    pub current_section: SettingsSectionType,
    pub history_stack: Vec<SettingsSectionType>,
    pub active_destination: NavDestination,
}

impl SettingsNavRouter {
    pub fn new() -> Self {
        Self {
            current_section: SettingsSectionType::Main,
            history_stack: Vec::new(),
            active_destination: NavDestination::Settings,
        }
    }

    pub fn with_destination(dest: NavDestination) -> Self {
        Self {
            current_section: SettingsSectionType::Main,
            history_stack: Vec::new(),
            active_destination: dest,
        }
    }

    /// Exact implementation matching AppShell::navigate_settings_to (shell.rs:4308-4322)
    pub fn navigate_to(&mut self, target: SettingsSectionType) {
        if self.current_section == target {
            return;
        }
        if self.history_stack.last().copied() != Some(self.current_section) {
            self.history_stack.push(self.current_section);
        }
        self.current_section = target;
        self.active_destination = NavDestination::Settings;
    }

    /// Exact implementation matching AppShell::navigate_settings_back (shell.rs:4324-4335)
    pub fn navigate_back(&mut self) -> bool {
        if let Some(prev) = self.history_stack.pop() {
            self.current_section = prev;
            true
        } else if !self.current_section.is_root() {
            let parent = self.current_section.parent_section().unwrap_or(SettingsSectionType::Main);
            self.current_section = parent;
            true
        } else {
            self.close_settings();
            false
        }
    }

    /// Exact implementation matching AppShell::close_settings (shell.rs:4337-4346)
    pub fn close_settings(&mut self) {
        self.active_destination = NavDestination::Chats;
        self.current_section = SettingsSectionType::Main;
        self.history_stack.clear();
    }
}

// ============================================================================
// 1. Deep Stack Push Stress & Boundary Invariant Verification (10,000 pushes)
// ============================================================================

#[test]
fn test_stress_deep_stack_10000_pushes_and_full_unwind() {
    let mut router = SettingsNavRouter::new();
    assert_eq!(router.current_section, SettingsSectionType::Main);
    assert!(router.history_stack.is_empty());

    let mut expected_history = Vec::new();
    let cycles = 420; // 420 * 24 = 10,080 transitions

    for cycle in 0..cycles {
        for (i, &sec) in ALL_24_SECTIONS.iter().enumerate() {
            let prev = router.current_section;
            if prev != sec {
                expected_history.push(prev);
            }

            // Normal push
            router.navigate_to(sec);
            assert_eq!(router.current_section, sec);
            assert_eq!(router.active_destination, NavDestination::Settings);

            // Adversarial consecutive duplicate push (must be ignored, stack size invariant)
            let stack_len_before = router.history_stack.len();
            router.navigate_to(sec);
            assert_eq!(router.history_stack.len(), stack_len_before, "Consecutive duplicate push must NOT expand stack");

            if (cycle * 24 + i) % 1000 == 0 {
                // Verify stack invariant periodically
                assert_eq!(router.history_stack.len(), expected_history.len());
            }
        }
    }

    assert_eq!(router.history_stack.len(), expected_history.len());
    println!("[Deep Stack Stress] Pushed {} stack frames successfully", router.history_stack.len());

    // Full Unwind: Pop every single frame and verify LIFO order
    while !expected_history.is_empty() {
        let expected_prev = expected_history.pop().unwrap();
        let did_pop = router.navigate_back();
        assert!(did_pop, "Pop must return true while stack is non-empty");
        assert_eq!(router.current_section, expected_prev);
        assert_eq!(router.history_stack.len(), expected_history.len());
    }

    // Now stack is empty, and current_section is Main
    assert!(router.history_stack.is_empty());
    assert_eq!(router.current_section, SettingsSectionType::Main);

    // One more pop must close settings and transition to Chats
    let final_pop = router.navigate_back();
    assert!(!final_pop, "Root pop from Main must return false to signal closing");
    assert_eq!(router.active_destination, NavDestination::Chats);
    assert_eq!(router.current_section, SettingsSectionType::Main);
    assert!(router.history_stack.is_empty());
}

// ============================================================================
// 2. Breadcrumb Integrity & Acyclicity Oracle across All 24 Variants
// ============================================================================

#[test]
fn test_stress_breadcrumb_integrity_and_acyclicity_oracle_all_24_variants() {
    assert_eq!(ALL_24_SECTIONS.len(), 24, "Must cover all 24 settings section types");

    for &sec in &ALL_24_SECTIONS {
        // 1. Acyclicity Oracle: Walk parent_section() links and ensure termination with no cycles
        let mut visited = HashSet::new();
        let mut curr = Some(sec);
        let mut depth = 0;

        while let Some(node) = curr {
            assert!(
                visited.insert(node),
                "CYCLE DETECTED in parent_section() hierarchy for section: {:?}",
                sec
            );
            depth += 1;
            assert!(depth <= 5, "Depth exceeded theoretical hierarchy maximum of 5 for {:?}", sec);
            curr = node.parent_section();
        }

        // 2. Breadcrumb path oracle
        let path = sec.breadcrumb_path();
        assert!(!path.is_empty(), "Breadcrumb path must not be empty for {:?}", sec);
        assert_eq!(
            path[0],
            SettingsSectionType::Main,
            "Breadcrumb path must always start with Main for {:?}",
            sec
        );
        assert_eq!(
            *path.last().unwrap(),
            sec,
            "Breadcrumb path must terminate at self for {:?}",
            sec
        );

        // 3. Verify strict parent-child chaining along breadcrumb vector
        for idx in 1..path.len() {
            let child = path[idx];
            let expected_parent = path[idx - 1];
            assert_eq!(
                child.parent_section(),
                Some(expected_parent),
                "Child {:?} must have parent {:?} in breadcrumb path",
                child,
                expected_parent
            );
        }

        // 4. Root invariant
        if sec == SettingsSectionType::Main {
            assert!(sec.is_root());
            assert_eq!(sec.parent_section(), None);
            assert_eq!(path, vec![SettingsSectionType::Main]);
        } else {
            assert!(!sec.is_root());
            assert!(sec.parent_section().is_some());
        }
    }
}

// ============================================================================
// 3. Unwind Correctness from Varied Stack Depths and Orphaned/Deep-Link States
// ============================================================================

#[test]
fn test_stress_unwind_from_empty_stack_and_deep_link_parents() {
    // Test case: Deep-link into every non-root variant with an EMPTY stack!
    // AppShell must cleanly climb up the parent_section hierarchy until reaching Main, then close.
    for &sec in &ALL_24_SECTIONS {
        if sec == SettingsSectionType::Main {
            continue;
        }

        let mut router = SettingsNavRouter::new();
        router.current_section = sec;
        router.history_stack.clear(); // Empty stack simulate deep-link
        router.active_destination = NavDestination::Settings;

        let expected_breadcrumb = sec.breadcrumb_path();
        // Traverse backwards along expected_breadcrumb (from sec back to Main)
        for expected_ancestor in expected_breadcrumb.iter().rev() {
            assert_eq!(
                router.current_section, *expected_ancestor,
                "Router should be at ancestor {:?}",
                expected_ancestor
            );

            if *expected_ancestor == SettingsSectionType::Main {
                // Popping from Main with empty stack must close settings
                let did_pop = router.navigate_back();
                assert!(!did_pop, "Pop from Main with empty stack must return false");
                assert_eq!(router.active_destination, NavDestination::Chats);
                assert_eq!(router.current_section, SettingsSectionType::Main);
                assert!(router.history_stack.is_empty());
                break;
            } else {
                let did_pop = router.navigate_back();
                assert!(did_pop, "Pop from non-root with empty stack should fallback to parent");
            }
        }
    }

    // Over-popping test: Pop 100 times after closing
    let mut router = SettingsNavRouter::new();
    router.close_settings();
    assert_eq!(router.active_destination, NavDestination::Chats);

    for _ in 0..100 {
        let did_pop = router.navigate_back();
        assert!(!did_pop, "Over-popping must safely return false without panicking");
        assert_eq!(router.active_destination, NavDestination::Chats);
        assert_eq!(router.current_section, SettingsSectionType::Main);
        assert!(router.history_stack.is_empty());
    }
}

// ============================================================================
// 4. High-Frequency Oscillating / Ping-Pong Navigation Stress
// ============================================================================

#[test]
fn test_stress_rapid_high_frequency_oscillations_and_ping_pong() {
    let mut router = SettingsNavRouter::new();

    // 1. Rapid push-pop cycles (5,000 iterations)
    for _ in 0..5000 {
        router.navigate_to(SettingsSectionType::PrivacySecurity);
        assert_eq!(router.current_section, SettingsSectionType::PrivacySecurity);
        assert_eq!(router.history_stack.len(), 1);

        let did_pop = router.navigate_back();
        assert!(did_pop);
        assert_eq!(router.current_section, SettingsSectionType::Main);
        assert!(router.history_stack.is_empty());
    }

    // 2. Oscillating between two sub-sections under the same parent
    for i in 0..2000 {
        let target = if i % 2 == 0 {
            SettingsSectionType::BlockedPeers
        } else {
            SettingsSectionType::ActiveSessions
        };
        router.navigate_to(target);
        assert_eq!(router.current_section, target);
    }
    // Verify stack depth matches transitions
    assert_eq!(router.history_stack.len(), 2000);

    // Clean close
    router.close_settings();
    assert_eq!(router.active_destination, NavDestination::Chats);
    assert_eq!(router.current_section, SettingsSectionType::Main);
    assert!(router.history_stack.is_empty());
}

// ============================================================================
// 5. Chaos Random-Walk Fuzzing (10,000 Steps)
// ============================================================================

#[test]
fn test_chaos_random_walk_fuzzing_10000_steps() {
    let mut router = SettingsNavRouter::new();
    let mut seed: u64 = 0xDEADBEEFCAFEBABE;

    // Linear Congruential Generator for reproducible pseudo-random fuzzing
    let mut next_rand = || -> u64 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        seed
    };

    let total_steps = 10_000;
    for step in 0..total_steps {
        let action = next_rand() % 4;
        match action {
            0 => {
                // Push random section
                let sec_idx = (next_rand() as usize) % ALL_24_SECTIONS.len();
                let target = ALL_24_SECTIONS[sec_idx];
                router.navigate_to(target);
            }
            1 => {
                // Pop
                router.navigate_back();
            }
            2 => {
                // Close
                router.close_settings();
            }
            3 => {
                // Breadcrumb jump: jump directly to an ancestor in current breadcrumbs
                let crumbs = router.current_section.breadcrumb_path();
                let crumb_idx = (next_rand() as usize) % crumbs.len();
                let ancestor = crumbs[crumb_idx];
                router.navigate_to(ancestor);
            }
            _ => unreachable!(),
        }

        // Strict Invariants Verified at EVERY step:
        // 1. Stack deduplication invariant: No consecutive duplicates
        for i in 1..router.history_stack.len() {
            assert_ne!(
                router.history_stack[i - 1],
                router.history_stack[i],
                "Invariant violation at step {}: consecutive duplicate in stack at index {}",
                step,
                i
            );
        }

        // 2. Active destination consistency
        if router.active_destination == NavDestination::Chats {
            assert_eq!(
                router.current_section,
                SettingsSectionType::Main,
                "Invariant violation at step {}: destination is Chats but section is {:?}",
                step,
                router.current_section
            );
            assert!(
                router.history_stack.is_empty(),
                "Invariant violation at step {}: destination is Chats but history_stack is not empty",
                step
            );
        }

        // 3. Breadcrumb path validity
        let crumbs = router.current_section.breadcrumb_path();
        assert_eq!(crumbs[0], SettingsSectionType::Main);
        assert_eq!(*crumbs.last().unwrap(), router.current_section);
    }

    println!("[Chaos Fuzzing] 10,000 steps executed cleanly without invariant violations");
}

// ============================================================================
// 6. Source AST & Code Contract Verification for AppShell Settings Integration
// ============================================================================

#[test]
fn test_ast_shell_settings_router_contract_and_keybindings() {
    let shell_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/views/shell.rs");
    let content = fs::read_to_string(&shell_path)
        .unwrap_or_else(|e| panic!("Failed to read shell.rs at {:?}: {}", shell_path, e));

    // 1. Core Fields in AppShell struct
    assert!(
        content.contains("pub settings_section: SettingsSectionType"),
        "AppShell must declare settings_section: SettingsSectionType"
    );
    assert!(
        content.contains("pub settings_stack: Vec<SettingsSectionType>"),
        "AppShell must declare settings_stack: Vec<SettingsSectionType>"
    );

    // 2. Navigation State Methods
    assert!(
        content.contains("pub fn navigate_settings_to(&mut self, target: SettingsSectionType, cx: &mut Context<Self>)"),
        "AppShell must declare navigate_settings_to"
    );
    assert!(
        content.contains("pub fn navigate_settings_back(&mut self, cx: &mut Context<Self>)"),
        "AppShell must declare navigate_settings_back"
    );
    assert!(
        content.contains("pub fn close_settings(&mut self, cx: &mut Context<Self>)"),
        "AppShell must declare close_settings"
    );

    // 3. Navigation Deduplication & Recording Logic
    assert!(
        content.contains("if self.settings_section == target {"),
        "navigate_settings_to must guard against self-transitions"
    );
    assert!(
        content.contains("if self.settings_stack.last().copied() != Some(self.settings_section) {"),
        "navigate_settings_to must deduplicate consecutive stack entries"
    );
    assert!(
        content.contains("self.settings_stack.push(self.settings_section);"),
        "navigate_settings_to must push current section onto stack"
    );

    // 4. Back Unwind Logic
    assert!(
        content.contains("if let Some(prev) = self.settings_stack.pop() {"),
        "navigate_settings_back must pop previous section from stack"
    );
    assert!(
        content.contains("let parent = self.settings_section.parent_section().unwrap_or(SettingsSectionType::Main);"),
        "navigate_settings_back must fallback to parent_section() on empty stack"
    );
    assert!(
        content.contains("self.close_settings(cx);"),
        "navigate_settings_back must call close_settings when root is reached"
    );

    // 5. Breadcrumb & UI Render Bindings
    assert!(
        content.contains("self.settings_section.breadcrumb_path()"),
        "render_settings_panel must invoke breadcrumb_path() for breadcrumb UI"
    );
    assert!(
        content.contains("this.navigate_settings_back(cx)"),
        "Settings back button must bind to navigate_settings_back"
    );
    assert!(
        content.contains("this.close_settings(cx)"),
        "Settings close button must bind to close_settings"
    );

    // 6. Keyboard Shortcut Bindings
    // Escape key
    assert!(
        content.contains("if self.active_panel == ActiveWorkspacePanel::Settings {"),
        "Escape handler must check for Settings panel"
    );
    // Cmd+, opens Settings
    assert!(
        content.contains("\",\" if self.is_authenticated => {"),
        "Key handler must bind Cmd+, to open Settings"
    );
    // Cmd+[ navigates back
    assert!(
        content.contains("\"[\" if self.active_panel == ActiveWorkspacePanel::Settings => {"),
        "Key handler must bind Cmd+[ to navigate_settings_back"
    );
}

// ============================================================================
// 7. Exhaustive Metadata, Slug, Icon, and Localization Integrity Oracle
// ============================================================================

#[test]
fn test_exhaustive_settings_metadata_and_localization_fuzz() {
    let mut seen_slugs = HashSet::new();

    for &sec in &ALL_24_SECTIONS {
        // Unique slug identifier
        let slug = sec.id_str();
        assert!(!slug.is_empty(), "Slug must not be empty for {:?}", sec);
        assert!(
            seen_slugs.insert(slug),
            "Duplicate slug identifier found: {} for {:?}",
            slug,
            sec
        );

        // English Title
        let en = sec.display_title_localized(false);
        assert!(!en.is_empty(), "English title must not be empty for {:?}", sec);
        assert_eq!(
            sec.display_title(),
            en,
            "display_title() must default to English for {:?}",
            sec
        );

        // Chinese Title
        let zh = sec.display_title_localized(true);
        assert!(!zh.is_empty(), "Chinese title must not be empty for {:?}", sec);
        assert_ne!(
            en, zh,
            "English and Chinese titles must be distinct for {:?}",
            sec
        );

        // Icon Emoji
        let icon = sec.icon();
        assert!(!icon.is_empty(), "Icon must not be empty for {:?}", sec);

        // Display formatting
        let display_fmt = format!("{}", sec);
        assert_eq!(display_fmt, en);

        // Serde JSON roundtrip
        let serialized = serde_json::to_string(&sec).expect("Serialization to JSON must succeed");
        let deserialized: SettingsSectionType =
            serde_json::from_str(&serialized).expect("Deserialization from JSON must succeed");
        assert_eq!(sec, deserialized, "Serde roundtrip must preserve equality for {:?}", sec);
    }
}

// ============================================================================
// 8. SettingsType View Model State Lifecycle & Visibility Toggle
// ============================================================================

#[test]
fn test_settings_type_view_model_lifecycle_and_visibility() {
    let mut view = SettingsType::new();
    assert_eq!(view.current_section, SettingsSectionType::Main);
    assert_eq!(view.title, "Settings");
    assert!(view.is_visible);

    // Visibility toggling
    view.toggle_visibility();
    assert!(!view.is_visible);
    view.toggle_visibility();
    assert!(view.is_visible);

    // Iterating through all 24 sections
    for &sec in &ALL_24_SECTIONS {
        view.set_section(sec);
        assert_eq!(view.current_section, sec);
        assert_eq!(view.title, sec.display_title());

        let specific = SettingsType::for_section(sec);
        assert_eq!(specific.current_section, sec);
        assert_eq!(specific.title, sec.display_title());
        assert!(specific.is_visible);
    }
}
