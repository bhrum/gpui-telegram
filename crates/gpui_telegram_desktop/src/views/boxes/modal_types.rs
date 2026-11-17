use crate::views::boxes::edit_privacy_box::PrivacyRuleOption;
use crate::views::boxes::report_messages_box::ReportReason;
use crate::views::stage::cards::{PollOption, PollType};
use gpui_telegram_client::ProxyEndpointRecord;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallLogEntry {
    pub peer_name: String,
    pub description: String,
    pub timestamp: String,
    pub is_missed: bool,
    pub is_video: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorySlideEntry {
    pub caption: String,
    pub bg_color: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModalDialogKind {
    CreatePoll,
    ForwardMessages {
        message_ids: Vec<u64>,
        preview_snippet: String,
    },
    NewGroup,
    NewChannel,
    Contacts,
    AddContact,
    CallsHistory,
    ConnectionProxy,
    TranslateMessage {
        original_text: String,
    },
    MediaEditor {
        initial_caption: String,
        is_video: bool,
    },
    DeleteMessages {
        message_ids: Vec<u64>,
        peer_name: String,
    },
    PinMessage {
        message_id: u64,
        peer_name: String,
        is_group: bool,
    },
    ReportMessage {
        message_ids: Vec<u64>,
        peer_name: String,
    },
    BackgroundPicker {
        peer_name: Option<String>,
    },
    PasscodeSetup,
    LanguagePicker,
    EditPeer {
        peer_title: String,
        peer_username: String,
        peer_bio: String,
        is_channel: bool,
    },
    EditPrivacy {
        category: String,
    },
    StarGifts {
        peer_name: String,
    },
    SendCredits {
        invoice_title: String,
        stars_amount: u64,
        balance_stars: u64,
    },
    ExportData {
        chat_title: String,
    },
    InstantView {
        title: String,
        url: String,
        body_paragraphs: Vec<String>,
    },
    StoriesViewer {
        peer_name: String,
    },
    ShortcutsLegend,
    AboutTelegram,
    InfoDocument {
        title: String,
        body: String,
    },
    ClearChatHistory {
        peer_name: String,
    },
    LeaveOrDeleteChat {
        peer_name: String,
        is_group: bool,
    },
}

#[derive(Debug, Clone)]
pub enum ModalActionOutcome {
    CreatePollSubmitted {
        question: String,
        poll_type: PollType,
        options: Vec<PollOption>,
    },
    ForwardToChat {
        target_chat_id: u64,
        message_ids: Vec<u64>,
        show_sender: bool,
        extra_caption: String,
    },
    CreateGroupSubmitted {
        title: String,
        members_count: usize,
        enable_forum_topics: bool,
    },
    CreateChannelSubmitted {
        title: String,
        username_slug: String,
        is_public: bool,
    },
    OpenContactChat {
        name: String,
        initials: String,
        color: u32,
    },
    AddContactSubmitted {
        first_name: String,
        last_name: String,
        phone: String,
    },
    StartCallWithPeer {
        peer_name: String,
        is_video: bool,
    },
    SelectProxy {
        proxy_id: u32,
    },
    AddProxy(ProxyEndpointRecord),
    CopyToClipboard(String),
    SendEditedMedia {
        is_video: bool,
        aspect_label: String,
        rotation_deg: u32,
        has_spoiler: bool,
        uncompressed_file: bool,
        caption: String,
    },
    ConfirmDeleteMessages {
        message_ids: Vec<u64>,
        revoke_for_everyone: bool,
    },
    ConfirmPinMessage {
        message_id: u64,
        notify_members: bool,
        pin_for_both: bool,
    },
    SubmitReport {
        message_ids: Vec<u64>,
        reason: ReportReason,
        comment: String,
    },
    SelectBackground {
        preset_id: String,
    },
    SetLocalPasscode(Option<String>),
    SelectLanguage {
        language_code: String,
    },
    SavePeerEdit {
        title: String,
        username: String,
        bio: String,
    },
    SavePrivacyRule {
        category: String,
        rule: PrivacyRuleOption,
    },
    SendStarGift {
        emoji: String,
        title: String,
        stars: u64,
    },
    ConfirmSendCredits {
        invoice_title: String,
        stars_amount: u64,
    },
    RunChatExport {
        format_json: bool,
    },
    SendStoryReply {
        peer_name: String,
        text: String,
    },
    ConfirmClearChatHistory {
        also_for_peer: bool,
    },
    ConfirmLeaveOrDeleteChat,
    CheckForUpdates,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveTextField {
    Primary,
    Secondary,
    Tertiary,
    OptionIndex(usize),
}
