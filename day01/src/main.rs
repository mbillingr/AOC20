use common::input::Input;

fn main() {
    let input = Input::from_file("data/day01-input.txt");

    let entries: Vec<_> = input.iter_numbers().collect();

    println!("Part 1: {}", part1(&entries));
    println!("Part 2: {}", part2(&entries));
}

fn part1(entries: &[i64]) -> i64 {
    for first in entries.iter() {
        for second in entries.iter() {
            if first + second == 2020 {
                return first * second;
            }
        }
    }
    panic!("Part 1: No solution")
}

fn part2(entries: &[i64]) -> i64 {
    for first in entries.iter() {
        for second in entries.iter() {
            for third in entries.iter() {
                if first + second + third == 2020 {
                    return first * second * third;
                }
            }
        }
    }
    panic!("Part 2: No solution")
}
