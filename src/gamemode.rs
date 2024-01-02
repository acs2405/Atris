use std::fmt::Debug;

use rand::Rng;

use crate::piece::Piece;

pub trait GameMode<R: Rng>: Debug {
    fn initialize(&mut self) {}
    fn next_piece(&self, rng: &mut R) -> Piece;
    // fn on_placed(&self);
}