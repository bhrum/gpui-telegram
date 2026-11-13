//! Reactions Subsystem Components (Slice 7)
pub mod button;
pub mod list;
pub mod paid_toast;
pub mod selector;
pub mod strip;
pub mod transcribe;

pub use button::{
    ButtonParameters, ExpandDirection, ReactionButtonState, ReactionKey, ReactionPill,
    opacity_for_scale, scale_for_state, sort_reaction_pills,
    K_ACTIVATE_DURATION_MS, K_BUTTON_EXPAND_DELAY_MS, K_COLLAPSE_DURATION_MS,
    K_EXPAND_DURATION_MS, K_MAX_RECENT_USERPICS, K_MAX_REACTIONS_SCROLL_AT_ONCE,
    K_TOGGLE_DURATION_MS, REACTION_CORNER_ADDED_HEIGHT_MAX, REACTION_CORNER_SIZE,
    REACTION_CORNER_SKIP, REACTION_TAG_ARROW, REACTION_TAG_LEFT_RADIUS, REACTION_TAG_RIGHT_RADIUS,
};
pub use list::{
    ReactorItem, WhoReactedList, K_PER_PAGE, K_PER_PAGE_FIRST,
};
pub use paid_toast::{
    PaidReactionToast, StarParticle, PAID_REACTION_STAR_TIERS, UNDO_COUNTDOWN_TOTAL_MS,
};
pub use selector::{
    ReactionCategory, ReactionOption, ReactionsSelector, SelectorState,
    K_ACCEPT_CLICKS_AFTER_MS, K_DEFAULT_COLUMNS, K_EXPAND_DELAY_MS,
    K_EXPAND_DURATION_MS as SELECTOR_K_EXPAND_DURATION_MS,
    K_FRAMES_COUNT, K_MIN_NON_TRANSPARENT_COLUMNS, K_SCALE_DURATION_MS,
};
pub use strip::{
    ReactionsStrip, StripReactionItem,
    K_EMOJI_CACHE_INDEX, K_HOVER_SCALE, K_HOVER_SCALE_DURATION_MS, K_SIZE_FOR_DOWNSCALE,
};
pub use transcribe::{
    TranscribeButton, TranscribeState,
    K_ARROW_PIVOT_FAR, K_ARROW_PIVOT_NEAR,
    TRANSCRIBE_SIZE_AUDIO, TRANSCRIBE_SIZE_ROUND_VIDEO,
};
