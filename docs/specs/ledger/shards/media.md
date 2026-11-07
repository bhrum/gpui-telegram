# Subsystem Shard: `media`

**Total Source Files**: 151  
**Total Source Lines**: 65,265  
**Primary Feature IDs**: TCG-N06, TCG-N07, TCG-N08, TCG-N09  
**JSON Shard Path**: `docs/specs/ledger/shards/media.json`  

## Component Inventory & Parity Mapping

| Source Path | Feature ID | Target Symbol | Target Rust / GPUI Path | Lines | Test ID |
|---|---|---|---|---|---|
| `Telegram/SourceFiles/ffmpeg/ffmpeg_bytes_io_wrap.h` | `TCG-N06` | `FfmpegBytesIoWrap` | `crates/gpui_telegram_media/src/media/ffmpeg_bytes_io_wrap.rs` | 130 | `tests/test_tcg_n06_ffmpeg_bytes_io_wrap.rs` |
| `Telegram/SourceFiles/ffmpeg/ffmpeg_frame_generator.cpp` | `TCG-N06` | `FfmpegFrameGenerator` | `crates/gpui_telegram_media/src/media/ffmpeg_frame_generator.rs` | 428 | `tests/test_tcg_n06_ffmpeg_frame_generator.rs` |
| `Telegram/SourceFiles/ffmpeg/ffmpeg_frame_generator.h` | `TCG-N06` | `FfmpegFrameGenerator` | `crates/gpui_telegram_media/src/media/ffmpeg_frame_generator.rs` | 41 | `tests/test_tcg_n06_ffmpeg_frame_generator.rs` |
| `Telegram/SourceFiles/ffmpeg/ffmpeg_utility.cpp` | `TCG-N06` | `FfmpegUtility` | `crates/gpui_telegram_media/src/media/ffmpeg_utility.rs` | 883 | `tests/test_tcg_n06_ffmpeg_utility.rs` |
| `Telegram/SourceFiles/ffmpeg/ffmpeg_utility.h` | `TCG-N06` | `FfmpegUtility` | `crates/gpui_telegram_media/src/media/ffmpeg_utility.rs` | 261 | `tests/test_tcg_n06_ffmpeg_utility.rs` |
| `Telegram/SourceFiles/media/audio/media_audio.cpp` | `TCG-N06` | `MediaAudioAudio` | `crates/gpui_telegram_media/src/media/audio/media_audio.rs` | 1649 | `tests/test_tcg_n06_media_audio.rs` |
| `Telegram/SourceFiles/media/audio/media_audio.h` | `TCG-N06` | `MediaAudioAudio` | `crates/gpui_telegram_media/src/media/audio/media_audio.rs` | 422 | `tests/test_tcg_n06_media_audio.rs` |
| `Telegram/SourceFiles/media/audio/media_audio_capture.cpp` | `TCG-N06` | `MediaAudioCaptureAudio` | `crates/gpui_telegram_media/src/media/audio/media_audio_capture.rs` | 870 | `tests/test_tcg_n06_media_audio_capture.rs` |
| `Telegram/SourceFiles/media/audio/media_audio_capture.h` | `TCG-N06` | `MediaAudioCaptureAudio` | `crates/gpui_telegram_media/src/media/audio/media_audio_capture.rs` | 84 | `tests/test_tcg_n06_media_audio_capture.rs` |
| `Telegram/SourceFiles/media/audio/media_audio_capture_common.h` | `TCG-N06` | `MediaAudioCaptureCommonAudio` | `crates/gpui_telegram_media/src/media/audio/media_audio_capture_common.rs` | 19 | `tests/test_tcg_n06_media_audio_capture_common.rs` |
| `Telegram/SourceFiles/media/audio/media_audio_edit.cpp` | `TCG-N06` | `MediaAudioEditAudio` | `crates/gpui_telegram_media/src/media/audio/media_audio_edit.rs` | 407 | `tests/test_tcg_n06_media_audio_edit.rs` |
| `Telegram/SourceFiles/media/audio/media_audio_edit.h` | `TCG-N06` | `MediaAudioEditAudio` | `crates/gpui_telegram_media/src/media/audio/media_audio_edit.rs` | 29 | `tests/test_tcg_n06_media_audio_edit.rs` |
| `Telegram/SourceFiles/media/audio/media_audio_ffmpeg_loader.cpp` | `TCG-N06` | `MediaAudioFfmpegLoaderAudio` | `crates/gpui_telegram_media/src/media/audio/media_audio_ffmpeg_loader.rs` | 777 | `tests/test_tcg_n06_media_audio_ffmpeg_loader.rs` |
| `Telegram/SourceFiles/media/audio/media_audio_ffmpeg_loader.h` | `TCG-N06` | `MediaAudioFfmpegLoaderAudio` | `crates/gpui_telegram_media/src/media/audio/media_audio_ffmpeg_loader.rs` | 197 | `tests/test_tcg_n06_media_audio_ffmpeg_loader.rs` |
| `Telegram/SourceFiles/media/audio/media_audio_loader.cpp` | `TCG-N06` | `MediaAudioLoaderAudio` | `crates/gpui_telegram_media/src/media/audio/media_audio_loader.rs` | 92 | `tests/test_tcg_n06_media_audio_loader.rs` |
| `Telegram/SourceFiles/media/audio/media_audio_loader.h` | `TCG-N06` | `MediaAudioLoaderAudio` | `crates/gpui_telegram_media/src/media/audio/media_audio_loader.rs` | 86 | `tests/test_tcg_n06_media_audio_loader.rs` |
| `Telegram/SourceFiles/media/audio/media_audio_loaders.cpp` | `TCG-N06` | `MediaAudioLoadersAudio` | `crates/gpui_telegram_media/src/media/audio/media_audio_loaders.rs` | 607 | `tests/test_tcg_n06_media_audio_loaders.rs` |
| `Telegram/SourceFiles/media/audio/media_audio_loaders.h` | `TCG-N06` | `MediaAudioLoadersAudio` | `crates/gpui_telegram_media/src/media/audio/media_audio_loaders.rs` | 82 | `tests/test_tcg_n06_media_audio_loaders.rs` |
| `Telegram/SourceFiles/media/audio/media_audio_local_cache.cpp` | `TCG-N34` | `MediaAudioLocalCacheAudio` | `crates/gpui_telegram_media/src/media/audio/media_audio_local_cache.rs` | 340 | `tests/test_tcg_n34_media_audio_local_cache.rs` |
| `Telegram/SourceFiles/media/audio/media_audio_local_cache.h` | `TCG-N34` | `MediaAudioLocalCacheAudio` | `crates/gpui_telegram_media/src/media/audio/media_audio_local_cache.rs` | 46 | `tests/test_tcg_n34_media_audio_local_cache.rs` |
| `Telegram/SourceFiles/media/audio/media_audio_track.cpp` | `TCG-N06` | `MediaAudioTrackAudio` | `crates/gpui_telegram_media/src/media/audio/media_audio_track.rs` | 373 | `tests/test_tcg_n06_media_audio_track.rs` |
| `Telegram/SourceFiles/media/audio/media_audio_track.h` | `TCG-N06` | `MediaAudioTrackAudio` | `crates/gpui_telegram_media/src/media/audio/media_audio_track.rs` | 139 | `tests/test_tcg_n06_media_audio_track.rs` |
| `Telegram/SourceFiles/media/audio/media_child_ffmpeg_loader.cpp` | `TCG-N06` | `MediaChildFfmpegLoaderAudio` | `crates/gpui_telegram_media/src/media/audio/media_child_ffmpeg_loader.rs` | 113 | `tests/test_tcg_n06_media_child_ffmpeg_loader.rs` |
| `Telegram/SourceFiles/media/audio/media_child_ffmpeg_loader.h` | `TCG-N06` | `MediaChildFfmpegLoaderAudio` | `crates/gpui_telegram_media/src/media/audio/media_child_ffmpeg_loader.rs` | 63 | `tests/test_tcg_n06_media_child_ffmpeg_loader.rs` |
| `Telegram/SourceFiles/media/clip/media_clip_check_streaming.cpp` | `TCG-N09` | `MediaClipCheckStreamingClip` | `crates/gpui_telegram_media/src/media/clip/media_clip_check_streaming.rs` | 97 | `tests/test_tcg_n09_media_clip_check_streaming.rs` |
| `Telegram/SourceFiles/media/clip/media_clip_check_streaming.h` | `TCG-N09` | `MediaClipCheckStreamingClip` | `crates/gpui_telegram_media/src/media/clip/media_clip_check_streaming.rs` | 22 | `tests/test_tcg_n09_media_clip_check_streaming.rs` |
| `Telegram/SourceFiles/media/clip/media_clip_ffmpeg.cpp` | `TCG-N09` | `MediaClipFfmpegClip` | `crates/gpui_telegram_media/src/media/clip/media_clip_ffmpeg.rs` | 510 | `tests/test_tcg_n09_media_clip_ffmpeg.rs` |
| `Telegram/SourceFiles/media/clip/media_clip_ffmpeg.h` | `TCG-N09` | `MediaClipFfmpegClip` | `crates/gpui_telegram_media/src/media/clip/media_clip_ffmpeg.rs` | 120 | `tests/test_tcg_n09_media_clip_ffmpeg.rs` |
| `Telegram/SourceFiles/media/clip/media_clip_implementation.cpp` | `TCG-N09` | `MediaClipImplementationClip` | `crates/gpui_telegram_media/src/media/clip/media_clip_implementation.rs` | 31 | `tests/test_tcg_n09_media_clip_implementation.rs` |
| `Telegram/SourceFiles/media/clip/media_clip_implementation.h` | `TCG-N09` | `MediaClipImplementationClip` | `crates/gpui_telegram_media/src/media/clip/media_clip_implementation.rs` | 75 | `tests/test_tcg_n09_media_clip_implementation.rs` |
| `Telegram/SourceFiles/media/clip/media_clip_reader.cpp` | `TCG-N09` | `MediaClipReaderClip` | `crates/gpui_telegram_media/src/media/clip/media_clip_reader.rs` | 1065 | `tests/test_tcg_n09_media_clip_reader.rs` |
| `Telegram/SourceFiles/media/clip/media_clip_reader.h` | `TCG-N09` | `MediaClipReaderClip` | `crates/gpui_telegram_media/src/media/clip/media_clip_reader.rs` | 248 | `tests/test_tcg_n09_media_clip_reader.rs` |
| `Telegram/SourceFiles/media/media_audio_waveform.cpp` | `TCG-N07` | `MediaAudioWaveform` | `crates/gpui_telegram_media/src/media/media_audio_waveform.rs` | 258 | `tests/test_tcg_n07_media_audio_waveform.rs` |
| `Telegram/SourceFiles/media/media_audio_waveform.h` | `TCG-N07` | `MediaAudioWaveform` | `crates/gpui_telegram_media/src/media/media_audio_waveform.rs` | 27 | `tests/test_tcg_n07_media_audio_waveform.rs` |
| `Telegram/SourceFiles/media/media_common.h` | `TCG-N06` | `MediaCommon` | `crates/gpui_telegram_media/src/media/media_common.rs` | 60 | `tests/test_tcg_n06_media_common.rs` |
| `Telegram/SourceFiles/media/media_video_canvas.cpp` | `TCG-N06` | `MediaVideoCanvas` | `crates/gpui_telegram_media/src/media/media_video_canvas.rs` | 70 | `tests/test_tcg_n06_media_video_canvas.rs` |
| `Telegram/SourceFiles/media/media_video_canvas.h` | `TCG-N06` | `MediaVideoCanvas` | `crates/gpui_telegram_media/src/media/media_video_canvas.rs` | 28 | `tests/test_tcg_n06_media_video_canvas.rs` |
| `Telegram/SourceFiles/media/media_video_encode.cpp` | `TCG-N06` | `MediaVideoEncode` | `crates/gpui_telegram_media/src/media/media_video_encode.rs` | 2766 | `tests/test_tcg_n06_media_video_encode.rs` |
| `Telegram/SourceFiles/media/media_video_encode.h` | `TCG-N06` | `MediaVideoEncode` | `crates/gpui_telegram_media/src/media/media_video_encode.rs` | 189 | `tests/test_tcg_n06_media_video_encode.rs` |
| `Telegram/SourceFiles/media/media_video_frames.cpp` | `TCG-N06` | `MediaVideoFrames` | `crates/gpui_telegram_media/src/media/media_video_frames.rs` | 400 | `tests/test_tcg_n06_media_video_frames.rs` |
| `Telegram/SourceFiles/media/media_video_frames.h` | `TCG-N06` | `MediaVideoFrames` | `crates/gpui_telegram_media/src/media/media_video_frames.rs` | 45 | `tests/test_tcg_n06_media_video_frames.rs` |
| `Telegram/SourceFiles/media/player/media_player.style` | `TCG-N08` | `MediaPlayerPlayerStyleSheet` | `crates/gpui_telegram_desktop/src/theme/media/media_player_style.rs` | 351 | `tests/test_tcg_n08_media_player.rs` |
| `Telegram/SourceFiles/media/player/media_player_button.cpp` | `TCG-N08` | `MediaPlayerButtonPlayer` | `crates/gpui_telegram_media/src/media/player/media_player_button.rs` | 523 | `tests/test_tcg_n08_media_player_button.rs` |
| `Telegram/SourceFiles/media/player/media_player_button.h` | `TCG-N08` | `MediaPlayerButtonPlayer` | `crates/gpui_telegram_media/src/media/player/media_player_button.rs` | 162 | `tests/test_tcg_n08_media_player_button.rs` |
| `Telegram/SourceFiles/media/player/media_player_dropdown.cpp` | `TCG-N08` | `MediaPlayerDropdownPlayer` | `crates/gpui_telegram_media/src/media/player/media_player_dropdown.rs` | 1048 | `tests/test_tcg_n08_media_player_dropdown.rs` |
| `Telegram/SourceFiles/media/player/media_player_dropdown.h` | `TCG-N08` | `MediaPlayerDropdownPlayer` | `crates/gpui_telegram_media/src/media/player/media_player_dropdown.rs` | 193 | `tests/test_tcg_n08_media_player_dropdown.rs` |
| `Telegram/SourceFiles/media/player/media_player_float.cpp` | `TCG-N08` | `MediaPlayerFloatPlayer` | `crates/gpui_telegram_media/src/media/player/media_player_float.rs` | 667 | `tests/test_tcg_n08_media_player_float.rs` |
| `Telegram/SourceFiles/media/player/media_player_float.h` | `TCG-N08` | `MediaPlayerFloatPlayer` | `crates/gpui_telegram_media/src/media/player/media_player_float.rs` | 287 | `tests/test_tcg_n08_media_player_float.rs` |
| `Telegram/SourceFiles/media/player/media_player_instance.cpp` | `TCG-N08` | `MediaPlayerInstancePlayer` | `crates/gpui_telegram_media/src/media/player/media_player_instance.rs` | 1629 | `tests/test_tcg_n08_media_player_instance.rs` |
| `Telegram/SourceFiles/media/player/media_player_instance.h` | `TCG-N08` | `MediaPlayerInstancePlayer` | `crates/gpui_telegram_media/src/media/player/media_player_instance.rs` | 365 | `tests/test_tcg_n08_media_player_instance.rs` |
| `Telegram/SourceFiles/media/player/media_player_listen_tracker.cpp` | `TCG-N08` | `MediaPlayerListenTrackerPlayer` | `crates/gpui_telegram_media/src/media/player/media_player_listen_tracker.rs` | 102 | `tests/test_tcg_n08_media_player_listen_tracker.rs` |
| `Telegram/SourceFiles/media/player/media_player_listen_tracker.h` | `TCG-N08` | `MediaPlayerListenTrackerPlayer` | `crates/gpui_telegram_media/src/media/player/media_player_listen_tracker.rs` | 40 | `tests/test_tcg_n08_media_player_listen_tracker.rs` |
| `Telegram/SourceFiles/media/player/media_player_panel.cpp` | `TCG-N08` | `MediaPlayerPanelPlayer` | `crates/gpui_telegram_media/src/media/player/media_player_panel.rs` | 485 | `tests/test_tcg_n08_media_player_panel.rs` |
| `Telegram/SourceFiles/media/player/media_player_panel.h` | `TCG-N08` | `MediaPlayerPanelPlayer` | `crates/gpui_telegram_media/src/media/player/media_player_panel.rs` | 117 | `tests/test_tcg_n08_media_player_panel.rs` |
| `Telegram/SourceFiles/media/player/media_player_volume_controller.cpp` | `TCG-N08` | `MediaPlayerVolumeControllerPlayer` | `crates/gpui_telegram_media/src/media/player/media_player_volume_controller.rs` | 115 | `tests/test_tcg_n08_media_player_volume_controller.rs` |
| `Telegram/SourceFiles/media/player/media_player_volume_controller.h` | `TCG-N08` | `MediaPlayerVolumeControllerPlayer` | `crates/gpui_telegram_media/src/media/player/media_player_volume_controller.rs` | 52 | `tests/test_tcg_n08_media_player_volume_controller.rs` |
| `Telegram/SourceFiles/media/player/media_player_widget.cpp` | `TCG-N08` | `MediaPlayerWidgetPlayer` | `crates/gpui_telegram_media/src/media/player/media_player_widget.rs` | 831 | `tests/test_tcg_n08_media_player_widget.rs` |
| `Telegram/SourceFiles/media/player/media_player_widget.h` | `TCG-N08` | `MediaPlayerWidgetPlayer` | `crates/gpui_telegram_media/src/media/player/media_player_widget.rs` | 169 | `tests/test_tcg_n08_media_player_widget.rs` |
| `Telegram/SourceFiles/media/stories/media_stories.style` | `TCG-N38` | `MediaStoriesStoriesStyleSheet` | `crates/gpui_telegram_desktop/src/theme/media/media_stories_style.rs` | 80 | `tests/test_tcg_n38_media_stories.rs` |
| `Telegram/SourceFiles/media/stories/media_stories_caption_full_view.cpp` | `TCG-N38` | `MediaStoriesCaptionFullViewStories` | `crates/gpui_telegram_media/src/media/stories/media_stories_caption_full_view.rs` | 226 | `tests/test_tcg_n38_media_stories_caption_full_view.rs` |
| `Telegram/SourceFiles/media/stories/media_stories_caption_full_view.h` | `TCG-N38` | `MediaStoriesCaptionFullViewStories` | `crates/gpui_telegram_media/src/media/stories/media_stories_caption_full_view.rs` | 54 | `tests/test_tcg_n38_media_stories_caption_full_view.rs` |
| `Telegram/SourceFiles/media/stories/media_stories_controller.cpp` | `TCG-N38` | `MediaStoriesControllerStories` | `crates/gpui_telegram_media/src/media/stories/media_stories_controller.rs` | 2139 | `tests/test_tcg_n38_media_stories_controller.rs` |
| `Telegram/SourceFiles/media/stories/media_stories_controller.h` | `TCG-N38` | `MediaStoriesControllerStories` | `crates/gpui_telegram_media/src/media/stories/media_stories_controller.rs` | 406 | `tests/test_tcg_n38_media_stories_controller.rs` |
| `Telegram/SourceFiles/media/stories/media_stories_delegate.cpp` | `TCG-N38` | `MediaStoriesDelegateStories` | `crates/gpui_telegram_media/src/media/stories/media_stories_delegate.rs` | 9 | `tests/test_tcg_n38_media_stories_delegate.rs` |
| `Telegram/SourceFiles/media/stories/media_stories_delegate.h` | `TCG-N38` | `MediaStoriesDelegateStories` | `crates/gpui_telegram_media/src/media/stories/media_stories_delegate.rs` | 64 | `tests/test_tcg_n38_media_stories_delegate.rs` |
| `Telegram/SourceFiles/media/stories/media_stories_header.cpp` | `TCG-N38` | `MediaStoriesHeaderStories` | `crates/gpui_telegram_media/src/media/stories/media_stories_header.rs` | 976 | `tests/test_tcg_n38_media_stories_header.rs` |
| `Telegram/SourceFiles/media/stories/media_stories_header.h` | `TCG-N38` | `MediaStoriesHeaderStories` | `crates/gpui_telegram_media/src/media/stories/media_stories_header.rs` | 114 | `tests/test_tcg_n38_media_stories_header.rs` |
| `Telegram/SourceFiles/media/stories/media_stories_reactions.cpp` | `TCG-N38` | `MediaStoriesReactionsStories` | `crates/gpui_telegram_media/src/media/stories/media_stories_reactions.rs` | 1480 | `tests/test_tcg_n38_media_stories_reactions.rs` |
| `Telegram/SourceFiles/media/stories/media_stories_reactions.h` | `TCG-N38` | `MediaStoriesReactionsStories` | `crates/gpui_telegram_media/src/media/stories/media_stories_reactions.rs` | 150 | `tests/test_tcg_n38_media_stories_reactions.rs` |
| `Telegram/SourceFiles/media/stories/media_stories_recent_views.cpp` | `TCG-N38` | `MediaStoriesRecentViewsStories` | `crates/gpui_telegram_media/src/media/stories/media_stories_recent_views.rs` | 671 | `tests/test_tcg_n38_media_stories_recent_views.rs` |
| `Telegram/SourceFiles/media/stories/media_stories_recent_views.h` | `TCG-N38` | `MediaStoriesRecentViewsStories` | `crates/gpui_telegram_media/src/media/stories/media_stories_recent_views.rs` | 138 | `tests/test_tcg_n38_media_stories_recent_views.rs` |
| `Telegram/SourceFiles/media/stories/media_stories_reply.cpp` | `TCG-N38` | `MediaStoriesReplyStories` | `crates/gpui_telegram_media/src/media/stories/media_stories_reply.rs` | 1038 | `tests/test_tcg_n38_media_stories_reply.rs` |
| `Telegram/SourceFiles/media/stories/media_stories_reply.h` | `TCG-N38` | `MediaStoriesReplyStories` | `crates/gpui_telegram_media/src/media/stories/media_stories_reply.rs` | 188 | `tests/test_tcg_n38_media_stories_reply.rs` |
| `Telegram/SourceFiles/media/stories/media_stories_repost_view.cpp` | `TCG-N38` | `MediaStoriesRepostViewStories` | `crates/gpui_telegram_media/src/media/stories/media_stories_repost_view.rs` | 298 | `tests/test_tcg_n38_media_stories_repost_view.rs` |
| `Telegram/SourceFiles/media/stories/media_stories_repost_view.h` | `TCG-N38` | `MediaStoriesRepostViewStories` | `crates/gpui_telegram_media/src/media/stories/media_stories_repost_view.rs` | 71 | `tests/test_tcg_n38_media_stories_repost_view.rs` |
| `Telegram/SourceFiles/media/stories/media_stories_share.cpp` | `TCG-N38` | `MediaStoriesShareStories` | `crates/gpui_telegram_media/src/media/stories/media_stories_share.rs` | 319 | `tests/test_tcg_n38_media_stories_share.rs` |
| `Telegram/SourceFiles/media/stories/media_stories_share.h` | `TCG-N38` | `MediaStoriesShareStories` | `crates/gpui_telegram_media/src/media/stories/media_stories_share.rs` | 34 | `tests/test_tcg_n38_media_stories_share.rs` |
| `Telegram/SourceFiles/media/stories/media_stories_sibling.cpp` | `TCG-N38` | `MediaStoriesSiblingStories` | `crates/gpui_telegram_media/src/media/stories/media_stories_sibling.rs` | 436 | `tests/test_tcg_n38_media_stories_sibling.rs` |
| `Telegram/SourceFiles/media/stories/media_stories_sibling.h` | `TCG-N38` | `MediaStoriesSiblingStories` | `crates/gpui_telegram_media/src/media/stories/media_stories_sibling.rs` | 81 | `tests/test_tcg_n38_media_stories_sibling.rs` |
| `Telegram/SourceFiles/media/stories/media_stories_slider.cpp` | `TCG-N38` | `MediaStoriesSliderStories` | `crates/gpui_telegram_media/src/media/stories/media_stories_slider.rs` | 156 | `tests/test_tcg_n38_media_stories_slider.rs` |
| `Telegram/SourceFiles/media/stories/media_stories_slider.h` | `TCG-N38` | `MediaStoriesSliderStories` | `crates/gpui_telegram_media/src/media/stories/media_stories_slider.rs` | 63 | `tests/test_tcg_n38_media_stories_slider.rs` |
| `Telegram/SourceFiles/media/stories/media_stories_stealth.cpp` | `TCG-N38` | `MediaStoriesStealthStories` | `crates/gpui_telegram_media/src/media/stories/media_stories_stealth.rs` | 428 | `tests/test_tcg_n38_media_stories_stealth.rs` |
| `Telegram/SourceFiles/media/stories/media_stories_stealth.h` | `TCG-N38` | `MediaStoriesStealthStories` | `crates/gpui_telegram_media/src/media/stories/media_stories_stealth.rs` | 45 | `tests/test_tcg_n38_media_stories_stealth.rs` |
| `Telegram/SourceFiles/media/stories/media_stories_view.cpp` | `TCG-N38` | `MediaStoriesViewStories` | `crates/gpui_telegram_media/src/media/stories/media_stories_view.rs` | 194 | `tests/test_tcg_n38_media_stories_view.rs` |
| `Telegram/SourceFiles/media/stories/media_stories_view.h` | `TCG-N38` | `MediaStoriesViewStories` | `crates/gpui_telegram_media/src/media/stories/media_stories_view.rs` | 148 | `tests/test_tcg_n38_media_stories_view.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_audio_track.cpp` | `TCG-N52` | `MediaStreamingAudioTrackStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_audio_track.rs` | 253 | `tests/test_tcg_n52_media_streaming_audio_track.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_audio_track.h` | `TCG-N52` | `MediaStreamingAudioTrackStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_audio_track.rs` | 95 | `tests/test_tcg_n52_media_streaming_audio_track.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_common.h` | `TCG-N52` | `MediaStreamingCommonStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_common.rs` | 213 | `tests/test_tcg_n52_media_streaming_common.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_document.cpp` | `TCG-N52` | `MediaStreamingDocumentStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_document.rs` | 368 | `tests/test_tcg_n52_media_streaming_document.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_document.h` | `TCG-N52` | `MediaStreamingDocumentStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_document.rs` | 98 | `tests/test_tcg_n52_media_streaming_document.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_file.cpp` | `TCG-N52` | `MediaStreamingFileStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_file.rs` | 511 | `tests/test_tcg_n52_media_streaming_file.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_file.h` | `TCG-N52` | `MediaStreamingFileStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_file.rs` | 126 | `tests/test_tcg_n52_media_streaming_file.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_file_delegate.h` | `TCG-N52` | `MediaStreamingFileDelegateStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_file_delegate.rs` | 41 | `tests/test_tcg_n52_media_streaming_file_delegate.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_instance.cpp` | `TCG-N52` | `MediaStreamingInstanceStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_instance.rs` | 270 | `tests/test_tcg_n52_media_streaming_instance.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_instance.h` | `TCG-N52` | `MediaStreamingInstanceStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_instance.rs` | 105 | `tests/test_tcg_n52_media_streaming_instance.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_loader.cpp` | `TCG-N52` | `MediaStreamingLoaderStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_loader.rs` | 94 | `tests/test_tcg_n52_media_streaming_loader.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_loader.h` | `TCG-N52` | `MediaStreamingLoaderStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_loader.rs` | 80 | `tests/test_tcg_n52_media_streaming_loader.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_loader_local.cpp` | `TCG-N52` | `MediaStreamingLoaderLocalStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_loader_local.rs` | 115 | `tests/test_tcg_n52_media_streaming_loader_local.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_loader_local.h` | `TCG-N52` | `MediaStreamingLoaderLocalStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_loader_local.rs` | 55 | `tests/test_tcg_n52_media_streaming_loader_local.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_loader_mtproto.cpp` | `TCG-N52` | `MediaStreamingLoaderMtprotoStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_loader_mtproto.rs` | 221 | `tests/test_tcg_n52_media_streaming_loader_mtproto.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_loader_mtproto.h` | `TCG-N52` | `MediaStreamingLoaderMtprotoStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_loader_mtproto.rs` | 81 | `tests/test_tcg_n52_media_streaming_loader_mtproto.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_native_frame_mac.h` | `TCG-N52` | `MediaStreamingNativeFrameMacStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_native_frame_mac.rs` | 22 | `tests/test_tcg_n52_media_streaming_native_frame_mac.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_native_frame_mac.mm` | `TCG-N52` | `MediaStreamingNativeFrameMacStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_native_frame_mac.rs` | 129 | `tests/test_tcg_n52_media_streaming_native_frame_mac.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_player.cpp` | `TCG-N08` | `MediaStreamingPlayerStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_player.rs` | 1043 | `tests/test_tcg_n08_media_streaming_player.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_player.h` | `TCG-N08` | `MediaStreamingPlayerStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_player.rs` | 224 | `tests/test_tcg_n08_media_streaming_player.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_reader.cpp` | `TCG-N52` | `MediaStreamingReaderStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_reader.rs` | 1442 | `tests/test_tcg_n52_media_streaming_reader.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_reader.h` | `TCG-N52` | `MediaStreamingReaderStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_reader.rs` | 286 | `tests/test_tcg_n52_media_streaming_reader.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_round_preview.cpp` | `TCG-N52` | `MediaStreamingRoundPreviewStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_round_preview.rs` | 62 | `tests/test_tcg_n52_media_streaming_round_preview.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_round_preview.h` | `TCG-N52` | `MediaStreamingRoundPreviewStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_round_preview.rs` | 35 | `tests/test_tcg_n52_media_streaming_round_preview.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_utility.cpp` | `TCG-N52` | `MediaStreamingUtilityStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_utility.rs` | 456 | `tests/test_tcg_n52_media_streaming_utility.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_utility.h` | `TCG-N52` | `MediaStreamingUtilityStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_utility.rs` | 99 | `tests/test_tcg_n52_media_streaming_utility.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_video_track.cpp` | `TCG-N52` | `MediaStreamingVideoTrackStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_video_track.rs` | 1440 | `tests/test_tcg_n52_media_streaming_video_track.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_video_track.h` | `TCG-N52` | `MediaStreamingVideoTrackStreaming` | `crates/gpui_telegram_media/src/media/streaming/media_streaming_video_track.rs` | 188 | `tests/test_tcg_n52_media_streaming_video_track.rs` |
| `Telegram/SourceFiles/media/system_media_controls_manager.cpp` | `TCG-N06` | `SystemMediaControlsManager` | `crates/gpui_telegram_media/src/media/system_media_controls_manager.rs` | 635 | `tests/test_tcg_n06_system_media_controls_manager.rs` |
| `Telegram/SourceFiles/media/system_media_controls_manager.h` | `TCG-N06` | `SystemMediaControlsManager` | `crates/gpui_telegram_media/src/media/system_media_controls_manager.rs` | 69 | `tests/test_tcg_n06_system_media_controls_manager.rs` |
| `Telegram/SourceFiles/media/system_media_controls_video.h` | `TCG-N06` | `SystemMediaControlsVideo` | `crates/gpui_telegram_media/src/media/system_media_controls_video.rs` | 50 | `tests/test_tcg_n06_system_media_controls_video.rs` |
| `Telegram/SourceFiles/media/view/media_view.style` | `TCG-N10` | `MediaViewViewStyleSheet` | `crates/gpui_telegram_desktop/src/theme/media/media_view_style.rs` | 1294 | `tests/test_tcg_n10_media_view.rs` |
| `Telegram/SourceFiles/media/view/media_view_group_thumbs.cpp` | `TCG-N10` | `MediaViewGroupThumbsView` | `crates/gpui_telegram_media/src/media/view/media_view_group_thumbs.rs` | 1030 | `tests/test_tcg_n10_media_view_group_thumbs.rs` |
| `Telegram/SourceFiles/media/view/media_view_group_thumbs.h` | `TCG-N10` | `MediaViewGroupThumbsView` | `crates/gpui_telegram_media/src/media/view/media_view_group_thumbs.rs` | 178 | `tests/test_tcg_n10_media_view_group_thumbs.rs` |
| `Telegram/SourceFiles/media/view/media_view_metal_texture.h` | `TCG-N10` | `MediaViewMetalTextureView` | `crates/gpui_telegram_media/src/media/view/media_view_metal_texture.rs` | 46 | `tests/test_tcg_n10_media_view_metal_texture.rs` |
| `Telegram/SourceFiles/media/view/media_view_metal_texture.mm` | `TCG-N10` | `MediaViewMetalTextureView` | `crates/gpui_telegram_media/src/media/view/media_view_metal_texture.rs` | 163 | `tests/test_tcg_n10_media_view_metal_texture.rs` |
| `Telegram/SourceFiles/media/view/media_view_open_common.cpp` | `TCG-N10` | `MediaViewOpenCommonView` | `crates/gpui_telegram_media/src/media/view/media_view_open_common.rs` | 41 | `tests/test_tcg_n10_media_view_open_common.rs` |
| `Telegram/SourceFiles/media/view/media_view_open_common.h` | `TCG-N10` | `MediaViewOpenCommonView` | `crates/gpui_telegram_media/src/media/view/media_view_open_common.rs` | 184 | `tests/test_tcg_n10_media_view_open_common.rs` |
| `Telegram/SourceFiles/media/view/media_view_overlay_opengl.cpp` | `TCG-N10` | `MediaViewOverlayOpenglView` | `crates/gpui_telegram_media/src/media/view/media_view_overlay_opengl.rs` | 1332 | `tests/test_tcg_n10_media_view_overlay_opengl.rs` |
| `Telegram/SourceFiles/media/view/media_view_overlay_opengl.h` | `TCG-N10` | `MediaViewOverlayOpenglView` | `crates/gpui_telegram_media/src/media/view/media_view_overlay_opengl.rs` | 176 | `tests/test_tcg_n10_media_view_overlay_opengl.rs` |
| `Telegram/SourceFiles/media/view/media_view_overlay_raster.cpp` | `TCG-N10` | `MediaViewOverlayRasterView` | `crates/gpui_telegram_media/src/media/view/media_view_overlay_raster.rs` | 373 | `tests/test_tcg_n10_media_view_overlay_raster.rs` |
| `Telegram/SourceFiles/media/view/media_view_overlay_raster.h` | `TCG-N10` | `MediaViewOverlayRasterView` | `crates/gpui_telegram_media/src/media/view/media_view_overlay_raster.rs` | 85 | `tests/test_tcg_n10_media_view_overlay_raster.rs` |
| `Telegram/SourceFiles/media/view/media_view_overlay_renderer.h` | `TCG-N10` | `MediaViewOverlayRendererView` | `crates/gpui_telegram_media/src/media/view/media_view_overlay_renderer.rs` | 57 | `tests/test_tcg_n10_media_view_overlay_renderer.rs` |
| `Telegram/SourceFiles/media/view/media_view_overlay_rhi.cpp` | `TCG-N10` | `MediaViewOverlayRhiView` | `crates/gpui_telegram_media/src/media/view/media_view_overlay_rhi.rs` | 2102 | `tests/test_tcg_n10_media_view_overlay_rhi.rs` |
| `Telegram/SourceFiles/media/view/media_view_overlay_rhi.h` | `TCG-N10` | `MediaViewOverlayRhiView` | `crates/gpui_telegram_media/src/media/view/media_view_overlay_rhi.rs` | 229 | `tests/test_tcg_n10_media_view_overlay_rhi.rs` |
| `Telegram/SourceFiles/media/view/media_view_overlay_widget.cpp` | `TCG-N10` | `MediaViewOverlayWidgetView` | `crates/gpui_telegram_media/src/media/view/media_view_overlay_widget.rs` | 9081 | `tests/test_tcg_n10_media_view_overlay_widget.rs` |
| `Telegram/SourceFiles/media/view/media_view_overlay_widget.h` | `TCG-N10` | `MediaViewOverlayWidgetView` | `crates/gpui_telegram_media/src/media/view/media_view_overlay_widget.rs` | 912 | `tests/test_tcg_n10_media_view_overlay_widget.rs` |
| `Telegram/SourceFiles/media/view/media_view_pip.cpp` | `TCG-N10` | `MediaViewPipView` | `crates/gpui_telegram_media/src/media/view/media_view_pip.rs` | 1898 | `tests/test_tcg_n10_media_view_pip.rs` |
| `Telegram/SourceFiles/media/view/media_view_pip.h` | `TCG-N10` | `MediaViewPipView` | `crates/gpui_telegram_media/src/media/view/media_view_pip.rs` | 321 | `tests/test_tcg_n10_media_view_pip.rs` |
| `Telegram/SourceFiles/media/view/media_view_pip_opengl.cpp` | `TCG-N10` | `MediaViewPipOpenglView` | `crates/gpui_telegram_media/src/media/view/media_view_pip_opengl.rs` | 827 | `tests/test_tcg_n10_media_view_pip_opengl.rs` |
| `Telegram/SourceFiles/media/view/media_view_pip_opengl.h` | `TCG-N10` | `MediaViewPipOpenglView` | `crates/gpui_telegram_media/src/media/view/media_view_pip_opengl.rs` | 130 | `tests/test_tcg_n10_media_view_pip_opengl.rs` |
| `Telegram/SourceFiles/media/view/media_view_pip_raster.cpp` | `TCG-N10` | `MediaViewPipRasterView` | `crates/gpui_telegram_media/src/media/view/media_view_pip_raster.rs` | 226 | `tests/test_tcg_n10_media_view_pip_raster.rs` |
| `Telegram/SourceFiles/media/view/media_view_pip_raster.h` | `TCG-N10` | `MediaViewPipRasterView` | `crates/gpui_telegram_media/src/media/view/media_view_pip_raster.rs` | 66 | `tests/test_tcg_n10_media_view_pip_raster.rs` |
| `Telegram/SourceFiles/media/view/media_view_pip_renderer.h` | `TCG-N10` | `MediaViewPipRendererView` | `crates/gpui_telegram_media/src/media/view/media_view_pip_renderer.rs` | 37 | `tests/test_tcg_n10_media_view_pip_renderer.rs` |
| `Telegram/SourceFiles/media/view/media_view_pip_rhi.cpp` | `TCG-N10` | `MediaViewPipRhiView` | `crates/gpui_telegram_media/src/media/view/media_view_pip_rhi.rs` | 1294 | `tests/test_tcg_n10_media_view_pip_rhi.rs` |
| `Telegram/SourceFiles/media/view/media_view_pip_rhi.h` | `TCG-N10` | `MediaViewPipRhiView` | `crates/gpui_telegram_media/src/media/view/media_view_pip_rhi.rs` | 188 | `tests/test_tcg_n10_media_view_pip_rhi.rs` |
| `Telegram/SourceFiles/media/view/media_view_playback_controls.cpp` | `TCG-N10` | `MediaViewPlaybackControlsView` | `crates/gpui_telegram_media/src/media/view/media_view_playback_controls.rs` | 557 | `tests/test_tcg_n10_media_view_playback_controls.rs` |
| `Telegram/SourceFiles/media/view/media_view_playback_controls.h` | `TCG-N10` | `MediaViewPlaybackControlsView` | `crates/gpui_telegram_media/src/media/view/media_view_playback_controls.rs` | 150 | `tests/test_tcg_n10_media_view_playback_controls.rs` |
| `Telegram/SourceFiles/media/view/media_view_playback_progress.cpp` | `TCG-N10` | `MediaViewPlaybackProgressView` | `crates/gpui_telegram_media/src/media/view/media_view_playback_progress.rs` | 169 | `tests/test_tcg_n10_media_view_playback_progress.rs` |
| `Telegram/SourceFiles/media/view/media_view_playback_progress.h` | `TCG-N10` | `MediaViewPlaybackProgressView` | `crates/gpui_telegram_media/src/media/view/media_view_playback_progress.rs` | 62 | `tests/test_tcg_n10_media_view_playback_progress.rs` |
| `Telegram/SourceFiles/media/view/media_view_playback_sponsored.cpp` | `TCG-N10` | `MediaViewPlaybackSponsoredView` | `crates/gpui_telegram_media/src/media/view/media_view_playback_sponsored.rs` | 768 | `tests/test_tcg_n10_media_view_playback_sponsored.rs` |
| `Telegram/SourceFiles/media/view/media_view_playback_sponsored.h` | `TCG-N10` | `MediaViewPlaybackSponsoredView` | `crates/gpui_telegram_media/src/media/view/media_view_playback_sponsored.rs` | 83 | `tests/test_tcg_n10_media_view_playback_sponsored.rs` |
| `Telegram/SourceFiles/media/view/media_view_recognition_selection.cpp` | `TCG-N10` | `MediaViewRecognitionSelectionView` | `crates/gpui_telegram_media/src/media/view/media_view_recognition_selection.rs` | 415 | `tests/test_tcg_n10_media_view_recognition_selection.rs` |
| `Telegram/SourceFiles/media/view/media_view_recognition_selection.h` | `TCG-N10` | `MediaViewRecognitionSelectionView` | `crates/gpui_telegram_media/src/media/view/media_view_recognition_selection.rs` | 95 | `tests/test_tcg_n10_media_view_recognition_selection.rs` |
| `Telegram/SourceFiles/media/view/media_view_video_stream.cpp` | `TCG-N10` | `MediaViewVideoStreamView` | `crates/gpui_telegram_media/src/media/view/media_view_video_stream.rs` | 395 | `tests/test_tcg_n10_media_view_video_stream.rs` |
| `Telegram/SourceFiles/media/view/media_view_video_stream.h` | `TCG-N10` | `MediaViewVideoStreamView` | `crates/gpui_telegram_media/src/media/view/media_view_video_stream.rs` | 104 | `tests/test_tcg_n10_media_view_video_stream.rs` |

## Deep Source Reading Evidence

### `Telegram/SourceFiles/ffmpeg/ffmpeg_bytes_io_wrap.h`
- **Classes & Structs**: ReadFileWrap, ReadBytesWrap, WriteBytesWrap
- **Core Methods & Signals**: Read, AVERROR, Seek, Write, int64_t, checkedSeek
- **Target**: `FfmpegBytesIoWrap` in `crates/gpui_telegram_media/src/media/ffmpeg_bytes_io_wrap.rs`

### `Telegram/SourceFiles/ffmpeg/ffmpeg_frame_generator.cpp`
- **Classes & Structs**: FrameGenerator, ReadFrame
- **Core Methods & Signals**: Impl::Impl, Impl::Read, Impl::read, Impl::Seek, Impl::seek, Impl::renderCurrent
- **Target**: `FfmpegFrameGenerator` in `crates/gpui_telegram_media/src/media/ffmpeg_frame_generator.rs`

### `Telegram/SourceFiles/ffmpeg/ffmpeg_frame_generator.h`
- **Classes & Structs**: FrameGenerator, Impl
- **Core Methods & Signals**: FrameGenerator, count, rate, renderNext, renderCurrent, jumpToStart
- **Target**: `FfmpegFrameGenerator` in `crates/gpui_telegram_media/src/media/ffmpeg_frame_generator.rs`

### `Telegram/SourceFiles/ffmpeg/ffmpeg_utility.cpp`
- **Classes & Structs**: AVCodecContext, HwAccelDescriptor, AlignedFrameStorageLayout, AVPixelFormat, GetFormatMethod
- **Core Methods & Signals**: Platform::LoadLibrary, IODeleter::operator, gsl::finally, QString::fromUtf8, FormatDeleter::operator, CodecDeleter::operator
- **Target**: `FfmpegUtility` in `crates/gpui_telegram_media/src/media/ffmpeg_utility.rs`

### `Telegram/SourceFiles/ffmpeg/ffmpeg_utility.h`
- **Classes & Structs**: QImage, AvErrorWrap, Packet, IODeleter, FormatDeleter, FormatSettings, CodecDeleter, CodecDescriptor, FrameDeleter, SwscaleDeleter, SwresampleDeleter, IOPointer
- **Core Methods & Signals**: QString::fromUtf8, failed, release, RestrictToCustomIO, ClearFrameMemory, LogError
- **Target**: `FfmpegUtility` in `crates/gpui_telegram_media/src/media/ffmpeg_utility.rs`

### `Telegram/SourceFiles/media/audio/media_audio.cpp`
- **Classes & Structs**: FFMpegAttributesReader, FFMpegWaveformCounter
- **Core Methods & Signals**: Player::mixer, Player::Mixer, Track::createStream, Track::destroyStream, Track::reattach, Track::detach
- **Target**: `MediaAudioAudio` in `crates/gpui_telegram_media/src/media/audio/media_audio.rs`

### `Telegram/SourceFiles/media/audio/media_audio.h`
- **Classes & Structs**: PreparedFileInformation, ExternalSoundData, ExternalSoundPart, TimePoint, DeviceResolvedId, Instance, Fader, Loaders, State, TrackState, Mixer, Track
- **Core Methods & Signals**: gsl::make_span, Start, Finish, IsAttachedToDevice, AttachToDevice, ScheduleDetachFromDeviceSafe
- **Target**: `MediaAudioAudio` in `crates/gpui_telegram_media/src/media/audio/media_audio.rs`

### `Telegram/SourceFiles/media/audio/media_audio_capture.cpp`
- **Classes & Structs**: Instance, Private
- **Core Methods & Signals**: Instance::Instance, Instance::start, Audio::Current, Instance::stop, Instance::pause, Instance::check
- **Target**: `MediaAudioCaptureAudio` in `crates/gpui_telegram_media/src/media/audio/media_audio_capture.rs`

### `Telegram/SourceFiles/media/audio/media_audio_capture.h`
- **Classes & Structs**: Update, Error, Chunk, Result, Instance, Inner
- **Core Methods & Signals**: Start, Finish, Instance, check, rpl::updated, rpl::startedChanges
- **Target**: `MediaAudioCaptureAudio` in `crates/gpui_telegram_media/src/media/audio/media_audio_capture.rs`

### `Telegram/SourceFiles/media/audio/media_audio_capture_common.h`
- **Classes & Structs**: Result
- **Target**: `MediaAudioCaptureCommonAudio` in `crates/gpui_telegram_media/src/media/audio/media_audio_capture_common.rs`

