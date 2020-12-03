#[macro_use]
mod ascii_enum;

pub mod grid;
pub mod input;

pub use itertools;
pub use regex;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
