//! Info Modal Layer Stack Widget
//! Upstream: Telegram/SourceFiles/info/info_layer_widget.h, Telegram/SourceFiles/info/info_layer_widget.cpp

pub const K_MAX_LAYER_ATTEMPTS: usize = 16;
pub const K_MAX_LAYER_DEPTH: usize = 8;

#[derive(Debug, Clone)]
pub struct InfoLayerWidget {
    pub is_layer_active: bool,
    pub layer_depth: usize,
    pub max_layer_depth: usize,
    pub has_float_player: bool,
    pub opacity: f32,
    pub background_dim_opacity: f32,
}

impl Default for InfoLayerWidget {
    fn default() -> Self {
        Self {
            is_layer_active: false,
            layer_depth: 0,
            max_layer_depth: K_MAX_LAYER_DEPTH,
            has_float_player: false,
            opacity: 1.0,
            background_dim_opacity: 0.45,
        }
    }
}

impl InfoLayerWidget {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open_layer(&mut self) -> bool {
        if self.layer_depth < self.max_layer_depth {
            self.layer_depth += 1;
            self.is_layer_active = true;
            true
        } else {
            false
        }
    }

    pub fn close_layer(&mut self) -> bool {
        if self.layer_depth > 0 {
            self.layer_depth -= 1;
            if self.layer_depth == 0 {
                self.is_layer_active = false;
            }
            true
        } else {
            false
        }
    }

    pub fn set_float_player(&mut self, active: bool) {
        self.has_float_player = active;
    }

    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.clamp(0.0, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_widget_stack_limits() {
        let mut layer = InfoLayerWidget::new();
        assert!(!layer.is_layer_active);

        assert!(layer.open_layer());
        assert_eq!(layer.layer_depth, 1);
        assert!(layer.is_layer_active);

        for _ in 1..K_MAX_LAYER_DEPTH {
            assert!(layer.open_layer());
        }
        assert_eq!(layer.layer_depth, K_MAX_LAYER_DEPTH);
        // Exceeding max layer depth fails
        assert!(!layer.open_layer());

        assert!(layer.close_layer());
        assert_eq!(layer.layer_depth, K_MAX_LAYER_DEPTH - 1);
    }
}
