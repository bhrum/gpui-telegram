//! Module definitions for `crates/gpui_telegram_desktop/src/views/stage`

pub mod admin_log;
pub mod history;
pub mod history_drag_area;
pub mod history_inner_widget;
pub mod history_inner_widget_accessibility;
pub mod history_item;
pub mod history_item_components;
pub mod history_item_edition;
pub mod history_item_helpers;
pub mod history_item_reply_markup;
pub mod history_item_text;
pub mod history_location_manager;
pub mod history_message_selection;
pub mod history_streamed_drafts;
pub mod history_translation;
pub mod history_unread_things;
pub mod history_view_highlight_manager;
pub mod history_view_pull_to_next_channel;
pub mod history_view_swipe_back_session;
pub mod history_view_top_toast;
pub mod history_widget;
pub mod layout;
pub mod poll_link_box;
pub mod poll_link_thumbnail;
pub mod poll_media_upload;
pub mod view;
pub mod media;
pub use media::*;
pub mod cards;
pub use cards::*;

pub mod bottom_info;
pub mod history_element;
pub mod history_message;
pub mod history_service;
pub mod reply_view;
pub mod rich_quote;

pub mod list_widget;
pub mod chat_section;
pub mod top_bar;
pub mod pinned_bar;
pub mod scheduled_section;
pub mod translate_bar;

pub use bottom_info::*;
pub use history_element::*;
pub use history_message::*;
pub use history_service::*;
pub use reply_view::*;
pub use rich_quote::*;
pub use list_widget::*;
pub use chat_section::*;
pub use top_bar::*;
pub use pinned_bar::*;
pub use scheduled_section::*;
pub use translate_bar::*;
pub mod reactions;
pub use reactions::*;
pub mod item;
pub use item::*;

