//! Upstream Source: Telegram/SourceFiles/settings/settings_search.cpp, Telegram/SourceFiles/settings/settings_search.h
//! Subsystem: settings | Feature ID: TCG-N21 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use super::settings_type::SettingsSectionType;

#[derive(Debug, Clone)]
pub struct SearchResultItem {
    pub id: String,
    pub title: String,
    pub section: SettingsSectionType,
    pub subtitle: String,
    pub keywords: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SettingsSearch {
    pub query: String,
    pub all_entries: Vec<SearchResultItem>,
    pub results: Vec<SearchResultItem>,
    pub recent_entry_ids: Vec<String>,
    pub selected_index: usize,
    pub is_visible: bool,
}

impl SettingsSearch {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            all_entries: Vec::new(),
            results: Vec::new(),
            recent_entry_ids: Vec::new(),
            selected_index: 0,
            is_visible: true,
        }
    }

    pub fn with_default_entries() -> Self {
        let mut inst = Self::new();
        inst.init_search_index();
        inst
    }

    pub fn init_search_index(&mut self) {
        self.all_entries = vec![
            SearchResultItem {
                id: "account_name".into(),
                title: "Edit Name".into(),
                section: SettingsSectionType::Information,
                subtitle: "Account > Edit Profile".into(),
                keywords: vec!["first", "last", "name", "profile"].into_iter().map(String::from).collect(),
            },
            SearchResultItem {
                id: "account_username".into(),
                title: "Username".into(),
                section: SettingsSectionType::Information,
                subtitle: "Account > Edit Profile".into(),
                keywords: vec!["username", "handle", "t.me"].into_iter().map(String::from).collect(),
            },
            SearchResultItem {
                id: "account_bio".into(),
                title: "Bio".into(),
                section: SettingsSectionType::Information,
                subtitle: "Account > Edit Profile".into(),
                keywords: vec!["bio", "about", "status"].into_iter().map(String::from).collect(),
            },
            SearchResultItem {
                id: "notif_sounds".into(),
                title: "Notifications & Sounds".into(),
                section: SettingsSectionType::Notifications,
                subtitle: "Settings > Notifications".into(),
                keywords: vec!["sound", "alert", "ringtone", "quiet"].into_iter().map(String::from).collect(),
            },
            SearchResultItem {
                id: "privacy_2fa".into(),
                title: "Two-Step Verification".into(),
                section: SettingsSectionType::PrivacySecurity,
                subtitle: "Settings > Privacy".into(),
                keywords: vec!["password", "2fa", "security", "cloud"].into_iter().map(String::from).collect(),
            },
            SearchResultItem {
                id: "privacy_passcode".into(),
                title: "Local Passcode Lock".into(),
                section: SettingsSectionType::LocalPasscode,
                subtitle: "Settings > Privacy".into(),
                keywords: vec!["lock", "passcode", "pin", "autolock"].into_iter().map(String::from).collect(),
            },
            SearchResultItem {
                id: "privacy_passkeys".into(),
                title: "Passkeys".into(),
                section: SettingsSectionType::Passkeys,
                subtitle: "Settings > Privacy".into(),
                keywords: vec!["fido", "webauthn", "biometric", "passkey"].into_iter().map(String::from).collect(),
            },
            SearchResultItem {
                id: "stars_balance".into(),
                title: "Telegram Stars Balance".into(),
                section: SettingsSectionType::Credits,
                subtitle: "Settings > Telegram Stars".into(),
                keywords: vec!["stars", "credits", "balance", "topup"].into_iter().map(String::from).collect(),
            },
            SearchResultItem {
                id: "premium_features".into(),
                title: "Telegram Premium".into(),
                section: SettingsSectionType::Premium,
                subtitle: "Settings > Premium".into(),
                keywords: vec!["premium", "4gb", "speed", "subscription"].into_iter().map(String::from).collect(),
            },
            SearchResultItem {
                id: "calls_device".into(),
                title: "Call Audio & Video Settings".into(),
                section: SettingsSectionType::Calls,
                subtitle: "Settings > Calls".into(),
                keywords: vec!["microphone", "mic", "speakers", "camera", "volume"].into_iter().map(String::from).collect(),
            },
        ];
    }

    pub fn index_item(
        &mut self,
        id: impl Into<String>,
        title: impl Into<String>,
        subtitle: impl Into<String>,
        section: SettingsSectionType,
    ) {
        let t = title.into();
        let sub = subtitle.into();
        let kw: Vec<String> = t.to_lowercase().split_whitespace().map(String::from).collect();
        self.all_entries.push(SearchResultItem {
            id: id.into(),
            title: t,
            subtitle: sub,
            section,
            keywords: kw,
        });
    }

    pub fn search(&mut self, query: &str) -> Vec<SearchResultItem> {
        self.set_query(query);
        self.results.clone()
    }

    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
        self.rebuild_results();
    }

    pub fn rebuild_results(&mut self) {
        let q = self.query.trim().to_lowercase();
        if q.is_empty() {
            self.results.clear();
            self.selected_index = 0;
            return;
        }

        self.results = self
            .all_entries
            .iter()
            .filter(|item| {
                item.title.to_lowercase().contains(&q)
                    || item.subtitle.to_lowercase().contains(&q)
                    || item.keywords.iter().any(|k| k.contains(&q))
            })
            .cloned()
            .collect();

        self.selected_index = 0;
    }

    pub fn bump_recent_entry(&mut self, entry_id: &str) {
        self.recent_entry_ids.retain(|id| id != entry_id);
        self.recent_entry_ids.insert(0, entry_id.to_string());
        if self.recent_entry_ids.len() > 10 {
            self.recent_entry_ids.truncate(10);
        }
    }

    pub fn select_next(&mut self) {
        if !self.results.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.results.len();
        }
    }

    pub fn select_prev(&mut self) {
        if !self.results.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.results.len() - 1
            } else {
                self.selected_index - 1
            };
        }
    }

    pub fn selected_entry(&self) -> Option<&SearchResultItem> {
        self.results.get(self.selected_index)
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }
}

impl Default for SettingsSearch {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsSearch {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }

        let sel_idx = self.selected_index;

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .bg(rgb(0x0e1621))
            .child(
                // Search bar input field mockup
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .w_full()
                    .px(px(12.0))
                    .py(px(8.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x17212b))
                    .border_1()
                    .border_color(rgb(0x2b3846))
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x6c7883))
                            .child("🔍 "),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(if self.query.is_empty() { rgb(0x6c7883) } else { rgb(0xffffff) })
                            .child(if self.query.is_empty() { "Search settings...".into() } else { self.query.clone() }),
                    ),
            )
            .child(
                // Search results list
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .pt(px(8.0))
                    .children(self.results.iter().enumerate().map(|(idx, res)| {
                        let is_selected = idx == sel_idx;
                        div()
                            .flex()
                            .flex_col()
                            .px(px(12.0))
                            .py(px(8.0))
                            .rounded(px(6.0))
                            .bg(if is_selected { rgb(0x232e3c) } else { rgb(0x17212b) })
                            .cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _window, cx| {
                                this.selected_index = idx;
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(0xffffff))
                                    .child(res.title.clone()),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x6c7883))
                                    .child(res.subtitle.clone()),
                            )
                    })),
            )
            .into_any_element()
    }
}
