//! Target Symbol: IvPch
//! Module 13: Instant View Precompiled Header Prelude & Subsystem Info
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub type RichBlockId = usize;
pub type MediaId = u64;
pub type ZoomLevel = i32;

#[derive(Debug, Clone)]
pub struct IvSubsystemCapabilities {
    pub supports_math_latex: bool,
    pub supports_ton_sites: bool,
    pub supports_html_export: bool,
    pub supports_rich_quotes: bool,
    pub supports_nested_tables: bool,
}

impl Default for IvSubsystemCapabilities {
    fn default() -> Self {
        Self {
            supports_math_latex: true,
            supports_ton_sites: true,
            supports_html_export: true,
            supports_rich_quotes: true,
            supports_nested_tables: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IvPch {
    pub subsystem_name: &'static str,
    pub version_major: u32,
    pub version_minor: u32,
    pub capabilities: IvSubsystemCapabilities,
}

impl Default for IvPch {
    fn default() -> Self {
        Self {
            subsystem_name: "Instant View Subsystem (GPUI Native)",
            version_major: 2,
            version_minor: 1,
            capabilities: IvSubsystemCapabilities::default(),
        }
    }
}

impl IvPch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn version_string(&self) -> String {
        format!("v{}.{}", self.version_major, self.version_minor)
    }

    pub fn is_feature_supported(&self, feature: &str) -> bool {
        match feature.to_lowercase().as_str() {
            "math" | "latex" => self.capabilities.supports_math_latex,
            "ton" => self.capabilities.supports_ton_sites,
            "export" | "html" => self.capabilities.supports_html_export,
            "quotes" => self.capabilities.supports_rich_quotes,
            "tables" => self.capabilities.supports_nested_tables,
            _ => false,
        }
    }
}

impl Render for IvPch {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .p(px(8.0))
            .bg(rgb(0x1B2028))
            .rounded(px(6.0))
            .gap(px(2.0))
            .child(
                div()
                    .text_size(px(12.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xCCD2DE))
                    .child(format!("{} {}", self.subsystem_name, self.version_string())),
            )
            .child(
                div()
                    .text_size(px(11.0))
                    .text_color(rgb(0x757F90))
                    .child("LaTeX, TON Sites, Offline HTML Export, Rich Quotes"),
            )
    }
}
