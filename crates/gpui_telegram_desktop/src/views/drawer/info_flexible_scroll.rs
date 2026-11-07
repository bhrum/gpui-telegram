//! Flexible Kinetic Scroll Helper
//! Upstream: Telegram/SourceFiles/info/info_flexible_scroll.h, Telegram/SourceFiles/info/info_flexible_scroll.cpp

pub const K_SCROLL_STEP_TIME_MS: u32 = 260;

#[derive(Debug, Clone)]
pub struct InfoFlexibleScroll {
    pub scroll_y: f32,
    pub target_scroll_y: f32,
    pub min_scroll_y: f32,
    pub max_scroll_y: f32,
    pub step_time_ms: u32,
    pub snap_anchors: Vec<f32>,
    pub friction: f32,
}

impl Default for InfoFlexibleScroll {
    fn default() -> Self {
        Self {
            scroll_y: 0.0,
            target_scroll_y: 0.0,
            min_scroll_y: 0.0,
            max_scroll_y: 1000.0,
            step_time_ms: K_SCROLL_STEP_TIME_MS,
            snap_anchors: Vec::new(),
            friction: 0.88,
        }
    }
}

impl InfoFlexibleScroll {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_bounds(&mut self, min: f32, max: f32) {
        self.min_scroll_y = min;
        self.max_scroll_y = max.max(min);
        self.scroll_y = self.scroll_y.clamp(self.min_scroll_y, self.max_scroll_y);
        self.target_scroll_y = self.target_scroll_y.clamp(self.min_scroll_y, self.max_scroll_y);
    }

    pub fn scroll_to(&mut self, y: f32) {
        self.target_scroll_y = y.clamp(self.min_scroll_y, self.max_scroll_y);
    }

    pub fn add_snap_anchor(&mut self, anchor: f32) {
        if !self.snap_anchors.iter().any(|&a| (a - anchor).abs() < 1.0) {
            self.snap_anchors.push(anchor);
            self.snap_anchors.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        }
    }

    pub fn find_nearest_anchor(&self, pos: f32) -> Option<f32> {
        self.snap_anchors
            .iter()
            .copied()
            .min_by(|&a, &b| (a - pos).abs().partial_cmp(&(b - pos).abs()).unwrap_or(std::cmp::Ordering::Equal))
    }

    pub fn tick(&mut self, delta_factor: f32) {
        let diff = self.target_scroll_y - self.scroll_y;
        if diff.abs() > 0.5 {
            let step = diff * (1.0 - self.friction) * delta_factor.clamp(0.1, 2.0);
            self.scroll_y += step;
        } else {
            self.scroll_y = self.target_scroll_y;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flexible_scroll_physics() {
        let mut scroll = InfoFlexibleScroll::new();
        scroll.set_bounds(0.0, 500.0);
        scroll.add_snap_anchor(0.0);
        scroll.add_snap_anchor(180.0);
        scroll.add_snap_anchor(360.0);

        assert_eq!(scroll.find_nearest_anchor(175.0), Some(180.0));
        assert_eq!(scroll.find_nearest_anchor(10.0), Some(0.0));

        scroll.scroll_to(200.0);
        assert_eq!(scroll.target_scroll_y, 200.0);
        scroll.tick(1.0);
        assert!(scroll.scroll_y > 0.0);
    }
}
