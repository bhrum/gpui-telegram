//! Upstream C++ source: `Telegram/SourceFiles/ui/power_saving.cpp:L1-L56` & `power_saving.h:L1-L40`
//! Authentic GPUI implementation of battery-conscious performance throttles and power modes.
#![allow(dead_code)]

/// Power saving feature flags from `power_saving.h:L16-L24`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PowerSavingFlags {
    pub autoplay_stickers: bool,
    pub autoplay_video: bool,
    pub smooth_animations: bool,
    pub particle_effects: bool,
}

/// Power saving controller monitoring battery status and disabling expensive animations.
/// C++ companion: `Ui::PowerSaving` (`power_saving.cpp:L15-L56`).
#[derive(Debug, Clone)]
pub struct PowerSaving {
    flags: PowerSavingFlags,
    battery_level_percent: u8,
    is_on_battery: bool,
}

impl PowerSaving {
    pub fn new() -> Self {
        Self {
            flags: PowerSavingFlags {
                autoplay_stickers: true,
                autoplay_video: true,
                smooth_animations: true,
                particle_effects: true,
            },
            battery_level_percent: 100,
            is_on_battery: false,
        }
    }

    /// Automatically enables power saving when battery level drops below 20%.
    pub fn update_battery(&mut self, percent: u8, on_battery: bool) {
        self.battery_level_percent = percent;
        self.is_on_battery = on_battery;
        if on_battery && percent <= 20 {
            self.flags.autoplay_stickers = false;
            self.flags.autoplay_video = false;
            self.flags.particle_effects = false;
        }
    }

    pub fn should_play_animation(&self) -> bool {
        self.flags.smooth_animations
    }

    pub fn should_autoplay_stickers(&self) -> bool {
        self.flags.autoplay_stickers
    }
}

impl Default for PowerSaving {
    fn default() -> Self {
        Self::new()
    }
}
