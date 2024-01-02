use std::iter::{zip, Zip};

use crate::block::{Block, BlockType};
use crate::algebra::IVector;
use super::shape::Shape;

#[derive(Clone, Debug)]
pub struct Piece<'bt> {
    blocks: Vec<Block<'bt>>,
    shape: Shape,
}

impl<'bt> Piece<'bt> {
    /// Constructs a new Figure from a list of `Block`s and a `Shape`. Both must have the same `len()`.
    /// 
    /// ```
	/// use atris::block::BlockType;
	/// use atris::piece::{shape::Shapes, Piece};
    /// use atris::blocktypes::*;
    /// use rand::thread_rng;
    /// 
    /// let mut rng = thread_rng();
    /// let mut shapes = Shapes::new();
    /// shapes.gen_until(4);
    /// let bt = standard::StandardType{};
    /// let fig1 = Piece::uniform(&bt, shapes.random(4, &mut rng));
    /// let fig2 = Piece::uniform(&bt, shapes.random(4, &mut rng));
    /// ```
    pub fn new(blocks: Vec<Block<'bt>>, shape: Shape) -> Self {
        if blocks.len() != shape.len() {
            panic!("Creating piece with different number of blocks and points in shape");
        };
        Self {
            blocks: blocks,
            shape: shape,
        }
    }

    /// Constructs a new Figure from a (borrowed) `BlockType` and a `Shape`. All the blocks will have the same type.
    /// 
    /// ```
	/// use atris::block::BlockType;
	/// use atris::piece::{shape::Shapes, Piece};
    /// use atris::blocktypes::*;
    /// use rand::thread_rng;
    /// 
    /// let mut rng = thread_rng();
    /// let mut shapes = Shapes::new();
    /// shapes.gen_until(4);
    /// let bt = standard::StandardType{};
    /// let fig1 = Piece::uniform(&bt, shapes.random(4, &mut rng));
    /// let fig2 = Piece::uniform(&bt, shapes.random(4, &mut rng));
    /// ```
    pub fn uniform(t: &'bt dyn BlockType, shape: Shape) -> Self {
        let mut blocks = Vec::new();
        blocks.resize(shape.positions().len(), Block::new(t));
        Self {
            blocks: blocks,
            shape: shape,
        }
    }

    pub fn blocks(&self) -> &Vec<Block<'bt>> { &self.blocks }

    pub fn shape(&self) -> &Shape { &self.shape }

    pub fn iter(&self) -> Zip<std::slice::Iter<'_, Block<'bt>>, std::slice::Iter<'_, IVector>> {
        zip(self.blocks.iter(), self.shape.iter())
    }
}

impl<'bt> IntoIterator for Piece<'bt> {
    type Item = (Block<'bt>, IVector);
    type IntoIter = <Zip<<Vec<Block<'bt>> as IntoIterator>::IntoIter, <Vec<IVector> as IntoIterator>::IntoIter> as IntoIterator>::IntoIter;

    /// Returns an iterator over tuples made by `Block` and point (in `Shape`).
    fn into_iter(self) -> Self::IntoIter {zip(self.blocks, self.shape)}
}
