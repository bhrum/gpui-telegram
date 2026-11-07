//! Authentic GPUI History View Save Document Action Component.
//!
//! Upstream:
//! - `Telegram/SourceFiles/history/view/media/history_view_save_document_action.h:L1-L42`
//! - `Telegram/SourceFiles/history/view/media/history_view_save_document_action.cpp:L1-L133`
//!
//! Context Menu Actions (`history_view_save_document_action.cpp:L54-L62`):
//! - `Save Video` for video documents
//! - `Save Audio` for voice messages
//! - `Save Music` / `Save File` for generic audio & binary files

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveDocumentActionType {
    SaveVideo,
    SaveAudio,
    SaveMusic,
    SaveFile,
}

impl SaveDocumentActionType {
    pub fn label(&self) -> &'static str {
        match self {
            Self::SaveVideo => "Save Video As...",
            Self::SaveAudio => "Save Audio As...",
            Self::SaveMusic => "Save Music As...",
            Self::SaveFile => "Save File As...",
        }
    }
}

/// Helper model for context menu save document operations.
/// Upstream: `AddSaveDocumentAction` (`history_view_save_document_action.h:L29-L40`)
#[derive(Debug, Clone)]
pub struct HistoryViewSaveDocumentActionViewMedia {
    pub document_id: u64,
    pub action_type: SaveDocumentActionType,
    pub default_file_name: String,
    pub can_save_to_profile: bool,
}

impl HistoryViewSaveDocumentActionViewMedia {
    pub fn new(document_id: u64, file_name: impl Into<String>, action_type: SaveDocumentActionType) -> Self {
        Self {
            document_id,
            action_type,
            default_file_name: file_name.into(),
            can_save_to_profile: false,
        }
    }

    pub fn menu_label(&self) -> &'static str {
        self.action_type.label()
    }
}
