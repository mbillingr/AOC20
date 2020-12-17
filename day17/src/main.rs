use common::input::Input;
use std::collections::HashSet;
use std::hash::Hash;

type Coord3d = (isize, isize, isize);
type Coord4d = (isize, isize, isize, isize);
type Grid<C> = HashSet<C>;

fn main() {
    let input = Input::from_file("data/day17-input.txt");
    println!("Part 1: {}", solve::<Coord3d>(&input));
    println!("Part 2: {}", solve::<Coord4d>(&input));
}

fn solve<C>(input: &Input) -> usize
where
    C: GridPos + Eq + Hash,
{
    let mut grid: Grid<C> = parse_input(&input);
    for _ in 0..6 {
        grid = step_cycle(&grid);
    }
    grid.len()
}

fn parse_input<C>(input: &Input) -> Grid<C>
where
    C: GridPos + Eq + Hash,
{
    input
        .enumerate_grid()
        .map(|(x, y, ch)| (C::init_2d(x as isize, y as isize), ch))
        .filter(|&(_, cell)| cell == '#')
        .map(|(coord, _)| coord)
        .collect()
}

fn step_cycle<C>(grid: &Grid<C>) -> Grid<C>
where
    C: GridPos + Eq + Hash,
{
    let cells_to_simulate: HashSet<_> =
        grid.iter().copied().flat_map(C::surrounding_cube).collect();

    let mut out = Grid::new();

    for cell in cells_to_simulate {
        let cell_is_active = grid.contains(&cell);
        let n_active_neighbors = cell.neighbors().filter(|c| grid.contains(c)).count();

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

trait GridPos: 'static + Copy + PartialEq {
    fn init_2d(x: isize, y: isize) -> Self;
    fn surrounding_cube(self) -> Box<dyn Iterator<Item = Self>>;

    fn neighbors(self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(Self::surrounding_cube(self).filter(move |&pos| pos != self))
    }
}

impl GridPos for Coord3d {
    fn init_2d(x: isize, y: isize) -> Self {
        (x, y, 0)
    }

    fn surrounding_cube(self) -> Box<dyn Iterator<Item = Self>> {
        let (x0, y0, z0) = self;
        Box::new(
            (-1..=1)
                .flat_map(|x| (-1..=1).flat_map(move |y| (-1..=1).map(move |z| (x, y, z))))
                .map(move |(x, y, z)| (x + x0, y + y0, z + z0)),
        )
    }
}

impl GridPos for Coord4d {
    fn init_2d(x: isize, y: isize) -> Self {
        (x, y, 0, 0)
    }

    fn surrounding_cube(self) -> Box<dyn Iterator<Item = Self>> {
        let (x0, y0, z0, w0) = self;
        Box::new(
            Coord3d::surrounding_cube((x0, y0, z0))
                .flat_map(move |(x, y, z)| (-1..=1).map(move |w| (x, y, z, w + w0))),
        )
    }
}
