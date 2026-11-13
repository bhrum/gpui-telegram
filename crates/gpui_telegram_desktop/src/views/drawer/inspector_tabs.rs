//! Inspector Drawer Tabs: Profile, Members, Shared Media, Settings (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.5).

use gpui::*;
use gpui::prelude::FluentBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InspectorTab {
    Profile,
    Members,
    SharedMedia,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SharedMediaSubTab {
    Media,
    Files,
    Voice,
    Links,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemberItem {
    pub id: u64,
    pub name: String,
    pub initials: String,
    pub color: u32,
    pub role: String, // "Owner", "Admin", "Member"
    pub is_online: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedFileItem {
    pub name: String,
    pub size_text: String,
    pub extension: String,
    pub date: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedLinkItem {
    pub title: String,
    pub url: String,
    pub domain: String,
}

pub struct ProfileTabModel {
    pub name: String,
    pub username: String,
    pub phone: String,
    pub bio: String,
    pub avatar_initials: String,
    pub avatar_color: u32,
    pub is_notifications_enabled: bool,
}

pub struct MembersTabModel {
    pub members: Vec<MemberItem>,
    pub search_query: String,
}

pub struct SharedMediaTabModel {
    pub active_subtab: SharedMediaSubTab,
    pub media_count: u32,
    pub files: Vec<SharedFileItem>,
    pub links: Vec<SharedLinkItem>,
}

pub struct SettingsTabModel {
    pub e2ee_fingerprint: String,
    pub auto_delete_period: String,
    pub notification_sound: String,
}

use crate::views::ContextInspector;

pub struct InspectorTabsView;

impl InspectorTabsView {
    pub fn render_profile(
        model: &ProfileTabModel,
        is_secret_chat_active: bool,
        is_contact_blocked: bool,
        cx: &mut Context<ContextInspector>,
    ) -> impl IntoElement {
        let notif_on = model.is_notifications_enabled;

        div()
            .flex()
            .flex_col()
            .gap(px(16.0))
            // 1. Avatar + Name + Username Header
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(8.0))
                    .py(px(12.0))
                    // 80px Large Avatar
                    .child(
                        div()
                            .w(px(80.0))
                            .h(px(80.0))
                            .rounded_full()
                            .bg(rgb(model.avatar_color))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(28.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child(model.avatar_initials.clone()),
                    )
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child(model.name.clone()),
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(rgb(0x2AABEE))
                            .child(model.username.clone()),
                    ),
            )
            // 2. Info Items (Phone, Bio)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .p(px(12.0))
                    .rounded(px(10.0))
                    .bg(rgb(0x1B2836))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(div().text_size(px(11.0)).text_color(rgb(0x8A9AA8)).child("Phone"))
                            .child(div().text_size(px(14.0)).text_color(rgb(0xFFFFFF)).child(model.phone.clone())),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(div().text_size(px(11.0)).text_color(rgb(0x8A9AA8)).child("Bio"))
                            .child(div().text_size(px(13.0)).text_color(rgb(0xFFFFFF)).child(model.bio.clone())),
                    ),
            )
            // 3. Notifications switch (Clickable)
            .child(
                div()
                    .cursor_pointer()
                    .flex()
                    .items_center()
                    .justify_between()
                    .p(px(12.0))
                    .rounded(px(10.0))
                    .bg(rgb(0x1B2836))
                    .hover(|s| s.bg(rgb(0x243447)))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _event, _window, cx| {
                            this.profile.is_notifications_enabled = !this.profile.is_notifications_enabled;
                            cx.notify();
                        }),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(div().text_size(px(16.0)).child("🔔"))
                            .child(div().text_size(px(14.0)).text_color(rgb(0xFFFFFF)).child("Notifications")),
                    )
                    .child(
                        div()
                            .w(px(36.0))
                            .h(px(20.0))
                            .rounded_full()
                            .bg(if notif_on { rgb(0x2AABEE) } else { rgb(0x6B808D) })
                            .p(px(2.0))
                            .flex()
                            .when(notif_on, |s| s.justify_end())
                            .when(!notif_on, |s| s.justify_start())
                            .child(
                                div()
                                    .w(px(16.0))
                                    .h(px(16.0))
                                    .rounded_full()
                                    .bg(rgb(0xFFFFFF)),
                            ),
                    ),
            )
            // 4. Quick Actions (Clickable)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .child(
                        div()
                            .cursor_pointer()
                            .h(px(36.0))
                            .rounded(px(8.0))
                            .bg(if is_secret_chat_active { rgba(0x4CAF5033) } else { rgba(0x2AABEE22) })
                            .hover(|s| s.bg(rgba(0x2AABEE44)))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(if is_secret_chat_active { rgb(0x4CAF50) } else { rgb(0x2AABEE) })
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _event, _window, cx| {
                                    this.is_secret_chat_active = !this.is_secret_chat_active;
                                    cx.notify();
                                }),
                            )
                            .child(if is_secret_chat_active {
                                "🔒 Secret Chat Active (E2EE)"
                            } else {
                                "Start Secret Chat"
                            }),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .h(px(36.0))
                            .rounded(px(8.0))
                            .bg(if is_contact_blocked { rgba(0xE5393555) } else { rgba(0xE5393522) })
                            .hover(|s| s.bg(rgba(0xE5393544)))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xE53935))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _event, _window, cx| {
                                    this.is_contact_blocked = !this.is_contact_blocked;
                                    cx.notify();
                                }),
                            )
                            .child(if is_contact_blocked {
                                "🚫 Unblock Contact"
                            } else {
                                "Block Contact"
                            }),
                    ),
            )
    }

    pub fn render_members(model: &MembersTabModel, cx: &mut Context<ContextInspector>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            // Search / Add header
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x8A9AA8))
                            .child(format!("{} MEMBERS", model.members.len())),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .px(px(6.0))
                            .py(px(2.0))
                            .rounded(px(4.0))
                            .hover(|s| s.bg(rgb(0x243447)))
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x2AABEE))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _event, _window, cx| {
                                    let next_id = (this.members.members.len() + 1) as u64;
                                    this.members.members.push(MemberItem {
                                        id: next_id,
                                        name: format!("Contributor #{}", next_id),
                                        initials: format!("C{}", next_id),
                                        color: 0x3498DB,
                                        role: "Member".to_string(),
                                        is_online: true,
                                    });
                                    cx.notify();
                                }),
                            )
                            .child("+ Add Member"),
                    ),
            )
            // Members list
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .children(model.members.iter().map(|m| {
                        let member_id = m.id;
                        div()
                            .cursor_pointer()
                            .h(px(48.0))
                            .px(px(8.0))
                            .rounded(px(8.0))
                            .bg(rgb(0x1B2836))
                            .hover(|s| s.bg(rgb(0x243447)))
                            .flex()
                            .items_center()
                            .justify_between()
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _event, _window, cx| {
                                    if let Some(mem) = this.members.members.iter_mut().find(|item| item.id == member_id) {
                                        mem.is_online = !mem.is_online;
                                    }
                                    cx.notify();
                                }),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(10.0))
                                    // Avatar
                                    .child(
                                        div()
                                            .w(px(32.0))
                                            .h(px(32.0))
                                            .rounded_full()
                                            .bg(rgb(m.color))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_size(px(12.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xFFFFFF))
                                            .child(m.initials.clone()),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(
                                                div()
                                                    .text_size(px(13.0))
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(rgb(0xFFFFFF))
                                                    .child(m.name.clone()),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(11.0))
                                                    .text_color(if m.is_online { rgb(0x4CAF50) } else { rgb(0x8A9AA8) })
                                                    .child(if m.is_online { "online" } else { "last seen recently" }),
                                            ),
                                    ),
                            )
                            // Role badge (Owner, Admin)
                            .when(!m.role.is_empty(), |el| {
                                el.child(
                                    div()
                                        .px(px(6.0))
                                        .py(px(2.0))
                                        .rounded(px(4.0))
                                        .bg(rgba(0x2AABEE22))
                                        .text_size(px(10.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0x2AABEE))
                                        .child(m.role.clone()),
                                )
                            })
                    })),
            )
    }

    pub fn render_shared_media(model: &SharedMediaTabModel, cx: &mut Context<ContextInspector>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(12.0))
            // Subtabs chips (Clickable)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .child(Self::render_subtab_chip(
                        "Media",
                        SharedMediaSubTab::Media,
                        model.active_subtab == SharedMediaSubTab::Media,
                        cx,
                    ))
                    .child(Self::render_subtab_chip(
                        "Files",
                        SharedMediaSubTab::Files,
                        model.active_subtab == SharedMediaSubTab::Files,
                        cx,
                    ))
                    .child(Self::render_subtab_chip(
                        "Voice",
                        SharedMediaSubTab::Voice,
                        model.active_subtab == SharedMediaSubTab::Voice,
                        cx,
                    ))
                    .child(Self::render_subtab_chip(
                        "Links",
                        SharedMediaSubTab::Links,
                        model.active_subtab == SharedMediaSubTab::Links,
                        cx,
                    )),
            )
            // Content according to subtab
            .child(
                match model.active_subtab {
                    SharedMediaSubTab::Media => {
                        div()
                            .flex()
                            .flex_wrap()
                            .gap(px(4.0))
                            .children((0..6).map(|i| {
                                let url = format!("https://example.com/shared_photo_{}.jpg", i + 1);
                                div()
                                    .cursor_pointer()
                                    .w(px(90.0))
                                    .h(px(90.0))
                                    .rounded(px(6.0))
                                    .bg(rgb(0x243447 + i * 0x050505))
                                    .hover(|s| s.border_2().border_color(rgb(0x2AABEE)))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_size(px(20.0))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _event, _window, cx| {
                                            this.pending_media_open = Some((url.clone(), false));
                                            cx.notify();
                                        }),
                                    )
                                    .child("🖼️")
                            }))
                            .into_any_element()
                    }
                    SharedMediaSubTab::Files => {
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(6.0))
                            .children(model.files.iter().map(|f| {
                                let fname = f.name.clone();
                                div()
                                    .cursor_pointer()
                                    .h(px(44.0))
                                    .px(px(8.0))
                                    .rounded(px(6.0))
                                    .bg(rgb(0x1B2836))
                                    .hover(|s| s.bg(rgb(0x243447)))
                                    .flex()
                                    .items_center()
                                    .gap(px(8.0))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _event, _window, cx| {
                                            this.pending_media_open = Some((fname.clone(), false));
                                            cx.notify();
                                        }),
                                    )
                                    .child(
                                        div()
                                            .w(px(30.0))
                                            .h(px(30.0))
                                            .rounded(px(4.0))
                                            .bg(rgb(0x2AABEE))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_size(px(10.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xFFFFFF))
                                            .child(f.extension.clone()),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(div().text_size(px(12.0)).text_color(rgb(0xFFFFFF)).child(f.name.clone()))
                                            .child(div().text_size(px(10.0)).text_color(rgb(0x8A9AA8)).child(format!("{} • {}", f.size_text, f.date))),
                                    )
                            }))
                            .into_any_element()
                    }
                    SharedMediaSubTab::Links => {
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(6.0))
                            .children(model.links.iter().map(|l| {
                                let link_url = l.url.clone();
                                div()
                                    .cursor_pointer()
                                    .p(px(8.0))
                                    .rounded(px(6.0))
                                    .bg(rgb(0x1B2836))
                                    .hover(|s| s.bg(rgb(0x243447)))
                                    .flex()
                                    .flex_col()
                                    .gap(px(2.0))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _event, _window, cx| {
                                            this.pending_media_open = Some((link_url.clone(), false));
                                            cx.notify();
                                        }),
                                    )
                                    .child(div().text_size(px(12.0)).font_weight(FontWeight::BOLD).text_color(rgb(0x2AABEE)).child(l.title.clone()))
                                    .child(div().text_size(px(11.0)).text_color(rgb(0x8A9AA8)).child(l.url.clone()))
                            }))
                            .into_any_element()
                    }
                    SharedMediaSubTab::Voice => {
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(6.0))
                            .child(
                                div()
                                    .cursor_pointer()
                                    .p(px(10.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(0x1B2836))
                                    .hover(|s| s.bg(rgb(0x243447)))
                                    .text_size(px(12.0))
                                    .text_color(rgb(0xFFFFFF))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _event, _window, cx| {
                                            this.pending_media_open = Some(("voice_note_1.ogg".to_string(), true));
                                            cx.notify();
                                        }),
                                    )
                                    .child("🎙️ Voice Note (00:42) — Today at 14:30"),
                            )
                            .into_any_element()
                    }
                },
            )
    }

    pub fn render_settings(model: &SettingsTabModel, cx: &mut Context<ContextInspector>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(12.0))
            // E2EE Encryption fingerprint card
            .child(
                div()
                    .p(px(12.0))
                    .rounded(px(10.0))
                    .bg(rgb(0x1B2836))
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x4CAF50))
                            .child("🔒 Encryption Key Fingerprint"),
                    )
                    .child(
                        div()
                            .text_size(px(20.0))
                            .child(model.e2ee_fingerprint.clone()),
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(0x8A9AA8))
                            .child("Compare with peer to verify end-to-end encryption."),
                    ),
            )
            // Auto-delete timer (Clickable)
            .child(
                div()
                    .cursor_pointer()
                    .p(px(12.0))
                    .rounded(px(10.0))
                    .bg(rgb(0x1B2836))
                    .hover(|s| s.bg(rgb(0x243447)))
                    .flex()
                    .items_center()
                    .justify_between()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _event, _window, cx| {
                            let next = match this.settings.auto_delete_period.as_str() {
                                "Off" => "24 Hours",
                                "24 Hours" => "7 Days",
                                "7 Days" => "30 Days",
                                _ => "Off",
                            };
                            this.set_auto_delete_period(next);
                            cx.notify();
                        }),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(div().text_size(px(13.0)).text_color(rgb(0xFFFFFF)).child("Auto-Delete Messages"))
                            .child(div().text_size(px(11.0)).text_color(rgb(0x8A9AA8)).child("Click to cycle self-destruct timer")),
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x2AABEE))
                            .child(model.auto_delete_period.clone()),
                    ),
            )
            // Notification sound (Clickable)
            .child(
                div()
                    .cursor_pointer()
                    .p(px(12.0))
                    .rounded(px(10.0))
                    .bg(rgb(0x1B2836))
                    .hover(|s| s.bg(rgb(0x243447)))
                    .flex()
                    .items_center()
                    .justify_between()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _event, _window, cx| {
                            let next = match this.settings.notification_sound.as_str() {
                                "Default (Aurora)" => "Note",
                                "Note" => "Chime",
                                "Chime" => "Silent",
                                _ => "Default (Aurora)",
                            };
                            this.settings.notification_sound = next.to_string();
                            cx.notify();
                        }),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(div().text_size(px(13.0)).text_color(rgb(0xFFFFFF)).child("Notification Sound"))
                            .child(div().text_size(px(11.0)).text_color(rgb(0x8A9AA8)).child("Click to change chat tone")),
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(rgb(0x2AABEE))
                            .child(model.notification_sound.clone()),
                    ),
            )
    }

    fn render_subtab_chip(
        label: &str,
        subtab: SharedMediaSubTab,
        is_active: bool,
        cx: &mut Context<ContextInspector>,
    ) -> impl IntoElement {
        div()
            .cursor_pointer()
            .px(px(8.0))
            .py(px(4.0))
            .rounded(px(6.0))
            .bg(if is_active { rgb(0x2AABEE) } else { rgb(0x1B2836) })
            .hover(|s| s.bg(rgb(0x2B5278)))
            .text_size(px(12.0))
            .font_weight(FontWeight::BOLD)
            .text_color(if is_active { rgb(0xFFFFFF) } else { rgb(0x8A9AA8) })
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _event, _window, cx| {
                    this.shared_media.active_subtab = subtab;
                    cx.notify();
                }),
            )
            .child(label.to_string())
    }
}
