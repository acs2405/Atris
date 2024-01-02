pub mod algebra;
pub mod block;
pub mod piece;
pub mod grid;
pub mod gamemode;
pub mod gridsession;
pub mod atrissession;

pub mod blocktypes;
pub mod gamemodes;

use rand::{rngs::ThreadRng, thread_rng};

pub struct Atris {
    pub rng: ThreadRng,
}

impl Atris {
    pub fn new() -> Self {
        Self {
            rng: thread_rng(),
        }
    }
}