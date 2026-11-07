//! Font and typography manager for Instant View
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IvFontFamily {
    #[default]
    System,
    Serif,
    SansSerif,
    Monospace,
}

#[derive(Debug, Clone)]
pub struct IvFontManager {
    pub current_family: IvFontFamily,
    pub base_size: f32,
    pub line_height: f32,
}

impl Default for IvFontManager {
    fn default() -> Self {
        Self {
            current_family: IvFontFamily::System,
            base_size: 16.0,
            line_height: 1.6,
        }
    }
}

impl IvFontManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_family(&mut self, family: IvFontFamily) {
        self.current_family = family;
    }

    pub fn set_base_size(&mut self, size: f32) {
        self.base_size = size.clamp(12.0, 28.0);
    }
}
