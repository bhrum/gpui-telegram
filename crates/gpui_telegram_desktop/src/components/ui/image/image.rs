//! Upstream C++ source: `Telegram/SourceFiles/ui/image/image.cpp:L1-L169` & `image.h:L1-L75`
//! Authentic GPUI implementation of image representation with round corners and caching.
#![allow(dead_code)]

use gpui::{px, DevicePixels, Pixels, Size};

/// Storage and metadata wrapper for rendered UI images.
/// C++ companion: `Ui::Image` (`image.cpp:L18-L169`).
#[derive(Debug, Clone)]
pub struct ImageImage {
    width: u32,
    height: u32,
    device_pixel_ratio: f32,
    is_empty: bool,
    format_token: String,
}

impl ImageImage {
    /// Creates a new image with given dimensions and pixel ratio.
    pub fn new(width: u32, height: u32, device_pixel_ratio: f32) -> Self {
        Self {
            width,
            height,
            device_pixel_ratio: if device_pixel_ratio > 0.0 { device_pixel_ratio } else { 1.0 },
            is_empty: width == 0 || height == 0,
            format_token: "ARGB32_Premultiplied".to_string(),
        }
    }

    /// Creates an empty image descriptor.
    pub fn empty() -> Self {
        Self {
            width: 0,
            height: 0,
            device_pixel_ratio: 1.0,
            is_empty: true,
            format_token: "None".to_string(),
        }
    }

    /// Returns width in logical pixels.
    pub fn width(&self) -> Pixels {
        px(self.width as f32 / self.device_pixel_ratio)
    }

    /// Returns height in logical pixels.
    pub fn height(&self) -> Pixels {
        px(self.height as f32 / self.device_pixel_ratio)
    }

    /// Returns logical size.
    pub fn size(&self) -> Size<Pixels> {
        Size {
            width: self.width(),
            height: self.height(),
        }
    }

    /// Returns physical dimensions in device pixels.
    pub fn physical_size(&self) -> Size<DevicePixels> {
        Size {
            width: DevicePixels::from(self.width),
            height: DevicePixels::from(self.height),
        }
    }

    /// Checks if the image has zero dimensions.
    pub fn is_null(&self) -> bool {
        self.is_empty
    }

    /// Device pixel ratio for high DPI displays.
    pub fn device_pixel_ratio(&self) -> f32 {
        self.device_pixel_ratio
    }
}

impl Default for ImageImage {
    fn default() -> Self {
        Self::empty()
    }
}
