use tui::style::Color;

pub struct CellOption {
    pub valid: bool,
    pub fg: Color,
    pub bg: Color,
}

impl Default for CellOption {
    fn default() -> Self {
        Self {
            valid: true,
            fg: Color::White,
            bg: Color::Black,
        }
    }
}

#[derive(Default)]
pub struct CellOptions {
    pub values: [CellOption; 9],
}
