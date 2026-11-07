# Subsystem Shard: `editor`

**Total Source Files**: 95  
**Total Source Lines**: 24,175  
**Primary Feature IDs**: TCG-N06, TCG-N08, TCG-N09, TCG-N15  
**JSON Shard Path**: `docs/specs/ledger/shards/editor.json`  

## Component Inventory & Parity Mapping

| Source Path | Feature ID | Target Symbol | Target Rust / GPUI Path | Lines | Test ID |
|---|---|---|---|---|---|
| `Telegram/SourceFiles/editor/audio_track_timeline.cpp` | `TCG-N06` | `AudioTrackTimeline` | `crates/gpui_telegram_media/src/editor/audio_track_timeline.rs` | 80 | `tests/test_tcg_n06_audio_track_timeline.rs` |
| `Telegram/SourceFiles/editor/audio_track_timeline.h` | `TCG-N06` | `AudioTrackTimeline` | `crates/gpui_telegram_media/src/editor/audio_track_timeline.rs` | 42 | `tests/test_tcg_n06_audio_track_timeline.rs` |
| `Telegram/SourceFiles/editor/color_picker.cpp` | `TCG-N06` | `ColorPicker` | `crates/gpui_telegram_media/src/editor/color_picker.rs` | 1116 | `tests/test_tcg_n06_color_picker.rs` |
| `Telegram/SourceFiles/editor/color_picker.h` | `TCG-N06` | `ColorPicker` | `crates/gpui_telegram_media/src/editor/color_picker.rs` | 120 | `tests/test_tcg_n06_color_picker.rs` |
| `Telegram/SourceFiles/editor/controllers/controllers.h` | `TCG-N06` | `ControllersControllers` | `crates/gpui_telegram_media/src/editor/controllers/controllers.rs` | 43 | `tests/test_tcg_n06_controllers.rs` |
| `Telegram/SourceFiles/editor/controllers/stickers_panel_controller.cpp` | `TCG-N15` | `StickersPanelControllerControllers` | `crates/gpui_telegram_media/src/editor/controllers/stickers_panel_controller.rs` | 113 | `tests/test_tcg_n15_stickers_panel_controller.rs` |
| `Telegram/SourceFiles/editor/controllers/stickers_panel_controller.h` | `TCG-N15` | `StickersPanelControllerControllers` | `crates/gpui_telegram_media/src/editor/controllers/stickers_panel_controller.rs` | 53 | `tests/test_tcg_n15_stickers_panel_controller.rs` |
| `Telegram/SourceFiles/editor/controllers/undo_controller.cpp` | `TCG-N06` | `UndoControllerControllers` | `crates/gpui_telegram_media/src/editor/controllers/undo_controller.rs` | 39 | `tests/test_tcg_n06_undo_controller.rs` |
| `Telegram/SourceFiles/editor/controllers/undo_controller.h` | `TCG-N06` | `UndoControllerControllers` | `crates/gpui_telegram_media/src/editor/controllers/undo_controller.rs` | 41 | `tests/test_tcg_n06_undo_controller.rs` |
| `Telegram/SourceFiles/editor/editor.style` | `TCG-N06` | `EditorStyleSheet` | `crates/gpui_telegram_desktop/src/theme/editor/editor_style.rs` | 399 | `tests/test_tcg_n06_editor.rs` |
| `Telegram/SourceFiles/editor/editor_audio_disc_button.cpp` | `TCG-N06` | `EditorAudioDiscButton` | `crates/gpui_telegram_media/src/editor/editor_audio_disc_button.rs` | 154 | `tests/test_tcg_n06_editor_audio_disc_button.rs` |
| `Telegram/SourceFiles/editor/editor_audio_disc_button.h` | `TCG-N06` | `EditorAudioDiscButton` | `crates/gpui_telegram_media/src/editor/editor_audio_disc_button.rs` | 57 | `tests/test_tcg_n06_editor_audio_disc_button.rs` |
| `Telegram/SourceFiles/editor/editor_audio_menu.cpp` | `TCG-N06` | `EditorAudioMenu` | `crates/gpui_telegram_media/src/editor/editor_audio_menu.rs` | 228 | `tests/test_tcg_n06_editor_audio_menu.rs` |
| `Telegram/SourceFiles/editor/editor_audio_menu.h` | `TCG-N06` | `EditorAudioMenu` | `crates/gpui_telegram_media/src/editor/editor_audio_menu.rs` | 80 | `tests/test_tcg_n06_editor_audio_menu.rs` |
| `Telegram/SourceFiles/editor/editor_audio_timeline.cpp` | `TCG-N06` | `EditorAudioTimeline` | `crates/gpui_telegram_media/src/editor/editor_audio_timeline.rs` | 298 | `tests/test_tcg_n06_editor_audio_timeline.rs` |
| `Telegram/SourceFiles/editor/editor_audio_timeline.h` | `TCG-N06` | `EditorAudioTimeline` | `crates/gpui_telegram_media/src/editor/editor_audio_timeline.rs` | 69 | `tests/test_tcg_n06_editor_audio_timeline.rs` |
| `Telegram/SourceFiles/editor/editor_crop.cpp` | `TCG-N06` | `EditorCrop` | `crates/gpui_telegram_media/src/editor/editor_crop.rs` | 663 | `tests/test_tcg_n06_editor_crop.rs` |
| `Telegram/SourceFiles/editor/editor_crop.h` | `TCG-N06` | `EditorCrop` | `crates/gpui_telegram_media/src/editor/editor_crop.rs` | 125 | `tests/test_tcg_n06_editor_crop.rs` |
| `Telegram/SourceFiles/editor/editor_keys_legend.cpp` | `TCG-N06` | `EditorKeysLegend` | `crates/gpui_telegram_media/src/editor/editor_keys_legend.rs` | 301 | `tests/test_tcg_n06_editor_keys_legend.rs` |
| `Telegram/SourceFiles/editor/editor_keys_legend.h` | `TCG-N06` | `EditorKeysLegend` | `crates/gpui_telegram_media/src/editor/editor_keys_legend.rs` | 49 | `tests/test_tcg_n06_editor_keys_legend.rs` |
| `Telegram/SourceFiles/editor/editor_layer_widget.cpp` | `TCG-N06` | `EditorLayerWidget` | `crates/gpui_telegram_media/src/editor/editor_layer_widget.rs` | 92 | `tests/test_tcg_n06_editor_layer_widget.rs` |
| `Telegram/SourceFiles/editor/editor_layer_widget.h` | `TCG-N06` | `EditorLayerWidget` | `crates/gpui_telegram_media/src/editor/editor_layer_widget.rs` | 50 | `tests/test_tcg_n06_editor_layer_widget.rs` |
| `Telegram/SourceFiles/editor/editor_link_box.cpp` | `TCG-N06` | `EditorLinkBox` | `crates/gpui_telegram_media/src/editor/editor_link_box.rs` | 1008 | `tests/test_tcg_n06_editor_link_box.rs` |
| `Telegram/SourceFiles/editor/editor_link_box.h` | `TCG-N06` | `EditorLinkBox` | `crates/gpui_telegram_media/src/editor/editor_link_box.rs` | 37 | `tests/test_tcg_n06_editor_link_box.rs` |
| `Telegram/SourceFiles/editor/editor_link_pill.cpp` | `TCG-N06` | `EditorLinkPill` | `crates/gpui_telegram_media/src/editor/editor_link_pill.rs` | 135 | `tests/test_tcg_n06_editor_link_pill.rs` |
| `Telegram/SourceFiles/editor/editor_link_pill.h` | `TCG-N06` | `EditorLinkPill` | `crates/gpui_telegram_media/src/editor/editor_link_pill.rs` | 58 | `tests/test_tcg_n06_editor_link_pill.rs` |
| `Telegram/SourceFiles/editor/editor_message_render.cpp` | `TCG-N06` | `EditorMessageRender` | `crates/gpui_telegram_media/src/editor/editor_message_render.rs` | 397 | `tests/test_tcg_n06_editor_message_render.rs` |
| `Telegram/SourceFiles/editor/editor_message_render.h` | `TCG-N06` | `EditorMessageRender` | `crates/gpui_telegram_media/src/editor/editor_message_render.rs` | 70 | `tests/test_tcg_n06_editor_message_render.rs` |
| `Telegram/SourceFiles/editor/editor_message_source.cpp` | `TCG-N06` | `EditorMessageSource` | `crates/gpui_telegram_media/src/editor/editor_message_source.rs` | 172 | `tests/test_tcg_n06_editor_message_source.rs` |
| `Telegram/SourceFiles/editor/editor_message_source.h` | `TCG-N06` | `EditorMessageSource` | `crates/gpui_telegram_media/src/editor/editor_message_source.rs` | 58 | `tests/test_tcg_n06_editor_message_source.rs` |
| `Telegram/SourceFiles/editor/editor_message_video.cpp` | `TCG-N06` | `EditorMessageVideo` | `crates/gpui_telegram_media/src/editor/editor_message_video.rs` | 206 | `tests/test_tcg_n06_editor_message_video.rs` |
| `Telegram/SourceFiles/editor/editor_message_video.h` | `TCG-N06` | `EditorMessageVideo` | `crates/gpui_telegram_media/src/editor/editor_message_video.rs` | 57 | `tests/test_tcg_n06_editor_message_video.rs` |
| `Telegram/SourceFiles/editor/editor_paint.cpp` | `TCG-N06` | `EditorPaint` | `crates/gpui_telegram_media/src/editor/editor_paint.rs` | 1196 | `tests/test_tcg_n06_editor_paint.rs` |
| `Telegram/SourceFiles/editor/editor_paint.h` | `TCG-N06` | `EditorPaint` | `crates/gpui_telegram_media/src/editor/editor_paint.rs` | 202 | `tests/test_tcg_n06_editor_paint.rs` |
| `Telegram/SourceFiles/editor/editor_trim_timeline.cpp` | `TCG-N06` | `EditorTrimTimeline` | `crates/gpui_telegram_media/src/editor/editor_trim_timeline.rs` | 1207 | `tests/test_tcg_n06_editor_trim_timeline.rs` |
| `Telegram/SourceFiles/editor/editor_trim_timeline.h` | `TCG-N06` | `EditorTrimTimeline` | `crates/gpui_telegram_media/src/editor/editor_trim_timeline.rs` | 207 | `tests/test_tcg_n06_editor_trim_timeline.rs` |
| `Telegram/SourceFiles/editor/photo_editor.cpp` | `TCG-N06` | `PhotoEditor` | `crates/gpui_telegram_media/src/editor/photo_editor.rs` | 712 | `tests/test_tcg_n06_photo_editor.rs` |
| `Telegram/SourceFiles/editor/photo_editor.h` | `TCG-N06` | `PhotoEditor` | `crates/gpui_telegram_media/src/editor/photo_editor.rs` | 99 | `tests/test_tcg_n06_photo_editor.rs` |
| `Telegram/SourceFiles/editor/photo_editor_common.cpp` | `TCG-N06` | `PhotoEditorCommon` | `crates/gpui_telegram_media/src/editor/photo_editor_common.rs` | 308 | `tests/test_tcg_n06_photo_editor_common.rs` |
| `Telegram/SourceFiles/editor/photo_editor_common.h` | `TCG-N06` | `PhotoEditorCommon` | `crates/gpui_telegram_media/src/editor/photo_editor_common.rs` | 99 | `tests/test_tcg_n06_photo_editor_common.rs` |
| `Telegram/SourceFiles/editor/photo_editor_content.cpp` | `TCG-N06` | `PhotoEditorContent` | `crates/gpui_telegram_media/src/editor/photo_editor_content.rs` | 630 | `tests/test_tcg_n06_photo_editor_content.rs` |
| `Telegram/SourceFiles/editor/photo_editor_content.h` | `TCG-N06` | `PhotoEditorContent` | `crates/gpui_telegram_media/src/editor/photo_editor_content.rs` | 127 | `tests/test_tcg_n06_photo_editor_content.rs` |
| `Telegram/SourceFiles/editor/photo_editor_controls.cpp` | `TCG-N06` | `PhotoEditorControls` | `crates/gpui_telegram_media/src/editor/photo_editor_controls.rs` | 1395 | `tests/test_tcg_n06_photo_editor_controls.rs` |
| `Telegram/SourceFiles/editor/photo_editor_controls.h` | `TCG-N06` | `PhotoEditorControls` | `crates/gpui_telegram_media/src/editor/photo_editor_controls.rs` | 148 | `tests/test_tcg_n06_photo_editor_controls.rs` |
| `Telegram/SourceFiles/editor/photo_editor_inner_common.h` | `TCG-N06` | `PhotoEditorInnerCommon` | `crates/gpui_telegram_media/src/editor/photo_editor_inner_common.rs` | 105 | `tests/test_tcg_n06_photo_editor_inner_common.rs` |
| `Telegram/SourceFiles/editor/photo_editor_layer_widget.cpp` | `TCG-N06` | `PhotoEditorLayerWidget` | `crates/gpui_telegram_media/src/editor/photo_editor_layer_widget.rs` | 181 | `tests/test_tcg_n06_photo_editor_layer_widget.rs` |
| `Telegram/SourceFiles/editor/photo_editor_layer_widget.h` | `TCG-N06` | `PhotoEditorLayerWidget` | `crates/gpui_telegram_media/src/editor/photo_editor_layer_widget.rs` | 56 | `tests/test_tcg_n06_photo_editor_layer_widget.rs` |
| `Telegram/SourceFiles/editor/scene/scene.cpp` | `TCG-N06` | `SceneScene` | `crates/gpui_telegram_media/src/editor/scene/scene.rs` | 1262 | `tests/test_tcg_n06_scene.rs` |
| `Telegram/SourceFiles/editor/scene/scene.h` | `TCG-N06` | `SceneScene` | `crates/gpui_telegram_media/src/editor/scene/scene.rs` | 217 | `tests/test_tcg_n06_scene.rs` |
| `Telegram/SourceFiles/editor/scene/scene_emoji_document.cpp` | `TCG-N15` | `SceneEmojiDocumentScene` | `crates/gpui_telegram_media/src/editor/scene/scene_emoji_document.rs` | 263 | `tests/test_tcg_n15_scene_emoji_document.rs` |
| `Telegram/SourceFiles/editor/scene/scene_emoji_document.h` | `TCG-N15` | `SceneEmojiDocumentScene` | `crates/gpui_telegram_media/src/editor/scene/scene_emoji_document.rs` | 29 | `tests/test_tcg_n15_scene_emoji_document.rs` |
| `Telegram/SourceFiles/editor/scene/scene_item_animated.cpp` | `TCG-N06` | `SceneItemAnimatedScene` | `crates/gpui_telegram_media/src/editor/scene/scene_item_animated.rs` | 108 | `tests/test_tcg_n06_scene_item_animated.rs` |
| `Telegram/SourceFiles/editor/scene/scene_item_animated.h` | `TCG-N06` | `SceneItemAnimatedScene` | `crates/gpui_telegram_media/src/editor/scene/scene_item_animated.rs` | 53 | `tests/test_tcg_n06_scene_item_animated.rs` |
| `Telegram/SourceFiles/editor/scene/scene_item_base.cpp` | `TCG-N06` | `SceneItemBaseScene` | `crates/gpui_telegram_media/src/editor/scene/scene_item_base.rs` | 872 | `tests/test_tcg_n06_scene_item_base.rs` |
| `Telegram/SourceFiles/editor/scene/scene_item_base.h` | `TCG-N06` | `SceneItemBaseScene` | `crates/gpui_telegram_media/src/editor/scene/scene_item_base.rs` | 267 | `tests/test_tcg_n06_scene_item_base.rs` |
| `Telegram/SourceFiles/editor/scene/scene_item_canvas.cpp` | `TCG-N06` | `SceneItemCanvasScene` | `crates/gpui_telegram_media/src/editor/scene/scene_item_canvas.rs` | 518 | `tests/test_tcg_n06_scene_item_canvas.rs` |
| `Telegram/SourceFiles/editor/scene/scene_item_canvas.h` | `TCG-N06` | `SceneItemCanvasScene` | `crates/gpui_telegram_media/src/editor/scene/scene_item_canvas.rs` | 110 | `tests/test_tcg_n06_scene_item_canvas.rs` |
| `Telegram/SourceFiles/editor/scene/scene_item_image.cpp` | `TCG-N06` | `SceneItemImageScene` | `crates/gpui_telegram_media/src/editor/scene/scene_item_image.rs` | 48 | `tests/test_tcg_n06_scene_item_image.rs` |
| `Telegram/SourceFiles/editor/scene/scene_item_image.h` | `TCG-N06` | `SceneItemImageScene` | `crates/gpui_telegram_media/src/editor/scene/scene_item_image.rs` | 32 | `tests/test_tcg_n06_scene_item_image.rs` |
| `Telegram/SourceFiles/editor/scene/scene_item_line.cpp` | `TCG-N06` | `SceneItemLineScene` | `crates/gpui_telegram_media/src/editor/scene/scene_item_line.rs` | 93 | `tests/test_tcg_n06_scene_item_line.rs` |
| `Telegram/SourceFiles/editor/scene/scene_item_line.h` | `TCG-N06` | `SceneItemLineScene` | `crates/gpui_telegram_media/src/editor/scene/scene_item_line.rs` | 47 | `tests/test_tcg_n06_scene_item_line.rs` |
| `Telegram/SourceFiles/editor/scene/scene_item_link.cpp` | `TCG-N06` | `SceneItemLinkScene` | `crates/gpui_telegram_media/src/editor/scene/scene_item_link.rs` | 181 | `tests/test_tcg_n06_scene_item_link.rs` |
| `Telegram/SourceFiles/editor/scene/scene_item_link.h` | `TCG-N06` | `SceneItemLinkScene` | `crates/gpui_telegram_media/src/editor/scene/scene_item_link.rs` | 59 | `tests/test_tcg_n06_scene_item_link.rs` |
| `Telegram/SourceFiles/editor/scene/scene_item_message.cpp` | `TCG-N06` | `SceneItemMessageScene` | `crates/gpui_telegram_media/src/editor/scene/scene_item_message.rs` | 499 | `tests/test_tcg_n06_scene_item_message.rs` |
| `Telegram/SourceFiles/editor/scene/scene_item_message.h` | `TCG-N06` | `SceneItemMessageScene` | `crates/gpui_telegram_media/src/editor/scene/scene_item_message.rs` | 121 | `tests/test_tcg_n06_scene_item_message.rs` |
| `Telegram/SourceFiles/editor/scene/scene_item_shape.cpp` | `TCG-N06` | `SceneItemShapeScene` | `crates/gpui_telegram_media/src/editor/scene/scene_item_shape.rs` | 463 | `tests/test_tcg_n06_scene_item_shape.rs` |
| `Telegram/SourceFiles/editor/scene/scene_item_shape.h` | `TCG-N06` | `SceneItemShapeScene` | `crates/gpui_telegram_media/src/editor/scene/scene_item_shape.rs` | 93 | `tests/test_tcg_n06_scene_item_shape.rs` |
| `Telegram/SourceFiles/editor/scene/scene_item_sticker.cpp` | `TCG-N15` | `SceneItemStickerScene` | `crates/gpui_telegram_media/src/editor/scene/scene_item_sticker.rs` | 236 | `tests/test_tcg_n15_scene_item_sticker.rs` |
| `Telegram/SourceFiles/editor/scene/scene_item_sticker.h` | `TCG-N15` | `SceneItemStickerScene` | `crates/gpui_telegram_media/src/editor/scene/scene_item_sticker.rs` | 73 | `tests/test_tcg_n15_scene_item_sticker.rs` |
| `Telegram/SourceFiles/editor/scene/scene_item_text.cpp` | `TCG-N06` | `SceneItemTextScene` | `crates/gpui_telegram_media/src/editor/scene/scene_item_text.rs` | 1015 | `tests/test_tcg_n06_scene_item_text.rs` |
| `Telegram/SourceFiles/editor/scene/scene_item_text.h` | `TCG-N06` | `SceneItemTextScene` | `crates/gpui_telegram_media/src/editor/scene/scene_item_text.rs` | 136 | `tests/test_tcg_n06_scene_item_text.rs` |
| `Telegram/SourceFiles/editor/scene/scene_item_video.cpp` | `TCG-N06` | `SceneItemVideoScene` | `crates/gpui_telegram_media/src/editor/scene/scene_item_video.rs` | 141 | `tests/test_tcg_n06_scene_item_video.rs` |
| `Telegram/SourceFiles/editor/scene/scene_item_video.h` | `TCG-N06` | `SceneItemVideoScene` | `crates/gpui_telegram_media/src/editor/scene/scene_item_video.rs` | 54 | `tests/test_tcg_n06_scene_item_video.rs` |
| `Telegram/SourceFiles/editor/scene/scene_text_editing.cpp` | `TCG-N06` | `SceneTextEditingScene` | `crates/gpui_telegram_media/src/editor/scene/scene_text_editing.rs` | 708 | `tests/test_tcg_n06_scene_text_editing.rs` |
| `Telegram/SourceFiles/editor/scene/scene_text_editing.h` | `TCG-N06` | `SceneTextEditingScene` | `crates/gpui_telegram_media/src/editor/scene/scene_text_editing.rs` | 90 | `tests/test_tcg_n06_scene_text_editing.rs` |
| `Telegram/SourceFiles/editor/video/video_clip.cpp` | `TCG-N09` | `VideoClipVideo` | `crates/gpui_telegram_media/src/editor/video/video_clip.rs` | 147 | `tests/test_tcg_n09_video_clip.rs` |
| `Telegram/SourceFiles/editor/video/video_clip.h` | `TCG-N09` | `VideoClipVideo` | `crates/gpui_telegram_media/src/editor/video/video_clip.rs` | 71 | `tests/test_tcg_n09_video_clip.rs` |
| `Telegram/SourceFiles/editor/video/video_editor.cpp` | `TCG-N06` | `VideoEditorVideo` | `crates/gpui_telegram_media/src/editor/video/video_editor.rs` | 999 | `tests/test_tcg_n06_video_editor.rs` |
| `Telegram/SourceFiles/editor/video/video_editor.h` | `TCG-N06` | `VideoEditorVideo` | `crates/gpui_telegram_media/src/editor/video/video_editor.rs` | 145 | `tests/test_tcg_n06_video_editor.rs` |
| `Telegram/SourceFiles/editor/video/video_editor_common.cpp` | `TCG-N06` | `VideoEditorCommonVideo` | `crates/gpui_telegram_media/src/editor/video/video_editor_common.rs` | 116 | `tests/test_tcg_n06_video_editor_common.rs` |
| `Telegram/SourceFiles/editor/video/video_editor_common.h` | `TCG-N06` | `VideoEditorCommonVideo` | `crates/gpui_telegram_media/src/editor/video/video_editor_common.rs` | 65 | `tests/test_tcg_n06_video_editor_common.rs` |
| `Telegram/SourceFiles/editor/video/video_editor_layer.cpp` | `TCG-N06` | `VideoEditorLayerVideo` | `crates/gpui_telegram_media/src/editor/video/video_editor_layer.rs` | 318 | `tests/test_tcg_n06_video_editor_layer.rs` |
| `Telegram/SourceFiles/editor/video/video_editor_layer.h` | `TCG-N06` | `VideoEditorLayerVideo` | `crates/gpui_telegram_media/src/editor/video/video_editor_layer.rs` | 68 | `tests/test_tcg_n06_video_editor_layer.rs` |
| `Telegram/SourceFiles/editor/video/video_editor_quality.cpp` | `TCG-N06` | `VideoEditorQualityVideo` | `crates/gpui_telegram_media/src/editor/video/video_editor_quality.rs` | 46 | `tests/test_tcg_n06_video_editor_quality.rs` |
| `Telegram/SourceFiles/editor/video/video_editor_quality.h` | `TCG-N06` | `VideoEditorQualityVideo` | `crates/gpui_telegram_media/src/editor/video/video_editor_quality.rs` | 26 | `tests/test_tcg_n06_video_editor_quality.rs` |
| `Telegram/SourceFiles/editor/video/video_quality_slider.cpp` | `TCG-N06` | `VideoQualitySliderVideo` | `crates/gpui_telegram_media/src/editor/video/video_quality_slider.rs` | 225 | `tests/test_tcg_n06_video_quality_slider.rs` |
| `Telegram/SourceFiles/editor/video/video_quality_slider.h` | `TCG-N06` | `VideoQualitySliderVideo` | `crates/gpui_telegram_media/src/editor/video/video_quality_slider.rs` | 57 | `tests/test_tcg_n06_video_quality_slider.rs` |
| `Telegram/SourceFiles/editor/video/video_segment_player.cpp` | `TCG-N08` | `VideoSegmentPlayerVideo` | `crates/gpui_telegram_media/src/editor/video/video_segment_player.rs` | 259 | `tests/test_tcg_n08_video_segment_player.rs` |
| `Telegram/SourceFiles/editor/video/video_segment_player.h` | `TCG-N08` | `VideoSegmentPlayerVideo` | `crates/gpui_telegram_media/src/editor/video/video_segment_player.rs` | 98 | `tests/test_tcg_n08_video_segment_player.rs` |
| `Telegram/SourceFiles/editor/video/video_timeline.cpp` | `TCG-N06` | `VideoTimelineVideo` | `crates/gpui_telegram_media/src/editor/video/video_timeline.rs` | 287 | `tests/test_tcg_n06_video_timeline.rs` |
| `Telegram/SourceFiles/editor/video/video_timeline.h` | `TCG-N06` | `VideoTimelineVideo` | `crates/gpui_telegram_media/src/editor/video/video_timeline.rs` | 92 | `tests/test_tcg_n06_video_timeline.rs` |
| `Telegram/SourceFiles/editor/video/video_timeline_seeker.cpp` | `TCG-N06` | `VideoTimelineSeekerVideo` | `crates/gpui_telegram_media/src/editor/video/video_timeline_seeker.rs` | 86 | `tests/test_tcg_n06_video_timeline_seeker.rs` |
| `Telegram/SourceFiles/editor/video/video_timeline_seeker.h` | `TCG-N06` | `VideoTimelineSeekerVideo` | `crates/gpui_telegram_media/src/editor/video/video_timeline_seeker.rs` | 39 | `tests/test_tcg_n06_video_timeline_seeker.rs` |
| `Telegram/SourceFiles/editor/video_item_timeline.cpp` | `TCG-N06` | `VideoItemTimeline` | `crates/gpui_telegram_media/src/editor/video_item_timeline.rs` | 111 | `tests/test_tcg_n06_video_item_timeline.rs` |
| `Telegram/SourceFiles/editor/video_item_timeline.h` | `TCG-N06` | `VideoItemTimeline` | `crates/gpui_telegram_media/src/editor/video_item_timeline.rs` | 50 | `tests/test_tcg_n06_video_item_timeline.rs` |

## Deep Source Reading Evidence

### `Telegram/SourceFiles/editor/audio_track_timeline.h`
- **Classes & Structs**: AudioTimeline, AudioTrack, AudioTrackTimeline
- **Core Methods & Signals**: AudioTrackTimeline, setTrack, refreshTrim, commitPendingEdit, refreshVolume, resizeGetHeight
- **Target**: `AudioTrackTimeline` in `crates/gpui_telegram_media/src/editor/audio_track_timeline.rs`

### `Telegram/SourceFiles/editor/color_picker.cpp`
- **Classes & Structs**: PlusCircle, ColorButton, ToolLottieButton, State
- **Core Methods & Signals**: QWidget::rect, rect::center, Lottie::MakeIcon, ColorPicker::ColorPicker, ColorPicker::moveLine, ColorPicker::setCanvasRect
- **Target**: `ColorPicker` in `crates/gpui_telegram_media/src/editor/color_picker.rs`

### `Telegram/SourceFiles/editor/color_picker.h`
- **Classes & Structs**: RpWidget, Show, ColorPicker
- **Core Methods & Signals**: ColorPicker, moveLine, setCanvasRect, setVisible, setColor, setToolSelectionVisible
- **Target**: `ColorPicker` in `crates/gpui_telegram_media/src/editor/color_picker.rs`

### `Telegram/SourceFiles/editor/controllers/controllers.h`
- **Classes & Structs**: Show, Controllers
- **Target**: `ControllersControllers` in `crates/gpui_telegram_media/src/editor/controllers/controllers.rs`

### `Telegram/SourceFiles/editor/controllers/stickers_panel_controller.h`
- **Classes & Structs**: TabbedPanel, Show, RpWidget, StickersPanelController, ShowRequest
- **Core Methods & Signals**: setShowRequestChanges, setMoveRequestChanges, rpl::photoRequests, rpl::audioRequests, rpl::linkRequests, rpl::panelShown
- **Target**: `StickersPanelControllerControllers` in `crates/gpui_telegram_media/src/editor/controllers/stickers_panel_controller.rs`

### `Telegram/SourceFiles/editor/controllers/undo_controller.cpp`
- **Classes & Structs**: EnableRequest
- **Core Methods & Signals**: UndoController::UndoController, UndoController::setCanPerformChanges, UndoController::setPerformRequestChanges, UndoController::canPerformChanges, UndoController::performRequestChanges
- **Target**: `UndoControllerControllers` in `crates/gpui_telegram_media/src/editor/controllers/undo_controller.rs`

### `Telegram/SourceFiles/editor/controllers/undo_controller.h`
- **Classes & Structs**: Undo, UndoController, EnableRequest
- **Core Methods & Signals**: setCanPerformChanges, setPerformRequestChanges, rpl::canPerformChanges, rpl::performRequestChanges
- **Target**: `UndoControllerControllers` in `crates/gpui_telegram_media/src/editor/controllers/undo_controller.rs`

### `Telegram/SourceFiles/editor/editor.style`
- **Core Methods & Signals**: margins, TextStyle, font, FlatLabel, align, IconButton
- **Style Tokens**: photoEditorControlsHeight: 146px, photoEditorControlsBottomSkip: 20px, photoEditorControlsCenterSkip: 6px, photoEditorContentMargins: margins, photoEditorBarAnimationDuration: 100, photoEditorButtonIconFg: mediaviewPipControlsFg
- **Target**: `EditorStyleSheet` in `crates/gpui_telegram_desktop/src/theme/editor/editor_style.rs`

### `Telegram/SourceFiles/editor/editor_audio_disc_button.h`
- **Classes & Structs**: PopupMenu, AudioTrack, AudioDiscButton
- **Core Methods & Signals**: AudioDiscButton, setTrack, setActive, paintEvent, contextMenuEvent, onStateChanged
- **Target**: `EditorAudioDiscButton` in `crates/gpui_telegram_media/src/editor/editor_audio_disc_button.rs`

### `Telegram/SourceFiles/editor/editor_audio_menu.h`
- **Classes & Structs**: CrossLineAnimation, MediaSlider, PopupMenu, ArcsAnimation, AudioTrack, MenuVolumeItem
- **Core Methods & Signals**: MenuVolumeItem, action, isEnabled, contentHeight, initArcsAnimation, applyVolume
- **Target**: `EditorAudioMenu` in `crates/gpui_telegram_media/src/editor/editor_audio_menu.rs`

