# Subsystem Shard: `calls`

**Total Source Files**: 88  
**Total Source Lines**: 44,391  
**Primary Feature IDs**: TCG-N24, TCG-N25, TCG-N26  
**JSON Shard Path**: `docs/specs/ledger/shards/calls.json`  

## Component Inventory & Parity Mapping

| Source Path | Feature ID | Target Symbol | Target Rust / GPUI Path | Lines | Test ID |
|---|---|---|---|---|---|
| `Telegram/SourceFiles/calls/calls.style` | `TCG-N24` | `CallsStyleSheet` | `crates/gpui_telegram_desktop/src/theme/calls/calls_style.rs` | 1913 | `tests/test_tcg_n24_calls.rs` |
| `Telegram/SourceFiles/calls/calls_box_controller.cpp` | `TCG-N24` | `CallsBoxController` | `crates/gpui_telegram_webrtc/src/calls/calls_box_controller.rs` | 930 | `tests/test_tcg_n24_calls_box_controller.rs` |
| `Telegram/SourceFiles/calls/calls_box_controller.h` | `TCG-N24` | `CallsBoxController` | `crates/gpui_telegram_webrtc/src/calls/calls_box_controller.rs` | 88 | `tests/test_tcg_n24_calls_box_controller.rs` |
| `Telegram/SourceFiles/calls/calls_call.cpp` | `TCG-N24` | `CallsCall` | `crates/gpui_telegram_webrtc/src/calls/calls_call.rs` | 1723 | `tests/test_tcg_n24_calls_call.rs` |
| `Telegram/SourceFiles/calls/calls_call.h` | `TCG-N24` | `CallsCall` | `crates/gpui_telegram_webrtc/src/calls/calls_call.rs` | 407 | `tests/test_tcg_n24_calls_call.rs` |
| `Telegram/SourceFiles/calls/calls_controller.cpp` | `TCG-N24` | `CallsController` | `crates/gpui_telegram_webrtc/src/calls/calls_controller.rs` | 53 | `tests/test_tcg_n24_calls_controller.rs` |
| `Telegram/SourceFiles/calls/calls_controller.h` | `TCG-N24` | `CallsController` | `crates/gpui_telegram_webrtc/src/calls/calls_controller.rs` | 59 | `tests/test_tcg_n24_calls_controller.rs` |
| `Telegram/SourceFiles/calls/calls_controller_tgvoip.h` | `TCG-N24` | `CallsControllerTgvoip` | `crates/gpui_telegram_webrtc/src/calls/calls_controller_tgvoip.rs` | 99 | `tests/test_tcg_n24_calls_controller_tgvoip.rs` |
| `Telegram/SourceFiles/calls/calls_controller_webrtc.cpp` | `TCG-N24` | `CallsControllerWebrtc` | `crates/gpui_telegram_webrtc/src/calls/calls_controller_webrtc.rs` | 175 | `tests/test_tcg_n24_calls_controller_webrtc.rs` |
| `Telegram/SourceFiles/calls/calls_controller_webrtc.h` | `TCG-N24` | `CallsControllerWebrtc` | `crates/gpui_telegram_webrtc/src/calls/calls_controller_webrtc.rs` | 60 | `tests/test_tcg_n24_calls_controller_webrtc.rs` |
| `Telegram/SourceFiles/calls/calls_emoji_fingerprint.cpp` | `TCG-N24` | `CallsEmojiFingerprint` | `crates/gpui_telegram_webrtc/src/calls/calls_emoji_fingerprint.rs` | 838 | `tests/test_tcg_n24_calls_emoji_fingerprint.rs` |
| `Telegram/SourceFiles/calls/calls_emoji_fingerprint.h` | `TCG-N24` | `CallsEmojiFingerprint` | `crates/gpui_telegram_webrtc/src/calls/calls_emoji_fingerprint.rs` | 72 | `tests/test_tcg_n24_calls_emoji_fingerprint.rs` |
| `Telegram/SourceFiles/calls/calls_instance.cpp` | `TCG-N24` | `CallsInstance` | `crates/gpui_telegram_webrtc/src/calls/calls_instance.rs` | 1240 | `tests/test_tcg_n24_calls_instance.rs` |
| `Telegram/SourceFiles/calls/calls_instance.h` | `TCG-N24` | `CallsInstance` | `crates/gpui_telegram_webrtc/src/calls/calls_instance.rs` | 238 | `tests/test_tcg_n24_calls_instance.rs` |
| `Telegram/SourceFiles/calls/calls_panel.cpp` | `TCG-N24` | `CallsPanel` | `crates/gpui_telegram_webrtc/src/calls/calls_panel.rs` | 1949 | `tests/test_tcg_n24_calls_panel.rs` |
| `Telegram/SourceFiles/calls/calls_panel.h` | `TCG-N24` | `CallsPanel` | `crates/gpui_telegram_webrtc/src/calls/calls_panel.rs` | 258 | `tests/test_tcg_n24_calls_panel.rs` |
| `Telegram/SourceFiles/calls/calls_panel_background.cpp` | `TCG-N24` | `CallsPanelBackground` | `crates/gpui_telegram_webrtc/src/calls/calls_panel_background.rs` | 237 | `tests/test_tcg_n24_calls_panel_background.rs` |
| `Telegram/SourceFiles/calls/calls_panel_background.h` | `TCG-N24` | `CallsPanelBackground` | `crates/gpui_telegram_webrtc/src/calls/calls_panel_background.rs` | 77 | `tests/test_tcg_n24_calls_panel_background.rs` |
| `Telegram/SourceFiles/calls/calls_rate_call.cpp` | `TCG-N24` | `CallsRateCall` | `crates/gpui_telegram_webrtc/src/calls/calls_rate_call.rs` | 528 | `tests/test_tcg_n24_calls_rate_call.rs` |
| `Telegram/SourceFiles/calls/calls_rate_call.h` | `TCG-N24` | `CallsRateCall` | `crates/gpui_telegram_webrtc/src/calls/calls_rate_call.rs` | 116 | `tests/test_tcg_n24_calls_rate_call.rs` |
| `Telegram/SourceFiles/calls/calls_signal_bars.cpp` | `TCG-N24` | `CallsSignalBars` | `crates/gpui_telegram_webrtc/src/calls/calls_signal_bars.rs` | 65 | `tests/test_tcg_n24_calls_signal_bars.rs` |
| `Telegram/SourceFiles/calls/calls_signal_bars.h` | `TCG-N24` | `CallsSignalBars` | `crates/gpui_telegram_webrtc/src/calls/calls_signal_bars.rs` | 37 | `tests/test_tcg_n24_calls_signal_bars.rs` |
| `Telegram/SourceFiles/calls/calls_top_bar.cpp` | `TCG-N24` | `CallsTopBar` | `crates/gpui_telegram_webrtc/src/calls/calls_top_bar.rs` | 837 | `tests/test_tcg_n24_calls_top_bar.rs` |
| `Telegram/SourceFiles/calls/calls_top_bar.h` | `TCG-N24` | `CallsTopBar` | `crates/gpui_telegram_webrtc/src/calls/calls_top_bar.rs` | 114 | `tests/test_tcg_n24_calls_top_bar.rs` |
| `Telegram/SourceFiles/calls/calls_userpic.cpp` | `TCG-N24` | `CallsUserpic` | `crates/gpui_telegram_webrtc/src/calls/calls_userpic.rs` | 231 | `tests/test_tcg_n24_calls_userpic.rs` |
| `Telegram/SourceFiles/calls/calls_userpic.h` | `TCG-N24` | `CallsUserpic` | `crates/gpui_telegram_webrtc/src/calls/calls_userpic.rs` | 67 | `tests/test_tcg_n24_calls_userpic.rs` |
| `Telegram/SourceFiles/calls/calls_video_bubble.cpp` | `TCG-N24` | `CallsVideoBubble` | `crates/gpui_telegram_webrtc/src/calls/calls_video_bubble.rs` | 266 | `tests/test_tcg_n24_calls_video_bubble.rs` |
| `Telegram/SourceFiles/calls/calls_video_bubble.h` | `TCG-N24` | `CallsVideoBubble` | `crates/gpui_telegram_webrtc/src/calls/calls_video_bubble.rs` | 71 | `tests/test_tcg_n24_calls_video_bubble.rs` |
| `Telegram/SourceFiles/calls/calls_video_incoming.cpp` | `TCG-N24` | `CallsVideoIncoming` | `crates/gpui_telegram_webrtc/src/calls/calls_video_incoming.rs` | 1209 | `tests/test_tcg_n24_calls_video_incoming.rs` |
| `Telegram/SourceFiles/calls/calls_video_incoming.h` | `TCG-N24` | `CallsVideoIncoming` | `crates/gpui_telegram_webrtc/src/calls/calls_video_incoming.rs` | 46 | `tests/test_tcg_n24_calls_video_incoming.rs` |
| `Telegram/SourceFiles/calls/calls_window.cpp` | `TCG-N24` | `CallsWindow` | `crates/gpui_telegram_webrtc/src/calls/calls_window.rs` | 295 | `tests/test_tcg_n24_calls_window.rs` |
| `Telegram/SourceFiles/calls/calls_window.h` | `TCG-N24` | `CallsWindow` | `crates/gpui_telegram_webrtc/src/calls/calls_window.rs` | 117 | `tests/test_tcg_n24_calls_window.rs` |
| `Telegram/SourceFiles/calls/group/calls_choose_join_as.cpp` | `TCG-N25` | `CallsChooseJoinAsGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_choose_join_as.rs` | 516 | `tests/test_tcg_n25_calls_choose_join_as.rs` |
| `Telegram/SourceFiles/calls/group/calls_choose_join_as.h` | `TCG-N25` | `CallsChooseJoinAsGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_choose_join_as.rs` | 65 | `tests/test_tcg_n25_calls_choose_join_as.rs` |
| `Telegram/SourceFiles/calls/group/calls_cover_item.cpp` | `TCG-N25` | `CallsCoverItemGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_cover_item.rs` | 107 | `tests/test_tcg_n25_calls_cover_item.rs` |
| `Telegram/SourceFiles/calls/group/calls_cover_item.h` | `TCG-N25` | `CallsCoverItemGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_cover_item.rs` | 69 | `tests/test_tcg_n25_calls_cover_item.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_call.cpp` | `TCG-N25` | `CallsGroupCallGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_call.rs` | 4378 | `tests/test_tcg_n25_calls_group_call.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_call.h` | `TCG-N25` | `CallsGroupCallGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_call.rs` | 816 | `tests/test_tcg_n25_calls_group_call.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_common.cpp` | `TCG-N25` | `CallsGroupCommonGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_common.rs` | 544 | `tests/test_tcg_n25_calls_group_common.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_common.h` | `TCG-N25` | `CallsGroupCommonGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_common.rs` | 215 | `tests/test_tcg_n25_calls_group_common.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_invite_controller.cpp` | `TCG-N25` | `CallsGroupInviteControllerGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_invite_controller.rs` | 1215 | `tests/test_tcg_n25_calls_group_invite_controller.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_invite_controller.h` | `TCG-N25` | `CallsGroupInviteControllerGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_invite_controller.rs` | 105 | `tests/test_tcg_n25_calls_group_invite_controller.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_members.cpp` | `TCG-N25` | `CallsGroupMembersGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_members.rs` | 2236 | `tests/test_tcg_n25_calls_group_members.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_members.h` | `TCG-N25` | `CallsGroupMembersGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_members.rs` | 132 | `tests/test_tcg_n25_calls_group_members.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_members_row.cpp` | `TCG-N25` | `CallsGroupMembersRowGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_members_row.rs` | 822 | `tests/test_tcg_n25_calls_group_members_row.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_members_row.h` | `TCG-N25` | `CallsGroupMembersRowGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_members_row.rs` | 230 | `tests/test_tcg_n25_calls_group_members_row.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_menu.cpp` | `TCG-N25` | `CallsGroupMenuGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_menu.rs` | 623 | `tests/test_tcg_n25_calls_group_menu.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_menu.h` | `TCG-N25` | `CallsGroupMenuGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_menu.rs` | 61 | `tests/test_tcg_n25_calls_group_menu.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_message_encryption.cpp` | `TCG-N25` | `CallsGroupMessageEncryptionGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_message_encryption.rs` | 367 | `tests/test_tcg_n25_calls_group_message_encryption.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_message_encryption.h` | `TCG-N25` | `CallsGroupMessageEncryptionGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_message_encryption.rs` | 21 | `tests/test_tcg_n25_calls_group_message_encryption.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_message_field.cpp` | `TCG-N25` | `CallsGroupMessageFieldGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_message_field.rs` | 689 | `tests/test_tcg_n25_calls_group_message_field.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_message_field.h` | `TCG-N25` | `CallsGroupMessageFieldGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_message_field.rs` | 89 | `tests/test_tcg_n25_calls_group_message_field.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_messages.cpp` | `TCG-N25` | `CallsGroupMessagesGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_messages.rs` | 731 | `tests/test_tcg_n25_calls_group_messages.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_messages.h` | `TCG-N25` | `CallsGroupMessagesGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_messages.rs` | 199 | `tests/test_tcg_n25_calls_group_messages.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_messages_ui.cpp` | `TCG-N25` | `CallsGroupMessagesUiGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_messages_ui.rs` | 1943 | `tests/test_tcg_n25_calls_group_messages_ui.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_messages_ui.h` | `TCG-N25` | `CallsGroupMessagesUiGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_messages_ui.rs` | 204 | `tests/test_tcg_n25_calls_group_messages_ui.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_panel.cpp` | `TCG-N25` | `CallsGroupPanelGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_panel.rs` | 3135 | `tests/test_tcg_n25_calls_group_panel.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_panel.h` | `TCG-N25` | `CallsGroupPanelGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_panel.rs` | 293 | `tests/test_tcg_n25_calls_group_panel.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_rtmp.cpp` | `TCG-N25` | `CallsGroupRtmpGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_rtmp.rs` | 359 | `tests/test_tcg_n25_calls_group_rtmp.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_rtmp.h` | `TCG-N25` | `CallsGroupRtmpGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_rtmp.rs` | 75 | `tests/test_tcg_n25_calls_group_rtmp.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_settings.cpp` | `TCG-N25` | `CallsGroupSettingsGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_settings.rs` | 1014 | `tests/test_tcg_n25_calls_group_settings.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_settings.h` | `TCG-N25` | `CallsGroupSettingsGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_settings.rs` | 49 | `tests/test_tcg_n25_calls_group_settings.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_stars_box.cpp` | `TCG-N25` | `CallsGroupStarsBoxGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_stars_box.rs` | 147 | `tests/test_tcg_n25_calls_group_stars_box.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_stars_box.h` | `TCG-N25` | `CallsGroupStarsBoxGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_stars_box.rs` | 54 | `tests/test_tcg_n25_calls_group_stars_box.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_toasts.cpp` | `TCG-N25` | `CallsGroupToastsGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_toasts.rs` | 183 | `tests/test_tcg_n25_calls_group_toasts.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_toasts.h` | `TCG-N25` | `CallsGroupToastsGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_toasts.rs` | 38 | `tests/test_tcg_n25_calls_group_toasts.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_viewport.cpp` | `TCG-N25` | `CallsGroupViewportGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_viewport.rs` | 1146 | `tests/test_tcg_n25_calls_group_viewport.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_viewport.h` | `TCG-N25` | `CallsGroupViewportGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_viewport.rs` | 253 | `tests/test_tcg_n25_calls_group_viewport.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_viewport_opengl.cpp` | `TCG-N25` | `CallsGroupViewportOpenglGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_viewport_opengl.rs` | 1482 | `tests/test_tcg_n25_calls_group_viewport_opengl.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_viewport_opengl.h` | `TCG-N25` | `CallsGroupViewportOpenglGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_viewport_opengl.rs` | 165 | `tests/test_tcg_n25_calls_group_viewport_opengl.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_viewport_raster.cpp` | `TCG-N25` | `CallsGroupViewportRasterGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_viewport_raster.rs` | 347 | `tests/test_tcg_n25_calls_group_viewport_raster.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_viewport_raster.h` | `TCG-N25` | `CallsGroupViewportRasterGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_viewport_raster.rs` | 67 | `tests/test_tcg_n25_calls_group_viewport_raster.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_viewport_rhi.cpp` | `TCG-N25` | `CallsGroupViewportRhiGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_viewport_rhi.rs` | 2035 | `tests/test_tcg_n25_calls_group_viewport_rhi.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_viewport_rhi.h` | `TCG-N25` | `CallsGroupViewportRhiGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_viewport_rhi.rs` | 257 | `tests/test_tcg_n25_calls_group_viewport_rhi.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_viewport_tile.cpp` | `TCG-N25` | `CallsGroupViewportTileGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_viewport_tile.rs` | 278 | `tests/test_tcg_n25_calls_group_viewport_tile.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_viewport_tile.h` | `TCG-N25` | `CallsGroupViewportTileGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_viewport_tile.rs` | 142 | `tests/test_tcg_n25_calls_group_viewport_tile.rs` |
| `Telegram/SourceFiles/calls/group/calls_volume_item.cpp` | `TCG-N25` | `CallsVolumeItemGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_volume_item.rs` | 305 | `tests/test_tcg_n25_calls_volume_item.rs` |
| `Telegram/SourceFiles/calls/group/calls_volume_item.h` | `TCG-N25` | `CallsVolumeItemGroup` | `crates/gpui_telegram_webrtc/src/calls/group/calls_volume_item.rs` | 93 | `tests/test_tcg_n25_calls_volume_item.rs` |
| `Telegram/SourceFiles/calls/group/ui/calls_group_recording_box.cpp` | `TCG-N25` | `CallsGroupRecordingBoxGroupUi` | `crates/gpui_telegram_webrtc/src/calls/group/ui/calls_group_recording_box.rs` | 370 | `tests/test_tcg_n25_calls_group_recording_box.rs` |
| `Telegram/SourceFiles/calls/group/ui/calls_group_recording_box.h` | `TCG-N25` | `CallsGroupRecordingBoxGroupUi` | `crates/gpui_telegram_webrtc/src/calls/group/ui/calls_group_recording_box.rs` | 42 | `tests/test_tcg_n25_calls_group_recording_box.rs` |
| `Telegram/SourceFiles/calls/group/ui/calls_group_scheduled_labels.cpp` | `TCG-N25` | `CallsGroupScheduledLabelsGroupUi` | `crates/gpui_telegram_webrtc/src/calls/group/ui/calls_group_scheduled_labels.rs` | 139 | `tests/test_tcg_n25_calls_group_scheduled_labels.rs` |
| `Telegram/SourceFiles/calls/group/ui/calls_group_scheduled_labels.h` | `TCG-N25` | `CallsGroupScheduledLabelsGroupUi` | `crates/gpui_telegram_webrtc/src/calls/group/ui/calls_group_scheduled_labels.rs` | 27 | `tests/test_tcg_n25_calls_group_scheduled_labels.rs` |
| `Telegram/SourceFiles/calls/group/ui/calls_group_stars_coloring.cpp` | `TCG-N25` | `CallsGroupStarsColoringGroupUi` | `crates/gpui_telegram_webrtc/src/calls/group/ui/calls_group_stars_coloring.rs` | 119 | `tests/test_tcg_n25_calls_group_stars_coloring.rs` |
| `Telegram/SourceFiles/calls/group/ui/calls_group_stars_coloring.h` | `TCG-N25` | `CallsGroupStarsColoringGroupUi` | `crates/gpui_telegram_webrtc/src/calls/group/ui/calls_group_stars_coloring.rs` | 47 | `tests/test_tcg_n25_calls_group_stars_coloring.rs` |
| `Telegram/SourceFiles/calls/group/ui/desktop_capture_choose_source.cpp` | `TCG-N26` | `DesktopCaptureChooseSourceGroupUi` | `crates/gpui_telegram_webrtc/src/calls/group/ui/desktop_capture_choose_source.rs` | 616 | `tests/test_tcg_n26_desktop_capture_choose_source.rs` |
| `Telegram/SourceFiles/calls/group/ui/desktop_capture_choose_source.h` | `TCG-N26` | `DesktopCaptureChooseSourceGroupUi` | `crates/gpui_telegram_webrtc/src/calls/group/ui/desktop_capture_choose_source.rs` | 34 | `tests/test_tcg_n26_desktop_capture_choose_source.rs` |
| `Telegram/SourceFiles/calls/ui/calls_device_menu.cpp` | `TCG-N24` | `CallsDeviceMenuUi` | `crates/gpui_telegram_webrtc/src/calls/ui/calls_device_menu.rs` | 252 | `tests/test_tcg_n24_calls_device_menu.rs` |
| `Telegram/SourceFiles/calls/ui/calls_device_menu.h` | `TCG-N24` | `CallsDeviceMenuUi` | `crates/gpui_telegram_webrtc/src/calls/ui/calls_device_menu.rs` | 36 | `tests/test_tcg_n24_calls_device_menu.rs` |

## Deep Source Reading Evidence

### `Telegram/SourceFiles/calls/calls.style`
- **Core Methods & Signals**: point, size, margins, FlatLabel, TextStyle, font
- **Style Tokens**: button: IconButton, bg: color, bgSize: pixels, bgPosition: point, angle: double, outerRadius: pixels
- **Target**: `CallsStyleSheet` in `crates/gpui_telegram_desktop/src/theme/calls/calls_style.rs`

### `Telegram/SourceFiles/calls/calls_box_controller.cpp`
- **Classes & Structs**: GroupCallRow, BoxController, Type, CallType, State, Flag
- **Core Methods & Signals**: GroupCallRow::GroupCallRow, GroupCallRow::rightActionPaint, style::rtlrect, GroupCallRow::rightActionAddRipple, RippleAnimation::EllipseMask, GroupCallRow::rightActionStopLastRipple
- **Target**: `CallsBoxController` in `crates/gpui_telegram_webrtc/src/calls/calls_box_controller.rs`

### `Telegram/SourceFiles/calls/calls_box_controller.h`
- **Classes & Structs**: SessionController, ListController, BoxController, GroupCallRow, Row, InsertWay
- **Core Methods & Signals**: ListController, prepare, rowClicked, rowRightActionClicked, BoxController, loadMoreRows
- **Target**: `CallsBoxController` in `crates/gpui_telegram_webrtc/src/calls/calls_box_controller.rs`

### `Telegram/SourceFiles/calls/calls_call.cpp`
- **Classes & Structs**: InstanceImpl, InstanceV2Impl, InstanceV2ReferenceImpl, State, From, To, ProxyData, Video
- **Core Methods & Signals**: openssl::Sha1, views::all, views::transform, Meta::Versions, Call::Call, Core::App
- **Target**: `CallsCall` in `crates/gpui_telegram_webrtc/src/calls/calls_call.rs`

### `Telegram/SourceFiles/calls/calls_call.h`
- **Classes & Structs**: GroupCall, Track, Instance, VideoCaptureInterface, State, VideoState, AudioState, VideoTrack, DeviceResolvedId, StartConferenceInfo, DhConfig, ErrorType
- **Core Methods & Signals**: getDhConfig, callFinished, callFailed, callRedial, callPlaySound, start
- **Target**: `CallsCall` in `crates/gpui_telegram_webrtc/src/calls/calls_call.rs`

### `Telegram/SourceFiles/calls/calls_controller.h`
- **Classes & Structs**: Controller
- **Core Methods & Signals**: setNetworkType, setMuteMicrophone, setAudioOutputGainControlEnabled, setEchoCancellationStrength, setAudioInputDevice, setAudioOutputDevice
- **Target**: `CallsController` in `crates/gpui_telegram_webrtc/src/calls/calls_controller.rs`

### `Telegram/SourceFiles/calls/calls_controller_tgvoip.h`
- **Classes & Structs**: TgVoipController
- **Core Methods & Signals**: TgVoip::makeInstance, TgVoip::getVersion, version, Version, setNetworkType, setMuteMicrophone
- **Target**: `CallsControllerTgvoip` in `crates/gpui_telegram_webrtc/src/calls/calls_controller_tgvoip.rs`

### `Telegram/SourceFiles/calls/calls_controller_webrtc.h`
- **Classes & Structs**: CallContext, WebrtcController
- **Core Methods & Signals**: version, setNetworkType, setMuteMicrophone, setAudioOutputGainControlEnabled, setEchoCancellationStrength, setAudioInputDevice
- **Target**: `CallsControllerWebrtc` in `crates/gpui_telegram_webrtc/src/calls/calls_controller_webrtc.rs`

### `Telegram/SourceFiles/calls/calls_emoji_fingerprint.cpp`
- **Classes & Structs**: EmojiTooltipShower, State
- **Core Methods & Signals**: QString::fromRawData, Emoji::Find, QCursor::pos, tr::lng_call_fingerprint_tooltip, Tooltip::Show, Tooltip::Hide
- **Target**: `CallsEmojiFingerprint` in `crates/gpui_telegram_webrtc/src/calls/calls_emoji_fingerprint.rs`

### `Telegram/SourceFiles/calls/calls_emoji_fingerprint.h`
- **Classes & Structs**: RpWidget, Call, FingerprintBadgeState, Entry, FingerprintBadge, FingerprintBadgeCache, Emoji
- **Core Methods & Signals**: SetupFingerprintBadge, SetupFingerprintBadgeWidget, PaintFingerprintEntry
- **Target**: `CallsEmojiFingerprint` in `crates/gpui_telegram_webrtc/src/calls/calls_emoji_fingerprint.rs`

