use common::ascii_enum;
use common::input::Input;

fn main() {
    let input = Input::from_file("data/day12-input.txt");

    let actions = input
        .iter_lines()
        .map(Direction::from_str)
        .inspect(|x| println!("{:?}", x))
        .collect::<Vec<_>>();

    let mut pos = Position::new();

    for &action in &actions {
        pos = pos.apply_dir(action);
    }


    println!("Hello, world!");
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
        Direction {
            kind, arg
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    North, South, East, West
}

impl Dir {
    fn turn(self, steps: i64) -> Self {
        unimplemented!()
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
            x: 0, y: 0, heading: Dir::East,
        }
    }

    pub fn apply_dir(self, dir: Direction) -> Self {
        use DirType::*;
        match dir {
            Direction{kind: Forward, arg} => self.forward(arg),
            Direction{kind: Left, arg} => Position{heading: self.heading.turn(-arg/90), ..self},
            _ => unimplemented!("{:?}", dir)
        }
    }

    pub fn forward(self, n: i64) -> Self {
        let Position{mut x, mut y, heading} = self;
        match heading {
            Dir::North => y += n,
            Dir::South => y -= n,
            Dir::East => x += n,
            Dir::West => x -= n,
        }
        Position {
            x, y, heading
        }
    }
}