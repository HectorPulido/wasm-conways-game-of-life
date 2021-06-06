use std::fmt;
use wasm_bindgen::prelude::*;

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

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Self {
        let cells = (0..width * height)
            .map(|x| {
                if x % 7 == 0 || x % 3 == 0 {
                    return Cell::Alive;
                } else {
                    return Cell::Dead;
                }
            })
            .collect();

        return Universe {
            width: width,
            height: height,
            cells: cells,
        };
    }

    pub fn get_index(&self, row: u32, column: u32) -> usize {
        let row = row % self.width;
        let column = column % self.height;
        return (row * self.width + column) as usize;
    }

    fn alive_neighbors(&self, row: u32, column: u32) -> u8 {
        let mut neighbors = Vec::new();
        neighbors.push(self.get_index(row + 1, column - 1));
        neighbors.push(self.get_index(row + 1, column));
        neighbors.push(self.get_index(row + 1, column + 1));
        neighbors.push(self.get_index(row, column - 1));
        neighbors.push(self.get_index(row, column + 1));
        neighbors.push(self.get_index(row - 1, column - 1));
        neighbors.push(self.get_index(row - 1, column));
        neighbors.push(self.get_index(row - 1, column + 1));

        let mut count: u8 = 0;
        for neighbor in neighbors {
            count += self.cells[neighbor] as u8;
        }

        return count;
    }

    // what is time? if nobody ask me i know, but if somebody ask me, then idk bro
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let indx = self.get_index(row, col);
                let cell = self.cells[indx];
                let alive_neighbors = self.alive_neighbors(row, col);

                let alive_cell = match (cell, alive_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Alive, _) => Cell::Alive,
                    (Cell::Dead, 3) => Cell::Alive,
                    (Cell::Dead, _) => Cell::Dead,
                };

                next[indx] = alive_cell;
            }
        }

        self.cells = next;
    }

    pub fn render(&self) -> String {
        return self.to_string();
    }

    pub fn width(&self) -> u32 {
        return self.width;
    }

    pub fn height(&self) -> u32 {
        return self.width;
    }

    pub fn cells(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();

        for cell in self.cells.iter().cloned() {
            out.push(cell as u8);
        }

        return out;
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
