use gpui::*;
use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::settings::settings_theme_ui::{
    settings_card, settings_container, settings_section_header, settings_subsection_title,
};

pub const K_MEGABYTE: u64 = 1024 * 1024;
pub const K_CHART_PARTS_COUNT: usize = 6;
pub const K_TOTAL_SIZE_LIMITS_COUNT: usize = 18;

#[derive(Debug, Clone)]
pub struct StorageCategorySize {
    pub name: String,
    pub bytes: u64,
    pub color_rgb: u32,
}

pub struct SettingsLocalStorageSections {
    pub categories: [StorageCategorySize; 6],
    pub max_cache_gb: u64,
    pub retention_weeks: u32,
    pub is_clearing: bool,
    pub is_visible: bool,
    pub content_ready: bool,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
    pub pending_clear_cache: bool,
    pub pending_prefs_sync: bool,
}

impl SettingsLocalStorageSections {
    pub fn format_bytes(bytes: u64) -> String {
        const KB: u64 = 1024;
        const GB: u64 = 1024 * 1024 * 1024;
        if bytes == 0 {
            "0 B".to_string()
        } else if bytes < KB {
            format!("{} B", bytes)
        } else if bytes < K_MEGABYTE {
            format!("{:.1} KB", bytes as f64 / KB as f64)
        } else if bytes < GB {
            if bytes % K_MEGABYTE == 0 {
                format!("{} MB", bytes / K_MEGABYTE)
            } else {
                format!("{:.1} MB", bytes as f64 / K_MEGABYTE as f64)
            }
        } else {
            format!("{:.2} GB", bytes as f64 / GB as f64)
        }
    }

    pub fn new() -> Self {
        Self {
            categories: [
                StorageCategorySize { name: "Photos".into(), bytes: 120 * K_MEGABYTE, color_rgb: 0x4ba3ff },
                StorageCategorySize { name: "Videos".into(), bytes: 450 * K_MEGABYTE, color_rgb: 0xff5a5f },
                StorageCategorySize { name: "Files".into(), bytes: 210 * K_MEGABYTE, color_rgb: 0x4caf50 },
                StorageCategorySize { name: "Voice Messages".into(), bytes: 35 * K_MEGABYTE, color_rgb: 0xffb400 },
                StorageCategorySize { name: "Stickers & Emoji".into(), bytes: 48 * K_MEGABYTE, color_rgb: 0x9c27b0 },
                StorageCategorySize { name: "Other".into(), bytes: 15 * K_MEGABYTE, color_rgb: 0x8a909d },
            ],
            max_cache_gb: 16,
            retention_weeks: 4,
            is_clearing: false,
            is_visible: false,
            content_ready: false,
            palette: TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x3390ec, 14, 12),
            language_id: "en".into(),
            pending_clear_cache: false,
            pending_prefs_sync: false,
        }
    }

    pub fn new_live(palette: TdesktopThemePalette, language_id: &str) -> Self {
        Self {
            categories: [
                StorageCategorySize { name: "Photos".into(), bytes: 0, color_rgb: 0x4ba3ff },
                StorageCategorySize { name: "Videos".into(), bytes: 0, color_rgb: 0xff5a5f },
                StorageCategorySize { name: "Files".into(), bytes: 0, color_rgb: 0x4caf50 },
                StorageCategorySize { name: "Voice Messages".into(), bytes: 0, color_rgb: 0xffb400 },
                StorageCategorySize { name: "Stickers & Emoji".into(), bytes: 0, color_rgb: 0x9c27b0 },
                StorageCategorySize { name: "Other".into(), bytes: 0, color_rgb: 0x8a909d },
            ],
            max_cache_gb: 16,
            retention_weeks: 4,
            is_clearing: false,
            is_visible: true,
            content_ready: true,
            palette,
            language_id: language_id.into(),
            pending_clear_cache: false,
            pending_prefs_sync: false,
        }
    }

    pub fn sync_category_bytes(&mut self, bytes_by_slot: [u64; 6]) {
        for (cat, b) in self.categories.iter_mut().zip(bytes_by_slot.into_iter()) { cat.bytes = b; }
        self.content_ready = true;
        self.is_clearing = false;
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) { self.palette = palette; }
    pub fn set_language(&mut self, language_id: impl Into<String>) { self.language_id = language_id.into(); }
    pub fn toggle_visibility(&mut self) { self.is_visible = !self.is_visible; }
    pub fn setup_content(&mut self) { self.content_ready = true; }
    pub fn lng_weeks(&self) -> String { format!("{} weeks", self.retention_weeks) }
    pub fn lng_local_storage_voice_title(&self) -> &'static str { "Voice & Video Messages" }
    pub fn show_finished(&mut self) { self.is_clearing = false; }
    pub fn lng_local_storage_sticker_title(&self) -> &'static str { "Stickers & Emoji" }

    pub fn start_clearing(&mut self) {
        self.is_clearing = true;
        self.pending_clear_cache = true;
        for cat in &mut self.categories { cat.bytes = 0; }
    }

    pub fn total_bytes(&self) -> u64 { self.categories.iter().map(|c| c.bytes).sum() }
}

impl Default for SettingsLocalStorageSections {
    fn default() -> Self { Self::new() }
}

impl Render for SettingsLocalStorageSections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette.clone();
        let lang = self.language_id.clone();
        let total = self.total_bytes();
        let total_display = Self::format_bytes(total);
        let categories = self.categories.clone();

        let quota_desc = if lang.starts_with("zh") {
            format!("上限: {} GB · 保留: {} 周", self.max_cache_gb, self.retention_weeks)
        } else {
            format!("Limit: {} GB · Keep: {}", self.max_cache_gb, self.lng_weeks())
        };

        settings_container(pal.clone())
            .child(settings_section_header(IconName::FileDoc, tr_str(&lang, "本地存储", "Local Storage"), Some(tr_str(&lang, "管理媒体缓存、清理周期与最大磁盘配额", "Manage media cache, retention, and maximum disk quota")), pal.clone()))
            .child(
                settings_card(pal.clone())
                    .child(div().flex().flex_row().justify_between().items_center()
                        .child(div().flex().flex_col()
                            .child(div().text_sm().font_weight(FontWeight::SEMIBOLD).text_color(rgb(pal.text_primary)).child(tr_str(&lang, "已缓存数据总量", "Total Cached Data")))
                            .child(div().text_xs().text_color(rgb(pal.text_secondary)).child(quota_desc)))
                        .child(div().text_lg().font_weight(FontWeight::BOLD).text_color(rgb(pal.accent)).child(total_display)))
                    .child(div().flex().flex_row().h(px(8.0)).w_full().rounded_full().overflow_hidden().bg(rgb(pal.input_bg))
                        .children(categories.iter().filter(|c| c.bytes > 0).map(|cat| {
                            let pct = if total > 0 { ((cat.bytes as f32 / total as f32) * 100.0).max(4.0) } else { 0.0 };
                            div().h_full().w(relative(pct / 100.0)).bg(rgb(cat.color_rgb))
                        })))
                    .child(div().flex().justify_end().child(
                        div().px(px(14.0)).py(px(7.0)).rounded(px(8.0)).bg(rgb(if total == 0 { pal.input_bg } else { pal.danger })).text_xs().font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(if total == 0 { pal.text_secondary } else { pal.active_button_fg })).cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| { this.start_clearing(); cx.notify(); }))
                            .child(if self.is_clearing { tr_str(&lang, "缓存已清空", "Cache Cleared") } else { tr_str(&lang, "清空全部缓存", "Clear Entire Cache") }),
                    )),
            )
            .child(settings_subsection_title(tr_str(&lang, "存储分类统计", "Storage Breakdown"), pal.clone()))
            .child(settings_card(pal.clone()).children(categories.into_iter().map(|cat| {
                let size_label = Self::format_bytes(cat.bytes);
                let cat_label = if lang.starts_with("zh") {
                    match cat.name.as_str() {
                        "Photos" => "照片".to_string(),
                        "Videos" => "视频".to_string(),
                        "Files" => "文件与文档".to_string(),
                        "Voice Messages" => "语音与视频消息".to_string(),
                        "Stickers & Emoji" => "贴纸与表情".to_string(),
                        "Other" => "其他缓存".to_string(),
                        _ => cat.name,
                    }
                } else {
                    cat.name
                };
                div().flex().flex_row().justify_between().items_center().px(px(10.0)).py(px(6.0))
                    .child(div().flex().flex_row().gap(px(10.0)).items_center()
                        .child(div().size(px(10.0)).rounded_full().bg(rgb(cat.color_rgb)))
                        .child(div().text_sm().text_color(rgb(pal.text_primary)).child(cat_label)))
                    .child(div().text_xs().text_color(rgb(pal.text_secondary)).child(size_label))
            })))
            .child(settings_subsection_title(tr_str(&lang, "最大缓存上限", "Maximum Cache Size"), pal.clone()))
            .child(div().flex().flex_row().gap(px(8.0)).children([5_u64, 16, 32, 64].into_iter().map(|gb| {
                let active = self.max_cache_gb == gb;
                div().flex_1().py(px(8.0)).rounded(px(8.0)).flex().items_center().justify_center()
                    .bg(rgb(if active { pal.accent } else { pal.card_bg })).border_1().border_color(rgb(if active { pal.accent } else { pal.border_fg }))
                    .text_xs().font_weight(FontWeight::SEMIBOLD).text_color(rgb(if active { pal.active_button_fg } else { pal.text_primary })).cursor_pointer()
                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                        this.max_cache_gb = gb;
                        this.pending_prefs_sync = true;
                        cx.notify();
                    }))
                    .child(format!("{} GB", gb))
            })))
            .child(settings_subsection_title(tr_str(&lang, "自动清理周期", "Cache Retention Period"), pal.clone()))
            .child(div().flex().flex_row().gap(px(8.0)).children([1_u32, 2, 4, 12].into_iter().map(|wk| {
                let active = self.retention_weeks == wk;
                div().flex_1().py(px(8.0)).rounded(px(8.0)).flex().items_center().justify_center()
                    .bg(rgb(if active { pal.accent } else { pal.card_bg })).border_1().border_color(rgb(if active { pal.accent } else { pal.border_fg }))
                    .text_xs().font_weight(FontWeight::SEMIBOLD).text_color(rgb(if active { pal.active_button_fg } else { pal.text_primary })).cursor_pointer()
                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                        this.retention_weeks = wk;
                        this.pending_prefs_sync = true;
                        cx.notify();
                    }))
                    .child(if lang.starts_with("zh") { format!("{} 周", wk) } else { format!("{} wk", wk) })
            })))
    }
}
