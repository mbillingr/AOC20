use common::input::Input;
use common::itertools::Itertools;

/*
I originally wrote nested for loops to solve parts 1 and 2.
Later, I refactored them to use more generic find_pair and find_triple functions.
I further rewrote the solutions with `Itertools::combinations`, which is shortest but visibly slower
in Debug builds. Release builds don't optimize the temporary allocations away (as far I can tell
from inspecting the assembly), so I leave both versions in for future reference.
*/

fn main() {
    let input = Input::from_file("data/day01-input.txt");

    let items: Vec<_> = input.iter_numbers().collect();

    println!("Part 1: {}", part1(&items));
    println!("Part 2: {}", part2(&items));
}

fn part1(items: &[i64]) -> i64 {
    // use slower Itertools::combinations implementation. Simply call with `k=3` to solve Part 2.
    find_combination(items, 2).expect("Part 1: No solution")
}

fn find_combination(items: &[i64], k: usize) -> Option<i64> {
    items
        .iter()
        .combinations(k)
        .filter(|x| x.into_iter().copied().sum::<i64>() == 2020)
        .map(|x| x.into_iter().product())
        .next()
}

fn part2(items: &[i64]) -> i64 {
    // use faster explicit looping.
    find_triple(items, |first, second, third| first + second + third == 2020)
        .map(|(first, second, third)| first * second * third)
        .expect("Part 2: No solution")
}

fn find_pair<'a, T>(
    items: impl Copy + IntoIterator<Item = &'a T>,
    predicate: impl Fn(&T, &T) -> bool,
) -> Option<(&'a T, &'a T)> {
    for first in items.into_iter() {
        for second in items.into_iter() {
            if predicate(first, second) {
                return Some((first, second));
            }
        }
    }
    None
}

fn find_triple<'a, T>(
    items: impl Copy + IntoIterator<Item = &'a T>,
    predicate: impl Fn(&T, &T, &T) -> bool,
) -> Option<(&'a T, &'a T, &'a T)> {
    for third in items.into_iter() {
        if let Some((first, second)) =
            find_pair(items, |first, second| predicate(first, second, third))
        {
            return Some((first, second, third));
        }
    }
    None
}
