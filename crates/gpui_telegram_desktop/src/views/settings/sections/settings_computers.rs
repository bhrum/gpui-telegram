use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_telegram_proto::{
    AuthorizedControllerEntry, AuthorizedControllerKind, CloudAuthMethod, ComputerNodeDto,
    ComputerNodeKind, ComputerRunPolicy, UpdateComputerPolicyRequest, UpsertCloudComputerRequest,
};

use crate::components::ui::{
    TextInputController, delete_prev_word, pop_last_char, sanitize_paste_text,
};
use crate::i18n::tr_str;
use crate::theme::assets::{IconName, tg_icon};
use crate::theme::palette::TdesktopThemePalette;
use crate::views::computer_workbench::render_ime_text_preview;
use crate::views::settings::settings_theme_ui::{
    settings_card, settings_container, settings_subsection_title,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SettingsComputersPage {
    #[default]
    Overview,
    Detail(String),
    CloudForm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CloudFormField {
    #[default]
    None,
    DisplayName,
    Host,
    Port,
    Username,
    Credential,
    AgentUrl,
    Workdir,
}

#[derive(Debug, Clone)]
pub enum SettingsComputersEvent {
    SelectNode(String),
    BackToOverview,
    OpenCreateCloudModal,
    SaveCloudServer(UpsertCloudComputerRequest),
    UpdatePolicy {
        node_id: String,
        req: UpdateComputerPolicyRequest,
    },
    DeleteNode(String),
    TestNode(String),
    OpenRemoteWorkbench(String),
    RefreshComputers,
}

pub struct SettingsComputersSections {
    pub palette: TdesktopThemePalette,
    pub language_id: String,
    pub page: SettingsComputersPage,
    pub computers: Vec<ComputerNodeDto>,
    pub known_bots: Vec<(u64, String)>,
    pub current_bot_id: Option<u64>,
    pub status_message: Option<String>,
    pub policy_menu_open: bool,
    pub device_picker_open: bool,
    pub bot_picker_open: bool,
    pub detail_more_open: bool,
    pub pending_event: Option<SettingsComputersEvent>,

    // Cloud Server Form state + TextInputControllers
    pub editing_node_id: Option<String>,
    pub form_focus: CloudFormField,
    pub form_name: String,
    pub form_host: String,
    pub form_port: String,
    pub form_username: String,
    pub form_auth_method: CloudAuthMethod,
    pub form_credential: String,
    pub form_agent_url: String,
    pub form_workdir: String,
    pub form_bind_to_current_bot: bool,

    pub input_name: Option<Entity<TextInputController>>,
    pub input_host: Option<Entity<TextInputController>>,
    pub input_port: Option<Entity<TextInputController>>,
    pub input_username: Option<Entity<TextInputController>>,
    pub input_credential: Option<Entity<TextInputController>>,
    pub input_agent_url: Option<Entity<TextInputController>>,
    pub input_workdir: Option<Entity<TextInputController>>,
}

impl SettingsComputersSections {
    pub fn new_live(palette: TdesktopThemePalette, lang: &str) -> Self {
        Self {
            palette,
            language_id: lang.to_string(),
            page: SettingsComputersPage::Overview,
            computers: Vec::new(),
            known_bots: Vec::new(),
            current_bot_id: None,
            status_message: None,
            policy_menu_open: false,
            device_picker_open: false,
            bot_picker_open: false,
            detail_more_open: false,
            pending_event: None,
            editing_node_id: None,
            form_focus: CloudFormField::None,
            form_name: String::new(),
            form_host: String::new(),
            form_port: "22".to_string(),
            form_username: "root".to_string(),
            form_auth_method: CloudAuthMethod::SshPassword,
            form_credential: String::new(),
            form_agent_url: String::new(),
            form_workdir: "/workspace".to_string(),
            form_bind_to_current_bot: false,
            input_name: None,
            input_host: None,
            input_port: None,
            input_username: None,
            input_credential: None,
            input_agent_url: None,
            input_workdir: None,
        }
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) {
        self.palette = palette;
    }

    pub fn set_language(&mut self, lang: &str) {
        self.language_id = lang.to_string();
    }

    pub fn sync_computers(&mut self, computers: Vec<ComputerNodeDto>) {
        self.computers = computers;
    }

    pub fn sync_known_bots(&mut self, bots: Vec<(u64, String)>, current_bot_id: Option<u64>) {
        self.known_bots = bots;
        self.current_bot_id = current_bot_id;
    }

    pub fn open_overview(&mut self) {
        self.page = SettingsComputersPage::Overview;
        self.form_focus = CloudFormField::None;
        self.policy_menu_open = false;
        self.device_picker_open = false;
        self.bot_picker_open = false;
        self.detail_more_open = false;
    }

    pub fn open_detail(&mut self, node_id: &str) {
        self.page = SettingsComputersPage::Detail(node_id.to_string());
        self.form_focus = CloudFormField::None;
        self.policy_menu_open = false;
        self.device_picker_open = false;
        self.bot_picker_open = false;
        self.detail_more_open = false;
    }

    pub fn open_cloud_form(
        &mut self,
        bind_bot_id: Option<u64>,
        edit_node: Option<&ComputerNodeDto>,
    ) {
        self.page = SettingsComputersPage::CloudForm;
        self.form_focus = CloudFormField::DisplayName;
        self.policy_menu_open = false;
        self.device_picker_open = false;
        self.bot_picker_open = false;
        self.detail_more_open = false;
        if let Some(id) = bind_bot_id {
            self.current_bot_id = Some(id);
            self.form_bind_to_current_bot = true;
        } else {
            self.form_bind_to_current_bot = false;
        }

        if let Some(node) = edit_node {
            self.editing_node_id = Some(node.node_id.clone());
            self.form_name = node.display_name.clone();
            self.form_host = node.host.clone().unwrap_or_else(|| node.ip_address.clone());
            self.form_port = node.ssh_port.unwrap_or(22).to_string();
            self.form_username = node.ssh_username.clone().unwrap_or_else(|| "root".to_string());
            self.form_auth_method = node.auth_method.unwrap_or(CloudAuthMethod::SshPassword);
            self.form_credential.clear();
            self.form_agent_url = node.agent_endpoint_url.clone().unwrap_or_default();
            self.form_workdir = node.workdir.clone().unwrap_or_else(|| "/workspace".to_string());
        } else {
            self.editing_node_id = None;
            self.form_name.clear();
            self.form_host.clear();
            self.form_port = "22".to_string();
            self.form_username = "root".to_string();
            self.form_auth_method = CloudAuthMethod::SshPassword;
            self.form_credential.clear();
            self.form_agent_url.clear();
            self.form_workdir = "/workspace".to_string();
        }
    }

    pub fn take_event(&mut self) -> Option<SettingsComputersEvent> {
        self.pending_event.take()
    }

    fn ensure_form_inputs(&mut self, cx: &mut Context<Self>) {
        if self.input_name.is_none() {
            self.input_name = Some(cx.new(|_| TextInputController::new()));
        }
        if self.input_host.is_none() {
            self.input_host = Some(cx.new(|_| TextInputController::new()));
        }
        if self.input_port.is_none() {
            self.input_port = Some(cx.new(|_| TextInputController::new()));
        }
        if self.input_username.is_none() {
            self.input_username = Some(cx.new(|_| TextInputController::new()));
        }
        if self.input_credential.is_none() {
            self.input_credential = Some(cx.new(|_| TextInputController::new()));
        }
        if self.input_agent_url.is_none() {
            self.input_agent_url = Some(cx.new(|_| TextInputController::new()));
        }
        if self.input_workdir.is_none() {
            self.input_workdir = Some(cx.new(|_| TextInputController::new()));
        }
    }

    pub fn is_input_active(&self) -> bool {
        self.page == SettingsComputersPage::CloudForm && self.form_focus != CloudFormField::None
    }

    fn active_controller(&self) -> Option<&Entity<TextInputController>> {
        match self.form_focus {
            CloudFormField::DisplayName => self.input_name.as_ref(),
            CloudFormField::Host => self.input_host.as_ref(),
            CloudFormField::Port => self.input_port.as_ref(),
            CloudFormField::Username => self.input_username.as_ref(),
            CloudFormField::Credential => self.input_credential.as_ref(),
            CloudFormField::AgentUrl => self.input_agent_url.as_ref(),
            CloudFormField::Workdir => self.input_workdir.as_ref(),
            CloudFormField::None => None,
        }
    }

    pub fn active_field_text(&self) -> &str {
        match self.form_focus {
            CloudFormField::DisplayName => &self.form_name,
            CloudFormField::Host => &self.form_host,
            CloudFormField::Port => &self.form_port,
            CloudFormField::Username => &self.form_username,
            CloudFormField::Credential => &self.form_credential,
            CloudFormField::AgentUrl => &self.form_agent_url,
            CloudFormField::Workdir => &self.form_workdir,
            CloudFormField::None => "",
        }
    }

    fn active_field_mut(&mut self) -> Option<&mut String> {
        match self.form_focus {
            CloudFormField::DisplayName => Some(&mut self.form_name),
            CloudFormField::Host => Some(&mut self.form_host),
            CloudFormField::Port => Some(&mut self.form_port),
            CloudFormField::Username => Some(&mut self.form_username),
            CloudFormField::Credential => Some(&mut self.form_credential),
            CloudFormField::AgentUrl => Some(&mut self.form_agent_url),
            CloudFormField::Workdir => Some(&mut self.form_workdir),
            CloudFormField::None => None,
        }
    }

    pub fn active_input_text_and_preedit(&self, cx: &App) -> (String, Option<String>) {
        let committed = self.active_field_text().to_string();
        if let Some(ctrl) = self.active_controller() {
            ctrl.read(cx).text_and_preedit(committed)
        } else {
            (committed, None)
        }
    }

    pub fn clear_active_input(&mut self, cx: &mut Context<Self>) {
        if let Some(ctrl) = self.active_controller().cloned() {
            ctrl.update(cx, |c, _| c.clear_preedit());
        }
        if let Some(dst) = self.active_field_mut() {
            dst.clear();
        }
        cx.notify();
    }

    pub fn push_input_text(&mut self, text: &str, cx: &mut Context<Self>) {
        let allow_nl = self.form_focus == CloudFormField::Credential
            && self.form_auth_method == CloudAuthMethod::SshPrivateKey;
        let clean = sanitize_paste_text(text, allow_nl);
        if clean.is_empty() {
            return;
        }
        if let Some(ctrl) = self.active_controller().cloned() {
            ctrl.update(cx, |c, _| c.clear_preedit());
        }
        if let Some(dst) = self.active_field_mut() {
            dst.push_str(&clean);
        }
        cx.notify();
    }

    pub fn handle_non_char_key(
        &mut self,
        key: &str,
        is_shift: bool,
        is_alt: bool,
        cx: &mut Context<Self>,
    ) -> bool {
        if !self.is_input_active() {
            return false;
        }
        match key {
            "backspace" | "delete" => {
                if let Some(dst) = self.active_field_mut() {
                    let changed = if is_alt {
                        delete_prev_word(dst)
                    } else {
                        pop_last_char(dst)
                    };
                    if changed {
                        cx.notify();
                    }
                }
                true
            }
            "tab" => {
                let order = [
                    CloudFormField::DisplayName,
                    CloudFormField::Host,
                    CloudFormField::Port,
                    CloudFormField::Username,
                    CloudFormField::Credential,
                    CloudFormField::AgentUrl,
                    CloudFormField::Workdir,
                ];
                if let Some(pos) = order.iter().position(|f| *f == self.form_focus) {
                    let next = if is_shift {
                        (pos + order.len() - 1) % order.len()
                    } else {
                        (pos + 1) % order.len()
                    };
                    self.form_focus = order[next];
                    cx.notify();
                }
                true
            }
            "enter" => {
                self.submit_cloud_form();
                cx.notify();
                true
            }
            _ => false,
        }
    }

    pub fn replace_and_mark_input(
        &mut self,
        new_text: &str,
        cx: &mut Context<Self>,
    ) -> Option<String> {
        self.ensure_form_inputs(cx);
        let commit = self
            .active_controller()
            .cloned()
            .and_then(|c| c.update(cx, |ctrl, _| ctrl.handle_replace_and_mark(new_text)));
        if let Some(txt) = commit.as_ref() {
            if let Some(dst) = self.active_field_mut() {
                dst.push_str(txt);
            }
        }
        cx.notify();
        commit
    }

    pub fn set_ime_preedit(&mut self, preedit: Option<&str>, cx: &mut Context<Self>) {
        self.ensure_form_inputs(cx);
        if let Some(ctrl) = self.active_controller().cloned() {
            ctrl.update(cx, |c, _| {
                c.ime_preedit = preedit.filter(|s| !s.is_empty()).map(str::to_string);
            });
            cx.notify();
        }
    }

    pub fn submit_cloud_form(&mut self) -> bool {
        let host = self.form_host.trim().to_string();
        if host.is_empty() {
            self.status_message = Some(
                tr_str(
                    &self.language_id,
                    "请填写云服务器的主机地址或 IP",
                    "Please enter a cloud host or IP address",
                )
                .to_string(),
            );
            return false;
        }
        let port = self.form_port.trim().parse::<u16>().unwrap_or(22);
        let display_name = if self.form_name.trim().is_empty() {
            host.clone()
        } else {
            self.form_name.trim().to_string()
        };
        let username = if self.form_username.trim().is_empty() {
            "root".to_string()
        } else {
            self.form_username.trim().to_string()
        };
        let secret = (!self.form_credential.trim().is_empty())
            .then(|| self.form_credential.trim().to_string());
        let agent_endpoint_url = (!self.form_agent_url.trim().is_empty())
            .then(|| self.form_agent_url.trim().to_string());
        let workdir = (!self.form_workdir.trim().is_empty())
            .then(|| self.form_workdir.trim().to_string());
        let bind_bot_id = if self.form_bind_to_current_bot {
            self.current_bot_id
        } else {
            None
        };

        self.form_focus = CloudFormField::None;
        self.status_message = None;
        self.pending_event = Some(SettingsComputersEvent::SaveCloudServer(
            UpsertCloudComputerRequest {
                node_id: self.editing_node_id.clone(),
                display_name,
                host,
                port,
                username,
                auth_method: self.form_auth_method,
                secret,
                agent_endpoint_url,
                workdir,
                bind_bot_id,
                verify_connection: true,
            },
        ));
        true
    }

    fn policy_label(policy: ComputerRunPolicy, lang: &str) -> &'static str {
        if lang.starts_with("zh") {
            policy.label_zh()
        } else {
            policy.label_en()
        }
    }

    fn node_endpoint(node: &ComputerNodeDto) -> String {
        node.host
            .as_deref()
            .filter(|s| !s.is_empty())
            .unwrap_or(&node.ip_address)
            .to_string()
    }

    fn render_overview(&self, pal: TdesktopThemePalette, lang: &str, cx: &mut Context<Self>) -> AnyElement {
        let locals: Vec<&ComputerNodeDto> = self
            .computers
            .iter()
            .filter(|c| c.kind == ComputerNodeKind::LocalDevice)
            .collect();
        let clouds: Vec<&ComputerNodeDto> = self
            .computers
            .iter()
            .filter(|c| c.kind == ComputerNodeKind::CloudServer)
            .collect();

        let mut local_card = settings_card(pal);
        if locals.is_empty() {
            local_card = local_card.child(
                div()
                    .px(px(16.0))
                    .py(px(16.0))
                    .text_size(px(13.0))
                    .text_color(rgb(pal.text_secondary))
                    .child(tr_str(
                        lang,
                        "暂未发现已登录账号的本地设备，点击右上角刷新同步当前设备。",
                        "No local devices registered yet. Click Refresh to sync.",
                    )),
            );
        } else {
            for (idx, node) in locals.into_iter().enumerate() {
                local_card = local_card.child(self.render_overview_node_row(
                    node,
                    idx,
                    true,
                    pal,
                    lang,
                    cx,
                ));
            }
        }

        let mut cloud_card = settings_card(pal);
        if clouds.is_empty() {
            cloud_card = cloud_card.child(
                div()
                    .p(px(22.0))
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .w(px(42.0))
                            .h(px(42.0))
                            .rounded(px(12.0))
                            .bg(TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.14))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(tg_icon(IconName::ComputerDesktop, 20.0, pal.accent)),
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(pal.text_primary))
                            .child(tr_str(
                                lang,
                                "每个 Agent 都拥有自己的电脑",
                                "Every Agent gets its own computer",
                            )),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(pal.text_secondary))
                            .text_center()
                            .child(tr_str(
                                lang,
                                "只有在你的 Agent 需要保持 24/7 运行时，才添加一台常驻云电脑。",
                                "Add an always-on cloud computer only when your Agent needs 24/7 execution.",
                            )),
                    )
                    .child(
                        div()
                            .id("sc-empty-create-cloud")
                            .cursor_pointer()
                            .mt(px(4.0))
                            .px(px(14.0))
                            .py(px(7.0))
                            .rounded(px(8.0))
                            .bg(rgb(pal.accent))
                            .hover(|s| s.bg(rgb(pal.accent_hover)))
                            .text_size(px(12.5))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(pal.active_button_fg))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.open_cloud_form(this.current_bot_id, None);
                                    cx.notify();
                                }),
                            )
                            .child(tr_str(lang, "+ 创建云服务器", "+ Create Cloud Server")),
                    ),
            );
        } else {
            for (idx, node) in clouds.into_iter().enumerate() {
                cloud_card = cloud_card.child(self.render_overview_node_row(
                    node,
                    idx,
                    false,
                    pal,
                    lang,
                    cx,
                ));
            }
        }

        settings_container(pal)
            .child(
                div()
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(rgb(pal.text_secondary))
                            .child(tr_str(
                                lang,
                                "管理当前账号下所有本地设备与常驻云电脑，控制权限与运行审批策略。",
                                "Manage local devices, cloud computers, remote authorization & execution safety.",
                            )),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .id("sc-refresh-btn")
                                    .cursor_pointer()
                                    .px(px(11.0))
                                    .py(px(6.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(pal.input_bg))
                                    .border_1()
                                    .border_color(rgb(pal.border_fg))
                                    .hover(|s| s.bg(rgb(pal.hover_bg)))
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            this.pending_event =
                                                Some(SettingsComputersEvent::RefreshComputers);
                                            cx.notify();
                                        }),
                                    )
                                    .child(tr_str(lang, "刷新", "Refresh")),
                            )
                            .child(
                                div()
                                    .id("sc-create-cloud-btn")
                                    .cursor_pointer()
                                    .px(px(12.0))
                                    .py(px(6.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(pal.accent))
                                    .hover(|s| s.bg(rgb(pal.accent_hover)))
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(pal.active_button_fg))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            this.open_cloud_form(this.current_bot_id, None);
                                            cx.notify();
                                        }),
                                    )
                                    .child(tr_str(lang, "+ 创建云服务器", "+ Add Cloud Server")),
                            ),
                    ),
            )
            .when_some(self.status_message.as_ref(), |col, msg| {
                col.child(
                    div()
                        .p(px(10.0))
                        .rounded(px(8.0))
                        .bg(TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.14))
                        .text_size(px(12.0))
                        .text_color(rgb(pal.text_primary))
                        .child(msg.clone()),
                )
            })
            .child(settings_subsection_title(
                tr_str(lang, "本地电脑", "Local Computers"),
                pal,
            ))
            .child(local_card)
            .child(settings_subsection_title(
                tr_str(lang, "云电脑", "Cloud Computers"),
                pal,
            ))
            .child(cloud_card)
            .into_any_element()
    }

    fn render_overview_node_row(
        &self,
        node: &ComputerNodeDto,
        idx: usize,
        is_local: bool,
        pal: TdesktopThemePalette,
        lang: &str,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let nid_select = node.node_id.clone();
        let nid_ctrl = node.node_id.clone();
        let nid_test = node.node_id.clone();
        let endpoint = Self::node_endpoint(node);
        let badge = if is_local {
            if node.is_current_device {
                tr_str(lang, "当前设备", "Current Device")
            } else {
                tr_str(lang, "同账号设备", "Account Device")
            }
        } else {
            tr_str(lang, "云服务器", "Cloud Server")
        };

        div()
            .id((if is_local { "sc-local-row" } else { "sc-cloud-row" }, idx))
            .px(px(16.0))
            .py(px(12.0))
            .border_b_1()
            .border_color(rgb(pal.border_fg))
            .hover(|s| s.bg(rgb(pal.hover_bg)))
            .flex()
            .items_center()
            .justify_between()
            .gap(px(12.0))
            .child(
                div()
                    .id((if is_local { "sc-local-open" } else { "sc-cloud-open" }, idx))
                    .cursor_pointer()
                    .flex_1()
                    .min_w(px(0.0))
                    .flex()
                    .items_center()
                    .gap(px(12.0))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            this.open_detail(&nid_select);
                            this.pending_event =
                                Some(SettingsComputersEvent::SelectNode(nid_select.clone()));
                            cx.notify();
                        }),
                    )
                    .child(
                        div()
                            .w(px(36.0))
                            .h(px(36.0))
                            .rounded(px(10.0))
                            .bg(TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.14))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(tg_icon(IconName::ComputerDesktop, 18.0, pal.accent)),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(3.0))
                            .min_w(px(0.0))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .child(
                                        div()
                                            .text_size(px(13.5))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(pal.text_primary))
                                            .truncate()
                                            .child(node.display_name.clone()),
                                    )
                                    .child(
                                        div()
                                            .px(px(7.0))
                                            .py(px(1.5))
                                            .rounded(px(999.0))
                                            .bg(rgb(pal.input_bg))
                                            .text_size(px(10.5))
                                            .text_color(rgb(pal.text_secondary))
                                            .child(badge.to_string()),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .child(
                                        div()
                                            .w(px(7.0))
                                            .h(px(7.0))
                                            .rounded_full()
                                            .bg(rgb(if node.online {
                                                pal.success
                                            } else {
                                                pal.text_secondary
                                            })),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(11.5))
                                            .text_color(rgb(pal.text_secondary))
                                            .truncate()
                                            .child(format!(
                                                "{} · {} · {}",
                                                if node.online {
                                                    tr_str(lang, "在线", "Online")
                                                } else {
                                                    tr_str(lang, "离线", "Offline")
                                                },
                                                endpoint,
                                                Self::policy_label(node.run_policy, lang),
                                            )),
                                    ),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .when(!is_local, |row| {
                        row.child(
                            div()
                                .id(("sc-test-node", idx))
                                .cursor_pointer()
                                .px(px(8.0))
                                .py(px(5.0))
                                .rounded(px(7.0))
                                .bg(rgb(pal.input_bg))
                                .border_1()
                                .border_color(rgb(pal.border_fg))
                                .hover(|s| s.bg(rgb(pal.dialogs_hover_bg)))
                                .text_size(px(11.5))
                                .text_color(rgb(pal.text_primary))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _, _, cx| {
                                        this.pending_event =
                                            Some(SettingsComputersEvent::TestNode(nid_test.clone()));
                                        cx.notify();
                                    }),
                                )
                                .child(tr_str(lang, "测试", "Test")),
                        )
                    })
                    .child(
                        div()
                            .id((if is_local { "sc-local-wb" } else { "sc-cloud-wb" }, idx))
                            .cursor_pointer()
                            .px(px(9.0))
                            .py(px(5.0))
                            .rounded(px(7.0))
                            .bg(TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.14))
                            .hover(|s| s.bg(TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.24)))
                            .text_size(px(11.5))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(pal.accent))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| {
                                    this.pending_event = Some(
                                        SettingsComputersEvent::OpenRemoteWorkbench(nid_ctrl.clone()),
                                    );
                                    cx.notify();
                                }),
                            )
                            .child(tr_str(lang, "远程操控", "Remote Control")),
                    )
                    .child(tg_icon(IconName::ChevronRight, 14.0, pal.icon_fg)),
            )
            .into_any_element()
    }

    fn render_detail(
        &self,
        node_id: &str,
        pal: TdesktopThemePalette,
        lang: &str,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(node) = self.computers.iter().find(|c| c.node_id == node_id) else {
            return settings_container(pal)
                .child(self.detail_back_bar(None, pal, lang, cx))
                .child(
                    settings_card(pal).child(
                        div()
                            .p(px(20.0))
                            .text_size(px(13.0))
                            .text_color(rgb(pal.text_secondary))
                            .child(tr_str(
                                lang,
                                "未找到此计算机节点，可能已被删除或尚未同步。",
                                "Computer node not found or removed.",
                            )),
                    ),
                )
                .into_any_element();
        };

        let endpoint = Self::node_endpoint(node);
        let badge = match node.kind {
            ComputerNodeKind::LocalDevice if node.is_current_device => {
                tr_str(lang, "当前设备", "Current Device")
            }
            ComputerNodeKind::LocalDevice => tr_str(lang, "同账号设备", "Account Device"),
            ComputerNodeKind::CloudServer => tr_str(lang, "云服务器", "Cloud Server"),
            ComputerNodeKind::CloudSandbox => tr_str(lang, "云沙箱", "Cloud Sandbox"),
        };
        let nid_wb = node.node_id.clone();
        let nid_policy = node.node_id.clone();
        let nid_add_dev = node.node_id.clone();
        let nid_add_bot = node.node_id.clone();
        let nid_remove = node.node_id.clone();
        let nid_test = node.node_id.clone();
        let nid_del = node.node_id.clone();
        let is_cloud = node.kind == ComputerNodeKind::CloudServer;
        let node_clone_for_edit = node.clone();

        // Middle ACL card: 谁可以控制 <device>
        let ctrl_heading = if lang.starts_with("zh") {
            format!("谁可以控制 {}", node.display_name)
        } else {
            format!("Who can control {}", node.display_name)
        };

        let mut acl_card = settings_card(pal).child(
            div()
                .px(px(16.0))
                .py(px(12.0))
                .border_b_1()
                .border_color(rgb(pal.border_fg))
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_size(px(14.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.text_primary))
                        .child(ctrl_heading),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(6.0))
                        .child(
                            div()
                                .id("sc-acl-add-device-btn")
                                .cursor_pointer()
                                .px(px(9.0))
                                .py(px(5.0))
                                .rounded(px(7.0))
                                .bg(TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.14))
                                .hover(|s| s.bg(TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.24)))
                                .text_size(px(11.5))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(pal.accent))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _, _, cx| {
                                        this.device_picker_open = !this.device_picker_open;
                                        this.bot_picker_open = false;
                                        cx.notify();
                                    }),
                                )
                                .child(tr_str(lang, "+ 添加设备", "+ Add Device")),
                        )
                        .child(
                            div()
                                .id("sc-acl-add-bot-btn")
                                .cursor_pointer()
                                .px(px(9.0))
                                .py(px(5.0))
                                .rounded(px(7.0))
                                .bg(rgb(pal.input_bg))
                                .border_1()
                                .border_color(rgb(pal.border_fg))
                                .hover(|s| s.bg(rgb(pal.hover_bg)))
                                .text_size(px(11.5))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(pal.text_primary))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _, _, cx| {
                                        this.bot_picker_open = !this.bot_picker_open;
                                        this.device_picker_open = false;
                                        cx.notify();
                                    }),
                                )
                                .child(tr_str(lang, "+ 授权 AI Bot", "+ Authorize Bot")),
                        ),
                ),
        );

        if node.authorized_controllers.is_empty() {
            acl_card = acl_card.child(
                div()
                    .p(px(22.0))
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(6.0))
                    .child(
                        div()
                            .w(px(38.0))
                            .h(px(38.0))
                            .rounded(px(10.0))
                            .bg(rgb(pal.input_bg))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(tg_icon(IconName::ComputerDesktop, 18.0, pal.icon_fg)),
                    )
                    .child(
                        div()
                            .text_size(px(13.5))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(pal.text_primary))
                            .child(tr_str(lang, "没有已授权的设备", "No authorized devices")),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(pal.text_secondary))
                            .child(tr_str(
                                lang,
                                "添加设备以远程控制此计算机。",
                                "Add a device or AI Bot to allow remote control of this computer.",
                            )),
                    ),
            );
        } else {
            for (idx, entry) in node.authorized_controllers.iter().enumerate() {
                let cid = entry.controller_id.clone();
                let nid = nid_remove.clone();
                let kind_label = match entry.kind {
                    AuthorizedControllerKind::Device => tr_str(lang, "设备", "Device"),
                    AuthorizedControllerKind::Bot => "AI Bot",
                    AuthorizedControllerKind::AccountAllBots => {
                        tr_str(lang, "全部 Bot", "All Bots")
                    }
                };
                acl_card = acl_card.child(
                    div()
                        .px(px(16.0))
                        .py(px(10.0))
                        .border_b_1()
                        .border_color(rgb(pal.border_fg))
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .child(tg_icon(
                                    match entry.kind {
                                        AuthorizedControllerKind::Device => {
                                            IconName::ComputerDesktop
                                        }
                                        _ => IconName::BotRobot,
                                    },
                                    15.0,
                                    pal.accent,
                                ))
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .font_weight(FontWeight::MEDIUM)
                                                .text_color(rgb(pal.text_primary))
                                                .child(entry.display_name.clone()),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(11.0))
                                                .text_color(rgb(pal.text_secondary))
                                                .child(format!("{} · {}", kind_label, cid)),
                                        ),
                                ),
                        )
                        .child(
                            div()
                                .id(("sc-acl-remove", idx))
                                .cursor_pointer()
                                .px(px(8.0))
                                .py(px(4.0))
                                .rounded(px(6.0))
                                .bg(rgb(pal.input_bg))
                                .hover(|s| s.bg(rgb(pal.hover_bg)))
                                .text_size(px(11.5))
                                .text_color(rgb(pal.danger))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _, _, cx| {
                                        this.pending_event =
                                            Some(SettingsComputersEvent::UpdatePolicy {
                                                node_id: nid.clone(),
                                                req: UpdateComputerPolicyRequest {
                                                    remove_controller_id: Some(cid.clone()),
                                                    ..Default::default()
                                                },
                                            });
                                        cx.notify();
                                    }),
                                )
                                .child(tr_str(lang, "移除", "Remove")),
                        ),
                );
            }
        }

        if self.device_picker_open {
            let other_locals: Vec<&ComputerNodeDto> = self
                .computers
                .iter()
                .filter(|c| c.kind == ComputerNodeKind::LocalDevice && c.node_id != node.node_id)
                .collect();
            let nid_all_dev = nid_add_dev.clone();
            let mut picker = div()
                .p(px(12.0))
                .bg(rgb(pal.input_bg))
                .border_t_1()
                .border_color(rgb(pal.border_fg))
                .flex()
                .flex_col()
                .gap(px(6.0))
                .child(
                    div()
                        .text_size(px(11.5))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.text_secondary))
                        .child(tr_str(lang, "选择同账号登录设备授权", "Authorize Account Device")),
                )
                .child(
                    div()
                        .id("sc-pick-all-devices")
                        .cursor_pointer()
                        .px(px(10.0))
                        .py(px(7.0))
                        .rounded(px(7.0))
                        .bg(rgb(pal.card_bg))
                        .hover(|s| s.bg(rgb(pal.hover_bg)))
                        .text_size(px(12.0))
                        .text_color(rgb(pal.accent))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
                                this.device_picker_open = false;
                                this.pending_event = Some(SettingsComputersEvent::UpdatePolicy {
                                    node_id: nid_all_dev.clone(),
                                    req: UpdateComputerPolicyRequest {
                                        add_controller: Some(AuthorizedControllerEntry {
                                            controller_id: "account_all_devices".to_string(),
                                            kind: AuthorizedControllerKind::Device,
                                            display_name: tr_str(
                                                &this.language_id,
                                                "允许所有同账号设备",
                                                "All Same-Account Devices",
                                            )
                                            .to_string(),
                                            added_at_ms: 0,
                                        }),
                                        ..Default::default()
                                    },
                                });
                                cx.notify();
                            }),
                        )
                        .child(tr_str(lang, "✓ 允许所有同账号设备", "✓ Allow All Account Devices")),
                );
            if other_locals.is_empty() {
                picker = picker.child(
                    div()
                        .text_size(px(11.5))
                        .text_color(rgb(pal.text_secondary))
                        .child(tr_str(
                            lang,
                            "当前账号暂无其他在线桌面设备",
                            "No other logged-in devices on this account",
                        )),
                );
            } else {
                for (i, dev) in other_locals.into_iter().enumerate() {
                    let target_id = dev
                        .device_id
                        .clone()
                        .unwrap_or_else(|| dev.node_id.clone());
                    let label = format!("{} ({})", dev.display_name, Self::node_endpoint(dev));
                    let nid = nid_add_dev.clone();
                    picker = picker.child(
                        div()
                            .id(("sc-pick-dev", i))
                            .cursor_pointer()
                            .px(px(10.0))
                            .py(px(7.0))
                            .rounded(px(7.0))
                            .bg(rgb(pal.card_bg))
                            .hover(|s| s.bg(rgb(pal.hover_bg)))
                            .text_size(px(12.0))
                            .text_color(rgb(pal.text_primary))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| {
                                    this.device_picker_open = false;
                                    this.pending_event =
                                        Some(SettingsComputersEvent::UpdatePolicy {
                                            node_id: nid.clone(),
                                            req: UpdateComputerPolicyRequest {
                                                add_controller: Some(AuthorizedControllerEntry {
                                                    controller_id: target_id.clone(),
                                                    kind: AuthorizedControllerKind::Device,
                                                    display_name: label.clone(),
                                                    added_at_ms: 0,
                                                }),
                                                ..Default::default()
                                            },
                                        });
                                    cx.notify();
                                }),
                            )
                            .child(format!("{} ({})", dev.display_name, Self::node_endpoint(dev))),
                    );
                }
            }
            acl_card = acl_card.child(picker);
        }

        if self.bot_picker_open {
            let nid_all_bots = nid_add_bot.clone();
            let mut picker = div()
                .p(px(12.0))
                .bg(rgb(pal.input_bg))
                .border_t_1()
                .border_color(rgb(pal.border_fg))
                .flex()
                .flex_col()
                .gap(px(6.0))
                .child(
                    div()
                        .text_size(px(11.5))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.text_secondary))
                        .child(tr_str(lang, "选择授权的 AI Bot", "Authorize AI Bot")),
                )
                .child(
                    div()
                        .id("sc-pick-all-bots")
                        .cursor_pointer()
                        .px(px(10.0))
                        .py(px(7.0))
                        .rounded(px(7.0))
                        .bg(rgb(pal.card_bg))
                        .hover(|s| s.bg(rgb(pal.hover_bg)))
                        .text_size(px(12.0))
                        .text_color(rgb(pal.accent))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
                                this.bot_picker_open = false;
                                this.pending_event = Some(SettingsComputersEvent::UpdatePolicy {
                                    node_id: nid_all_bots.clone(),
                                    req: UpdateComputerPolicyRequest {
                                        add_controller: Some(AuthorizedControllerEntry {
                                            controller_id: "account_all_bots".to_string(),
                                            kind: AuthorizedControllerKind::AccountAllBots,
                                            display_name: tr_str(
                                                &this.language_id,
                                                "允许账号下所有 AI Bot",
                                                "All Account AI Bots",
                                            )
                                            .to_string(),
                                            added_at_ms: 0,
                                        }),
                                        ..Default::default()
                                    },
                                });
                                cx.notify();
                            }),
                        )
                        .child(tr_str(lang, "✓ 允许账号下所有 AI Bot", "✓ Allow All Account AI Bots")),
                );
            for (i, (bid, bname)) in self.known_bots.iter().enumerate() {
                let bid_str = format!("bot:{bid}");
                let label = bname.clone();
                let nid = nid_add_bot.clone();
                picker = picker.child(
                    div()
                        .id(("sc-pick-bot", i))
                        .cursor_pointer()
                        .px(px(10.0))
                        .py(px(7.0))
                        .rounded(px(7.0))
                        .bg(rgb(pal.card_bg))
                        .hover(|s| s.bg(rgb(pal.hover_bg)))
                        .text_size(px(12.0))
                        .text_color(rgb(pal.text_primary))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
                                this.bot_picker_open = false;
                                this.pending_event = Some(SettingsComputersEvent::UpdatePolicy {
                                    node_id: nid.clone(),
                                    req: UpdateComputerPolicyRequest {
                                        add_controller: Some(AuthorizedControllerEntry {
                                            controller_id: bid_str.clone(),
                                            kind: AuthorizedControllerKind::Bot,
                                            display_name: label.clone(),
                                            added_at_ms: 0,
                                        }),
                                        ..Default::default()
                                    },
                                });
                                cx.notify();
                            }),
                        )
                        .child(format!("{bname} (#{bid})")),
                );
            }
            acl_card = acl_card.child(picker);
        }

        // Bottom Run Policy Card: 在此计算机上运行
        let policies = [
            ComputerRunPolicy::AskEveryTime,
            ComputerRunPolicy::AllowReadOnly,
            ComputerRunPolicy::AlwaysAllowAuthorized,
            ComputerRunPolicy::Disabled,
        ];
        let mut policy_card = settings_card(pal).child(
            div()
                .p(px(16.0))
                .flex()
                .items_center()
                .justify_between()
                .gap(px(12.0))
                .child(
                    div()
                        .flex_1()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child(
                            div()
                                .text_size(px(14.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(pal.text_primary))
                                .child(tr_str(lang, "在此计算机上运行", "Run on this computer")),
                        )
                        .child(
                            div()
                                .text_size(px(12.0))
                                .text_color(rgb(pal.text_secondary))
                                .child(tr_str(
                                    lang,
                                    "AI Bot 和所有 Agent 在此计算机上执行每条命令或修改文件前，都会先征求你的同意。",
                                    "AI Bots and Agents will follow this safety rule before running commands or modifying files here.",
                                )),
                        ),
                )
                .child(
                    div()
                        .id("sc-policy-dropdown-btn")
                        .cursor_pointer()
                        .px(px(12.0))
                        .py(px(7.0))
                        .rounded(px(8.0))
                        .bg(rgb(pal.input_bg))
                        .border_1()
                        .border_color(rgb(pal.border_fg))
                        .hover(|s| s.bg(rgb(pal.hover_bg)))
                        .flex()
                        .items_center()
                        .gap(px(6.0))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, _, cx| {
                                this.policy_menu_open = !this.policy_menu_open;
                                cx.notify();
                            }),
                        )
                        .child(
                            div()
                                .text_size(px(12.5))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(pal.text_primary))
                                .child(Self::policy_label(node.run_policy, lang)),
                        )
                        .child(tg_icon(IconName::ChevronDown, 12.0, pal.icon_fg)),
                ),
        );

        if self.policy_menu_open {
            let mut menu = div()
                .p(px(10.0))
                .border_t_1()
                .border_color(rgb(pal.border_fg))
                .bg(rgb(pal.input_bg))
                .flex()
                .flex_col()
                .gap(px(4.0));
            for (idx, pol) in policies.into_iter().enumerate() {
                let selected = node.run_policy == pol;
                let nid = nid_policy.clone();
                menu = menu.child(
                    div()
                        .id(("sc-policy-choice", idx))
                        .cursor_pointer()
                        .px(px(10.0))
                        .py(px(8.0))
                        .rounded(px(7.0))
                        .bg(if selected {
                            TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.16)
                        } else {
                            rgb(pal.card_bg).into()
                        })
                        .hover(|s| s.bg(rgb(pal.hover_bg)))
                        .flex()
                        .items_center()
                        .justify_between()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
                                this.policy_menu_open = false;
                                this.pending_event = Some(SettingsComputersEvent::UpdatePolicy {
                                    node_id: nid.clone(),
                                    req: UpdateComputerPolicyRequest {
                                        run_policy: Some(pol),
                                        ..Default::default()
                                    },
                                });
                                cx.notify();
                            }),
                        )
                        .child(
                            div()
                                .text_size(px(12.5))
                                .font_weight(if selected {
                                    FontWeight::BOLD
                                } else {
                                    FontWeight::NORMAL
                                })
                                .text_color(rgb(if selected {
                                    pal.accent
                                } else {
                                    pal.text_primary
                                }))
                                .child(Self::policy_label(pol, lang)),
                        )
                        .when(selected, |row| {
                            row.child(tg_icon(IconName::SelectCheck, 13.0, pal.accent))
                        }),
                );
            }
            policy_card = policy_card.child(menu);
        }

        let top_more_menu = if self.detail_more_open {
            Some(
                div()
                    .p(px(10.0))
                    .border_t_1()
                    .border_color(rgb(pal.border_fg))
                    .bg(rgb(pal.input_bg))
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .id("sc-detail-test-btn")
                            .cursor_pointer()
                            .px(px(10.0))
                            .py(px(5.0))
                            .rounded(px(7.0))
                            .bg(rgb(pal.card_bg))
                            .border_1()
                            .border_color(rgb(pal.border_fg))
                            .text_size(px(12.0))
                            .text_color(rgb(pal.text_primary))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| {
                                    this.detail_more_open = false;
                                    this.pending_event =
                                        Some(SettingsComputersEvent::TestNode(nid_test.clone()));
                                    cx.notify();
                                }),
                            )
                            .child(tr_str(lang, "测试连接", "Test Connection")),
                    )
                    .when(is_cloud, |row| {
                        let edit_snapshot = node_clone_for_edit.clone();
                        row.child(
                            div()
                                .id("sc-detail-edit-cloud")
                                .cursor_pointer()
                                .px(px(10.0))
                                .py(px(5.0))
                                .rounded(px(7.0))
                                .bg(rgb(pal.card_bg))
                                .border_1()
                                .border_color(rgb(pal.border_fg))
                                .text_size(px(12.0))
                                .text_color(rgb(pal.accent))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _, _, cx| {
                                        this.open_cloud_form(this.current_bot_id, Some(&edit_snapshot));
                                        cx.notify();
                                    }),
                                )
                                .child(tr_str(lang, "编辑配置", "Edit Cloud Config")),
                        )
                        .child(
                            div()
                                .id("sc-detail-del-cloud")
                                .cursor_pointer()
                                .px(px(10.0))
                                .py(px(5.0))
                                .rounded(px(7.0))
                                .bg(rgb(pal.card_bg))
                                .border_1()
                                .border_color(rgb(pal.danger))
                                .text_size(px(12.0))
                                .text_color(rgb(pal.danger))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _, _, cx| {
                                        this.detail_more_open = false;
                                        this.pending_event = Some(
                                            SettingsComputersEvent::DeleteNode(nid_del.clone()),
                                        );
                                        cx.notify();
                                    }),
                                )
                                .child(tr_str(lang, "删除云服务器", "Delete Cloud Server")),
                        )
                    }),
            )
        } else {
            None
        };

        settings_container(pal)
            .child(self.detail_back_bar(
                Some(tr_str(lang, "计算机详情", "Computer Details")),
                pal,
                lang,
                cx,
            ))
            .when_some(self.status_message.as_ref(), |col, msg| {
                col.child(
                    div()
                        .p(px(10.0))
                        .rounded(px(8.0))
                        .bg(TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.14))
                        .text_size(px(12.0))
                        .text_color(rgb(pal.text_primary))
                        .child(msg.clone()),
                )
            })
            .child(
                settings_card(pal)
                    .child(
                        div()
                            .p(px(16.0))
                            .flex()
                            .items_center()
                            .justify_between()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(12.0))
                                    .child(
                                        div()
                                            .w(px(44.0))
                                            .h(px(44.0))
                                            .rounded(px(12.0))
                                            .bg(TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.14))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .child(tg_icon(IconName::ComputerDesktop, 22.0, pal.accent)),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(4.0))
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap(px(8.0))
                                                    .child(
                                                        div()
                                                            .text_size(px(15.0))
                                                            .font_weight(FontWeight::BOLD)
                                                            .text_color(rgb(pal.text_primary))
                                                            .child(node.display_name.clone()),
                                                    )
                                                    .child(
                                                        div()
                                                            .px(px(8.0))
                                                            .py(px(2.0))
                                                            .rounded(px(999.0))
                                                            .bg(rgb(pal.input_bg))
                                                            .text_size(px(11.0))
                                                            .text_color(rgb(pal.text_secondary))
                                                            .child(badge.to_string()),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap(px(6.0))
                                                    .child(
                                                        div()
                                                            .w(px(7.0))
                                                            .h(px(7.0))
                                                            .rounded_full()
                                                            .bg(rgb(if node.online {
                                                                pal.success
                                                            } else {
                                                                pal.text_secondary
                                                            })),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(12.0))
                                                            .text_color(rgb(pal.text_secondary))
                                                            .child(format!(
                                                                "{} · {} · {}",
                                                                if node.online {
                                                                    tr_str(lang, "在线", "Online")
                                                                } else {
                                                                    tr_str(lang, "离线", "Offline")
                                                                },
                                                                endpoint,
                                                                node.os_name,
                                                            )),
                                                    ),
                                            ),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .child(
                                        div()
                                            .id("sc-detail-remote-control")
                                            .cursor_pointer()
                                            .px(px(12.0))
                                            .py(px(7.0))
                                            .rounded(px(8.0))
                                            .bg(rgb(pal.accent))
                                            .hover(|s| s.bg(rgb(pal.accent_hover)))
                                            .text_size(px(12.0))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(pal.active_button_fg))
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(move |this, _, _, cx| {
                                                    this.pending_event = Some(
                                                        SettingsComputersEvent::OpenRemoteWorkbench(
                                                            nid_wb.clone(),
                                                        ),
                                                    );
                                                    cx.notify();
                                                }),
                                            )
                                            .child(tr_str(lang, "远程操控", "Remote Control")),
                                    )
                                    .child(
                                        div()
                                            .id("sc-detail-more-btn")
                                            .cursor_pointer()
                                            .w(px(30.0))
                                            .h(px(30.0))
                                            .rounded(px(8.0))
                                            .bg(rgb(pal.input_bg))
                                            .hover(|s| s.bg(rgb(pal.hover_bg)))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(|this, _, _, cx| {
                                                    this.detail_more_open = !this.detail_more_open;
                                                    cx.notify();
                                                }),
                                            )
                                            .child(tg_icon(IconName::MoreVertical, 15.0, pal.icon_fg)),
                                    ),
                            ),
                    )
                    .when_some(top_more_menu, |card, menu| card.child(menu)),
            )
            .child(acl_card)
            .child(policy_card)
            .into_any_element()
    }

    fn detail_back_bar(
        &self,
        title: Option<&str>,
        pal: TdesktopThemePalette,
        lang: &str,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        div()
            .id("sc-detail-back-bar")
            .cursor_pointer()
            .flex()
            .items_center()
            .gap(px(6.0))
            .text_size(px(13.5))
            .font_weight(FontWeight::BOLD)
            .text_color(rgb(pal.accent))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.open_overview();
                    this.pending_event = Some(SettingsComputersEvent::BackToOverview);
                    cx.notify();
                }),
            )
            .child(tg_icon(IconName::BackArrow, 15.0, pal.accent))
            .child(
                title
                    .unwrap_or_else(|| tr_str(lang, "返回电脑列表", "Back to Computers"))
                    .to_string(),
            )
            .into_any_element()
    }

    fn render_cloud_form(
        &self,
        pal: TdesktopThemePalette,
        lang: &str,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let auth_choices = [
            (
                CloudAuthMethod::SshPassword,
                tr_str(lang, "SSH 密码", "SSH Password"),
            ),
            (
                CloudAuthMethod::SshPrivateKey,
                tr_str(lang, "SSH 私钥", "SSH Private Key"),
            ),
            (
                CloudAuthMethod::AgentHttpToken,
                tr_str(lang, "Agent HTTP Token", "Agent HTTP Token"),
            ),
        ];

        settings_container(pal)
            .child(self.detail_back_bar(
                Some(tr_str(lang, "配置云服务器", "Configure Cloud Server")),
                pal,
                lang,
                cx,
            ))
            .when_some(self.status_message.as_ref(), |col, msg| {
                col.child(
                    div()
                        .p(px(10.0))
                        .rounded(px(8.0))
                        .bg(TdesktopThemePalette::rgba_with_alpha(pal.danger, 0.14))
                        .text_size(px(12.0))
                        .text_color(rgb(pal.danger))
                        .child(msg.clone()),
                )
            })
            .child(
                settings_card(pal)
                    .p(px(16.0))
                    .gap(px(12.0))
                    .child(self.form_field_box(
                        "sc-f-name",
                        tr_str(lang, "名称", "Display Name"),
                        &self.form_name,
                        self.input_name.as_ref(),
                        CloudFormField::DisplayName,
                        "prod-cloud-01",
                        false,
                        pal,
                        cx,
                    ))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(10.0))
                            .child(
                                div().flex_1().child(self.form_field_box(
                                    "sc-f-host",
                                    tr_str(lang, "主机地址 / IP", "Host / IP"),
                                    &self.form_host,
                                    self.input_host.as_ref(),
                                    CloudFormField::Host,
                                    "203.0.113.10",
                                    false,
                                    pal,
                                    cx,
                                )),
                            )
                            .child(
                                div().w(px(110.0)).child(self.form_field_box(
                                    "sc-f-port",
                                    tr_str(lang, "端口", "Port"),
                                    &self.form_port,
                                    self.input_port.as_ref(),
                                    CloudFormField::Port,
                                    "22",
                                    false,
                                    pal,
                                    cx,
                                )),
                            ),
                    )
                    .child(self.form_field_box(
                        "sc-f-user",
                        tr_str(lang, "用户名", "Username"),
                        &self.form_username,
                        self.input_username.as_ref(),
                        CloudFormField::Username,
                        "root",
                        false,
                        pal,
                        cx,
                    ))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(6.0))
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(pal.text_secondary))
                                    .child(tr_str(lang, "认证方式", "Auth Method")),
                            )
                            .child(div().flex().flex_wrap().gap(px(6.0)).children(
                                auth_choices.into_iter().enumerate().map(|(i, (method, label))| {
                                    let active = self.form_auth_method == method;
                                    div()
                                        .id(("sc-auth-method", i))
                                        .cursor_pointer()
                                        .px(px(10.0))
                                        .py(px(6.0))
                                        .rounded(px(8.0))
                                        .bg(if active {
                                            TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.16)
                                        } else {
                                            rgb(pal.input_bg).into()
                                        })
                                        .border_1()
                                        .border_color(rgb(if active {
                                            pal.accent
                                        } else {
                                            pal.border_fg
                                        }))
                                        .text_size(px(12.0))
                                        .font_weight(if active {
                                            FontWeight::SEMIBOLD
                                        } else {
                                            FontWeight::NORMAL
                                        })
                                        .text_color(rgb(if active {
                                            pal.accent
                                        } else {
                                            pal.text_primary
                                        }))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _, _, cx| {
                                                this.form_auth_method = method;
                                                cx.notify();
                                            }),
                                        )
                                        .child(label.to_string())
                                }),
                            )),
                    )
                    .child(self.form_field_box(
                        "sc-f-cred",
                        tr_str(lang, "密码 / 私钥 / Token", "Password / Private Key / Token"),
                        &self.form_credential,
                        self.input_credential.as_ref(),
                        CloudFormField::Credential,
                        if self.form_auth_method == CloudAuthMethod::SshPrivateKey {
                            "-----BEGIN OPENSSH PRIVATE KEY-----"
                        } else {
                            "••••••••"
                        },
                        self.form_auth_method == CloudAuthMethod::SshPassword,
                        pal,
                        cx,
                    ))
                    .child(self.form_field_box(
                        "sc-f-agent-url",
                        tr_str(lang, "Agent URL (可选)", "Agent URL (Optional)"),
                        &self.form_agent_url,
                        self.input_agent_url.as_ref(),
                        CloudFormField::AgentUrl,
                        "http://203.0.113.10:8787",
                        false,
                        pal,
                        cx,
                    ))
                    .child(self.form_field_box(
                        "sc-f-workdir",
                        tr_str(lang, "工作目录", "Workdir"),
                        &self.form_workdir,
                        self.input_workdir.as_ref(),
                        CloudFormField::Workdir,
                        "/workspace",
                        false,
                        pal,
                        cx,
                    ))
                    .child(
                        div()
                            .id("sc-f-bind-bot-toggle")
                            .cursor_pointer()
                            .py(px(6.0))
                            .flex()
                            .items_center()
                            .justify_between()
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.form_bind_to_current_bot = !this.form_bind_to_current_bot;
                                    cx.notify();
                                }),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(rgb(pal.text_primary))
                                            .child(tr_str(
                                                lang,
                                                "绑定到当前 Bot",
                                                "Bind to Current Bot",
                                            )),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(11.0))
                                            .text_color(rgb(pal.text_secondary))
                                            .child(match self.current_bot_id {
                                                Some(bid) => format!("Bot #{bid}"),
                                                None => tr_str(
                                                    lang,
                                                    "当前未在 Bot 对话中打开",
                                                    "No active Bot selected",
                                                )
                                                .to_string(),
                                            }),
                                    ),
                            )
                            .child(
                                div()
                                    .w(px(38.0))
                                    .h(px(20.0))
                                    .rounded_full()
                                    .bg(rgb(if self.form_bind_to_current_bot {
                                        pal.accent
                                    } else {
                                        pal.input_bg
                                    }))
                                    .border_1()
                                    .border_color(rgb(if self.form_bind_to_current_bot {
                                        pal.accent
                                    } else {
                                        pal.border_fg
                                    }))
                                    .px(px(2.0))
                                    .flex()
                                    .items_center()
                                    .justify_end()
                                    .when(!self.form_bind_to_current_bot, |el| el.justify_start())
                                    .child(div().w(px(14.0)).h(px(14.0)).rounded_full().bg(rgb(0xFFFFFF))),
                            ),
                    )
                    .child(
                        div()
                            .pt(px(6.0))
                            .flex()
                            .items_center()
                            .justify_end()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .id("sc-f-cancel-btn")
                                    .cursor_pointer()
                                    .px(px(14.0))
                                    .py(px(8.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(pal.input_bg))
                                    .border_1()
                                    .border_color(rgb(pal.border_fg))
                                    .text_size(px(12.5))
                                    .text_color(rgb(pal.text_primary))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            this.open_overview();
                                            cx.notify();
                                        }),
                                    )
                                    .child(tr_str(lang, "取消", "Cancel")),
                            )
                            .child(
                                div()
                                    .id("sc-f-save-btn")
                                    .cursor_pointer()
                                    .px(px(16.0))
                                    .py(px(8.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(pal.accent))
                                    .hover(|s| s.bg(rgb(pal.accent_hover)))
                                    .text_size(px(12.5))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.active_button_fg))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            this.submit_cloud_form();
                                            cx.notify();
                                        }),
                                    )
                                    .child(tr_str(
                                        lang,
                                        "测试连接并保存到云端",
                                        "Test & Save to Cloud",
                                    )),
                            ),
                    ),
            )
            .into_any_element()
    }

    #[allow(clippy::too_many_arguments)]
    fn form_field_box(
        &self,
        id: &'static str,
        label: &str,
        value: &str,
        ctrl: Option<&Entity<TextInputController>>,
        field: CloudFormField,
        placeholder: &str,
        masked: bool,
        pal: TdesktopThemePalette,
        cx: &mut Context<Self>,
    ) -> Div {
        let focused = self.form_focus == field;
        let (committed, preedit) = ctrl
            .map(|c| c.read(cx).text_and_preedit(value.to_string()))
            .unwrap_or_else(|| (value.to_string(), None));

        div()
            .flex()
            .flex_col()
            .gap(px(5.0))
            .child(
                div()
                    .text_size(px(12.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(pal.text_secondary))
                    .child(label.to_string()),
            )
            .child(
                div()
                    .id(id)
                    .cursor_text()
                    .min_h(px(36.0))
                    .px(px(10.0))
                    .py(px(7.0))
                    .rounded(px(8.0))
                    .bg(rgb(pal.input_bg))
                    .border_1()
                    .border_color(rgb(if focused { pal.accent } else { pal.border_fg }))
                    .flex()
                    .items_center()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            this.ensure_form_inputs(cx);
                            this.form_focus = field;
                            cx.notify();
                        }),
                    )
                    .child(render_ime_text_preview(
                        &committed,
                        preedit.as_deref(),
                        placeholder,
                        focused,
                        masked,
                        pal.text_primary,
                        pal,
                    )),
            )
    }
}

impl Render for SettingsComputersSections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.ensure_form_inputs(cx);
        let pal = self.palette;
        let lang = self.language_id.clone();
        match self.page.clone() {
            SettingsComputersPage::Overview => self.render_overview(pal, &lang, cx),
            SettingsComputersPage::Detail(node_id) => {
                self.render_detail(&node_id, pal, &lang, cx)
            }
            SettingsComputersPage::CloudForm => self.render_cloud_form(pal, &lang, cx),
        }
    }
}
