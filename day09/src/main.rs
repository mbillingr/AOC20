use common::ansi_term::{Color, Style};
use common::input::Input;
use common::itertools::Itertools;
use common::terminal_size::terminal_size;
use std::collections::VecDeque;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let input = Input::from_file("data/day09-input.txt");
    let numbers = input.iter_numbers().collect::<Vec<_>>();

    let first_inconsistent_number = find_first_inconsistency(&numbers);
    println!("Part 1: {}", first_inconsistent_number);
    sleep(Duration::from_secs(1));

    let target_window = find_rolling_sum_slice(first_inconsistent_number, &numbers);
    let smallest = target_window.iter().min().unwrap();
    let largest = target_window.iter().max().unwrap();
    println!("Part 2: {}", smallest + largest);
}

fn find_first_inconsistency(numbers: &Vec<i64>) -> i64 {
    let mut last_25: VecDeque<_> = numbers[..25].iter().copied().collect();

    for (i, n) in numbers[25..].iter().copied().enumerate() {
        if !find_sum(n, &last_25) {
            visualize_numbers(numbers, i + 12, &[(i, i + 25)], &[], &[(i + 25, i + 26)]);
            return n;
        }
        visualize_numbers(numbers, i + 12, &[(i, i + 25)], &[(i + 25, i + 26)], &[]);
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
        visualize_numbers(numbers, (begin + end) / 2, &[(begin, end)], &[], &[]);
        if sum < n {
            sum += numbers[end];
            end += 1;
        } else if sum > n {
            sum -= numbers[begin];
            begin += 1;
        }
    }
    assert_eq!(numbers[begin..end].iter().sum::<i64>(), n);

    let smallest = begin
        + numbers[begin..end]
            .iter()
            .enumerate()
            .min_by(|(_, n1), (_, n2)| n1.cmp(n2))
            .map(|(i, _)| i)
            .unwrap();
    let largest = begin
        + numbers[begin..end]
            .iter()
            .enumerate()
            .max_by(|(_, n1), (_, n2)| n1.cmp(n2))
            .map(|(i, _)| i)
            .unwrap();
    visualize_numbers(
        numbers,
        (begin + end) / 2,
        &[],
        &[(begin, end)],
        &[(smallest, smallest + 1), (largest, largest + 1)],
    );

    &numbers[begin..end]
}

fn visualize_numbers<T: std::fmt::Display>(
    items: &[T],
    focus: usize,
    white_ranges: &[(usize, usize)],
    green_ranges: &[(usize, usize)],
    red_ranges: &[(usize, usize)],
) {
    if terminal_size().is_none() {
        return;
    }

    let size = terminal_size().unwrap();
    let width = (size.0).0 as usize - 1;
    let height = (size.1).0 as usize;

    let column_width = items.iter().map(|x| format!("{}", x).len()).max().unwrap();

    let n_cols = width / (column_width + 1);
    let n_rows = height - 2;
    let n_items = (n_cols * n_rows).min(items.len());

    let mut first_item = usize::saturating_sub(focus, n_items / 2);
    if first_item + n_items > items.len() {
        first_item = items.len() - n_items;
    }

    // clear terminal
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    let white = Style::new().reverse();
    let green = Style::new().fg(Color::Green).reverse().blink();
    let red = Style::new().fg(Color::Red).reverse().blink();

    for row in 0..n_rows {
        for col in 0..n_cols {
            let idx = first_item + row + col * n_rows;
            if idx >= items.len() {
                break;
            }

            let num = format!("{:w$}", &items[idx], w = column_width);

            if in_range(idx, red_ranges) {
                print!("{} ", red.paint(num));
            } else if in_range(idx, green_ranges) {
                print!("{} ", green.paint(num));
            } else if in_range(idx, white_ranges) {
                print!("{} ", white.paint(num));
            } else {
                print!("{} ", num);
            }
        }
        println!()
    }
    sleep(Duration::from_millis(10));
}

fn in_range(n: usize, ranges: &[(usize, usize)]) -> bool {
    for &(begin, end) in ranges {
        if n >= begin && n < end {
            return true;
        }
    }
    false
}
