use common::input::Input;
use std::collections::HashSet;

fn main() {
    let input = Input::from_file("data/day06-input.txt");

    let sum_any = input
        .iter_blocks()
        .map(|block| {
            block
                .chars()
                .filter(|ch| ch.is_alphabetic())
                .collect::<HashSet<_>>()
        })
        .map(|set| set.len())
        .sum::<usize>();

    println!("Part 1: {}", sum_any);

    let sum_all = input
        .iter_blocks()
        .map(|block| {
            block
                .lines()
                .map(|line| line.chars().collect::<HashSet<char>>())
        })
        .map(all_intersections)
        .map(Option::unwrap)
        .map(|set| set.len())
        .sum::<usize>();

    println!("Part 2: {}", sum_all);
}

fn all_intersections(mut iter: impl Iterator<Item = HashSet<char>>) -> Option<HashSet<char>> {
    let first = iter.next()?;

    Some(iter.fold(first, |acc, set| acc.intersection(&set).copied().collect()))
}
