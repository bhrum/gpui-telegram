//! Profile & Channel Boost Levels Theme Style Sheet
//! Upstream: Telegram/SourceFiles/info/profile/info_levels.style
//! Exact metrics and shape positions for channel boost level indicators (level 1 through 90).

use gpui::{px, rgb, Pixels, Point, Rgba};

#[derive(Debug, Clone)]
pub struct InfoLevelsProfileStyleSheet {
    pub base_position: Point<Pixels>,
    pub level1_position: Point<Pixels>,
    pub negative_icon_position: Point<Pixels>,
    pub negative_bubble_size: Pixels,
    pub level_icon_size: Pixels,
    pub warning_icon_size: Pixels,
    pub text_fg: Rgba,
    pub bg_active: Rgba,
    pub warning_fg: Rgba,
}

impl Default for InfoLevelsProfileStyleSheet {
    fn default() -> Self {
        Self {
            base_position: Point {
                x: px(0.0),
                y: px(6.0),
            },
            level1_position: Point {
                x: px(-1.0),
                y: px(6.0),
            },
            negative_icon_position: Point {
                x: px(6.0),
                y: px(5.0),
            },
            negative_bubble_size: px(28.0),
            level_icon_size: px(26.0),
            warning_icon_size: px(18.0),
            text_fg: rgb(0xFFFFFF),
            bg_active: rgb(0x2AABEE),
            warning_fg: rgb(0xE04F54),
        }
    }
}

impl InfoLevelsProfileStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn position_for_level(&self, level: u32) -> Point<Pixels> {
        if (1..=9).contains(&level) {
            self.level1_position
        } else {
            self.base_position
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_info_levels_style_metrics() {
        let style = InfoLevelsProfileStyleSheet::new();
        assert_eq!(style.level_icon_size, px(26.0));
        assert_eq!(style.negative_bubble_size, px(28.0));
        assert_eq!(style.position_for_level(1), Point { x: px(-1.0), y: px(6.0) });
        assert_eq!(style.position_for_level(10), Point { x: px(0.0), y: px(6.0) });
    }
}
