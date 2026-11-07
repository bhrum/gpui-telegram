//! macOS Apple Vision Framework Native Text Recognition
//! Upstream Source: Telegram/SourceFiles/platform/mac/text_recognition_mac.h, Telegram/SourceFiles/platform/mac/text_recognition_mac.mm
//! Subsystem: platform | Feature ID: TCG-N46 | Disposition: rust_domain_or_server_1to1

use gpui_telegram_core::error::Result;
use serde::{Deserialize, Serialize};

/// Recognized text line item with normalized and canvas bounding boxes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextRecognitionLine {
    pub text: String,
    pub confidence: f32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub glyph_boxes: Vec<(f32, f32, f32, f32)>,
}

/// Recognition request and output bundle matching Apple Vision `VNRecognizedTextObservation`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextRecognitionResult {
    pub success: bool,
    pub items: Vec<TextRecognitionLine>,
}

/// Apple Vision framework text recognition engine for macOS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextRecognitionMacMac {
    pub is_available: bool,
    pub recognition_level: String,
    pub total_recognized_lines: usize,
}

impl TextRecognitionMacMac {
    pub fn new() -> Self {
        Self {
            is_available: true, // macOS 10.15+
            recognition_level: "VNRequestTextRecognitionLevelAccurate".into(),
            total_recognized_lines: 0,
        }
    }

    /// Converts Apple Vision normalized coordinates `(x, y, w, h)` to canvas coordinates.
    ///
    /// Upstream formula:
    /// `x = box.origin.x * image_width`
    /// `y = (1.0 - box.origin.y - box.size.height) * image_height`
    pub fn convert_bounding_box(
        norm_x: f32,
        norm_y: f32,
        norm_w: f32,
        norm_h: f32,
        image_w: f32,
        image_h: f32,
    ) -> (f32, f32, f32, f32) {
        let x = norm_x * image_w;
        let y = (1.0 - norm_y - norm_h) * image_h;
        let w = norm_w * image_w;
        let h = norm_h * image_h;
        (x, y, w, h)
    }

    pub fn parse_recognized_observations(
        &mut self,
        raw_items: Vec<(String, f32, (f32, f32, f32, f32))>,
        image_w: f32,
        image_h: f32,
    ) -> Result<TextRecognitionResult> {
        let mut lines = Vec::new();
        for (text, conf, norm_rect) in raw_items {
            let (x, y, w, h) = Self::convert_bounding_box(
                norm_rect.0, norm_rect.1, norm_rect.2, norm_rect.3, image_w, image_h,
            );
            lines.push(TextRecognitionLine {
                text,
                confidence: conf,
                x,
                y,
                width: w,
                height: h,
                glyph_boxes: Vec::new(),
            });
        }
        self.total_recognized_lines += lines.len();
        Ok(TextRecognitionResult {
            success: true,
            items: lines,
        })
    }
}

impl Default for TextRecognitionMacMac {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_recognition_bounding_box_conversion() {
        let mut engine = TextRecognitionMacMac::new();
        assert!(engine.is_available);

        // Normalized box at top-left: x=0.1, y=0.8, w=0.5, h=0.2 on 1000x1000 image
        let (x, y, w, h) = TextRecognitionMacMac::convert_bounding_box(0.1, 0.8, 0.5, 0.2, 1000.0, 1000.0);
        assert_eq!(x, 100.0);
        assert!((y - 0.0).abs() < 1e-4); // (1.0 - 0.8 - 0.2) = 0
        assert_eq!(w, 500.0);
        assert_eq!(h, 200.0);

        let res = engine
            .parse_recognized_observations(
                vec![("Hello World".into(), 0.99, (0.1, 0.8, 0.5, 0.2))],
                1000.0,
                1000.0,
            )
            .unwrap();
        assert!(res.success);
        assert_eq!(res.items.len(), 1);
        assert_eq!(res.items[0].text, "Hello World");
    }
}
