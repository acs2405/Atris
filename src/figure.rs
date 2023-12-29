pub mod algebra;
pub mod shape;

use std::iter::{zip, Zip};
// use core::any::Any;

use crate::block::{Block, BlockType};
use algebra::IVector;
use shape::Shape;

#[derive(Clone, Debug)]
pub struct Figure<'bt> {
    blocks: Vec<Block<'bt>>,
    shape: Shape,
    // rotation: IVector,
}

impl<'bt> Figure<'bt> {
    /// Constructs a new Figure from a (borrowed) `BlockType` and a `Shape`. All the blocks will have the same type.
    /// 
    /// ```
	/// use atris::block::BlockType;
	/// use atris::figure::{shape::Shapes, Figure};
    /// use atris::block::blocktype::*;
    /// use rand::thread_rng;
    /// 
    /// let mut rng = thread_rng();
    /// let mut shapes = Shapes::new(&mut rng);
    /// shapes.gen_until(4);
    /// let bt = standard::StandardType{};
    /// let fig1 = Figure::uniform(&bt, shapes.random_shape(4));
    /// let fig2 = Figure::uniform(&bt, shapes.random_shape(4));
    /// ```
    pub fn uniform(t: &'bt dyn BlockType, shape: Shape) -> Self {
        let mut blocks = Vec::new();
        blocks.resize(shape.positions().len(), Block::new(t));
        Self {
            blocks: blocks,
            shape: shape,
            // rotation: Default::default(),
        }
    }
}

impl<'bt> IntoIterator for Figure<'bt> {
    type Item = (Block<'bt>, IVector);
    type IntoIter = <Zip<<Vec<Block<'bt>> as IntoIterator>::IntoIter, <Vec<IVector> as IntoIterator>::IntoIter> as IntoIterator>::IntoIter;

    /// Returns an iterator over tuples made by `Block` and point (in `Shape`).
    fn into_iter(self) -> Self::IntoIter {zip(self.blocks, self.shape)}
}
