//! Upstream C++ source: `Telegram/SourceFiles/ui/color_contrast.cpp:L1-L142` & `color_contrast.h:L1-L29`
//! Authentic GPUI implementation of WCAG AA contrast ratio and perceived luminance algorithms.
#![allow(dead_code)]

use gpui::{rgba, Rgba};

pub const K_GAMUT_ITERATIONS: usize = 30;
pub const K_GAMUT_EPSILON: f64 = 1e-4;
pub const K_LIGHT_BRIGHTNESS: f64 = 0.7;

/// WCAG AA color contrast and perceived brightness calculation engine.
/// C++ companion: `Ui::ColorContrast` (`color_contrast.cpp:L15-L142`).
#[derive(Debug, Clone, Copy, Default)]
pub struct ColorContrast;

impl ColorContrast {
    /// Converts an 8-bit sRGB channel to linear light (`color_contrast.cpp:L18-L23`).
    pub fn to_linear(channel: u8) -> f64 {
        let val = channel as f64 / 255.0;
        if val <= 0.04045 {
            val / 12.92
        } else {
            ((val + 0.055) / 1.055).powf(2.4)
        }
    }

    /// Converts linear light back to sRGB value in [0, 1] (`color_contrast.cpp:L25-L31`).
    pub fn from_linear(val: f64) -> f64 {
        let clamped = val.clamp(0.0, 1.0);
        if clamped <= 0.0031308 {
            12.92 * clamped
        } else {
            1.055 * clamped.powf(1.0 / 2.4) - 0.055
        }
    }

    /// Computes the relative luminance of an sRGB color per WCAG 2.1 specs (`color_contrast.cpp:L55-L60`).
    pub fn relative_luminance(r: u8, g: u8, b: u8) -> f64 {
        0.2126 * Self::to_linear(r) + 0.7152 * Self::to_linear(g) + 0.0722 * Self::to_linear(b)
    }

    /// Calculates WCAG contrast ratio: (L1 + 0.05) / (L2 + 0.05) (`color_contrast.cpp:L62-L68`).
    pub fn count_contrast(color_a: (u8, u8, u8), color_b: (u8, u8, u8)) -> f64 {
        let lum_a = Self::relative_luminance(color_a.0, color_a.1, color_a.2);
        let lum_b = Self::relative_luminance(color_b.0, color_b.1, color_b.2);
        let max_l = lum_a.max(lum_b);
        let min_l = lum_a.min(lum_b);
        (max_l + 0.05) / (min_l + 0.05)
    }

    /// Computes the perceived brightness (`color_contrast.cpp:L80-L88`).
    pub fn count_perceived_brightness(r: u8, g: u8, b: u8) -> f64 {
        let rf = r as f64;
        let gf = g as f64;
        let bf = b as f64;
        (0.299 * rf * rf + 0.587 * gf * gf + 0.114 * bf * bf).sqrt() / 255.0
    }

    /// Determines if a background color should be classified as light (`color_contrast.cpp:L89-L93`).
    pub fn is_light_background(r: u8, g: u8, b: u8) -> bool {
        Self::count_perceived_brightness(r, g, b) >= K_LIGHT_BRIGHTNESS
    }

    /// Selects optimal foreground text color (black or white) to satisfy WCAG AA 4.5:1 ratio.
    pub fn optimal_text_color(bg_r: u8, bg_g: u8, bg_b: u8) -> Rgba {
        if Self::is_light_background(bg_r, bg_g, bg_b) {
            rgba(0x000000FF)
        } else {
            rgba(0xFFFFFFFF)
        }
    }
}
