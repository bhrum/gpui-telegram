//! Upstream Source: Telegram/SourceFiles/settings/settings_builder.cpp, Telegram/SourceFiles/settings/settings_builder.h
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui::prelude::FluentBuilder;
use super::settings_type::SettingsSectionType;

#[derive(Debug, Clone)]
pub struct SettingsSearchEntry {
    pub id: String,
    pub title: String,
    pub keywords: Vec<String>,
    pub section: SettingsSectionType,
    pub deeplink: Option<String>,
}

#[derive(Debug, Clone)]
pub enum SettingsBuilderItem {
    Button {
        id: String,
        title: String,
        label: Option<String>,
        icon: Option<String>,
        keywords: Vec<String>,
        is_toggled: Option<bool>,
    },
    SectionButton {
        title: String,
        target_section: SettingsSectionType,
        icon: Option<String>,
        keywords: Vec<String>,
    },
    SubsectionTitle {
        title: String,
    },
    Divider,
    Skip {
        height: f32,
    },
}

#[derive(Debug, Clone, Default)]
pub struct SettingsSearchRegistry {
    pub entries: Vec<SettingsSearchEntry>,
}

impl SettingsSearchRegistry {
    pub fn search(&self, query: &str) -> Vec<SettingsSearchEntry> {
        let q = query.to_lowercase();
        self.entries
            .iter()
            .filter(|e| e.title.to_lowercase().contains(&q) || e.keywords.iter().any(|k| k.to_lowercase().contains(&q)))
            .cloned()
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct SettingsBuilder {
    pub title: String,
    pub items: Vec<SettingsBuilderItem>,
    pub search_registry: SettingsSearchRegistry,
    pub is_visible: bool,
}

impl SettingsBuilder {
    pub fn new() -> Self {
        Self {
            title: "SettingsBuilder".into(),
            items: Vec::new(),
            search_registry: SettingsSearchRegistry::default(),
            is_visible: true,
        }
    }

    pub fn add_button(
        &mut self,
        id: impl Into<String>,
        title: impl Into<String>,
        label: Option<&str>,
        icon: Option<&str>,
    ) -> &mut Self {
        let sid = id.into();
        let stitle = title.into();
        self.search_registry.entries.push(SettingsSearchEntry {
            id: sid.clone(),
            title: stitle.clone(),
            keywords: vec![stitle.clone()],
            section: SettingsSectionType::Main,
            deeplink: None,
        });
        self.items.push(SettingsBuilderItem::Button {
            id: sid,
            title: stitle,
            label: label.map(String::from),
            icon: icon.map(String::from),
            keywords: Vec::new(),
            is_toggled: None,
        });
        self
    }

    pub fn add_section_button(
        &mut self,
        id: impl Into<String>,
        title: impl Into<String>,
        target: SettingsSectionType,
        icon: Option<&str>,
    ) -> &mut Self {
        let sid = id.into();
        let stitle = title.into();
        self.search_registry.entries.push(SettingsSearchEntry {
            id: sid,
            title: stitle.clone(),
            keywords: vec![stitle.clone()],
            section: target,
            deeplink: None,
        });
        self.items.push(SettingsBuilderItem::SectionButton {
            title: stitle,
            target_section: target,
            icon: icon.map(String::from),
            keywords: Vec::new(),
        });
        self
    }

    pub fn add_subsection_title(&mut self, title: impl Into<String>) -> &mut Self {
        let stitle = title.into();
        self.search_registry.entries.push(SettingsSearchEntry {
            id: stitle.clone(),
            title: stitle.clone(),
            keywords: vec![stitle.clone()],
            section: SettingsSectionType::Main,
            deeplink: None,
        });
        self.items.push(SettingsBuilderItem::SubsectionTitle {
            title: stitle,
        });
        self
    }

    pub fn add_divider(&mut self) -> &mut Self {
        self.items.push(SettingsBuilderItem::Divider);
        self
    }

    pub fn add_skip(&mut self, height: f32) -> &mut Self {
        self.items.push(SettingsBuilderItem::Skip { height });
        self
    }

    pub fn collect_search_entries(&self, section: SettingsSectionType) -> Vec<SettingsSearchEntry> {
        let mut entries = Vec::new();
        for item in &self.items {
            match item {
                SettingsBuilderItem::Button { id, title, keywords, .. } => {
                    entries.push(SettingsSearchEntry {
                        id: id.clone(),
                        title: title.clone(),
                        keywords: keywords.clone(),
                        section,
                        deeplink: None,
                    });
                }
                SettingsBuilderItem::SectionButton { title, target_section, keywords, .. } => {
                    entries.push(SettingsSearchEntry {
                        id: format!("section_{}", target_section.id_str()),
                        title: title.clone(),
                        keywords: keywords.clone(),
                        section: *target_section,
                        deeplink: None,
                    });
                }
                _ => {}
            }
        }
        entries
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }
}

impl Default for SettingsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsBuilder {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }

        div()
            .flex()
            .flex_col()
            .w_full()
            .children(self.items.iter().map(|item| match item {
                SettingsBuilderItem::SubsectionTitle { title } => div()
                    .px(px(16.0))
                    .pt(px(16.0))
                    .pb(px(6.0))
                    .text_xs()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x5288c1))
                    .child(title.clone()),
                SettingsBuilderItem::Button { title, label, icon, .. } => div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .px(px(16.0))
                    .py(px(12.0))
                    .bg(rgb(0x17212b))
                    .hover(|s| s.bg(rgb(0x232e3c)))
                    .cursor_pointer()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(12.0))
                            .when_some(icon.clone(), |this, ic| {
                                this.child(
                                    div()
                                        .size(px(20.0))
                                        .text_sm()
                                        .text_color(rgb(0x6c7883))
                                        .child(ic),
                                )
                            })
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0xffffff))
                                    .child(title.clone()),
                            ),
                    )
                    .when_some(label.clone(), |this, lb| {
                        this.child(
                            div()
                                .text_xs()
                                .text_color(rgb(0x6c7883))
                                .child(lb),
                        )
                    }),
                SettingsBuilderItem::SectionButton { title, target_section: _, icon, .. } => div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .px(px(16.0))
                    .py(px(12.0))
                    .bg(rgb(0x17212b))
                    .hover(|s| s.bg(rgb(0x232e3c)))
                    .cursor_pointer()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(12.0))
                            .when_some(icon.clone(), |this, ic| {
                                this.child(
                                    div()
                                        .size(px(20.0))
                                        .text_sm()
                                        .text_color(rgb(0x6c7883))
                                        .child(ic),
                                )
                            })
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0xffffff))
                                    .child(title.clone()),
                            ),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x6c7883))
                            .child("›"),
                    ),
                SettingsBuilderItem::Divider => div()
                    .h(px(1.0))
                    .w_full()
                    .bg(rgb(0x0e1621)),
                SettingsBuilderItem::Skip { height } => div().h(px(*height)),
            }))
            .into_any_element()
    }
}
