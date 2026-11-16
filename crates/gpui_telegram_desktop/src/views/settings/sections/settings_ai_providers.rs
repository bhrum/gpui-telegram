//! Pure-Rust GPUI Settings View for Magpie AI Subscriptions, Presets, Routing & Privacy Masking.
//! Backend contract: `crates/gpui_telegram_client/src/net/hermes_client.rs` (`Magpie*Dto`).

use std::collections::{BTreeMap, HashMap};

use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_telegram_client::net::hermes_client::{
    MagpieImportRequest, MagpieMaskingConfigDto, MagpieMaskingPreviewRequest,
    MagpieMaskingPreviewResponse, MagpiePresetDto, MagpieProbeResponse, MagpieProviderDto,
    MagpieRoutingGroupDto, MagpieRoutingGroupMemberDto, MagpieSaveProviderRequest, MagpieStateDto,
};

use crate::components::ui::{
    TextInputController, delete_prev_word, pop_last_char, sanitize_paste_text,
};
use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::theme::palette::TdesktopThemePalette;
use crate::views::computer_workbench::render_ime_text_preview;
use crate::views::settings::settings_theme_ui::{
    settings_card, settings_container, settings_empty_state, settings_section_header,
    settings_subsection_title, settings_toggle_row,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AiProvidersTab {
    #[default]
    Overview,
    Routing,
    PrivacyImport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AiPresetCategory {
    #[default]
    All,
    Vendor,
    Relay,
    Local,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AiProviderField {
    #[default]
    None,
    PresetSearch,
    ProviderId,
    DisplayName,
    BaseUrl,
    ApiKey,
    Model,
    GroupId,
    GroupName,
    GroupTargets,
    GroupFastModel,
    GroupDeepModel,
    MaskCustomWords,
    MaskAllowList,
    MaskPreviewText,
    ImportText,
}

#[derive(Debug, Clone)]
pub enum SettingsAiProvidersEvent {
    RefreshState,
    ScanSubscriptions,
    SaveProvider(MagpieSaveProviderRequest),
    DeleteProvider(String),
    ProbeProvider {
        provider_id: String,
        model: Option<String>,
    },
    ToggleAllowLocalFallback(bool),
    SetRoutingStrategy(String),
    UpsertRoutingGroup(MagpieRoutingGroupDto),
    DeleteRoutingGroup(String),
    UpdateMasking(MagpieMaskingConfigDto),
    PreviewMasking(MagpieMaskingPreviewRequest),
    ImportConfig(MagpieImportRequest),
    CopyText(String),
}

pub struct SettingsAiProvidersSections {
    pub palette: TdesktopThemePalette,
    pub language_id: String,
    pub tab: AiProvidersTab,
    pub category: AiPresetCategory,
    pub state: Option<MagpieStateDto>,
    pub loading: bool,
    pub status_message: Option<String>,
    pub error_message: Option<String>,
    pub probe_result: Option<MagpieProbeResponse>,
    pub masking_preview_result: Option<MagpieMaskingPreviewResponse>,

    // Provider preset / custom editor state
    pub editor_open: bool,
    pub selected_preset_id: Option<String>,
    pub selected_region_id: Option<String>,
    pub draft_provider_id: String,
    pub draft_name: String,
    pub draft_api_mode: String,
    pub draft_base_url: String,
    pub draft_api_key: String,
    pub draft_model: String,
    pub draft_no_key: bool,
    pub draft_editing_has_key: bool,
    pub draft_allow_local_fallback: bool,
    pub preset_search: String,

    // Routing group form drafts
    pub group_id_draft: String,
    pub group_name_draft: String,
    pub group_strategy_draft: String,
    pub group_targets_draft: String,
    pub group_fast_model_draft: String,
    pub group_deep_model_draft: String,

    // Masking & Import drafts
    pub masking_draft: MagpieMaskingConfigDto,
    pub mask_custom_words_draft: String,
    pub mask_allow_list_draft: String,
    pub mask_preview_draft: String,
    pub import_raw_draft: String,
    pub import_activate: bool,

    pub active_field: AiProviderField,
    pub ime_controller: Option<Entity<TextInputController>>,
    pub pending_event: Option<SettingsAiProvidersEvent>,
}

impl SettingsAiProvidersSections {
    /// Creates a new live AI Providers settings section view initialized with `palette` and `lang`.
    pub fn new_live(palette: TdesktopThemePalette, lang: &str) -> Self {
        Self {
            palette,
            language_id: lang.to_string(),
            tab: AiProvidersTab::Overview,
            category: AiPresetCategory::All,
            state: None,
            loading: true,
            status_message: None,
            error_message: None,
            probe_result: None,
            masking_preview_result: None,
            editor_open: false,
            selected_preset_id: None,
            selected_region_id: None,
            draft_provider_id: String::new(),
            draft_name: String::new(),
            draft_api_mode: "chat_completions".to_string(),
            draft_base_url: String::new(),
            draft_api_key: String::new(),
            draft_model: String::new(),
            draft_no_key: false,
            draft_editing_has_key: false,
            draft_allow_local_fallback: false,
            preset_search: String::new(),
            group_id_draft: String::new(),
            group_name_draft: String::new(),
            group_strategy_draft: "smart".to_string(),
            group_targets_draft: String::new(),
            group_fast_model_draft: String::new(),
            group_deep_model_draft: String::new(),
            masking_draft: MagpieMaskingConfigDto::default(),
            mask_custom_words_draft: String::new(),
            mask_allow_list_draft: String::new(),
            mask_preview_draft: String::new(),
            import_raw_draft: String::new(),
            import_activate: true,
            active_field: AiProviderField::None,
            ime_controller: None,
            pending_event: None,
        }
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) {
        self.palette = palette;
    }

    pub fn set_language(&mut self, lang: &str) {
        self.language_id = lang.to_string();
    }

    pub fn take_event(&mut self) -> Option<SettingsAiProvidersEvent> {
        self.pending_event.take()
    }

    /// Hydrates live Magpie backend state into the view while preserving unsaved input drafts.
    pub fn sync_state(&mut self, state: MagpieStateDto) {
        self.loading = false;
        self.draft_allow_local_fallback = state.allow_local_fallback;
        self.masking_draft = state.masking.clone();
        if self.active_field != AiProviderField::MaskCustomWords {
            self.mask_custom_words_draft = state.masking.custom_words.join(", ");
        }
        if self.active_field != AiProviderField::MaskAllowList {
            self.mask_allow_list_draft = state.masking.allow_list.join(", ");
        }
        if self.group_strategy_draft.trim().is_empty() && !state.routing_strategy.is_empty() {
            self.group_strategy_draft = state.routing_strategy.clone();
        }
        self.state = Some(state);
    }

    pub fn set_status(&mut self, status: Option<String>, error: Option<String>) {
        self.loading = false;
        self.status_message = status;
        self.error_message = error;
    }

    fn current_preset(&self) -> Option<&MagpiePresetDto> {
        let pid = self.selected_preset_id.as_deref()?;
        self.state
            .as_ref()
            .and_then(|st| st.presets.iter().find(|p| p.id == pid))
    }

    fn resolve_preset_url(
        preset: &MagpiePresetDto,
        region_id: Option<&str>,
        proto: &str,
    ) -> (Option<String>, Vec<String>) {
        let region = region_id
            .and_then(|rid| preset.regions.iter().find(|r| r.id == rid))
            .or_else(|| preset.regions.first());
        let pick_url = |chat: &Option<String>,
                        resp: &Option<String>,
                        anth: &Option<String>,
                        gem: &Option<String>| {
            match proto {
                "codex_responses" => resp.clone().or_else(|| chat.clone()),
                "anthropic_messages" => anth.clone().or_else(|| chat.clone()),
                "gemini_generate_content" => gem.clone().or_else(|| chat.clone()),
                _ => chat
                    .clone()
                    .or_else(|| resp.clone())
                    .or_else(|| anth.clone())
                    .or_else(|| gem.clone()),
            }
        };
        if let Some(reg) = region {
            let url = pick_url(&reg.chat_url, &reg.responses_url, &reg.anthropic_url, &reg.gemini_url)
                .or_else(|| {
                    pick_url(
                        &preset.chat_url,
                        &preset.responses_url,
                        &preset.anthropic_url,
                        &preset.gemini_url,
                    )
                });
            let models = if reg.models.is_empty() {
                preset.suggested_models.clone()
            } else {
                reg.models.clone()
            };
            (url, models)
        } else {
            let url = pick_url(
                &preset.chat_url,
                &preset.responses_url,
                &preset.anthropic_url,
                &preset.gemini_url,
            );
            (url, preset.suggested_models.clone())
        }
    }

    /// Populates the provider configuration editor from a catalog preset ID.
    pub fn open_preset_editor(&mut self, preset_id: &str) {
        let preset_opt = self
            .state
            .as_ref()
            .and_then(|st| st.presets.iter().find(|p| p.id == preset_id))
            .cloned();
        self.tab = AiProvidersTab::Overview;
        self.editor_open = true;
        self.error_message = None;
        self.probe_result = None;
        self.draft_api_key.clear();

        if let Some(preset) = preset_opt {
            let reg_id = preset.regions.first().map(|r| r.id.clone());
            let proto = if preset.default_protocol.trim().is_empty() {
                "chat_completions".to_string()
            } else {
                preset.default_protocol.clone()
            };
            let (url, models) = Self::resolve_preset_url(&preset, reg_id.as_deref(), &proto);
            self.selected_preset_id = Some(preset.id.clone());
            self.selected_region_id = reg_id;
            self.draft_provider_id = preset.id.clone();
            self.draft_name = preset.name.clone();
            self.draft_api_mode = proto;
            self.draft_base_url = url.unwrap_or_default();
            self.draft_model = if !preset.default_model.trim().is_empty() {
                preset.default_model.clone()
            } else {
                models.first().cloned().unwrap_or_default()
            };
            self.draft_no_key = preset.no_key;
            self.draft_editing_has_key = false;
            self.active_field = if preset.no_key {
                AiProviderField::Model
            } else {
                AiProviderField::ApiKey
            };
        } else {
            self.selected_preset_id = None;
            self.selected_region_id = None;
            self.draft_provider_id = "custom".to_string();
            self.draft_name = "Custom Provider".to_string();
            self.draft_api_mode = "chat_completions".to_string();
            self.draft_base_url.clear();
            self.draft_model.clear();
            self.draft_no_key = false;
            self.draft_editing_has_key = false;
            self.active_field = AiProviderField::BaseUrl;
        }
    }

    /// Populates the provider configuration editor from an existing saved provider.
    pub fn open_provider_editor(&mut self, prov: &MagpieProviderDto) {
        self.tab = AiProvidersTab::Overview;
        self.editor_open = true;
        self.error_message = None;
        self.probe_result = None;
        self.selected_preset_id = prov.preset_id.clone();
        self.selected_region_id = prov.region_id.clone();
        self.draft_provider_id = prov.provider_id.clone();
        self.draft_name = prov.name.clone();
        self.draft_api_mode = if prov.api_mode.trim().is_empty() {
            "chat_completions".to_string()
        } else {
            prov.api_mode.clone()
        };
        self.draft_base_url = prov.base_url.clone().unwrap_or_default();
        self.draft_api_key.clear();
        self.draft_model = prov.default_model.clone();
        self.draft_no_key = prov.no_key_required;
        self.draft_editing_has_key = prov.has_api_key;
        self.active_field = if prov.no_key_required {
            AiProviderField::Model
        } else {
            AiProviderField::ApiKey
        };
    }

    pub fn select_region(&mut self, region_id: &str) {
        self.selected_region_id = Some(region_id.to_string());
        if let Some(preset) = self.current_preset().cloned() {
            let proto = self.draft_api_mode.clone();
            let (url, models) = Self::resolve_preset_url(&preset, Some(region_id), &proto);
            if let Some(u) = url {
                self.draft_base_url = u;
            }
            if !models.is_empty() && !models.contains(&self.draft_model) {
                self.draft_model = models[0].clone();
            }
        }
    }

    pub fn select_protocol(&mut self, proto: &str) {
        self.draft_api_mode = proto.to_string();
        if let Some(preset) = self.current_preset().cloned() {
            let rid = self.selected_region_id.clone();
            let (url, _) = Self::resolve_preset_url(&preset, rid.as_deref(), proto);
            if let Some(u) = url {
                self.draft_base_url = u;
            }
        }
    }

    /// Validates and emits `SettingsAiProvidersEvent::SaveProvider` for the active form.
    pub fn submit_provider_form(&mut self) -> bool {
        let pid = self.draft_provider_id.trim().to_string();
        if pid.is_empty() {
            self.error_message = Some(
                tr_str(
                    &self.language_id,
                    "请填写供应商 ID (例如 deepseek / openai / ollama)",
                    "Please enter a Provider ID (e.g. deepseek / openai / ollama)",
                )
                .to_string(),
            );
            return false;
        }
        let key_trimmed = self.draft_api_key.trim().to_string();
        if !self.draft_no_key
            && self.draft_api_mode != "local_hermes"
            && key_trimmed.is_empty()
            && !self.draft_editing_has_key
        {
            self.error_message = Some(
                tr_str(
                    &self.language_id,
                    "请输入该供应商的 API Key（或选择无需 Key 的本地 Ollama）",
                    "Please enter an API Key for this provider (or choose keyless local Ollama)",
                )
                .to_string(),
            );
            return false;
        }
        self.error_message = None;
        self.loading = true;
        self.pending_event = Some(SettingsAiProvidersEvent::SaveProvider(
            MagpieSaveProviderRequest {
                preset_id: self.selected_preset_id.clone(),
                provider_id: Some(pid),
                name: opt_non_empty(&self.draft_name),
                region_id: self.selected_region_id.clone(),
                api_key: if key_trimmed.is_empty() {
                    None
                } else {
                    Some(key_trimmed)
                },
                api_mode: opt_non_empty(&self.draft_api_mode),
                base_url: opt_non_empty(&self.draft_base_url),
                default_model: opt_non_empty(&self.draft_model),
                custom_headers: HashMap::new(),
                enabled: Some(true),
                no_key_required: Some(self.draft_no_key),
                activate: true,
                allow_local_fallback: None,
            },
        ));
        true
    }

    /// Parses group drafts into `MagpieRoutingGroupDto` and emits `UpsertRoutingGroup`.
    pub fn submit_group_form(&mut self) -> bool {
        let id = self.group_id_draft.trim().to_string();
        if id.is_empty() {
            self.error_message = Some(
                tr_str(
                    &self.language_id,
                    "请填写路由组 ID (如 coding / daily)",
                    "Please enter a group ID (e.g. coding / daily)",
                )
                .to_string(),
            );
            return false;
        }
        let members: Vec<MagpieRoutingGroupMemberDto> = self
            .group_targets_draft
            .split([',', '\n'])
            .filter_map(|raw| {
                let token = raw.trim();
                if token.is_empty() {
                    return None;
                }
                let (target, weight) = match token.split_once('@') {
                    Some((t, w)) => (t.trim().to_string(), w.trim().parse::<u32>().unwrap_or(1).max(1)),
                    None => (token.to_string(), 1),
                };
                if target.is_empty() {
                    None
                } else {
                    Some(MagpieRoutingGroupMemberDto {
                        target,
                        weight,
                        enabled: true,
                        input_cost_per_1m: None,
                        output_cost_per_1m: None,
                        free: false,
                    })
                }
            })
            .collect();
        if members.is_empty() {
            self.error_message = Some(
                tr_str(
                    &self.language_id,
                    "请至少填写一个目标模型 (如 deepseek/deepseek-chat, openai/gpt-4o-mini)",
                    "Enter at least one target (e.g. deepseek/deepseek-chat, openai/gpt-4o-mini)",
                )
                .to_string(),
            );
            return false;
        }
        self.error_message = None;
        let name = if self.group_name_draft.trim().is_empty() {
            id.clone()
        } else {
            self.group_name_draft.trim().to_string()
        };
        let strategy = if self.group_strategy_draft.trim().is_empty() {
            "smart".to_string()
        } else {
            self.group_strategy_draft.trim().to_string()
        };
        self.pending_event = Some(SettingsAiProvidersEvent::UpsertRoutingGroup(
            MagpieRoutingGroupDto {
                id,
                name,
                strategy,
                members,
                intent_fast_model: opt_non_empty(&self.group_fast_model_draft),
                intent_deep_model: opt_non_empty(&self.group_deep_model_draft),
                sticky_ttl_ms: 300_000,
            },
        ));
        true
    }

    pub fn submit_masking_form(&mut self) {
        let mut cfg = self.masking_draft.clone();
        cfg.custom_words = split_csv(&self.mask_custom_words_draft);
        cfg.allow_list = split_csv(&self.mask_allow_list_draft);
        if cfg.kind_actions.is_empty() {
            cfg.kind_actions = BTreeMap::new();
        }
        self.masking_draft = cfg.clone();
        self.pending_event = Some(SettingsAiProvidersEvent::UpdateMasking(cfg));
    }

    fn ensure_ime_controller(&mut self, cx: &mut Context<Self>) {
        if self.ime_controller.is_none() {
            self.ime_controller = Some(cx.new(|_| TextInputController::new()));
        }
    }

    pub fn is_input_active(&self) -> bool {
        self.active_field != AiProviderField::None
    }

    pub fn active_field_text(&self) -> &str {
        match self.active_field {
            AiProviderField::None => "",
            AiProviderField::PresetSearch => &self.preset_search,
            AiProviderField::ProviderId => &self.draft_provider_id,
            AiProviderField::DisplayName => &self.draft_name,
            AiProviderField::BaseUrl => &self.draft_base_url,
            AiProviderField::ApiKey => &self.draft_api_key,
            AiProviderField::Model => &self.draft_model,
            AiProviderField::GroupId => &self.group_id_draft,
            AiProviderField::GroupName => &self.group_name_draft,
            AiProviderField::GroupTargets => &self.group_targets_draft,
            AiProviderField::GroupFastModel => &self.group_fast_model_draft,
            AiProviderField::GroupDeepModel => &self.group_deep_model_draft,
            AiProviderField::MaskCustomWords => &self.mask_custom_words_draft,
            AiProviderField::MaskAllowList => &self.mask_allow_list_draft,
            AiProviderField::MaskPreviewText => &self.mask_preview_draft,
            AiProviderField::ImportText => &self.import_raw_draft,
        }
    }

    fn active_field_mut(&mut self) -> Option<&mut String> {
        match self.active_field {
            AiProviderField::None => None,
            AiProviderField::PresetSearch => Some(&mut self.preset_search),
            AiProviderField::ProviderId => Some(&mut self.draft_provider_id),
            AiProviderField::DisplayName => Some(&mut self.draft_name),
            AiProviderField::BaseUrl => Some(&mut self.draft_base_url),
            AiProviderField::ApiKey => Some(&mut self.draft_api_key),
            AiProviderField::Model => Some(&mut self.draft_model),
            AiProviderField::GroupId => Some(&mut self.group_id_draft),
            AiProviderField::GroupName => Some(&mut self.group_name_draft),
            AiProviderField::GroupTargets => Some(&mut self.group_targets_draft),
            AiProviderField::GroupFastModel => Some(&mut self.group_fast_model_draft),
            AiProviderField::GroupDeepModel => Some(&mut self.group_deep_model_draft),
            AiProviderField::MaskCustomWords => Some(&mut self.mask_custom_words_draft),
            AiProviderField::MaskAllowList => Some(&mut self.mask_allow_list_draft),
            AiProviderField::MaskPreviewText => Some(&mut self.mask_preview_draft),
            AiProviderField::ImportText => Some(&mut self.import_raw_draft),
        }
    }

    pub fn active_input_text_and_preedit(&self, cx: &App) -> (String, Option<String>) {
        let committed = self.active_field_text().to_string();
        if let Some(ctrl) = self.ime_controller.as_ref() {
            ctrl.read(cx).text_and_preedit(committed)
        } else {
            (committed, None)
        }
    }

    pub fn clear_active_input(&mut self, cx: &mut Context<Self>) {
        if let Some(ctrl) = self.ime_controller.clone() {
            ctrl.update(cx, |c, _| c.clear_preedit());
        }
        if let Some(dst) = self.active_field_mut() {
            dst.clear();
        }
        cx.notify();
    }

    pub fn push_input_text(&mut self, text: &str, cx: &mut Context<Self>) {
        let allow_nl = matches!(
            self.active_field,
            AiProviderField::GroupTargets | AiProviderField::ImportText | AiProviderField::MaskPreviewText
        );
        let clean = sanitize_paste_text(text, allow_nl);
        if clean.is_empty() {
            return;
        }
        if let Some(ctrl) = self.ime_controller.clone() {
            ctrl.update(cx, |c, _| c.clear_preedit());
        }
        if let Some(dst) = self.active_field_mut() {
            dst.push_str(&clean);
        }
        cx.notify();
    }

    pub fn set_ime_preedit(&mut self, preedit: Option<&str>, cx: &mut Context<Self>) {
        self.ensure_ime_controller(cx);
        if let Some(ctrl) = self.ime_controller.clone() {
            ctrl.update(cx, |c, _| {
                c.ime_preedit = preedit.filter(|s| !s.is_empty()).map(str::to_string);
            });
            cx.notify();
        }
    }

    fn tab_order_for_context(&self) -> &'static [AiProviderField] {
        if self.tab == AiProvidersTab::Overview && self.editor_open {
            &[
                AiProviderField::ProviderId,
                AiProviderField::DisplayName,
                AiProviderField::BaseUrl,
                AiProviderField::Model,
                AiProviderField::ApiKey,
            ]
        } else if self.tab == AiProvidersTab::Routing {
            &[
                AiProviderField::GroupId,
                AiProviderField::GroupName,
                AiProviderField::GroupTargets,
                AiProviderField::GroupFastModel,
                AiProviderField::GroupDeepModel,
            ]
        } else if self.tab == AiProvidersTab::PrivacyImport {
            &[
                AiProviderField::MaskCustomWords,
                AiProviderField::MaskAllowList,
                AiProviderField::MaskPreviewText,
                AiProviderField::ImportText,
            ]
        } else {
            &[AiProviderField::PresetSearch]
        }
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
                let order = self.tab_order_for_context();
                if let Some(pos) = order.iter().position(|f| *f == self.active_field) {
                    let next = if is_shift {
                        (pos + order.len() - 1) % order.len()
                    } else {
                        (pos + 1) % order.len()
                    };
                    self.active_field = order[next];
                    cx.notify();
                }
                true
            }
            "enter" => {
                match self.active_field {
                    AiProviderField::ProviderId
                    | AiProviderField::DisplayName
                    | AiProviderField::BaseUrl
                    | AiProviderField::ApiKey
                    | AiProviderField::Model => {
                        self.submit_provider_form();
                    }
                    AiProviderField::GroupId
                    | AiProviderField::GroupName
                    | AiProviderField::GroupTargets
                    | AiProviderField::GroupFastModel
                    | AiProviderField::GroupDeepModel => {
                        self.submit_group_form();
                    }
                    AiProviderField::MaskPreviewText => {
                        self.pending_event = Some(SettingsAiProvidersEvent::PreviewMasking(
                            MagpieMaskingPreviewRequest {
                                text: self.mask_preview_draft.clone(),
                                config: Some(self.masking_draft.clone()),
                            },
                        ));
                    }
                    AiProviderField::ImportText => {
                        if !self.import_raw_draft.trim().is_empty() {
                            self.pending_event = Some(SettingsAiProvidersEvent::ImportConfig(
                                MagpieImportRequest {
                                    raw: self.import_raw_draft.clone(),
                                    activate: self.import_activate,
                                },
                            ));
                        }
                    }
                    _ => {}
                }
                cx.notify();
                true
            }
            _ => false,
        }
    }

    fn pill_btn(
        id: &'static str,
        label: &str,
        bg_color: u32,
        fg_color: u32,
        on_click: impl Fn(&mut Self, &mut Context<Self>) + 'static,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        div()
            .id(id)
            .cursor_pointer()
            .px(px(10.0))
            .py(px(5.0))
            .rounded(px(7.0))
            .bg(rgb(bg_color))
            .text_xs()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(rgb(fg_color))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, _, cx| {
                    on_click(this, cx);
                    cx.notify();
                }),
            )
            .child(label.to_string())
    }

    fn render_input_field(
        &self,
        id: &'static str,
        field: AiProviderField,
        label: &str,
        value: &str,
        placeholder: &str,
        masked: bool,
        pal: TdesktopThemePalette,
        cx: &mut Context<Self>,
    ) -> Div {
        let focused = self.active_field == field;
        let preedit = if focused {
            self.ime_controller
                .as_ref()
                .and_then(|c| c.read(cx).ime_preedit.clone())
        } else {
            None
        };
        div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(
                div()
                    .text_size(px(11.5))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(pal.text_secondary))
                    .child(label.to_string()),
            )
            .child(
                div()
                    .id(id)
                    .cursor_text()
                    .min_h(px(34.0))
                    .px(px(10.0))
                    .py(px(6.0))
                    .rounded(px(8.0))
                    .bg(rgb(pal.input_bg))
                    .border_1()
                    .border_color(rgb(if focused { pal.accent } else { pal.border_fg }))
                    .flex()
                    .items_center()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            this.ensure_ime_controller(cx);
                            this.active_field = field;
                            cx.notify();
                        }),
                    )
                    .child(render_ime_text_preview(
                        value,
                        preedit.as_deref(),
                        placeholder,
                        focused,
                        masked,
                        pal.text_primary,
                        pal,
                    )),
            )
    }

    fn render_status_banners(&self, pal: TdesktopThemePalette) -> AnyElement {
        div()
            .flex()
            .flex_col()
            .gap(px(6.0))
            .when_some(self.error_message.clone(), |el, err| {
                el.child(
                    div()
                        .px(px(10.0))
                        .py(px(7.0))
                        .rounded(px(8.0))
                        .bg(TdesktopThemePalette::rgba_with_alpha(pal.danger, 0.14))
                        .border_1()
                        .border_color(rgb(pal.danger))
                        .text_xs()
                        .text_color(rgb(pal.danger))
                        .child(err),
                )
            })
            .when_some(self.status_message.clone(), |el, msg| {
                el.child(
                    div()
                        .px(px(10.0))
                        .py(px(7.0))
                        .rounded(px(8.0))
                        .bg(TdesktopThemePalette::rgba_with_alpha(pal.success, 0.14))
                        .border_1()
                        .border_color(rgb(pal.success))
                        .text_xs()
                        .text_color(rgb(pal.success))
                        .child(msg),
                )
            })
            .when_some(self.probe_result.clone(), |el, p| {
                let ok = p.ok;
                let col = if ok { pal.success } else { pal.danger };
                let text = if ok {
                    format!(
                        "Probe OK: {} ({}) · {} ms — {}",
                        p.provider_id,
                        p.model,
                        p.latency_ms,
                        p.reply_preview.unwrap_or_default()
                    )
                } else {
                    format!(
                        "Probe Failed: {} ({}) · {}",
                        p.provider_id,
                        p.model,
                        p.error.unwrap_or_else(|| "error".to_string())
                    )
                };
                el.child(
                    div()
                        .px(px(10.0))
                        .py(px(7.0))
                        .rounded(px(8.0))
                        .bg(TdesktopThemePalette::rgba_with_alpha(col, 0.14))
                        .border_1()
                        .border_color(rgb(col))
                        .text_xs()
                        .text_color(rgb(col))
                        .child(text),
                )
            })
            .into_any_element()
    }

    fn render_overview_tab(
        &self,
        pal: TdesktopThemePalette,
        lang: &str,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        if self.editor_open {
            return self.render_provider_editor(pal, lang, cx);
        }

        let st_opt = self.state.as_ref();
        let needs_sel = st_opt.is_none_or(|s| s.needs_provider_selection);
        let subs = st_opt.map(|s| s.subscriptions.clone()).unwrap_or_default();
        let providers = st_opt.map(|s| s.providers.clone()).unwrap_or_default();
        let presets = st_opt.map(|s| s.presets.clone()).unwrap_or_default();
        let q = self.preset_search.trim().to_ascii_lowercase();

        let filtered_presets: Vec<MagpiePresetDto> = presets
            .into_iter()
            .filter(|p| {
                let cat_ok = match self.category {
                    AiPresetCategory::All => true,
                    AiPresetCategory::Vendor => p.kind == "vendor",
                    AiPresetCategory::Relay => p.kind == "relay",
                    AiPresetCategory::Local => p.kind == "local",
                    AiPresetCategory::Custom => p.kind == "remote" || p.kind == "custom",
                };
                if !cat_ok {
                    return false;
                }
                if q.is_empty() {
                    return true;
                }
                p.id.to_ascii_lowercase().contains(&q)
                    || p.name.to_ascii_lowercase().contains(&q)
                    || p.default_model.to_ascii_lowercase().contains(&q)
            })
            .collect();

        div()
            .flex()
            .flex_col()
            .gap(px(12.0))
            // 1. Subscription status & fallback guard
            .child(
                settings_card(pal)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(settings_subsection_title(
                                tr_str(lang, "本地 AI 订阅检测", "Detected AI Subscriptions"),
                                pal,
                            ))
                            .child(Self::pill_btn(
                                "magpie-scan-subs-btn",
                                tr_str(lang, "扫描本地订阅", "Rescan Local"),
                                pal.input_bg,
                                pal.accent,
                                |this, _| {
                                    this.loading = true;
                                    this.pending_event =
                                        Some(SettingsAiProvidersEvent::ScanSubscriptions);
                                },
                                cx,
                            )),
                    )
                    .when(needs_sel, |el| {
                        el.child(
                            div()
                                .p(px(10.0))
                                .rounded(px(8.0))
                                .bg(TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.14))
                                .border_1()
                                .border_color(rgb(pal.accent))
                                .flex()
                                .flex_col()
                                .gap(px(4.0))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(pal.text_primary))
                                        .child(tr_str(
                                            lang,
                                            "未检测到激活的 AI 订阅或已配置供应商",
                                            "No Active AI Subscription or Configured Provider",
                                        )),
                                )
                                .child(
                                    div()
                                        .text_size(px(11.5))
                                        .text_color(rgb(pal.text_secondary))
                                        .child(tr_str(
                                            lang,
                                            "请在下方选择一个模型供应商预设（DeepSeek、Kimi、智谱 GLM、OpenAI、Anthropic、Gemini 或本地 Ollama）并保存。",
                                            "Pick a provider preset below (DeepSeek, Kimi, Zhipu GLM, OpenAI, Anthropic, Gemini, or local Ollama) to activate AI replies.",
                                        )),
                                ),
                        )
                    })
                    .when(!subs.is_empty(), |el| {
                        el.child(div().flex().flex_col().gap(px(6.0)).children(subs.into_iter().map(
                            |sub| {
                                let status_col = if sub.active { pal.success } else { pal.text_secondary };
                                div()
                                    .px(px(9.0))
                                    .py(px(6.0))
                                    .rounded(px(7.0))
                                    .bg(rgb(pal.input_bg))
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(pal.text_primary))
                                                    .child(format!(
                                                        "{} · {}",
                                                        sub.label,
                                                        sub.plan.as_deref().unwrap_or(&sub.kind)
                                                    )),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(11.0))
                                                    .text_color(rgb(pal.text_secondary))
                                                    .child(sub.email.unwrap_or(sub.source_path)),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(status_col))
                                            .child(if sub.active {
                                                tr_str(lang, "已激活", "Active")
                                            } else {
                                                tr_str(lang, "未登录/过期", "Inactive")
                                            }),
                                    )
                            },
                        )))
                    })
                    .when(subs_is_empty(st_opt) && !needs_sel, |el| {
                        el.child(
                            div()
                                .text_xs()
                                .text_color(rgb(pal.text_secondary))
                                .child(tr_str(
                                    lang,
                                    "当前通过已配置的模型供应商提供服务。",
                                    "Serving via configured API/Local provider.",
                                )),
                        )
                    })
                    .child(settings_toggle_row(
                        IconName::BotRobot,
                        tr_str(lang, "无供应商时允许本地回退 (hermes-local)", "Allow local hermes fallback when unconfigured"),
                        None,
                        self.draft_allow_local_fallback,
                        pal,
                        |this| {
                            this.draft_allow_local_fallback = !this.draft_allow_local_fallback;
                            this.pending_event = Some(SettingsAiProvidersEvent::ToggleAllowLocalFallback(
                                this.draft_allow_local_fallback,
                            ));
                        },
                        cx,
                    )),
            )
            // 2. Configured providers
            .child(
                settings_card(pal)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(settings_subsection_title(
                                tr_str(lang, "已配置供应商", "Configured Providers"),
                                pal,
                            ))
                            .child(Self::pill_btn(
                                "magpie-add-custom-btn",
                                tr_str(lang, "+ 自定义供应商", "+ Custom"),
                                pal.accent,
                                pal.active_button_fg,
                                |this, _| this.open_preset_editor("custom-openai"),
                                cx,
                            )),
                    )
                    .when(providers.is_empty(), |el| {
                        el.child(settings_empty_state(
                            IconName::BotRobot,
                            tr_str(lang, "尚未配置外部 AI 供应商", "No providers configured yet"),
                            pal,
                        ))
                    })
                    .children(providers.into_iter().map(|prov| {
                        let pid_act = prov.provider_id.clone();
                        let pid_probe = prov.provider_id.clone();
                        let pid_del = prov.provider_id.clone();
                        let model_probe = opt_non_empty(&prov.default_model);
                        let prov_edit = prov.clone();
                        div()
                            .p(px(9.0))
                            .rounded(px(8.0))
                            .bg(rgb(pal.input_bg))
                            .border_1()
                            .border_color(rgb(if prov.is_active { pal.accent } else { pal.border_fg }))
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
                                            .flex()
                                            .flex_col()
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(pal.text_primary))
                                                    .child(format!(
                                                        "{} ({}) · {}",
                                                        prov.name, prov.provider_id, prov.default_model
                                                    )),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(11.0))
                                                    .text_color(rgb(pal.text_secondary))
                                                    .child(format!(
                                                        "{} · {}",
                                                        prov.api_mode,
                                                        if prov.no_key_required {
                                                            "no-key".to_string()
                                                        } else {
                                                            prov.masked_key.unwrap_or_else(|| "missing key".to_string())
                                                        }
                                                    )),
                                            ),
                                    )
                                    .when(prov.is_active, |el| {
                                        el.child(
                                            div()
                                                .px(px(7.0))
                                                .py(px(2.0))
                                                .rounded(px(5.0))
                                                .bg(rgb(pal.success))
                                                .text_size(px(10.5))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgb(pal.active_button_fg))
                                                .child(tr_str(lang, "当前使用", "Active")),
                                        )
                                    }),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .gap(px(6.0))
                                    .when(!prov.is_active, |row| {
                                        row.child(Self::pill_btn(
                                            "magpie-activate-prov-btn",
                                            tr_str(lang, "设为默认", "Activate"),
                                            pal.accent,
                                            pal.active_button_fg,
                                            move |this, _| {
                                                this.pending_event = Some(SettingsAiProvidersEvent::SaveProvider(
                                                    MagpieSaveProviderRequest {
                                                        provider_id: Some(pid_act.clone()),
                                                        enabled: Some(true),
                                                        activate: true,
                                                        ..Default::default()
                                                    },
                                                ));
                                            },
                                            cx,
                                        ))
                                    })
                                    .child(Self::pill_btn(
                                        "magpie-probe-prov-btn",
                                        tr_str(lang, "测试连通", "Probe"),
                                        pal.card_bg,
                                        pal.accent,
                                        move |this, _| {
                                            this.pending_event = Some(SettingsAiProvidersEvent::ProbeProvider {
                                                provider_id: pid_probe.clone(),
                                                model: model_probe.clone(),
                                            });
                                        },
                                        cx,
                                    ))
                                    .child(Self::pill_btn(
                                        "magpie-edit-prov-btn",
                                        tr_str(lang, "编辑", "Edit"),
                                        pal.card_bg,
                                        pal.text_primary,
                                        move |this, _| this.open_provider_editor(&prov_edit),
                                        cx,
                                    ))
                                    .when(prov.provider_id != "local_hermes", |row| {
                                        row.child(Self::pill_btn(
                                            "magpie-del-prov-btn",
                                            tr_str(lang, "删除", "Delete"),
                                            pal.card_bg,
                                            pal.danger,
                                            move |this, _| {
                                                this.pending_event = Some(
                                                    SettingsAiProvidersEvent::DeleteProvider(pid_del.clone()),
                                                );
                                            },
                                            cx,
                                        ))
                                    }),
                            )
                    })),
            )
            // 3. Magpie 45+ Preset Catalog
            .child(
                settings_card(pal)
                    .child(settings_subsection_title(
                        tr_str(lang, "选择供应商预设 (Magpie 目录)", "Choose Provider Preset (Magpie Catalog)"),
                        pal,
                    ))
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap(px(6.0))
                            .children(
                                [
                                    (AiPresetCategory::All, tr_str(lang, "全部", "All")),
                                    (AiPresetCategory::Vendor, tr_str(lang, "官方厂商", "Vendors")),
                                    (AiPresetCategory::Relay, tr_str(lang, "聚合中转", "Relays")),
                                    (AiPresetCategory::Local, tr_str(lang, "本地推理", "Local")),
                                    (AiPresetCategory::Custom, tr_str(lang, "远程/自定义", "Remote & Custom")),
                                ]
                                .into_iter()
                                .map(|(cat, lbl)| {
                                    let active = self.category == cat;
                                    Self::pill_btn(
                                        "magpie-cat-filter-btn",
                                        lbl,
                                        if active { pal.accent } else { pal.input_bg },
                                        if active { pal.active_button_fg } else { pal.text_secondary },
                                        move |this, _| this.category = cat,
                                        cx,
                                    )
                                }),
                            ),
                    )
                    .child(self.render_input_field(
                        "magpie-preset-search-input",
                        AiProviderField::PresetSearch,
                        tr_str(lang, "搜索预设 / 模型", "Filter presets or models"),
                        &self.preset_search,
                        tr_str(lang, "输入 deepseek / kimi / gemini / ollama...", "Search deepseek / kimi / gemini / ollama..."),
                        false,
                        pal,
                        cx,
                    ))
                    .when(filtered_presets.is_empty(), |el| {
                        el.child(settings_empty_state(
                            IconName::Search,
                            tr_str(lang, "没有匹配的供应商预设", "No matching presets"),
                            pal,
                        ))
                    })
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap(px(8.0))
                            .children(filtered_presets.into_iter().map(|p| {
                                let pid = p.id.clone();
                                let badge = if p.no_key {
                                    tr_str(lang, "免 Key", "No Key")
                                } else {
                                    "API Key"
                                };
                                div()
                                    .id(SharedString::from(format!("magpie-preset-card-{}", p.id)))
                                    .cursor_pointer()
                                    .w(px(148.0))
                                    .p(px(9.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(pal.input_bg))
                                    .border_1()
                                    .border_color(rgb(pal.border_fg))
                                    .hover(move |s| s.border_color(rgb(pal.accent)))
                                    .flex()
                                    .flex_col()
                                    .gap(px(3.0))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _, _, cx| {
                                            this.open_preset_editor(&pid);
                                            cx.notify();
                                        }),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(pal.text_primary))
                                            .child(p.short_name.unwrap_or(p.name)),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(10.5))
                                            .text_color(rgb(pal.text_secondary))
                                            .child(format!("{} · {}", badge, p.default_model)),
                                    )
                            })),
                    ),
            )
            .into_any_element()
    }

    fn render_provider_editor(
        &self,
        pal: TdesktopThemePalette,
        lang: &str,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let preset_opt = self.current_preset().cloned();
        let regions = preset_opt.as_ref().map(|p| p.regions.clone()).unwrap_or_default();
        let protocols = preset_opt
            .as_ref()
            .filter(|p| !p.supported_protocols.is_empty())
            .map(|p| p.supported_protocols.clone())
            .unwrap_or_else(|| {
                vec![
                    "chat_completions".to_string(),
                    "codex_responses".to_string(),
                    "anthropic_messages".to_string(),
                    "gemini_generate_content".to_string(),
                ]
            });
        let suggested_models = preset_opt
            .as_ref()
            .map(|p| {
                Self::resolve_preset_url(p, self.selected_region_id.as_deref(), &self.draft_api_mode).1
            })
            .unwrap_or_default();
        let keys_url = preset_opt.as_ref().and_then(|p| {
            self.selected_region_id
                .as_deref()
                .and_then(|rid| p.regions.iter().find(|r| r.id == rid))
                .and_then(|r| r.keys_url.clone())
                .or_else(|| p.keys_url.clone())
        });
        let note = preset_opt.as_ref().and_then(|p| p.note.clone());
        let key_hint = preset_opt
            .as_ref()
            .and_then(|p| p.key_hint.clone())
            .unwrap_or_else(|| "sk-...".to_string());

        div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(
                settings_card(pal)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(settings_subsection_title(
                                &format!(
                                    "{}: {}",
                                    tr_str(lang, "配置模型供应商", "Configure Provider"),
                                    if self.draft_name.is_empty() {
                                        &self.draft_provider_id
                                    } else {
                                        &self.draft_name
                                    }
                                ),
                                pal,
                            ))
                            .child(Self::pill_btn(
                                "magpie-editor-back-btn",
                                tr_str(lang, "返回目录", "Back to Catalog"),
                                pal.input_bg,
                                pal.text_primary,
                                |this, _| {
                                    this.editor_open = false;
                                    this.active_field = AiProviderField::None;
                                },
                                cx,
                            )),
                    )
                    .when_some(note, |el, n| {
                        el.child(
                            div()
                                .text_size(px(11.5))
                                .text_color(rgb(pal.text_secondary))
                                .child(n),
                        )
                    })
                    .when(!regions.is_empty(), |el| {
                        el.child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(4.0))
                                .child(
                                    div()
                                        .text_size(px(11.5))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(rgb(pal.text_secondary))
                                        .child(tr_str(lang, "区域 / 计费通道", "Region / Plan")),
                                )
                                .child(div().flex().flex_wrap().gap(px(6.0)).children(
                                    regions.into_iter().map(|r| {
                                        let active =
                                            self.selected_region_id.as_deref() == Some(r.id.as_str());
                                        let rid = r.id.clone();
                                        Self::pill_btn(
                                            "magpie-region-pill",
                                            &r.name,
                                            if active { pal.accent } else { pal.input_bg },
                                            if active {
                                                pal.active_button_fg
                                            } else {
                                                pal.text_primary
                                            },
                                            move |this, _| this.select_region(&rid),
                                            cx,
                                        )
                                    }),
                                )),
                        )
                    })
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(4.0))
                            .child(
                                div()
                                    .text_size(px(11.5))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(pal.text_secondary))
                                    .child(tr_str(lang, "协议格式", "Protocol")),
                            )
                            .child(div().flex().flex_wrap().gap(px(6.0)).children(
                                protocols.into_iter().map(|proto| {
                                    let active = self.draft_api_mode == proto;
                                    let p_clone = proto.clone();
                                    Self::pill_btn(
                                        "magpie-proto-pill",
                                        &proto,
                                        if active { pal.accent } else { pal.input_bg },
                                        if active {
                                            pal.active_button_fg
                                        } else {
                                            pal.text_primary
                                        },
                                        move |this, _| this.select_protocol(&p_clone),
                                        cx,
                                    )
                                }),
                            )),
                    )
                    .child(self.render_input_field(
                        "magpie-field-provider-id",
                        AiProviderField::ProviderId,
                        tr_str(lang, "供应商 ID", "Provider ID"),
                        &self.draft_provider_id,
                        "deepseek",
                        false,
                        pal,
                        cx,
                    ))
                    .child(self.render_input_field(
                        "magpie-field-display-name",
                        AiProviderField::DisplayName,
                        tr_str(lang, "显示名称", "Display Name"),
                        &self.draft_name,
                        "DeepSeek",
                        false,
                        pal,
                        cx,
                    ))
                    .child(self.render_input_field(
                        "magpie-field-base-url",
                        AiProviderField::BaseUrl,
                        "Base URL",
                        &self.draft_base_url,
                        "https://api.deepseek.com/v1",
                        false,
                        pal,
                        cx,
                    ))
                    .when(!suggested_models.is_empty(), |el| {
                        el.child(div().flex().flex_wrap().gap(px(5.0)).children(
                            suggested_models.into_iter().map(|m| {
                                let active = self.draft_model == m;
                                let m_val = m.clone();
                                Self::pill_btn(
                                    "magpie-model-chip",
                                    &m,
                                    if active { pal.accent } else { pal.input_bg },
                                    if active { pal.active_button_fg } else { pal.text_primary },
                                    move |this, _| this.draft_model = m_val.clone(),
                                    cx,
                                )
                            }),
                        ))
                    })
                    .child(self.render_input_field(
                        "magpie-field-model",
                        AiProviderField::Model,
                        tr_str(lang, "默认模型", "Default Model"),
                        &self.draft_model,
                        "deepseek-chat",
                        false,
                        pal,
                        cx,
                    ))
                    .when(!self.draft_no_key, |el| {
                        let ph = if self.draft_editing_has_key {
                            tr_str(lang, "已配置 Key · 留空保持不变", "Key saved · leave blank to keep")
                                .to_string()
                        } else {
                            key_hint.clone()
                        };
                        el.child(self.render_input_field(
                            "magpie-field-api-key",
                            AiProviderField::ApiKey,
                            "API Key",
                            &self.draft_api_key,
                            &ph,
                            true,
                            pal,
                            cx,
                        ))
                    })
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap(px(8.0))
                            .pt(px(4.0))
                            .child(Self::pill_btn(
                                "magpie-save-provider-btn",
                                tr_str(lang, "保存并启用", "Save & Activate"),
                                pal.accent,
                                pal.active_button_fg,
                                |this, _| {
                                    this.submit_provider_form();
                                },
                                cx,
                            ))
                            .when_some(keys_url, |row, kurl| {
                                row.child(Self::pill_btn(
                                    "magpie-copy-keys-url-btn",
                                    tr_str(lang, "复制获取 Key 链接", "Copy Key Console URL"),
                                    pal.input_bg,
                                    pal.text_link,
                                    move |this, _| {
                                        this.pending_event =
                                            Some(SettingsAiProvidersEvent::CopyText(kurl.clone()));
                                    },
                                    cx,
                                ))
                            }),
                    ),
            )
            .into_any_element()
    }

    fn render_routing_tab(
        &self,
        pal: TdesktopThemePalette,
        lang: &str,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let strategies = ["smart", "order", "rotate", "usage", "pace", "cheapest", "drain"];
        let active_strat = self
            .state
            .as_ref()
            .map(|s| s.routing_strategy.as_str())
            .unwrap_or("smart");
        let groups = self
            .state
            .as_ref()
            .map(|s| s.routing_groups.clone())
            .unwrap_or_default();
        let traces = self
            .state
            .as_ref()
            .map(|s| s.recent_route_traces.clone())
            .unwrap_or_default();

        div()
            .flex()
            .flex_col()
            .gap(px(12.0))
            .child(
                settings_card(pal)
                    .child(settings_subsection_title(
                        tr_str(lang, "全局智能路由策略 (Magpie Routing)", "Global Routing Strategy"),
                        pal,
                    ))
                    .child(div().flex().flex_wrap().gap(px(6.0)).children(strategies.into_iter().map(
                        |s| {
                            let act = active_strat == s;
                            let strat = s.to_string();
                            Self::pill_btn(
                                "magpie-global-strategy-btn",
                                s,
                                if act { pal.accent } else { pal.input_bg },
                                if act { pal.active_button_fg } else { pal.text_primary },
                                move |this, _| {
                                    this.pending_event = Some(
                                        SettingsAiProvidersEvent::SetRoutingStrategy(strat.clone()),
                                    );
                                },
                                cx,
                            )
                        },
                    ))),
            )
            .child(
                settings_card(pal)
                    .child(settings_subsection_title(
                        tr_str(lang, "模型聚合组 (group/<id>)", "Model Routing Groups (group/<id>)"),
                        pal,
                    ))
                    .children(groups.into_iter().map(|g| {
                        let gid = g.id.clone();
                        let targets = g
                            .members
                            .iter()
                            .map(|m| format!("{}@{}", m.target, m.weight))
                            .collect::<Vec<_>>()
                            .join(", ");
                        div()
                            .p(px(8.0))
                            .rounded(px(8.0))
                            .bg(rgb(pal.input_bg))
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(pal.text_primary))
                                            .child(format!("group/{} · {} ({})", g.id, g.name, g.strategy)),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(11.0))
                                            .text_color(rgb(pal.text_secondary))
                                            .child(targets),
                                    ),
                            )
                            .child(Self::pill_btn(
                                "magpie-del-group-btn",
                                tr_str(lang, "删除", "Delete"),
                                pal.card_bg,
                                pal.danger,
                                move |this, _| {
                                    this.pending_event = Some(
                                        SettingsAiProvidersEvent::DeleteRoutingGroup(gid.clone()),
                                    );
                                },
                                cx,
                            ))
                    }))
                    .child(self.render_input_field(
                        "magpie-group-id-input",
                        AiProviderField::GroupId,
                        tr_str(lang, "分组 ID", "Group ID"),
                        &self.group_id_draft,
                        "coding",
                        false,
                        pal,
                        cx,
                    ))
                    .child(self.render_input_field(
                        "magpie-group-name-input",
                        AiProviderField::GroupName,
                        tr_str(lang, "分组名称", "Group Name"),
                        &self.group_name_draft,
                        "Coding Pool",
                        false,
                        pal,
                        cx,
                    ))
                    .child(self.render_input_field(
                        "magpie-group-targets-input",
                        AiProviderField::GroupTargets,
                        tr_str(lang, "成员目标 (逗号分隔)", "Targets (comma-separated, optional @weight)"),
                        &self.group_targets_draft,
                        "deepseek/deepseek-chat@2, openai/gpt-4o-mini@1",
                        false,
                        pal,
                        cx,
                    ))
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap(px(6.0))
                            .children(["smart", "order", "rotate", "cheapest"].into_iter().map(|s| {
                                let act = self.group_strategy_draft == s;
                                let val = s.to_string();
                                Self::pill_btn(
                                    "magpie-group-strat-btn",
                                    s,
                                    if act { pal.accent } else { pal.input_bg },
                                    if act { pal.active_button_fg } else { pal.text_primary },
                                    move |this, _| this.group_strategy_draft = val.clone(),
                                    cx,
                                )
                            })),
                    )
                    .child(self.render_input_field(
                        "magpie-group-fast-input",
                        AiProviderField::GroupFastModel,
                        tr_str(lang, "快速意图模型 (可选)", "Fast Intent Model (optional)"),
                        &self.group_fast_model_draft,
                        "deepseek/deepseek-chat",
                        false,
                        pal,
                        cx,
                    ))
                    .child(self.render_input_field(
                        "magpie-group-deep-input",
                        AiProviderField::GroupDeepModel,
                        tr_str(lang, "深度推理模型 (可选)", "Deep Intent Model (optional)"),
                        &self.group_deep_model_draft,
                        "anthropic/claude-sonnet-4-6",
                        false,
                        pal,
                        cx,
                    ))
                    .child(Self::pill_btn(
                        "magpie-save-group-btn",
                        tr_str(lang, "保存路由组", "Save Routing Group"),
                        pal.accent,
                        pal.active_button_fg,
                        |this, _| {
                            this.submit_group_form();
                        },
                        cx,
                    )),
            )
            .child(
                settings_card(pal)
                    .child(settings_subsection_title(
                        tr_str(lang, "最近路由决策轨迹", "Recent Route Decision Traces"),
                        pal,
                    ))
                    .when(traces.is_empty(), |el| {
                        el.child(settings_empty_state(
                            IconName::TerminalMcp,
                            tr_str(lang, "暂无路由记录", "No route traces recorded yet"),
                            pal,
                        ))
                    })
                    .children(traces.into_iter().rev().take(6).map(|tr| {
                        div()
                            .px(px(8.0))
                            .py(px(5.0))
                            .rounded(px(6.0))
                            .bg(rgb(pal.input_bg))
                            .text_size(px(11.0))
                            .text_color(rgb(pal.text_secondary))
                            .child(format!(
                                "{} -> {}/{} [{}] ({}) · mask={}",
                                tr.requested_model,
                                tr.resolved_provider_id,
                                tr.resolved_model,
                                tr.strategy,
                                tr.reason,
                                tr.masking_hits
                            ))
                    })),
            )
            .into_any_element()
    }

    fn render_privacy_import_tab(
        &self,
        pal: TdesktopThemePalette,
        lang: &str,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        div()
            .flex()
            .flex_col()
            .gap(px(12.0))
            .child(
                settings_card(pal)
                    .child(settings_subsection_title(
                        tr_str(lang, "请求隐私脱敏 (MaskingEngine)", "Prompt Privacy Masking"),
                        pal,
                    ))
                    .child(settings_toggle_row(
                        IconName::Lock,
                        tr_str(lang, "启用请求敏感信息自动脱敏", "Enable Automatic Prompt Masking"),
                        Some(tr_str(
                            lang,
                            "外发给上游 LLM 前替换密钥、JWT、邮箱与银行卡并在回包恢复",
                            "Masks secrets/PII before upstream calls and unmasks responses locally",
                        )),
                        self.masking_draft.enabled,
                        pal,
                        |this| this.masking_draft.enabled = !this.masking_draft.enabled,
                        cx,
                    ))
                    .child(settings_toggle_row(
                        IconName::Lock,
                        tr_str(lang, "脱敏 API Key / Bearer / PEM", "Mask Secrets (API Keys / JWT / PEM)"),
                        None,
                        self.masking_draft.mask_secrets,
                        pal,
                        |this| this.masking_draft.mask_secrets = !this.masking_draft.mask_secrets,
                        cx,
                    ))
                    .child(settings_toggle_row(
                        IconName::ContactsUsers,
                        tr_str(lang, "脱敏个人邮箱与银行卡", "Mask Personal Info (Emails & Luhn Cards)"),
                        None,
                        self.masking_draft.mask_personal,
                        pal,
                        |this| this.masking_draft.mask_personal = !this.masking_draft.mask_personal,
                        cx,
                    ))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .children(["mask", "warn", "block"].into_iter().map(|act| {
                                let is_sel = self.masking_draft.default_action == act;
                                let val = act.to_string();
                                Self::pill_btn(
                                    "magpie-mask-action-btn",
                                    act,
                                    if is_sel { pal.accent } else { pal.input_bg },
                                    if is_sel { pal.active_button_fg } else { pal.text_primary },
                                    move |this, _| this.masking_draft.default_action = val.clone(),
                                    cx,
                                )
                            })),
                    )
                    .child(self.render_input_field(
                        "magpie-mask-custom-words",
                        AiProviderField::MaskCustomWords,
                        tr_str(lang, "自定义脱敏词 (逗号分隔)", "Custom Words (comma-separated)"),
                        &self.mask_custom_words_draft,
                        "INTERNAL_CODENAME, ACME_CORP",
                        false,
                        pal,
                        cx,
                    ))
                    .child(self.render_input_field(
                        "magpie-mask-allow-list",
                        AiProviderField::MaskAllowList,
                        tr_str(lang, "白名单豁免 (逗号分隔)", "Allow List (comma-separated)"),
                        &self.mask_allow_list_draft,
                        "support@example.com",
                        false,
                        pal,
                        cx,
                    ))
                    .child(Self::pill_btn(
                        "magpie-save-masking-btn",
                        tr_str(lang, "保存脱敏规则", "Save Masking Settings"),
                        pal.accent,
                        pal.active_button_fg,
                        |this, _| this.submit_masking_form(),
                        cx,
                    ))
                    .child(self.render_input_field(
                        "magpie-mask-preview-input",
                        AiProviderField::MaskPreviewText,
                        tr_str(lang, "脱敏效果实时试测", "Test Masking Preview"),
                        &self.mask_preview_draft,
                        "My key is sk-live-secret1234567890",
                        false,
                        pal,
                        cx,
                    ))
                    .child(Self::pill_btn(
                        "magpie-run-mask-preview-btn",
                        tr_str(lang, "预览脱敏结果", "Preview Masking"),
                        pal.input_bg,
                        pal.accent,
                        |this, _| {
                            let mut cfg = this.masking_draft.clone();
                            cfg.enabled = true;
                            cfg.custom_words = split_csv(&this.mask_custom_words_draft);
                            cfg.allow_list = split_csv(&this.mask_allow_list_draft);
                            this.pending_event = Some(SettingsAiProvidersEvent::PreviewMasking(
                                MagpieMaskingPreviewRequest {
                                    text: this.mask_preview_draft.clone(),
                                    config: Some(cfg),
                                },
                            ));
                        },
                        cx,
                    ))
                    .when_some(self.masking_preview_result.clone(), |el, res| {
                        el.child(
                            div()
                                .p(px(8.0))
                                .rounded(px(8.0))
                                .bg(rgb(pal.input_bg))
                                .text_xs()
                                .text_color(rgb(pal.text_primary))
                                .child(format!(
                                    "hits={} blocked={} => {}",
                                    res.hit_count, res.blocked, res.masked_text
                                )),
                        )
                    }),
            )
            .child(
                settings_card(pal)
                    .child(settings_subsection_title(
                        tr_str(lang, "一键导入配置 (magpie:// / .env)", "Import Config (magpie:// / .env)"),
                        pal,
                    ))
                    .child(self.render_input_field(
                        "magpie-import-raw-input",
                        AiProviderField::ImportText,
                        tr_str(lang, "粘贴导入链接或 .env 内容", "Paste magpie://import?... or .env lines"),
                        &self.import_raw_draft,
                        "magpie://import?preset=deepseek&key=sk-... or DEEPSEEK_API_KEY=sk-...",
                        false,
                        pal,
                        cx,
                    ))
                    .child(Self::pill_btn(
                        "magpie-import-submit-btn",
                        tr_str(lang, "立即导入并激活", "Import & Activate"),
                        pal.accent,
                        pal.active_button_fg,
                        |this, _| {
                            if !this.import_raw_draft.trim().is_empty() {
                                this.pending_event = Some(SettingsAiProvidersEvent::ImportConfig(
                                    MagpieImportRequest {
                                        raw: this.import_raw_draft.clone(),
                                        activate: this.import_activate,
                                    },
                                ));
                            }
                        },
                        cx,
                    )),
            )
            .into_any_element()
    }
}

fn subs_is_empty(st: Option<&MagpieStateDto>) -> bool {
    st.is_none_or(|s| s.subscriptions.is_empty())
}

fn opt_non_empty(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn split_csv(raw: &str) -> Vec<String> {
    raw.split([',', '\n'])
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect()
}

impl Render for SettingsAiProvidersSections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.ensure_ime_controller(cx);
        let pal = self.palette;
        let lang = self.language_id.clone();

        let sub_tabs = [
            (
                AiProvidersTab::Overview,
                tr_str(&lang, "订阅与供应商", "Providers & Presets"),
            ),
            (
                AiProvidersTab::Routing,
                tr_str(&lang, "路由组与策略", "Groups & Routing"),
            ),
            (
                AiProvidersTab::PrivacyImport,
                tr_str(&lang, "脱敏与导入", "Masking & Import"),
            ),
        ];

        settings_container(pal)
            .child(settings_section_header(
                IconName::BotRobot,
                tr_str(&lang, "AI 订阅与模型供应商 (Magpie)", "AI Subscriptions & Providers (Magpie)"),
                Some(tr_str(
                    &lang,
                    "管理本地 CLI 订阅、45+ 供应商预设、智能路由组与隐私脱敏",
                    "Manage local subscriptions, 45+ provider presets, smart routing groups, and privacy masking",
                )),
                pal,
            ))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .items_center()
                    .gap(px(8.0))
                    .children(sub_tabs.into_iter().map(|(t, label)| {
                        let active = self.tab == t;
                        Self::pill_btn(
                            "magpie-subtab-btn",
                            label,
                            if active { pal.accent } else { pal.card_bg },
                            if active { pal.active_button_fg } else { pal.text_primary },
                            move |this, _| {
                                this.tab = t;
                                this.active_field = AiProviderField::None;
                            },
                            cx,
                        )
                    }))
                    .child(Self::pill_btn(
                        "magpie-refresh-state-btn",
                        if self.loading {
                            tr_str(&lang, "同步中...", "Syncing...")
                        } else {
                            tr_str(&lang, "刷新", "Refresh")
                        },
                        pal.input_bg,
                        pal.text_secondary,
                        |this, _| {
                            this.loading = true;
                            this.pending_event = Some(SettingsAiProvidersEvent::RefreshState);
                        },
                        cx,
                    )),
            )
            .child(self.render_status_banners(pal))
            .child(match self.tab {
                AiProvidersTab::Overview => self.render_overview_tab(pal, &lang, cx),
                AiProvidersTab::Routing => self.render_routing_tab(pal, &lang, cx),
                AiProvidersTab::PrivacyImport => self.render_privacy_import_tab(pal, &lang, cx),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::TdesktopThemeMode;
    use gpui_telegram_client::net::hermes_client::MagpiePresetRegionDto;

    fn sample_state() -> MagpieStateDto {
        MagpieStateDto {
            presets: vec![
                MagpiePresetDto {
                    id: "deepseek".to_string(),
                    name: "DeepSeek".to_string(),
                    kind: "vendor".to_string(),
                    chat_url: Some("https://api.deepseek.com/v1".to_string()),
                    default_protocol: "chat_completions".to_string(),
                    supported_protocols: vec![
                        "chat_completions".to_string(),
                        "anthropic_messages".to_string(),
                    ],
                    anthropic_url: Some("https://api.deepseek.com/anthropic".to_string()),
                    default_model: "deepseek-chat".to_string(),
                    suggested_models: vec![
                        "deepseek-chat".to_string(),
                        "deepseek-reasoner".to_string(),
                    ],
                    no_key: false,
                    ..Default::default()
                },
                MagpiePresetDto {
                    id: "zhipu".to_string(),
                    name: "Zhipu GLM".to_string(),
                    kind: "vendor".to_string(),
                    default_protocol: "chat_completions".to_string(),
                    default_model: "glm-5".to_string(),
                    regions: vec![
                        MagpiePresetRegionDto {
                            id: "payg".to_string(),
                            name: "Pay as you go".to_string(),
                            chat_url: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
                            models: vec!["glm-5".to_string()],
                            ..Default::default()
                        },
                        MagpiePresetRegionDto {
                            id: "coding".to_string(),
                            name: "Coding Plan".to_string(),
                            chat_url: Some("https://open.bigmodel.cn/api/coding/paas/v4".to_string()),
                            models: vec!["glm-5-code".to_string()],
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
                MagpiePresetDto {
                    id: "ollama".to_string(),
                    name: "Ollama".to_string(),
                    kind: "local".to_string(),
                    chat_url: Some("http://127.0.0.1:11434/v1".to_string()),
                    default_protocol: "chat_completions".to_string(),
                    default_model: "qwen2.5-coder".to_string(),
                    no_key: true,
                    ..Default::default()
                },
            ],
            needs_provider_selection: true,
            routing_strategy: "smart".to_string(),
            ..Default::default()
        }
    }

    #[std::prelude::v1::test]
    fn preset_selection_region_protocol_and_validation_work() {
        let pal = TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12);
        let mut view = SettingsAiProvidersSections::new_live(pal, "zh-CN");
        view.sync_state(sample_state());

        view.open_preset_editor("deepseek");
        assert!(view.editor_open);
        assert_eq!(view.draft_provider_id, "deepseek");
        assert_eq!(view.draft_base_url, "https://api.deepseek.com/v1");
        // Missing key fails cleanly for non-keyless preset
        assert!(!view.submit_provider_form());
        assert!(view.error_message.is_some());

        view.select_protocol("anthropic_messages");
        assert_eq!(view.draft_base_url, "https://api.deepseek.com/anthropic");

        view.draft_api_key = "sk-ds-test-123".to_string();
        assert!(view.submit_provider_form());
        match view.take_event() {
            Some(SettingsAiProvidersEvent::SaveProvider(req)) => {
                assert_eq!(req.preset_id.as_deref(), Some("deepseek"));
                assert_eq!(req.api_key.as_deref(), Some("sk-ds-test-123"));
                assert!(req.activate);
            }
            other => panic!("expected SaveProvider, got {other:?}"),
        }

        // Region switching updates URL & model
        view.open_preset_editor("zhipu");
        assert_eq!(view.selected_region_id.as_deref(), Some("payg"));
        view.select_region("coding");
        assert_eq!(
            view.draft_base_url,
            "https://open.bigmodel.cn/api/coding/paas/v4"
        );
        assert_eq!(view.draft_model, "glm-5-code");

        // Keyless preset succeeds without API key
        view.open_preset_editor("ollama");
        assert!(view.draft_no_key);
        assert!(view.submit_provider_form());
        match view.take_event() {
            Some(SettingsAiProvidersEvent::SaveProvider(req)) => {
                assert_eq!(req.provider_id.as_deref(), Some("ollama"));
                assert_eq!(req.no_key_required, Some(true));
                assert!(req.api_key.is_none());
            }
            other => panic!("expected keyless SaveProvider, got {other:?}"),
        }
    }

    #[std::prelude::v1::test]
    fn routing_group_and_masking_forms_parse_cleanly() {
        let pal = TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12);
        let mut view = SettingsAiProvidersSections::new_live(pal, "en");
        view.sync_state(sample_state());

        view.group_id_draft = "coding".to_string();
        view.group_name_draft = "Code Pool".to_string();
        view.group_targets_draft = "deepseek/deepseek-chat@3, openai/gpt-4o-mini".to_string();
        view.group_fast_model_draft = "deepseek/deepseek-chat".to_string();
        assert!(view.submit_group_form());
        match view.take_event() {
            Some(SettingsAiProvidersEvent::UpsertRoutingGroup(g)) => {
                assert_eq!(g.id, "coding");
                assert_eq!(g.members.len(), 2);
                assert_eq!(g.members[0].weight, 3);
                assert_eq!(g.members[1].weight, 1);
                assert_eq!(g.intent_fast_model.as_deref(), Some("deepseek/deepseek-chat"));
            }
            other => panic!("expected UpsertRoutingGroup, got {other:?}"),
        }

        view.masking_draft.enabled = true;
        view.mask_custom_words_draft = "PROJECT_ZEUS, SECRET_TOKEN".to_string();
        view.mask_allow_list_draft = "help@example.com".to_string();
        view.submit_masking_form();
        match view.take_event() {
            Some(SettingsAiProvidersEvent::UpdateMasking(cfg)) => {
                assert!(cfg.enabled);
                assert_eq!(cfg.custom_words, vec!["PROJECT_ZEUS", "SECRET_TOKEN"]);
                assert_eq!(cfg.allow_list, vec!["help@example.com"]);
            }
            other => panic!("expected UpdateMasking, got {other:?}"),
        }
    }
}
