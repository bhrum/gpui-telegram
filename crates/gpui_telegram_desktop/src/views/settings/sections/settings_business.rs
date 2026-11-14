use gpui::*;
use crate::i18n::tr_str;
use crate::theme::assets::{tg_icon, IconName};
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::settings::settings_theme_ui::{
    settings_card, settings_container, settings_section_header, settings_subsection_title,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BusinessToolItem {
    pub title: String,
    pub subtitle: String,
    pub is_configured: bool,
}

pub struct SettingsBusinessSections {
    pub business_active: bool,
    pub tools: Vec<BusinessToolItem>,
    pub is_visible: bool,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
    pub selected_tool_index: Option<usize>,
}

impl SettingsBusinessSections {
    pub fn new() -> Self {
        Self {
            business_active: true,
            tools: vec![
                BusinessToolItem { title: "Opening Hours".into(), subtitle: "Show your business schedule".into(), is_configured: false },
                BusinessToolItem { title: "Location".into(), subtitle: "Display business address".into(), is_configured: false },
                BusinessToolItem { title: "Quick Replies".into(), subtitle: "Shortcuts for frequently used messages".into(), is_configured: false },
                BusinessToolItem { title: "Greeting Messages".into(), subtitle: "Automatic welcome for new chats".into(), is_configured: false },
                BusinessToolItem { title: "Away Messages".into(), subtitle: "Auto-reply when closed".into(), is_configured: false },
                BusinessToolItem { title: "Chatbots".into(), subtitle: "Connect bots to handle messages".into(), is_configured: false },
            ],
            is_visible: false,
            palette: TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x3390ec, 14, 12),
            language_id: "en".into(),
            selected_tool_index: None,
        }
    }

    pub fn new_live(business_active: bool, palette: TdesktopThemePalette, language_id: &str) -> Self {
        let mut s = Self::new();
        s.business_active = business_active;
        s.palette = palette;
        s.language_id = language_id.into();
        s.is_visible = true;
        s
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) { self.palette = palette; }
    pub fn set_language(&mut self, language_id: impl Into<String>) { self.language_id = language_id.into(); }
    pub fn toggle_visibility(&mut self) { self.is_visible = !self.is_visible; }
    pub fn lng_business_subtitle_chatbots(&self) -> &'static str { "Automate replies via connected bots" }
    pub fn lng_business_about_opening_hours(&self) -> &'static str { "Set working hours for each day of the week" }
    pub fn lng_business_about_chat_intro(&self) -> &'static str { "Customize the welcome screen before a chat starts" }
    pub fn lng_business_subtitle_chat_links(&self) -> &'static str { "Create t.me links with pre-filled messages" }
    pub fn show_business(&mut self) { self.business_active = true; }
    pub fn lng_business_about_quick_replies(&self) -> &'static str { "Send saved templates using / commands" }
}

impl Default for SettingsBusinessSections {
    fn default() -> Self { Self::new() }
}

impl Render for SettingsBusinessSections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette.clone();
        let lang = self.language_id.clone();
        let tools = self.tools.clone();

        settings_container(pal.clone())
            .child(settings_section_header(IconName::BotRobot, tr_str(&lang, "Telegram 商业版", "Telegram Business"), Some("Tools and automation for business accounts"), pal.clone()))
            .child(settings_subsection_title(tr_str(&lang, "商业功能", "Business Tools"), pal.clone()))
            .child(settings_card(pal.clone()).children(tools.into_iter().enumerate().map(|(idx, item)| {
                let active = item.is_configured;
                div().flex().flex_row().justify_between().items_center().px(px(10.0)).py(px(8.0)).rounded(px(8.0)).cursor_pointer()
                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                        this.selected_tool_index = Some(idx);
                        if let Some(t) = this.tools.get_mut(idx) { t.is_configured = !t.is_configured; }
                        cx.notify();
                    }))
                    .child(div().flex().flex_row().items_center().gap(px(10.0))
                        .child(tg_icon(IconName::BotRobot, 16.0, pal.accent))
                        .child(div().flex().flex_col().child(div().text_sm().font_weight(FontWeight::SEMIBOLD).text_color(rgb(pal.text_primary)).child(item.title)).child(div().text_xs().text_color(rgb(pal.text_secondary)).child(item.subtitle))))
                    .child(div().text_xs().font_weight(FontWeight::SEMIBOLD).text_color(rgb(if active { pal.success } else { pal.text_link })).child(if active { "Configured" } else { "Set up" }))
            })))
    }
}
