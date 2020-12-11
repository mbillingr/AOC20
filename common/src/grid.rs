use crate::input::Input;
use std::any::type_name;

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    pub fn new(data: Vec<T>, width: usize, height: usize) -> Self {
        Grid {
            data,
            width,
            height,
        }
    }

    pub fn from_vec_vec(data: Vec<Vec<T>>) -> Self {
        let height = data.len();
        let width = data[0].len();
        Self::new(data.into_iter().flatten().collect(), width, height)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        debug_assert!(row < self.height);
        debug_assert!(col < self.width);
        &self.data[row * self.width + col]
    }

    pub fn safe_get(&self, row: isize, col: isize) -> Option<&T> {
        if row >= self.height as isize || col >= self.width as isize || row < 0 || col < 0 {
            return None;
        }
        Some(&self.data[row as usize * self.width + col as usize])
    }

    pub fn set(&mut self, row: usize, col: usize, val: T) {
        debug_assert!(row < self.height);
        debug_assert!(col < self.width);
        self.data[row * self.width + col] = val
    }

    pub fn swap(a: &mut Self, b: &mut Self) {
        std::mem::swap(a, b)
    }

    pub fn flat_iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn trace(
        &self,
        row_start: usize,
        col_start: usize,
        row_step: isize,
        col_step: isize,
    ) -> impl Iterator<Item = &T> {
        assert!(row_step != 0 || col_step != 0);

        let w = self.width as isize;
        let h = self.height as isize;

        //if row_start == 2 && col_start == 2 {println!("{},{}", row_step, col_step)}

        (0..)
            .map(move |t| {
                (
                    row_start as isize + t * row_step,
                    col_start as isize + t * col_step,
                )
            })
            //.inspect(move |x| if row_start == 2 && col_start == 2 {println!("{},{} {:?}", row_step, col_step, x)})
            .take_while(|&(r, c)| r >= 0 && c >= 0)
            .take_while(move |&(r, c)| r < h && c < w)
            .map(|(r, c)| (r as usize, c as usize))
            .map(move |(r, c)| self.get(r, c))
    }
}

impl<T: Clone> Grid<T> {
    pub fn like(&self, fill: T) -> Self {
        Grid {
            data: vec![fill; self.data.len()],
            width: self.width,
            height: self.height,
        }
    }
}

impl<T: From<char>> Grid<T> {
    pub fn from_input(input: Input) -> Self {
        let data: Vec<Vec<_>> = input
            .iter_lines()
            .map(|line| line.chars().map(From::<char>::from).collect())
            .collect();

        Self::from_vec_vec(data)
    }
}

impl<T> std::fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Grid::<{}>[{} x {}]",
            type_name::<T>(),
            self.height,
            self.width
        )
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                write!(f, "{:?}", self.get(row, col))?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
