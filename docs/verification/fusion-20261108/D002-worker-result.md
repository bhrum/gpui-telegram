Implemented **FUS-D002** in its exclusive write boundary.

### Changed Paths
- [conversation_stage.rs](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/conversation_stage.rs)
- [fusion_stage_truth_tests.rs](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/tests/fusion_stage_truth_tests.rs)

### Behavior Changes
- [`ConversationStage::new`](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/conversation_stage.rs#L249-L275): Initializes with `active_chat_id = None`, neutral `chat_title = "Select a chat"`, empty `chat_subtitle` and `chat_avatar_initials`, `pinned_message = None`, empty `messages` / `chat_histories` / `chat_pinned_messages` / `chat_scroll_positions`, `is_inspector_open = false`, zeroed unread count, cleared pending actions/reply/context menu, and a rebuilt empty [`RowHeightCache`](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/conversation_stage.rs#L98-L103).
- [`ConversationStage::new_demo`](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/conversation_stage.rs#L278-L309): Preserves the legacy 11-card fixture data behind an explicit non-production constructor for legacy fixture tests.
- [`ConversationStage::configure_for_cloud_workspace`](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/conversation_stage.rs#L340-L342) (via [`ConversationStage::reset`](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/conversation_stage.rs#L333-L338) and [`ConversationStage::clear_selection`](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/conversation_stage.rs#L311-L331)): Preserves the `(server_url, username)` signature without fabricating bot/saved/status messages, online subtitles, or connectivity claims; clears all histories, pin caches, saved scroll positions, measured row heights, unread count, pending call/media/bot/reply actions, and context menus, and rebuilds prefix sums.
- [`ConversationStage::set_chat`](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/conversation_stage.rs#L344-L384): Avoids writing phantom history entries when transitioning from `active_chat_id == None`, clears stale `active_context_menu` and `pending_reply` across peer switches, and preserves per-chat history, pin, and scroll restoration.
- [`ConversationStage::render`](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/conversation_stage.rs#L1068-L1340): Renders a neutral `"Select a chat"` empty state and suppresses peer header actions/empty avatar badge when `active_chat_id` is `None`, while preserving virtualized transcript rendering and scrolling when a chat is active.

### Remaining Integration Dependencies (for D004 Integrator)
1. **Shell wiring ([shell.rs](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/shell.rs))**:
   - Clear stage selection (`st.clear_selection()` or `st.reset()`) when an authoritative chat list refresh returns empty or when logging out/switching accounts.
   - Replace `.unwrap_or(1)` fallbacks on `stage.read(cx).active_chat_id` (e.g., lines 1125 and 1461 in [shell.rs](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/src/views/shell.rs#L1125)) so outgoing sends require an authoritative selected chat.
2. **Existing fixture-based tests to migrate to `ConversationStage::new_demo()` or explicit fixtures**:
   - [desktop_views_tests.rs](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/tests/desktop_views_tests.rs): `test_conversation_stage_initialization_and_metadata`, `test_conversation_stage_demo_messages_cover_11_canonical_cards`, `test_canonical_text_message_reactions_toggle`, `test_canonical_voice_message_5bit_waveform_renderer_samples`, `test_canonical_poll_voting_and_percentages`, `test_canonical_invoice_stars_payment_card`, `test_app_shell_views_and_overlay_lifecycle`, `test_interactive_click_state_transitions_and_modal_container`, `test_cloud_workspace_configuration_removes_demo_clutter`, `test_row_height_estimator_covers_all_11_canonical_cards`, `test_row_height_cache_prefix_sums_and_binary_search`, `test_chat_switching_isolation`, `test_chat_switching_preserves_independent_scroll_positions`, `test_context_menu_operations`.
   - [challenger_m1_call_state_boundary_tests.rs](file:///Users/bhrum/teamwork_projects/gpui_telegram/crates/gpui_telegram_desktop/tests/challenger_m1_call_state_boundary_tests.rs): `test_fuzz_every_message_type_bubble_click_never_triggers_call` (line 240).

### Unrun Test Commands (for Serialized Coordinator Execution)
```bash
cargo test -p gpui_telegram_desktop --test fusion_stage_truth_tests
python3 scripts/clean_build_garbage.py
```
