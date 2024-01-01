// use std::rc::Rc;
// use core::any::Any;

use std::cmp::Ordering;

// use crate::figure::algebra::IVector;
pub use super::state::{State, BitRange};
pub use super::blocktype::BlockType;

use crate::figure::algebra::UVector;

// pub type Cell = Option<Block>;

// #[derive(Debug)]
// pub struct PositionedBlock {
//     pub block: Block,
//     pub pos: IVector,
// }

#[derive(Clone, Debug)]
pub struct Block<'bt> {
    pub block_type: &'bt dyn BlockType, //Múltiples tipos? (hielo ardiendo)
    pub state: State<u64>,
}

impl<'bt> Block<'bt> {
    pub fn new(t: &'bt dyn BlockType) -> Self {
        Self { block_type: t, state: State::default() }
    }
    pub fn get_type(&self) -> &'bt dyn BlockType {
        self.block_type
    }
}

#[derive(Clone, Debug)]
pub struct PositionedBlock<'bt> {
    pub pos: UVector,
    pub block: Block<'bt>,
}

impl<'bt> PartialEq for PositionedBlock<'bt> {
	/// Partially checks equality of two `BlockPos`. Two `BlockPos` objects are equal if and only if their `pos` attribute are equal.
	fn eq(&self, rhs: &Self) -> bool {
        self.pos.eq(&rhs.pos)
	}
}

impl<'bt> Eq for PositionedBlock<'bt> {}

impl<'bt> PartialOrd for PositionedBlock<'bt> {
	/// Partially compares two `BlockPos`, comparing just their positions.
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.pos.partial_cmp(&rhs.pos)
	}
}

impl<'bt> Ord for PositionedBlock<'bt> {
	/// Compares two `BlockPos`, comparing just their positions.
	fn cmp(&self, rhs: &Self) -> Ordering {
        self.pos.cmp(&rhs.pos)
	}
}