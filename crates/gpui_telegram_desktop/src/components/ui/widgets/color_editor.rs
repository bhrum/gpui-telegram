//! Authentic GPUI Color Picker & Theme Color Editor Component.
//! Reference: Telegram/SourceFiles/ui/widgets/color_editor.cpp:L20-L60
//! Reference: Telegram/SourceFiles/ui/widgets/color_editor.h:L20-L106

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::{rgba, Rgba};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorEditorMode {
    #[default]
    RGB,
    HSB,
    Hex,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HsbColor {
    pub hue: f32,        // [0.0, 360.0]
    pub saturation: f32, // [0.0, 1.0]
    pub brightness: f32, // [0.0, 1.0]
    pub alpha: f32,      // [0.0, 1.0]
}

impl Default for HsbColor {
    fn default() -> Self {
        Self {
            hue: 200.0,
            saturation: 0.8,
            brightness: 0.9,
            alpha: 1.0,
        }
    }
}

/// Target symbol for visual palette color picker and theme color editor.
#[derive(Debug, Clone)]
pub struct ColorEditorWidgets {
    pub mode: ColorEditorMode,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
    pub hsb: HsbColor,
    pub hex_string: String,
    pub has_alpha_channel: bool,
    pub is_pipette_active: bool,
}

impl Default for ColorEditorWidgets {
    fn default() -> Self {
        Self::new_rgb(42, 171, 238) // Telegram default blue: #2AABEE
    }
}

impl ColorEditorWidgets {
    pub fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        let mut editor = Self {
            mode: ColorEditorMode::RGB,
            red: r,
            green: g,
            blue: b,
            alpha: 255,
            hsb: HsbColor::default(),
            hex_string: format!("#{r:02X}{g:02X}{b:02X}"),
            has_alpha_channel: false,
            is_pipette_active: false,
        };
        editor.update_hsb_from_rgb();
        editor
    }

    pub fn set_rgb(&mut self, r: u8, g: u8, b: u8) {
        self.red = r;
        self.green = g;
        self.blue = b;
        self.hex_string = if self.has_alpha_channel {
            format!("#{r:02X}{g:02X}{b:02X}{:02X}", self.alpha)
        } else {
            format!("#{r:02X}{g:02X}{b:02X}")
        };
        self.update_hsb_from_rgb();
    }

    /// Sets color from hex string formatted as #RRGGBB or #RRGGBBAA.
    pub fn set_hex(&mut self, hex: &str) -> bool {
        let clean = hex.trim_start_matches('#');
        if clean.len() == 6 {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&clean[0..2], 16),
                u8::from_str_radix(&clean[2..4], 16),
                u8::from_str_radix(&clean[4..6], 16),
            ) {
                self.set_rgb(r, g, b);
                return true;
            }
        } else if clean.len() == 8 {
            if let (Ok(r), Ok(g), Ok(b), Ok(a)) = (
                u8::from_str_radix(&clean[0..2], 16),
                u8::from_str_radix(&clean[2..4], 16),
                u8::from_str_radix(&clean[4..6], 16),
                u8::from_str_radix(&clean[6..8], 16),
            ) {
                self.alpha = a;
                self.set_rgb(r, g, b);
                return true;
            }
        }
        false
    }

    /// Converts internal RGB channels to HSB values:
    /// Reference: `color_editor.cpp:L34-L36`
    pub fn update_hsb_from_rgb(&mut self) {
        let r = self.red as f32 / 255.0;
        let g = self.green as f32 / 255.0;
        let b = self.blue as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let brightness = max;
        let saturation = if max > 0.0 { delta / max } else { 0.0 };
        let mut hue = if delta > 0.0 {
            let base = if (max - r).abs() < 1e-5 {
                (g - b) / delta
            } else if (max - g).abs() < 1e-5 {
                2.0 + (b - r) / delta
            } else {
                4.0 + (r - g) / delta
            };
            base * 60.0
        } else {
            0.0
        };
        if hue < 0.0 {
            hue += 360.0;
        }

        self.hsb = HsbColor {
            hue,
            saturation,
            brightness,
            alpha: self.alpha as f32 / 255.0,
        };
    }

    pub fn to_gpui_rgba(&self) -> Rgba {
        let val = ((self.red as u32) << 24)
            | ((self.green as u32) << 16)
            | ((self.blue as u32) << 8)
            | (self.alpha as u32);
        rgba(val)
    }
}
