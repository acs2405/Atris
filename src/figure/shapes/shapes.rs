use crate::figure::algebra::Vector;
use super::Shape;

/// An instance of Shapes can be used to generate all possible shapes of a figure with `n` blocks.
/// This is useful to generate figures with random shapes knowing their `size`.
#[derive(Debug)]
pub struct Shapes {
    size: usize,
    shapess: Vec<Vec<Shape>>,
}

impl Shapes {
    /// Creates a list with the list of all possible shapes of `size` 1 (just one shape with just one point: origin).
    /// `Self::size` is first set to 1.
    /// 
    /// ```
    /// use atris::figure::shapes::{Shape,Shapes};
    /// use atris::figure::algebra::Vector;
    /// 
    /// let mut s = Shapes::new();
    /// assert_eq!(s.max_size(), 1);
    /// assert_eq!(s.shapes(1), &vec!(Shape::unit()));
    /// ```
    pub fn new() -> Self {
        Self { size: 1, shapess: vec!(vec!(Shape::unit())) }
    }

    /// Returns current maximum `size` generated so far.
    /// 
    /// ```
    /// use atris::figure::shapes::{Shape,Shapes};
    /// use atris::figure::algebra::Vector;
    /// 
    /// let mut s = Shapes::new();
    /// assert_eq!(s.max_size(), 1);
    /// s.gen_until(3);
    /// assert_eq!(s.max_size(), 3);
    /// ```
    pub fn max_size(&self) -> usize {
        self.size
    }

    /// Returns the list of all possible shapes for some `size`.
    /// Needs to have run `Self::gen_until(n)` before, where `n` >= `size`, or this method will panic.
    pub fn shapes(&self, size: usize) -> &Vec<Shape> {
        //self.gen_until(size);
        &self.shapess[size-1]
    }

    /// Generates all the possible shapes from current `Self::size` to `size` contiguous blocks and sets `Self::size` to `size`.
    /// Needed to run `Self::shapes(sz)` for any `sz` between current `Self::size` and `size`
    /// 
    /// ```
    /// use atris::figure::shapes::{Shape,Shapes};
    /// use atris::figure::algebra::Vector;
    /// 
    /// let mut s = Shapes::new();
    /// s.gen_until(4);
    /// assert_eq!(s.shapes(1), &vec!(Shape::unit()));
    /// assert_eq!(s.shapes(2), &vec!(Shape::from_iter([Vector(0,0), Vector(0,1)])));
    /// assert_eq!(s.shapes(3), &vec!(
    ///     Shape::from_iter([Vector(0,0), Vector(0,1), Vector(1,0)]), // Angle
    ///     Shape::from_iter([Vector(-1,0), Vector(0,0), Vector(1,0)]) // Straight
    /// ));
    /// assert_eq!(s.shapes(4).len(), 7); // All 7 possible
    /// ```
    pub fn gen_until(&mut self, size: usize) {
        for _size in self.size..size {
            self.gen_next();
        }
    }

    /// Returns the last list of all possible shapes (for some `size`) generated
    fn last_shapes(&self) -> &Vec<Shape> {
        self.shapess.last().unwrap()
    }

    /// Generates all the possible shapes of `n+1` contiguous blocks having all the possible shapes of `n` contiguous blocks.
    /// Also, increments `Self::size` by 1.
    fn gen_next(&mut self) {
        let mut shapes = Vec::new();
        for last_shape in self.last_shapes().iter() {
            for p in last_shape.clone() {
                for dir in vec!(Vector(1,0), Vector(0,1), Vector(-1,0), Vector(0,-1)) {
                    let p2 = p + dir;
                    if !last_shape.contains(&p2) {
                        let mut shape = last_shape.clone();
                        shape.add_pos(p2);
                        if !shapes.contains(&shape) {
                            shapes.push(shape);
                        }
                    }
                }
            }
        }
        self.size += 1;
        self.shapess.push(shapes);
    }
}