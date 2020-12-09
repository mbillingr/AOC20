use common::input::Input;
use common::itertools::Itertools;
use std::collections::VecDeque;

fn main() {
    let input = Input::from_file("data/day09-input.txt");
    let numbers = input.iter_numbers().collect::<Vec<_>>();

    let first_inconsistent_number = find_first_inconsistency(&numbers);
    println!("Part 1: {}", first_inconsistent_number);

    let target_window = find_rolling_sum_slice(first_inconsistent_number, &numbers);
    let smallest = target_window.iter().min().unwrap();
    let largest = target_window.iter().max().unwrap();
    println!("Part 2: {}", smallest + largest);
}

fn find_first_inconsistency(numbers: &Vec<i64>) -> i64 {
    let mut last_25: VecDeque<_> = numbers[..25].iter().copied().collect();

    for &n in &numbers[25..] {
        if !find_sum(n, &last_25) {
            return n;
        }
        last_25.pop_front().unwrap();
        last_25.push_back(n);
    }
    panic!("Found no inconsistent number")
}

fn find_sum(n: i64, last: &VecDeque<i64>) -> bool {
    for pairs in last.iter().combinations(2) {
        if pairs.into_iter().sum::<i64>() == n {
            return true;
        }
    }
    return false;
}

fn find_rolling_sum_slice(n: i64, numbers: &[i64]) -> &[i64] {
    let mut begin = 0;
    let mut end = 0;
    let mut sum = 0;

    while sum != n {
        if sum < n {
            sum += numbers[end];
            end += 1;
        } else if sum > n {
            sum -= numbers[begin];
            begin += 1;
        }
    }
    assert_eq!(numbers[begin..end].iter().sum::<i64>(), n);
    &numbers[begin..end]
}
