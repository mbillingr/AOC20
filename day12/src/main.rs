use common::ascii_enum;
use common::input::Input;

fn main() {
    let input = Input::from_file("data/day12-input.txt");

    let actions = input
        .iter_lines()
        .map(Direction::from_str)
        .collect::<Vec<_>>();

    let mut pos = Position::new();

    for &action in &actions {
        pos = pos.apply_dir(action);
    }

    println!("Part 1: {}", pos.manhattan());

    let mut pos = Waypoint::new();

    for &action in &actions {
        pos = pos.apply_dir(action);
    }

    println!("Part 2: {}", pos.manhattan());
}

ascii_enum!(
    DirType =
        North('N') | South('S') | East('E') | West('W') | Left('L') | Right('R') | Forward('F')
);

#[derive(Debug, Copy, Clone, PartialEq)]
struct Direction {
    kind: DirType,
    arg: i64,
}

impl Direction {
    fn from_str(s: &str) -> Self {
        let ch = s.chars().next().unwrap();
        let kind = DirType::from_char(ch);

        let arg = s[1..].parse().unwrap();
        Direction { kind, arg }
    }
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn turn(self, steps: i64) -> Self {
        if steps % 4 == 0 {
            return self;
        }
        match (self, steps > 0) {
            (Dir::North, true) => Dir::East.turn(steps - 1),
            (Dir::East, true) => Dir::South.turn(steps - 1),
            (Dir::South, true) => Dir::West.turn(steps - 1),
            (Dir::West, true) => Dir::North.turn(steps - 1),
            (Dir::North, false) => Dir::West.turn(steps + 1),
            (Dir::East, false) => Dir::North.turn(steps + 1),
            (Dir::South, false) => Dir::East.turn(steps + 1),
            (Dir::West, false) => Dir::South.turn(steps + 1),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Position {
    x: i64,
    y: i64,
    heading: Dir,
}

impl Position {
    pub fn new() -> Self {
        Position {
            x: 0,
            y: 0,
            heading: Dir::East,
        }
    }

    pub fn apply_dir(self, dir: Direction) -> Self {
        use DirType::*;
        match dir {
            Direction { kind: Forward, arg } => self.forward(arg),
            Direction { kind: Left, arg } => Position {
                heading: self.heading.turn(-arg / 90),
                ..self
            },
            Direction { kind: Right, arg } => Position {
                heading: self.heading.turn(arg / 90),
                ..self
            },
            Direction { kind: North, arg } => Position {
                y: self.y + arg,
                ..self
            },
            Direction { kind: South, arg } => Position {
                y: self.y - arg,
                ..self
            },
            Direction { kind: East, arg } => Position {
                x: self.x + arg,
                ..self
            },
            Direction { kind: West, arg } => Position {
                x: self.x - arg,
                ..self
            },
        }
    }

    pub fn forward(self, n: i64) -> Self {
        let Position {
            mut x,
            mut y,
            heading,
        } = self;
        match heading {
            Dir::North => y += n,
            Dir::South => y -= n,
            Dir::East => x += n,
            Dir::West => x -= n,
        }
        Position { x, y, heading }
    }

    pub fn manhattan(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug, Copy, Clone)]
struct Waypoint {
    x: i64,
    y: i64,
    wx: i64,
    wy: i64,
}

impl Waypoint {
    pub fn new() -> Self {
        Waypoint {
            x: 0,
            y: 0,
            wx: 10,
            wy: 1,
        }
    }

    pub fn apply_dir(self, dir: Direction) -> Self {
        use DirType::*;
        match dir {
            Direction { kind: Forward, arg } => self.forward(arg),
            Direction { kind: Left, arg } => self.turn_left(arg / 90),
            Direction { kind: Right, arg } => self.turn_right(arg / 90),
            Direction { kind: North, arg } => Waypoint {
                wy: self.wy + arg,
                ..self
            },
            Direction { kind: South, arg } => Waypoint {
                wy: self.wy - arg,
                ..self
            },
            Direction { kind: East, arg } => Waypoint {
                wx: self.wx + arg,
                ..self
            },
            Direction { kind: West, arg } => Waypoint {
                wx: self.wx - arg,
                ..self
            },
        }
    }

    pub fn forward(mut self, n: i64) -> Self {
        if n == 0 {
            return self;
        }

        self.x += self.wx;
        self.y += self.wy;

        self.forward(n - 1)
    }

    fn turn_left(mut self, n: i64) -> Self {
        if n == 0 {
            return self;
        }

        std::mem::swap(&mut self.wx, &mut self.wy);
        self.wx = -self.wx;

        self.turn_left(n - 1)
    }

    fn turn_right(mut self, n: i64) -> Self {
        if n == 0 {
            return self;
        }

        std::mem::swap(&mut self.wx, &mut self.wy);
        self.wy = -self.wy;

        self.turn_right(n - 1)
    }

    pub fn manhattan(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}
