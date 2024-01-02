use rand::Rng;

use crate::{gamemode::GameMode, piece::{shape::Shapes, Piece}, block::BlockType, blocktypes::standard::StandardType};

#[derive(Debug)]
pub struct StandardMode {
    shapes: Shapes,
    standard_type: Box<dyn BlockType>,
}

impl StandardMode {
    pub fn new() -> Self {
        Self {
            shapes: Shapes::new(),
            standard_type: Box::new(StandardType{}),
        }
    }
}

impl<R: Rng> GameMode<R> for StandardMode {
    fn initialize(&mut self) {
        self.shapes.gen_until(4);
    }
    
    fn next_piece(&self, rng: &mut R) -> Piece {
        Piece::uniform(self.standard_type.as_ref(), self.shapes.random(4, rng))
    }
    
    // fn next_shape(&self, rng: &mut R) -> Shape {
    //     self.shapes.random(4, rng)
    // }

    // fn next_block_type(&self, rng: &mut R) -> &dyn BlockType {
    //     self.standard_type.as_ref()
    // }
}