use gpui::*;
use gpui::prelude::FluentBuilder;
use crate::i18n::tr_str;
use crate::theme::assets::{tg_icon, IconName};
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::settings::settings_theme_ui::{
    settings_card, settings_container, settings_empty_state,
};
use crate::views::settings::settings_type::SettingsSectionType;

#[derive(Debug, Clone)]
pub struct SearchResultItem {
    pub id: String,
    pub title: String,
    pub section: SettingsSectionType,
    pub subtitle: String,
    pub keywords: Vec<String>,
}

pub struct SettingsSearch {
    pub query: String,
    pub all_entries: Vec<SearchResultItem>,
    pub results: Vec<SearchResultItem>,
    pub recent_entry_ids: Vec<String>,
    pub selected_index: usize,
    pub is_visible: bool,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
    pub pending_selected_section: Option<SettingsSectionType>,
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
            palette: TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x3390ec, 14, 12),
            language_id: "en".into(),
            pending_selected_section: None,
        }
    }

    pub fn with_default_entries() -> Self {
        let mut inst = Self::new();
        inst.init_search_index();
        inst.rebuild_results();
        inst
    }

    pub fn new_live(palette: TdesktopThemePalette, language_id: &str) -> Self {
        let mut inst = Self::with_default_entries();
        inst.palette = palette;
        inst.language_id = language_id.into();
        inst
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) { self.palette = palette; }
    pub fn set_language(&mut self, language_id: impl Into<String>) { self.language_id = language_id.into(); }
    pub fn take_selected_section(&mut self) -> Option<SettingsSectionType> { self.pending_selected_section.take() }

    pub fn init_search_index(&mut self) {
        let rows = [
            ("account_name", "Edit Name", SettingsSectionType::Information, "Account > Edit Profile", &["first", "last", "name", "profile"][..]),
            ("account_username", "Username", SettingsSectionType::Information, "Account > Edit Profile", &["username", "handle", "t.me"]),
            ("account_bio", "Bio", SettingsSectionType::Information, "Account > Edit Profile", &["bio", "about", "status"]),
            ("notif_sounds", "Notifications & Sounds", SettingsSectionType::Notifications, "Settings > Notifications", &["sound", "alert", "ringtone", "quiet"]),
            ("privacy_2fa", "Two-Step Verification", SettingsSectionType::PrivacySecurity, "Settings > Privacy", &["password", "2fa", "security", "cloud"]),
            ("privacy_passcode", "Local Passcode Lock", SettingsSectionType::LocalPasscode, "Settings > Privacy", &["lock", "passcode", "pin", "autolock"]),
            ("privacy_passkeys", "Passkeys", SettingsSectionType::Passkeys, "Settings > Privacy", &["fido", "webauthn", "biometric", "passkey"]),
            ("chat_theme", "Chat Appearance & Theme", SettingsSectionType::Chat, "Settings > Chat Settings", &["theme", "dark", "night", "font", "wallpaper"]),
            ("folders_manage", "Chat Folders", SettingsSectionType::Folders, "Settings > Folders", &["folder", "filter", "tab"]),
            ("storage_cache", "Local Storage & Cache", SettingsSectionType::LocalStorage, "Settings > Advanced", &["cache", "storage", "disk", "clear"]),
            ("stars_balance", "Telegram Stars Balance", SettingsSectionType::Credits, "Settings > Telegram Stars", &["stars", "credits", "balance", "topup"]),
            ("premium_features", "Telegram Premium", SettingsSectionType::Premium, "Settings > Premium", &["premium", "4gb", "speed", "subscription"]),
            ("calls_device", "Call Audio & Video Settings", SettingsSectionType::Calls, "Settings > Calls", &["microphone", "mic", "speakers", "camera", "volume"]),
            ("lang_picker", "Language", SettingsSectionType::Language, "Settings > Language", &["language", "english", "chinese", "locale"]),
        ];
        self.all_entries = rows.into_iter().map(|(id, title, section, subtitle, kw)| SearchResultItem {
            id: id.into(),
            title: title.into(),
            section,
            subtitle: subtitle.into(),
            keywords: kw.iter().map(|s| (*s).to_string()).collect(),
        }).collect();
    }

    pub fn index_item(&mut self, id: impl Into<String>, title: impl Into<String>, subtitle: impl Into<String>, section: SettingsSectionType) {
        let title_str = title.into();
        let keywords: Vec<String> = title_str.split_whitespace().map(|w| w.to_lowercase()).collect();
        self.all_entries.push(SearchResultItem { id: id.into(), title: title_str, section, subtitle: subtitle.into(), keywords });
    }

    pub fn search(&mut self, query: &str) -> Vec<SearchResultItem> { self.set_query(query); self.results.clone() }
    pub fn set_query(&mut self, query: impl Into<String>) { self.query = query.into(); self.rebuild_results(); }

    pub fn handle_key_input(&mut self, key: &str) {
        if key == "backspace" { self.query.pop(); }
        else if key == "escape" { self.query.clear(); }
        else if key == "down" { self.select_next(); return; }
        else if key == "up" { self.select_prev(); return; }
        else if key == "enter" {
            if let Some(sel) = self.selected_entry().cloned() {
                self.pending_selected_section = Some(sel.section);
                self.bump_recent_entry(&sel.id);
            }
            return;
        } else if key.chars().count() == 1 { self.query.push_str(key); }
        self.rebuild_results();
    }

    pub fn rebuild_results(&mut self) {
        let q = self.query.trim().to_lowercase();
        self.results = if q.is_empty() {
            self.all_entries.clone()
        } else {
            self.all_entries.iter().filter(|item| {
                item.title.to_lowercase().contains(&q) || item.subtitle.to_lowercase().contains(&q) || item.keywords.iter().any(|k| k.contains(&q))
            }).cloned().collect()
        };
        self.selected_index = 0;
    }

    pub fn bump_recent_entry(&mut self, entry_id: &str) {
        self.recent_entry_ids.retain(|id| id != entry_id);
        self.recent_entry_ids.insert(0, entry_id.to_string());
        if self.recent_entry_ids.len() > 10 { self.recent_entry_ids.truncate(10); }
    }

    pub fn select_next(&mut self) { if !self.results.is_empty() { self.selected_index = (self.selected_index + 1) % self.results.len(); } }
    pub fn select_prev(&mut self) {
        if !self.results.is_empty() { self.selected_index = if self.selected_index == 0 { self.results.len() - 1 } else { self.selected_index - 1 }; }
    }
    pub fn selected_entry(&self) -> Option<&SearchResultItem> { self.results.get(self.selected_index) }
    pub fn toggle_visibility(&mut self) { self.is_visible = !self.is_visible; }
}

impl Default for SettingsSearch {
    fn default() -> Self { Self::new() }
}

impl Render for SettingsSearch {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible { return div().into_any_element(); }
        let pal = self.palette.clone();
        let lang = self.language_id.clone();
        let sel_idx = self.selected_index;
        let query_empty = self.query.is_empty();

        settings_container(pal.clone())
            .child(
                div().flex().flex_row().items_center().justify_between().w_full().px(px(12.0)).py(px(9.0)).rounded(px(8.0))
                    .bg(rgb(pal.input_bg)).border_1().border_color(rgb(pal.border_fg))
                    .child(div().flex().flex_row().items_center().gap(px(8.0))
                        .child(tg_icon(IconName::Search, 15.0, pal.icon_fg))
                        .child(div().text_sm().text_color(rgb(if query_empty { pal.text_secondary } else { pal.text_primary }))
                            .child(if query_empty { tr_str(&lang, "搜索设置...", "Search settings...").to_string() } else { self.query.clone() })))
                    .when(!query_empty, |el| el.child(div().cursor_pointer().on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| { this.set_query(""); cx.notify(); })).child(tg_icon(IconName::Close, 13.0, pal.icon_fg)))),
            )
            .child(div().flex().flex_row().flex_wrap().gap(px(6.0)).children(["", "privacy", "theme", "stars", "mic", "language"].into_iter().map(|q| {
                let active = self.query == q;
                let lbl = if q.is_empty() { "All" } else { q };
                div().px(px(10.0)).py(px(4.0)).rounded_full().bg(rgb(if active { pal.accent } else { pal.card_bg })).border_1().border_color(rgb(if active { pal.accent } else { pal.border_fg }))
                    .text_xs().text_color(rgb(if active { pal.active_button_fg } else { pal.text_primary })).cursor_pointer()
                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| { this.set_query(q); cx.notify(); })).child(lbl)
            })))
            .child(if self.results.is_empty() {
                settings_empty_state(IconName::Search, tr_str(&lang, "未找到匹配设置", "No matching settings found."), pal).into_any_element()
            } else {
                settings_card(pal.clone()).children(self.results.iter().enumerate().map(|(idx, res)| {
                    let is_sel = idx == sel_idx;
                    let sec = res.section;
                    let rid = res.id.clone();
                    div().flex().flex_col().px(px(10.0)).py(px(8.0)).rounded(px(8.0))
                        .bg(rgb(if is_sel { pal.dialogs_hover_bg } else { pal.card_bg })).cursor_pointer()
                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                            this.selected_index = idx;
                            this.pending_selected_section = Some(sec);
                            this.bump_recent_entry(&rid);
                            cx.notify();
                        }))
                        .child(div().text_sm().font_weight(FontWeight::MEDIUM).text_color(rgb(pal.text_primary)).child(res.title.clone()))
                        .child(div().text_xs().text_color(rgb(pal.text_secondary)).child(res.subtitle.clone()))
                })).into_any_element()
            })
            .into_any_element()
    }
}
