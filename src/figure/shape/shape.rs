use std::iter::zip;
use std::ops::{Add, Sub, Neg};

use crate::figure::algebra::{Vector, IVector};

/// Represents a shape os blocks. For example, a `Figure` must have one, so that its blocks are relatively positioned
/// some specific way.
#[derive(Clone, Debug)]
pub struct Shape {
    positions: Vec<IVector>,
}

impl Shape {
    /// Creates a new `Shape` object, with no points in it.
    pub fn new() -> Self {
        Self { positions: Vec::new() }
    }

    /// Creates a `Shape` object with just the origin point in it.
    pub fn unit() -> Self {
        Self { positions: vec!(Vector(0,0)) }
    }

    /// Checks whether the `Shape` points `Vec` includes some point.
    pub fn contains(&self, pos: &IVector) -> bool {
        self.positions.contains(pos)
    }

    /// Inserts a point to the `Shape` points, only if there isn't another identical point already there.
    /// This methods keeps the `Vec` ordered.
    pub fn add_pos(&mut self, pos: IVector) -> bool {
        match self.positions.binary_search(&pos) {
            Ok(_) => return false, // Si encuentra la posición en la lista, no añade nada
            Err(i) => self.positions.insert(i, pos)
        };
        true
    }

    /// Returns the shape rotated `angle`*90 degrees counter-clockwise.
    pub fn rotated(&self, angle: i32) -> Self {
        let mut s = Self::new();
        for p in self.positions.iter() {
            s.add_pos(p.rotated(angle));
        }
        s
    }

    /// Returns the shape centered (with the origin (0,0) more or less at the center of the shape).
    pub fn centered(&self) -> Self {
        let center = self.center();
        let mut s = Self::new();
        for &p in self.positions.iter() {
            s.add_pos(p - center);
        }
        s
    }

    /// Returns the center point of the `Shape` object.
    pub fn center(&self) -> IVector {
        let bounds = self.bounds();
        bounds.0 + (bounds.1 - bounds.0) / 2
    }

    /// Returns the bounds of the `Shape` object as a two-`Vector<i32>` tuple: the minimum coordinates and the maximum coordinates.
    pub fn bounds(&self) -> (IVector, IVector) {
        let mut bounds = (Vector(0,0), Vector(0,0));
        for p in self.positions.iter() {
            if p.0 < bounds.0.0 {bounds.0.0 = p.0}
            if p.1 < bounds.0.1 {bounds.0.1 = p.1}
            if p.0 > bounds.1.0 {bounds.1.0 = p.0}
            if p.1 > bounds.1.1 {bounds.1.1 = p.1}
        }
        bounds
    }

    /// Returns the number of points in the `Shape`.
    pub fn size(&self) -> usize { self.positions.len() }

    /// Returns an inmutable reference to the inner `Vec` of points.
    pub fn positions(&self) -> &Vec<IVector> { &self.positions }

    /// Returns an inmutable iterator over the inner points.
    pub fn iter(&self) -> std::slice::Iter<'_, IVector> { self.positions.iter() }

    /// Returns a mutable iterator over the inner points.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, IVector> { self.positions.iter_mut() }
}

impl Add<IVector> for Shape {
	type Output = Self;
	/// Sums a `Vector<i32>` to a `Shape`.
	fn add(self, rhs: IVector) -> Self::Output {
        let mut s = Self::new();
        for &p in self.positions.iter() {
            s.add_pos(p + rhs);
        }
        s
	}
}

impl Sub<IVector> for Shape {
	type Output = Self;
	/// Substracts a `Vector<i32>` to a `Shape`.
	fn sub(self, rhs: IVector) -> Self::Output {
        let mut s = Self::new();
        for &p in self.positions.iter() {
            s.add_pos(p - rhs);
        }
        s
	}
}

impl Neg for Shape {
	type Output = Self;
	/// Negates all the points of a `Shape`.
	fn neg(self) -> Self::Output {
        let mut s = Self::new();
        for &p in self.positions.iter() {
            s.add_pos(-p);
        }
        s
	}
}

impl FromIterator<IVector> for Shape {
    /// Constructs a `Shape` object from an iterator of points.
    fn from_iter<I: IntoIterator<Item=IVector>>(iter: I) -> Self {
        let mut s = Shape::new();
        for p in iter {
            s.add_pos(p);
        }
        s
    }
}

impl IntoIterator for Shape {
    type Item = IVector;
    type IntoIter = <Vec<IVector> as IntoIterator>::IntoIter;

    /// Returns an iterator of the `Shape` object's internal `Vec` of points.
    fn into_iter(self) -> Self::IntoIter {self.positions.into_iter()}
}

impl PartialEq for Shape {
    /// Checks whether two `Shape` objects are equivalent, this is, when their points are equally positioned
    /// rotation- and translation-independently.
    fn eq (&self, rhs: &Self) -> bool {
        if self.positions.len() != rhs.positions.len() {
            return false;
        }
        if self.positions.len() == 0 {
            return true;
        }
        'angle: for angle in 0..4 {
            let rotated_rhs;
            if angle == 0 {
                rotated_rhs = rhs.clone();
            } else {
                rotated_rhs = rhs.rotated(angle);
            }
            let d = self.positions[0] - rotated_rhs.positions[0];
            for (&pself, &prhs) in zip(self.positions.iter(), rotated_rhs.positions.iter()) {
                if pself != prhs + d {
                    continue 'angle;
                }
            }
            return true;
        }
        false
    }
}