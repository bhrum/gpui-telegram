//! Module definitions for `crates/gpui_telegram_desktop/src/views/app_shell`

pub mod gift_resale_filter;
pub mod main_window;
pub mod modal_container;
pub mod menu_action_with_thumbnail;
pub mod menu_antispam_validator;
pub mod menu_check_item;
pub mod menu_checked_action;
pub mod menu_dock;
pub mod menu_emoji_status;
pub mod menu_item_download_files;
pub mod menu_item_rate_transcribe;
pub mod menu_item_rate_transcribe_session;
pub mod menu_mark_as_read;
pub mod menu_mute;
pub mod menu_send;
pub mod menu_send_details;
pub mod menu_sponsored;
pub mod menu_timecode_action;
pub mod menu_ttl;
pub mod menu_ttl_validator;
pub mod notifications_manager;
pub mod notifications_manager_default;
pub mod notifications_utilities;
pub mod section_memento;
pub mod section_widget;
pub mod session;
pub mod support_autocomplete;
pub mod support_common;
pub mod support_helper;
pub mod support_preload;
pub mod support_templates;
pub mod themes;
pub mod window_adaptive;
pub mod window_chat_preview;
pub mod window_chat_switch_process;
pub mod window_connecting_widget;
pub mod window_controller;
pub mod window_filters_favorite;
pub mod window_filters_menu;
pub mod window_history_hider;
pub mod window_lock_widgets;
pub mod window_main_menu;
pub mod window_main_menu_helpers;
pub mod window_media_preview;
pub mod window_no_paid_token;
pub mod window_peer_menu;
pub mod window_restore_shell;
pub mod window_saved_windows;
pub mod window_section_common;
pub mod window_separate_id;
pub mod window_session_controller;
pub mod window_session_controller_contact;
pub mod window_session_controller_link_info;
pub mod window_setup_email;
pub mod window_slide_animation;
pub mod window_text_drop;
pub mod window_top_bar_wrap;
pub mod window_unlock_passcode_box;

pub use gift_resale_filter::GiftResaleFilter;
pub use main_window::MainWindow;
pub use menu_action_with_thumbnail::MenuActionWithThumbnail;
pub use menu_antispam_validator::MenuAntispamValidator;
pub use menu_check_item::MenuCheckItem;
pub use menu_checked_action::MenuCheckedAction;
pub use menu_dock::MenuDock;
pub use menu_emoji_status::MenuEmojiStatus;
pub use menu_item_download_files::MenuItemDownloadFiles;
pub use menu_item_rate_transcribe::MenuItemRateTranscribe;
pub use menu_item_rate_transcribe_session::MenuItemRateTranscribeSession;
pub use menu_mark_as_read::MenuMarkAsRead;
pub use menu_mute::MenuMute;
pub use menu_send::MenuSend;
pub use menu_send_details::{
    CaptionState, CoverState, MenuSendDetails, PhotoQualityState, SendDetails, SendMenuType,
    SpoilerState,
};
pub use menu_sponsored::MenuSponsored;
pub use menu_timecode_action::MenuTimecodeAction;
pub use menu_ttl::MenuTtl;
pub use menu_ttl_validator::MenuTtlValidator;
pub use modal_container::ModalContainer;
pub use notifications_manager::NotificationsManager;
pub use notifications_manager_default::NotificationsManagerDefault;
pub use notifications_utilities::NotificationsUtilities;
pub use section_memento::SectionMemento;
pub use section_widget::SectionWidget;
pub use support_autocomplete::SupportAutocomplete;
pub use support_common::SupportCommon;
pub use support_helper::SupportHelper;
pub use support_preload::{
    FastReplySnippet, SupportPreload, SupportPreloadRequest, PRELOAD_MESSAGES_COUNT,
};
pub use support_templates::SupportTemplates;
pub use window_adaptive::WindowAdaptive;
pub use window_chat_preview::WindowChatPreview;
pub use window_chat_switch_process::WindowChatSwitchProcess;
pub use window_connecting_widget::WindowConnectingWidget;
pub use window_controller::WindowController;
pub use window_filters_favorite::WindowFiltersFavorite;
pub use window_filters_menu::WindowFiltersMenu;
pub use window_history_hider::WindowHistoryHider;
pub use window_lock_widgets::WindowLockWidgets;
pub use window_main_menu::WindowMainMenu;
pub use window_main_menu_helpers::WindowMainMenuHelpers;
pub use window_media_preview::WindowMediaPreview;
pub use window_no_paid_token::WindowNoPaidToken;
pub use window_peer_menu::WindowPeerMenu;
pub use window_restore_shell::WindowRestoreShell;
pub use window_saved_windows::WindowSavedWindows;
pub use window_section_common::{
    EasingCurve, SectionActionResult, SectionSlideParams, SectionTransition, WindowSectionCommon,
};
pub use window_separate_id::WindowSeparateId;
pub use window_session_controller::WindowSessionController;
pub use window_session_controller_contact::WindowSessionControllerContact;
pub use window_session_controller_link_info::{
    LinkInfoType, PeerByLinkInfo, RepliesByLinkInfo, ResolveType, WindowSessionControllerLinkInfo,
};
pub use window_setup_email::WindowSetupEmail;
pub use window_slide_animation::WindowSlideAnimation;
pub use window_text_drop::WindowTextDrop;
pub use window_top_bar_wrap::WindowTopBarWrap;
pub use window_unlock_passcode_box::WindowUnlockPasscodeBox;
