//! History Media Basic Views Subsystem & Adversarial Tests.
//!
//! Covers Slice 5 components matching upstream:
//! - `Telegram/SourceFiles/history/view/media/history_view_document.h/.cpp`
//! - `Telegram/SourceFiles/history/view/media/history_view_file.h/.cpp`
//! - `Telegram/SourceFiles/history/view/media/history_view_photo.h/.cpp`
//! - `Telegram/SourceFiles/history/view/media/history_view_gif.h/.cpp`
//! - `Telegram/SourceFiles/history/view/media/history_view_sticker.h/.cpp`
//! - `Telegram/SourceFiles/history/view/media/history_view_media_grouped.h/.cpp`
//! - `Telegram/SourceFiles/history/view/media/history_view_media_spoiler.h/.cpp`
//! - `Telegram/SourceFiles/history/view/media/history_view_media.h/.cpp`
//! - `Telegram/SourceFiles/history/view/media/history_view_media_common.h/.cpp`
//! - `Telegram/SourceFiles/history/view/media/history_view_video_message_seek.h/.cpp`

use gpui_telegram_desktop::views::stage::media::*;
use gpui_telegram_desktop::views::stage::view::media::*;

#[test]
fn test_document_download_status_codes_and_formatting() {
    let mut file_media = HistoryViewFileViewMedia::new(1024 * 1024); // 1 MB
    assert_eq!(file_media.status_size, STATUS_CODE_NOT_DOWNLOADED);
    assert_eq!(file_media.to_file_status(), MediaFileStatus::NotDownloaded);

    // Test transition to downloading (512 KB loaded)
    file_media.set_status_size(512 * 1024, 1024 * 1024, -1, 0);
    assert_eq!(file_media.status_size, 512 * 1024);
    assert_eq!(
        file_media.to_file_status(),
        MediaFileStatus::Downloading {
            loaded_bytes: 512 * 1024,
            total_bytes: 1024 * 1024,
        }
    );

    // Test transition to already downloaded
    file_media.set_status_size(STATUS_CODE_ALREADY_DOWNLOADED, 1024 * 1024, -1, 0);
    assert_eq!(file_media.to_file_status(), MediaFileStatus::Downloaded);

    // Test transition to failed
    file_media.set_status_size(STATUS_CODE_FAILED, 1024 * 1024, -1, 0);
    assert_eq!(file_media.to_file_status(), MediaFileStatus::Failed);

    // Test transition to audio played (30 seconds in, 180 seconds total)
    // Upstream code: `_statusSize = -(seconds + 1)` -> `-(30 + 1) = -31`
    file_media.set_status_size(-31, 1024 * 1024, 180, 180);
    assert_eq!(
        file_media.to_file_status(),
        MediaFileStatus::Played {
            seconds: 30,
            duration: 180,
        }
    );
    assert_eq!(file_media.status_text, "00:30");
}

#[test]
fn test_document_voice_playback_blobs_constants() {
    let cfg = VoicePlaybackBlobConfig::default();
    assert_eq!(cfg.minor_scale, 0.88);
    assert_eq!(cfg.major_scale, 0.85);
    assert_eq!(cfg.alpha, 0.25);
    assert_eq!(cfg.level_duration_ms, 265.0);
    assert_eq!(cfg.idle_level, 0.45);
    assert_eq!(cfg.minor_segments, 6);
    assert_eq!(cfg.major_segments, 8);

    assert_eq!(K_VOICE_BLOB_MINOR_SCALE, 0.88);
    assert_eq!(K_VOICE_BLOB_MAJOR_SCALE, 0.85);
    assert_eq!(K_VOICE_BLOB_LEVEL_DURATION_MS, 265.0);
    assert_eq!(K_VOICE_BLOB_IDLE_LEVEL, 0.45);
    assert_eq!(K_VOICE_BLOB_ALPHA, 0.25);
}

#[test]
fn test_photo_aspect_ratio_and_story_bounds() {
    assert_eq!(K_STORY_WIDTH, 720);
    assert_eq!(K_STORY_HEIGHT, 1280);
    assert_eq!(STORY_WIDTH, 720);
    assert_eq!(STORY_HEIGHT, 1280);

    let story_photo = HistoryViewPhotoViewMedia::new_story(9901);
    assert!(story_photo.state.dimensions.is_story());
    let ratio = story_photo.state.dimensions.ratio();
    assert!((ratio - (720.0 / 1280.0)).abs() < 1e-4);

    let (render_w, render_h) = story_photo.state.compute_render_size(360.0);
    assert!(render_w >= PHOTO_MIN_SIZE && render_w <= PHOTO_MAX_SIZE);
    assert!(render_h >= 240.0 && render_h <= 480.0);

    let mut regular_photo = HistoryViewPhotoViewMedia::new(9902, 1920, 1080);
    assert!(!regular_photo.state.dimensions.is_story());
    let (rw, rh) = regular_photo.state.compute_render_size(360.0);
    assert_eq!(rw, 360.0);
    assert!((rh - (360.0 / (1920.0 / 1080.0))).abs() < 1.0);

    // Test spoiler reveal
    regular_photo.state.is_spoiler = true;
    assert!(!regular_photo.state.is_revealed);
    regular_photo.reveal_spoiler();
    assert!(regular_photo.state.is_revealed);
}

#[test]
fn test_gif_inline_area_and_preview_intervals() {
    assert_eq!(K_MAX_INLINE_AREA, 1920 * 1080);
    assert_eq!(K_MAX_INSTANT_VIEW_INLINE_AREA, 1920 * 1920);
    assert_eq!(K_SEEK_PREVIEW_INTERVAL_MS, 100);
    assert_eq!(K_MAX_GIF_FORWARDED_BAR_LINES, 4);
    assert_eq!(K_USE_NON_BLURRED_THRESHOLD, 240);

    let gif_view = HistoryViewGifViewMedia::new(5001, 3840, 2160, 0); // 4K GIF, area exceeds 1920*1080
    assert!(gif_view.state.is_looping);
    let (w, h) = gif_view.state.compute_render_size(360.0);
    assert_eq!(w, 360.0);
    assert!((h - (360.0 / (16.0 / 9.0))).abs() < 1.0);

    let mut round_video = HistoryViewGifViewMedia::new(5002, 480, 480, 15);
    round_video.state.is_video_message_round = true;
    let (rw, rh) = round_video.state.compute_render_size(360.0);
    assert_eq!(rw, 240.0);
    assert_eq!(rh, 240.0);
}

#[test]
fn test_sticker_sizing_multipliers_and_dice() {
    assert_eq!(K_MAX_SIZE_FIXED, 512);
    assert_eq!(K_MAX_EMOJI_SIZE_FIXED, 256);
    assert!((K_PREMIUM_MULTIPLIER - 1.49).abs() < 1e-4);
    assert_eq!(K_EMOJI_MULTIPLIER, 3);
    assert_eq!(K_MESSAGE_EFFECT_MULTIPLIER, 2);

    let mut sticker = HistoryViewStickerViewMedia::new(7001, StickerFormat::LottieTgs);
    assert_eq!(sticker.state.frames_count, 60);
    assert_eq!(sticker.compute_size(), 180.0);

    sticker.state.is_premium = true;
    assert_eq!(sticker.compute_size(), 180.0 * 1.49);

    // Frame advancement and at_the_end check
    assert!(!sticker.at_the_end());
    sticker.state.frame_index = 59;
    assert!(sticker.at_the_end());

    sticker.state.advance_frame();
    assert_eq!(sticker.state.frame_index, 0); // Looped back to start

    sticker.state.stop_on_last_frame = true;
    sticker.state.frame_index = 59;
    sticker.state.advance_frame();
    assert!(!sticker.state.is_playing); // Stopped on last frame
}

#[test]
fn test_grouped_media_mosaic_layout_algorithm() {
    assert_eq!(MAX_GROUP_SIZE, 10);
    assert_eq!(K_MAX_GROUPED_MEDIA_ITEMS, 10);

    let mut grouped = HistoryViewMediaGroupedViewMedia::new(GroupedMediaMode::Grid);
    for i in 1..=4 {
        assert!(grouped.add_item(GroupedMediaItem {
            item_id: i,
            width: 400.0,
            height: 300.0,
            sides: RectParts::default(),
            title: Some(format!("Photo {}", i)),
        }));
    }
    grouped.recount_layout(360.0);

    assert_eq!(grouped.layout_geometry.len(), 4);
    // 4 items arrange into a 2x2 grid
    let g0 = &grouped.layout_geometry[0];
    let g1 = &grouped.layout_geometry[1];
    let g2 = &grouped.layout_geometry[2];
    let g3 = &grouped.layout_geometry[3];

    // Top-left
    assert!(g0.sides.top && g0.sides.left && !g0.sides.bottom && !g0.sides.right);
    // Top-right
    assert!(g1.sides.top && g1.sides.right && !g1.sides.bottom && !g1.sides.left);
    // Bottom-left
    assert!(g2.sides.bottom && g2.sides.left && !g2.sides.top && !g2.sides.right);
    // Bottom-right
    assert!(g3.sides.bottom && g3.sides.right && !g3.sides.top && !g3.sides.left);

    // Column playlist layout test
    let mut playlist = HistoryViewMediaGroupedViewMedia::new(GroupedMediaMode::Column);
    for i in 1..=3 {
        playlist.add_item(GroupedMediaItem {
            item_id: i,
            width: 360.0,
            height: 52.0,
            sides: RectParts::default(),
            title: Some(format!("Track {}", i)),
        });
    }
    playlist.recount_layout(360.0);
    assert_eq!(playlist.layout_geometry.len(), 3);
    assert!(playlist.layout_geometry[0].sides.top);
    assert!(playlist.layout_geometry[2].sides.bottom);
    assert_eq!(playlist.total_height, 52.0 * 3.0);
}

#[test]
fn test_media_spoiler_particle_animation_and_price_tags() {
    let mut spoiler = HistoryViewMediaSpoilerViewMedia::new()
        .with_paid_stars(50);
    assert_eq!(spoiler.paid_tag.as_ref().unwrap().price, 50);
    assert!(!spoiler.is_revealed());

    assert_eq!(spoiler.spoiler_state.particles.len(), 64);
    assert_eq!(spoiler.spoiler_state.opacity(), 1.0);

    // Test animation advancement
    let x0 = spoiler.spoiler_state.particles[0].x;
    spoiler.spoiler_state.advance_frame();
    assert_eq!(spoiler.spoiler_state.current_frame, 1);
    assert_ne!(spoiler.spoiler_state.particles[0].x, x0);

    // Reveal transition
    spoiler.reveal();
    assert!(spoiler.is_revealed());
    spoiler.spoiler_state.advance_frame();
    assert!(spoiler.spoiler_state.opacity() < 1.0);
}

#[test]
fn test_video_message_seek_and_ghost_jump() {
    assert_eq!(K_SHOW_DURATION_MS, 220);
    assert_eq!(K_HIDE_DURATION_MS, 150);
    assert_eq!(K_GRAB_DURATION_MS, 150);
    assert_eq!(K_GHOST_DURATION_MS, 150);
    assert_eq!(K_TRACK_OPACITY, 0.2);
    assert_eq!(K_SHADOW_INNER_STOP, 0.7);
    assert_eq!(K_GHOST_JUMP, 0.9);

    let mut seek = HistoryViewVideoMessageSeekViewMedia::new();
    seek.set_progress(0.05);
    assert_eq!(seek.progress, 0.05);

    // Minor scrub: ghost jump NOT triggered
    seek.set_dragged_progress(0.07);
    assert_eq!(seek.progress, 0.07);
    assert_eq!(seek.ghost_progress, 0.0);

    // Major scrub across > 0.9: ghost jump triggered! (0.98 - 0.07 = 0.91 > 0.9)
    seek.set_dragged_progress(0.98);
    assert_eq!(seek.progress, 0.98);
    assert_eq!(seek.ghost_progress, 0.07);
}

#[test]
fn test_timestamp_links_and_youtube_duration() {
    assert_eq!(K_MAX_YOUTUBE_TIMESTAMP_DURATION_SECS, 360_000);

    // Test timestamp parsing: "01:23:45"
    let parsed = parse_timestamp_from_match(Some("1"), "23", "45");
    assert_eq!(parsed, Some(1 * 3600 + 23 * 60 + 45)); // 5025s

    // Test timestamp parsing without hours: "04:30"
    let parsed_mm_ss = parse_timestamp_from_match(None, "4", "30");
    assert_eq!(parsed_mm_ss, Some(4 * 60 + 30)); // 270s

    // Invalid seconds >= 60
    assert_eq!(parse_timestamp_from_match(None, "4", "65"), None);

    // Timestamp link base construction
    let link_base = build_timestamp_link_base(12345, 67890, 42);
    assert_eq!(link_base, "internal:media_timestamp?base=doc12345_67890_42&t=");
}

#[test]
fn test_media_common_downscaling_and_age_gating() {
    assert_eq!(K_MEDIA_UNLOCKED_TOOLTIP_DURATION_MS, 5000);
    assert_eq!(K_VERIFY_AGE_ABOUT_PREFIX, "cloud_lng_age_verify_about_");

    // 800x600 into 400x400 box -> 400x300
    let (scaled_w, scaled_h) = downscaled_size(800.0, 600.0, 400.0, 400.0);
    assert_eq!(scaled_w, 400.0);
    assert_eq!(scaled_h, 300.0);

    let mut common = HistoryViewMediaCommonViewMedia::new();
    assert!(!common.can_view_sensitive());

    common.set_verified_age(16);
    assert!(!common.can_view_sensitive());

    common.set_verified_age(18);
    assert!(common.can_view_sensitive());
}
