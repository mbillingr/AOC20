use common::ascii_enum;
use common::grid::Grid;
use common::input::Input;

ascii_enum! {
    Cell = Floor('.') | Free('L') | Full('#')
}

fn main() {
    /*let input = Input::from_str(
            "L.LL.LL.LL
    LLLLLLL.LL
    L.L.L..L..
    LLLL.LL.LL
    L.LL.LL.LL
    L.LLLLL.LL
    ..L.L.....
    LLLLLLLLLL
    L.LLLLLL.L
    L.LLLLL.LL",
        );*/

    let input = Input::from_file("data/day11-input.txt");

    let start = Grid::<Cell>::from_input(input);
    println!("{:?}", start);

    let mut front = start.clone();
    let mut back = front.like(Cell::Floor);
    while front != back {
        simstep1(&front, &mut back);
        Grid::swap(&mut front, &mut back);
    }

    let n_occupied = front.flat_iter().filter(|&&c| c == Cell::Full).count();
    println!("Part 1: {}", n_occupied);

    let mut front = start.clone();
    let mut back = front.like(Cell::Floor);
    while front != back {
        simstep2(&front, &mut back);
        Grid::swap(&mut front, &mut back);
    }

    let n_occupied = front.flat_iter().filter(|&&c| c == Cell::Full).count();
    println!("Part 2: {}", n_occupied);
}

fn simstep1(front: &Grid<Cell>, back: &mut Grid<Cell>) {
    for i in 0..front.height {
        for j in 0..front.width {
            let n = count_occupied_neighbors(i, j, front);
            let out = match *front.get(i, j) {
                Cell::Free if n == 0 => Cell::Full,
                Cell::Full if n >= 4 => Cell::Free,
                other => other,
            };
            back.set(i, j, out);
        }
    }
}

fn count_occupied_neighbors(i: usize, j: usize, grid: &Grid<Cell>) -> usize {
    let i = i as isize;
    let j = j as isize;

    let neighbors = [
        grid.safe_get(i - 1, j - 1),
        grid.safe_get(i - 1, j),
        grid.safe_get(i - 1, j + 1),
        grid.safe_get(i, j - 1),
        grid.safe_get(i, j + 1),
        grid.safe_get(i + 1, j - 1),
        grid.safe_get(i + 1, j),
        grid.safe_get(i + 1, j + 1),
    ];

    neighbors
        .iter()
        .filter(|&&c| c == Some(&Cell::Full))
        .count()
}

fn simstep2(front: &Grid<Cell>, back: &mut Grid<Cell>) {
    for i in 0..front.height {
        for j in 0..front.width {
            let n = count_visibly_occupied(i, j, front);
            let out = match *front.get(i, j) {
                Cell::Free if n == 0 => Cell::Full,
                Cell::Full if n >= 5 => Cell::Free,
                other => other,
            };
            back.set(i, j, out);
        }
    }
}

fn count_visibly_occupied(i: usize, j: usize, grid: &Grid<Cell>) -> usize {
    let vis = [
        trace(i, j, -1, -1, &grid),
        trace(i, j, -1, 0, &grid),
        trace(i, j, -1, 1, &grid),
        trace(i, j, 0, -1, &grid),
        trace(i, j, 0, 1, &grid),
        trace(i, j, 1, -1, &grid),
        trace(i, j, 1, 0, &grid),
        trace(i, j, 1, 1, &grid),
    ];
    vis.iter().filter(|&&c| c == Some(Cell::Full)).count()
}

fn trace(i: usize, j: usize, di: isize, dj: isize, grid: &Grid<Cell>) -> Option<Cell> {
    grid.trace(i, j, di, dj)
        .skip(1)
        .filter(|&&c| c != Cell::Floor)
        .next()
        .copied()
}
