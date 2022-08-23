use tui::style::Color;

use crate::theme::Theme;

pub struct CellOption {
    pub valid: bool,
    pub fg: Color,
    pub bg: Color,
    default_bg: Color,
    default_fg: Color,
}

impl Default for CellOption {
    fn default() -> Self {
        Self {
            valid: false,
            fg: Theme::default().white,
            bg: Theme::default().dark_grey,
            default_fg: Theme::default().white,
            default_bg: Theme::default().dark_grey,
        }
    }
}

impl CellOption {
    pub fn reset_bg(&mut self) {
        self.bg = self.default_bg;
    }
    pub fn reset_fg(&mut self) {
        self.fg = self.default_fg;
    }

    pub fn reset_colors(&mut self) {
        self.reset_bg();
        self.reset_fg();
    }
}

#[derive(Default)]
pub struct CellOptions {
    pub values: [CellOption; 9],
}
