use common::ascii_enum;
use common::grid::Grid;
use common::input::Input;

ascii_enum! {
    Cell = Open('.') | Tree('#')
}

fn main() {
    let input = Input::from_file("data/day03-input.txt");

    let grid = Grid::from_input(input);
    println!("{:?}", grid);

    let n_trees = count_trees(1, 3, &grid);

    println!("Part 1: {}", n_trees);

    let mut total = 1;
    for (row_step, col_step) in vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)] {
        let n_trees = count_trees(row_step, col_step, &grid);
        total *= n_trees;
    }

    println!("Part 2: {}", total);
}

fn count_trees(row_step: isize, col_step: isize, grid: &Grid<Cell>) -> usize {
    slope(0, 0, row_step, col_step)
        .map(|(row, col)| (row as usize, (col % grid.width as isize) as usize))
        .take_while(|(row, _)| *row < grid.height)
        .filter(|(row, col)| *grid.get(*row, *col) == Cell::Tree)
        .count()
}

fn slope(
    row_start: isize,
    col_start: isize,
    row_step: isize,
    col_step: isize,
) -> impl Iterator<Item = (isize, isize)> {
    let mut row = row_start;
    let mut col = col_start;
    (0..).map(move |_| {
        row += row_step;
        col += col_step;
        (row, col)
    })
}
