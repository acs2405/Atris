pub mod standard;

use core::fmt::Debug;

use super::Block;

/// This `trait` represents the behaviour of each type of `Block`.
pub trait BlockType: Debug {
    // const NAME: str;
    fn on_destroy(&self, b: &Block);
    // fn new_block(&self) -> Block<Self> {
    //     Block::new(self)
    // }
}

// #[derive(Clone, Debug)]
// pub struct BlockType {
//     pub name: str,
//     pub on_destroy: fn(&Block) -> Option<Block>,
// }

// const STANDARD: BlockType = BlockType {
//     name: "Standard",
//     on_destroy: |b| {None},
// };

// #[derive(Clone, Debug)]
// pub enum BlockType {
//     Standard,
//     Rock,
//     Gravity,
//     Ice,
//     Fire,
// }

// impl Default for BlockType {
//     fn default() -> Self {Self::Standard}
// }