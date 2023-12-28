// use crate::figure::algebra::IVector;

pub type Cell = Option<Block>;

// #[derive(Debug)]
// pub struct PositionedBlock {
//     pub block: Block,
//     pub pos: IVector,
// }

#[derive(Clone, Debug)]
pub struct Block {
    pub btype: BlockType, //Múltiples tipos? (hielo ardiendo)
}

impl Block {
    pub fn new(t: BlockType) -> Self {
        Self { btype: t }
    }
}

#[derive(Clone, Debug)]
pub enum BlockType {
    Standard,
    Rock,
    Gravity,
    Ice,
    Fire,
}

impl Default for BlockType {
    fn default() -> Self {Self::Standard}
}