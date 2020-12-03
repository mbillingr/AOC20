use common::input::Input;

fn main() {
    let input = Input::from_file("data/day03-input.txt");

    let grid = Grid::from_input(input);

    let n_trees = count_trees(1, 3, &grid);

    println!("Part 1: {}", n_trees);

    let mut total = 1;
    for (row_step, col_step) in vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)] {
        let n_trees = count_trees(row_step, col_step, &grid);
        total *= n_trees;
    }

    println!("Part 2: {}", total);
}

fn count_trees(row_step: isize, col_step: isize, grid: &Grid) -> usize {
    slope(0, 0, row_step, col_step)
        .map(|(row, col)| (row as usize, col as usize))
        .take_while(|(row, _)| *row < grid.height)
        .filter(|(row, col)| grid.get(*row, *col) == Cell::Tree)
        .count()
}

fn slope(row_start: isize, col_start: isize, row_step: isize, col_step: isize) -> impl Iterator<Item=(isize, isize)> {
    let mut row = row_start;
    let mut col = col_start;
    (0..).map(move |_|{
        row += row_step;
        col += col_step;
        (row, col)
    })
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cell {
    Open,
    Tree
}

impl Cell {
    fn from_char(ch: char) -> Self {
        match ch {
            '.' => Cell::Open,
            '#' => Cell::Tree,
            _ => panic!("Unknown cell {:?}", ch)
        }
    }

    fn to_char(&self) -> char {
        match self {
            Cell::Open => '.',
            Cell::Tree => '#',
        }
    }
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}


#[derive(Debug)]
struct Grid {
    field: Vec<Vec<Cell>>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    fn from_input(input: Input) -> Self {
        let field: Vec<Vec<_>> = input.iter_lines()
            .map(|line| line.chars().map(Cell::from_char).collect())
            .collect();

        let height = field.len();
        let width = field[0].len();

        Grid {
            field,
            width,
            height,
        }
    }

    fn get(&self, row: usize, col: usize) -> Cell {
        self.field[row][col % self.width]
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.field {
            for cell in row {
                write!(f, "{:?}", cell)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}