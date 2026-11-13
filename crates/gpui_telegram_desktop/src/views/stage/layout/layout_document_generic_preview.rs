//! Generic document attachment preview generator with 4-color categorization
//! Upstream: Telegram/SourceFiles/layout/layout_document_generic_preview.h, layout_document_generic_preview.cpp
//! Subsystem: layout | Feature ID: TCG-N04 | Disposition: gpui_ui_1to1

/// Color classification indices matching upstream `BuildFromColorIndex`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentColorIndex {
    Blue = 0,   // text, docs, psd
    Green = 1,  // spreadsheets (xls, csv)
    Red = 2,    // pdf, ppt, key
    Yellow = 3, // archives, audio, video
}

impl DocumentColorIndex {
    pub fn from_index(idx: usize) -> Self {
        match idx & 3 {
            0 => Self::Blue,
            1 => Self::Green,
            2 => Self::Red,
            _ => Self::Yellow,
        }
    }

    pub fn to_index(self) -> usize {
        self as usize
    }

    /// Primary background color (RGBA hex)
    pub fn bg_color(self) -> u32 {
        match self {
            Self::Blue => 0xFF5B9BE6,
            Self::Green => 0xFF53BF6B,
            Self::Red => 0xFFE25656,
            Self::Yellow => 0xFFE5A638,
        }
    }

    /// Dark icon badge color (RGBA hex)
    pub fn dark_color(self) -> u32 {
        match self {
            Self::Blue => 0xFF457AB8,
            Self::Green => 0xFF419C55,
            Self::Red => 0xFFB84141,
            Self::Yellow => 0xFFB88328,
        }
    }

    /// Hover color (RGBA hex)
    pub fn over_color(self) -> u32 {
        match self {
            Self::Blue => 0xFF6CA8EE,
            Self::Green => 0xFF62C979,
            Self::Red => 0xFFEE6868,
            Self::Yellow => 0xFFEEB44B,
        }
    }

    /// Selected color (RGBA hex)
    pub fn selected_color(self) -> u32 {
        0xFFFFFFFF
    }

    /// Icon identifier
    pub fn icon_name(self) -> &'static str {
        match self {
            Self::Blue => "mediaviewFileBlue",
            Self::Green => "mediaviewFileGreen",
            Self::Red => "mediaviewFileRed",
            Self::Yellow => "mediaviewFileYellow",
        }
    }
}

/// Generic document preview badge styling and metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutDocumentGenericPreview {
    pub index: usize,
    pub color: u32,
    pub dark: u32,
    pub over: u32,
    pub selected: u32,
    pub ext: String,
    pub icon: &'static str,
}

impl LayoutDocumentGenericPreview {
    /// Determines the 4-color category index (0..3) for a filename and optional MIME type.
    /// In accordance with C++ `ColorIndexForName`:
    pub fn color_index_for_name(name: &str, mime: Option<&str>) -> usize {
        let name_lower = name.to_lowercase();
        let mime_lower = mime.map(|m| m.to_lowercase());

        if name_lower.ends_with(".doc")
            || name_lower.ends_with(".docx")
            || name_lower.ends_with(".txt")
            || name_lower.ends_with(".psd")
            || mime_lower.as_deref().map_or(false, |m| m.starts_with("text/"))
        {
            return 0; // Blue
        } else if name_lower.ends_with(".xls")
            || name_lower.ends_with(".xlsx")
            || name_lower.ends_with(".csv")
        {
            return 1; // Green
        } else if name_lower.ends_with(".pdf")
            || name_lower.ends_with(".ppt")
            || name_lower.ends_with(".pptx")
            || name_lower.ends_with(".key")
        {
            return 2; // Red
        } else if name_lower.ends_with(".zip")
            || name_lower.ends_with(".rar")
            || name_lower.ends_with(".ai")
            || name_lower.ends_with(".mp3")
            || name_lower.ends_with(".mov")
            || name_lower.ends_with(".avi")
        {
            return 3; // Yellow
        }

        // Fallback: hash the first character of extension or name
        let last_dot = name_lower.rfind('.');
        let ch = if let Some(dot_idx) = last_dot {
            name_lower[dot_idx + 1..].chars().next().unwrap_or('0')
        } else if let Some(first_ch) = name_lower.chars().next() {
            first_ch
        } else if let Some(first_mime_ch) = mime_lower.as_deref().and_then(|m| m.chars().next()) {
            first_mime_ch
        } else {
            '0'
        };

        ((ch as usize) % 4) & 3
    }

    /// Extracts the file extension (without dot).
    pub fn extract_extension(filename: &str) -> String {
        let name_lower = filename.to_lowercase();
        if let Some(dot_idx) = name_lower.rfind('.') {
            if dot_idx + 1 < name_lower.len() {
                return name_lower[dot_idx + 1..].to_string();
            }
        }
        String::new()
    }

    /// Creates a generic preview from a filename.
    pub fn create(filename: &str) -> Self {
        Self::create_with_mime(filename, None)
    }

    /// Creates a generic preview from a filename and MIME type.
    pub fn create_with_mime(filename: &str, mime: Option<&str>) -> Self {
        let color_idx = Self::color_index_for_name(filename, mime);
        let palette = DocumentColorIndex::from_index(color_idx);
        let ext = Self::extract_extension(filename);

        Self {
            index: color_idx,
            color: palette.bg_color(),
            dark: palette.dark_color(),
            over: palette.over_color(),
            selected: palette.selected_color(),
            ext,
            icon: palette.icon_name(),
        }
    }

    /// Returns the color classification enum.
    pub fn color_palette(&self) -> DocumentColorIndex {
        DocumentColorIndex::from_index(self.index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_generic_preview_colors() {
        let doc = LayoutDocumentGenericPreview::create("document.docx");
        assert_eq!(doc.index, 0);
        assert_eq!(doc.ext, "docx");

        let sheet = LayoutDocumentGenericPreview::create("finances.xlsx");
        assert_eq!(sheet.index, 1);
        assert_eq!(sheet.ext, "xlsx");

        let pdf = LayoutDocumentGenericPreview::create("report.pdf");
        assert_eq!(pdf.index, 2);
        assert_eq!(pdf.ext, "pdf");

        let archive = LayoutDocumentGenericPreview::create("bundle.zip");
        assert_eq!(archive.index, 3);
        assert_eq!(archive.ext, "zip");

        let text_mime = LayoutDocumentGenericPreview::create_with_mime("source.rs", Some("text/rust"));
        assert_eq!(text_mime.index, 0);
    }
}
