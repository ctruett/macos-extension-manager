//! Loading spinner component

/// A loading spinner widget
pub struct LoadingSpinner {
    frame: usize,
}

impl LoadingSpinner {
    /// Create a new loading spinner
    pub fn new() -> Self {
        Self { frame: 0 }
    }

    /// Advance the spinner
    pub fn tick(&mut self) {
        self.frame = (self.frame + 1) % 4;
    }

    /// Get the current spinner character
    pub fn current(&self) -> &'static str {
        match self.frame {
            0 => "⠋",
            1 => "⠙",
            2 => "⠹",
            _ => "⠸",
        }
    }
}

impl Default for LoadingSpinner {
    fn default() -> Self {
        Self::new()
    }
}