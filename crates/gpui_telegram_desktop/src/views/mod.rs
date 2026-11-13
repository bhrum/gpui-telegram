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
pub mod main_menu_drawer;
pub mod media_viewer;
pub mod modal_boxes_suite;
pub mod nav_rail;
pub mod settings;
pub mod settings_workspace;
pub mod shell;
pub mod sidebar;
pub mod stage;
pub mod tdesktop_theme;

pub use app_shell::ModalContainer;
pub use call_overlay::{CallOverlay, TalkModeState, TranscriptEntry};
pub use canonical_composer::CanonicalComposer;
pub use chat_sidebar::ChatSidebar;
pub use computer_workbench::{ComputerWorkbench, HostDeviceSpec};
pub use context_inspector::ContextInspector;
pub use conversation_stage::ConversationStage;
pub use global_tasks_workspace::GlobalTasksWorkspace;
pub use main_menu_drawer::{MainMenuCommand, MainMenuDrawer};
pub use media_viewer::{MediaViewer, MediaViewerItem};
pub use modal_boxes_suite::{ActiveModalDialog, ModalActionOutcome, ModalDialogKind};
pub use nav_rail::NavRail;
pub use settings_workspace::{SettingsMutation, SettingsPageId, SettingsWorkspaceView};
pub use shell::AppShell;
pub use stage::cards::*;
pub use tdesktop_theme::{TdesktopThemeMode, TdesktopThemePalette, tr_str};
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
