//! Native GPUI Video Viewport Component with Aspect Ratio Containment & PiP Bubble

use crate::theme::assets::{tg_icon, IconName};
use crate::theme::palette::TdesktopThemePalette;
use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_telegram_webrtc::calls::calls_video_bubble::{BubbleCorner, CallsVideoBubble};
use std::sync::Arc;

/// Helper encoding RGBA8 byte buffer to in-memory PNG for gpui::Image
pub fn encode_rgba_to_gpui_image(
    width: u32,
    height: u32,
    rgba_bytes: &[u8],
) -> Option<Arc<gpui::Image>> {
    if width == 0 || height == 0 {
        return None;
    }
    let expected_len = (width as usize)
        .checked_mul(height as usize)
        .and_then(|n| n.checked_mul(4))?;
    if rgba_bytes.len() != expected_len {
        return None;
    }
    let mut png_bytes = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut png_bytes);
    use image::ImageEncoder;
    if encoder
        .write_image(rgba_bytes, width, height, image::ExtendedColorType::Rgba8)
        .is_err()
    {
        return None;
    }
    Some(Arc::new(gpui::Image::from_bytes(
        gpui::ImageFormat::Png,
        png_bytes,
    )))
}

/// Native GPUI Video Viewport rendering remote video stream with aspect ratio containment
/// and floating local camera preview bubble.
pub struct VideoViewport {
    /// Cached remote video frame for GPUI renderer
    pub remote_frame: Option<Arc<gpui::Image>>,
    /// Cached local camera preview frame
    pub local_frame: Option<Arc<gpui::Image>>,
    /// Dimensions of remote video stream (width, height)
    pub remote_resolution: (u32, u32),
    /// Dimensions of local video stream (width, height)
    pub local_resolution: (u32, u32),
    /// Aspect ratio containment mode (defaults to ObjectFit::Contain)
    pub aspect_ratio_mode: ObjectFit,
    /// Whether remote peer video stream is active and unmuted
    pub is_remote_video_active: bool,
    /// Whether local camera preview is enabled
    pub is_local_camera_active: bool,
    /// Whether screen sharing is active
    pub is_screen_sharing: bool,
    /// Draggable picture-in-picture local camera bubble
    pub bubble: CallsVideoBubble,
    /// Peer avatar local path (if cached) for fallback
    pub avatar_path: Option<String>,
    /// Fallback peer initials
    pub peer_initials: String,
    /// Fallback avatar background color
    pub avatar_color: u32,
    /// Peer display name
    pub peer_name: String,
    /// Theme palette
    pub palette: TdesktopThemePalette,
}

impl VideoViewport {
    pub fn new(peer_name: impl Into<String>, palette: TdesktopThemePalette) -> Self {
        let name = peer_name.into();
        let initials = name
            .chars()
            .next()
            .unwrap_or('T')
            .to_uppercase()
            .to_string();
        Self {
            remote_frame: None,
            local_frame: None,
            remote_resolution: (0, 0),
            local_resolution: (0, 0),
            aspect_ratio_mode: ObjectFit::Contain,
            is_remote_video_active: false,
            is_local_camera_active: false,
            is_screen_sharing: false,
            bubble: CallsVideoBubble::new(),
            avatar_path: None,
            peer_initials: initials,
            avatar_color: 0x5288C1,
            peer_name: name,
            palette,
        }
    }

    /// Sets remote video frame from decoded image
    pub fn set_remote_frame(&mut self, frame: Option<Arc<gpui::Image>>, width: u32, height: u32) {
        if frame.is_none() || width == 0 || height == 0 {
            self.remote_resolution = (0, 0);
            self.is_remote_video_active = false;
            self.remote_frame = None;
        } else {
            self.remote_resolution = (width, height);
            self.is_remote_video_active = true;
            self.remote_frame = frame;
        }
    }

    /// Sets local camera preview frame
    pub fn set_local_frame(&mut self, frame: Option<Arc<gpui::Image>>, width: u32, height: u32) {
        if frame.is_none() || width == 0 || height == 0 {
            self.local_resolution = (0, 0);
            self.is_local_camera_active = false;
            self.local_frame = None;
        } else {
            self.local_resolution = (width, height);
            self.is_local_camera_active = true;
            self.local_frame = frame;
        }
        self.bubble.is_visible = self.is_local_camera_active;
    }

    /// Ingests raw RGBA8 frame bytes, converting them to an Arc<gpui::Image>
    pub fn ingest_raw_remote_rgba(
        &mut self,
        width: u32,
        height: u32,
        rgba_bytes: &[u8],
    ) -> bool {
        if let Some(img) = encode_rgba_to_gpui_image(width, height, rgba_bytes) {
            self.set_remote_frame(Some(img), width, height);
            true
        } else {
            false
        }
    }

    /// Ingests raw RGBA8 local camera preview bytes
    pub fn ingest_raw_local_rgba(&mut self, width: u32, height: u32, rgba_bytes: &[u8]) -> bool {
        if let Some(img) = encode_rgba_to_gpui_image(width, height, rgba_bytes) {
            self.set_local_frame(Some(img), width, height);
            true
        } else {
            false
        }
    }

    /// Toggles local camera preview
    pub fn set_local_camera_active(&mut self, active: bool) {
        self.is_local_camera_active = active;
        self.bubble.is_visible = active;
    }

    /// Sets screen sharing status
    pub fn set_screen_sharing(&mut self, active: bool) {
        self.is_screen_sharing = active;
    }

    /// Cycles the local camera bubble to the next corner
    pub fn cycle_bubble_corner(&mut self, parent_w: f32, parent_h: f32) {
        let next = match self.bubble.active_corner {
            BubbleCorner::TopLeft => BubbleCorner::TopRight,
            BubbleCorner::TopRight => BubbleCorner::BottomRight,
            BubbleCorner::BottomRight => BubbleCorner::BottomLeft,
            BubbleCorner::BottomLeft => BubbleCorner::TopLeft,
        };
        self.bubble.snap_to_corner(parent_w, parent_h, next);
    }

    /// Returns true if actively rendering video (either remote or local or screen share)
    pub fn has_active_video(&self) -> bool {
        self.is_remote_video_active || self.is_local_camera_active || self.is_screen_sharing
    }

    /// Renders the video viewport element with letterboxed remote stream and floating local PiP bubble
    pub fn render_viewport(&self, viewport_w: f32, viewport_h: f32) -> AnyElement {
        let pal = self.palette.clone();

        // Main Surface: Remote Video with ObjectFit::Contain or Avatar Fallback
        let main_surface: AnyElement = if let Some(ref remote_img) = self.remote_frame {
            div()
                .size_full()
                .bg(rgb(0x000000))
                .flex()
                .items_center()
                .justify_center()
                .overflow_hidden()
                .child({
                    let fit = match self.aspect_ratio_mode {
                        ObjectFit::Fill => ObjectFit::Fill,
                        ObjectFit::Contain => ObjectFit::Contain,
                        ObjectFit::Cover => ObjectFit::Cover,
                        ObjectFit::ScaleDown => ObjectFit::ScaleDown,
                        ObjectFit::None => ObjectFit::None,
                    };
                    img(remote_img.clone())
                        .size_full()
                        .object_fit(fit)
                })
                .into_any_element()
        } else {
            // Avatar Fallback view
            let initial = self.peer_initials.clone();
            let av_color = self.avatar_color;
            div()
                .size_full()
                .bg(rgb(pal.wallpaper_top))
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap(px(14.0))
                .child(
                    div()
                        .size(px(140.0))
                        .rounded_full()
                        .bg(TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.20))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(if let Some(ref path) = self.avatar_path {
                            div()
                                .size(px(112.0))
                                .rounded_full()
                                .overflow_hidden()
                                .child(img(path.clone()).size_full().object_fit(ObjectFit::Cover))
                                .into_any_element()
                        } else {
                            div()
                                .size(px(112.0))
                                .rounded_full()
                                .bg(rgb(av_color))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_size(px(40.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(pal.active_button_fg))
                                .child(initial)
                                .into_any_element()
                        }),
                )
                .child(
                    div()
                        .text_lg()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.text_primary))
                        .child(self.peer_name.clone()),
                )
                .into_any_element()
        };

        // Floating Local Camera Preview Bubble (PiP)
        let bubble_element: Option<AnyElement> = if self.is_local_camera_active {
            let bw = self.bubble.width;
            let bh = self.bubble.height;
            let safe_vw = if viewport_w.is_finite() { viewport_w.max(0.0) } else { 0.0 };
            let safe_vh = if viewport_h.is_finite() { viewport_h.max(0.0) } else { 0.0 };
            let raw_x = if self.bubble.x.is_finite() { self.bubble.x } else { 8.0 };
            let raw_y = if self.bubble.y.is_finite() { self.bubble.y } else { 8.0 };
            let bx = raw_x.clamp(8.0, (safe_vw - bw - 8.0).max(8.0));
            let by = raw_y.clamp(8.0, (safe_vh - bh - 8.0).max(8.0));

            let bubble_content: AnyElement = if let Some(ref local_img) = self.local_frame {
                img(local_img.clone())
                    .size_full()
                    .object_fit(ObjectFit::Cover)
                    .into_any_element()
            } else {
                div()
                    .size_full()
                    .bg(rgb(pal.card_bg))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(tg_icon(IconName::VideoCall, 24.0, pal.accent))
                    .into_any_element()
            };

            Some(
                div()
                    .absolute()
                    .left(px(bx))
                    .top(px(by))
                    .w(px(bw))
                    .h(px(bh))
                    .rounded(px(10.0))
                    .border_2()
                    .border_color(rgb(pal.accent))
                    .overflow_hidden()
                    .shadow_md()
                    .child(bubble_content)
                    .into_any_element(),
            )
        } else {
            None
        };

        div()
            .relative()
            .size_full()
            .child(main_surface)
            .when_some(bubble_element, |el, bubble| el.child(bubble))
            .into_any_element()
    }
}
