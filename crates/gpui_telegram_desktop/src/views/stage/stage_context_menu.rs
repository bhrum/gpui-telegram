//! Out-of-Flow Floating Reaction Strip, Expandable Emoji Reaction Selector, & Right-Click Context Menu.
//!
//! Upstream references:
//! - `Telegram/SourceFiles/history/view/reactions/history_view_reactions_button.cpp:L30-L330`
//! - `Telegram/SourceFiles/history/view/reactions/history_view_reactions_strip.cpp:L20-L80`
//! - `Telegram/SourceFiles/history/view/reactions/history_view_reactions_selector.cpp:L45-L160`

use gpui::*;
use gpui::prelude::FluentBuilder;

use crate::i18n::tr_key_or;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReactionOverlayMode {
    /// Non-overlapping side-corner circular reaction button (`history_view_reactions_button.cpp` `ButtonState::Shown`/`Active`).
    CornerButton,
    /// Expanded quick-reaction pill (`history_view_reactions_strip.cpp` `ButtonState::Inside`).
    QuickStrip,
    /// Full 8-column reaction selector grid (`history_view_reactions_selector.cpp`).
    ExpandedSelector,
}

#[derive(Debug, Clone)]
pub enum ContextMenuAction {
    React(u64, String),
    ToggleExpandReactions,
    Reply(u64),
    Edit(u64),
    TogglePin(u64),
    CopyText(u64),
    Forward(u64),
    Translate(u64),
    Select(u64),
    Delete(u64),
    CloseMenu,
}

/// Canonical Telegram quick-reaction strip emojis (`history_view_reactions_strip.cpp`).
pub const QUICK_REACTIONS: &[&str] = &["👍", "❤️", "🔥", "🎉", "👏", "😁", "🤔"];

/// Extended 8-column Telegram reaction catalog (`history_view_reactions_selector.cpp: K_DEFAULT_COLUMNS = 8`).
pub const EXTENDED_REACTIONS: &[&str] = &[
    "👍", "👎", "❤️", "🔥", "🥰", "👏", "😁", "🤔",
    "🤯", "😱", "🤬", "😢", "🎉", "🤩", "🤮", "💩",
    "🙏", "👌", "🕊️", "🤡", "😍", "❤️‍🔥", "💯", "🤣",
    "⚡", "🏆", "🐳", "🌚", "🌭", "🍌", "🥱", "🥴",
];

/// Renders an out-of-flow 3-state reaction overlay anchored to `bubble`.
///
/// Key invariants matching Telegram Desktop (`history_view_reactions_button.cpp`, `strip.cpp`, `selector.cpp`):
/// 1. Rendered via `deferred(...).with_priority(30)` + `.absolute()`, taking **0x0** flow space so
///    message row heights and bubble bounds NEVER shift or jitter when hovering bubble edges.
/// Viewport-clamped, non-occluding geometry parameters for `render_hover_reaction_overlay`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReactionOverlayLayout {
    /// Whether `ExpandedSelector` (`188px` tall) opens downward below `#bubble-{msg_id}` (`true`)
    /// or upward above `#bubble-{msg_id}` (`false`).
    pub selector_below: bool,
    /// `top` offset for `ExpandedSelector` when `selector_below == false` (`-192.0` normally so
    /// it clears `#bubble-{msg_id}` by `4px`; clamped only when an ultra-tall bubble leaves `< 196px`
    /// both above and below).
    pub selector_top_when_above: f32,
    /// `bottom` offset for `ExpandedSelector` when `selector_below == true` (`-192.0` normally so
    /// it clears `#bubble-{msg_id}` by `4px`; clamped to stay `>= 8px` above viewport bottom).
    pub selector_bottom_when_below: f32,
    /// Whether `QuickStrip` (`32px` tall) flips above `#bubble-{msg_id}` (`true`, when a row's
    /// bottom is scrolled right against the bottom viewport edge) or sits `4px` below `#bubble-{msg_id}`.
    pub strip_above: bool,
    /// Clamped horizontal offset (`left` for incoming, `right` for outgoing) for `QuickStrip` (`232px` wide).
    pub strip_side_offset: f32,
    /// Clamped horizontal offset (`left` for incoming, `right` for outgoing) for `ExpandedSelector` (`286px` wide).
    pub selector_side_offset: f32,
}

pub const REACTION_CORNER_OVERHANG_PX: f32 = 36.0;
pub const REACTION_QUICK_STRIP_WIDTH_PX: f32 = 232.0;
pub const REACTION_QUICK_STRIP_HEIGHT_PX: f32 = 32.0;
pub const REACTION_SELECTOR_WIDTH_PX: f32 = 286.0;
pub const REACTION_SELECTOR_HEIGHT_PX: f32 = 188.0;

/// Computes the clamped horizontal offset (`left` for incoming bubbles, `right` for outgoing bubbles)
/// so `QuickStrip` (`232px`) and `ExpandedSelector` (`286px`) anchor right at the outer `CornerButton` /
/// `▾` edge (`36px <= 56px` reserved side margin) while never clipping off narrow windows or narrow bubbles.
pub fn compute_reaction_overlay_side_offset(
    overlay_width: f32,
    bubble_width: f32,
    stage_width: f32,
    is_outgoing: bool,
    reserve_avatar_column: bool,
) -> f32 {
    let scroll_and_row_pad = 28.0_f32; // 12px scroll px + 16px row px
    let viewport_margin = 8.0_f32;
    let avatar_skip = if !is_outgoing && reserve_avatar_column {
        40.0_f32
    } else {
        0.0_f32
    };
    let anchor_inset = scroll_and_row_pad + avatar_skip;
    let min_offset = viewport_margin - anchor_inset;
    let max_stage_offset = (stage_width - viewport_margin - anchor_inset - overlay_width).max(min_offset);
    let desired = (bubble_width + REACTION_CORNER_OVERHANG_PX - overlay_width).max(0.0);
    desired.min(max_stage_offset).max(min_offset)
}

/// Selects whether `ExpandedSelector` (`188px` tall + `4px` gap) should open below (`true`) or above (`false`)
/// `#bubble-{msg_id}`, preferring above when `rel_top >= 196px`, below when `viewport_height - rel_bottom >= 196px`,
/// and whichever side has strictly greater viewport clearance on compact viewports where neither side has `196px`.
pub fn should_open_selector_below(rel_top: f32, rel_bottom: f32, viewport_height: f32) -> bool {
    let space_above = rel_top;
    let space_below = viewport_height - rel_bottom;
    if space_above >= (REACTION_SELECTOR_HEIGHT_PX + 8.0) {
        false
    } else if space_below >= (REACTION_SELECTOR_HEIGHT_PX + 8.0) {
        true
    } else {
        space_below > space_above
    }
}

impl ReactionOverlayLayout {
    pub fn compute(
        selector_below: bool,
        rel_top: f32,
        rel_bottom: f32,
        viewport_height: f32,
        bubble_width: f32,
        stage_width: f32,
        is_outgoing: bool,
        reserve_avatar_column: bool,
    ) -> Self {
        let strip_above = (viewport_height - rel_bottom) < 38.0 && rel_top >= 38.0;
        let selector_top_when_above = if rel_top >= (REACTION_SELECTOR_HEIGHT_PX + 8.0) {
            -(REACTION_SELECTOR_HEIGHT_PX + 4.0)
        } else {
            (8.0 - rel_top).max(-(REACTION_SELECTOR_HEIGHT_PX + 4.0))
        };
        let space_below = viewport_height - rel_bottom;
        let selector_bottom_when_below = if space_below >= (REACTION_SELECTOR_HEIGHT_PX + 8.0) {
            -(REACTION_SELECTOR_HEIGHT_PX + 4.0)
        } else {
            (rel_bottom + 8.0 - viewport_height).max(-(REACTION_SELECTOR_HEIGHT_PX + 4.0))
        };
        let strip_side_offset = compute_reaction_overlay_side_offset(
            REACTION_QUICK_STRIP_WIDTH_PX,
            bubble_width,
            stage_width,
            is_outgoing,
            reserve_avatar_column,
        );
        let selector_side_offset = compute_reaction_overlay_side_offset(
            REACTION_SELECTOR_WIDTH_PX,
            bubble_width,
            stage_width,
            is_outgoing,
            reserve_avatar_column,
        );
        Self {
            selector_below,
            selector_top_when_above,
            selector_bottom_when_below,
            strip_above,
            strip_side_offset,
            selector_side_offset,
        }
    }
}

/// Renders the Telegram Desktop 3-state out-of-flow reaction overlay (`deferred`):
/// 1. Rendered inside `deferred(...)` so mounting/unmounting **never** mutates message row height
///    or causes layout jitter when the pointer crosses message edges.
/// 2. `CornerButton` sits strictly outside `bubble` (`[W, W+36]` incoming / `[-36, 0]` outgoing,
///    `bottom: 0`, `30px` tall) with **0.0px** overlap with `bubble` or adjacent bubbles.
/// 3. `QuickStrip` sits `4px` outside `bubble` (`y ∈ [H+4, H+36]` normally, or `y ∈ [-36, -4]` when
///    scrolled against the bottom edge) with an outer-gutter corner bridge (`[W, W+36]`), so it has
///    **0.0px** overlap with message text, timestamps, or `#rx-pill` chips and overhangs at most
///    `36px` (`<= 56px` reserved side margin).
/// 4. `ExpandedSelector` anchors at the same outer corner (`36px` overhang, horizontally clamped to
///    `stage_width`) and opens `4px` above or below `bubble` without covering the bubble body.
#[allow(clippy::too_many_arguments)]
pub fn render_hover_reaction_overlay<V: 'static>(
    msg_id: u64,
    is_outgoing: bool,
    layout: ReactionOverlayLayout,
    mode: ReactionOverlayMode,
    chosen_emojis: &[String],
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_hover_zone: impl Fn(&mut V, bool, &mut Context<V>) + 'static,
    on_hover_bridge: impl Fn(&mut V, bool, &mut Context<V>) + 'static,
    on_toggle_expand: impl Fn(&mut V, &mut Context<V>) + Clone + 'static,
    on_react: impl Fn(&mut V, String, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let chosen_bg = TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.22);
    let hover_bg = rgb(palette.menu_hover_bg);

    let zone = match mode {
        ReactionOverlayMode::CornerButton => {
            let toggle_exp = on_toggle_expand.clone();
            let btn = div()
                .id(SharedString::from(format!("rx-corner-btn-{}", msg_id)))
                .w(px(30.0))
                .h(px(30.0))
                .rounded_full()
                .bg(rgb(palette.menu_bg))
                .border_1()
                .border_color(rgb(palette.border_fg))
                .shadow_sm()
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .hover(move |s| s.bg(hover_bg))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _, _, cx| {
                        cx.stop_propagation();
                        toggle_exp(this, cx);
                    }),
                )
                .on_mouse_down(MouseButton::Right, |_, _, cx| cx.stop_propagation())
                .child(tg_icon(IconName::HeartReaction, 15.0, palette.icon_fg));

            div()
                .id(SharedString::from(format!("rx-hover-zone-{}", msg_id)))
                .absolute()
                .when(!layout.strip_above, |el| el.bottom(px(0.0)))
                .when(layout.strip_above, |el| el.top(px(0.0)))
                .w(px(36.0))
                .h(px(30.0))
                .when(is_outgoing, |el| el.left(px(-36.0)).pr(px(6.0)))
                .when(!is_outgoing, |el| el.right(px(-36.0)).pl(px(6.0)))
                .flex()
                .items_center()
                .justify_center()
                .block_mouse_except_scroll()
                .on_hover(cx.listener(move |this, hovered, _, cx| {
                    on_hover_zone(this, *hovered, cx);
                }))
                .child(btn)
        }
        ReactionOverlayMode::QuickStrip => {
            let mut row = div()
                .id(SharedString::from(format!("rx-strip-card-{}", msg_id)))
                .w(px(REACTION_QUICK_STRIP_WIDTH_PX))
                .h(px(REACTION_QUICK_STRIP_HEIGHT_PX))
                .px(px(6.0))
                .rounded_full()
                .bg(rgb(palette.menu_bg))
                .border_1()
                .border_color(rgb(palette.border_fg))
                .shadow_md()
                .flex()
                .items_center()
                .justify_between()
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .on_mouse_down(MouseButton::Right, |_, _, cx| cx.stop_propagation());

            for (idx, &emoji) in QUICK_REACTIONS.iter().enumerate() {
                let em = emoji.to_string();
                let is_chosen = chosen_emojis.iter().any(|c| c == emoji);
                let act = on_react.clone();
                row = row.child(
                    div()
                        .id(SharedString::from(format!("qreact-{}-{}", msg_id, idx)))
                        .w(px(26.0))
                        .h(px(26.0))
                        .rounded_full()
                        .flex_shrink_0()
                        .flex()
                        .items_center()
                        .justify_center()
                        .cursor_pointer()
                        .when(is_chosen, |el| {
                            el.bg(chosen_bg)
                                .border_1()
                                .border_color(rgb(palette.accent))
                        })
                        .hover(move |s| s.bg(hover_bg).text_size(px(17.0)))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
                                cx.stop_propagation();
                                act(this, em.clone(), cx);
                            }),
                        )
                        .text_size(px(15.0))
                        .child(emoji),
                );
            }

            let toggle_exp = on_toggle_expand.clone();
            row = row.child(
                div()
                    .id(SharedString::from(format!("qreact-expand-{}", msg_id)))
                    .w(px(22.0))
                    .h(px(22.0))
                    .rounded_full()
                    .flex_shrink_0()
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .bg(TdesktopThemePalette::rgba_with_alpha(palette.border_fg, 0.35))
                    .hover(move |s| s.bg(hover_bg))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            cx.stop_propagation();
                            toggle_exp(this, cx);
                        }),
                    )
                    .child(tg_icon(IconName::ChevronDown, 12.0, palette.icon_fg)),
            );

            // Outer-gutter bridge strictly outside `#bubble-{msg_id}` (`[W, W+36]` / `[-36, 0]`)
            // so expanding `CornerButton -> QuickStrip` keeps the pointer hovered without covering
            // a single pixel of `#bubble-{msg_id}`.
            let bridge = div()
                .id(SharedString::from(format!("rx-strip-bridge-{}", msg_id)))
                .absolute()
                .w(px(REACTION_CORNER_OVERHANG_PX))
                .when(is_outgoing, |el| el.left(px(-REACTION_CORNER_OVERHANG_PX)))
                .when(!is_outgoing, |el| el.right(px(-REACTION_CORNER_OVERHANG_PX)))
                .when(!layout.strip_above, |el| el.bottom(px(-4.0)).h(px(34.0)))
                .when(layout.strip_above, |el| el.top(px(-4.0)).h(px(34.0)))
                .block_mouse_except_scroll()
                .on_hover(cx.listener(move |this, hovered, _, cx| {
                    on_hover_bridge(this, *hovered, cx);
                }));

            let strip_zone = div()
                .id(SharedString::from(format!("rx-hover-zone-{}", msg_id)))
                .absolute()
                .w(px(REACTION_QUICK_STRIP_WIDTH_PX))
                .h(px(REACTION_QUICK_STRIP_HEIGHT_PX + 4.0))
                .when(is_outgoing, |el| el.right(px(layout.strip_side_offset)))
                .when(!is_outgoing, |el| el.left(px(layout.strip_side_offset)))
                .when(!layout.strip_above, |el| {
                    el.bottom(px(-(REACTION_QUICK_STRIP_HEIGHT_PX + 4.0))).pt(px(4.0))
                })
                .when(layout.strip_above, |el| {
                    el.top(px(-(REACTION_QUICK_STRIP_HEIGHT_PX + 4.0))).pb(px(4.0))
                })
                .flex()
                .items_center()
                .block_mouse_except_scroll()
                .on_hover(cx.listener(move |this, hovered, _, cx| {
                    on_hover_zone(this, *hovered, cx);
                }))
                .child(row);

            div()
                .id(SharedString::from(format!("rx-quick-wrap-{}", msg_id)))
                .absolute()
                .inset_0()
                .child(bridge)
                .child(strip_zone)
        }
        ReactionOverlayMode::ExpandedSelector => {
            let toggle_collapse = on_toggle_expand.clone();
            let header = div()
                .w_full()
                .h(px(26.0))
                .px(px(4.0))
                .pb(px(4.0))
                .border_b_1()
                .border_color(rgb(palette.border_fg))
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_size(px(11.5))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(palette.text_secondary))
                        .child(tr_key_or(lang_id, "lng_reactions_title", "Reactions")),
                )
                .child(
                    div()
                        .id(SharedString::from(format!("qreact-collapse-{}", msg_id)))
                        .w(px(20.0))
                        .h(px(20.0))
                        .rounded_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .cursor_pointer()
                        .hover(move |s| s.bg(hover_bg))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
                                cx.stop_propagation();
                                toggle_collapse(this, cx);
                            }),
                        )
                        .child(tg_icon(IconName::Close, 11.0, palette.icon_fg)),
                );

            let mut grid = div()
                .w_full()
                .pt(px(6.0))
                .flex()
                .flex_wrap()
                .gap(px(4.0));

            for (idx, &emoji) in EXTENDED_REACTIONS.iter().enumerate() {
                let em = emoji.to_string();
                let is_chosen = chosen_emojis.iter().any(|c| c == emoji);
                let act = on_react.clone();
                grid = grid.child(
                    div()
                        .id(SharedString::from(format!("qreact-grid-{}-{}", msg_id, idx)))
                        .w(px(30.0))
                        .h(px(30.0))
                        .rounded(px(8.0))
                        .flex_shrink_0()
                        .flex()
                        .items_center()
                        .justify_center()
                        .cursor_pointer()
                        .when(is_chosen, |el| {
                            el.bg(chosen_bg)
                                .border_1()
                                .border_color(rgb(palette.accent))
                        })
                        .hover(move |s| s.bg(hover_bg).text_size(px(18.0)))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
                                cx.stop_propagation();
                                act(this, em.clone(), cx);
                            }),
                        )
                        .text_size(px(16.0))
                        .child(emoji),
                );
            }

            let card = div()
                .id(SharedString::from(format!("rx-selector-card-{}", msg_id)))
                .w(px(REACTION_SELECTOR_WIDTH_PX))
                .h(px(REACTION_SELECTOR_HEIGHT_PX))
                .p(px(8.0))
                .rounded(px(14.0))
                .bg(rgb(palette.menu_bg))
                .border_1()
                .border_color(rgb(palette.border_fg))
                .shadow_lg()
                .flex()
                .flex_col()
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .on_mouse_down(MouseButton::Right, |_, _, cx| cx.stop_propagation())
                .child(header)
                .child(grid);

            div()
                .id(SharedString::from(format!("rx-hover-zone-{}", msg_id)))
                .absolute()
                .w(px(REACTION_SELECTOR_WIDTH_PX))
                .h(px(REACTION_SELECTOR_HEIGHT_PX))
                .when(is_outgoing, |el| el.right(px(layout.selector_side_offset)))
                .when(!is_outgoing, |el| el.left(px(layout.selector_side_offset)))
                .when(!layout.selector_below, |el| {
                    el.top(px(layout.selector_top_when_above))
                })
                .when(layout.selector_below, |el| {
                    el.bottom(px(layout.selector_bottom_when_below))
                })
                .block_mouse_except_scroll()
                .on_hover(cx.listener(move |this, hovered, _, cx| {
                    on_hover_zone(this, *hovered, cx);
                }))
                .child(card)
        }
    };

    deferred(zone).with_priority(30).into_any_element()
}

pub fn render_hover_reaction_pill<V: 'static>(
    msg_id: u64,
    is_outgoing: bool,
    palette: &TdesktopThemePalette,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, ContextMenuAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let layout = ReactionOverlayLayout::compute(
        false,
        240.0,
        280.0,
        600.0,
        240.0,
        720.0,
        is_outgoing,
        !is_outgoing,
    );
    render_hover_reaction_overlay(
        msg_id,
        is_outgoing,
        layout,
        ReactionOverlayMode::QuickStrip,
        &[],
        palette,
        "en",
        cx,
        |_, _, _| {},
        |_, _, _| {},
        |_, _| {},
        move |this, emoji, cx| {
            on_action(this, ContextMenuAction::React(msg_id, emoji), cx);
        },
    )
}

#[allow(clippy::too_many_arguments)]
pub fn render_message_context_menu<V: 'static>(
    msg_id: u64,
    pos: Point<Pixels>,
    stage_size: Option<Size<Pixels>>,
    is_outgoing: bool,
    is_pinned: bool,
    can_edit: bool,
    is_translated: bool,
    reactions_expanded: bool,
    chosen_emojis: &[String],
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, ContextMenuAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let act_bg_left = on_action.clone();
    let act_bg_right = on_action.clone();
    let act_react = on_action.clone();
    let menu_w = if reactions_expanded { 286.0 } else { 248.0 };
    let est_menu_h = if reactions_expanded { 468.0 } else { 320.0 };
    let stage_w = stage_size
        .map(|s| f32::from(s.width))
        .filter(|&w| w > 10.0)
        .unwrap_or(720.0);
    let stage_h = stage_size
        .map(|s| f32::from(s.height))
        .filter(|&h| h > 10.0)
        .unwrap_or(640.0);
    let max_x = (stage_w - menu_w - 8.0).max(8.0);
    let max_y = (stage_h - est_menu_h - 8.0).max(8.0);
    let menu_x = f32::from(pos.x).clamp(8.0, max_x);
    let menu_y = f32::from(pos.y).clamp(8.0, max_y);
    let chosen_bg = TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.22);
    let hover_bg = rgb(palette.menu_hover_bg);

    let mut menu_box = div()
        .id("msg-context-menu-popup")
        .absolute()
        .left(px(menu_x))
        .top(px(menu_y))
        .w(px(menu_w))
        .p(px(6.0))
        .rounded(px(10.0))
        .bg(rgb(palette.menu_bg))
        .border_1()
        .border_color(rgb(palette.border_fg))
        .shadow_lg()
        .flex()
        .flex_col()
        .gap(px(2.0))
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
        .on_mouse_down(MouseButton::Right, |_, _, cx| cx.stop_propagation());

    // Quick-reactions top strip + expand toggle (`▾`)
    let mut rx_row = div()
        .flex()
        .items_center()
        .justify_between()
        .px(px(2.0))
        .py(px(4.0))
        .when(!reactions_expanded, |el| {
            el.mb(px(4.0))
                .border_b_1()
                .border_color(rgb(palette.border_fg))
        });

    for (idx, &emoji) in QUICK_REACTIONS.iter().enumerate() {
        let em = emoji.to_string();
        let is_chosen = chosen_emojis.iter().any(|c| c == emoji);
        let a = act_react.clone();
        rx_row = rx_row.child(
            div()
                .id(SharedString::from(format!("ctx-rx-{}-{}", msg_id, idx)))
                .w(px(26.0))
                .h(px(26.0))
                .rounded_full()
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .when(is_chosen, |el| {
                    el.bg(chosen_bg)
                        .border_1()
                        .border_color(rgb(palette.accent))
                })
                .hover(move |s| s.bg(hover_bg))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _, _, cx| {
                        cx.stop_propagation();
                        a(this, ContextMenuAction::React(msg_id, em.clone()), cx);
                    }),
                )
                .text_size(px(15.0))
                .child(emoji),
        );
    }

    let act_toggle_exp = on_action.clone();
    rx_row = rx_row.child(
        div()
            .id(SharedString::from(format!("ctx-rx-expand-{}", msg_id)))
            .w(px(22.0))
            .h(px(22.0))
            .rounded_full()
            .flex_shrink_0()
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .bg(TdesktopThemePalette::rgba_with_alpha(palette.border_fg, 0.35))
            .hover(move |s| s.bg(hover_bg))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, _, cx| {
                    cx.stop_propagation();
                    act_toggle_exp(this, ContextMenuAction::ToggleExpandReactions, cx);
                }),
            )
            .child(tg_icon(
                if reactions_expanded {
                    IconName::Close
                } else {
                    IconName::ChevronDown
                },
                11.0,
                palette.icon_fg,
            )),
    );

    menu_box = menu_box.child(rx_row);

    if reactions_expanded {
        let mut grid = div()
            .w_full()
            .px(px(2.0))
            .pb(px(6.0))
            .mb(px(4.0))
            .border_b_1()
            .border_color(rgb(palette.border_fg))
            .flex()
            .flex_wrap()
            .gap(px(4.0));

        for (idx, &emoji) in EXTENDED_REACTIONS.iter().enumerate() {
            let em = emoji.to_string();
            let is_chosen = chosen_emojis.iter().any(|c| c == emoji);
            let a = on_action.clone();
            grid = grid.child(
                div()
                    .id(SharedString::from(format!("ctx-rx-grid-{}-{}", msg_id, idx)))
                    .w(px(30.0))
                    .h(px(30.0))
                    .rounded(px(8.0))
                    .flex_shrink_0()
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .when(is_chosen, |el| {
                        el.bg(chosen_bg)
                            .border_1()
                            .border_color(rgb(palette.accent))
                    })
                    .hover(move |s| s.bg(hover_bg).text_size(px(18.0)))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            cx.stop_propagation();
                            a(this, ContextMenuAction::React(msg_id, em.clone()), cx);
                        }),
                    )
                    .text_size(px(16.0))
                    .child(emoji),
            );
        }
        menu_box = menu_box.child(grid);
    }

    menu_box = menu_box
        .child(ctx_menu_item("ctx-reply", IconName::Reply, tr_key_or(lang_id, "lng_context_reply_msg", "Reply"), false, palette, cx, {
            let a = on_action.clone();
            move |this, cx| a(this, ContextMenuAction::Reply(msg_id), cx)
        }))
        .when(is_outgoing && can_edit, |el| {
            let a = on_action.clone();
            el.child(ctx_menu_item("ctx-edit", IconName::Edit, tr_key_or(lang_id, "lng_context_edit_msg", "Edit"), false, palette, cx, move |this, cx| {
                a(this, ContextMenuAction::Edit(msg_id), cx)
            }))
        })
        .child(ctx_menu_item(
            "ctx-pin",
            IconName::Pin,
            if is_pinned {
                tr_key_or(lang_id, "lng_context_unpin_msg", "Unpin")
            } else {
                tr_key_or(lang_id, "lng_context_pin_msg", "Pin")
            },
            false,
            palette,
            cx,
            {
                let a = on_action.clone();
                move |this, cx| a(this, ContextMenuAction::TogglePin(msg_id), cx)
            },
        ))
        .child(ctx_menu_item("ctx-copy", IconName::Copy, tr_key_or(lang_id, "lng_context_copy_text", "Copy Text"), false, palette, cx, {
            let a = on_action.clone();
            move |this, cx| a(this, ContextMenuAction::CopyText(msg_id), cx)
        }))
        .child(ctx_menu_item("ctx-forward", IconName::Forward, tr_key_or(lang_id, "lng_context_forward_msg", "Forward"), false, palette, cx, {
            let a = on_action.clone();
            move |this, cx| a(this, ContextMenuAction::Forward(msg_id), cx)
        }))
        .child(ctx_menu_item(
            "ctx-translate",
            IconName::Translate,
            if is_translated {
                crate::i18n::tr_str(lang_id, "显示原文", "Show Original")
            } else {
                tr_key_or(lang_id, "lng_context_translate", "Translate")
            },
            false,
            palette,
            cx,
            {
                let a = on_action.clone();
                move |this, cx| a(this, ContextMenuAction::Translate(msg_id), cx)
            },
        ))
        .child(ctx_menu_item("ctx-select", IconName::SelectCheck, tr_key_or(lang_id, "lng_context_select_msg", "Select"), false, palette, cx, {
            let a = on_action.clone();
            move |this, cx| a(this, ContextMenuAction::Select(msg_id), cx)
        }))
        .child(div().my(px(2.0)).h(px(1.0)).w_full().bg(rgb(palette.border_fg)))
        .child(ctx_menu_item("ctx-delete", IconName::DeleteTrash, tr_key_or(lang_id, "lng_context_delete_msg", "Delete"), true, palette, cx, {
            let a = on_action.clone();
            move |this, cx| a(this, ContextMenuAction::Delete(msg_id), cx)
        }));

    div()
        .id("msg-ctx-backdrop")
        .absolute()
        .inset_0()
        .block_mouse_except_scroll()
        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
            act_bg_left(this, ContextMenuAction::CloseMenu, cx);
        }))
        .on_mouse_down(MouseButton::Right, cx.listener(move |this, _, _, cx| {
            act_bg_right(this, ContextMenuAction::CloseMenu, cx);
        }))
        .child(menu_box)
        .into_any_element()
}

fn ctx_menu_item<V: 'static>(
    id: &'static str,
    icon: IconName,
    label: &str,
    danger: bool,
    palette: &TdesktopThemePalette,
    cx: &mut Context<V>,
    on_click: impl Fn(&mut V, &mut Context<V>) + 'static,
) -> Stateful<Div> {
    let fg = if danger { palette.danger } else { palette.text_primary };
    let icon_fg = if danger { palette.danger } else { palette.icon_fg };
    let hover_bg = palette.menu_hover_bg;
    div()
        .id(id)
        .px(px(10.0))
        .py(px(6.0))
        .rounded(px(6.0))
        .flex()
        .items_center()
        .gap(px(10.0))
        .cursor_pointer()
        .hover(move |s| s.bg(rgb(hover_bg)))
        .on_click(cx.listener(move |this, _, _, cx| on_click(this, cx)))
        .child(tg_icon(icon, 15.0, icon_fg))
        .child(div().text_size(px(13.0)).text_color(rgb(fg)).child(label.to_string()))
}
