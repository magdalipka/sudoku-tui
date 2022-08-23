use tui::style::Color;

pub struct Theme {
    pub white: Color,
    pub black: Color,
    pub red: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub purple: Color,
    pub cyan: Color,
    pub dark_grey: Color,
    pub grey: Color,
    pub light_grey: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            white: Color::Rgb(200, 200, 200),
            black: Color::Rgb(31, 35, 53),
            red: Color::Rgb(247, 118, 142),
            green: Color::Rgb(115, 218, 202),
            yellow: Color::Rgb(224, 175, 104),
            blue: Color::Rgb(122, 162, 247),
            purple: Color::Rgb(187, 154, 247),
            cyan: Color::Rgb(127, 207, 255),
            dark_grey: Color::Rgb(65, 72, 104),
            grey: Color::Rgb(121, 130, 169),
            light_grey: Color::Rgb(169, 177, 214),
        }
    }
}
