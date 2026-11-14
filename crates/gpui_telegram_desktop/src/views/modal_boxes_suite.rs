use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::boxes::edit_privacy_box::PrivacyRuleOption;
use crate::views::boxes::modal_render::render_dialog_overlay;
pub use crate::views::boxes::modal_types::{
    ActiveTextField, CallLogEntry, ModalActionOutcome, ModalDialogKind, StorySlideEntry,
};
use crate::views::boxes::report_messages_box::ReportReason;
use gpui::*;
use gpui_telegram_client::ProxyEndpointRecord;

pub struct ActiveModalDialog {
    pub kind: Option<ModalDialogKind>,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
    pub active_field: ActiveTextField,
    pub primary_input: String,
    pub secondary_input: String,
    pub tertiary_input: String,
    pub validation_error: Option<String>,
    pub poll_options: Vec<String>,
    pub poll_anonymous: bool,
    pub poll_multiple: bool,
    pub poll_quiz_mode: bool,
    pub poll_correct_idx: usize,
    pub flag_a: bool,
    pub flag_b: bool,
    pub selected_members: Vec<String>,
    pub contacts_list: Vec<(String, String, String, u32, bool)>,
    pub call_logs: Vec<CallLogEntry>,
    pub calls_missed_only: bool,
    pub proxies: Vec<ProxyEndpointRecord>,
    pub new_proxy_kind: String,
    pub target_lang: String,
    pub translated_text: Option<String>,
    pub crop_aspect: String,
    pub rotation_deg: u32,
    pub brush_color: u32,
    pub media_spoiler: bool,
    pub send_uncompressed: bool,
    pub selected_report_reason: Option<ReportReason>,
    pub selected_privacy_rule: PrivacyRuleOption,
    pub selected_wallpaper_id: String,
    pub iv_zoom_pct: u32,
    pub iv_serif: bool,
    pub iv_sepia: bool,
    pub stories: Vec<StorySlideEntry>,
    pub story_slide_idx: usize,
    pub export_status_path: Option<String>,
    pub forward_targets: Vec<(u64, String, String, u32)>,
    pub pending_outcome: Option<ModalActionOutcome>,
}

impl ActiveModalDialog {
    pub fn new(proxies: Vec<ProxyEndpointRecord>) -> Self {
        let palette = TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12);
        Self {
            kind: None,
            palette,
            language_id: "en".to_string(),
            active_field: ActiveTextField::Primary,
            primary_input: String::new(),
            secondary_input: String::new(),
            tertiary_input: String::new(),
            validation_error: None,
            poll_options: vec![String::new(), String::new()],
            poll_anonymous: true,
            poll_multiple: false,
            poll_quiz_mode: false,
            poll_correct_idx: 0,
            flag_a: true,
            flag_b: false,
            selected_members: Vec::new(),
            contacts_list: Vec::new(),
            call_logs: Vec::new(),
            calls_missed_only: false,
            proxies,
            new_proxy_kind: "MTProto".to_string(),
            target_lang: "English".to_string(),
            translated_text: None,
            crop_aspect: "Original".to_string(),
            rotation_deg: 0,
            brush_color: 0x40A7E3,
            media_spoiler: false,
            send_uncompressed: false,
            selected_report_reason: None,
            selected_privacy_rule: PrivacyRuleOption::Contacts,
            selected_wallpaper_id: "default_night".to_string(),
            iv_zoom_pct: 100,
            iv_serif: true,
            iv_sepia: false,
            stories: Vec::new(),
            story_slide_idx: 0,
            export_status_path: None,
            forward_targets: Vec::new(),
            pending_outcome: None,
        }
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) {
        self.palette = palette;
    }

    pub fn set_language(&mut self, language_id: impl Into<String>) {
        self.language_id = language_id.into();
    }

    pub fn sync_contacts(&mut self, contacts: Vec<(String, String, String, u32, bool)>) {
        self.contacts_list = contacts;
    }

    pub fn sync_calls(&mut self, calls: Vec<CallLogEntry>) {
        self.call_logs = calls;
    }

    pub fn sync_stories(&mut self, stories: Vec<StorySlideEntry>) {
        self.stories = stories;
        self.story_slide_idx = 0;
    }

    pub fn is_open(&self) -> bool {
        self.kind.is_some()
    }

    pub fn close(&mut self) {
        self.kind = None;
        self.validation_error = None;
        self.export_status_path = None;
    }

    pub fn open(&mut self, kind: ModalDialogKind, forward_targets: Vec<(u64, String, String, u32)>) {
        self.forward_targets = forward_targets;
        self.active_field = ActiveTextField::Primary;
        self.validation_error = None;
        self.export_status_path = None;
        self.primary_input.clear();
        self.secondary_input.clear();
        self.tertiary_input.clear();
        match &kind {
            ModalDialogKind::NewGroup | ModalDialogKind::NewChannel => self.flag_a = true,
            ModalDialogKind::EditPeer { peer_title, peer_username, peer_bio, .. } => {
                self.primary_input = peer_title.clone();
                self.secondary_input = peer_username.clone();
                self.tertiary_input = peer_bio.clone();
            }
            ModalDialogKind::MediaEditor { initial_caption, .. } => {
                self.primary_input = initial_caption.clone();
                self.rotation_deg = 0;
                self.media_spoiler = false;
            }
            ModalDialogKind::ConnectionProxy => self.secondary_input = "443".to_string(),
            _ => {}
        }
        self.kind = Some(kind);
    }

    pub fn open_for_title(&mut self, title: &str, forward_targets: Vec<(u64, String, String, u32)>) {
        let low = title.to_ascii_lowercase();
        let kind = if low.contains("poll") {
            ModalDialogKind::CreatePoll
        } else if low.contains("forward") {
            ModalDialogKind::ForwardMessages { message_ids: Vec::new(), preview_snippet: String::new() }
        } else if low.contains("new group") {
            ModalDialogKind::NewGroup
        } else if low.contains("new channel") {
            ModalDialogKind::NewChannel
        } else if low.contains("edit") {
            ModalDialogKind::EditPeer {
                peer_title: title.to_string(),
                peer_username: String::new(),
                peer_bio: String::new(),
                is_channel: low.contains("channel"),
            }
        } else if low.contains("add contact") {
            ModalDialogKind::AddContact
        } else if low.contains("contact") {
            ModalDialogKind::Contacts
        } else if low.contains("call") {
            ModalDialogKind::CallsHistory
        } else if low.contains("proxy") || low.contains("connection") {
            ModalDialogKind::ConnectionProxy
        } else if low.contains("passcode") || low.contains("password") {
            ModalDialogKind::PasscodeSetup
        } else if low.contains("language") {
            ModalDialogKind::LanguagePicker
        } else if low.contains("background") || low.contains("wallpaper") {
            ModalDialogKind::BackgroundPicker { peer_name: None }
        } else if low.contains("privacy") {
            ModalDialogKind::EditPrivacy { category: title.to_string() }
        } else if low.contains("shortcut") || low.contains("key") {
            ModalDialogKind::ShortcutsLegend
        } else if low.contains("about") {
            ModalDialogKind::AboutTelegram
        } else {
            ModalDialogKind::InfoDocument { title: title.to_string(), body: title.to_string() }
        };
        self.open(kind, forward_targets);
    }

    pub fn handle_key_input(&mut self, key: &str, key_char: Option<&str>) -> bool {
        if self.kind.is_none() {
            return false;
        }
        if key == "escape" {
            self.close();
            return true;
        }
        let target: &mut String = match self.active_field {
            ActiveTextField::Primary => &mut self.primary_input,
            ActiveTextField::Secondary => &mut self.secondary_input,
            ActiveTextField::Tertiary => &mut self.tertiary_input,
            ActiveTextField::OptionIndex(idx) => self.poll_options.get_mut(idx).unwrap_or(&mut self.primary_input),
        };
        if key == "backspace" {
            target.pop();
            self.validation_error = None;
            return true;
        }
        if key == "space" {
            target.push(' ');
            self.validation_error = None;
            return true;
        }
        if let Some(ch) = key_char.filter(|s| !s.chars().any(char::is_control)) {
            target.push_str(ch);
            self.validation_error = None;
            return true;
        }
        false
    }
}

impl Render for ActiveModalDialog {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_open() {
            return div().absolute().w(px(0.0)).h(px(0.0)).overflow_hidden().into_any_element();
        }
        render_dialog_overlay(self, |this| this, |this| this.close(), cx)
    }
}
