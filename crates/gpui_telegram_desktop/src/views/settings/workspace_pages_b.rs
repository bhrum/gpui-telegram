use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::theme::palette::TdesktopThemePalette;
use crate::views::boxes::language_box::LanguageBox;
use crate::views::settings::settings_theme_ui::{
    settings_action_row, settings_card, settings_empty_state, settings_info_row,
    settings_section_header, settings_subsection_title, settings_toggle_row,
};
use crate::views::settings_workspace::{SettingsMutation, SettingsWorkspaceView};
use gpui::prelude::FluentBuilder;
use gpui::*;

pub fn render_folders_page(
    view: &SettingsWorkspaceView,
    pal: TdesktopThemePalette,
    lang: &str,
    cx: &mut Context<SettingsWorkspaceView>,
) -> AnyElement {
    let folders = view.state_snapshot.custom_folders.clone();
    div().flex().flex_col().gap(px(12.0))
        .child(settings_card(pal)
            .child(settings_section_header(IconName::FolderAll, tr_str(lang, "聊天分组", "Chat Folders"), None, pal))
            .map(|el| {
                if folders.is_empty() {
                    el.child(settings_empty_state(IconName::FolderAll, tr_str(lang, "暂无自定义分组", "No custom folders yet"), pal))
                } else {
                    el.children(folders.into_iter().map(|(fname, _)| {
                        let f_del = fname.clone();
                        settings_action_row(IconName::FolderChat, &fname, tr_str(lang, "删除", "Remove"), pal, true, move |this| {
                            this.pending_mutation = Some(SettingsMutation::RemoveCustomFolder(f_del.clone()));
                        }, cx)
                    }))
                }
            })
            .child(div().flex().gap(px(8.0))
                .child(div().cursor_pointer().flex_1().px(px(10.0)).py(px(8.0)).rounded(px(8.0)).bg(rgb(pal.input_bg))
                    .border_1().border_color(rgb(if view.editing_field == 2 { pal.accent } else { pal.border_fg }))
                    .text_size(px(13.0)).text_color(rgb(pal.text_primary))
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| { this.editing_field = 2; cx.notify(); }))
                    .child(if view.new_folder_input.is_empty() { tr_str(lang, "输入新分组名...", "New folder name...").to_string() } else { view.new_folder_input.clone() }))
                .child(div().cursor_pointer().px(px(14.0)).py(px(8.0)).rounded(px(8.0)).bg(rgb(pal.accent))
                    .text_size(px(13.0)).font_weight(FontWeight::BOLD).text_color(rgb(pal.active_button_fg))
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| {
                        let nm = this.new_folder_input.trim().to_string();
                        if !nm.is_empty() {
                            this.pending_mutation = Some(SettingsMutation::AddCustomFolder(nm));
                            this.new_folder_input.clear();
                        }
                        cx.notify();
                    }))
                    .child(tr_str(lang, "创建", "Create").to_string()))))
        .into_any_element()
}

pub fn render_advanced_storage_page(
    view: &SettingsWorkspaceView,
    pal: TdesktopThemePalette,
    lang: &str,
    cx: &mut Context<SettingsWorkspaceView>,
) -> AnyElement {
    let total_mb = view.state_snapshot.cache_media_mb + view.state_snapshot.cache_files_mb;
    div().flex().flex_col().gap(px(12.0))
        .child(settings_card(pal)
            .child(settings_section_header(IconName::SettingsGear, tr_str(lang, "高级与本地存储", "Advanced & Storage"), None, pal))
            .child(settings_info_row(tr_str(lang, "本地缓存占用", "Local Cache Size"), &format!("{:.1} MB", total_mb), pal))
            .child(settings_action_row(IconName::DeleteTrash, tr_str(lang, "清理本地缓存", "Clear Local Cache"), "", pal, true, |this| {
                this.pending_mutation = Some(SettingsMutation::ClearLocalCache);
            }, cx))
            .child(settings_action_row(IconName::SettingsGear, tr_str(lang, "连接与代理设置", "Connection Type / Proxy"), "", pal, false, |this| {
                this.pending_mutation = Some(SettingsMutation::OpenProxyModal);
            }, cx))
            .child(settings_action_row(IconName::FileDoc, tr_str(lang, "导出聊天数据", "Export Chat Data"), "", pal, false, |this| {
                this.pending_mutation = Some(SettingsMutation::OpenExportModal);
            }, cx)))
        .into_any_element()
}

pub fn render_calls_page(
    view: &SettingsWorkspaceView,
    pal: TdesktopThemePalette,
    lang: &str,
    cx: &mut Context<SettingsWorkspaceView>,
) -> AnyElement {
    div().flex().flex_col().gap(px(12.0))
        .child(settings_card(pal)
            .child(settings_section_header(IconName::PhoneCall, tr_str(lang, "通话与音视频设备", "Call Settings"), None, pal))
            .child(settings_info_row(tr_str(lang, "音频输入设备", "Input Device"), "Default Microphone", pal))
            .child(settings_info_row(tr_str(lang, "音频输出设备", "Output Device"), "Default Speakers", pal))
            .child(settings_toggle_row(IconName::Mic, tr_str(lang, "AI 背景降噪", "Noise Suppression"), None, view.call_noise_suppression, pal, |this| {
                this.call_noise_suppression = !this.call_noise_suppression;
            }, cx))
            .child(settings_toggle_row(IconName::Lock, tr_str(lang, "仅联系人使用 P2P 直连", "P2P Contacts Only"), None, view.call_p2p_contacts_only, pal, |this| {
                this.call_p2p_contacts_only = !this.call_p2p_contacts_only;
            }, cx)))
        .into_any_element()
}

pub fn render_language_page(
    view: &SettingsWorkspaceView,
    pal: TdesktopThemePalette,
    lang: &str,
    cx: &mut Context<SettingsWorkspaceView>,
) -> AnyElement {
    let langs = LanguageBox::default_languages();
    div().flex().flex_col().gap(px(12.0))
        .child(settings_card(pal)
            .child(settings_section_header(IconName::Translate, tr_str(lang, "多语言与翻译", "Language"), None, pal))
            .child(settings_toggle_row(IconName::Translate, tr_str(lang, "显示翻译按钮", "Show Translate Button"), None, view.state_snapshot.show_translate_bar, pal, |this| {
                this.pending_mutation = Some(SettingsMutation::ToggleTranslateBar);
            }, cx))
            .child(settings_subsection_title(tr_str(lang, "界面语言", "Interface Language"), pal))
            .children(langs.into_iter().map(|l| {
                let lid = l.id.clone();
                let active = view.state_snapshot.language_id == lid;
                settings_action_row(IconName::Translate, &l.native_name, if active { "✓" } else { &l.name }, pal, false, move |this| {
                    this.pending_mutation = Some(SettingsMutation::SetLanguage(lid.clone()));
                }, cx)
            })))
        .into_any_element()
}
