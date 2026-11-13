use gpui::rgb;

pub struct ThemePalette;

impl ThemePalette {
    pub const DAY_BLUE_PRIMARY: u32 = 0x2AABEE;
    pub const DAY_BLUE_BG: u32 = 0xFFFFFF;
    pub const DAY_BLUE_SIDEBAR: u32 = 0xF1F1F1;
    pub const DAY_BLUE_BUBBLE_OUT: u32 = 0xEEFFDE;
    pub const DAY_BLUE_BUBBLE_IN: u32 = 0xFFFFFF;

    pub const NIGHT_PRIMARY: u32 = 0x5288C1;
    pub const NIGHT_BG: u32 = 0x18222D;
    pub const NIGHT_SIDEBAR: u32 = 0x0E1621;
    pub const NIGHT_BUBBLE_OUT: u32 = 0x2B5278;
    pub const NIGHT_BUBBLE_IN: u32 = 0x182533;

    pub fn primary_color() -> gpui::Rgba {
        rgb(Self::DAY_BLUE_PRIMARY)
    }
}
