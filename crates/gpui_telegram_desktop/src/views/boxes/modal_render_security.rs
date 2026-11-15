use crate::i18n::tr_str;
use crate::theme::assets::{IconName, tg_icon};
use crate::views::boxes::connection_box::{ProxyConfig, ProxyItemState, ProxyType, proxy_to_tg_link};
use crate::views::boxes::edit_privacy_box::PrivacyRuleOption;
use crate::views::boxes::modal_theme::{
    modal_card, modal_error_banner, modal_field, modal_footer_cancel, modal_header, DialogClose,
    DialogMut,
};
use crate::views::boxes::modal_types::{ActiveTextField, ModalActionOutcome};
use crate::views::boxes::passcode_box::{PasscodeBox, PasscodeMode};
use crate::views::modal_boxes_suite::ActiveModalDialog;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_telegram_client::ProxyEndpointRecord;

pub fn render_connection_proxy<V: 'static>(
    dlg: &ActiveModalDialog,
    access: DialogMut<V>,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> AnyElement {
    let pal = dlg.palette;
    let lang = &dlg.language_id;
    let lang_owned = lang.clone();
    let is_direct_active = !dlg.proxies.iter().any(|p| p.enabled);

    modal_card(450.0, pal)
        .child(modal_header(
            IconName::SettingsGear,
            tr_str(lang, "连接与代理设置", "Connection Settings"),
            pal,
            on_close,
            cx,
        ))
        .child(
            div()
                .id("conn-proxy-direct-row")
                .cursor_pointer()
                .px(px(10.0))
                .py(px(8.0))
                .rounded(px(8.0))
                .bg(rgb(pal.input_bg))
                .border_1()
                .border_color(rgb(if is_direct_active { pal.accent } else { pal.border_fg }))
                .flex()
                .items_center()
                .gap(px(8.0))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _e, _w, cx| {
                        let d = access(this);
                        for item in &mut d.proxies {
                            item.enabled = false;
                        }
                        d.pending_outcome = Some(ModalActionOutcome::SelectProxy { proxy_id: 0 });
                        on_close(this);
                        cx.notify();
                    }),
                )
                .child(tg_icon(
                    IconName::SelectCheck,
                    14.0,
                    if is_direct_active { pal.accent } else { pal.text_secondary },
                ))
                .child(
                    div()
                        .text_size(px(12.5))
                        .font_weight(if is_direct_active { FontWeight::SEMIBOLD } else { FontWeight::NORMAL })
                        .text_color(rgb(pal.text_primary))
                        .child(tr_str(lang, "直连 (默认，不使用代理)", "Direct Connection (No Proxy)").to_string()),
                ),
        )
        .map(|el| {
            if dlg.proxies.is_empty() {
                el.child(
                    div()
                        .py(px(6.0))
                        .text_size(px(12.5))
                        .text_color(rgb(pal.text_secondary))
                        .child(tr_str(lang, "尚未添加自定义代理服务器", "No custom proxies configured")),
                )
            } else {
                el.children(dlg.proxies.iter().cloned().map(|p| {
                    let pid = p.id;
                    let cfg = ProxyConfig {
                        id: p.id,
                        proxy_type: match p.kind.to_ascii_lowercase().as_str() {
                            "socks5" => ProxyType::Socks5,
                            "mtproto" => ProxyType::Mtproto,
                            _ => ProxyType::Web,
                        },
                        host: p.host.clone(),
                        port: p.port,
                        user: None,
                        password: None,
                        secret: if p.secret_or_user.is_empty() {
                            None
                        } else {
                            Some(p.secret_or_user.clone())
                        },
                        ping_ms: Some(p.ping_ms),
                        state: ProxyItemState::Online,
                        is_enabled: p.enabled,
                    };
                    let tg_link = proxy_to_tg_link(&cfg);
                    div()
                        .px(px(10.0))
                        .py(px(8.0))
                        .rounded(px(8.0))
                        .bg(rgb(pal.input_bg))
                        .border_1()
                        .border_color(rgb(if p.enabled { pal.accent } else { pal.border_fg }))
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .cursor_pointer()
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _e, _w, cx| {
                                        let d = access(this);
                                        for item in &mut d.proxies {
                                            item.enabled = item.id == pid;
                                        }
                                        d.pending_outcome = Some(ModalActionOutcome::SelectProxy { proxy_id: pid });
                                        on_close(this);
                                        cx.notify();
                                    }),
                                )
                                .child(tg_icon(
                                    IconName::SelectCheck,
                                    14.0,
                                    if p.enabled { pal.accent } else { pal.text_secondary },
                                ))
                                .child(
                                    div()
                                        .text_size(px(12.5))
                                        .text_color(rgb(pal.text_primary))
                                        .child(format!("[{}] {}:{}", p.kind, p.host, p.port)),
                                ),
                        )
                        .child(
                            div()
                                .cursor_pointer()
                                .text_size(px(11.5))
                                .text_color(rgb(pal.text_link))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _e, _w, cx| {
                                        cx.stop_propagation();
                                        access(this).pending_outcome =
                                            Some(ModalActionOutcome::CopyToClipboard(tg_link.clone()));
                                        cx.notify();
                                    }),
                                )
                                .child(tr_str(lang, "复制链接", "Copy").to_string()),
                        )
                }))
            }
        })
        .child(
            div()
                .flex()
                .gap(px(6.0))
                .children(["MTProto", "SOCKS5", "HTTP"].into_iter().map(|k| {
                    let active = dlg.new_proxy_kind == k;
                    let ks = k.to_string();
                    div()
                        .cursor_pointer()
                        .px(px(10.0))
                        .py(px(5.0))
                        .rounded(px(6.0))
                        .bg(rgb(if active { pal.accent } else { pal.input_bg }))
                        .text_size(px(12.0))
                        .text_color(rgb(if active { pal.active_button_fg } else { pal.text_primary }))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _e, _w, cx| {
                                access(this).new_proxy_kind = ks.clone();
                                cx.notify();
                            }),
                        )
                        .child(k)
                })),
        )
        .child(modal_field(
            tr_str(lang, "服务器地址", "Server"),
            &dlg.primary_input,
            "127.0.0.1",
            ActiveTextField::Primary,
            dlg.active_field == ActiveTextField::Primary,
            pal,
            access,
            cx,
        ))
        .child(modal_field(
            tr_str(lang, "端口", "Port"),
            &dlg.secondary_input,
            "443",
            ActiveTextField::Secondary,
            dlg.active_field == ActiveTextField::Secondary,
            pal,
            access,
            cx,
        ))
        .child(modal_field(
            tr_str(lang, "密钥 / 用户名", "Secret / Credentials"),
            &dlg.tertiary_input,
            tr_str(lang, "选填...", "Optional..."),
            ActiveTextField::Tertiary,
            dlg.active_field == ActiveTextField::Tertiary,
            pal,
            access,
            cx,
        ))
        .child(modal_error_banner(dlg.validation_error.as_ref(), pal))
        .child(
            div()
                .flex()
                .justify_end()
                .gap(px(8.0))
                .child(modal_footer_cancel(lang, pal, on_close, cx))
                .child(
                    div()
                        .cursor_pointer()
                        .px(px(14.0))
                        .py(px(7.0))
                        .rounded(px(8.0))
                        .bg(rgb(pal.accent))
                        .text_size(px(13.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.active_button_fg))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _e, _w, cx| {
                                let d = access(this);
                                let host = d.primary_input.trim().to_string();
                                let port_val = d.secondary_input.trim().parse::<u16>();
                                let Some(port) = port_val.ok().filter(|&p| p > 0) else {
                                    d.validation_error = Some(
                                        tr_str(&lang_owned, "请输入有效端口 (1-65535)", "Valid port (1-65535) is required").into(),
                                    );
                                    cx.notify();
                                    return;
                                };
                                if host.is_empty() {
                                    d.validation_error = Some(
                                        tr_str(&lang_owned, "请输入服务器地址", "Server host is required").into(),
                                    );
                                    cx.notify();
                                    return;
                                }
                                let next_id = d.proxies.iter().map(|p| p.id).max().unwrap_or(0) + 1;
                                let rec = ProxyEndpointRecord {
                                    id: next_id,
                                    kind: d.new_proxy_kind.clone(),
                                    host,
                                    port,
                                    secret_or_user: d.tertiary_input.clone(),
                                    ping_ms: 0,
                                    enabled: true,
                                };
                                for p in &mut d.proxies {
                                    p.enabled = false;
                                }
                                d.proxies.push(rec.clone());
                                d.pending_outcome = Some(ModalActionOutcome::AddProxy(rec));
                                d.validation_error = None;
                                on_close(this);
                                cx.notify();
                            }),
                        )
                        .child(tr_str(lang, "保存并启用", "Save Proxy").to_string()),
                ),
        )
        .into_any_element()
}

pub fn render_passcode_setup<V: 'static>(
    dlg: &ActiveModalDialog,
    access: DialogMut<V>,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> AnyElement {
    let pal = dlg.palette;
    let lang = &dlg.language_id;
    let mask_a = "*".repeat(dlg.primary_input.chars().count());
    let mask_b = "*".repeat(dlg.secondary_input.chars().count());

    modal_card(390.0, pal)
        .child(modal_header(
            IconName::Lock,
            tr_str(lang, "本地锁定密码", "Local Passcode"),
            pal,
            on_close,
            cx,
        ))
        .child(modal_field(
            tr_str(lang, "新密码", "New Passcode"),
            &mask_a,
            tr_str(lang, "至少 4 位字符...", "At least 4 characters..."),
            ActiveTextField::Primary,
            dlg.active_field == ActiveTextField::Primary,
            pal,
            access,
            cx,
        ))
        .child(modal_field(
            tr_str(lang, "确认密码", "Re-enter Passcode"),
            &mask_b,
            tr_str(lang, "再次输入新密码...", "Repeat passcode..."),
            ActiveTextField::Secondary,
            dlg.active_field == ActiveTextField::Secondary,
            pal,
            access,
            cx,
        ))
        .child(modal_error_banner(dlg.validation_error.as_ref(), pal))
        .child(
            div()
                .flex()
                .justify_between()
                .gap(px(8.0))
                .child(
                    div()
                        .cursor_pointer()
                        .px(px(10.0))
                        .py(px(7.0))
                        .rounded(px(8.0))
                        .bg(rgb(pal.input_bg))
                        .text_size(px(12.0))
                        .text_color(rgb(pal.danger))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _e, _w, cx| {
                                access(this).pending_outcome =
                                    Some(ModalActionOutcome::SetLocalPasscode(None));
                                on_close(this);
                                cx.notify();
                            }),
                        )
                        .child(tr_str(lang, "关闭密码", "Turn Off").to_string()),
                )
                .child(
                    div()
                        .flex()
                        .gap(px(8.0))
                        .child(modal_footer_cancel(lang, pal, on_close, cx))
                        .child(
                            div()
                                .cursor_pointer()
                                .px(px(14.0))
                                .py(px(7.0))
                                .rounded(px(8.0))
                                .bg(rgb(pal.accent))
                                .text_size(px(13.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(pal.active_button_fg))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _e, _w, cx| {
                                        let d = access(this);
                                        let mut pb = PasscodeBox::new(PasscodeMode::Create, false);
                                        pb.set_new_passcode(d.primary_input.clone());
                                        pb.set_reenter_passcode(d.secondary_input.clone());
                                        if let Err(err) = pb.validate() {
                                            let localized = if d.language_id.starts_with("zh") {
                                                let es = err.to_string();
                                                if es.contains("match") {
                                                    "两次输入的密码不一致".to_string()
                                                } else {
                                                    "密码长度至少需要 4 位字符".to_string()
                                                }
                                            } else {
                                                err.to_string()
                                            };
                                            d.validation_error = Some(localized);
                                            cx.notify();
                                            return;
                                        }
                                        d.pending_outcome = Some(ModalActionOutcome::SetLocalPasscode(
                                            Some(d.primary_input.clone()),
                                        ));
                                        on_close(this);
                                        cx.notify();
                                    }),
                                )
                                .child(tr_str(lang, "保存", "Save").to_string()),
                        ),
                ),
        )
        .into_any_element()
}

pub fn render_edit_privacy<V: 'static>(
    dlg: &ActiveModalDialog,
    category: String,
    access: DialogMut<V>,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> AnyElement {
    let pal = dlg.palette;
    let lang = &dlg.language_id;
    let options = [
        (PrivacyRuleOption::Everyone, tr_str(lang, "所有人", "Everybody")),
        (PrivacyRuleOption::Contacts, tr_str(lang, "我的联系人", "My Contacts")),
        (PrivacyRuleOption::Nobody, tr_str(lang, "不允许任何人", "Nobody")),
        (PrivacyRuleOption::Premium, tr_str(lang, "仅限 Premium", "Premium Only")),
    ];

    modal_card(400.0, pal)
        .child(modal_header(
            IconName::Lock,
            &format!("{} — {}", tr_str(lang, "隐私规则", "Privacy Rule"), category),
            pal,
            on_close,
            cx,
        ))
        .children(options.into_iter().map(|(rule, label)| {
            let active = dlg.selected_privacy_rule == rule;
            div()
                .cursor_pointer()
                .px(px(12.0))
                .py(px(9.0))
                .rounded(px(8.0))
                .bg(rgb(pal.input_bg))
                .border_1()
                .border_color(rgb(if active { pal.accent } else { pal.border_fg }))
                .flex()
                .items_center()
                .justify_between()
                .text_size(px(13.0))
                .text_color(rgb(pal.text_primary))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _e, _w, cx| {
                        access(this).selected_privacy_rule = rule;
                        cx.notify();
                    }),
                )
                .child(label.to_string())
                .child(tg_icon(
                    IconName::SelectCheck,
                    14.0,
                    if active { pal.accent } else { pal.text_secondary },
                ))
        }))
        .child(
            div()
                .flex()
                .justify_end()
                .gap(px(8.0))
                .child(modal_footer_cancel(lang, pal, on_close, cx))
                .child(
                    div()
                        .cursor_pointer()
                        .px(px(16.0))
                        .py(px(7.0))
                        .rounded(px(8.0))
                        .bg(rgb(pal.accent))
                        .text_size(px(13.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.active_button_fg))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _e, _w, cx| {
                                let d = access(this);
                                d.pending_outcome = Some(ModalActionOutcome::SavePrivacyRule {
                                    category: category.clone(),
                                    rule: d.selected_privacy_rule,
                                });
                                on_close(this);
                                cx.notify();
                            }),
                        )
                        .child(tr_str(lang, "应用", "Apply").to_string()),
                ),
        )
        .into_any_element()
}
