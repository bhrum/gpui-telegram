//! Upstream: `Telegram/SourceFiles/ui/effects/animated_string.cpp:L15-L58`, `L100-L200`
//! Upstream: `Telegram/SourceFiles/ui/effects/animated_string.h:L20-L89`
//!
//! Smooth sliding/fade animated string transition for numerical counters and status tags.
//! Diffs character tokens into Equal, New, Old regions with sliding vertical offset.

use gpui::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegionType {
    Equal,
    New,
    Old,
}

#[derive(Debug, Clone)]
pub struct StringRegion {
    pub region_type: RegionType,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct AnimatedStringEffects {
    current_text: String,
    target_text: String,
    progress: f32,
    regions: Vec<StringRegion>,
}

impl AnimatedStringEffects {
    pub fn new(initial: impl Into<String>) -> Self {
        let text = initial.into();
        let region = StringRegion {
            region_type: RegionType::Equal,
            text: text.clone(),
        };
        Self {
            current_text: text.clone(),
            target_text: text,
            progress: 1.0,
            regions: vec![region],
        }
    }

    /// C++: `animated_string.cpp:L26-L47`: Tokenize and calculate diff regions
    pub fn set_text(&mut self, new_text: impl Into<String>, animated: bool) {
        let next = new_text.into();
        if next == self.current_text {
            return;
        }
        self.target_text = next.clone();
        if !animated {
            self.current_text = next.clone();
            self.progress = 1.0;
            self.regions = vec![StringRegion {
                region_type: RegionType::Equal,
                text: next,
            }];
            return;
        }

        self.progress = 0.0;
        let mut regions = Vec::new();
        // Common prefix
        let common_len = self.current_text.chars().zip(next.chars())
            .take_while(|(a, b)| a == b)
            .count();

        if common_len > 0 {
            let prefix: String = self.current_text.chars().take(common_len).collect();
            regions.push(StringRegion {
                region_type: RegionType::Equal,
                text: prefix,
            });
        }

        let old_suffix: String = self.current_text.chars().skip(common_len).collect();
        let new_suffix: String = next.chars().skip(common_len).collect();

        if !old_suffix.is_empty() {
            regions.push(StringRegion {
                region_type: RegionType::Old,
                text: old_suffix,
            });
        }
        if !new_suffix.is_empty() {
            regions.push(StringRegion {
                region_type: RegionType::New,
                text: new_suffix,
            });
        }

        self.regions = regions;
        self.current_text = next;
    }

    pub fn set_progress(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 1.0);
    }

    pub fn regions(&self) -> &[StringRegion] {
        &self.regions
    }

    pub fn progress(&self) -> f32 {
        self.progress
    }
}

impl Default for AnimatedStringEffects {
    fn default() -> Self {
        Self::new("0")
    }
}

impl Render for AnimatedStringEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .text_size(px(14.0))
            .text_color(rgb(0xF5F5F5))
            .child(self.target_text.clone())
    }
}
