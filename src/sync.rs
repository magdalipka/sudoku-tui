use crate::grid::Grid;
use std::fs::File;
use std::io::Write;
use std::io::{prelude::*, BufReader};

pub fn save(grid: &Grid) {
    let mut file = File::create("recent.sudoku").expect("file creation failed");

    let mut y = 0;
    loop {
        let mut x = 0;
        loop {
            if grid.cells[y][x].value != 0 {
                file.write_all(grid.cells[y][x].value.to_string().as_bytes())
                    .expect("cannot save sudoku");
                file.write_all("-".as_bytes()).expect("cannot save sudoku");
                if grid.cells[y][x].initial {
                    file.write_all("i".as_bytes()).expect("cannot save sudoku");
                }
                file.write_all("\n".as_bytes()).expect("cannot save sudoku");
            } else {
                file.write_all("0-".as_bytes()).expect("cannot save sudoku");
                let mut i = 0;
                loop {
                    if grid.cells[y][x].options.values[i].valid {
                        file.write_all((i + 1).to_string().as_bytes())
                            .expect("cannot save sudoku");
                    }

                    i += 1;
                    if i == 9 {
                        break;
                    }
                }
                file.write_all("\n".as_bytes()).expect("cannot save sudoku");
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

pub fn load(path: &String) -> Grid {
    let mut grid = Grid::default();
    let reader = BufReader::new(File::open(path).expect("file read failed"));

    let mut cell_index = 0;
    for _line in reader.lines() {
        let line = _line.unwrap();
        let mut parts = line.split("-");

        let left = parts.next().unwrap();
        let right = parts.next().unwrap();

        let column = cell_index % 9;
        let row = (cell_index - column) / 9;

        if left == "0" {
            for char in right.to_string().chars() {
                let value = char.to_digit(10).unwrap() as usize;
                grid.cells[row][column].options.values[value - 1].valid = true;
            }
        } else {
            let value: usize = left.parse().unwrap();
            grid.cells[row][column].value = value;
            if right == "i" {
                grid.cells[row][column].initial = true;
            }
        }

        cell_index += 1;
        if cell_index == 81 {
            break;
        }
    }

    grid
}
