use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    File::open("data/day01-input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let entries = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&entries));
}

fn part1(entries: &[i32]) -> i32 {
    for first in entries.iter() {
        for second in entries.iter() {
            if first + second == 2020 {
                return first * second
            }
        }
    }
    panic!("Part 1: No solution")
}