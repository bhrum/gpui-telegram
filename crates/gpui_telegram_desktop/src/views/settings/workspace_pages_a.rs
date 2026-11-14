use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::theme::palette::{ACCENT_SWATCHES, TdesktopThemeMode, TdesktopThemePalette, WALLPAPER_PRESETS};
use crate::views::settings::settings_theme_ui::{
    settings_action_row, settings_card, settings_info_row, settings_section_header,
    settings_subsection_title, settings_toggle_row,
};
use crate::views::settings_workspace::{SettingsMutation, SettingsWorkspaceView};
use gpui::prelude::FluentBuilder;
use gpui::*;

pub fn render_account_page(
    view: &SettingsWorkspaceView,
    pal: TdesktopThemePalette,
    lang: &str,
    cx: &mut Context<SettingsWorkspaceView>,
) -> AnyElement {
    let active = view
        .state_snapshot
        .accounts
        .iter()
        .find(|a| a.account_id == view.state_snapshot.active_account_id)
        .or_else(|| view.state_snapshot.accounts.first());

    let (name, phone, uname) = match active {
        Some(a) => (a.display_name.clone(), a.phone.clone(), format!("@{}", a.username)),
        None => (tr_str(lang, "未登录账号", "Not signed in").to_string(), String::new(), String::new()),
    };

    div().flex().flex_col().gap(px(12.0))
        .child(settings_card(pal)
            .child(settings_section_header(IconName::ContactsUsers, &name, Some(&format!("{}  {}", phone, uname)), pal))
            .child(settings_subsection_title(tr_str(lang, "个人简介 (点击编辑，回车保存)", "Bio (click to edit, Enter to save)"), pal))
            .child(div().cursor_pointer().p(px(10.0)).rounded(px(8.0)).bg(rgb(pal.input_bg))
                .border_1().border_color(rgb(if view.editing_field == 3 { pal.accent } else { pal.border_fg }))
                .text_size(px(13.0)).text_color(rgb(pal.text_primary))
                .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| { this.editing_field = 3; cx.notify(); }))
                .child(if view.bio_edit_input.is_empty() { tr_str(lang, "输入个人简介...", "Add a few words about yourself...").to_string() } else { view.bio_edit_input.clone() }))
            .child(settings_action_row(IconName::Close, tr_str(lang, "退出登录", "Log Out"), "", pal, true, |this| {
                this.pending_mutation = Some(SettingsMutation::LogOut);
            }, cx)))
        .into_any_element()
}

pub fn render_notifications_page(
    view: &SettingsWorkspaceView,
    pal: TdesktopThemePalette,
    lang: &str,
    cx: &mut Context<SettingsWorkspaceView>,
) -> AnyElement {
    div().flex().flex_col().gap(px(12.0))
        .child(settings_card(pal)
            .child(settings_section_header(IconName::Speaker, tr_str(lang, "通知与声音", "Notifications and Sounds"), None, pal))
            .child(settings_toggle_row(IconName::ContactsUsers, tr_str(lang, "私聊消息通知", "Private Chats"), None, view.notify_private, pal, |this| {
                this.notify_private = !this.notify_private;
            }, cx))
            .child(settings_toggle_row(IconName::GroupUsers, tr_str(lang, "群组消息通知", "Groups"), None, view.notify_groups, pal, |this| {
                this.notify_groups = !this.notify_groups;
            }, cx))
            .child(settings_toggle_row(IconName::ChannelMegaphone, tr_str(lang, "频道通知", "Channels"), None, view.notify_channels, pal, |this| {
                this.notify_channels = !this.notify_channels;
            }, cx))
            .child(settings_toggle_row(IconName::Speaker, tr_str(lang, "提示音", "Play Sound"), None, view.notify_sound, pal, |this| {
                this.notify_sound = !this.notify_sound;
            }, cx)))
        .into_any_element()
}

pub fn render_privacy_page(
    view: &SettingsWorkspaceView,
    pal: TdesktopThemePalette,
    lang: &str,
    cx: &mut Context<SettingsWorkspaceView>,
) -> AnyElement {
    let passcode_on = view.state_snapshot.local_passcode_pin.is_some();
    let devices = view.state_snapshot.active_devices.clone();

    div().flex().flex_col().gap(px(12.0))
        .child(settings_card(pal)
            .child(settings_section_header(IconName::Lock, tr_str(lang, "隐私与安全", "Privacy and Security"), None, pal))
            .child(settings_action_row(IconName::Lock, tr_str(lang, "本地锁定密码", "Local Passcode"), if passcode_on { "On" } else { "Off" }, pal, false, |this| {
                this.pending_mutation = Some(SettingsMutation::OpenPasscodeModal);
            }, cx))
            .when(passcode_on, |el| {
                el.child(settings_action_row(IconName::Lock, tr_str(lang, "立即锁定应用", "Lock Now"), "", pal, false, |this| {
                    this.pending_mutation = Some(SettingsMutation::LockNow);
                }, cx))
            })
            .child(settings_info_row(tr_str(lang, "已屏蔽用户", "Blocked Users"), &format!("{}", view.state_snapshot.blocked_peers.len()), pal))
            .child(settings_info_row(tr_str(lang, "全局自动删除", "Auto-Delete Messages"), &view.state_snapshot.auto_delete_ttl_label, pal)))
        .child(settings_card(pal)
            .child(settings_subsection_title(tr_str(lang, "活跃设备与会话", "Active Sessions"), pal))
            .children(devices.into_iter().map(|d| {
                let sid = d.session_id.clone();
                let is_cur = d.is_current;
                settings_action_row(IconName::ComputerDesktop, &d.device_model, if is_cur { "Current" } else { "Terminate" }, pal, !is_cur, move |this| {
                    if !is_cur {
                        this.pending_mutation = Some(SettingsMutation::RevokeSession(sid.clone()));
                    }
                }, cx)
            })))
        .into_any_element()
}

pub fn render_chat_appearance_page(
    view: &SettingsWorkspaceView,
    pal: TdesktopThemePalette,
    lang: &str,
    cx: &mut Context<SettingsWorkspaceView>,
) -> AnyElement {
    let modes = [TdesktopThemeMode::Night, TdesktopThemeMode::Tinted, TdesktopThemeMode::Day, TdesktopThemeMode::Classic];
    let cur_mode = pal.mode;

    div().flex().flex_col().gap(px(12.0))
        .child(settings_card(pal)
            .child(settings_section_header(IconName::NightMoon, tr_str(lang, "聊天外观与主题", "Chat Settings"), None, pal))
            .child(div().flex().gap(px(8.0)).children(modes.into_iter().map(|m| {
                let active = cur_mode == m;
                div().cursor_pointer().flex_1().py(px(8.0)).rounded(px(8.0)).bg(rgb(if active { pal.accent } else { pal.input_bg }))
                    .border_1().border_color(rgb(if active { pal.accent } else { pal.border_fg }))
                    .flex().items_center().justify_center().text_size(px(12.5)).font_weight(FontWeight::BOLD)
                    .text_color(rgb(if active { pal.active_button_fg } else { pal.text_primary }))
                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _e, _w, cx| {
                        this.pending_mutation = Some(SettingsMutation::SetThemeMode(m));
                        cx.notify();
                    }))
                    .child(m.as_name())
            })))
            .child(settings_subsection_title(tr_str(lang, "主题强调色", "Accent Color"), pal))
            .child(div().flex().flex_wrap().gap(px(8.0)).children(ACCENT_SWATCHES.iter().map(|sw| {
                let hex = sw.hex;
                let active = pal.accent == hex;
                div().cursor_pointer().w(px(28.0)).h(px(28.0)).rounded_full().bg(rgb(hex))
                    .border_2().border_color(rgb(if active { pal.text_primary } else { pal.border_fg }))
                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _e, _w, cx| {
                        this.pending_mutation = Some(SettingsMutation::SetAccentColor(hex));
                        cx.notify();
                    }))
            })))
            .child(settings_subsection_title(tr_str(lang, "聊天背景", "Chat Background"), pal))
            .child(div().flex().flex_wrap().gap(px(8.0)).children(WALLPAPER_PRESETS.iter().map(|wp| {
                let id = wp.id.to_string();
                let active = view.selected_wallpaper_id == wp.id;
                div().cursor_pointer().w(px(72.0)).h(px(46.0)).rounded(px(8.0)).bg(rgb(wp.top_hex))
                    .border_2().border_color(rgb(if active { pal.accent } else { pal.border_fg }))
                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _e, _w, cx| {
                        this.selected_wallpaper_id = id.clone();
                        this.pending_mutation = Some(SettingsMutation::SetWallpaper(id.clone()));
                        cx.notify();
                    }))
            })))
            .child(div().flex().items_center().justify_between().gap(px(8.0))
                .child(div().flex().gap(px(6.0)).children([12u32, 14, 16, 18].into_iter().map(|sz| {
                    let act = pal.font_size_px == sz;
                    div().cursor_pointer().px(px(8.0)).py(px(4.0)).rounded(px(6.0)).bg(rgb(if act { pal.accent } else { pal.input_bg }))
                        .text_size(px(12.0)).text_color(rgb(if act { pal.active_button_fg } else { pal.text_primary }))
                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _e, _w, cx| {
                            this.pending_mutation = Some(SettingsMutation::SetFontSize(sz));
                            cx.notify();
                        }))
                        .child(format!("{}px", sz))
                })))
                .child(div().flex().gap(px(6.0)).children([6u32, 10, 14, 18].into_iter().map(|r| {
                    let act = pal.bubble_radius_px == r;
                    div().cursor_pointer().px(px(8.0)).py(px(4.0)).rounded(px(6.0)).bg(rgb(if act { pal.accent } else { pal.input_bg }))
                        .text_size(px(12.0)).text_color(rgb(if act { pal.active_button_fg } else { pal.text_primary }))
                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _e, _w, cx| {
                            this.pending_mutation = Some(SettingsMutation::SetBubbleRadius(r));
                            cx.notify();
                        }))
                        .child(format!("R{}", r))
                }))))
            .child(settings_toggle_row(IconName::SendPlane, tr_str(lang, "按 Enter 发送消息", "Send with Enter"), None, view.state_snapshot.send_with_enter, pal, |this| {
                this.pending_mutation = Some(SettingsMutation::ToggleSendWithEnter);
            }, cx)))
        .into_any_element()
}
