//! Inspector Drawer sub-panels: Members, Shared Media/Files/Links/Music/Voice, Settings & Statistics.

use crate::i18n::tr_str;
use crate::theme::assets::{IconName, tg_icon};
use crate::theme::palette::TdesktopThemePalette;
use crate::views::drawer::inspector_tabs::{
    InspectorAction, MembersTabModel, SettingsTabModel, SharedFileItem, SharedMediaSubTab,
    SharedMediaTabModel,
};
use crate::views::drawer::statistics::info_statistics_inner_widget::ChartDataPoint;
use crate::views::ContextInspector;
use gpui::prelude::FluentBuilder;
use gpui::*;

pub fn render_members_panel(
    model: &MembersTabModel,
    pal: TdesktopThemePalette,
    lang: &str,
    cx: &mut Context<ContextInspector>,
) -> AnyElement {
    let invite = model.invite_link.clone();
    div().flex().flex_col().gap(px(10.0))
        .child(div().flex().items_center().justify_between()
            .child(div().text_size(px(12.0)).font_weight(FontWeight::BOLD).text_color(rgb(pal.text_secondary))
                .child(format!("{} {}", model.members.len(), tr_str(lang, "位成员", "MEMBERS"))))
            .child(div().cursor_pointer().px(px(8.0)).py(px(4.0)).rounded(px(6.0)).bg(rgb(pal.card_bg))
                .hover(move |s| s.bg(rgb(pal.dialogs_hover_bg))).text_size(px(12.0)).font_weight(FontWeight::BOLD).text_color(rgb(pal.accent))
                .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| {
                    this.pending_action = Some(InspectorAction::OpenAddMember);
                    cx.notify();
                }))
                .child(tr_str(lang, "+ 添加成员", "+ Add Member").to_string())))
        .when(!invite.is_empty(), |el| {
            let cp = invite.clone();
            el.child(div().p(px(10.0)).rounded(px(8.0)).bg(rgb(pal.card_bg)).flex().items_center().justify_between()
                .child(div().flex().flex_col().child(div().text_size(px(11.0)).text_color(rgb(pal.text_secondary)).child(tr_str(lang, "邀请链接", "Invite Link")))
                    .child(div().text_size(px(12.0)).text_color(rgb(pal.text_link)).child(invite)))
                .child(div().cursor_pointer().on_mouse_down(MouseButton::Left, cx.listener(move |this, _e, _w, cx| {
                    this.pending_action = Some(InspectorAction::CopyText(cp.clone()));
                    cx.notify();
                })).child(tg_icon(IconName::Copy, 14.0, pal.icon_fg))))
        })
        .map(|el| {
            if model.members.is_empty() {
                el.child(empty_panel(IconName::GroupUsers, tr_str(lang, "暂无成员列表", "No members loaded"), pal))
            } else {
                el.children(model.members.iter().map(|m| {
                    let mid = m.id;
                    div().cursor_pointer().h(px(48.0)).px(px(8.0)).rounded(px(8.0)).bg(rgb(pal.card_bg)).hover(move |s| s.bg(rgb(pal.dialogs_hover_bg)))
                        .flex().items_center().justify_between()
                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _e, _w, cx| {
                            this.pending_action = Some(InspectorAction::CycleMemberRole(mid));
                            cx.notify();
                        }))
                        .child(div().flex().items_center().gap(px(10.0))
                            .child(div().w(px(32.0)).h(px(32.0)).rounded_full().bg(rgb(m.color)).flex().items_center().justify_center()
                                .text_size(px(12.0)).font_weight(FontWeight::BOLD).text_color(rgb(pal.active_button_fg)).child(m.initials.clone()))
                            .child(div().flex().flex_col().child(div().text_size(px(13.0)).text_color(rgb(pal.text_primary)).child(m.name.clone()))
                                .child(div().text_size(px(11.0)).text_color(if m.is_online { rgb(pal.online_dot_fg) } else { rgb(pal.text_secondary) })
                                    .child(if m.is_online { "online" } else { "offline" }))))
                        .child(div().px(px(6.0)).py(px(2.0)).rounded(px(4.0)).bg(rgb(pal.input_bg)).text_size(px(10.5)).text_color(rgb(pal.accent)).child(m.role.clone()))
                }))
            }
        })
        .into_any_element()
}

pub fn render_shared_media_panel(
    model: &SharedMediaTabModel,
    pal: TdesktopThemePalette,
    lang: &str,
    cx: &mut Context<ContextInspector>,
) -> AnyElement {
    let tabs = [
        (tr_str(lang, "媒体", "Media"), SharedMediaSubTab::Media),
        (tr_str(lang, "文件", "Files"), SharedMediaSubTab::Files),
        (tr_str(lang, "链接", "Links"), SharedMediaSubTab::Links),
        (tr_str(lang, "音乐", "Music"), SharedMediaSubTab::Music),
        (tr_str(lang, "语音", "Voice"), SharedMediaSubTab::Voice),
    ];

    let body = match model.active_subtab {
        SharedMediaSubTab::Media if model.media_items.is_empty() => empty_panel(IconName::Camera, tr_str(lang, "暂无共享媒体", "No shared media yet"), pal),
        SharedMediaSubTab::Media => div().flex().flex_wrap().gap(px(6.0)).children(model.media_items.iter().map(|item| {
            let url = item.url.clone();
            let is_vid = item.is_video;
            let thumb = item.local_path.as_ref().map(|p| img(p.clone()).w(px(88.0)).h(px(88.0)).object_fit(ObjectFit::Cover).into_any_element())
                .unwrap_or_else(|| tg_icon(if is_vid { IconName::VideoCall } else { IconName::Camera }, 24.0, pal.accent).into_any_element());
            div().cursor_pointer().w(px(88.0)).h(px(88.0)).rounded(px(8.0)).overflow_hidden().bg(rgb(pal.card_bg)).border_1().border_color(rgb(pal.border_fg))
                .flex().items_center().justify_center().on_mouse_down(MouseButton::Left, cx.listener(move |this, _e, _w, cx| {
                    this.pending_media_open = Some((url.clone(), is_vid));
                    this.pending_action = Some(InspectorAction::OpenMedia { url: url.clone(), is_video: is_vid });
                    cx.notify();
                })).child(thumb)
        })).into_any_element(),
        SharedMediaSubTab::Files if model.files.is_empty() => empty_panel(IconName::FileDoc, tr_str(lang, "暂无共享文件", "No shared files yet"), pal),
        SharedMediaSubTab::Files => render_file_list(&model.files, pal, cx),
        SharedMediaSubTab::Music if model.music_items.is_empty() => empty_panel(IconName::Speaker, tr_str(lang, "暂无共享音乐", "No shared music yet"), pal),
        SharedMediaSubTab::Music => render_file_list(&model.music_items, pal, cx),
        SharedMediaSubTab::Links if model.links.is_empty() => empty_panel(IconName::Forward, tr_str(lang, "暂无共享链接", "No shared links yet"), pal),
        SharedMediaSubTab::Links => div().flex().flex_col().gap(px(6.0)).children(model.links.iter().map(|l| {
            let u = l.url.clone();
            div().cursor_pointer().p(px(10.0)).rounded(px(8.0)).bg(rgb(pal.card_bg)).hover(move |s| s.bg(rgb(pal.dialogs_hover_bg)))
                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _e, _w, cx| { this.pending_media_open = Some((u.clone(), false)); cx.notify(); }))
                .child(div().text_size(px(12.5)).font_weight(FontWeight::BOLD).text_color(rgb(pal.text_link)).child(l.title.clone()))
                .child(div().text_size(px(11.0)).text_color(rgb(pal.text_secondary)).child(l.url.clone()))
        })).into_any_element(),
        SharedMediaSubTab::Voice if model.voice_notes.is_empty() => empty_panel(IconName::Mic, tr_str(lang, "暂无语音消息", "No voice messages yet"), pal),
        SharedMediaSubTab::Voice => div().flex().flex_col().gap(px(6.0)).children(model.voice_notes.iter().map(|v| {
            let t = v.title.clone();
            div().cursor_pointer().p(px(10.0)).rounded(px(8.0)).bg(rgb(pal.card_bg)).hover(move |s| s.bg(rgb(pal.dialogs_hover_bg)))
                .flex().items_center().gap(px(8.0))
                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _e, _w, cx| { this.pending_media_open = Some((t.clone(), true)); cx.notify(); }))
                .child(tg_icon(IconName::Play, 14.0, pal.accent))
                .child(div().text_size(px(12.0)).text_color(rgb(pal.text_primary)).child(format!("{} ({}s) · {}", v.title, v.duration_sec, v.date)))
        })).into_any_element(),
    };

    div().flex().flex_col().gap(px(10.0))
        .child(div().flex().flex_wrap().gap(px(4.0)).children(tabs.into_iter().map(|(lbl, st)| {
            let act = model.active_subtab == st;
            div().cursor_pointer().px(px(8.0)).py(px(4.0)).rounded(px(6.0)).bg(rgb(if act { pal.accent } else { pal.card_bg }))
                .text_size(px(11.5)).font_weight(FontWeight::BOLD).text_color(rgb(if act { pal.active_button_fg } else { pal.text_secondary }))
                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _e, _w, cx| { this.shared_media.active_subtab = st; cx.notify(); }))
                .child(lbl.to_string())
        })))
        .child(body).into_any_element()
}

pub fn render_settings_and_stats_panel(
    model: &SettingsTabModel,
    boost_lvl: u32,
    boost_cur: u32,
    boost_tgt: u32,
    stats_bars: &[ChartDataPoint],
    pal: TdesktopThemePalette,
    lang: &str,
    cx: &mut Context<ContextInspector>,
) -> AnyElement {
    let max_val = stats_bars.iter().map(|b| b.value).max().unwrap_or(1).max(1) as f32;
    div().flex().flex_col().gap(px(10.0))
        .when(!model.e2ee_fingerprint.is_empty(), |el| {
            el.child(div().p(px(10.0)).rounded(px(8.0)).bg(rgb(pal.card_bg)).flex().flex_col().gap(px(4.0))
                .child(div().flex().items_center().gap(px(6.0)).child(tg_icon(IconName::Lock, 13.0, pal.success))
                    .child(div().text_size(px(12.0)).font_weight(FontWeight::BOLD).text_color(rgb(pal.success)).child(tr_str(lang, "加密指纹", "Encryption Key"))))
                .child(div().text_size(px(18.0)).child(model.e2ee_fingerprint.clone())))
        })
        .child(div().cursor_pointer().p(px(10.0)).rounded(px(8.0)).bg(rgb(pal.card_bg)).flex().items_center().justify_between()
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| {
                let next = match this.settings.auto_delete_period.as_str() { "Off" => "24 Hours", "24 Hours" => "7 Days", "7 Days" => "30 Days", _ => "Off" };
                this.set_auto_delete_period(next);
                this.pending_action = Some(InspectorAction::ChangeAutoDelete(next.to_string()));
                cx.notify();
            }))
            .child(div().text_size(px(12.5)).text_color(rgb(pal.text_primary)).child(tr_str(lang, "自动删除消息", "Auto-Delete")))
            .child(div().text_size(px(12.5)).font_weight(FontWeight::BOLD).text_color(rgb(pal.accent)).child(model.auto_delete_period.clone())))
        .child(div().p(px(10.0)).rounded(px(8.0)).bg(rgb(pal.card_bg)).flex().flex_col().gap(px(6.0))
            .child(div().text_size(px(12.0)).font_weight(FontWeight::BOLD).text_color(rgb(pal.text_primary))
                .child(format!("Level {} · {}/{} boosts", boost_lvl, boost_cur, boost_tgt)))
            .when(!stats_bars.is_empty(), |el| {
                el.child(div().h(px(64.0)).flex().items_end().gap(px(4.0)).children(stats_bars.iter().take(12).map(|pt| {
                    let h = ((pt.value as f32 / max_val) * 52.0).max(4.0);
                    div().flex_1().h(px(h)).rounded(px(2.0)).bg(rgb(pal.accent))
                })))
            }))
        .into_any_element()
}

fn render_file_list(items: &[SharedFileItem], pal: TdesktopThemePalette, cx: &mut Context<ContextInspector>) -> AnyElement {
    div().flex().flex_col().gap(px(6.0)).children(items.iter().map(|f| {
        let name = f.name.clone();
        div().cursor_pointer().p(px(8.0)).rounded(px(8.0)).bg(rgb(pal.card_bg)).hover(move |s| s.bg(rgb(pal.dialogs_hover_bg)))
            .flex().items_center().gap(px(8.0))
            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _e, _w, cx| { this.pending_media_open = Some((name.clone(), false)); cx.notify(); }))
            .child(tg_icon(IconName::FileDoc, 18.0, pal.accent))
            .child(div().flex().flex_col().child(div().text_size(px(12.5)).text_color(rgb(pal.text_primary)).child(f.name.clone()))
                .child(div().text_size(px(10.5)).text_color(rgb(pal.text_secondary)).child(format!("{} · {}", f.size_text, f.date))))
    })).into_any_element()
}

fn empty_panel(icon: IconName, text: &str, pal: TdesktopThemePalette) -> AnyElement {
    div().py(px(24.0)).flex().flex_col().items_center().gap(px(6.0))
        .child(tg_icon(icon, 28.0, pal.text_secondary))
        .child(div().text_size(px(12.5)).text_color(rgb(pal.text_secondary)).child(text.to_string()))
        .into_any_element()
}
