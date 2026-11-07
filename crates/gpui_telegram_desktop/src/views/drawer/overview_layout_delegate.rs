//! Overview Layout Delegate Abstraction
//! Upstream: Telegram/SourceFiles/overview/overview_layout_delegate.h

use gpui::*;

#[derive(Debug, Clone, Default)]
pub struct OverviewLayoutDelegate {
    pub repaint_needed: bool,
    pub visible_items: Vec<u64>,
    pub selected_items: Vec<u64>,
}

impl OverviewLayoutDelegate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn request_repaint(&mut self) {
        self.repaint_needed = true;
    }

    pub fn reset_repaint_flag(&mut self) {
        self.repaint_needed = false;
    }

    pub fn set_item_visible(&mut self, item_id: u64, visible: bool) {
        if visible {
            if !self.visible_items.contains(&item_id) {
                self.visible_items.push(item_id);
            }
        } else {
            self.visible_items.retain(|&id| id != item_id);
        }
    }

    pub fn is_item_visible(&self, item_id: u64) -> bool {
        self.visible_items.contains(&item_id)
    }

    pub fn toggle_item_selection(&mut self, item_id: u64) -> bool {
        if let Some(pos) = self.selected_items.iter().position(|&id| id == item_id) {
            self.selected_items.remove(pos);
            false
        } else {
            self.selected_items.push(item_id);
            true
        }
    }

    pub fn is_item_selected(&self, item_id: u64) -> bool {
        self.selected_items.contains(&item_id)
    }

    pub fn clear_selection(&mut self) {
        self.selected_items.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overview_layout_delegate() {
        let mut delegate = OverviewLayoutDelegate::new();
        delegate.request_repaint();
        assert!(delegate.repaint_needed);
        delegate.reset_repaint_flag();
        assert!(!delegate.repaint_needed);

        delegate.set_item_visible(42, true);
        assert!(delegate.is_item_visible(42));
        delegate.set_item_visible(42, false);
        assert!(!delegate.is_item_visible(42));

        assert!(delegate.toggle_item_selection(101));
        assert!(delegate.is_item_selected(101));
        assert!(!delegate.toggle_item_selection(101));
        assert!(!delegate.is_item_selected(101));
    }
}
