pub mod figure;
pub mod block;

// use figure::shape::Shapes;
use rand::rngs::ThreadRng;

pub struct Atris {
    pub rng: ThreadRng,
    // pub shapes: Result<Shapes<'static>, String>,
}

impl Atris {
    // pub fn new() -> Self {
    //     let mut atris = Self {
    //         rng: thread_rng(),
    //         shapes: Err(String::from("Not initialized yet")),
    //     };
    //     atris.shapes = Ok(Shapes::new(&atris.rng));
    //     atris
    // }
}