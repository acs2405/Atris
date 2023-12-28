use std::iter::zip;

use crate::figure::algebra::{Vector, IVector};

#[derive(Clone, Debug)]
pub struct Shape {
    positions: Vec<IVector>,
}

impl Shape {
    pub fn new() -> Self {
        Self { positions: Vec::new() }
    }

    pub fn unit() -> Self {
        Self { positions: vec!(Vector(0,0)) }
    }

    pub fn contains(&self, pos: &IVector) -> bool {
        self.positions.contains(pos)
    }

    pub fn add_pos(&mut self, pos: IVector) -> bool {
        match self.positions.binary_search(&pos) {
            Ok(_) => return false, // Si encuentra la posición en la lista, no añade nada
            Err(i) => self.positions.insert(i, pos)
        };
        true
    }

    pub fn rotated(&self, angle: i32) -> Self {
        let mut s = Self::new();
        for p in self.positions.iter() {
            s.add_pos(p.rotated(angle));
        }
        s
    }

    pub fn positions(&self) -> &Vec<IVector> {&self.positions}
}

impl FromIterator<IVector> for Shape {
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

    fn into_iter(self) -> Self::IntoIter {self.positions.into_iter()}
}

impl PartialEq for Shape {
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