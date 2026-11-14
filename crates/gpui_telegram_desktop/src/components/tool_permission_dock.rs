use crate::i18n::tr_str;
use crate::theme::{assets::{IconName, tg_icon}, TdesktopThemePalette};
use gpui::prelude::FluentBuilder;
use gpui::*;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolApprovalDecision {
    AllowOnce,
    AllowAlways,
    Deny,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingToolApproval {
    pub id: String,
    pub tool_name: String,
    pub summary: String,
    pub arguments_preview: String,
    pub risk_level: String,
    pub created_at: String,
}

pub struct ToolPermissionDock {
    pub pending_count: usize,
    pub pending_requests: Vec<PendingToolApproval>,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
    pub last_decision: Option<(String, ToolApprovalDecision)>,
    pub on_decision: Option<Arc<dyn Fn(String, ToolApprovalDecision) + Send + Sync>>,
}

impl ToolPermissionDock {
    pub fn new(pending_count: usize) -> Self {
        Self {
            pending_count,
            pending_requests: Vec::new(),
            palette: TdesktopThemePalette::default(),
            language_id: "en".to_string(),
            last_decision: None,
            on_decision: None,
        }
    }

    pub fn with_palette(mut self, palette: TdesktopThemePalette, language_id: impl Into<String>) -> Self {
        self.palette = palette;
        self.language_id = language_id.into();
        self
    }

    pub fn sync_requests(&mut self, requests: Vec<PendingToolApproval>) {
        self.pending_requests = requests;
        self.pending_count = self.pending_requests.len();
    }

    pub fn push_request(&mut self, req: PendingToolApproval) {
        if !self.pending_requests.iter().any(|r| r.id == req.id) {
            self.pending_requests.push(req);
            self.pending_count = self.pending_requests.len();
        }
    }

    pub fn decide(&mut self, id: &str, decision: ToolApprovalDecision) {
        self.pending_requests.retain(|r| r.id != id);
        self.pending_count = self.pending_requests.len();
        self.last_decision = Some((id.to_string(), decision));
        if let Some(ref cb) = self.on_decision {
            cb(id.to_string(), decision);
        }
    }
}

impl Render for ToolPermissionDock {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;
        let lang = &self.language_id;
        let card_bg = rgb(pal.card_bg);
        let warn_bg = TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.14);

        div()
            .w_full()
            .p(px(12.0))
            .rounded(px(10.0))
            .bg(card_bg)
            .border_1()
            .border_color(rgb(pal.border_fg))
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(tg_icon(IconName::TerminalMcp, 16.0, pal.accent))
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .child(format!(
                                        "{} ({})",
                                        tr_str(lang, "待审批工具调用", "Tool Permission Queue"),
                                        self.pending_count
                                    )),
                            ),
                    ),
            )
            .map(|el| {
                if self.pending_requests.is_empty() {
                    el.child(
                        div()
                            .p(px(10.0))
                            .rounded(px(8.0))
                            .bg(rgb(pal.input_bg))
                            .text_size(px(12.0))
                            .text_color(rgb(pal.text_secondary))
                            .child(tr_str(
                                lang,
                                "暂无需要人工授权的工具调用请求。",
                                "No tool calls currently awaiting approval.",
                            )),
                    )
                } else {
                    el.children(self.pending_requests.iter().map(|req| {
                        let id_once = req.id.clone();
                        let id_always = req.id.clone();
                        let id_deny = req.id.clone();
                        div()
                            .p(px(10.0))
                            .rounded(px(8.0))
                            .bg(warn_bg)
                            .border_1()
                            .border_color(rgb(pal.border_fg))
                            .flex()
                            .flex_col()
                            .gap(px(6.0))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(pal.text_primary))
                                            .child(req.tool_name.clone()),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(11.0))
                                            .text_color(rgb(pal.text_secondary))
                                            .child(format!("{} • {}", req.risk_level, req.created_at)),
                                    ),
                            )
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(rgb(pal.text_secondary))
                                    .child(req.summary.clone()),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap(px(6.0))
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .px(px(10.0))
                                            .py(px(5.0))
                                            .rounded(px(6.0))
                                            .bg(rgb(pal.accent))
                                            .text_size(px(12.0))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(pal.active_button_fg))
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(move |this, _e, _w, cx| {
                                                    this.decide(&id_once, ToolApprovalDecision::AllowOnce);
                                                    cx.notify();
                                                }),
                                            )
                                            .child(tr_str(lang, "允许一次", "Allow Once")),
                                    )
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .px(px(10.0))
                                            .py(px(5.0))
                                            .rounded(px(6.0))
                                            .bg(rgb(pal.success))
                                            .text_size(px(12.0))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(pal.active_button_fg))
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(move |this, _e, _w, cx| {
                                                    this.decide(&id_always, ToolApprovalDecision::AllowAlways);
                                                    cx.notify();
                                                }),
                                            )
                                            .child(tr_str(lang, "始终允许", "Always Allow")),
                                    )
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .px(px(10.0))
                                            .py(px(5.0))
                                            .rounded(px(6.0))
                                            .bg(rgb(pal.danger))
                                            .text_size(px(12.0))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(pal.active_button_fg))
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(move |this, _e, _w, cx| {
                                                    this.decide(&id_deny, ToolApprovalDecision::Deny);
                                                    cx.notify();
                                                }),
                                            )
                                            .child(tr_str(lang, "拒绝", "Deny")),
                                    ),
                            )
                    }))
                }
            })
    }
}
