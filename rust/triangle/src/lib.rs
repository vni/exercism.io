// It's not a perfect solution
// A better one should compare point to each other
// using EPSILON for float types.


use std::ops::{Add, Sub};
use std::cmp::{PartialEq, PartialOrd};

pub struct Triangle<T: Add + Sub + PartialEq + PartialOrd>(T, T, T);

impl<T: Add<T, Output = T> + Sub + PartialEq + PartialOrd + Default + Copy> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if sides.iter().any(|&side| side == T::default()) {
            return None;
        }

        // any 2 sides >= 3rd side
        let a = sides[0];
        let b = sides[1];
        let c = sides[2];
        if (a + b < c) || (a + c < b) || (b + c < a) {
            return None;
        }
        Some(Triangle(a, b, c))
    }

    pub fn is_equilateral(&self) -> bool {
        self.0 == self.1 && self.1 == self.2
    }

    pub fn is_scalene(&self) -> bool {
        self.0 != self.1 && self.1 != self.2 && self.2 != self.0
    }

    pub fn is_isosceles(&self) -> bool {
        self.0 == self.1 || self.1 == self.2 || self.2 == self.0
    }

    pub fn is_degenerate(&self) -> bool {
        self.0 + self.1 == self.2 || self.1 + self.2 == self.0 || self.2 + self.0 == self.1
    }
}
