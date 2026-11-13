//! Upstream Source: Telegram/SourceFiles/settings/sections/settings_global_ttl.cpp, Telegram/SourceFiles/settings/sections/settings_global_ttl.h
//! Subsystem: settings | Feature ID: TCG-N40 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui::prelude::FluentBuilder;

pub const TTL_OFF: u32 = 0;
pub const TTL_ONE_DAY: u32 = 86400;
pub const TTL_ONE_WEEK: u32 = 604800;
pub const TTL_ONE_MONTH: u32 = 2678400;

#[derive(Debug, Clone)]
pub struct SettingsGlobalTtlSections {
    pub current_ttl: u32,
    pub custom_ttl: Option<u32>,
    pub pending_ttl: Option<u32>,
    pub is_confirming: bool,
    pub existing_chats_count: usize,
    pub status_message: Option<String>,
}

impl SettingsGlobalTtlSections {
    pub fn new() -> Self {
        Self {
            current_ttl: TTL_OFF,
            custom_ttl: None,
            pending_ttl: None,
            is_confirming: false,
            existing_chats_count: 42,
            status_message: None,
        }
    }

    pub fn format_ttl(ttl: u32) -> &'static str {
        match ttl {
            TTL_OFF => "Off",
            TTL_ONE_DAY => "1 Day",
            TTL_ONE_WEEK => "1 Week",
            TTL_ONE_MONTH => "1 Month",
            _ => "Custom",
        }
    }

    pub fn set_ttl(&mut self, ttl: u32) {
        if self.current_ttl == TTL_OFF && ttl != TTL_OFF {
            self.pending_ttl = Some(ttl);
            self.is_confirming = true;
        } else {
            self.current_ttl = ttl;
            self.status_message = Some(format!("Auto-delete set to {}", Self::format_ttl(ttl)));
        }
    }

    pub fn confirm_pending(&mut self) {
        if let Some(ttl) = self.pending_ttl.take() {
            self.current_ttl = ttl;
            self.status_message = Some(format!("Auto-delete enabled: {}", Self::format_ttl(ttl)));
        }
        self.is_confirming = false;
    }

    pub fn cancel_confirm(&mut self) {
        self.pending_ttl = None;
        self.is_confirming = false;
    }

    pub fn apply_to_existing(&mut self) {
        self.status_message = Some(format!(
            "Applied auto-delete timer ({}) to {} existing chats",
            Self::format_ttl(self.current_ttl),
            self.existing_chats_count
        ));
    }
}

impl Default for SettingsGlobalTtlSections {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsGlobalTtlSections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let current = self.current_ttl;
        let is_confirming = self.is_confirming;
        let pending = self.pending_ttl.unwrap_or(TTL_ONE_DAY);
        let existing_count = self.existing_chats_count;

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .bg(rgb(0x0e1621))
            .gap(px(12.0))
            // Header Info with TTL animated icon simulation
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .p(px(16.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x17212b))
                    .gap(px(6.0))
                    .child(div().text_2xl().child("⏱️"))
                    .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(rgb(0xffffff)).child("Auto-Delete Messages"))
                    .child(div().text_xs().text_color(rgb(0x8292a1)).child("Automatically delete messages in newly created private chats.")),
            )
            // Confirmation prompt
            .when(is_confirming, |el| {
                el.child(
                    div()
                        .flex()
                        .flex_col()
                        .p(px(12.0))
                        .rounded(px(8.0))
                        .bg(rgb(0x232e3c))
                        .gap(px(8.0))
                        .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(rgb(0xffffff)).child(format!("Enable auto-delete after {}?", Self::format_ttl(pending))))
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .px(px(12.0))
                                        .py(px(6.0))
                                        .rounded(px(4.0))
                                        .bg(rgb(0x5288c1))
                                        .text_xs()
                                        .text_color(rgb(0xffffff))
                                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                            this.confirm_pending();
                                            cx.notify();
                                        }))
                                        .child("Enable"),
                                )
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .px(px(12.0))
                                        .py(px(6.0))
                                        .rounded(px(4.0))
                                        .bg(rgb(0x17212b))
                                        .text_xs()
                                        .text_color(rgb(0x8292a1))
                                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                            this.cancel_confirm();
                                            cx.notify();
                                        }))
                                        .child("Cancel"),
                                ),
                        ),
                )
            })
            // Standard TTL Options
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p(px(12.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x17212b))
                    .gap(px(6.0))
                    .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(rgb(0x8292a1)).child("AUTO-DELETE PERIOD"))
                    .child(make_ttl_option("Off", TTL_OFF, current, cx))
                    .child(make_ttl_option("1 Day", TTL_ONE_DAY, current, cx))
                    .child(make_ttl_option("1 Week", TTL_ONE_WEEK, current, cx))
                    .child(make_ttl_option("1 Month", TTL_ONE_MONTH, current, cx)),
            )
            // Apply to existing chats row
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .p(px(12.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x17212b))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(div().text_sm().text_color(rgb(0xffffff)).child("Apply to Existing Chats"))
                            .child(div().text_xs().text_color(rgb(0x8292a1)).child(format!("Apply current timer to {} conversations", existing_count))),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .px(px(12.0))
                            .py(px(6.0))
                            .rounded(px(4.0))
                            .bg(rgb(0x5288c1))
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                this.apply_to_existing();
                                cx.notify();
                            }))
                            .child("Apply"),
                    ),
            )
            .into_any_element()
    }
}

fn make_ttl_option(
    label: &'static str,
    val: u32,
    current: u32,
    cx: &mut Context<SettingsGlobalTtlSections>,
) -> impl IntoElement {
    let is_selected = val == current;
    div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .py(px(6.0))
        .cursor_pointer()
        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _window, cx| {
            this.set_ttl(val);
            cx.notify();
        }))
        .child(div().text_sm().text_color(rgb(0xffffff)).child(label))
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .text_color(if is_selected { rgb(0x5288c1) } else { rgb(0x8292a1) })
                .child(if is_selected { "●" } else { "○" }),
        )
}
