pub mod state;
pub mod blocktype;

// use std::rc::Rc;
// use core::any::Any;

// use crate::figure::algebra::IVector;
pub use state::{State, BitRange};
pub use blocktype::BlockType;

// pub type Cell = Option<Block>;

// #[derive(Debug)]
// pub struct PositionedBlock {
//     pub block: Block,
//     pub pos: IVector,
// }

#[derive(Clone, Debug)]
pub struct Block<'bt> {
    // Convertir a referencia
    block_type: &'bt dyn BlockType, //Múltiples tipos? (hielo ardiendo)
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