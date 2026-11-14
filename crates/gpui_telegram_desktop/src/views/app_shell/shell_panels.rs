//! Authoritative panel state and theme-driven UI builders for Calls, ACP Task Center, and MCP Tools.

use crate::i18n::tr_str;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use gpui::*;
use gpui_telegram_proto::contracts::features::CallRecordDto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellAcpTaskRow {
    pub task_id: String,
    pub title: String,
    pub status: String,
    pub requires_approval: bool,
    pub chat_id: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellMcpServerRow {
    pub server_id: String,
    pub name: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellMcpToolRow {
    pub tool_id: String,
    pub server_name: String,
    pub name: String,
    pub description: String,
    pub permission: String,
}

#[derive(Debug, Clone, Default)]
pub struct ShellPanelState {
    pub calls: Vec<CallRecordDto>,
    pub calls_loaded: bool,
    pub calls_loading: bool,
    pub calls_error: Option<String>,
    pub tasks: Vec<ShellAcpTaskRow>,
    pub tasks_loaded: bool,
    pub tasks_loading: bool,
    pub tasks_error: Option<String>,
    pub mcp_servers: Vec<ShellMcpServerRow>,
    pub mcp_tools: Vec<ShellMcpToolRow>,
    pub mcp_loaded: bool,
    pub mcp_loading: bool,
    pub mcp_error: Option<String>,
    pub mcp_last_result: Option<String>,
}

impl ShellPanelState {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

pub fn parse_acp_tasks(items: &[serde_json::Value]) -> Vec<ShellAcpTaskRow> {
    items
        .iter()
        .enumerate()
        .map(|(idx, v)| {
            let task_id = v
                .get("task_id")
                .or_else(|| v.get("id"))
                .and_then(|x| {
                    x.as_str()
                        .map(str::to_string)
                        .or_else(|| x.as_u64().map(|n| n.to_string()))
                })
                .unwrap_or_else(|| format!("{}", idx + 1));
            let title = v
                .get("title")
                .or_else(|| v.get("goal"))
                .or_else(|| v.get("prompt"))
                .and_then(|x| x.as_str())
                .unwrap_or("Task")
                .to_string();
            let status = v
                .get("status")
                .or_else(|| v.get("state"))
                .and_then(|x| x.as_str())
                .unwrap_or("pending")
                .to_string();
            let requires_approval = v
                .get("requires_approval")
                .and_then(|x| x.as_bool())
                .unwrap_or_else(|| status.eq_ignore_ascii_case("awaiting_approval"));
            let chat_id = v.get("chat_id").and_then(|x| x.as_u64());
            ShellAcpTaskRow {
                task_id,
                title,
                status,
                requires_approval,
                chat_id,
            }
        })
        .collect()
}

pub fn parse_mcp_servers(items: &[serde_json::Value]) -> Vec<ShellMcpServerRow> {
    items
        .iter()
        .map(|v| ShellMcpServerRow {
            server_id: v
                .get("server_id")
                .or_else(|| v.get("id"))
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string(),
            name: v
                .get("name")
                .or_else(|| v.get("url"))
                .and_then(|x| x.as_str())
                .unwrap_or("MCP Server")
                .to_string(),
            status: v
                .get("status")
                .and_then(|x| x.as_str())
                .unwrap_or("configured")
                .to_string(),
        })
        .collect()
}

pub fn parse_mcp_tools(items: &[serde_json::Value]) -> Vec<ShellMcpToolRow> {
    items
        .iter()
        .map(|v| ShellMcpToolRow {
            tool_id: v
                .get("tool_id")
                .or_else(|| v.get("id"))
                .or_else(|| v.get("name"))
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string(),
            server_name: v
                .get("server_name")
                .or_else(|| v.get("server_id"))
                .and_then(|x| x.as_str())
                .unwrap_or("mcp")
                .to_string(),
            name: v
                .get("name")
                .and_then(|x| x.as_str())
                .unwrap_or("tool")
                .to_string(),
            description: v
                .get("description")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string(),
            permission: v
                .get("permission")
                .or_else(|| v.get("policy"))
                .and_then(|x| x.as_str())
                .unwrap_or("ask")
                .to_string(),
        })
        .collect()
}

pub fn render_panel_header(
    icon: IconName,
    title: impl Into<String>,
    back_label: impl Into<String>,
    palette: &TdesktopThemePalette,
) -> (Div, Div) {
    let title = title.into();
    let back_label = back_label.into();
    let title_block = div()
        .flex()
        .items_center()
        .gap(px(10.0))
        .child(tg_icon(icon, 20.0, palette.accent))
        .child(
            div()
                .text_size(px(20.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(palette.text_primary))
                .child(title),
        );
    let back_btn = div()
        .cursor_pointer()
        .px(px(12.0))
        .py(px(6.0))
        .rounded(px(6.0))
        .bg(rgb(palette.accent))
        .hover(|s| s.bg(rgb(palette.accent_hover)))
        .flex()
        .items_center()
        .gap(px(6.0))
        .text_size(px(12.0))
        .font_weight(FontWeight::BOLD)
        .text_color(rgb(palette.active_button_fg))
        .child(tg_icon(IconName::BackArrow, 14.0, palette.active_button_fg))
        .child(back_label);
    (title_block, back_btn)
}

pub fn render_empty_panel_notice(
    icon: IconName,
    title: &str,
    subtitle: &str,
    error: Option<&str>,
    palette: &TdesktopThemePalette,
) -> Div {
    let mut box_el = div()
        .flex_1()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(px(12.0))
        .child(
            div()
                .w(px(68.0))
                .h(px(68.0))
                .rounded_full()
                .bg(rgb(palette.input_bg))
                .flex()
                .items_center()
                .justify_center()
                .child(tg_icon(icon, 30.0, palette.accent)),
        )
        .child(
            div()
                .text_size(px(17.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(palette.text_primary))
                .child(title.to_string()),
        )
        .child(
            div()
                .text_size(px(13.0))
                .text_color(rgb(palette.text_secondary))
                .child(subtitle.to_string()),
        );
    if let Some(err) = error {
        box_el = box_el.child(
            div()
                .text_size(px(12.0))
                .text_color(rgb(palette.danger))
                .child(err.to_string()),
        );
    }
    box_el
}

pub fn localized_panel_label<'a>(lang_id: &str, zh: &'a str, en: &'a str) -> &'a str {
    tr_str(lang_id, zh, en)
}
