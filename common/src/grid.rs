use crate::input::Input;
use std::any::type_name;

#[derive(Clone)]
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
