//! Authentic GPUI Horizontal Fit Layout Container.
//! Reference: Telegram/SourceFiles/ui/widgets/horizontal_fit_container.cpp:L14-L56
//! Reference: Telegram/SourceFiles/ui/widgets/horizontal_fit_container.h:L14-L36

#![allow(dead_code, unused_imports, unused_variables)]

#[derive(Debug, Clone, PartialEq)]
pub struct ChildButtonSlot {
    pub id: i32,
    pub label: String,
    pub x: f32,
    pub width: f32,
    pub is_enabled: bool,
}

/// Target symbol for horizontal container distributing available width evenly across child buttons.
#[derive(Debug, Clone)]
pub struct HorizontalFitContainerWidgets {
    pub spacing: f32,
    pub next_id: i32,
    pub buttons: Vec<ChildButtonSlot>,
    pub container_width: f32,
    pub container_height: f32,
}

impl Default for HorizontalFitContainerWidgets {
    fn default() -> Self {
        Self::new(8.0)
    }
}

impl HorizontalFitContainerWidgets {
    pub fn new(spacing: f32) -> Self {
        Self {
            spacing,
            next_id: 1,
            buttons: Vec::new(),
            container_width: 300.0,
            container_height: 40.0,
        }
    }

    /// Adds a button to the container and triggers layout update.
    /// Reference: `horizontal_fit_container.cpp:L24-L31`
    pub fn add(&mut self, label: impl Into<String>) -> i32 {
        let id = self.next_id;
        self.next_id += 1;
        self.buttons.push(ChildButtonSlot {
            id,
            label: label.into(),
            x: 0.0,
            width: 0.0,
            is_enabled: true,
        });
        self.update_layout();
        id
    }

    /// Removes button by ID and recalibrates layout.
    /// Reference: `horizontal_fit_container.cpp:L33-L38`
    pub fn remove(&mut self, id: i32) -> bool {
        let initial_len = self.buttons.len();
        self.buttons.retain(|b| b.id != id);
        let removed = self.buttons.len() < initial_len;
        if removed {
            self.update_layout();
        }
        removed
    }

    pub fn set_container_size(&mut self, width: f32, height: f32) {
        self.container_width = width;
        self.container_height = height;
        self.update_layout();
    }

    /// Computes equal widths and X offsets for all child slots:
    /// `buttonWidth = (size.width() - totalSpacing) / count`.
    /// Reference: `horizontal_fit_container.cpp:L45-L55`
    pub fn update_layout(&mut self) {
        if self.buttons.is_empty() {
            return;
        }
        let count = self.buttons.len() as f32;
        let total_spacing = self.spacing * (count - 1.0);
        let button_width = ((self.container_width - total_spacing) / count).max(0.0);

        let mut current_x = 0.0f32;
        for btn in &mut self.buttons {
            btn.x = current_x;
            btn.width = button_width;
            current_x += button_width + self.spacing;
        }
    }
}
