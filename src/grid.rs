use crate::cell::Cell;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget},
    Terminal,
};

#[derive(Default)]
pub struct Grid {
    // outer array is of rows, inner are cells
    pub cells: [[Cell; 9]; 9],
}

impl Grid {
    pub fn cell_at(&self, row: usize, column: usize) -> &Cell {
        &self.cells[row][column]
    }

    pub fn reset_markings(&mut self) {
        // for (_, mut row) in self.cells.into_iter().enumerate() {
        //     for (_, mut cell) in row.into_iter().enumerate() {
        //         cell.reset_colors();
        //     }
        // }

        let mut y: usize = 0;
        loop {
            let mut x: usize = 0;

            loop {
                self.cells[y][x].reset_colors();

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
