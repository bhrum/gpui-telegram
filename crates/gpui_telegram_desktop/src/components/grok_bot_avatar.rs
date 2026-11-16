//! Grok Bot 0.18 dynamic character avatar engine in native Rust + GPUI.
//!
//! Ported 1:1 from `b-nnett/grok-bot-0.18-reconstructed` (`character.tsx` and `scene.ts`):
//! - ViewBox `(-15.0, -15.0, 259.0, 259.0)` with canonical center `114.2705`
//! - 8 normalized procedural/Bezier body silhouettes cached in `OnceLock`
//! - 11-color Grok palette with automatic dark-surface contrast flip for `"black"`
//! - 39 persona activity/emotion motion states, staggered blinking, cursor gaze,
//!   and 1000ms spin / 700ms bounce interaction impulses.

use std::cell::RefCell;
use std::collections::HashMap;
use std::f32::consts::{PI, TAU};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use gpui::*;
use gpui_telegram_proto::miniapp_contracts::{
    grok_shipped_hash, resolve_grok_persona_color, resolve_grok_persona_shape,
    CLASSIC_INITIALS_AVATAR_SHAPE, GROK_AVATAR_SHAPES,
};
use parking_lot::Mutex;

/// ViewBox origin (X and Y) matching `(-15, -15, 259, 259)`.
pub const GROK_VIEWBOX_MIN: f32 = -15.0;
/// ViewBox extent (width and height) matching `(-15, -15, 259, 259)`.
pub const GROK_VIEWBOX_SIZE: f32 = 259.0;
/// Canonical Grok Bot character center (`228.541 / 2.0`).
pub const GROK_AVATAR_CENTER: f32 = 114.2705;

const NORMALIZED_TARGET_EXTENT: f32 = 228.44;
const SPIN_DURATION_MS: f32 = 1000.0;
const BOUNCE_DURATION_MS: f32 = 700.0;
const BLINK_DURATION_MS: f64 = 140.0;
const ANIM_DEDUP_MS: u64 = 8;

/// 39 character animation and expression states from Grok Bot 0.18.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GrokPersonaState {
    Sleeping,
    Waking,
    #[default]
    Idle,
    Listening,
    Thinking,
    Searching,
    Working,
    Loading,
    Excited,
    Surprised,
    Suspicious,
    Angry,
    Drowsy,
    Happy,
    Curious,
    Confused,
    Bored,
    Proud,
    Shy,
    Sad,
    Laughing,
    Scared,
    Playful,
    Celebrate,
    Orbit,
    Radar,
    Progress,
    Spawning,
    Humming,
    Dictating,
    Writing,
    Sending,
    Receiving,
    Uploading,
    Notifying,
    Alerting,
    Dragging,
    Bouncing,
    PoweringDown,
}

/// Physical motion parameters for a [`GrokPersonaState`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GrokMotionSpec {
    pub amplitude: f32,
    pub period_ms: f32,
    pub tilt_deg: f32,
    pub eye_scale: f32,
}

impl GrokPersonaState {
    /// Complete array of all 39 [`GrokPersonaState`] variants.
    pub const ALL: [GrokPersonaState; 39] = [
        Self::Sleeping,
        Self::Waking,
        Self::Idle,
        Self::Listening,
        Self::Thinking,
        Self::Searching,
        Self::Working,
        Self::Loading,
        Self::Excited,
        Self::Surprised,
        Self::Suspicious,
        Self::Angry,
        Self::Drowsy,
        Self::Happy,
        Self::Curious,
        Self::Confused,
        Self::Bored,
        Self::Proud,
        Self::Shy,
        Self::Sad,
        Self::Laughing,
        Self::Scared,
        Self::Playful,
        Self::Celebrate,
        Self::Orbit,
        Self::Radar,
        Self::Progress,
        Self::Spawning,
        Self::Humming,
        Self::Dictating,
        Self::Writing,
        Self::Sending,
        Self::Receiving,
        Self::Uploading,
        Self::Notifying,
        Self::Alerting,
        Self::Dragging,
        Self::Bouncing,
        Self::PoweringDown,
    ];

    /// Maps an agent `execution_state` string and `is_typing` flag to a [`GrokPersonaState`].
    pub fn from_activity_str(execution_state: &str, is_typing: bool) -> Self {
        let key: String = execution_state
            .trim()
            .chars()
            .filter(|c| !matches!(c, '-' | '_' | ' '))
            .flat_map(|c| c.to_lowercase())
            .collect();

        match key.as_str() {
            "sleeping" | "sleep" | "offline" | "paused" => Self::Sleeping,
            "waking" | "wake" => Self::Waking,
            "idle" | "ready" | "standby" | "" => {
                if is_typing {
                    Self::Thinking
                } else {
                    Self::Idle
                }
            }
            "listening" | "listen" => Self::Listening,
            "thinking" | "think" | "reasoning" => Self::Thinking,
            "searching" | "search" | "browsing" => Self::Searching,
            "working" | "running" | "executing" | "busy" | "active" => {
                if is_typing {
                    Self::Writing
                } else {
                    Self::Working
                }
            }
            "loading" => Self::Loading,
            "excited" => Self::Excited,
            "surprised" => Self::Surprised,
            "suspicious" => Self::Suspicious,
            "angry" | "error" | "failed" => Self::Angry,
            "drowsy" => Self::Drowsy,
            "happy" => Self::Happy,
            "curious" => Self::Curious,
            "confused" => Self::Confused,
            "bored" => Self::Bored,
            "proud" => Self::Proud,
            "shy" => Self::Shy,
            "sad" => Self::Sad,
            "laughing" => Self::Laughing,
            "scared" => Self::Scared,
            "playful" => Self::Playful,
            "celebrate" | "done" | "completed" | "success" => Self::Celebrate,
            "orbit" => Self::Orbit,
            "radar" => Self::Radar,
            "progress" => Self::Progress,
            "spawning" | "starting" => Self::Spawning,
            "humming" => Self::Humming,
            "dictating" => Self::Dictating,
            "writing" | "typing" | "streaming" | "composing" => Self::Writing,
            "sending" => Self::Sending,
            "receiving" => Self::Receiving,
            "uploading" => Self::Uploading,
            "notifying" => Self::Notifying,
            "alerting" => Self::Alerting,
            "dragging" => Self::Dragging,
            "bouncing" => Self::Bouncing,
            "poweringdown" | "stopping" => Self::PoweringDown,
            _ => {
                if is_typing {
                    Self::Thinking
                } else {
                    Self::Idle
                }
            }
        }
    }

    /// Returns the 39-entry physical bob, tilt, and eye scale spec from `character.tsx`.
    pub fn motion(self) -> GrokMotionSpec {
        match self {
            Self::Sleeping => GrokMotionSpec {
                amplitude: 0.0,
                period_ms: 6000.0,
                tilt_deg: 0.0,
                eye_scale: 0.12,
            },
            Self::Waking => GrokMotionSpec {
                amplitude: 2.0,
                period_ms: 3200.0,
                tilt_deg: -2.0,
                eye_scale: 0.65,
            },
            Self::Idle => GrokMotionSpec {
                amplitude: 1.5,
                period_ms: 9000.0,
                tilt_deg: 0.0,
                eye_scale: 1.0,
            },
            Self::Listening => GrokMotionSpec {
                amplitude: 2.2,
                period_ms: 2800.0,
                tilt_deg: -4.0,
                eye_scale: 1.06,
            },
            Self::Thinking => GrokMotionSpec {
                amplitude: 1.0,
                period_ms: 2000.0,
                tilt_deg: 3.0,
                eye_scale: 0.75,
            },
            Self::Searching => GrokMotionSpec {
                amplitude: 2.5,
                period_ms: 2200.0,
                tilt_deg: 4.0,
                eye_scale: 0.92,
            },
            Self::Working => GrokMotionSpec {
                amplitude: 2.0,
                period_ms: 1800.0,
                tilt_deg: -3.0,
                eye_scale: 1.0,
            },
            Self::Loading => GrokMotionSpec {
                amplitude: 2.0,
                period_ms: 1600.0,
                tilt_deg: 0.0,
                eye_scale: 0.88,
            },
            Self::Excited => GrokMotionSpec {
                amplitude: 5.5,
                period_ms: 1500.0,
                tilt_deg: 0.0,
                eye_scale: 1.15,
            },
            Self::Surprised => GrokMotionSpec {
                amplitude: 4.0,
                period_ms: 1800.0,
                tilt_deg: -2.0,
                eye_scale: 1.25,
            },
            Self::Suspicious => GrokMotionSpec {
                amplitude: 0.8,
                period_ms: 4200.0,
                tilt_deg: 4.5,
                eye_scale: 0.55,
            },
            Self::Angry => GrokMotionSpec {
                amplitude: 1.8,
                period_ms: 1400.0,
                tilt_deg: -4.0,
                eye_scale: 0.62,
            },
            Self::Drowsy => GrokMotionSpec {
                amplitude: 0.6,
                period_ms: 6500.0,
                tilt_deg: 2.5,
                eye_scale: 0.35,
            },
            Self::Happy => GrokMotionSpec {
                amplitude: 3.0,
                period_ms: 2500.0,
                tilt_deg: 0.0,
                eye_scale: 1.08,
            },
            Self::Curious => GrokMotionSpec {
                amplitude: 2.4,
                period_ms: 2600.0,
                tilt_deg: -6.0,
                eye_scale: 1.10,
            },
            Self::Confused => GrokMotionSpec {
                amplitude: 1.2,
                period_ms: 3000.0,
                tilt_deg: 6.5,
                eye_scale: 0.85,
            },
            Self::Bored => GrokMotionSpec {
                amplitude: 0.8,
                period_ms: 7000.0,
                tilt_deg: 1.5,
                eye_scale: 0.58,
            },
            Self::Proud => GrokMotionSpec {
                amplitude: 2.6,
                period_ms: 2800.0,
                tilt_deg: -2.0,
                eye_scale: 0.95,
            },
            Self::Shy => GrokMotionSpec {
                amplitude: 1.2,
                period_ms: 3800.0,
                tilt_deg: 3.5,
                eye_scale: 0.82,
            },
            Self::Sad => GrokMotionSpec {
                amplitude: 0.7,
                period_ms: 5500.0,
                tilt_deg: -3.5,
                eye_scale: 0.70,
            },
            Self::Laughing => GrokMotionSpec {
                amplitude: 6.0,
                period_ms: 1100.0,
                tilt_deg: 0.0,
                eye_scale: 0.45,
            },
            Self::Scared => GrokMotionSpec {
                amplitude: 3.5,
                period_ms: 950.0,
                tilt_deg: 2.0,
                eye_scale: 1.28,
            },
            Self::Playful => GrokMotionSpec {
                amplitude: 4.2,
                period_ms: 1900.0,
                tilt_deg: -4.5,
                eye_scale: 1.08,
            },
            Self::Celebrate => GrokMotionSpec {
                amplitude: 7.0,
                period_ms: 1400.0,
                tilt_deg: 0.0,
                eye_scale: 1.12,
            },
            Self::Orbit => GrokMotionSpec {
                amplitude: 3.0,
                period_ms: 2400.0,
                tilt_deg: 5.0,
                eye_scale: 1.0,
            },
            Self::Radar => GrokMotionSpec {
                amplitude: 2.0,
                period_ms: 2200.0,
                tilt_deg: -5.0,
                eye_scale: 0.95,
            },
            Self::Progress => GrokMotionSpec {
                amplitude: 2.2,
                period_ms: 2000.0,
                tilt_deg: -2.0,
                eye_scale: 0.96,
            },
            Self::Spawning => GrokMotionSpec {
                amplitude: 4.0,
                period_ms: 1600.0,
                tilt_deg: 0.0,
                eye_scale: 1.05,
            },
            Self::Humming => GrokMotionSpec {
                amplitude: 2.5,
                period_ms: 3000.0,
                tilt_deg: 2.0,
                eye_scale: 0.78,
            },
            Self::Dictating => GrokMotionSpec {
                amplitude: 2.8,
                period_ms: 2100.0,
                tilt_deg: -2.5,
                eye_scale: 1.02,
            },
            Self::Writing => GrokMotionSpec {
                amplitude: 2.2,
                period_ms: 1700.0,
                tilt_deg: -3.5,
                eye_scale: 0.94,
            },
            Self::Sending => GrokMotionSpec {
                amplitude: 4.5,
                period_ms: 1500.0,
                tilt_deg: -5.0,
                eye_scale: 1.06,
            },
            Self::Receiving => GrokMotionSpec {
                amplitude: 3.5,
                period_ms: 1700.0,
                tilt_deg: 3.5,
                eye_scale: 1.06,
            },
            Self::Uploading => GrokMotionSpec {
                amplitude: 4.2,
                period_ms: 1600.0,
                tilt_deg: -2.0,
                eye_scale: 1.04,
            },
            Self::Notifying => GrokMotionSpec {
                amplitude: 5.0,
                period_ms: 1400.0,
                tilt_deg: 3.0,
                eye_scale: 1.14,
            },
            Self::Alerting => GrokMotionSpec {
                amplitude: 6.0,
                period_ms: 1100.0,
                tilt_deg: -4.0,
                eye_scale: 1.20,
            },
            Self::Dragging => GrokMotionSpec {
                amplitude: 1.0,
                period_ms: 2400.0,
                tilt_deg: 5.0,
                eye_scale: 1.08,
            },
            Self::Bouncing => GrokMotionSpec {
                amplitude: 7.5,
                period_ms: 1200.0,
                tilt_deg: 0.0,
                eye_scale: 1.10,
            },
            Self::PoweringDown => GrokMotionSpec {
                amplitude: 0.4,
                period_ms: 5200.0,
                tilt_deg: 0.0,
                eye_scale: 0.25,
            },
        }
    }
}

/// Palette swatch and 135-degree body gradient stop definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GrokColorSpec {
    pub id: &'static str,
    pub label_en: &'static str,
    pub label_zh: &'static str,
    pub swatch_rgb: u32,
    pub grad_light_rgb: u32,
    pub grad_dark_rgb: u32,
}

/// Canonical 11-color Grok Bot 0.18 palette.
pub const GROK_COLOR_PALETTE: [GrokColorSpec; 11] = [
    GrokColorSpec {
        id: "black",
        label_en: "Black",
        label_zh: "曜石黑",
        swatch_rgb: 0x111114,
        grad_light_rgb: 0x323238,
        grad_dark_rgb: 0x050507,
    },
    GrokColorSpec {
        id: "brown",
        label_en: "Brown",
        label_zh: "琥珀棕",
        swatch_rgb: 0x936439,
        grad_light_rgb: 0xA27952,
        grad_dark_rgb: 0x855C36,
    },
    GrokColorSpec {
        id: "red",
        label_en: "Red",
        label_zh: "炽焰红",
        swatch_rgb: 0xFF263C,
        grad_light_rgb: 0xFF3E51,
        grad_dark_rgb: 0xE02135,
    },
    GrokColorSpec {
        id: "orange",
        label_en: "Orange",
        label_zh: "活力橙",
        swatch_rgb: 0xFF6700,
        grad_light_rgb: 0xFF781C,
        grad_dark_rgb: 0xFF6700,
    },
    GrokColorSpec {
        id: "yellow",
        label_en: "Yellow",
        label_zh: "暖阳金",
        swatch_rgb: 0xFF9800,
        grad_light_rgb: 0xFFAF38,
        grad_dark_rgb: 0xFF9800,
    },
    GrokColorSpec {
        id: "green",
        label_en: "Green",
        label_zh: "翡翠绿",
        swatch_rgb: 0x00C972,
        grad_light_rgb: 0x00C972,
        grad_dark_rgb: 0x009957,
    },
    GrokColorSpec {
        id: "cyan",
        label_en: "Cyan",
        label_zh: "青碧蓝",
        swatch_rgb: 0x00BCA6,
        grad_light_rgb: 0x1CC3B0,
        grad_dark_rgb: 0x00A592,
    },
    GrokColorSpec {
        id: "blue",
        label_en: "Blue",
        label_zh: "星辰蓝",
        swatch_rgb: 0x1084FE,
        grad_light_rgb: 0x2A92FE,
        grad_dark_rgb: 0x0E74E0,
    },
    GrokColorSpec {
        id: "violet",
        label_en: "Violet",
        label_zh: "极光紫",
        swatch_rgb: 0x9159FE,
        grad_light_rgb: 0xA97EFE,
        grad_dark_rgb: 0x804EE0,
    },
    GrokColorSpec {
        id: "magenta",
        label_en: "Magenta",
        label_zh: "品红",
        swatch_rgb: 0xFF309B,
        grad_light_rgb: 0xFF5EB1,
        grad_dark_rgb: 0xE02A88,
    },
    GrokColorSpec {
        id: "gray",
        label_en: "Gray",
        label_zh: "星际灰",
        swatch_rgb: 0x777777,
        grad_light_rgb: 0x959595,
        grad_dark_rgb: 0x777777,
    },
];

/// Rendering configuration for [`render_grok_bot_avatar`].
#[derive(Debug, Clone)]
pub struct GrokBotAvatarProps {
    pub source_id: String,
    pub shape: String,
    pub color: String,
    pub size_px: f32,
    pub state: GrokPersonaState,
    pub paused: bool,
    pub follow_pointer: bool,
    pub emphasis: bool,
    pub surface_bg_rgb: u32,
}

impl GrokBotAvatarProps {
    pub fn new(
        source_id: impl AsRef<str>,
        shape: impl AsRef<str>,
        color: impl AsRef<str>,
        size_px: f32,
    ) -> Self {
        Self {
            source_id: source_id.as_ref().to_string(),
            shape: shape.as_ref().to_string(),
            color: color.as_ref().to_string(),
            size_px,
            state: GrokPersonaState::Idle,
            paused: false,
            follow_pointer: false,
            emphasis: false,
            surface_bg_rgb: 0xFFFFFF,
        }
    }

    pub fn with_state(mut self, state: GrokPersonaState) -> Self {
        self.state = state;
        self
    }

    pub fn with_paused(mut self, paused: bool) -> Self {
        self.paused = paused;
        self
    }

    pub fn with_follow_pointer(mut self, follow_pointer: bool) -> Self {
        self.follow_pointer = follow_pointer;
        self
    }

    pub fn with_emphasis(mut self, emphasis: bool) -> Self {
        self.emphasis = emphasis;
        self
    }

    pub fn with_surface_bg(mut self, surface_bg_rgb: u32) -> Self {
        self.surface_bg_rgb = surface_bg_rgb;
        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct AvatarImpulse {
    spin_started_at: Option<Instant>,
    bounce_started_at: Option<Instant>,
}

fn impulse_registry() -> &'static Mutex<HashMap<String, AvatarImpulse>> {
    static REGISTRY: OnceLock<Mutex<HashMap<String, AvatarImpulse>>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

fn engine_epoch() -> Instant {
    static EPOCH: OnceLock<Instant> = OnceLock::new();
    *EPOCH.get_or_init(Instant::now)
}

fn prune_expired_impulses(map: &mut HashMap<String, AvatarImpulse>, now: Instant) {
    let max_age = Duration::from_secs(2);
    map.retain(|_, imp| {
        let spin_alive = imp
            .spin_started_at
            .is_some_and(|t| now.saturating_duration_since(t) <= max_age);
        let bounce_alive = imp
            .bounce_started_at
            .is_some_and(|t| now.saturating_duration_since(t) <= max_age);
        spin_alive || bounce_alive
    });
}

/// Triggers a 1000ms 360-degree spin impulse on any live avatar sharing `source_id`.
pub fn trigger_grok_avatar_spin(source_id: &str) {
    let now = Instant::now();
    let mut map = impulse_registry().lock();
    prune_expired_impulses(&mut map, now);
    map.entry(source_id.to_string()).or_default().spin_started_at = Some(now);
}

/// Triggers a 700ms `+8.0` ViewBox vertical bounce impulse on any live avatar sharing `source_id`.
pub fn trigger_grok_avatar_bounce(source_id: &str) {
    let now = Instant::now();
    let mut map = impulse_registry().lock();
    prune_expired_impulses(&mut map, now);
    map.entry(source_id.to_string())
        .or_default()
        .bounce_started_at = Some(now);
}

fn sample_impulses(source_id: &str, now: Instant) -> (f32, f32, bool) {
    let guard = impulse_registry().lock();
    let Some(imp) = guard.get(source_id) else {
        return (0.0, 0.0, false);
    };

    let mut spin_deg = 0.0;
    let mut bounce_offset = 0.0;
    let mut active = false;

    if let Some(started) = imp.spin_started_at {
        let ms = now.saturating_duration_since(started).as_secs_f32() * 1000.0;
        if ms < SPIN_DURATION_MS {
            let t = (ms / SPIN_DURATION_MS).clamp(0.0, 1.0);
            let eased = if t < 0.5 {
                4.0 * t * t * t
            } else {
                1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
            };
            spin_deg = eased * 360.0;
            active = true;
        }
    }

    if let Some(started) = imp.bounce_started_at {
        let ms = now.saturating_duration_since(started).as_secs_f32() * 1000.0;
        if ms < BOUNCE_DURATION_MS {
            let t = (ms / BOUNCE_DURATION_MS).clamp(0.0, 1.0);
            bounce_offset = (t * PI).sin() * (1.0 - 0.12 * t) * 8.0;
            active = true;
        }
    }

    (spin_deg, bounce_offset, active)
}

#[derive(Debug, Clone, Copy)]
enum PathCmd {
    MoveTo(f32, f32),
    LineTo(f32, f32),
    CubicTo {
        c1: (f32, f32),
        c2: (f32, f32),
        to: (f32, f32),
    },
    Close,
}

#[inline]
fn eval_cubic_1d(p0: f64, p1: f64, p2: f64, p3: f64, t: f64) -> f64 {
    let mt = 1.0 - t;
    mt * mt * mt * p0 + 3.0 * mt * mt * t * p1 + 3.0 * mt * t * t * p2 + t * t * t * p3
}

fn extend_bbox_cubic_1d(p0: f32, p1: f32, p2: f32, p3: f32, min_v: &mut f32, max_v: &mut f32) {
    *min_v = (*min_v).min(p0).min(p3);
    *max_v = (*max_v).max(p0).max(p3);

    let (p0, p1, p2, p3) = (f64::from(p0), f64::from(p1), f64::from(p2), f64::from(p3));
    let scale = p0.abs().max(p1.abs()).max(p2.abs()).max(p3.abs()).max(1.0);
    let a = -p0 + 3.0 * p1 - 3.0 * p2 + p3;
    let b = 2.0 * (p0 - 2.0 * p1 + p2);
    let c = p1 - p0;

    let mut check_t = |t: f64| {
        if t > 0.0 && t < 1.0 {
            let val = eval_cubic_1d(p0, p1, p2, p3, t) as f32;
            *min_v = (*min_v).min(val);
            *max_v = (*max_v).max(val);
        }
    };

    if a.abs() <= 1e-5 * scale {
        if b.abs() > 1e-12 * scale {
            check_t(-c / b);
        }
    } else {
        let disc = b * b - 4.0 * a * c;
        if disc >= 0.0 {
            let root = disc.sqrt();
            if b.abs() > 1e-12 * scale {
                let q = -0.5 * (b + b.signum() * root);
                check_t(q / a);
                check_t(c / q);
            } else {
                check_t((-b + root) / (2.0 * a));
                check_t((-b - root) / (2.0 * a));
            }
        }
    }
}

fn compute_bbox(cmds: &[PathCmd]) -> (f32, f32, f32, f32) {
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    let mut cur = (0.0_f32, 0.0_f32);
    let mut start = (0.0_f32, 0.0_f32);

    for &cmd in cmds {
        match cmd {
            PathCmd::MoveTo(x, y) => {
                min_x = min_x.min(x);
                max_x = max_x.max(x);
                min_y = min_y.min(y);
                max_y = max_y.max(y);
                cur = (x, y);
                start = (x, y);
            }
            PathCmd::LineTo(x, y) => {
                min_x = min_x.min(x);
                max_x = max_x.max(x);
                min_y = min_y.min(y);
                max_y = max_y.max(y);
                cur = (x, y);
            }
            PathCmd::CubicTo { c1, c2, to } => {
                extend_bbox_cubic_1d(cur.0, c1.0, c2.0, to.0, &mut min_x, &mut max_x);
                extend_bbox_cubic_1d(cur.1, c1.1, c2.1, to.1, &mut min_y, &mut max_y);
                cur = to;
            }
            PathCmd::Close => {
                cur = start;
            }
        }
    }

    if !min_x.is_finite() {
        (0.0, 0.0, 0.0, 0.0)
    } else {
        (min_x, min_y, max_x, max_y)
    }
}

fn normalize_artifact_commands(cmds: Vec<PathCmd>) -> Vec<PathCmd> {
    let (min_x, min_y, max_x, max_y) = compute_bbox(&cmds);
    let w = (max_x - min_x).max(0.0);
    let h = (max_y - min_y).max(0.0);
    let max_dim = w.max(h);
    let cx = (min_x + max_x) * 0.5;
    let cy = (min_y + max_y) * 0.5;
    let scale = if max_dim > 1e-4 {
        (NORMALIZED_TARGET_EXTENT / max_dim).clamp(0.9, 1.35)
    } else {
        1.0
    };

    let map_pt = |(x, y): (f32, f32)| -> (f32, f32) {
        (
            GROK_AVATAR_CENTER + (x - cx) * scale,
            GROK_AVATAR_CENTER + (y - cy) * scale,
        )
    };

    cmds.into_iter()
        .map(|cmd| match cmd {
            PathCmd::MoveTo(x, y) => {
                let (nx, ny) = map_pt((x, y));
                PathCmd::MoveTo(nx, ny)
            }
            PathCmd::LineTo(x, y) => {
                let (nx, ny) = map_pt((x, y));
                PathCmd::LineTo(nx, ny)
            }
            PathCmd::CubicTo { c1, c2, to } => PathCmd::CubicTo {
                c1: map_pt(c1),
                c2: map_pt(c2),
                to: map_pt(to),
            },
            PathCmd::Close => PathCmd::Close,
        })
        .collect()
}

fn push_arc_cubics(
    cmds: &mut Vec<PathCmd>,
    cx: f32,
    cy: f32,
    r: f32,
    start_angle: f32,
    sweep_angle: f32,
) {
    let segments = (sweep_angle.abs() / (PI * 0.5)).ceil().max(1.0) as usize;
    let step = sweep_angle / (segments as f32);
    for i in 0..segments {
        let a0 = start_angle + step * (i as f32);
        let a1 = a0 + step;
        let k = (4.0 / 3.0) * (step * 0.25).tan();
        let (s0, c0) = a0.sin_cos();
        let (s1, c1) = a1.sin_cos();
        let p0 = (cx + r * c0, cy + r * s0);
        let p3 = (cx + r * c1, cy + r * s1);
        let cp1 = (p0.0 - k * r * s0, p0.1 + k * r * c0);
        let cp2 = (p3.0 + k * r * s1, p3.1 - k * r * c1);
        cmds.push(PathCmd::CubicTo {
            c1: cp1,
            c2: cp2,
            to: p3,
        });
    }
}

fn build_blob_commands() -> Vec<PathCmd> {
    // 8 C1-smooth cubic Bezier segments forming Grok Bot's organic blob silhouette.
    struct Anchor {
        p: (f32, f32),
        t: (f32, f32),
        h_in: f32,
        h_out: f32,
    }
    let anchors = [
        Anchor {
            p: (118.0, 4.0),
            t: (0.996, 0.087),
            h_in: 38.0,
            h_out: 36.0,
        },
        Anchor {
            p: (196.0, 46.0),
            t: (0.616, 0.788),
            h_in: 34.0,
            h_out: 34.0,
        },
        Anchor {
            p: (224.0, 134.0),
            t: (-0.139, 0.990),
            h_in: 38.0,
            h_out: 36.0,
        },
        Anchor {
            p: (182.0, 208.0),
            t: (-0.799, 0.602),
            h_in: 34.0,
            h_out: 34.0,
        },
        Anchor {
            p: (108.0, 224.5),
            t: (-0.999, -0.052),
            h_in: 36.0,
            h_out: 38.0,
        },
        Anchor {
            p: (32.0, 194.0),
            t: (-0.643, -0.766),
            h_in: 34.0,
            h_out: 34.0,
        },
        Anchor {
            p: (3.5, 108.0),
            t: (0.105, -0.995),
            h_in: 38.0,
            h_out: 38.0,
        },
        Anchor {
            p: (42.0, 26.0),
            t: (0.766, -0.643),
            h_in: 34.0,
            h_out: 36.0,
        },
    ];

    let n = anchors.len();
    let mut cmds = Vec::with_capacity(n + 2);
    cmds.push(PathCmd::MoveTo(anchors[0].p.0, anchors[0].p.1));
    for i in 0..n {
        let a = &anchors[i];
        let b = &anchors[(i + 1) % n];
        let c1 = (a.p.0 + a.t.0 * a.h_out, a.p.1 + a.t.1 * a.h_out);
        let c2 = (b.p.0 - b.t.0 * b.h_in, b.p.1 - b.t.1 * b.h_in);
        cmds.push(PathCmd::CubicTo { c1, c2, to: b.p });
    }
    cmds.push(PathCmd::Close);
    cmds
}

fn build_pebble_commands() -> Vec<PathCmd> {
    // 128-point closed uniform Catmull-Rom river pebble silhouette.
    let c = GROK_AVATAR_CENTER;
    let ctrl = [
        (c - 6.0, c - 92.0),
        (c + 76.0, c - 66.0),
        (c + 114.0, c + 4.0),
        (c + 84.0, c + 72.0),
        (c + 2.0, c + 94.0),
        (c - 82.0, c + 74.0),
        (c - 112.0, c + 8.0),
        (c - 78.0, c - 64.0),
    ];
    let n = ctrl.len();
    let steps = 128_usize;
    let mut cmds = Vec::with_capacity(steps + 2);

    for step in 0..steps {
        let u = (step as f32 / steps as f32) * (n as f32);
        let seg = (u.floor() as usize) % n;
        let t = u - u.floor();
        let t2 = t * t;
        let t3 = t2 * t;

        let p0 = ctrl[(seg + n - 1) % n];
        let p1 = ctrl[seg];
        let p2 = ctrl[(seg + 1) % n];
        let p3 = ctrl[(seg + 2) % n];

        let x = 0.5
            * ((2.0 * p1.0)
                + (-p0.0 + p2.0) * t
                + (2.0 * p0.0 - 5.0 * p1.0 + 4.0 * p2.0 - p3.0) * t2
                + (-p0.0 + 3.0 * p1.0 - 3.0 * p2.0 + p3.0) * t3);
        let y = 0.5
            * ((2.0 * p1.1)
                + (-p0.1 + p2.1) * t
                + (2.0 * p0.1 - 5.0 * p1.1 + 4.0 * p2.1 - p3.1) * t2
                + (-p0.1 + 3.0 * p1.1 - 3.0 * p2.1 + p3.1) * t3);

        if step == 0 {
            cmds.push(PathCmd::MoveTo(x, y));
        } else {
            cmds.push(PathCmd::LineTo(x, y));
        }
    }
    cmds.push(PathCmd::Close);
    cmds
}

fn build_squircle_commands() -> Vec<PathCmd> {
    // 128-point superellipse with exponent 4.2.
    let c = GROK_AVATAR_CENTER;
    let r = 114.0_f32;
    let power = 2.0_f32 / 4.2_f32;
    let steps = 128_usize;
    let mut cmds = Vec::with_capacity(steps + 2);

    for step in 0..steps {
        let theta = TAU * (step as f32) / (steps as f32);
        let (sin_t, cos_t) = theta.sin_cos();
        let x = c + r * cos_t.signum() * cos_t.abs().powf(power);
        let y = c + r * sin_t.signum() * sin_t.abs().powf(power);
        if step == 0 {
            cmds.push(PathCmd::MoveTo(x, y));
        } else {
            cmds.push(PathCmd::LineTo(x, y));
        }
    }
    cmds.push(PathCmd::Close);
    cmds
}

fn build_tablet_commands() -> Vec<PathCmd> {
    // Horizontal capsule with half-width 114 and half-height 74.
    let c = GROK_AVATAR_CENTER;
    let rx = 114.0_f32;
    let ry = 74.0_f32;
    let flat_half = rx - ry;

    let mut cmds = Vec::with_capacity(10);
    cmds.push(PathCmd::MoveTo(c - flat_half, c - ry));
    cmds.push(PathCmd::LineTo(c + flat_half, c - ry));
    push_arc_cubics(&mut cmds, c + flat_half, c, ry, -PI * 0.5, PI);
    cmds.push(PathCmd::LineTo(c - flat_half, c + ry));
    push_arc_cubics(&mut cmds, c - flat_half, c, ry, PI * 0.5, PI);
    cmds.push(PathCmd::Close);
    cmds
}

fn build_rounded_polygon_commands(
    cx: f32,
    cy: f32,
    sides: usize,
    r: f32,
    cr: f32,
    start_angle: f32,
) -> Vec<PathCmd> {
    let mut verts = Vec::with_capacity(sides);
    for i in 0..sides {
        let a = start_angle + TAU * (i as f32) / (sides as f32);
        let (s, c) = a.sin_cos();
        verts.push((cx + r * c, cy + r * s));
    }

    let mut cmds = Vec::with_capacity(sides * 2 + 2);
    for i in 0..sides {
        let prev = verts[(i + sides - 1) % sides];
        let curr = verts[i];
        let next = verts[(i + 1) % sides];

        let vin = (prev.0 - curr.0, prev.1 - curr.1);
        let vout = (next.0 - curr.0, next.1 - curr.1);
        let lin = (vin.0 * vin.0 + vin.1 * vin.1).sqrt().max(1e-4);
        let lout = (vout.0 * vout.0 + vout.1 * vout.1).sqrt().max(1e-4);
        let d_in = cr.min(lin * 0.48);
        let d_out = cr.min(lout * 0.48);

        let p_in = (curr.0 + (vin.0 / lin) * d_in, curr.1 + (vin.1 / lin) * d_in);
        let p_out = (
            curr.0 + (vout.0 / lout) * d_out,
            curr.1 + (vout.1 / lout) * d_out,
        );

        // Convert quadratic Bezier Q(curr, p_out) to exact cubic Bezier.
        let c1 = (
            p_in.0 + (2.0 / 3.0) * (curr.0 - p_in.0),
            p_in.1 + (2.0 / 3.0) * (curr.1 - p_in.1),
        );
        let c2 = (
            p_out.0 + (2.0 / 3.0) * (curr.0 - p_out.0),
            p_out.1 + (2.0 / 3.0) * (curr.1 - p_out.1),
        );

        if i == 0 {
            cmds.push(PathCmd::MoveTo(p_in.0, p_in.1));
        } else {
            cmds.push(PathCmd::LineTo(p_in.0, p_in.1));
        }
        cmds.push(PathCmd::CubicTo { c1, c2, to: p_out });
    }
    cmds.push(PathCmd::Close);
    cmds
}

fn build_cloud_commands() -> Vec<PathCmd> {
    // 160-point outer boundary of a 5-circle union centered around GROK_AVATAR_CENTER.
    let c = GROK_AVATAR_CENTER;
    let circles = [
        (c, c + 22.0, 78.0_f32),
        (c - 62.0, c + 16.0, 52.0_f32),
        (c + 62.0, c + 16.0, 52.0_f32),
        (c - 28.0, c - 34.0, 62.0_f32),
        (c + 32.0, c - 26.0, 56.0_f32),
    ];

    let steps = 160_usize;
    let mut cmds = Vec::with_capacity(steps + 2);
    for step in 0..steps {
        let theta = TAU * (step as f32) / (steps as f32);
        let (sin_t, cos_t) = theta.sin_cos();

        let mut best_t = 0.0_f32;
        for &(cx, cy, r) in &circles {
            let ox = c - cx;
            let oy = c - cy;
            let b = ox * cos_t + oy * sin_t;
            let q = ox * ox + oy * oy - r * r;
            let disc = b * b - q;
            if disc >= 0.0 {
                let t = -b + disc.sqrt();
                if t > best_t {
                    best_t = t;
                }
            }
        }

        let x = c + best_t * cos_t;
        let y = c + best_t * sin_t;
        if step == 0 {
            cmds.push(PathCmd::MoveTo(x, y));
        } else {
            cmds.push(PathCmd::LineTo(x, y));
        }
    }
    cmds.push(PathCmd::Close);
    cmds
}

fn build_teardrop_commands() -> Vec<PathCmd> {
    // w=88, top=CENTER-114, bottom=CENTER+26, cr=18 connected via C1 outer tangents.
    let c = GROK_AVATAR_CENTER;
    let w = 88.0_f32;
    let top = c - 114.0;
    let bottom = c + 26.0;
    let cr = 18.0_f32;

    let cy1 = top + cr;
    let cy2 = bottom;
    let dy = cy2 - cy1;
    let dr = w - cr;
    let ny = -(dr / dy).clamp(-0.999, 0.999);
    let nx = (1.0 - ny * ny).sqrt();

    let p1_right = (c + cr * nx, cy1 + cr * ny);
    let p2_right = (c + w * nx, cy2 + w * ny);

    let theta_r = ny.atan2(nx);
    let theta_l = PI - theta_r;
    let sweep_bottom = theta_l - theta_r;
    let top_end = TAU + theta_r;
    let sweep_top = top_end - theta_l;

    let p1_left = (c - cr * nx, cy1 + cr * ny);

    let mut cmds = Vec::with_capacity(16);
    cmds.push(PathCmd::MoveTo(p1_right.0, p1_right.1));
    cmds.push(PathCmd::LineTo(p2_right.0, p2_right.1));
    push_arc_cubics(&mut cmds, c, cy2, w, theta_r, sweep_bottom);
    cmds.push(PathCmd::LineTo(p1_left.0, p1_left.1));
    push_arc_cubics(&mut cmds, c, cy1, cr, theta_l, sweep_top);
    cmds.push(PathCmd::Close);
    cmds
}

fn normalized_shape_cache() -> &'static [Vec<PathCmd>; 8] {
    static SHAPES: OnceLock<[Vec<PathCmd>; 8]> = OnceLock::new();
    SHAPES.get_or_init(|| {
        let c = GROK_AVATAR_CENTER;
        [
            normalize_artifact_commands(build_blob_commands()),
            normalize_artifact_commands(build_pebble_commands()),
            normalize_artifact_commands(build_squircle_commands()),
            normalize_artifact_commands(build_tablet_commands()),
            normalize_artifact_commands(build_rounded_polygon_commands(
                c,
                c,
                3,
                130.0,
                60.0,
                -PI * 0.5,
            )),
            normalize_artifact_commands(build_rounded_polygon_commands(
                c,
                c,
                6,
                114.0,
                20.0,
                PI / 6.0,
            )),
            normalize_artifact_commands(build_cloud_commands()),
            normalize_artifact_commands(build_teardrop_commands()),
        ]
    })
}

fn shape_index_from_resolved(resolved: &str) -> usize {
    GROK_AVATAR_SHAPES
        .iter()
        .position(|&s| s == resolved)
        .unwrap_or(0)
}

fn normalized_shape_commands(source_id: &str, shape: &str) -> &'static [PathCmd] {
    let trimmed = shape.trim();
    let candidate = if trimmed.is_empty() || trimmed.eq_ignore_ascii_case(CLASSIC_INITIALS_AVATAR_SHAPE) {
        None
    } else {
        Some(trimmed)
    };
    let resolved = resolve_grok_persona_shape(source_id, candidate);
    let idx = if resolved == CLASSIC_INITIALS_AVATAR_SHAPE {
        0
    } else {
        shape_index_from_resolved(&resolved)
    };
    &normalized_shape_cache()[idx]
}

fn srgb_channel_luminance(byte: u8) -> f32 {
    let c = f32::from(byte) / 255.0;
    if c <= 0.03928 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

fn relative_luminance(rgb_hex: u32) -> f32 {
    let r = srgb_channel_luminance(((rgb_hex >> 16) & 0xFF) as u8);
    let g = srgb_channel_luminance(((rgb_hex >> 8) & 0xFF) as u8);
    let b = srgb_channel_luminance((rgb_hex & 0xFF) as u8);
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

fn blend_rgb(rgb_a: u32, rgb_b: u32, t: f32) -> u32 {
    let t = t.clamp(0.0, 1.0);
    let mix = |shift: u32| -> u32 {
        let a = f32::from(((rgb_a >> shift) & 0xFF) as u8);
        let b = f32::from(((rgb_b >> shift) & 0xFF) as u8);
        (a + (b - a) * t).round().clamp(0.0, 255.0) as u32
    };
    (mix(16) << 16) | (mix(8) << 8) | mix(0)
}

fn parse_hex_rgb(raw: &str) -> Option<u32> {
    let clean = raw.trim().trim_start_matches('#');
    if clean.len() == 6 && clean.bytes().all(|b| b.is_ascii_hexdigit()) {
        u32::from_str_radix(clean, 16).ok()
    } else if clean.len() == 3 && clean.bytes().all(|b| b.is_ascii_hexdigit()) {
        let v = u32::from_str_radix(clean, 16).ok()?;
        let r = (v >> 8) & 0xF;
        let g = (v >> 4) & 0xF;
        let b = v & 0xF;
        Some(((r * 0x11) << 16) | ((g * 0x11) << 8) | (b * 0x11))
    } else {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ResolvedBodyColors {
    grad_light_rgb: u32,
    grad_dark_rgb: u32,
    feature_rgb: u32,
    swatch_rgb: u32,
}

fn resolve_body_colors(source_id: &str, color: &str, surface_bg_rgb: u32) -> ResolvedBodyColors {
    let trimmed = color.trim();
    let lower = trimmed.to_ascii_lowercase();

    let from_spec = |spec: &GrokColorSpec| -> ResolvedBodyColors {
        if spec.id == "black" {
            if relative_luminance(surface_bg_rgb) < 0.45 {
                ResolvedBodyColors {
                    grad_light_rgb: 0xFAFAFC,
                    grad_dark_rgb: 0xCECED6,
                    feature_rgb: 0x111318,
                    swatch_rgb: spec.swatch_rgb,
                }
            } else {
                ResolvedBodyColors {
                    grad_light_rgb: spec.grad_light_rgb,
                    grad_dark_rgb: spec.grad_dark_rgb,
                    feature_rgb: 0xFFFFFF,
                    swatch_rgb: spec.swatch_rgb,
                }
            }
        } else {
            ResolvedBodyColors {
                grad_light_rgb: spec.grad_light_rgb,
                grad_dark_rgb: spec.grad_dark_rgb,
                feature_rgb: 0xFFFFFF,
                swatch_rgb: spec.swatch_rgb,
            }
        }
    };

    if let Some(spec) = GROK_COLOR_PALETTE.iter().find(|s| s.id == lower) {
        return from_spec(spec);
    }

    if let Some(hex) = parse_hex_rgb(trimmed) {
        if let Some(spec) = GROK_COLOR_PALETTE.iter().find(|s| s.swatch_rgb == hex) {
            return from_spec(spec);
        }
        let grad_light_rgb = blend_rgb(hex, 0xFFFFFF, 0.14);
        let grad_dark_rgb = blend_rgb(hex, 0x000000, 0.14);
        let feature_rgb = if relative_luminance(hex) > 0.72 {
            0x111318
        } else {
            0xFFFFFF
        };
        return ResolvedBodyColors {
            grad_light_rgb,
            grad_dark_rgb,
            feature_rgb,
            swatch_rgb: hex,
        };
    }

    let fallback_id = resolve_grok_persona_color(source_id, None);
    let spec = GROK_COLOR_PALETTE
        .iter()
        .find(|s| s.id == fallback_id)
        .unwrap_or(&GROK_COLOR_PALETTE[7]);
    from_spec(spec)
}

fn dedup_request_animation_frame(window: &Window) {
    thread_local! {
        static LAST_ANIM_REQ: RefCell<HashMap<EntityId, Instant>> = RefCell::new(HashMap::new());
    }
    let view_id = window.current_view();
    let now = Instant::now();
    let gate = Duration::from_millis(ANIM_DEDUP_MS);
    let should_req = LAST_ANIM_REQ.with(|cell| {
        let mut map = cell.borrow_mut();
        if map.len() > 32 {
            let stale = Duration::from_secs(5);
            map.retain(|_, ts| now.saturating_duration_since(*ts) <= stale);
        }
        let allow = map
            .get(&view_id)
            .is_none_or(|prev| now.saturating_duration_since(*prev) >= gate);
        if allow {
            map.insert(view_id, now);
        }
        allow
    });
    if should_req {
        window.request_animation_frame();
    }
}

#[derive(Debug, Clone, Copy)]
struct ViewBoxTransform {
    cos_a: f32,
    sin_a: f32,
    vertical_shift: f32,
    extra_scale: f32,
    origin_x: f32,
    origin_y: f32,
    px_scale: f32,
}

impl ViewBoxTransform {
    #[inline]
    fn map(&self, vx: f32, vy: f32) -> Point<Pixels> {
        let dx = (vx - GROK_AVATAR_CENTER) * self.extra_scale;
        let dy = (vy - GROK_AVATAR_CENTER) * self.extra_scale;
        let rx = GROK_AVATAR_CENTER + dx * self.cos_a - dy * self.sin_a;
        let ry = GROK_AVATAR_CENTER + dx * self.sin_a + dy * self.cos_a + self.vertical_shift;
        point(
            px(self.origin_x + (rx - GROK_VIEWBOX_MIN) * self.px_scale),
            px(self.origin_y + (ry - GROK_VIEWBOX_MIN) * self.px_scale),
        )
    }
}

fn emit_shape_commands(builder: &mut PathBuilder, cmds: &[PathCmd], xf: &ViewBoxTransform) {
    for &cmd in cmds {
        match cmd {
            PathCmd::MoveTo(x, y) => builder.move_to(xf.map(x, y)),
            PathCmd::LineTo(x, y) => builder.line_to(xf.map(x, y)),
            PathCmd::CubicTo { c1, c2, to } => builder.cubic_bezier_to(
                xf.map(to.0, to.1),
                xf.map(c1.0, c1.1),
                xf.map(c2.0, c2.1),
            ),
            PathCmd::Close => builder.close(),
        }
    }
}

fn emit_ellipse(
    builder: &mut PathBuilder,
    cx: f32,
    cy: f32,
    rx: f32,
    ry: f32,
    xf: &ViewBoxTransform,
) {
    const K: f32 = 0.55228475;
    let kx = rx * K;
    let ky = ry * K;

    builder.move_to(xf.map(cx, cy - ry));
    builder.cubic_bezier_to(
        xf.map(cx + rx, cy),
        xf.map(cx + kx, cy - ry),
        xf.map(cx + rx, cy - ky),
    );
    builder.cubic_bezier_to(
        xf.map(cx, cy + ry),
        xf.map(cx + rx, cy + ky),
        xf.map(cx + kx, cy + ry),
    );
    builder.cubic_bezier_to(
        xf.map(cx - rx, cy),
        xf.map(cx - kx, cy + ry),
        xf.map(cx - rx, cy + ky),
    );
    builder.cubic_bezier_to(
        xf.map(cx, cy - ry),
        xf.map(cx - rx, cy - ky),
        xf.map(cx - kx, cy - ry),
    );
    builder.close();
}

fn emit_pixel_circle(builder: &mut PathBuilder, center: Point<Pixels>, radius_px: f32) {
    const K: f32 = 0.55228475;
    let r = px(radius_px);
    let k = px(radius_px * K);
    let cx = center.x;
    let cy = center.y;

    builder.move_to(point(cx, cy - r));
    builder.cubic_bezier_to(point(cx + r, cy), point(cx + k, cy - r), point(cx + r, cy - k));
    builder.cubic_bezier_to(point(cx, cy + r), point(cx + r, cy + k), point(cx + k, cy + r));
    builder.cubic_bezier_to(point(cx - r, cy), point(cx - k, cy + r), point(cx - r, cy + k));
    builder.cubic_bezier_to(point(cx, cy - r), point(cx - r, cy - k), point(cx - k, cy - r));
    builder.close();
}

fn paint_grok_avatar(bounds: Bounds<Pixels>, props: &GrokBotAvatarProps, window: &mut Window) {
    let bw = f32::from(bounds.size.width);
    let bh = f32::from(bounds.size.height);
    let side = bw.min(bh);
    if !side.is_finite() || side <= 0.5 {
        return;
    }

    let origin_x = f32::from(bounds.origin.x) + (bw - side) * 0.5;
    let origin_y = f32::from(bounds.origin.y) + (bh - side) * 0.5;
    let px_scale = side / GROK_VIEWBOX_SIZE;

    let cmds = normalized_shape_commands(&props.source_id, &props.shape);
    let colors = resolve_body_colors(&props.source_id, &props.color, props.surface_bg_rgb);
    let motion = props.state.motion();
    let id_hash = grok_shipped_hash(&props.source_id);

    let (bob, bounce, spin_deg, blink_scale, gx, gy) = if props.paused {
        (0.0_f32, 0.0_f32, 0.0_f32, 1.0_f32, 0.0_f32, 0.0_f32)
    } else {
        let now = Instant::now();
        let elapsed_ms = now.saturating_duration_since(engine_epoch()).as_secs_f64() * 1000.0;

        let amp = if props.emphasis {
            motion.amplitude * 1.15
        } else {
            motion.amplitude
        };
        let phase_offset = f64::from(id_hash % 3600);
        let bob = if amp > 0.0 && motion.period_ms > 0.0 {
            let theta = ((elapsed_ms + phase_offset) / f64::from(motion.period_ms)) * std::f64::consts::TAU;
            (theta.sin() as f32) * amp
        } else {
            0.0
        };

        let (spin_deg, bounce, _impulse_active) = sample_impulses(&props.source_id, now);

        let blink_scale = if props.state == GrokPersonaState::Sleeping {
            1.0
        } else {
            let blink_period = 4200.0 + f64::from(id_hash % 1700);
            let blink_stagger = f64::from((id_hash >> 8) % 3000);
            let blink_phase = (elapsed_ms + blink_stagger) % blink_period;
            if blink_phase < BLINK_DURATION_MS {
                let u = (blink_phase / BLINK_DURATION_MS) as f32;
                let tri = (u * PI).sin();
                (1.0 - 0.92 * tri).max(0.08)
            } else {
                1.0
            }
        };

        let (gx, gy) = if props.follow_pointer && props.state != GrokPersonaState::Sleeping {
            let mouse = window.mouse_position();
            let mx = f32::from(mouse.x);
            let my = f32::from(mouse.y);
            if mx.is_finite() && my.is_finite() {
                let cx_px = f32::from(bounds.origin.x) + bw * 0.5;
                let cy_px = f32::from(bounds.origin.y) + bh * 0.5;
                let dx = mx - cx_px;
                let dy = my - cy_px;
                let dist = (dx * dx + dy * dy).sqrt();
                let denom = dist + 72.0;
                ((dx / denom).clamp(-1.0, 1.0), (dy / denom).clamp(-1.0, 1.0))
            } else {
                (0.0, 0.0)
            }
        } else if props.state == GrokPersonaState::Thinking {
            (0.3, -0.35)
        } else {
            (0.0, 0.0)
        };

        dedup_request_animation_frame(window);
        (bob, bounce, spin_deg, blink_scale, gx, gy)
    };

    let angle_rad = (motion.tilt_deg + spin_deg).to_radians();
    let (sin_a, cos_a) = angle_rad.sin_cos();
    let xf = ViewBoxTransform {
        cos_a,
        sin_a,
        vertical_shift: -bob - bounce,
        extra_scale: 1.0,
        origin_x,
        origin_y,
        px_scale,
    };

    if props.emphasis {
        let halo_xf = ViewBoxTransform {
            extra_scale: 1.06,
            ..xf
        };
        let mut halo_builder = PathBuilder::fill();
        emit_shape_commands(&mut halo_builder, cmds, &halo_xf);
        if let Ok(path) = halo_builder.build() {
            let halo_color: Hsla = rgb(colors.grad_light_rgb).into();
            window.paint_path(path, halo_color.opacity(0.22));
        }
    }

    let mut body_builder = PathBuilder::fill();
    emit_shape_commands(&mut body_builder, cmds, &xf);
    if let Ok(path) = body_builder.build() {
        window.paint_path(
            path,
            linear_gradient(
                135.0,
                linear_color_stop(rgb(colors.grad_light_rgb), 0.0),
                linear_color_stop(rgb(colors.grad_dark_rgb), 1.0),
            ),
        );
    }

    let effective_size = side.min(props.size_px.max(0.0));
    if effective_size < 14.0 {
        return;
    }

    let base_ry = if props.state == GrokPersonaState::Sleeping {
        2.0
    } else {
        7.0
    };
    let raw_ry = base_ry * motion.eye_scale * blink_scale;
    let min_ry_px = if props.state == GrokPersonaState::Sleeping || blink_scale < 0.35 {
        0.65
    } else {
        0.85
    };
    let rx_vb = 10.0_f32.max(1.0 / px_scale);
    let ry_vb = raw_ry.max(min_ry_px / px_scale);

    let eye_y = GROK_AVATAR_CENTER - 8.0 + gy * 3.0;
    let left_eye_x = GROK_AVATAR_CENTER - 29.0 + gx * 4.0;
    let right_eye_x = GROK_AVATAR_CENTER + 29.0 + gx * 4.0;

    let mut eyes_builder = PathBuilder::fill();
    emit_ellipse(&mut eyes_builder, left_eye_x, eye_y, rx_vb, ry_vb, &xf);
    emit_ellipse(&mut eyes_builder, right_eye_x, eye_y, rx_vb, ry_vb, &xf);
    if let Ok(path) = eyes_builder.build() {
        window.paint_path(path, rgb(colors.feature_rgb));
    }

    if matches!(
        props.state,
        GrokPersonaState::Happy
            | GrokPersonaState::Excited
            | GrokPersonaState::Celebrate
            | GrokPersonaState::Laughing
    ) {
        let stroke_px = (5.0_f32 * px_scale).max(1.0_f32);
        let p0 = xf.map(GROK_AVATAR_CENTER - 20.0, GROK_AVATAR_CENTER + 24.0);
        let ctrl = xf.map(GROK_AVATAR_CENTER, GROK_AVATAR_CENTER + 38.0);
        let p1 = xf.map(GROK_AVATAR_CENTER + 20.0, GROK_AVATAR_CENTER + 24.0);

        let mut smile_builder = PathBuilder::stroke(px(stroke_px));
        smile_builder.move_to(p0);
        smile_builder.curve_to(p1, ctrl);
        if let Ok(path) = smile_builder.build() {
            window.paint_path(path, rgb(colors.feature_rgb));
        }

        let mut caps_builder = PathBuilder::fill();
        let cap_r = stroke_px * 0.5;
        emit_pixel_circle(&mut caps_builder, p0, cap_r);
        emit_pixel_circle(&mut caps_builder, p1, cap_r);
        if let Ok(path) = caps_builder.build() {
            window.paint_path(path, rgb(colors.feature_rgb));
        }
    }
}

/// Renders an animated (or paused) Grok Bot 0.18 dynamic vector character avatar.
pub fn render_grok_bot_avatar(props: GrokBotAvatarProps) -> gpui::Div {
    let size_px = if props.size_px.is_finite() && props.size_px > 0.0 {
        props.size_px
    } else {
        32.0
    };
    let render_props = GrokBotAvatarProps {
        size_px,
        ..props
    };

    div()
        .relative()
        .w(px(size_px))
        .h(px(size_px))
        .flex_shrink_0()
        .flex()
        .items_center()
        .justify_center()
        .child(
            canvas(
                |_, _, _| {},
                move |bounds, _, window, _cx| {
                    paint_grok_avatar(bounds, &render_props, window);
                },
            )
            .w(px(size_px))
            .h(px(size_px)),
        )
}

fn persona_initials(fallback_label: &str, source_id: &str) -> String {
    let source = if fallback_label.trim().is_empty() || fallback_label.trim() == "?" {
        source_id
    } else {
        fallback_label
    };
    let mut out = String::new();
    for part in source.split_whitespace().take(2) {
        if let Some(c) = part.chars().next() {
            out.push(c);
        }
    }
    if out.is_empty() {
        for c in source.trim().chars().take(2) {
            if c.is_alphanumeric() {
                out.push(c);
            }
        }
    }
    if out.is_empty() {
        out.push('?');
    }
    out.to_uppercase()
}

/// Renders a bot persona avatar, falling back to classic circular initials when `shape` is `"classic"`.
pub fn render_grok_persona_avatar(
    source_id: &str,
    shape: &str,
    color: &str,
    fallback_label: &str,
    size_px: f32,
    state: GrokPersonaState,
    surface_bg_rgb: u32,
    paused: bool,
) -> gpui::Div {
    let clean_size = if size_px.is_finite() && size_px > 0.0 {
        size_px
    } else {
        32.0
    };
    if shape.trim().eq_ignore_ascii_case(CLASSIC_INITIALS_AVATAR_SHAPE) {
        let resolved = resolve_body_colors(source_id, color, surface_bg_rgb);
        let bg_rgb = resolved.swatch_rgb;
        let fg_rgb = if relative_luminance(bg_rgb) > 0.72 {
            0x111318
        } else {
            0xFFFFFF
        };
        let initials = persona_initials(fallback_label, source_id);
        return div()
            .relative()
            .w(px(clean_size))
            .h(px(clean_size))
            .flex_shrink_0()
            .rounded_full()
            .bg(rgb(bg_rgb))
            .border_1()
            .border_color(rgba(0xFFFFFF38))
            .flex()
            .items_center()
            .justify_center()
            .text_color(rgb(fg_rgb))
            .text_size(px((clean_size * 0.36).clamp(8.0, 24.0)))
            .font_weight(FontWeight::BOLD)
            .child(initials);
    }

    render_grok_bot_avatar(
        GrokBotAvatarProps::new(source_id, shape, color, clean_size)
            .with_state(state)
            .with_surface_bg(surface_bg_rgb)
            .with_paused(paused)
            .with_follow_pointer(!paused),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[std::prelude::v1::test]
    fn all_8_normalized_shapes_tessellate_and_center_near_114_27() {
        let xf = ViewBoxTransform {
            cos_a: 1.0,
            sin_a: 0.0,
            vertical_shift: 0.0,
            extra_scale: 1.0,
            origin_x: 0.0,
            origin_y: 0.0,
            px_scale: 64.0 / GROK_VIEWBOX_SIZE,
        };

        for &shape in &GROK_AVATAR_SHAPES {
            let cmds = normalized_shape_commands("test-agent", shape);
            assert!(!cmds.is_empty(), "shape {shape} must emit commands");

            let (min_x, min_y, max_x, max_y) = compute_bbox(cmds);
            let cx = (min_x + max_x) * 0.5;
            let cy = (min_y + max_y) * 0.5;
            let w = max_x - min_x;
            let h = max_y - min_y;

            assert!(
                (cx - GROK_AVATAR_CENTER).abs() < 0.05,
                "shape {shape} bbox cx={cx} should be near {GROK_AVATAR_CENTER}"
            );
            assert!(
                (cy - GROK_AVATAR_CENTER).abs() < 0.05,
                "shape {shape} bbox cy={cy} should be near {GROK_AVATAR_CENTER}"
            );
            assert!(
                w.max(h) >= 180.0 && w.max(h) <= 230.0,
                "shape {shape} normalized extent {} out of expected bounds",
                w.max(h)
            );
            assert!(
                min_x >= GROK_VIEWBOX_MIN && max_x <= GROK_VIEWBOX_MIN + GROK_VIEWBOX_SIZE,
                "shape {shape} x=[{min_x}, {max_x}] must stay inside ViewBox"
            );
            assert!(
                min_y >= GROK_VIEWBOX_MIN && max_y <= GROK_VIEWBOX_MIN + GROK_VIEWBOX_SIZE,
                "shape {shape} y=[{min_y}, {max_y}] must stay inside ViewBox"
            );

            let mut builder = PathBuilder::fill();
            emit_shape_commands(&mut builder, cmds, &xf);
            let built = builder
                .build()
                .unwrap_or_else(|e| panic!("shape {shape} failed PathBuilder tessellation: {e}"));
            let dbg = format!("{built:?}");
            assert!(
                !dbg.contains("vertices: []"),
                "shape {shape} produced empty vertex buffer"
            );
        }
    }

    #[std::prelude::v1::test]
    fn all_11_palette_colors_and_custom_hex_resolve_with_black_dark_contrast() {
        assert_eq!(GROK_COLOR_PALETTE.len(), 11);

        for spec in GROK_COLOR_PALETTE {
            let light_res = resolve_body_colors("agent-x", spec.id, 0xFFFFFF);
            assert_eq!(light_res.swatch_rgb, spec.swatch_rgb);
            assert_eq!(light_res.grad_light_rgb, spec.grad_light_rgb);
            assert_eq!(light_res.grad_dark_rgb, spec.grad_dark_rgb);
            assert_eq!(light_res.feature_rgb, 0xFFFFFF);
        }

        let black_dark = resolve_body_colors("agent-x", "black", 0x111318);
        assert_eq!(black_dark.grad_light_rgb, 0xFAFAFC);
        assert_eq!(black_dark.grad_dark_rgb, 0xCECED6);
        assert_eq!(black_dark.feature_rgb, 0x111318);

        let black_light = resolve_body_colors("agent-x", "black", 0xF8FAFC);
        assert_eq!(black_light.grad_light_rgb, 0x323238);
        assert_eq!(black_light.grad_dark_rgb, 0x050507);
        assert_eq!(black_light.feature_rgb, 0xFFFFFF);

        let custom_6 = resolve_body_colors("agent-x", "#10B981", 0xFFFFFF);
        assert_eq!(custom_6.swatch_rgb, 0x10B981);
        assert_eq!(custom_6.feature_rgb, 0xFFFFFF);

        let custom_3 = resolve_body_colors("agent-x", "#f60", 0xFFFFFF);
        assert_eq!(custom_3.swatch_rgb, 0xFF6600);

        let fallback = resolve_body_colors("agent-x", "", 0xFFFFFF);
        assert!(GROK_COLOR_PALETTE
            .iter()
            .any(|s| s.swatch_rgb == fallback.swatch_rgb));
    }

    #[std::prelude::v1::test]
    fn all_39_states_return_finite_motion_specs_and_activity_mapping() {
        assert_eq!(GrokPersonaState::ALL.len(), 39);
        for state in GrokPersonaState::ALL {
            let m = state.motion();
            assert!(m.amplitude.is_finite() && m.amplitude >= 0.0);
            assert!(m.period_ms.is_finite() && m.period_ms > 0.0);
            assert!(m.tilt_deg.is_finite());
            assert!(m.eye_scale.is_finite() && m.eye_scale > 0.0);
        }

        let sleeping = GrokPersonaState::Sleeping.motion();
        assert_eq!(sleeping.amplitude, 0.0);
        assert_eq!(sleeping.period_ms, 6000.0);
        assert_eq!(sleeping.eye_scale, 0.12);

        let idle = GrokPersonaState::Idle.motion();
        assert_eq!(idle.amplitude, 1.5);
        assert_eq!(idle.period_ms, 9000.0);
        assert_eq!(idle.eye_scale, 1.0);

        let celebrate = GrokPersonaState::Celebrate.motion();
        assert_eq!(celebrate.amplitude, 7.0);
        assert_eq!(celebrate.period_ms, 1400.0);
        assert_eq!(celebrate.eye_scale, 1.12);

        assert_eq!(
            GrokPersonaState::from_activity_str("idle", false),
            GrokPersonaState::Idle
        );
        assert_eq!(
            GrokPersonaState::from_activity_str("idle", true),
            GrokPersonaState::Thinking
        );
        assert_eq!(
            GrokPersonaState::from_activity_str("running", false),
            GrokPersonaState::Working
        );
        assert_eq!(
            GrokPersonaState::from_activity_str("powering-down", false),
            GrokPersonaState::PoweringDown
        );
    }
}
