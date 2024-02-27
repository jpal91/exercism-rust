// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    y: i32,
    x: i32,
    d: Direction
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            y, x, d
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let d = match self.d {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North
        };

        Self {
            y: self.y,
            x: self.x,
            d
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let d = match self.d {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South
        };

        Self {
            y: self.y,
            x: self.x,
            d
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let (y, x) = match self.d {
            Direction::North => (self.y + 1, self.x),
            Direction::East => (self.y, self.x + 1),
            Direction::South => (self.y - 1, self.x),
            Direction::West => (self.y, self.x - 1)
        };

        Self {
            y, x, d: self.d
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut new_bot = Self { y: self.y, x: self.x, d: self.d };

        for i in instructions.chars() {
            match i {
                'R' => new_bot = new_bot.turn_right(),
                'L' => new_bot = new_bot.turn_left(),
                'A' => new_bot = new_bot.advance(),
                _ => unreachable!()
            };
        };

        new_bot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
