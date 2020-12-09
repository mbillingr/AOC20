#[macro_use]
mod ascii_enum;

pub mod grid;
pub mod input;

pub use ansi_term;
pub use hex;
pub use itertools;
pub use regex;
pub use terminal_size;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
