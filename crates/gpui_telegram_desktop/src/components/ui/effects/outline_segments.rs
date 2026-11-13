//! Upstream: `Telegram/SourceFiles/ui/effects/outline_segments.cpp:L12-L65`
//! Upstream: `Telegram/SourceFiles/ui/effects/outline_segments.h:L10-L38`
//!
//! Segmented avatar and story ring outline geometry. Divides a circular outline
//! into distinct colored arcs separated by gaps, supporting up to 16 story segments.

use gpui::*;

pub const OUTLINE_SEGMENTS_MAX: usize = 16;
pub const ARC_FULL_LENGTH: f32 = 360.0; // In degrees
pub const SMALL_SEPARATOR: f32 = 12.0; // In degrees

#[derive(Debug, Clone)]
pub struct StorySegment {
    pub color: u32,
    pub width: f32,
    pub is_unread: bool,
}

#[derive(Debug, Clone)]
pub struct OutlineSegmentsEffects {
    segments: Vec<StorySegment>,
    spin_progress: f32,
    diameter: f32,
}

impl OutlineSegmentsEffects {
    pub fn new(diameter: f32) -> Self {
        Self {
            segments: Vec::new(),
            spin_progress: 1.0,
            diameter: if diameter > 0.0 { diameter } else { 54.0 },
        }
    }

    pub fn set_segments(&mut self, segments: Vec<StorySegment>) {
        let count = segments.len().min(OUTLINE_SEGMENTS_MAX);
        self.segments = segments.into_iter().take(count).collect();
    }

    pub fn set_spin_progress(&mut self, progress: f32) {
        self.spin_progress = progress.clamp(0.0, 1.0);
    }

    /// C++: `outline_segments.cpp:L28-L33`:
    /// Computes `(separator_deg, arc_length_deg)`
    pub fn compute_arc_geometry(&self) -> (f32, f32) {
        let count = self.segments.len();
        if count <= 1 {
            return (0.0, ARC_FULL_LENGTH);
        }
        let count_f = count as f32;
        let separator = if ARC_FULL_LENGTH > 1.1 * SMALL_SEPARATOR * count_f {
            SMALL_SEPARATOR
        } else {
            ARC_FULL_LENGTH / (count_f * 1.1)
        };
        let left = ARC_FULL_LENGTH - (separator * count_f);
        let arc_length = left / count_f;
        (separator, arc_length)
    }

    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }
}

impl Default for OutlineSegmentsEffects {
    fn default() -> Self {
        Self::new(54.0)
    }
}

impl Render for OutlineSegmentsEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let diameter = self.diameter;
        div()
            .size(px(diameter))
            .rounded(px(diameter / 2.0))
            .border_2()
            .border_color(if self.segments.iter().any(|s| s.is_unread) {
                rgb(0x2AABEE)
            } else {
                rgb(0x50555A)
            })
    }
}
