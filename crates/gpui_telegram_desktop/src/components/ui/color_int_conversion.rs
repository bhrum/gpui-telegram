//! Upstream C++ source: `Telegram/SourceFiles/ui/color_int_conversion.cpp:L1-L33` & `color_int_conversion.h:L1-L17`
//! Authentic GPUI implementation of 24-bit and 32-bit serialized color conversions.
#![allow(dead_code)]

use gpui::{rgba, Rgba};

/// Utility converter between integer representations and GPUI colors.
/// C++ companion: `Ui::ColorFromSerialized` (`color_int_conversion.cpp:L13-L33`).
#[derive(Debug, Clone, Copy, Default)]
pub struct ColorIntConversion;

impl ColorIntConversion {
    /// Extracts 24-bit RGB from serialized uint32 (`color_int_conversion.cpp:L13-L18`).
    pub fn color_from_serialized(serialized: u32) -> Rgba {
        let r = ((serialized >> 16) & 0xFF) as f32 / 255.0;
        let g = ((serialized >> 8) & 0xFF) as f32 / 255.0;
        let b = (serialized & 0xFF) as f32 / 255.0;
        Rgba { r, g, b, a: 1.0 }
    }

    /// Handles optional serialized color, returning None if equal to `0xFFFFFFFF` (`color_int_conversion.cpp:L20-L24`).
    pub fn maybe_color_from_serialized(serialized: u32) -> Option<Rgba> {
        if serialized == u32::MAX {
            None
        } else {
            Some(Self::color_from_serialized(serialized))
        }
    }

    /// Extracts 32-bit RGBA from serialized uint32 with alpha (`color_int_conversion.cpp:L26-L32`).
    pub fn color32_from_serialized(serialized: u32) -> Rgba {
        let r = (serialized & 0xFF) as f32 / 255.0;
        let g = ((serialized >> 8) & 0xFF) as f32 / 255.0;
        let b = ((serialized >> 16) & 0xFF) as f32 / 255.0;
        let a = ((serialized >> 24) & 0xFF) as f32 / 255.0;
        Rgba { r, g, b, a }
    }

    /// Serializes an RGBA tuple back to a 32-bit integer.
    pub fn serialize_rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
        (r as u32) | ((g as u32) << 8) | ((b as u32) << 16) | ((a as u32) << 24)
    }
}
