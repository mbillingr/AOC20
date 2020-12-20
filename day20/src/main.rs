use common::input::Input;
use std::convert::TryInto;
use std::rc::Rc;
use std::collections::HashMap;

const TILE_SIZE: usize = 10;
const IDX_TOP_LEFT: usize = 0;
const IDX_TOP_RIGHT: usize = TILE_SIZE - 1;
const IDX_BOTTOM_RIGHT: usize = TILE_SIZE * TILE_SIZE - 1;
const IDX_BOTTOM_LEFT: usize = TILE_SIZE * (TILE_SIZE - 1);

fn main() {
    let input = Input::from_file("data/day20-input.txt");

    let tiles = input
        .iter_blocks()
        .filter(|blk| !blk.is_empty())
        .map(Tile::from_str)
        //.inspect(|x| println!("{:?}", x))
        .collect::<Vec<_>>();

    let mut matches = HashMap::<Side, Vec<(u8, Tile)>>::new();
    for mut tile in tiles {
        for _ in 0..2 {
            for r in 0..4 {
                let tile = tile.rotate(r);
                let side = tile.top_row();
                matches.entry(side)
                    .or_insert(vec![])
                    .push((r, tile));
            }
            tile = tile.flip();
        }
    }

    let mut tile_matches = HashMap::new();
    for m in matches.values() {
        if m.len() == 2 {
            for (_, t) in m {
                *tile_matches.entry(t.id).or_insert(0) += 1;
            }
        }
    }

    let product_of_corner_ids = tile_matches.iter()
        .filter(|(_, n)| **n == 4)
        .map(|(t, _)| *t)
        .product::<usize>();

    println!("Part 1: {}", product_of_corner_ids);
}

type Side = [char; TILE_SIZE];

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    start: usize,
    row_step: isize,
    col_step: isize,
    flip: bool,
    data: Rc<Vec<char>>,
}

impl Tile {
    fn from_str(s: &str) -> Self {
        let id = s[5..9].parse().unwrap();
        let data = s.lines().skip(1).flat_map(str::chars).collect();
        Tile {
            id,
            data: Rc::new(data),
            start: 0,
            row_step: TILE_SIZE as isize,
            col_step: 1,
            flip: false,
        }
    }

    fn top_row(&self) -> Side {
        let mut tmp = [' '; TILE_SIZE];
        let mut idx = self.start;
        for i in 0..TILE_SIZE {
            tmp[i] = self.data[idx];
            idx = (idx as isize + self.col_step) as usize;
        }
        if self.flip {
            tmp.reverse();
        }
        tmp.try_into().unwrap()
    }

    fn rotate(&self, n: u8) -> Self {
        if n % 4 == 0 {
            return self.clone()
        } else {
            Tile {
                row_step: self.col_step,
                col_step: -self.row_step,
                start: match self.start {
                    IDX_TOP_LEFT => IDX_BOTTOM_LEFT,
                    IDX_BOTTOM_LEFT => IDX_BOTTOM_RIGHT,
                    IDX_BOTTOM_RIGHT => IDX_TOP_RIGHT,
                    IDX_TOP_RIGHT => IDX_TOP_LEFT,
                    _ => unreachable!(),
                },
                ..self.clone()
            }.rotate(n-1)
        }
    }

    fn flip(&self)-> Self {
        Tile {
        flip: !self.flip,
        ..self.clone()
        }
    }
}