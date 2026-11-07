//! Module definitions for `crates/gpui_telegram_desktop/src/views`

pub mod app_shell;
pub mod auth;
pub mod boxes;
pub mod call_overlay;
pub mod canonical_composer;
pub mod chat_sidebar;
pub mod composer;
pub mod computer_workbench;
pub mod context_inspector;
pub mod conversation_stage;
pub mod drawer;
pub mod global_tasks_workspace;
pub mod media_viewer;
pub mod nav_rail;
pub mod settings;
pub mod shell;
pub mod sidebar;
pub mod stage;

pub use app_shell::ModalContainer;
pub use call_overlay::{CallOverlay, TalkModeState, TranscriptEntry};
pub use canonical_composer::CanonicalComposer;
pub use chat_sidebar::ChatSidebar;
pub use computer_workbench::{ComputerWorkbench, HostDeviceSpec};
pub use context_inspector::ContextInspector;
pub use conversation_stage::ConversationStage;
pub use global_tasks_workspace::GlobalTasksWorkspace;
pub use media_viewer::{MediaViewer, MediaViewerItem};
pub use nav_rail::NavRail;
pub use shell::AppShell;
pub use stage::cards::*;
pub use composer::{
    AttachmentsDrawer, FormattingBar, ImeSafetyMachine, ImeState, KeyAction,
    TextStyleKind, VoiceGestureMachine, VoiceGestureState,
};
pub use drawer::inspector_tabs::{
    InspectorTab, MemberItem, MembersTabModel, ProfileTabModel, SettingsTabModel,
    SharedFileItem, SharedLinkItem, SharedMediaSubTab, SharedMediaTabModel,
};

pub mod iv;
pub use iv::*;
