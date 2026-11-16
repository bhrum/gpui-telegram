//! Upstream `.tdesktop-theme` color specifications for Night, Tinted, Day, and Classic.

use super::palette_schemes::{
    TdesktopThemeMode, PEER_AVATAR_BOTTOM, PEER_AVATAR_TOP, PEER_NAME_COLORS_DAY,
    PEER_NAME_COLORS_NIGHT, PEER_NAME_COLORS_TINTED, adjust_rgb, mix_rgb,
};
use super::tdesktop_palette::TdesktopThemePalette;

struct ThemeSpec {
    window_bg: u32,
    rail_bg: u32,
    dialogs_bg: u32,
    dialogs_hover_bg: u32,
    dialogs_active_bg: u32,
    stage_bg: u32,
    topbar_bg: u32,
    composer_bg: u32,
    input_bg: u32,
    card_bg: u32,
    modal_bg: u32,
    tooltip_bg: u32,
    tooltip_fg: u32,
    menu_bg: u32,
    menu_hover_bg: u32,
    scrollbar_bg: u32,
    scrollbar_hover_bg: u32,
    msg_in_bg: u32,
    msg_out_bg: u32,
    msg_in_selected_bg: u32,
    msg_out_selected_bg: u32,
    msg_in_date_fg: u32,
    msg_out_date_fg: u32,
    code_bg: u32,
    code_fg: u32,
    service_pill_bg: u32,
    service_pill_fg: u32,
    muted_badge_bg: u32,
    online_dot_fg: u32,
    check_read_fg: u32,
    check_sent_fg: u32,
    icon_fg: u32,
    border_fg: u32,
    text_primary: u32,
    text_secondary: u32,
    text_link: u32,
    reaction_bg: u32,
    danger: u32,
    success: u32,
    wallpaper_top: u32,
    wallpaper_bottom: u32,
    wallpaper_pattern: u32,
    peer_name_colors: [u32; 7],
}

const NIGHT_SPEC: ThemeSpec = ThemeSpec {
    window_bg: 0x0B0B0B,
    rail_bg: 0x111111,
    dialogs_bg: 0x111111,
    dialogs_hover_bg: 0x1C1C1C,
    dialogs_active_bg: 0x2A2A2A,
    stage_bg: 0x070707,
    topbar_bg: 0x0B0B0B,
    composer_bg: 0x070707,
    input_bg: 0x1A1A1A,
    card_bg: 0x161616,
    modal_bg: 0x141414,
    tooltip_bg: 0x222222,
    tooltip_fg: 0xF5F5F5,
    menu_bg: 0x181818,
    menu_hover_bg: 0x252525,
    scrollbar_bg: 0x525252,
    scrollbar_hover_bg: 0x737373,
    msg_in_bg: 0x1A1A1A,
    msg_out_bg: 0x2C2C2C,
    msg_in_selected_bg: 0x262626,
    msg_out_selected_bg: 0x383838,
    msg_in_date_fg: 0x7A7A7A,
    msg_out_date_fg: 0x9A9A9A,
    code_bg: 0x1A1D19,
    code_fg: 0xD9DED4,
    service_pill_bg: 0x1A1A1A,
    service_pill_fg: 0x9A9A9A,
    muted_badge_bg: 0x3A3A3A,
    online_dot_fg: 0x00C972,
    check_read_fg: 0x459FFE,
    check_sent_fg: 0x8E8E8E,
    icon_fg: 0x8A8A8A,
    border_fg: 0x222222,
    text_primary: 0xFAFAFA,
    text_secondary: 0x949494,
    text_link: 0x459FFE,
    reaction_bg: 0x242424,
    danger: 0xFF5667,
    success: 0x00C972,
    wallpaper_top: 0x070707,
    wallpaper_bottom: 0x0A0A0A,
    wallpaper_pattern: 0x1A1A1A,
    peer_name_colors: PEER_NAME_COLORS_NIGHT,
};

const TINTED_SPEC: ThemeSpec = ThemeSpec {
    window_bg: 0x101215,
    rail_bg: 0x15181C,
    dialogs_bg: 0x15181C,
    dialogs_hover_bg: 0x1F2329,
    dialogs_active_bg: 0x2A3038,
    stage_bg: 0x0E1013,
    topbar_bg: 0x121519,
    composer_bg: 0x0E1013,
    input_bg: 0x1B1F25,
    card_bg: 0x191D23,
    modal_bg: 0x161A20,
    tooltip_bg: 0x242A32,
    tooltip_fg: 0xF5F7FA,
    menu_bg: 0x181C22,
    menu_hover_bg: 0x242A32,
    scrollbar_bg: 0x5A626D,
    scrollbar_hover_bg: 0x7A8492,
    msg_in_bg: 0x1A1E24,
    msg_out_bg: 0x273240,
    msg_in_selected_bg: 0x242A33,
    msg_out_selected_bg: 0x324052,
    msg_in_date_fg: 0x7E8792,
    msg_out_date_fg: 0x98A7B8,
    code_bg: 0x1A1D19,
    code_fg: 0xD9DED4,
    service_pill_bg: 0x1C2026,
    service_pill_fg: 0x9BA3AE,
    muted_badge_bg: 0x424A54,
    online_dot_fg: 0x00C972,
    check_read_fg: 0x459FFE,
    check_sent_fg: 0x8E98A4,
    icon_fg: 0x88919C,
    border_fg: 0x242930,
    text_primary: 0xF5F7FA,
    text_secondary: 0x8E96A0,
    text_link: 0x459FFE,
    reaction_bg: 0x242A32,
    danger: 0xFF5667,
    success: 0x00C972,
    wallpaper_top: 0x0E1013,
    wallpaper_bottom: 0x13161B,
    wallpaper_pattern: 0x22262E,
    peer_name_colors: PEER_NAME_COLORS_TINTED,
};

const DAY_SPEC: ThemeSpec = ThemeSpec {
    window_bg: 0xFCFCFC,
    rail_bg: 0xF7F7F7,
    dialogs_bg: 0xF7F7F7,
    dialogs_hover_bg: 0xEDEDED,
    dialogs_active_bg: 0xE4E4E4,
    stage_bg: 0xFCFCFC,
    topbar_bg: 0xFCFCFC,
    composer_bg: 0xFCFCFC,
    input_bg: 0xF1F1F1,
    card_bg: 0xF5F5F5,
    modal_bg: 0xFCFCFC,
    tooltip_bg: 0x1F1F1F,
    tooltip_fg: 0xFFFFFF,
    menu_bg: 0xFFFFFF,
    menu_hover_bg: 0xF0F0F0,
    scrollbar_bg: 0xB0B0B0,
    scrollbar_hover_bg: 0x888888,
    msg_in_bg: 0xF3F3F3,
    msg_out_bg: 0xE7EDF5,
    msg_in_selected_bg: 0xE5E5E5,
    msg_out_selected_bg: 0xD8E3F0,
    msg_in_date_fg: 0x8A8A8A,
    msg_out_date_fg: 0x747E8A,
    code_bg: 0xE6E8EB,
    code_fg: 0x1F2328,
    service_pill_bg: 0xEFEFEF,
    service_pill_fg: 0x6E6E6E,
    muted_badge_bg: 0xC4C4C4,
    online_dot_fg: 0x10B981,
    check_read_fg: 0x1084FE,
    check_sent_fg: 0x8A8A8A,
    icon_fg: 0x767676,
    border_fg: 0xE8E8E8,
    text_primary: 0x141414,
    text_secondary: 0x6E6E6E,
    text_link: 0x0C64C1,
    reaction_bg: 0xEAEAEA,
    danger: 0xE02E2A,
    success: 0x10B981,
    wallpaper_top: 0xFCFCFC,
    wallpaper_bottom: 0xFAFAFA,
    wallpaper_pattern: 0xECECEC,
    peer_name_colors: PEER_NAME_COLORS_DAY,
};

const CLASSIC_SPEC: ThemeSpec = ThemeSpec {
    window_bg: 0xFFFFFF,
    rail_bg: 0xEDEDED,
    dialogs_bg: 0xFFFFFF,
    dialogs_hover_bg: 0xF1F1F1,
    dialogs_active_bg: 0x419FD9,
    stage_bg: 0xD5E8C8,
    topbar_bg: 0xFFFFFF,
    composer_bg: 0xFFFFFF,
    input_bg: 0xF1F1F1,
    card_bg: 0xF6F8F5,
    modal_bg: 0xFFFFFF,
    tooltip_bg: 0xEEF2F5,
    tooltip_fg: 0x5D6C80,
    menu_bg: 0xFFFFFF,
    menu_hover_bg: 0xF1F1F1,
    scrollbar_bg: 0x8B998E,
    scrollbar_hover_bg: 0x6E7D72,
    msg_in_bg: 0xFFFFFF,
    msg_out_bg: 0xEFFDDE,
    msg_in_selected_bg: 0xE8F3E5,
    msg_out_selected_bg: 0xDAEECC,
    msg_in_date_fg: 0xA0ACB6,
    msg_out_date_fg: 0x6FAB69,
    code_bg: 0xEFF5EC,
    code_fg: 0x459866,
    service_pill_bg: 0x527C41,
    service_pill_fg: 0xFFFFFF,
    muted_badge_bg: 0xBBBBBB,
    online_dot_fg: 0x4DC920,
    check_read_fg: 0x4B9432,
    check_sent_fg: 0x6FAB69,
    icon_fg: 0x8E97A0,
    border_fg: 0xE4E8E2,
    text_primary: 0x000000,
    text_secondary: 0x707579,
    text_link: 0x168ACD,
    reaction_bg: 0xEAF4E4,
    danger: 0xE53935,
    success: 0x4B9432,
    wallpaper_top: 0xD5E8C8,
    wallpaper_bottom: 0xB8D8A3,
    wallpaper_pattern: 0x93BD7E,
    peer_name_colors: PEER_NAME_COLORS_DAY,
};

pub(super) fn preset_palette(
    mode: TdesktopThemeMode,
    accent: u32,
    font_size_px: u32,
    bubble_radius_px: u32,
) -> TdesktopThemePalette {
    let spec = match mode {
        TdesktopThemeMode::Night => &NIGHT_SPEC,
        TdesktopThemeMode::Tinted => &TINTED_SPEC,
        TdesktopThemeMode::Day => &DAY_SPEC,
        TdesktopThemeMode::Classic => &CLASSIC_SPEC,
    };
    let eff = if accent == 0 { mode.default_accent() } else { accent };
    let is_dark = mode.is_dark();
    let is_fabushi = mode.is_fabushi_style();
    let default_family = match mode {
        TdesktopThemeMode::Night => matches!(eff, 0x1084FE | 0x1687FF | 0x2AABEE | 0x40A7E3 | 0x5288C1 | 0x3390EC),
        TdesktopThemeMode::Tinted => matches!(eff, 0x1687FF | 0x1084FE | 0x1ABC9C | 0x3FC1B0 | 0x009687),
        TdesktopThemeMode::Day => matches!(eff, 0x1084FE | 0x1687FF | 0x2AABEE | 0x40A7E3 | 0x419FD9 | 0x2481CC | 0x5288C1),
        TdesktopThemeMode::Classic => matches!(eff, 0x2AABEE | 0x40A7E3 | 0x419FD9),
    };
    let dialogs_active_bg = if is_fabushi {
        if default_family {
            spec.dialogs_active_bg
        } else {
            mix_rgb(spec.dialogs_active_bg, eff, 0.14)
        }
    } else if default_family {
        spec.dialogs_active_bg
    } else if is_dark {
        mix_rgb(spec.dialogs_bg, eff, 0.44)
    } else {
        eff
    };
    let msg_out_bg = if mode == TdesktopThemeMode::Classic || default_family {
        spec.msg_out_bg
    } else if is_fabushi {
        if is_dark {
            mix_rgb(spec.msg_out_bg, eff, 0.18)
        } else {
            mix_rgb(spec.msg_out_bg, eff, 0.12)
        }
    } else if is_dark {
        mix_rgb(spec.msg_in_bg, eff, 0.36)
    } else {
        mix_rgb(0xFFFFFF, eff, 0.16)
    };
    let msg_out_selected_bg = if mode == TdesktopThemeMode::Classic || default_family {
        spec.msg_out_selected_bg
    } else if is_fabushi {
        if is_dark {
            mix_rgb(spec.msg_out_selected_bg, eff, 0.24)
        } else {
            mix_rgb(spec.msg_out_selected_bg, eff, 0.18)
        }
    } else if is_dark {
        mix_rgb(spec.msg_in_bg, eff, 0.48)
    } else {
        mix_rgb(0xFFFFFF, eff, 0.28)
    };
    let text_link = if mode == TdesktopThemeMode::Classic || default_family {
        spec.text_link
    } else if is_dark {
        mix_rgb(0xFFFFFF, eff, 0.72)
    } else {
        adjust_rgb(eff, -22)
    };
    let check_read_fg = if mode == TdesktopThemeMode::Classic || default_family {
        spec.check_read_fg
    } else {
        eff
    };
    let accent_hover = if is_dark { adjust_rgb(eff, 16) } else { adjust_rgb(eff, -18) };
    TdesktopThemePalette {
        mode,
        window_bg: spec.window_bg,
        hover_bg: spec.dialogs_hover_bg,
        nav_rail_bg: spec.rail_bg,
        rail_bg: spec.rail_bg,
        dialogs_bg: spec.dialogs_bg,
        dialogs_hover_bg: spec.dialogs_hover_bg,
        dialogs_active_bg,
        history_bg: spec.stage_bg,
        stage_bg: spec.stage_bg,
        top_bar_bg: spec.topbar_bg,
        topbar_bg: spec.topbar_bg,
        compose_bg: spec.composer_bg,
        composer_bg: spec.composer_bg,
        input_bg: spec.input_bg,
        card_bg: spec.card_bg,
        modal_bg: spec.modal_bg,
        tooltip_bg: spec.tooltip_bg,
        tooltip_fg: spec.tooltip_fg,
        menu_bg: spec.menu_bg,
        menu_hover_bg: spec.menu_hover_bg,
        scrollbar_bg: spec.scrollbar_bg,
        scrollbar_hover_bg: spec.scrollbar_hover_bg,
        msg_in_bg: spec.msg_in_bg,
        msg_out_bg,
        msg_in_selected_bg: spec.msg_in_selected_bg,
        msg_out_selected_bg,
        msg_in_date_fg: spec.msg_in_date_fg,
        msg_out_date_fg: spec.msg_out_date_fg,
        reply_bar_fg: eff,
        reply_bar_bg: eff,
        code_bg: spec.code_bg,
        code_fg: spec.code_fg,
        service_pill_bg: spec.service_pill_bg,
        service_pill_fg: spec.service_pill_fg,
        unread_badge_bg: eff,
        unread_badge_fg: 0xFFFFFF,
        muted_badge_bg: spec.muted_badge_bg,
        online_dot_fg: spec.online_dot_fg,
        check_read_fg,
        check_sent_fg: spec.check_sent_fg,
        icon_fg: spec.icon_fg,
        icon_active: eff,
        border_fg: spec.border_fg,
        shadow_fg: 0x000000,
        text_primary: spec.text_primary,
        text_secondary: spec.text_secondary,
        text_link,
        accent: eff,
        accent_hover,
        active_button_fg: 0xFFFFFF,
        reaction_bg: spec.reaction_bg,
        reaction_active_bg: eff,
        danger: spec.danger,
        success: spec.success,
        wallpaper_top: spec.wallpaper_top,
        wallpaper_bottom: spec.wallpaper_bottom,
        wallpaper_pattern: spec.wallpaper_pattern,
        peer_name_colors: spec.peer_name_colors,
        peer_avatar_colors: PEER_AVATAR_TOP,
        peer_avatar_colors_bottom: PEER_AVATAR_BOTTOM,
        font_size_px: font_size_px.clamp(12, 20),
        bubble_radius_px: bubble_radius_px.clamp(6, 18),
        ui_scale_pct: 100,
        blur_wallpaper: false,
    }
}
