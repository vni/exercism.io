pub struct Triangle(u64, u64, u64);

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides.iter().any(|&side| side == 0) {
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
