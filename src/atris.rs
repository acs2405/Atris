pub mod figure;
pub mod block;

use figure::shape::Shapes;

pub struct Atris {
    pub shapes: Shapes,
}

impl Atris {
    pub fn new() -> Self {
        Self { shapes: Shapes::new() }
    }
}