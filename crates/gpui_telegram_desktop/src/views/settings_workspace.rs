//! Complete 1:1 Telegram Desktop (`tdesktop`) Multi-Section Settings Workspace.

use crate::i18n::tr_str;
use crate::theme::assets::{IconName, tg_icon};
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::settings::workspace_pages_a::{
    render_account_page, render_chat_appearance_page, render_notifications_page,
    render_privacy_page,
};
use crate::views::settings::workspace_pages_b::{
    render_advanced_storage_page, render_calls_page, render_folders_page, render_language_page,
};
use crate::views::settings::workspace_pages_c::{
    render_business_passport_page, render_premium_stars_page, render_ton_wallet_page,
};
use gpui::*;
use gpui_telegram_client::PersistedClientState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsPageId {
    Account,
    Notifications,
    PrivacySecurity,
    ChatAppearance,
    Folders,
    AdvancedStorage,
    CallDevices,
    LanguageTranslate,
    PremiumStars,
    TonWallet,
    BusinessPassport,
}

#[derive(Debug, Clone)]
pub enum SettingsMutation {
    CloseSettings,
    LogOut,
    SetThemeMode(TdesktopThemeMode),
    SetAccentColor(u32),
    SetWallpaper(String),
    SetFontSize(u32),
    SetBubbleRadius(u32),
    ToggleSendWithEnter,
    SetLanguage(String),
    ToggleTranslateBar,
    AddCustomFolder(String),
    RemoveCustomFolder(String),
    ClearLocalCache,
    OpenProxyModal,
    OpenPasscodeModal,
    OpenExportModal,
    LockNow,
    RevokeSession(String),
    BuyOrTopUpStars(u64),
    SendTonTransfer,
    UpdateBio(String),
}

pub struct SettingsWorkspaceView {
    pub active_page: SettingsPageId,
    pub search_filter: String,
    pub new_folder_input: String,
    pub bio_edit_input: String,
    pub editing_field: u8,
    pub notify_private: bool,
    pub notify_groups: bool,
    pub notify_channels: bool,
    pub notify_sound: bool,
    pub call_noise_suppression: bool,
    pub call_p2p_contacts_only: bool,
    pub business_hours_enabled: bool,
    pub business_quick_replies: usize,
    pub passport_verified_docs: usize,
    pub selected_wallpaper_id: String,
    pub state_snapshot: PersistedClientState,
    pub palette: TdesktopThemePalette,
    pub pending_mutation: Option<SettingsMutation>,
}

impl SettingsWorkspaceView {
    pub fn new(state_snapshot: PersistedClientState, palette: TdesktopThemePalette) -> Self {
        let bio = state_snapshot
            .accounts
            .iter()
            .find(|a| a.account_id == state_snapshot.active_account_id)
            .map(|a| a.bio.clone())
            .unwrap_or_default();
        Self {
            active_page: SettingsPageId::ChatAppearance,
            search_filter: String::new(),
            new_folder_input: String::new(),
            bio_edit_input: bio,
            editing_field: 0,
            notify_private: true,
            notify_groups: true,
            notify_channels: false,
            notify_sound: true,
            call_noise_suppression: true,
            call_p2p_contacts_only: true,
            business_hours_enabled: false,
            business_quick_replies: 0,
            passport_verified_docs: 0,
            selected_wallpaper_id: "default_night".to_string(),
            state_snapshot,
            palette,
            pending_mutation: None,
        }
    }

    pub fn sync_from_state(&mut self, state: PersistedClientState, palette: TdesktopThemePalette) {
        self.state_snapshot = state;
        self.palette = palette;
    }

    pub fn handle_key_input(&mut self, key: &str, key_char: Option<&str>) -> bool {
        if self.editing_field == 0 {
            return false;
        }
        let target = match self.editing_field {
            1 => &mut self.search_filter,
            2 => &mut self.new_folder_input,
            _ => &mut self.bio_edit_input,
        };
        if key == "backspace" { target.pop(); return true; }
        if key == "space" { target.push(' '); return true; }
        if key == "enter" {
            if self.editing_field == 3 {
                self.pending_mutation = Some(SettingsMutation::UpdateBio(self.bio_edit_input.clone()));
            }
            self.editing_field = 0;
            return true;
        }
        if let Some(ch) = key_char.filter(|c| !c.chars().any(char::is_control)) {
            target.push_str(ch);
            return true;
        }
        false
    }
}

impl Render for SettingsWorkspaceView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;
        let lang = self.state_snapshot.language_id.clone();
        let cur_page = self.active_page;
        let q = self.search_filter.to_lowercase();

        let nav_entries = [
            (SettingsPageId::Account, IconName::ContactsUsers, "我的账号与资料", "My Account"),
            (SettingsPageId::Notifications, IconName::Speaker, "通知与声音", "Notifications and Sounds"),
            (SettingsPageId::PrivacySecurity, IconName::Lock, "隐私与安全", "Privacy and Security"),
            (SettingsPageId::ChatAppearance, IconName::NightMoon, "聊天外观与主题", "Chat Settings"),
            (SettingsPageId::Folders, IconName::FolderAll, "聊天分组", "Chat Folders"),
            (SettingsPageId::AdvancedStorage, IconName::SettingsGear, "高级与存储", "Advanced & Storage"),
            (SettingsPageId::CallDevices, IconName::PhoneCall, "通话与音视频", "Calls"),
            (SettingsPageId::LanguageTranslate, IconName::Translate, "语言与翻译", "Language"),
            (SettingsPageId::PremiumStars, IconName::PremiumStar, "Premium 与星币", "Telegram Premium"),
            (SettingsPageId::TonWallet, IconName::WalletStars, "TON 钱包", "TON Wallet"),
            (SettingsPageId::BusinessPassport, IconName::BotRobot, "商业版与 Passport", "Business & Passport"),
        ];

        let detail = match cur_page {
            SettingsPageId::Account => render_account_page(self, pal, &lang, cx),
            SettingsPageId::Notifications => render_notifications_page(self, pal, &lang, cx),
            SettingsPageId::PrivacySecurity => render_privacy_page(self, pal, &lang, cx),
            SettingsPageId::ChatAppearance => render_chat_appearance_page(self, pal, &lang, cx),
            SettingsPageId::Folders => render_folders_page(self, pal, &lang, cx),
            SettingsPageId::AdvancedStorage => render_advanced_storage_page(self, pal, &lang, cx),
            SettingsPageId::CallDevices => render_calls_page(self, pal, &lang, cx),
            SettingsPageId::LanguageTranslate => render_language_page(self, pal, &lang, cx),
            SettingsPageId::PremiumStars => render_premium_stars_page(self, pal, &lang, cx),
            SettingsPageId::TonWallet => render_ton_wallet_page(self, pal, &lang, cx),
            SettingsPageId::BusinessPassport => render_business_passport_page(self, pal, &lang, cx),
        };

        div().flex_1().min_w(px(0.0)).h_full().bg(rgb(pal.window_bg)).flex()
            .child(div().w(px(264.0)).flex_shrink_0().h_full().bg(rgb(pal.dialogs_bg)).border_r_1().border_color(rgb(pal.border_fg)).flex().flex_col()
                .child(div().h(px(52.0)).px(px(14.0)).border_b_1().border_color(rgb(pal.border_fg)).flex().items_center().justify_between()
                    .child(div().flex().items_center().gap(px(8.0)).child(tg_icon(IconName::SettingsGear, 17.0, pal.accent))
                        .child(div().text_size(px(15.0)).font_weight(FontWeight::BOLD).text_color(rgb(pal.text_primary)).child(tr_str(&lang, "设置", "Settings").to_string())))
                    .child(div().cursor_pointer().px(px(8.0)).py(px(4.0)).rounded(px(6.0)).bg(rgb(pal.accent)).flex().items_center().gap(px(4.0))
                        .text_size(px(11.5)).font_weight(FontWeight::BOLD).text_color(rgb(pal.active_button_fg))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| { this.pending_mutation = Some(SettingsMutation::CloseSettings); cx.notify(); }))
                        .child(tg_icon(IconName::BackArrow, 12.0, pal.active_button_fg)).child(tr_str(&lang, "返回", "Back").to_string())))
                .child(div().p(px(10.0)).child(div().cursor_pointer().h(px(32.0)).px(px(10.0)).rounded(px(16.0)).bg(rgb(pal.input_bg))
                    .flex().items_center().gap(px(6.0)).text_size(px(12.0)).text_color(rgb(pal.text_secondary))
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| { this.editing_field = 1; cx.notify(); }))
                    .child(tg_icon(IconName::Search, 13.0, pal.icon_fg))
                    .child(if self.search_filter.is_empty() { tr_str(&lang, "搜索设置...", "Search settings...").to_string() } else { self.search_filter.clone() })))
                .child(div().id("settings-left-nav-scroll").flex_1().min_h(px(0.0)).overflow_y_scroll().flex().flex_col()
                    .children(nav_entries.into_iter().filter(|(_, _, z, e)| q.is_empty() || z.to_lowercase().contains(&q) || e.to_lowercase().contains(&q)).map(|(pid, icon, z, e)| {
                        let sel = pid == cur_page;
                        div().cursor_pointer().h(px(40.0)).px(px(14.0)).bg(rgb(if sel { pal.dialogs_active_bg } else { pal.dialogs_bg }))
                            .hover(move |s| if sel { s } else { s.bg(rgb(pal.dialogs_hover_bg)) }).flex().items_center().gap(px(10.0))
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _e, _w, cx| { this.active_page = pid; this.editing_field = 0; cx.notify(); }))
                            .child(tg_icon(icon, 16.0, if sel { pal.active_button_fg } else { pal.icon_fg }))
                            .child(div().text_size(px(13.0)).font_weight(if sel { FontWeight::BOLD } else { FontWeight::NORMAL })
                                .text_color(rgb(if sel { pal.active_button_fg } else { pal.text_primary })).child(tr_str(&lang, z, e).to_string()))
                    }))))
            .child(div().id("settings-right-detail-scroll").flex_1().min_w(px(0.0)).h_full().overflow_y_scroll().p(px(24.0)).child(detail))
            .into_any_element()
    }
}
