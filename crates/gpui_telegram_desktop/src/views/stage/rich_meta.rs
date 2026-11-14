//! Sidecar Rich Metadata for ConversationStage Messages.
//! Preserves 100% compatibility with existing `MessageItemModel` struct literals & `MessageContent` matches.

use gpui::{ImageSource, Resource};
use std::collections::HashMap;
use std::path::PathBuf;

use super::entities::RichEntity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForwardHeader {
    pub sender_name: String,
    pub peer_id: Option<u64>,
    pub origin_date: Option<String>,
    pub via_bot: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebPageMeta {
    pub url: String,
    pub site_name: Option<String>,
    pub title: String,
    pub description: String,
    pub thumb_url: Option<String>,
    pub has_instant_view: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DocumentMeta {
    pub file_name: String,
    pub mime_type: String,
    pub file_size_bytes: u64,
    pub download_url: String,
    pub local_path: Option<String>,
    pub is_downloading: bool,
    pub download_progress: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContactMeta {
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub user_id: Option<u64>,
}

#[derive(Debug, Clone, Default)]
pub struct MessageRichMeta {
    pub entities: Vec<RichEntity>,
    pub forward_from: Option<ForwardHeader>,
    pub views_count: Option<u32>,
    pub edit_date: Option<String>,
    pub timestamp_unix: i64,
    pub date_group_label: Option<String>,
    pub local_media_path: Option<String>,
    pub avatar_url: Option<String>,
    pub webpage: Option<WebPageMeta>,
    pub document: Option<DocumentMeta>,
    pub contact: Option<ContactMeta>,
    pub quiz_explanation: Option<String>,
    pub translated_text: Option<String>,
    pub translated_lang: Option<String>,
    pub show_translation: bool,
    pub is_silent: bool,
    pub scheduled_at: Option<i64>,
    pub is_spoiler_media: bool,
}

pub type MessageMetaMap = HashMap<u64, MessageRichMeta>;

/// Build a safe GPUI `ImageSource` from either an HTTP(S)/file URL or a local file path.
pub fn resolve_image_source(url: &str, local_path: Option<&str>) -> Option<ImageSource> {
    if let Some(lp) = local_path {
        let trimmed = lp.trim();
        if !trimmed.is_empty() && std::path::Path::new(trimmed).exists() {
            return Some(ImageSource::Resource(Resource::Path(std::sync::Arc::from(
                PathBuf::from(trimmed),
            ))));
        }
    }
    let trimmed = url.trim();
    if trimmed.is_empty() {
        return None;
    }
    if trimmed.starts_with("http://") || trimmed.starts_with("https://") || trimmed.starts_with("file://") {
        return Some(ImageSource::Resource(Resource::Uri(trimmed.to_string().into())));
    }
    if std::path::Path::new(trimmed).exists() {
        return Some(ImageSource::Resource(Resource::Path(std::sync::Arc::from(
            PathBuf::from(trimmed),
        ))));
    }
    None
}

pub fn format_view_count(count: u32) -> String {
    if count >= 1_000_000 {
        format!("{:.1}M", count as f32 / 1_000_000.0)
    } else if count >= 1_000 {
        format!("{:.1}K", count as f32 / 1_000.0)
    } else {
        count.to_string()
    }
}

pub fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;
    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.0} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
