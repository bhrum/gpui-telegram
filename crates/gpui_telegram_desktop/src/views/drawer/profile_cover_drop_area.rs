//! Profile Cover Drag-and-Drop Area
//! Upstream: Telegram/SourceFiles/profile/profile_cover_drop_area.h, Telegram/SourceFiles/profile/profile_cover_drop_area.cpp

#[derive(Debug, Clone)]
pub struct ProfileCoverDropArea {
    pub is_drag_active: bool,
    pub is_visible: bool,
    pub animation_opacity: f32,
    pub height: f32,
    pub dash_length: f32,
    pub dash_gap: f32,
    pub dropped_files: Vec<String>,
}

impl Default for ProfileCoverDropArea {
    fn default() -> Self {
        Self {
            is_drag_active: false,
            is_visible: false,
            animation_opacity: 0.0,
            height: 120.0,
            dash_length: 6.0,
            dash_gap: 4.0,
            dropped_files: Vec::new(),
        }
    }
}

impl ProfileCoverDropArea {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show_animated(&mut self) {
        self.is_visible = true;
        self.animation_opacity = 1.0;
    }

    pub fn hide_animated(&mut self) {
        self.is_visible = false;
        self.is_drag_active = false;
        self.animation_opacity = 0.0;
    }

    pub fn drag_enter(&mut self) {
        self.is_drag_active = true;
        self.show_animated();
    }

    pub fn drag_leave(&mut self) {
        self.is_drag_active = false;
    }

    pub fn drop_file(&mut self, file_path: impl Into<String>) {
        self.dropped_files.push(file_path.into());
        self.hide_animated();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_cover_drop_area_lifecycle() {
        let mut drop_area = ProfileCoverDropArea::new();
        assert!(!drop_area.is_visible);
        assert!(!drop_area.is_drag_active);

        drop_area.drag_enter();
        assert!(drop_area.is_visible);
        assert!(drop_area.is_drag_active);
        assert_eq!(drop_area.animation_opacity, 1.0);

        drop_area.drop_file("/path/to/banner.png");
        assert_eq!(drop_area.dropped_files.len(), 1);
        assert!(!drop_area.is_visible);
        assert!(!drop_area.is_drag_active);
    }
}
