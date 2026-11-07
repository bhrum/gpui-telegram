//! Authentic GPUI Cocoon Privacy Translation Modal Box.
//! Reference: Telegram/SourceFiles/ui/boxes/about_cocoon_box.cpp:L31-L318
//! Reference: Telegram/SourceFiles/ui/boxes/about_cocoon_box.h:L1-L17

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Total orbiting star particles in the radial logo animation.
/// Reference: `about_cocoon_box.cpp:L94`
pub const K_PARTICLES_COUNT: usize = 50;

/// Number of gradient-interpolated particle color stops.
/// Reference: `about_cocoon_box.cpp:L101`
pub const K_PARTICLE_COLORS: usize = 12;

/// Background gradient edge color (0x06, 0x11, 0x29).
/// Reference: `about_cocoon_box.cpp:L35`
pub const GRADIENT_EDGE: (u8, u8, u8) = (0x06, 0x11, 0x29);

/// Background gradient center color (0x09, 0x16, 0x43).
/// Reference: `about_cocoon_box.cpp:L36`
pub const GRADIENT_CENTER: (u8, u8, u8) = (0x09, 0x16, 0x43);

/// Particle color ramp start (0x68, 0xc9, 0xff).
/// Reference: `about_cocoon_box.cpp:L38`
pub const GRADIENT_LEFT: (u8, u8, u8) = (0x68, 0xc9, 0xff);

/// Particle color ramp end (0xcb, 0x57, 0xff).
/// Reference: `about_cocoon_box.cpp:L39`
pub const GRADIENT_RIGHT: (u8, u8, u8) = (0xcb, 0x57, 0xff);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CocoonFeatureItem {
    pub icon_name: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CocoonLinkInfo {
    pub text: String,
    pub url: String,
}

/// Target symbol for Cocoon translation box.
/// Reference: `about_cocoon_box.h:L1-L17`
#[derive(Debug, Clone)]
pub struct AboutCocoonBoxBoxes {
    pub is_open: bool,
    pub particles_active: bool,
    pub speed_factor: f32,
    pub features: Vec<CocoonFeatureItem>,
    pub direct_link: CocoonLinkInfo,
    pub understood_clicked: bool,
}

impl Default for AboutCocoonBoxBoxes {
    fn default() -> Self {
        Self::new()
    }
}

impl AboutCocoonBoxBoxes {
    pub fn new() -> Self {
        Self {
            is_open: true,
            particles_active: true,
            speed_factor: 0.05,
            features: vec![
                CocoonFeatureItem {
                    icon_name: "menu_icon_lock".to_string(),
                    title: "Private".to_string(),
                    description: "End-to-end encrypted translations without central logging".to_string(),
                },
                CocoonFeatureItem {
                    icon_name: "menu_icon_stats".to_string(),
                    title: "Efficient".to_string(),
                    description: "Sub-second decentralized peer response time".to_string(),
                },
                CocoonFeatureItem {
                    icon_name: "menu_icon_gift_premium".to_string(),
                    title: "Universal".to_string(),
                    description: "Available to all users worldwide".to_string(),
                },
            ],
            direct_link: CocoonLinkInfo {
                text: "telegram.org/cocoon".to_string(),
                url: "https://telegram.org/cocoon".to_string(),
            },
            understood_clicked: false,
        }
    }

    /// Evaluates linear color interpolation across the 12 particle steps.
    /// Reference: `about_cocoon_box.cpp:L104-L108`
    pub fn interpolate_particle_color(step: usize) -> (u8, u8, u8) {
        let t = (step.min(K_PARTICLE_COLORS - 1) as f32) / ((K_PARTICLE_COLORS - 1) as f32);
        let r = (GRADIENT_LEFT.0 as f32 + t * (GRADIENT_RIGHT.0 as f32 - GRADIENT_LEFT.0 as f32)) as u8;
        let g = (GRADIENT_LEFT.1 as f32 + t * (GRADIENT_RIGHT.1 as f32 - GRADIENT_LEFT.1 as f32)) as u8;
        let b = (GRADIENT_LEFT.2 as f32 + t * (GRADIENT_RIGHT.2 as f32 - GRADIENT_LEFT.2 as f32)) as u8;
        (r, g, b)
    }

    pub fn mark_understood(&mut self) {
        self.understood_clicked = true;
        self.is_open = false;
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }
}
