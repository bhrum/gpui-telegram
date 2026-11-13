//! Poll Option Link Dynamic Thumbnail Generator.
//!
//! Citations:
//! - Upstream: `Telegram/SourceFiles/poll/poll_link_thumbnail.h:L10-L20`
//! - Upstream: `Telegram/SourceFiles/poll/poll_link_thumbnail.cpp:L16-L55`
//! - Pixel Math & Constants:
//!   - Corner radius: `radius = size / 4.0;` (poll_link_thumbnail.cpp:L39)
//!   - Placeholder icon: `st::historyPollLinkPlaceholderIcon` (poll_link_thumbnail.cpp:L41)
//!   - Background brush: `st::windowBgOver` (poll_link_thumbnail.cpp:L38)

use gpui::*;

/// Dynamic thumbnail renderer for poll link previews.
/// C++ citation: `Poll::LinkThumbnail` (poll_link_thumbnail.cpp:L16-L57).
pub struct PollLinkThumbnail {
    size: f32,
    corner_radius: f32,
    custom_image_url: Option<String>,
    palette_version: u32,
}

impl PollLinkThumbnail {
    pub fn new(size: f32) -> Self {
        let corner_radius = size / 4.0; // poll_link_thumbnail.cpp:L39
        Self {
            size,
            corner_radius,
            custom_image_url: None,
            palette_version: 1,
        }
    }

    /// Sets optional remote thumbnail image URL if loaded.
    pub fn with_image_url(mut self, url: impl Into<String>) -> Self {
        self.custom_image_url = Some(url.into());
        self
    }

    /// Computed corner radius based on upstream ratio `size / 4.0`.
    pub fn corner_radius(&self) -> f32 {
        self.corner_radius
    }

    pub fn size(&self) -> f32 {
        self.size
    }
}

impl Default for PollLinkThumbnail {
    fn default() -> Self {
        Self::new(48.0)
    }
}

impl Render for PollLinkThumbnail {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let size_px = px(self.size);
        let radius_px = px(self.corner_radius);

        div()
            .w(size_px)
            .h(size_px)
            .rounded(radius_px)
            .bg(rgb(0xF4F4F5)) // st::windowBgOver
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .text_size(px(self.size * 0.45))
                    .text_color(rgb(0x71717A))
                    .child("🔗")
            )
    }
}
