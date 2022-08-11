use crate::{cell::Cell, grid::Grid};
use tui::{
    backend::CrosstermBackend,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    widgets::{Block, Borders, StatefulWidget, Widget},
    Terminal,
};
//
pub struct Board {
    grid: Grid,
    current_position: (usize, usize),
}

impl Default for Board {
    fn default() -> Self {
        Self {
            grid: Grid::default(),
            current_position: (0, 0),
        }
    }
}

impl Board {
    pub fn move_up(&mut self) {
        self.current_position.0 = (9 + self.current_position.0 - 1) % 9;
        // TODO: recolour bg
    }

    pub fn move_down(&mut self) {
        self.current_position.0 = (self.current_position.0 + 1) % 9;
    }

    pub fn move_right(&mut self) {
        self.current_position.1 = (9 + self.current_position.1 + 1) % 9;
    }

    pub fn move_left(&mut self) {
        self.current_position.1 = (self.current_position.1 - 1) % 9;
    }

    pub fn set_value(&mut self, value: usize) {
        let (x, y) = self.current_position;
        if self.grid.cells[x][y].initial {
            return ();
        };
        self.grid.cells[x][y].set_value(value);

        // TODO: check conflict

        let mut i = 0;
        loop {
            self.grid.cells[i][self.current_position.1].toggle_option(value);
            self.grid.cells[self.current_position.0][i].toggle_option(value);

            i += 1;
            if i == 9 {
                break;
            }
        }

        // calculate top left position of box, color from there
        let box_x: usize = x - (x % 3);
        let box_y: usize = y - (y % 3);

        self.grid.cells[box_y][box_x].toggle_option(value);
        self.grid.cells[box_y][box_x + 1].toggle_option(value);
        self.grid.cells[box_y][box_x + 2].toggle_option(value);
        self.grid.cells[box_y + 1][box_x].toggle_option(value);
        self.grid.cells[box_y + 1][box_x + 1].toggle_option(value);
        self.grid.cells[box_y + 1][box_x + 2].toggle_option(value);
        self.grid.cells[box_y + 2][box_x].toggle_option(value);
        self.grid.cells[box_y + 2][box_x + 1].toggle_option(value);
        self.grid.cells[box_y + 2][box_x + 2].toggle_option(value);
    }

    pub fn toggle_option(&mut self, value: usize) {
        let (x, y) = self.current_position;
        if !self.grid.cells[x][y].initial {
            self.grid.cells[x][y].value = 0;
            self.grid.cells[x][y].toggle_option(value);
        }
    }

    pub fn reset_colors(&mut self) {
        self.grid.reset_markings();

        todo!()
    }

    pub fn highlight(&mut self, value: usize) {
        self.grid.reset_markings();

        if value == 0 {
            return ();
        };

        let mut y: usize = 0;
        loop {
            let mut x: usize = 0;
            loop {
                if self.grid.cells[y][x].value == value {
                    self.grid.cells[y][x].bg = Color::Red;
                    self.grid.cells[y][x].fg = Color::White;
                } else if self.grid.cells[y][x].options.values[value - 1].valid {
                    self.grid.cells[y][x].options.values[value - 1].bg = Color::Red;
                    self.grid.cells[y][x].options.values[value - 1].fg = Color::White;
                }

                x += 1;
                if x == 9 {
                    break;
                }
            }
            y += 1;
            if y == 9 {
                break;
            }
        }
    }

    pub fn mark(&mut self, value: usize) {
        let (x, y) = self.current_position;
        if !self.grid.cells[x][y].value != 0 {
            self.grid.cells[x][y].options.values[value - 1].bg = Color::White;
            self.grid.cells[x][y].options.values[value - 1].fg = Color::Magenta;
        } else {
            self.grid.cells[x][y].bg = Color::White;
            self.grid.cells[x][y].fg = Color::Magenta;
        }
    }
}

pub struct BoardWidget {}

impl StatefulWidget for BoardWidget {
    type State = Board;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // render top line

        let mut row: u16 = 0;
        let mut column: u16 = 0;

        loop {
            loop {
                // loop controll

                // println!("areax: {} areay: {} areaw: {} areah: {}, bufx: {} bufy: {} statex: {} statey: {}", area.x, area.y, area.width, area.height, bufx, bufy, statex, statey);

                let cell = state.grid.cell_at(row as usize, column as usize);
                let value = char::from_u32(cell.value as u32 + 48).unwrap_or('e');

                let mut center_x = column * 8 + 1 + area.x + 3;
                let mut center_y = row * 4 + 1 + area.y + 1;

                if row > 5 {
                    center_y += 2;
                } else if row > 2 {
                    center_y += 1;
                };

                if column > 5 {
                    center_x += 2;
                } else if column > 2 {
                    center_x += 1;
                };

                if cell.value != 0 {
                    // solved cell
                    buf.get_mut(center_x, center_y)
                        .set_char(value)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);

                    // surroundings
                    buf.get_mut(center_x - 2, center_y - 1)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);
                    buf.get_mut(center_x - 2, center_y)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);
                    buf.get_mut(center_x - 2, center_y + 1)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);

                    buf.get_mut(center_x - 1, center_y - 1)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);
                    buf.get_mut(center_x - 1, center_y)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);
                    buf.get_mut(center_x - 1, center_y + 1)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);

                    buf.get_mut(center_x, center_y - 1)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);
                    buf.get_mut(center_x, center_y + 1)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);

                    buf.get_mut(center_x + 1, center_y - 1)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);
                    buf.get_mut(center_x + 1, center_y)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);
                    buf.get_mut(center_x + 1, center_y + 1)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);

                    buf.get_mut(center_x + 2, center_y - 1)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);
                    buf.get_mut(center_x + 2, center_y)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);
                    buf.get_mut(center_x + 2, center_y + 1)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);
                } else {
                    // cell with options
                    buf.get_mut(center_x - 3, center_y - 1)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);
                    buf.get_mut(center_x - 2, center_y - 1)
                        .set_char(if cell.options.values[0].valid {
                            '1'
                        } else {
                            ' '
                        })
                        .set_bg(cell.options.values[0].bg)
                        .set_fg(cell.options.values[0].fg);
                    buf.get_mut(center_x - 1, center_y - 1)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);
                    buf.get_mut(center_x, center_y - 1)
                        .set_char(if cell.options.values[1].valid {
                            '2'
                        } else {
                            ' '
                        })
                        .set_bg(cell.options.values[1].bg)
                        .set_fg(cell.options.values[1].fg);
                    buf.get_mut(center_x + 1, center_y - 1)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);
                    buf.get_mut(center_x + 2, center_y - 1)
                        .set_char(if cell.options.values[2].valid {
                            '3'
                        } else {
                            ' '
                        })
                        .set_bg(cell.options.values[2].bg)
                        .set_fg(cell.options.values[2].fg);
                    buf.get_mut(center_x + 3, center_y - 1)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);

                    buf.get_mut(center_x - 3, center_y)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);
                    buf.get_mut(center_x - 2, center_y)
                        .set_char(if cell.options.values[3].valid {
                            '4'
                        } else {
                            ' '
                        })
                        .set_bg(cell.options.values[3].bg)
                        .set_fg(cell.options.values[3].fg);
                    buf.get_mut(center_x - 1, center_y)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);
                    buf.get_mut(center_x, center_y)
                        .set_char(if cell.options.values[4].valid {
                            '5'
                        } else {
                            ' '
                        })
                        .set_bg(cell.options.values[4].bg)
                        .set_fg(cell.options.values[4].fg);
                    buf.get_mut(center_x + 1, center_y)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);
                    buf.get_mut(center_x + 2, center_y)
                        .set_char(if cell.options.values[5].valid {
                            '6'
                        } else {
                            ' '
                        })
                        .set_bg(cell.options.values[5].bg)
                        .set_fg(cell.options.values[5].fg);
                    buf.get_mut(center_x + 3, center_y)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);

                    buf.get_mut(center_x - 3, center_y + 1)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);
                    buf.get_mut(center_x - 2, center_y + 1)
                        .set_char(if cell.options.values[6].valid {
                            '7'
                        } else {
                            ' '
                        })
                        .set_bg(cell.options.values[6].bg)
                        .set_fg(cell.options.values[6].fg);
                    buf.get_mut(center_x - 1, center_y + 1)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);
                    buf.get_mut(center_x, center_y + 1)
                        .set_char(if cell.options.values[7].valid {
                            '8'
                        } else {
                            ' '
                        })
                        .set_bg(cell.options.values[7].bg)
                        .set_fg(cell.options.values[7].fg);
                    buf.get_mut(center_x + 1, center_y + 1)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);
                    buf.get_mut(center_x + 2, center_y + 1)
                        .set_char(if cell.options.values[8].valid {
                            '9'
                        } else {
                            ' '
                        })
                        .set_bg(cell.options.values[8].bg)
                        .set_fg(cell.options.values[8].fg);
                    buf.get_mut(center_x + 3, center_y + 1)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);
                }

                // buf.get_mut(area.x + bufx + 1, area.y + bufy + 1)
                //     .set_char(value);

                // loop controll
                column += 1;
                if column == 9 {
                    column = 0;
                    break;
                }
            }
            row += 1;
            if row == 9 {
                break;
            }
        }
    }
}

// pub struct BoardWidget {
//     buffer: Buffer,
// }

// impl BoardWidget {
//     pub fn from(board: Board) -> Self {
//         Self {
//             buffer: Buffer::default(),
//         }
//     }
// }

// impl Widget for BoardWidget {
//     fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
//         todo!()
//     }
// }
