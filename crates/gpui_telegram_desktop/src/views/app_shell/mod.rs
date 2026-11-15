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
pub mod shell_actions;
pub mod shell_calls_rtc;
pub mod shell_composer_actions;
pub mod shell_computers_sync;
pub mod shell_dialogs_fetch;
pub mod shell_hermes_studio_sync;
pub mod shell_input_ime;
pub mod shell_miniapp_catalog;
pub mod shell_miniapps_bots;
pub mod shell_modals;
pub mod shell_observers;
pub mod shell_overlays_render;
pub mod shell_panels;
pub mod shell_panels_sync;
pub mod shell_pts_updates;
pub mod shell_reactions_sync;
pub mod shell_settings_sync;
pub mod shell_shortcuts;
pub mod shell_stage_actions;
pub mod shell_state;
pub mod shell_theme;
pub mod shell_truth;
pub mod shell_ws_sync;
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
pub use shell_actions::{
    build_account_record, build_chat_profile_facts, calls_to_modal_logs,
    chat_dialogs_to_forward_targets, contacts_to_modal_rows, localize_call_state,
    map_channel_stats_to_chart, map_chat_members_to_inspector, map_stage_media_to_gallery,
    peer_avatar_color, resolve_active_target_chat_id, shell_mcp_to_entries,
    shell_tasks_to_acp_entries,
};
pub use shell_panels::{
    localized_panel_label, parse_acp_tasks, parse_mcp_servers, parse_mcp_tools,
    render_empty_panel_notice, render_panel_header, ShellAcpTaskRow, ShellMcpServerRow,
    ShellMcpToolRow, ShellPanelState,
};
pub use shell_state::{
    reconcile_stage_message_list, should_switch_to_chats_on_sidebar_selection, ShellComputersState,
    ShellMiniAppState, ShellOverlayState, ShellRtcTalkState, ShellSessionSyncState,
    ShellThemeVisualState,
};
pub use shell_theme::{
    compute_shell_palette, propagate_theme_and_locale, resolve_mode_from_preferences,
    tdesktop_mode_to_app_theme, toggle_desktop_theme_mode, ShellThemeTargets,
};
pub use shell_truth::{
    apply_authoritative_chat_list, apply_fenced_chat_list_response,
    apply_fenced_history_response, apply_fenced_history_response_preserving,
    format_authoritative_timestamp, historical_message_to_model,
    peer_subtitle_for_dialog, reset_account_conversation_views, select_authoritative_dialog,
    try_consume_composer_for_send, validate_selected_send_target, SessionRequestFence,
    VerifiedSendTarget,
};
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
