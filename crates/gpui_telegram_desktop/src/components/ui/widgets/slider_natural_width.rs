//! Custom Natural Width Provider for Discrete Settings Sliders.
//! Reference: Telegram/SourceFiles/ui/widgets/slider_natural_width.h:L14-L20

#![allow(dead_code, unused_imports, unused_variables)]

/// Target symbol providing natural width computation for segmented slider layouts.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SliderNaturalWidthWidgets {
    pub natural_width: f32,
    pub min_width: f32,
    pub max_width: f32,
    pub padding_horizontal: f32,
}

impl SliderNaturalWidthWidgets {
    pub fn new() -> Self {
        Self {
            natural_width: 0.0,
            min_width: 120.0,
            max_width: 800.0,
            padding_horizontal: 16.0,
        }
    }

    /// Computes natural width by summing individual section content widths with spacing.
    pub fn compute_natural_width(&mut self, content_widths: &[f32], spacing: f32) -> f32 {
        if content_widths.is_empty() {
            self.natural_width = self.min_width;
            return self.natural_width;
        }
        let total_content: f32 = content_widths.iter().sum();
        let total_spacing = spacing * (content_widths.len().saturating_sub(1) as f32);
        let computed = total_content + total_spacing + (self.padding_horizontal * 2.0);
        self.natural_width = computed.clamp(self.min_width, self.max_width);
        self.natural_width
    }

    pub fn set_natural_width(&mut self, width: f32) {
        self.natural_width = width.clamp(self.min_width, self.max_width);
    }
}
