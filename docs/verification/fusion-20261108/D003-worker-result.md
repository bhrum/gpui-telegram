Implemented **FUS-D003** within its exclusive write boundaries.

### Changed Paths
- [context_inspector.rs](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/context_inspector.rs)
- [fusion_inspector_truth_tests.rs](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/tests/fusion_inspector_truth_tests.rs)

### Behavior Changes
- [`ContextInspector::new`](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/context_inspector.rs#L34-L74) and [`ContextInspector::default`](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/context_inspector.rs#L249-L253) start with `is_open = false`, `active_tab = InspectorTab::Profile`, empty `profile` facts (`name`, `username`, `phone`, `bio`, `avatar_initials`, `avatar_color = 0`), empty `members` list and search query, empty `shared_media` (`media_count = 0`, `files`, `links`, `active_subtab = SharedMediaSubTab::Media`), empty `settings.e2ee_fingerprint`, reset security flags (`is_secret_chat_active = false`, `is_contact_blocked = false`), and `pending_media_open = None`.
- [`ContextInspector::new_demo`](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/context_inspector.rs#L76-L165) retains the legacy illustrative fixture data for explicit test fixtures only and is never called by production constructors or resets.
- [`ContextInspector::configure_for_cloud_workspace`](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/context_inspector.rs#L203-L205) and [`ContextInspector::clear_peer`](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/context_inspector.rs#L197-L201) close the drawer, reset the active tab to `InspectorTab::Profile`, clear all prior profile/members/shared-media/security/settings facts and `pending_media_open`, and do not fabricate a bot peer or copy `server_url` into `profile.phone`.
- [`ContextInspector::set_peer`](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/context_inspector.rs#L207-L226) populates only the supplied display facts (`name`, `bio` from `subtitle`, `avatar_initials`, `avatar_color`), never synthesizes a `@username` from `name`, clears prior peer `phone`, `username`, `members`, `shared_media`, security/settings facts, and `pending_media_open` when the observable peer identity `(name, initials, color)` changes (or when `name` is empty), and preserves same-peer facts when no identity change is observable.

### Remaining Integration Dependencies (for FUS-D004 Integrator)
1. **Shell empty-selection synchronization** ([shell.rs](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/shell.rs)): When an authoritative chat refresh yields an empty list or clears selection, call `insp.clear_peer()` (or `insp.configure_for_cloud_workspace(&srv_url)`) so the inspector is cleared and closed alongside the sidebar and stage.
2. **Existing fixture-based tests in [desktop_views_tests.rs](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/tests/desktop_views_tests.rs) assuming legacy demo/synthesized defaults**:
   - `test_context_inspector_drawer_320px_and_tabs` ([desktop_views_tests.rs](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/tests/desktop_views_tests.rs#L338-L374)): assumes `ContextInspector::new()` starts open with `"Alice Smith"`, 3 members, 2 files/links, and non-empty E2EE fingerprint (migrate to `ContextInspector::new_demo()` or explicit fixture setup).
   - `test_all_capability_views_instantiate` ([desktop_views_tests.rs](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/tests/desktop_views_tests.rs#L474-L475)): asserts `inspector.is_open` on `ContextInspector::new()`.
   - `test_interactive_click_state_transitions_and_modal_container` ([desktop_views_tests.rs](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/tests/desktop_views_tests.rs#L507-L512)): asserts `inspector.profile.username == "@rust_core_team"` after `inspector.set_peer("Rust Core Team", "12 members", "RC", 0xE67E22)`.
   - `test_cloud_workspace_configuration_removes_demo_clutter` ([desktop_views_tests.rs](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/tests/desktop_views_tests.rs#L577-L583)): asserts `inspector.profile.name == "Autonomous Agent Bot"` after `configure_for_cloud_workspace`.
   - `test_context_inspector_1to1_geometry_and_header_height` ([desktop_views_tests.rs](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/tests/desktop_views_tests.rs#L655-L659)): asserts `inspector.is_open` and `inspector.profile.avatar_initials == "AS"` on `ContextInspector::new()`.

### Unrun Test Commands (for Serialized Coordinator Verification)
```bash
cargo test -p gpui_telegram_desktop --test fusion_inspector_truth_tests
python3 scripts/clean_build_garbage.py
```
