use crate::algebra::IVector;
use crate::piece::Piece;

#[derive(Debug)]
pub struct Figure<'bt> {
    piece: Piece<'bt>,
    pub position: IVector,
}

impl<'bt> Figure<'bt> {
    // /// Constructs a new Figure from a (borrowed) `BlockType` and a `Shape`. All the blocks will have the same type.
    // pub fn uniform(t: &'bt dyn BlockType, shape: Shape) -> Self {
    //     let mut blocks = Vec::new();
    //     blocks.resize(shape.positions().len(), Block::new(t));
    //     Self {
    //         piece: Piece,
    //         position: Vector::default(),
    //     }
    // }

    pub fn piece(&self) -> &Piece { &self.piece }

    // pub fn blocks(&self) -> &Vec<Block<'bt>> { &self.blocks }

    // pub fn shape(&self) -> &Shape { &self.shape }
}
