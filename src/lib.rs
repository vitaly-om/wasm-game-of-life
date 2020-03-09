mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column)  as usize
    }

    fn normalize_coordinate(&self, value: u32, coord_max: u32) -> u32 {
        value.checked_sub(1).unwrap_or(coord_max - 1) % coord_max
    }

    fn get_live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut counter: u8 = 0;
        let deltas: [u32; 3] = [0, 1, 2];

        for delta_row in deltas.iter().cloned() {
            for delta_column in deltas.iter().cloned() {
                if delta_row == 1 && delta_column == 1 {
                    continue;
                }

                let curr_row = self.normalize_coordinate(row + delta_row, self.height);
                let curr_column = self.normalize_coordinate(column + delta_column, self.width);

                let idx = self.get_index(curr_row, curr_column);
                counter += self.cells[idx] as u8;
            };
        }

        counter
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next_cells = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.height {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.get_live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next_cells[idx] = next_cell;
            }
        }

        self.cells = next_cells;
    }

    pub fn new() -> Universe {
        let width: u32 = 64;
        let height: u32 = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {width, height, cells}
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
