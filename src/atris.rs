pub mod block;
pub mod figure;
pub mod grid;

use figure::shape::Shapes;
use rand::{rngs::ThreadRng, thread_rng};

pub struct Atris {
    pub rng: ThreadRng,
    pub shapes: Shapes,
}

impl Atris {
    pub fn new() -> Self {
        Self {
            rng: thread_rng(),
            shapes: Shapes::new(),
        }
    }
}