use std::collections::HashSet;

use crate::{cell::Cell, grid::Grid, theme::Theme};
use tui::{
    backend::CrosstermBackend,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
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
    pub fn from(grid: Grid) -> Self {
        Self {
            grid: grid,
            current_position: (0, 0),
        }
    }

    pub fn move_up(&mut self) {
        self.current_position.0 = (9 + self.current_position.0 - 1) % 9;
        // TODO: recolour bg
    }

    pub fn move_down(&mut self) {
        self.current_position.0 = (self.current_position.0 + 1) % 9;
    }

    pub fn move_right(&mut self) {
        self.current_position.1 = (self.current_position.1 + 1) % 9;
    }

    pub fn move_left(&mut self) {
        self.current_position.1 = (9 + self.current_position.1 - 1) % 9;
    }

    pub fn set_value(&mut self, value: usize) {
        let (y, x) = self.current_position;
        if self.grid.cells[y][x].initial {
            return ();
        };
        self.grid.cells[y][x].set_value(value);

        // TODO: check conflict

        let mut i = 0;
        loop {
            self.grid.cells[i][self.current_position.1].remove_option(value);
            self.grid.cells[i][self.current_position.1].options.values[value - 1].reset_colors();
            self.grid.cells[self.current_position.0][i].remove_option(value);
            self.grid.cells[self.current_position.0][i].options.values[value - 1].reset_colors();

            i += 1;
            if i == 9 {
                break;
            }
        }

        // calculate top left position of box, color from there
        let box_x: usize = x - (x % 3);
        let box_y: usize = y - (y % 3);

        self.grid.cells[box_y][box_x].remove_option(value);
        self.grid.cells[box_y][box_x].options.values[value - 1].reset_colors();
        self.grid.cells[box_y][box_x + 1].remove_option(value);
        self.grid.cells[box_y][box_x + 1].options.values[value - 1].reset_colors();
        self.grid.cells[box_y][box_x + 2].remove_option(value);
        self.grid.cells[box_y][box_x + 2].options.values[value - 1].reset_colors();
        self.grid.cells[box_y + 1][box_x].remove_option(value);
        self.grid.cells[box_y + 1][box_x].options.values[value - 1].reset_colors();
        self.grid.cells[box_y + 1][box_x + 1].remove_option(value);
        self.grid.cells[box_y + 1][box_x + 1].options.values[value - 1].reset_colors();
        self.grid.cells[box_y + 1][box_x + 2].remove_option(value);
        self.grid.cells[box_y + 1][box_x + 2].options.values[value - 1].reset_colors();
        self.grid.cells[box_y + 2][box_x].remove_option(value);
        self.grid.cells[box_y + 2][box_x].options.values[value - 1].reset_colors();
        self.grid.cells[box_y + 2][box_x + 1].remove_option(value);
        self.grid.cells[box_y + 2][box_x + 1].options.values[value - 1].reset_colors();
        self.grid.cells[box_y + 2][box_x + 2].remove_option(value);
        self.grid.cells[box_y + 2][box_x + 2].options.values[value - 1].reset_colors();
    }

    pub fn toggle_option(&mut self, value: usize) {
        let (x, y) = self.current_position;
        if !self.grid.cells[x][y].initial {
            self.grid.cells[x][y].value = 0;
            self.grid.cells[x][y].toggle_option(value);
            if self.grid.cells[x][y].options.values[value - 1].valid {
                self.grid.cells[x][y].options.values[value - 1].bg = Theme::default().red;
                self.grid.cells[x][y].options.values[value - 1].fg = Theme::default().white;
            } else {
                self.grid.cells[x][y].options.values[value - 1].reset_colors();
            }
        }
    }

    pub fn reset_colors(&mut self) {
        self.grid.reset_markings();
    }

    pub fn highlight_only(&mut self, value: usize) {
        self.grid.reset_markings();
        self.highlight(value);
    }

    pub fn highlight(&mut self, value: usize) {
        if value == 0 {
            return ();
        };

        let mut y: usize = 0;
        loop {
            let mut x: usize = 0;
            loop {
                if self.grid.cells[y][x].value == value {
                    self.grid.cells[y][x].bg = Theme::default().blue;
                    self.grid.cells[y][x].fg = Theme::default().black;
                } else if self.grid.cells[y][x].options.values[value - 1].valid {
                    self.grid.cells[y][x].options.values[value - 1].bg = Theme::default().blue;
                    self.grid.cells[y][x].options.values[value - 1].fg = Theme::default().black;
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
        if self.grid.cells[x][y].value == 0 && self.grid.cells[x][y].options.values[value - 1].valid
        {
            self.grid.cells[x][y].options.values[value - 1].bg = Theme::default().purple;
            self.grid.cells[x][y].options.values[value - 1].fg = Theme::default().black;
        }
    }

    pub fn autofill(&mut self) {
        let mut y: usize = 0;
        loop {
            let mut x: usize = 0;
            loop {
                if self.grid.cells[y][x].value == 0 {
                    // cell needs to be filled with notes

                    // loop over possible values
                    let mut value: usize = 1;
                    loop {
                        // check if sector contains value

                        let mut possible = true;

                        let mut i: usize = 0;

                        // box
                        let box_x: usize = x - (x % 3);
                        let box_y: usize = y - (y % 3);

                        if self.grid.cells[box_y][box_x].value == value
                            || self.grid.cells[box_y][box_x + 1].value == value
                            || self.grid.cells[box_y][box_x + 2].value == value
                            || self.grid.cells[box_y + 1][box_x].value == value
                            || self.grid.cells[box_y + 1][box_x + 1].value == value
                            || self.grid.cells[box_y + 1][box_x + 2].value == value
                            || self.grid.cells[box_y + 2][box_x].value == value
                            || self.grid.cells[box_y + 2][box_x + 1].value == value
                            || self.grid.cells[box_y + 2][box_x + 2].value == value
                        {
                            possible = false;
                        }
                        // line
                        if possible {
                            loop {
                                if self.grid.cells[y][i].value == value
                                    || self.grid.cells[i][x].value == value
                                {
                                    possible = false;
                                    break;
                                }

                                i += 1;
                                if i == 9 {
                                    break;
                                }
                            }
                        }

                        if possible {
                            self.grid.cells[y][x].add_option(value);
                        }

                        value += 1;
                        if value == 10 {
                            break;
                        }
                    }
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

                    if cell.initial {
                        buf.get_mut(center_x, center_y)
                            .set_char(value)
                            .set_bg(cell.bg)
                            .set_fg(cell.fg)
                            .set_style(Style::default().add_modifier(Modifier::UNDERLINED));
                    } else {
                        buf.get_mut(center_x, center_y)
                            .set_char(value)
                            .set_bg(cell.bg)
                            .set_fg(cell.fg)
                            .set_style(
                                Style::default()
                                    .add_modifier(Modifier::UNDERLINED)
                                    .add_modifier(Modifier::ITALIC),
                            );
                    }

                    // surroundings
                    buf.get_mut(center_x - 3, center_y - 1)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);
                    buf.get_mut(center_x - 3, center_y)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);
                    buf.get_mut(center_x - 3, center_y + 1)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);

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

                    buf.get_mut(center_x + 3, center_y - 1)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);
                    buf.get_mut(center_x + 3, center_y)
                        .set_bg(cell.bg)
                        .set_fg(cell.fg);
                    buf.get_mut(center_x + 3, center_y + 1)
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

        // render position
        if state.current_position.0 != 9 && state.current_position.1 != 9 {
            let mut center_x: u16 = (state.current_position.1 * 8 + 1 + (area.x as usize) + 3)
                .try_into()
                .unwrap();
            let mut center_y: u16 = (state.current_position.0 * 4 + 1 + (area.y as usize) + 1)
                .try_into()
                .unwrap();

            if state.current_position.0 > 5 {
                center_y += 2;
            } else if state.current_position.0 > 2 {
                center_y += 1;
            };

            if state.current_position.1 > 5 {
                center_x += 2;
            } else if state.current_position.1 > 2 {
                center_x += 1;
            };

            buf.get_mut(center_x - 3, center_y - 2).set_char('.');
            buf.get_mut(center_x - 2, center_y - 2).set_char('.');
            buf.get_mut(center_x - 1, center_y - 2).set_char('.');
            buf.get_mut(center_x, center_y - 2).set_char('.');
            buf.get_mut(center_x + 1, center_y - 2).set_char('.');
            buf.get_mut(center_x + 2, center_y - 2).set_char('.');
            buf.get_mut(center_x + 3, center_y - 2).set_char('.');

            buf.get_mut(center_x - 4, center_y - 1).set_char('.');
            buf.get_mut(center_x - 4, center_y).set_char('.');
            buf.get_mut(center_x - 4, center_y + 1).set_char('.');

            buf.get_mut(center_x + 4, center_y - 1).set_char('.');
            buf.get_mut(center_x + 4, center_y).set_char('.');
            buf.get_mut(center_x + 4, center_y + 1).set_char('.');

            buf.get_mut(center_x - 3, center_y + 2).set_char('.');
            buf.get_mut(center_x - 2, center_y + 2).set_char('.');
            buf.get_mut(center_x - 1, center_y + 2).set_char('.');
            buf.get_mut(center_x, center_y + 2).set_char('.');
            buf.get_mut(center_x + 1, center_y + 2).set_char('.');
            buf.get_mut(center_x + 2, center_y + 2).set_char('.');
            buf.get_mut(center_x + 3, center_y + 2).set_char('.');
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
