//! macOS Window Titlebar and Traffic Light Geometry
//! Upstream Source: Telegram/SourceFiles/platform/mac/window_title_mac.mm
//! Subsystem: platform | Feature ID: TCG-N46 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui_telegram_core::error::Result;
use serde::{Deserialize, Serialize};

/// System button color definition matching macOS titlebar palette.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemButtonColors {
    pub fill: u32,
    pub border: u32,
}

/// macOS Window Titlebar preview and geometry engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowTitleMacMac {
    pub title: String,
    pub title_height: i32,
    pub button_size: i32,
    pub button_skip: i32,
    pub aligns_left: bool,
    pub is_graphite: bool,
    pub is_dark_mode: bool,
}

impl WindowTitleMacMac {
    pub const DEFAULT_TITLE_HEIGHT: i32 = 22;
    pub const DEFAULT_BUTTON_SIZE: i32 = 12;
    pub const DEFAULT_BUTTON_SKIP: i32 = 8;

    pub fn new() -> Self {
        Self {
            title: "Telegram".into(),
            title_height: Self::DEFAULT_TITLE_HEIGHT,
            button_size: Self::DEFAULT_BUTTON_SIZE,
            button_skip: Self::DEFAULT_BUTTON_SKIP,
            aligns_left: false,
            is_graphite: false,
            is_dark_mode: false,
        }
    }

    pub fn preview_title_height(&self) -> i32 {
        self.title_height
    }

    pub fn close_button_colors(&self) -> SystemButtonColors {
        if self.is_graphite {
            SystemButtonColors { fill: 0x8d8d92, border: 0x68686d }
        } else {
            SystemButtonColors { fill: 0xfc605c, border: 0xde403b }
        }
    }

    pub fn minimize_button_colors(&self) -> SystemButtonColors {
        if self.is_graphite {
            SystemButtonColors { fill: 0x8d8d92, border: 0x68686d }
        } else {
            SystemButtonColors { fill: 0xfec041, border: 0xdd9819 }
        }
    }

    pub fn maximize_button_colors(&self) -> SystemButtonColors {
        if self.is_graphite {
            SystemButtonColors { fill: 0x8d8d92, border: 0x68686d }
        } else {
            SystemButtonColors { fill: 0x34c84a, border: 0x27ae38 }
        }
    }

    pub fn set_graphite_tint(&mut self, graphite: bool) -> Result<()> {
        self.is_graphite = graphite;
        Ok(())
    }

    pub fn set_alignment(&mut self, left: bool) -> Result<()> {
        self.aligns_left = left;
        Ok(())
    }
}

impl Default for WindowTitleMacMac {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowTitleMacMac {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let close = self.close_button_colors();
        let min = self.minimize_button_colors();
        let max = self.maximize_button_colors();
        let title = self.title.clone();
        let left_align = self.aligns_left;

        let bg_color = if self.is_dark_mode { rgb(0x1e1e1e) } else { rgb(0xebebeb) };
        let text_color = if self.is_dark_mode { rgb(0xffffff) } else { rgb(0x222222) };

        div()
            .h(px(self.title_height as f32))
            .w_full()
            .bg(bg_color)
            .flex()
            .flex_row()
            .items_center()
            .px_3()
            .child(
                // Traffic lights container
                div()
                    .flex()
                    .flex_row()
                    .gap_2()
                    .child(
                        div()
                            .w(px(self.button_size as f32))
                            .h(px(self.button_size as f32))
                            .rounded_full()
                            .bg(rgb(close.fill))
                            .border_1()
                            .border_color(rgb(close.border)),
                    )
                    .child(
                        div()
                            .w(px(self.button_size as f32))
                            .h(px(self.button_size as f32))
                            .rounded_full()
                            .bg(rgb(min.fill))
                            .border_1()
                            .border_color(rgb(min.border)),
                    )
                    .child(
                        div()
                            .w(px(self.button_size as f32))
                            .h(px(self.button_size as f32))
                            .rounded_full()
                            .bg(rgb(max.fill))
                            .border_1()
                            .border_color(rgb(max.border)),
                    ),
            )
            .child({
                let mut title_div = div()
                    .flex_1()
                    .text_xs()
                    .font_weight(FontWeight::BOLD)
                    .text_color(text_color);
                if left_align {
                    title_div = title_div.pl_4();
                } else {
                    title_div = title_div.text_center();
                }
                title_div.child(title)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_title_colors_and_metrics() {
        let mut title = WindowTitleMacMac::new();
        assert_eq!(title.preview_title_height(), 22);

        let c_norm = title.close_button_colors();
        assert_eq!(c_norm.fill, 0xfc605c);

        title.set_graphite_tint(true).unwrap();
        let c_graph = title.close_button_colors();
        assert_eq!(c_graph.fill, 0x8d8d92);
        assert_eq!(c_graph.border, 0x68686d);
    }
}
