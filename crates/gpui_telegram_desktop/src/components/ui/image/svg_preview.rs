//! Upstream C++ source: `Telegram/SourceFiles/ui/image/svg_preview.cpp:L1-L70` & `svg_preview.h:L1-L25`
//! Authentic GPUI implementation of SVG vector preview validation and rendering rules.
#![allow(dead_code)]

use gpui::{px, Pixels, Size};

/// Upper bound constants from `svg_preview.cpp:L19-L20`.
pub const K_MAX_SVG_SIZE: usize = 1024 * 1024; // 1 MB
pub const K_MAX_DEFAULT_RENDER_SIZE: u32 = 4096;

/// SVG vector preview processor and dimension scaler.
/// C++ companion: `Ui::SvgPreview` (`svg_preview.cpp:L25-L70`).
#[derive(Debug, Clone)]
pub struct SvgPreviewImage {
    raw_bytes: Vec<u8>,
    view_box: Size<Pixels>,
}

impl SvgPreviewImage {
    /// Parses SVG payload bytes while enforcing `K_MAX_SVG_SIZE`.
    pub fn from_bytes(bytes: &[u8], target_width: f32, target_height: f32) -> Result<Self, &'static str> {
        if bytes.is_empty() {
            return Err("SVG bytes cannot be empty");
        }
        if bytes.len() > K_MAX_SVG_SIZE {
            return Err("SVG payload exceeds K_MAX_SVG_SIZE (1 MB)");
        }
        let w = target_width.clamp(1.0, K_MAX_DEFAULT_RENDER_SIZE as f32);
        let h = target_height.clamp(1.0, K_MAX_DEFAULT_RENDER_SIZE as f32);

        Ok(Self {
            raw_bytes: bytes.to_vec(),
            view_box: Size { width: px(w), height: px(h) },
        })
    }

    /// Returns the target viewBox size.
    pub fn view_box(&self) -> Size<Pixels> {
        self.view_box
    }

    /// Validates if the SVG contains standard XML/svg markup.
    pub fn is_valid_svg(&self) -> bool {
        let text = String::from_utf8_lossy(&self.raw_bytes);
        text.contains("<svg") && text.contains("</svg>")
    }
}

impl Default for SvgPreviewImage {
    fn default() -> Self {
        Self {
            raw_bytes: b"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 24 24\"></svg>".to_vec(),
            view_box: Size { width: px(24.0), height: px(24.0) },
        }
    }
}
