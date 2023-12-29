use std::ops::{Add, Sub, Neg, Mul, Div};
use std::cmp::Ordering;

pub type IVector = Vector<i32>;

/// 2D generic vector (horizontal, vertical). It's implemented just for T=i32
#[derive(Default, Hash, Copy, Clone, Debug, PartialEq)]
pub struct Vector<T>(pub T, pub T);

impl Vector<i32> {
	/// Rotates a vector some angle, where each unit a quarter of a counter-clockwise rotation.
	///
	/// ```
	/// use atris::figure::algebra::Vector;
	/// let v = Vector(5, 9);
	/// assert_eq!(v.rotated(1), Vector(-9, 5));
	/// assert_eq!(v.rotated(2), Vector(-5, -9));
	/// assert_eq!(v.rotated(3), Vector(9, -5));
	/// assert_eq!(v.rotated(4), v);
	/// assert_eq!(v.rotated(-1), Vector(9, -5));
	/// ```
	pub fn rotated(&self, angle: i32) -> Vector<i32> {
		let angle = angle.rem_euclid(4);
		let half = -(angle & 0b0010) + 1; // if angle < 2 then half=1, else half=-1
		//(self.0, self.1) = 
		match angle {
			0|2 => Vector( half*self.0, half*self.1),
			1|3 => Vector(-half*self.1, half*self.0),
			_ => panic!()
		}
	} // rotated o rotate?
}

impl<T> PartialOrd for Vector<T>
where T: PartialOrd {
	/// Partially compares two vectors.
	///
	/// ```
	/// use atris::figure::algebra::Vector;
	/// use std::cmp::Ordering;
	/// let v1 = Vector(5, 3);
	/// let v2 = Vector(5, 9);
	/// let v3a = Vector(-1, 1);
	/// let v3b = Vector(-1, 1);
	/// assert_eq!(v1.partial_cmp(&v2), Some(Ordering::Less));
	/// assert_eq!(v1 < v2, true);
	/// assert_eq!(v1.partial_cmp(&v3a), Some(Ordering::Greater));
	/// assert_eq!(v1 <= v3a, false);
	/// assert_eq!(v2.partial_cmp(&v3a), Some(Ordering::Greater));
	/// assert_eq!(v2 > v3a, true);
	/// assert_eq!(v3a.partial_cmp(&v3b), Some(Ordering::Equal));
	/// assert_eq!(v3a == v3b, true);
	/// ```
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
		let cmp = self.0.partial_cmp(&rhs.0)?;
		if cmp.is_ne() {Some(cmp)} else {self.1.partial_cmp(&rhs.1)}
	}
}

impl Eq for Vector<i32> {}

impl Ord for Vector<i32> {
	/// Compares two vectors.
	///
	/// ```
	/// use atris::figure::algebra::Vector;
	/// use std::cmp::Ordering;
	/// let v1 = Vector(5, 3);
	/// let v2 = Vector(5, 9);
	/// let v3a = Vector(-1, 1);
	/// let v3b = Vector(-1, 1);
	/// assert_eq!(v1.cmp(&v2), Ordering::Less);
	/// assert_eq!(v1 < v2, true);
	/// assert_eq!(v1.cmp(&v3a), Ordering::Greater);
	/// assert_eq!(v1 <= v3a, false);
	/// assert_eq!(v2.cmp(&v3a), Ordering::Greater);
	/// assert_eq!(v2 > v3a, true);
	/// assert_eq!(v3a.cmp(&v3b), Ordering::Equal);
	/// assert_eq!(v3a == v3b, true);
	/// ```
	fn cmp(&self, rhs: &Self) -> Ordering {
		self.partial_cmp(rhs).unwrap()
	}
}

impl<T> Add<Self> for Vector<T>
	where T : Add<Output=T> {
	type Output = Self;
	/// Sums two vectors.
	///
	/// ```
	/// use atris::figure::algebra::Vector;
	/// let v1 = Vector(0, 9);
	/// let v2 = Vector(5, -3);
	/// let v3 = Vector(2, 2);
	/// assert_eq!(v1 + v2, Vector(5, 6));
	/// assert_eq!(v2 + v1, Vector(5, 6));
	/// assert_eq!(v1 + v3, Vector(2, 11));
	/// assert_eq!(v2 + v3, Vector(7, -1));
	/// assert_eq!(v1 + v2 + v3, Vector(7, 8));
	/// ```
	fn add(self, rhs: Self) -> Self::Output {
		let x = self.0 + rhs.0;
		let y = self.1 + rhs.1;
		Self(x, y)
	}
}

impl<T> Sub<Self> for Vector<T>
	where T : Sub<Output=T> {
	type Output = Self;
	/// Substracts two vectors.
	///
	/// ```
	/// use atris::figure::algebra::Vector;
	/// let v1 = Vector(0, 9);
	/// let v2 = Vector(5, -3);
	/// let v3 = Vector(2, 2);
	/// assert_eq!(v1 - v2, Vector(-5, 12));
	/// assert_eq!(v2 - v1, Vector(5, -12));
	/// assert_eq!(v1 - v3, Vector(-2, 7));
	/// assert_eq!(v2 - v3, Vector(3, -5));
	/// assert_eq!(v1 - v2 - v3, Vector(-7, 10));
	/// ```
	fn sub(self, rhs: Self) -> Self::Output {
		let x = self.0 - rhs.0;
		let y = self.1 - rhs.1;
		Self(x, y)
	}
}

impl<T> Neg for Vector<T>
	where T : Neg<Output=T> {
	type Output = Self;
	/// Negates a vector.
	///
	/// ```
	/// use atris::figure::algebra::Vector;
	/// let v1 = Vector(0, 9);
	/// let v2 = Vector(5, -3);
	/// assert_eq!(-v1, Vector(0, -9));
	/// assert_eq!(-v2, Vector(-5, 3));
	/// assert_eq!(-(-v1), v1);
	/// assert_eq!(-(-v2), v2);
	/// ```
	fn neg(self) -> Self::Output {
		Self(-self.0, -self.1)
	}
}

impl<T> Mul<T> for Vector<T>
	where T : Mul<Output=T> + Copy {
	type Output = Self;
	/// Multiplies a vector by a number.
	fn mul(self, rhs: T) -> Self::Output {
		let x = self.0 * rhs;
		let y = self.1 * rhs;
		Self(x, y)
	}
}

impl<T> Div<T> for Vector<T>
	where T : Div<Output=T> + Copy {
	type Output = Self;
	/// Divides a vector by a number.
	fn div(self, rhs: T) -> Self::Output {
		let x = self.0 / rhs;
		let y = self.1 / rhs;
		Self(x, y)
	}
}
