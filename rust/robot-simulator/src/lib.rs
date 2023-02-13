// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot(i32, i32, Direction);

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self(x, y, d)
    }

    pub fn turn_right(self) -> Self {
        match self.2 {
            Direction::North => Self(self.0, self.1, Direction::East),
            Direction::East => Self(self.0, self.1, Direction::South),
            Direction::South => Self(self.0, self.1, Direction::West),
            Direction::West => Self(self.0, self.1, Direction::North),
        }
    }

    pub fn turn_left(self) -> Self {
        match self.2 {
            Direction::North => Self(self.0, self.1, Direction::West),
            Direction::West => Self(self.0, self.1, Direction::South),
            Direction::South => Self(self.0, self.1, Direction::East),
            Direction::East => Self(self.0, self.1, Direction::North),
        }
    }

    pub fn advance(self) -> Self {
        match self.2 {
            Direction::North => Self(self.0, self.1 + 1, Direction::North),
            Direction::West => Self(self.0 - 1, self.1, Direction::West),
            Direction::South => Self(self.0, self.1 - 1, Direction::South),
            Direction::East => Self(self.0 + 1, self.1, Direction::East),
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut rob = self;
        for inst in instructions.chars() {
            match inst {
                'L' => rob = rob.turn_left(),
                'R' => rob = rob.turn_right(),
                'A' => rob = rob.advance(),
                _ => unreachable!(),
            }
        }
        rob
    }

    pub fn position(&self) -> (i32, i32) {
        (self.0, self.1)
    }

    pub fn direction(&self) -> &Direction {
        &self.2
    }
}
