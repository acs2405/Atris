use std::collections::HashMap;

use crate::{grid::Grid, figure::{Figure, algebra::UVector}};

#[derive(Debug)]
pub struct GridSession<'a> {
    id: usize,
    grid: Grid<'a>,
    // blocks: Vec<PositionedBlock<'a>>,
    figures: HashMap<usize, Figure<'a>>,
}

impl<'a> GridSession<'a> {
    pub fn new(grid_session_id: usize, grid_bounds: UVector) -> Self {
        Self {
            id: grid_session_id,
            grid: Grid::new(grid_bounds),
            figures: HashMap::new(),
        }
    }
}