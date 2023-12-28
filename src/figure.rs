pub mod algebra;
pub mod shape;

use std::iter::{zip, Zip};

use crate::block::{Block, BlockType};
use algebra::IVector;
use shape::Shape;

use rand::{thread_rng, Rng};

#[derive(Clone, Debug)]
pub struct Figure {
    blocks: Vec<Block>,
    shape: Shape,
    // rotation: IVector,
}

impl Figure {
    /// Constructs a new Figure from the type of all its blocks and the possible shapes to choose a random one from.
    /// 
    /// ```
	/// use atris::block::BlockType;
	/// use atris::figure::{shape::Shapes, Figure};
    /// let mut shapes = Shapes::new();
    /// shapes.gen_until(4);
    /// let shapes4 = shapes.shapes(4);
    /// let fig1 = Figure::random_uniform(BlockType::Standard, &shapes4);
    /// let fig2 = Figure::random_uniform(BlockType::Standard, &shapes4);
    /// ```
    pub fn random_uniform(t: BlockType, shapes: &Vec<Shape>) -> Self {
        let mut rng = thread_rng();
        let i: usize = rng.gen_range(0..shapes.len());
        Self::uniform(t, &shapes[i])
    }
    
    /// Constructs a new Figure from the type of all its blocks and the shape of the figure.
    /// 
    /// ```
	/// use atris::block::BlockType;
	/// use atris::figure::{shape::Shapes, Figure};
    /// let mut shapes = Shapes::new();
    /// shapes.gen_until(4);
    /// let shapes4 = shapes.shapes(4);
    /// let fig1 = Figure::uniform(BlockType::Standard, &shapes4[2]);
    /// let fig2 = Figure::uniform(BlockType::Standard, &shapes4[5]);
    /// ```
    pub fn uniform(t: BlockType, shape: &Shape) -> Self {
        let mut blocks = Vec::new();
        blocks.resize(shape.positions().len(), Block::new(t));
        Self {
            blocks: blocks,
            shape: shape.clone(),
            // rotation: Default::default(),
        }
    }
}

impl IntoIterator for Figure {
    type Item = (Block, IVector);
    type IntoIter = <Zip<<Vec<Block> as IntoIterator>::IntoIter, <Vec<IVector> as IntoIterator>::IntoIter> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {zip(self.blocks, self.shape)}
}
