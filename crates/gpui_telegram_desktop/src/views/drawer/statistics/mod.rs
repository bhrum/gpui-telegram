//! Module definitions for `crates/gpui_telegram_desktop/src/views/drawer/statistics`

pub mod info_statistics_common;
pub mod info_statistics_export;
pub mod info_statistics_inner_widget;
pub mod info_statistics_list_controllers;
pub mod info_statistics_recent_message;
pub mod info_statistics_tag;
pub mod info_statistics_widget;

pub use info_statistics_common::InfoStatisticsCommonStatistics;
pub use info_statistics_export::InfoStatisticsExportStatistics;
pub use info_statistics_inner_widget::InfoStatisticsInnerWidgetStatistics;
pub use info_statistics_list_controllers::InfoStatisticsListControllersStatistics;
pub use info_statistics_recent_message::InfoStatisticsRecentMessageStatistics;
pub use info_statistics_tag::InfoStatisticsTagStatistics;
pub use info_statistics_widget::InfoStatisticsWidgetStatistics;
