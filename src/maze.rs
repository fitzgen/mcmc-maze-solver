use crate::{Move, Path};
use rand::prelude::*;
use std::collections::HashSet;
use std::iter;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Cell {
    pub row: u32,
    pub col: u32,
}

pub struct Maze {
    edges: Vec<Vec<u32>>,
    rows: u32,
    cols: u32,
}

impl Maze {
    pub fn new(rng: &mut dyn RngCore, rows: u32, cols: u32) -> Maze {
        log::debug!("Maze::new");

        assert!(rows > 0);
        assert!(cols > 0);

        let mut edges = Vec::with_capacity((rows * cols) as usize);
        for _ in 0..rows {
            for _ in 0..cols {
                edges.push(vec![]);
            }
        }

        let mut maze = Maze { edges, rows, cols };

        let start = Cell {
            row: rng.gen_range(0, rows),
            col: rng.gen_range(0, cols),
        };

        let mut seen = HashSet::with_capacity(maze.edges.len());
        seen.insert(start);

        let mut stack = vec![start];
        let mut neighbors = Vec::with_capacity(4);
        while let Some(cell) = stack.last().cloned() {
            log::debug!("  cell = {:?}", cell);
            neighbors.clear();
            neighbors.extend(maze.neighbors(cell).filter(|n| !seen.contains(n)));
            log::debug!("    unseen neighbors = {:?}", neighbors);
            if let Some(neighbor) = neighbors.choose(rng).cloned() {
                let cell_index = maze.index_for_cell(cell);
                let neighbor_index = maze.index_for_cell(neighbor);
                maze.edges[cell_index].push(neighbor_index as u32);
                maze.edges[neighbor_index].push(cell_index as u32);
                debug_assert!(maze.edges[cell_index].len() <= 4);
                debug_assert!(maze.edges[neighbor_index].len() <= 4);
                seen.insert(neighbor);
                stack.push(neighbor);
            } else {
                stack.pop();
            }
        }

        maze
    }

    pub fn rows(&self) -> u32 {
        self.rows
    }

    pub fn cols(&self) -> u32 {
        self.cols
    }

    pub fn north(&self, from: Cell) -> Option<Cell> {
        if from.row > 0 {
            Some(Cell {
                row: from.row - 1,
                col: from.col,
            })
        } else {
            None
        }
    }

    pub fn east(&self, from: Cell) -> Option<Cell> {
        if from.col < self.cols - 1 {
            Some(Cell {
                row: from.row,
                col: from.col + 1,
            })
        } else {
            None
        }
    }

    pub fn south(&self, from: Cell) -> Option<Cell> {
        if from.row < self.rows - 1 {
            Some(Cell {
                row: from.row + 1,
                col: from.col,
            })
        } else {
            None
        }
    }

    pub fn west(&self, from: Cell) -> Option<Cell> {
        if from.col > 0 {
            Some(Cell {
                row: from.row,
                col: from.col - 1,
            })
        } else {
            None
        }
    }

    pub fn cells<'a>(&'a self) -> impl Iterator<Item = Cell> + 'a {
        let len = self.edges.len();
        (0..len).map(move |idx| self.cell_for_index(idx as u32))
    }

    fn index_for_cell(&self, cell: Cell) -> usize {
        debug_assert!(cell.row < self.rows);
        debug_assert!(cell.col < self.cols);
        let idx = (cell.row * self.cols + cell.col) as usize;
        debug_assert!(idx < self.edges.len());
        idx
    }

    fn cell_for_index(&self, index: u32) -> Cell {
        debug_assert!((index as usize) < self.edges.len());
        let row = index / self.cols;
        let col = index % self.cols;
        Cell { row, col }
    }

    pub fn is_edge_between(&self, a: Cell, b: Cell) -> bool {
        let a_idx = self.index_for_cell(a);
        let b_idx = self.index_for_cell(b) as u32;
        self.edges[a_idx].contains(&b_idx)
    }

    pub fn edges<'a>(&'a self, cell: Cell) -> impl Iterator<Item = Cell> + 'a {
        let index = self.index_for_cell(cell);
        self.edges[index]
            .iter()
            .cloned()
            .map(move |idx| self.cell_for_index(idx))
    }

    pub fn neighbors(&self, cell: Cell) -> impl Iterator<Item = Cell> {
        let n = self.north(cell);
        let e = self.east(cell);
        let s = self.south(cell);
        let w = self.west(cell);
        n.into_iter().chain(e).chain(s).chain(w)
    }

    pub fn follow_path<'a>(
        &'a self,
        start: Cell,
        path: &'a Path,
    ) -> impl Iterator<Item = Cell> + 'a {
        let mut moves = path.moves.iter().cloned();
        let mut current = start;
        iter::from_fn(move || {
            if let Some(cell) = match moves.next()? {
                Some(Move::North) => self.north(current),
                Some(Move::East) => self.east(current),
                Some(Move::South) => self.south(current),
                Some(Move::West) => self.west(current),
                None => None,
            } {
                current = cell;
            }
            Some(current)
        })
    }

    pub fn bird_flight_distance(&self, a: Cell, b: Cell) -> f64 {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_to_index_and_back() {
        let rows = 256;
        let cols = 128;
        let maze = Maze::new(&mut rand::thread_rng(), rows, cols);
        for row in 0..rows {
            for col in 0..cols {
                let cell = Cell { row, col };
                let idx = maze.index_for_cell(cell);
                let cell2 = maze.cell_for_index(idx as u32);
                assert_eq!(cell, cell2);
            }
        }
    }
}
