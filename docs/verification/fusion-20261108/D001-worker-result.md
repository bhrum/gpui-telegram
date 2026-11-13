### Changed Paths

- [chat_sidebar.rs](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/chat_sidebar.rs)
- [fusion_sidebar_truth_tests.rs](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/tests/fusion_sidebar_truth_tests.rs)

### Behavior Implemented (`FUS-D001`)

1. **Truthful Empty Initialization ([`ChatSidebar::new`](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/chat_sidebar.rs#L103-L133) & [`ChatSidebar::default`](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/chat_sidebar.rs#L426-L430))**:
   - Initializes with empty `dialogs`, `active_chat_id: None`, empty `stories`, empty `top_peers`, hidden `suggestions` with an empty `suggestion_manager` (no fabricated `PremiumGrace` notice), `inner_widget.total_rows = 0`, and `inner_widget.selected_index = None`.
   - Seeds only the structural `("All", 0)` folder tab (`active_folder = "All"`); does not seed `Personal`, `Work`, `Bots`, or `Unread` folder tabs or counts.
2. **Cloud Workspace Reset ([`ChatSidebar::configure_for_cloud_workspace`](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/chat_sidebar.rs#L336-L351))**:
   - Preserves the public signature `pub fn configure_for_cloud_workspace(&mut self, _server_url: &str)`.
   - Removes all fabricated cloud peers (`Autonomous Agent Bot` ID 4, `Saved Messages` ID 3, `Cloud Server Status` ID 5), fake online/connectivity strings, and forced `Some(4)` selection.
   - Clears all previous account state (`dialogs`, `active_chat_id`, `folders` reset to `[("All", 0)]`, `active_folder` reset to `"All"`, `search_query`, `search_focused`, `is_resizing`, `story_clicked_peer`, `widget`, `inner_widget`, `stories`, `top_peers`, `suggestion_manager`, and `suggestions`).
3. **Authoritative Selection Guard ([`ChatSidebar::select_dialog`](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/chat_sidebar.rs#L363-L369) & [`ChatSidebar::clear_selection`](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/chat_sidebar.rs#L371-L374))**:
   - `select_dialog` only selects a dialog ID when it exists in `self.dialogs`, synchronizing `inner_widget.selected_index` and marking the dialog as read.
   - `clear_selection` resets `active_chat_id` and `inner_widget.selected_index` to `None`.
4. **Demo Fixture Isolation ([`ChatSidebar::new_demo`](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/chat_sidebar.rs#L136-L326))**:
   - Retains the legacy illustrative dataset behind `ChatSidebar::new_demo()` (never called by production) for explicit fixture migration.

### Remaining Integration Dependencies (for `FUS-D004`)

- **Shell Chat Refresh & Selection Wiring ([shell.rs](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/shell.rs))**:
  - `AppShell::refresh_chats` ([shell.rs](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/shell.rs#L1680-L1735)) currently ignores empty `Ok(chats)` responses and merges into `sb.dialogs`; `D004` must replace the server-managed list authoritatively, clear selection/stage/inspector on empty list, and fence async responses to the active auth session.
  - `AppShell` fallback `.unwrap_or(1)` chat ID routing ([shell.rs](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/shell.rs#L1125) and [shell.rs](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/shell.rs#L1461)) and `NavDestination::SavedMessages => sb.select_dialog(3)` ([shell.rs](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/shell.rs#L296)) depend on `D004` integration.
- **Existing Fixture-Based Tests to Migrate in `D004`**:
  - [`test_chat_sidebar_wiring`](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/tests/dialogs_widget_subsystem_tests.rs#L323-L349) in [dialogs_widget_subsystem_tests.rs](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/tests/dialogs_widget_subsystem_tests.rs#L323-L349): expects `ChatSidebar::new()` to contain the 5 demo dialogs, 5 folders, 3 stories, and `PremiumGrace` suggestion (can switch to `ChatSidebar::new_demo()` or explicit fixture setup).
  - [`test_cloud_workspace_configuration_removes_demo_clutter`](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/tests/desktop_views_tests.rs#L544-L583) in [desktop_views_tests.rs](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/tests/desktop_views_tests.rs#L544-L583): asserts `sidebar.active_chat_id == Some(4)` and `sidebar.dialogs[0].peer_name == "Autonomous Agent Bot"` after `configure_for_cloud_workspace`.

### Unrun Test Commands (for Coordinator Serialized Verification)

```bash
cargo test -p gpui_telegram_desktop --test fusion_sidebar_truth_tests
python3 scripts/clean_build_garbage.py
```
