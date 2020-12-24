use common::input::Input;
use std::collections::HashSet;
use std::iter::once;

fn main() {
    let input = Input::from_file("data/day24-input.txt");

    let mut black_tiles = HashSet::new();

    for pos in input.iter_lines().map(parse_position) {
        if !black_tiles.remove(&pos) {
            black_tiles.insert(pos);
        }
    }

    println!("Part 1: {}", black_tiles.len());

    for _ in 0..100 {
        black_tiles = step(black_tiles);
    }

    println!("Part 2: {}", black_tiles.len());
}

fn parse_position(s: &str) -> HexPos {
    let mut pos = HexPos::origin();
    let mut chars = s.chars();
    while let Some(ch) = chars.next() {
        match ch {
            'e' => pos.x += 1,
            'w' => pos.x -= 1,
            's' => match chars.next().unwrap() {
                'e' => pos.y += 1,
                'w' => {
                    pos.x -= 1;
                    pos.y += 1
                }
                _ => panic!("invalid direction s{}", ch),
            },
            'n' => match chars.next().unwrap() {
                'e' => {
                    pos.x += 1;
                    pos.y -= 1
                }
                'w' => pos.y -= 1,
                _ => panic!("invalid direction n{}", ch),
            },
            _ => panic!("invalid direction {}", ch),
        }
    }
    pos
}

fn step(black_tiles: HashSet<HexPos>) -> HashSet<HexPos> {
    let may_flip: HashSet<_> = black_tiles.iter().flat_map(HexPos::ball).collect();

    may_flip
        .into_iter()
        .filter_map(|tile| {
            let is_black = black_tiles.contains(&tile);
            match (is_black, count_black_neighbors(tile, &black_tiles)) {
                (true, 1) | (true, 2) => Some(tile),
                (false, 2) => Some(tile),
                _ => None,
            }
        })
        .collect()
}

fn count_black_neighbors(tile: HexPos, black_tiles: &HashSet<HexPos>) -> usize {
    tile.neighbors().filter(|n| black_tiles.contains(n)).count()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct HexPos {
    x: i64,
    y: i64,
}

impl HexPos {
    pub fn origin() -> Self {
        HexPos { x: 0, y: 0 }
    }

    fn ball(&self) -> impl Iterator<Item = Self> {
        self.neighbors().chain(once(*self))
    }

    fn neighbors(&self) -> impl Iterator<Item = Self> {
        once(HexPos {
            x: self.x + 1,
            y: self.y,
        })
        .chain(once(HexPos {
            x: self.x,
            y: self.y + 1,
        }))
        .chain(once(HexPos {
            x: self.x - 1,
            y: self.y + 1,
        }))
        .chain(once(HexPos {
            x: self.x - 1,
            y: self.y,
        }))
        .chain(once(HexPos {
            x: self.x,
            y: self.y - 1,
        }))
        .chain(once(HexPos {
            x: self.x + 1,
            y: self.y - 1,
        }))
    }
}
