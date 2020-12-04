use std::borrow::Cow;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct Input {
    data: Cow<'static, str>,
}

impl Input {
    pub fn from_str(data: &'static str) -> Self {
        Input {
            data: Cow::from(data),
        }
    }

    pub fn from_file(filepath: impl AsRef<Path>) -> Self {
        let mut data = String::new();
        File::open(filepath)
            .unwrap()
            .read_to_string(&mut data)
            .unwrap();
        Input { data: data.into() }
    }

    pub fn as_str(&self) -> &str {
        self.data.as_ref()
    }

    pub fn into_string(self) -> String {
        self.data.into_owned()
    }

    pub fn iter_numbers(&self) -> impl Iterator<Item = i64> + '_ {
        self.iter_words().map(|line| line.parse().unwrap())
    }

    pub fn iter_numbers_in_lines(&self) -> impl Iterator<Item = Vec<i64>> + '_ {
        self.iter_lines().map(|line| {
            line.split_whitespace()
                .map(|word| word.parse().unwrap())
                .collect()
        })
    }

    pub fn iter_lines(&self) -> impl Iterator<Item = &str> {
        self.data.lines()
    }

    pub fn iter_words(&self) -> impl Iterator<Item = &str> {
        self.data.split_whitespace()
    }

    pub fn iter_blocks(&self) -> impl Iterator<Item = &str> {
        self.data.split("\n\n")
    }
}
