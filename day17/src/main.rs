use common::ascii_enum;
use common::input::Input;
use std::collections::{BTreeMap, HashMap, HashSet};

ascii_enum!(Cell = Active('#') | Dormant('.'));

type Coord3d = (isize, isize, isize);
type Grid3d = HashSet<Coord3d>;

type Coord4d = (isize, isize, isize, isize);
type Grid4d = HashSet<Coord4d>;

fn main() {
    let input = Input::from_file("data/day17-input.txt");

    let mut grid3d: Grid3d = input
        .enumerate_grid()
        .map(|(x, y, ch)| ((x as isize, y as isize, 0isize), Cell::from_char(ch)))
        .filter(|&(_, cell)| cell == Cell::Active)
        .map(|(coord, _)| coord)
        .collect();

    for _ in 0..6 {
        grid3d = step_cycle3d(&grid3d);
    }

    println!("Part 1: {}", grid3d.len());

    let mut grid4d: Grid4d = input
        .enumerate_grid()
        .map(|(x, y, ch)| {
            (
                (x as isize, y as isize, 0isize, 0isize),
                Cell::from_char(ch),
            )
        })
        .filter(|&(_, cell)| cell == Cell::Active)
        .map(|(coord, _)| coord)
        .collect();

    for _ in 0..6 {
        grid4d = step_cycle4d(&grid4d);
    }

    println!("Part 2: {}", grid4d.len());
}

fn step_cycle3d(grid: &Grid3d) -> Grid3d {
    let mut cells_to_simulate: HashSet<_> =
        grid.iter().copied().flat_map(cube_positions3d).collect();

    let mut out = Grid3d::new();

    for cell in cells_to_simulate {
        let cell_is_active = grid.contains(&cell);
        let n_active_neighbors = neighbor_positions3d(cell)
            .filter(|c| grid.contains(c))
            .count();

        if cell_is_active {
            if n_active_neighbors == 2 || n_active_neighbors == 3 {
                out.insert(cell);
            }
        } else {
            if n_active_neighbors == 3 {
                out.insert(cell);
            }
        }
    }

    out
}

fn neighbor_positions3d(center: Coord3d) -> impl Iterator<Item = Coord3d> {
    cube_positions3d(center).filter(move |&pos| pos != center)
}

fn cube_positions3d((x0, y0, z0): Coord3d) -> impl Iterator<Item = Coord3d> {
    (-1..=1)
        .flat_map(|x| (-1..=1).flat_map(move |y| (-1..=1).map(move |z| (x, y, z))))
        .map(move |(x, y, z)| (x + x0, y + y0, z + z0))
}

fn step_cycle4d(grid: &Grid4d) -> Grid4d {
    let mut cells_to_simulate: HashSet<_> =
        grid.iter().copied().flat_map(cube_positions4d).collect();

    let mut out = Grid4d::new();

    for cell in cells_to_simulate {
        let cell_is_active = grid.contains(&cell);
        let n_active_neighbors = neighbor_positions4d(cell)
            .filter(|c| grid.contains(c))
            .count();

        if cell_is_active {
            if n_active_neighbors == 2 || n_active_neighbors == 3 {
                out.insert(cell);
            }
        } else {
            if n_active_neighbors == 3 {
                out.insert(cell);
            }
        }
    }

    out
}

fn neighbor_positions4d(center: Coord4d) -> impl Iterator<Item = Coord4d> {
    cube_positions4d(center).filter(move |&pos| pos != center)
}

fn cube_positions4d((x0, y0, z0, w0): Coord4d) -> impl Iterator<Item = Coord4d> {
    (-1..=1)
        .flat_map(|x| {
            (-1..=1)
                .flat_map(move |y| (-1..=1).flat_map(move |z| (-1..=1).map(move |w| (x, y, z, w))))
        })
        .map(move |(x, y, z, w)| (x + x0, y + y0, z + z0, w + w0))
}
