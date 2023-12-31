// use std::iter::Zip;
use std::ops::{Index, IndexMut};

use crate::block::{Block, PositionedBlock};
use crate::figure::algebra::{Vector, UVector, IVector};
use crate::figure::{Figure, shape::Shapes};

#[derive(Debug)]
pub struct Grid<'bt> {
    bounds: UVector,
    rows: Vec<Vec<Option<Block<'bt>>>>,
    // blocks: Vec<PositionedBlock>,
}

impl<'bt> Grid<'bt> {
    pub fn new(bounds: UVector) -> Self {
        let mut g = Self { bounds: bounds, rows: Vec::new() };
        let mut row = Vec::new();
        row.resize(bounds.0, None);
        g.rows.resize(bounds.1, row);
        g
    }

    pub fn bounds(&self) -> UVector { self.bounds }

    pub fn n_cols(&self) -> usize { self.bounds.0 }
    pub fn n_rows(&self) -> usize { self.bounds.1 }

    pub fn put(&mut self, pos: UVector, b: Block<'bt>) -> Option<Block<'bt>> {
        let old = self.rows[pos.1][pos.0].replace(b);
        // match self.blocks.binary_search(&pb) {
        //     Ok(i) => self.blocks[i] = pos,
        //     Err(i) => self.blocks.insert(i, pb),
        // };
        old
    }
    
    pub fn take(&mut self, pos: UVector) -> Option<Block<'bt>> {
        let b = self.rows[pos.1][pos.0].take()?;
        // match self.blocks.binary_search(&pos) {
        //     Ok(i) => self.blocks[i] = pos,
        //     Err(i) => self.blocks.insert(i, pos),
        // };
        Some(b)
    }

    pub fn try_move(&self, fig: &Figure<'bt>, x_offset: i32) {todo!()}

    pub fn try_rotate(&self, fig: &Figure<'bt>, angle: i32) {todo!()}

    // fn pos_as_usize(&self, p: UVector) -> usize { self.bounds.0*p.1 + p.0 }

    pub fn fits_in(&self, fig: &Figure<'bt>, offset: IVector, angle: i32) -> bool {
        // for (b, &p) in fig.iter() { // To exclude ghost blocks
        for &p in fig.shape().rotated(angle).iter() {
            let pos = fig.position + p + offset;
            if !self.pos_available(pos) {
                return false;
            }
        };
        true
    }

    pub fn figure_pos_correction(&self, fig: Figure<'bt>, x_offset: i32, angle: i32) -> Option<IVector> {
        let offset = Vector(x_offset, 0);
        let mut correction = Vector(0i32, 0i32);
        for &p in fig.shape().rotated(angle).iter() {
            let pos = fig.position + p + offset;
            let corr: IVector = self.pos_correction(pos)?;
            // Check that one correction does not contradict others:
            if corr.0 != 0 && correction.0.signum() == -corr.0.signum() { //) ||
                // (corr.1 != 0 && correction.1.signum() == -corr.1.signum()) {
                return None;
            }
            correction.0 += corr.0;
            // correction = correction + corr;
        };
        Some(correction)
    }

    pub fn pos_available(&self, pos: IVector) -> bool {
        self.pos_in_bounds(pos) && !self.pos_overlaps(pos)
    }

    pub fn pos_overlaps(&self, pos: IVector) -> bool {
        // Devolver false si el bloque es fantasma
        self[UVector::try_from(pos).unwrap()].is_some()
    }

    pub fn pos_in_bounds(&self, pos: IVector) -> bool {
        (0..self.n_cols() as i32).contains(&pos.0) && (0..self.n_rows() as i32).contains(&pos.1)
    }
    
    pub fn pos_correction(&self, pos: IVector) -> Option<IVector> {
        let mut correction = Vector(0, 0);
        if pos.0 < 0 {correction.0 = -pos.0}
        else if pos.0 >= self.n_cols() as i32 {correction.0 = pos.0 - self.n_cols() as i32 - 1};
        // if pos.1 < 0 {correction.1 = -pos.1}
        // else if pos.1 >= self.n_rows() as i32 {correction.1 = pos.1 - self.n_rows() as i32 - 1};
        if self.pos_overlaps(pos + correction) {
            // TODO: take into account the center of the figure to move towards it when overlapping
            return None;
        };
        Some(correction)
    }

    pub fn full_rows(&self) -> Vec<usize> {
        let mut full_rows = Vec::new();
        for i in 0..self.n_rows() {
            if self.full_row(i) {
                full_rows.push(i);
            }
        };
        full_rows
    }

    pub fn empty_rows(&self) -> Vec<usize> {
        let mut empty_rows = Vec::new();
        let mut all_empty = true;
        for i in self.n_rows()-1..=0 {
            if all_empty {
                if self.empty_row(i) {
                    empty_rows.push(i);
                }
            } else if !self.empty_row(i) {
                all_empty = false;
            }
        };
        empty_rows
    }

    pub fn last_non_empty_row(&self) -> Option<usize> {
        for i in self.n_rows()-1..=0 {
            if !self.empty_row(i) {
                return Some(i);
            }
        };
        None
    }

    pub fn full_row(&self, i: usize) -> bool {
        // In the future, exclude ghost blocks
        self.rows[i].iter().filter(|cell| cell.is_some()).count() == self.n_cols()
    }

    pub fn empty_row(&self, i: usize) -> bool {
        self.rows[i].iter().filter(|cell| cell.is_none()).count() == self.n_cols()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Vec<Option<Block<'bt>>>> {
        self.rows.iter()
        // GridIterator { grid: self, index: 0 }
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Vec<Option<Block<'bt>>>> {
        self.rows.iter_mut()
        // GridMutIterator { grid: self, index: 0 }
    }
}

impl<'bt> Index<usize> for Grid<'bt> {
    type Output = [Option<Block<'bt>>];

    fn index(&self, row: usize) -> &Self::Output {
        &self.rows[row] //[self.bounds.0*i..self.bounds.0*(i + 1)]
    }
}

// impl IndexMut<usize> for Grid {
//     fn index_mut(&mut self, row: usize) -> &mut Self::Output {
//         &mut self.grid[row] //[self.ncols*i..self.ncols*(i + 1)]
//     }
// }

impl<'bt> Index<UVector> for Grid<'bt> {
    type Output = Option<Block<'bt>>;

    fn index(&self, pos: UVector) -> &Self::Output {
        &self.rows[pos.1][pos.0]
    }
}

impl<'bt> IndexMut<UVector> for Grid<'bt> {
    fn index_mut(&mut self, pos: UVector) -> &mut Self::Output {
        &mut self.rows[pos.1][pos.0]
    }
}

// impl IntoIterator for Grid {
//     type Item = BlockPos<'bt>;
//     type IntoIter = <Vec<BlockPos<'bt>> as IntoIterator>::IntoIter;

//     /// Returns an iterator over tuples made by `Block` and point (in `Shape`).
//     fn into_iter(self) -> Self::IntoIter { self.blocks.into_iter() }
// }

// struct GridIterator<'g> {
//     grid: &'g Grid,
//     index: usize,
// }

// impl<'b> Iterator for GridIterator<'b> {
//     type Item = &'b Option<Block<'bt>>;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.index < self.grid.grid.len() {
//             let b = &self.grid.grid[self.index];
//             self.index += 1;
//             Some(b)
//         } else {
//             None
//         }
//     }
// }
// struct GridMutIterator<'g> {
//     grid: &'g mut Grid,
//     index: usize,
// }

// impl<'g> Iterator for GridMutIterator<'g> {
//     type Item = &'g mut Option<Block<'bt>>;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.index < self.grid.grid.len() {
//             let b = &mut self.grid.grid[self.index];
//             self.index += 1;
//             Some(b)
//         } else {
//             None
//         }
//     }
// }