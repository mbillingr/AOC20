use common::input::Input;
use std::collections::HashMap;
use std::convert::TryInto;
use std::rc::Rc;

const TILE_SIZE: isize = 10;

fn main() {
    let input = Input::from_file("data/day20-input.txt");

    let tiles = input
        .iter_blocks()
        .filter(|blk| !blk.is_empty())
        .map(Tile::from_str)
        .collect::<Vec<_>>();

    let mut border_matches = HashMap::<Side, Vec<(u8, Tile)>>::new();
    for mut tile in tiles.iter().cloned() {
        for _ in 0..2 {
            for r in 0..4 {
                let tile = tile.rotate(r);
                let side = tile.top_row();
                border_matches.entry(side).or_insert(vec![]).push((r, tile));
            }
            tile = tile.flip();
        }
    }

    let mut tile_matches = HashMap::new();
    for m in border_matches.values() {
        if m.len() == 2 {
            for (_, t) in m {
                *tile_matches.entry(t.id).or_insert(0) += 1;
            }
        }
    }

    let product_of_corner_ids = tile_matches
        .iter()
        .filter(|(_, n)| **n == 4)
        .map(|(t, _)| *t)
        .product::<usize>();

    let mut grid_of_tiles = HashMap::<(isize, isize), Tile>::new();
    let start_tile = tiles[0].clone();
    build_grid((0, 0), start_tile, &border_matches, &mut grid_of_tiles);

    let min_y = grid_of_tiles.keys().map(|(row, _)| *row).min().unwrap();
    let min_x = grid_of_tiles.keys().map(|(_, col)| *col).min().unwrap();
    let max_x = grid_of_tiles.keys().map(|(_, col)| *col).max().unwrap();
    let grid_width = 1 + max_x - min_x;

    let n_tiles = tiles.len() as isize;
    let grid_height = n_tiles / grid_width;

    let image_width = grid_width * (TILE_SIZE - 2);
    let image_height = grid_height * (TILE_SIZE - 2);

    let mut grid = vec![];
    for i in 0..image_height {
        let i_tile = i / (TILE_SIZE - 2) + min_y;
        let row = i % (TILE_SIZE - 2) + 1;
        for j in 0..image_width {
            let j_tile = j / (TILE_SIZE - 2) + min_x;
            let col = j % (TILE_SIZE - 2) + 1;

            /*if /*row == 0 || col == 0 ||*/ row == TILE_SIZE || col == TILE_SIZE {
                grid.push(' ');
            } else {*/
            let tile = &grid_of_tiles[&(i_tile, j_tile)];
            let ch = tile.data.get(row, col);
            grid.push(*ch);
            //}
        }
    }

    let image_grid = Array2D::new(grid, image_width, image_height);

    let template_str = "??????????????????#?#????##????##????###?#??#??#??#??#??#???";
    let template = Array2D::new(template_str.chars().collect::<Vec<_>>(), 20, 3);

    let image_grid = image_grid.rotate(2);  // determined empirically
    println!("{}", image_grid);
    println!("{}", template);

    let mut n_monsters = 0;

    for i in 0..image_grid.height - template.height + 1 {
        for j in 0..image_grid.width - template.width + 1 {
            if find_monster(
                &image_grid.subarray(i, j, template.height, template.width),
                &template,
            ) {
                println!("{} {}", i, j);
                println!(
                    "{:?}",
                    image_grid.subarray(i, j, template.height, template.width)
                );
                n_monsters += 1;
            }
        }
    }
    let n_monster_hashes = template.iter().flatten().filter(|&&ch| ch == '#').count();
    let n_hashes = image_grid.iter().flatten().filter(|&&ch| ch == '#').count();

    println!("Part 1: {}", product_of_corner_ids);
    println!("Part 2: {}", n_hashes - n_monster_hashes * n_monsters);
}

type Side = [char; TILE_SIZE as usize];

#[derive(Debug, Clone, PartialEq)]
struct Tile {
    id: usize,
    data: Array2D<char>,
}

impl Tile {
    fn from_str(s: &str) -> Self {
        let id = s[5..9].parse().unwrap();
        let data = s.lines().skip(1).flat_map(str::chars).collect();
        Tile {
            id,
            data: Array2D::new(data, TILE_SIZE as isize, TILE_SIZE as isize),
        }
    }

    fn rotate(&self, n: u8) -> Self {
        Tile {
            data: self.data.rotate(n),
            ..self.clone()
        }
    }

    fn flip(&self) -> Self {
        Tile {
            data: self.data.flip(),
            ..self.clone()
        }
    }

    fn top_row(&self) -> Side {
        self.data
            .row(0)
            .copied()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    fn align_to(&self, rotated: u8, other_tile: &Tile) -> Tile {
        let other_top = other_tile.rotate(rotated).top_row();
        let mut new_tile = self.clone();
        while new_tile.top_row() != other_top {
            new_tile = new_tile.rotate(1);
        }
        new_tile = new_tile.flip();
        new_tile = new_tile.rotate(3 + 4 - rotated);
        new_tile
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.data.iter() {
            for c in row {
                write!(f, " {}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct Array2D<T> {
    start: isize,
    row_step: isize,
    col_step: isize,
    data: Rc<Vec<T>>,
    width: isize,
    height: isize,
}

impl<T> Clone for Array2D<T> {
    fn clone(&self) -> Self {
        Array2D {
            data: self.data.clone(),
            ..*self
        }
    }
}

impl<T> Array2D<T> {
    pub fn new(data: Vec<T>, width: isize, height: isize) -> Self {
        assert_eq!(data.len(), (width * height) as usize);
        Array2D {
            start: 0,
            row_step: width,
            col_step: 1,
            width,
            height,
            data: Rc::new(data),
        }
    }

    fn iter(&self) -> impl Iterator<Item = impl Iterator<Item = &T> + '_> + '_ {
        (0..self.height).map(move |row_idx| self.row(row_idx))
    }

    fn row(&self, row_idx: isize) -> impl Iterator<Item = &T> + '_ {
        let row_start = self.start + row_idx * self.row_step;
        (0..self.width)
            .map(move |col| row_start + col * self.col_step)
            .map(move |idx| &self.data[idx as usize])
    }

    fn get(&self, row: isize, col: isize) -> &T {
        &self.data[(self.start + row * self.row_step + col * self.col_step) as usize]
    }

    fn rotate(&self, n: u8) -> Self {
        if n % 4 == 0 {
            return self.clone();
        } else {
            Self {
                start: self.start + self.row_step * (self.width - 1) as isize,
                row_step: self.col_step,
                col_step: -self.row_step,
                width: self.height,
                height: self.width,
                ..self.clone()
            }
            .rotate(n - 1)
        }
    }

    fn flip(&self) -> Self {
        Self {
            row_step: self.col_step,
            col_step: self.row_step,
            width: self.height,
            height: self.width,
            ..self.clone()
        }
    }

    fn subarray(&self, row: isize, col: isize, height: isize, width: isize) -> Self {
        Self {
            start: self.start + row * self.row_step + col * self.col_step,
            width,
            height,
            ..self.clone()
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Array2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.iter() {
            for c in row {
                write!(f, "{:?}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Array2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.iter() {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: PartialEq + std::fmt::Debug> PartialEq for Array2D<T> {
    fn eq(&self, rhs: &Self) -> bool {
        if !(self.width == rhs.width && self.height == rhs.height) {
            return false;
        }

        self.iter()
            .flatten()
            .zip(rhs.iter().flatten())
            .all(|(a, b)| a == b)
    }
}

fn build_grid(
    pos: (isize, isize),
    tile: Tile,
    border_matches: &HashMap<Side, Vec<(u8, Tile)>>,
    tile_grid: &mut HashMap<(isize, isize), Tile>,
) {
    if tile_grid.contains_key(&pos) {
        assert_eq!(tile_grid[&pos].id, tile.id);
        return;
    }

    tile_grid.insert(pos, tile.clone());

    let neighbor_pos = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    for r in 0..4 {
        let side = tile.rotate(r).top_row();
        // todo: need to consider flipping?
        let matching_tiles = border_matches[&side]
            .iter()
            .filter(|(_, t)| t.id != tile.id)
            .map(|(_, t)| t.clone())
            .collect::<Vec<_>>();
        match matching_tiles.len() {
            0 => continue,
            1 => {}
            _ => panic!("too many matching tiles"),
        }
        let neighbor_tile = matching_tiles[0].align_to(r, &tile);
        build_grid(
            (
                neighbor_pos[r as usize].0 + pos.0,
                neighbor_pos[r as usize].1 + pos.1,
            ),
            neighbor_tile,
            border_matches,
            tile_grid,
        );
    }
}

fn find_monster(image_part: &Array2D<char>, template: &Array2D<char>) -> bool {
    image_part
        .iter()
        .flatten()
        .copied()
        .zip(template.iter().flatten().copied())
        .all(|x| match x {
            (_, '?') => true,
            (a, b) => a == b,
        })
}
