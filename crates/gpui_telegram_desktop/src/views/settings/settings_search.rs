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
        let rows: [(&str, &str, SettingsSectionType, &str, &[&str]); 23] = [
            ("ai_providers_magpie", "AI Subscriptions & Providers (Magpie)", SettingsSectionType::AiProviders, "Settings > AI Providers", &["ai", "magpie", "provider", "subscription", "model", "api", "key", "deepseek", "kimi", "moonshot", "zhipu", "glm", "qwen", "openai", "anthropic", "claude", "gemini", "ollama", "codex", "masking", "routing", "供应商", "模型", "订阅", "密钥", "脱敏", "路由"]),
            ("account_name", "Edit Name", SettingsSectionType::Information, "Account > Edit Profile", &["first", "last", "name", "profile", "编辑资料", "姓名", "昵称", "个人资料", "手机号", "phone"]),
            ("account_username", "Username", SettingsSectionType::Information, "Account > Edit Profile", &["username", "handle", "t.me", "用户名", "账号"]),
            ("account_bio", "Bio", SettingsSectionType::Information, "Account > Edit Profile", &["bio", "about", "status", "简介", "签名", "个人简介"]),
            ("notif_sounds", "Notifications & Sounds", SettingsSectionType::Notifications, "Settings > Notifications", &["sound", "alert", "ringtone", "quiet", "badge", "通知", "声音", "消息提醒", "免打扰", "角标"]),
            ("privacy_2fa", "Two-Step Verification", SettingsSectionType::PrivacySecurity, "Settings > Privacy", &["password", "2fa", "security", "cloud", "privacy", "隐私", "安全", "两步验证", "云密码"]),
            ("privacy_blocked", "Blocked Users", SettingsSectionType::BlockedPeers, "Settings > Privacy > Blocked", &["block", "blocked", "blacklist", "ban", "黑名单", "已屏蔽", "屏蔽", "拉黑"]),
            ("privacy_sessions", "Active Sessions & Devices", SettingsSectionType::ActiveSessions, "Settings > Privacy > Devices", &["session", "device", "terminate", "login", "活跃会话", "设备", "在线设备", "登录"]),
            ("privacy_passcode", "Local Passcode Lock", SettingsSectionType::LocalPasscode, "Settings > Privacy", &["lock", "passcode", "pin", "autolock", "本地密码", "锁屏", "密码锁", "自动锁定"]),
            ("privacy_ttl", "Auto-Delete Messages (Global TTL)", SettingsSectionType::GlobalTtl, "Settings > Privacy > Auto-Delete", &["ttl", "autodelete", "auto-delete", "timer", "自动删除", "定时销毁", "阅后即焚"]),
            ("privacy_passkeys", "Passkeys", SettingsSectionType::Passkeys, "Settings > Privacy", &["fido", "webauthn", "biometric", "passkey", "通行密钥", "生物识别", "指纹"]),
            ("privacy_websites", "Connected Websites", SettingsSectionType::Websites, "Settings > Privacy > Websites", &["website", "bot", "oauth", "connected", "授权网站", "网站登录"]),
            ("chat_theme", "Chat Appearance & Theme", SettingsSectionType::Chat, "Settings > Chat Settings", &["theme", "dark", "night", "font", "wallpaper", "enter", "聊天设置", "主题", "深色", "夜间", "字体", "壁纸", "回车发送"]),
            ("folders_manage", "Chat Folders", SettingsSectionType::Folders, "Settings > Folders", &["folder", "filter", "tab", "聊天文件夹", "分组", "标签"]),
            ("advanced_main", "Advanced & Connection", SettingsSectionType::Advanced, "Settings > Advanced", &["advanced", "proxy", "socks5", "download", "export", "opengl", "高级设置", "代理", "连接", "下载", "导出", "硬件加速"]),
            ("storage_cache", "Local Storage & Cache", SettingsSectionType::LocalStorage, "Settings > Advanced > Storage", &["cache", "storage", "disk", "clear", "本地存储", "缓存", "清理", "磁盘空间"]),
            ("power_saving", "Power Saving & Battery", SettingsSectionType::PowerSaving, "Settings > Advanced > Power Saving", &["power", "battery", "autoplay", "animation", "省电", "电池", "自动播放", "动画"]),
            ("shortcuts_keys", "Keyboard Shortcuts", SettingsSectionType::Shortcuts, "Settings > Shortcuts", &["shortcut", "hotkey", "keyboard", "快捷键", "热键", "键盘"]),
            ("stars_balance", "Stars Balance", SettingsSectionType::Credits, "Settings > Stars", &["stars", "credits", "balance", "topup", "wallet", "星币", "余额", "充值", "钱包"]),
            ("premium_features", "Premium", SettingsSectionType::Premium, "Settings > Premium", &["premium", "4gb", "speed", "subscription", "会员", "高级会员", "订阅", "特权"]),
            ("calls_device", "Call Audio & Video Settings", SettingsSectionType::Calls, "Settings > Calls", &["microphone", "mic", "speakers", "camera", "volume", "p2p", "通话", "音视频", "麦克风", "扬声器", "摄像头"]),
            ("business_tools", "Business Tools", SettingsSectionType::Business, "Settings > Business", &["business", "hours", "greeting", "quick reply", "商业", "企业", "快捷回复", "营业时间"]),
            ("lang_picker", "Language", SettingsSectionType::Language, "Settings > Language", &["language", "english", "chinese", "locale", "语言", "中文", "英文", "多语言"]),
        ];
        self.all_entries = rows.into_iter().map(|(id, title, section, subtitle, kw)| SearchResultItem {
            id: id.into(),
            title: title.into(),
            section,
            subtitle: subtitle.into(),
            keywords: kw.iter().map(|s| (*s).to_string()).collect(),
        }).collect();
    }

    fn localized_labels<'a>(item: &'a SearchResultItem, lang: &str) -> (&'a str, &'a str) {
        if !lang.starts_with("zh") {
            return (&item.title, &item.subtitle);
        }
        match item.id.as_str() {
            "ai_providers_magpie" => ("AI 订阅与模型供应商 (Magpie)", "设置 > AI 订阅与模型供应商"),
            "account_name" => ("编辑姓名与手机号", "账号 > 编辑资料"),
            "account_username" => ("用户名 (@Username)", "账号 > 编辑资料"),
            "account_bio" => ("个人简介 (Bio)", "账号 > 编辑资料"),
            "notif_sounds" => ("通知与声音", "设置 > 通知与声音"),
            "privacy_2fa" => ("两步验证与隐私安全", "设置 > 隐私与安全"),
            "privacy_blocked" => ("黑名单 (已屏蔽用户)", "设置 > 隐私与安全 > 黑名单"),
            "privacy_sessions" => ("活跃会话与登录设备", "设置 > 隐私与安全 > 活跃会话"),
            "privacy_passcode" => ("本地密码锁", "设置 > 隐私与安全 > 本地密码"),
            "privacy_ttl" => ("全局自动删除消息", "设置 > 隐私与安全 > 自动删除"),
            "privacy_passkeys" => ("通行密钥 (Passkeys)", "设置 > 隐私与安全"),
            "privacy_websites" => ("已授权网站", "设置 > 隐私与安全"),
            "chat_theme" => ("聊天外观、主题与回车发送", "设置 > 聊天设置"),
            "folders_manage" => ("聊天文件夹与分组", "设置 > 聊天文件夹"),
            "advanced_main" => ("高级设置、代理与导出", "设置 > 高级设置"),
            "storage_cache" => ("本地存储与缓存清理", "设置 > 高级设置 > 本地存储"),
            "power_saving" => ("省电模式与动画控制", "设置 > 高级设置 > 省电模式"),
            "shortcuts_keys" => ("键盘快捷键", "设置 > 快捷键"),
            "stars_balance" => ("星币余额与充值", "设置 > 星币"),
            "premium_features" => ("高级会员 (Premium)", "设置 > 高级会员"),
            "calls_device" => ("音视频通话与设备设置", "设置 > 通话设置"),
            "business_tools" => ("商业版工具", "设置 > 商业版"),
            "lang_picker" => ("界面语言 (Language)", "设置 > 语言"),
            _ => (&item.title, &item.subtitle),
        }
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
                let (zh_title, zh_sub) = Self::localized_labels(item, "zh-CN");
                item.title.to_lowercase().contains(&q)
                    || item.subtitle.to_lowercase().contains(&q)
                    || zh_title.to_lowercase().contains(&q)
                    || zh_sub.to_lowercase().contains(&q)
                    || item.keywords.iter().any(|k| k.to_lowercase().contains(&q))
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
            .child(div().flex().flex_row().flex_wrap().gap(px(6.0)).children([
                ("", tr_str(&lang, "全部", "All")),
                ("privacy", tr_str(&lang, "隐私", "privacy")),
                ("theme", tr_str(&lang, "主题", "theme")),
                ("storage", tr_str(&lang, "存储", "storage")),
                ("stars", tr_str(&lang, "星币", "stars")),
                ("mic", tr_str(&lang, "通话", "mic")),
                ("language", tr_str(&lang, "语言", "language")),
            ].into_iter().map(|(q, lbl)| {
                let active = self.query == q;
                div().px(px(10.0)).py(px(4.0)).rounded_full().bg(rgb(if active { pal.accent } else { pal.card_bg })).border_1().border_color(rgb(if active { pal.accent } else { pal.border_fg }))
                    .text_xs().text_color(rgb(if active { pal.active_button_fg } else { pal.text_primary })).cursor_pointer()
                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| { this.set_query(q); cx.notify(); })).child(lbl.to_string())
            })))
            .child(if self.results.is_empty() {
                settings_empty_state(IconName::Search, tr_str(&lang, "未找到匹配设置", "No matching settings found."), pal).into_any_element()
            } else {
                settings_card(pal.clone()).children(self.results.iter().enumerate().map(|(idx, res)| {
                    let is_sel = idx == sel_idx;
                    let sec = res.section;
                    let rid = res.id.clone();
                    let (disp_title, disp_sub) = Self::localized_labels(res, &lang);
                    div().flex().flex_col().px(px(10.0)).py(px(8.0)).rounded(px(8.0))
                        .bg(rgb(if is_sel { pal.dialogs_hover_bg } else { pal.card_bg })).cursor_pointer()
                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                            this.selected_index = idx;
                            this.pending_selected_section = Some(sec);
                            this.bump_recent_entry(&rid);
                            cx.notify();
                        }))
                        .child(div().text_sm().font_weight(FontWeight::MEDIUM).text_color(rgb(pal.text_primary)).child(disp_title.to_string()))
                        .child(div().text_xs().text_color(rgb(pal.text_secondary)).child(disp_sub.to_string()))
                })).into_any_element()
            })
            .into_any_element()
    }
}
