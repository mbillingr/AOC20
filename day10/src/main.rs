use common::input::Input;
use std::collections::HashMap;

fn main() {
    let input = Input::from_file("data/day10-input.txt");

    let mut adapters = input.iter_numbers().collect::<Vec<_>>();
    adapters.sort();
    println!("{:?}", adapters);

    let mut diffs = [0, 1, 0, 1];
    for pair in adapters.windows(2) {
        let d = (pair[1] - pair[0]) as usize;
        diffs[d] += 1;
    }

    println!("Part 1: {}", diffs[1] * diffs[3]);

    adapters.push(adapters.last().unwrap() + 3);

    println!(
        "Part 2 (correct): {}",
        ArrangementCounter::new().count(0, &adapters)
    );

    let mut adapters2 = vec![];
    adapters2.extend(adapters);
    println!(
        "Part 2 (wip): {}",
        ArrangementCounter::new().count2(&adapters2)
    );
}

struct ArrangementCounter {
    cache: HashMap<i64, usize>,
}

impl ArrangementCounter {
    fn new() -> Self {
        ArrangementCounter {
            cache: HashMap::new(),
        }
    }

    fn count(&mut self, current_joltage: i64, mut adapters: &[i64]) -> usize {
        if adapters.len() == 1 {
            return 1;
        }

        if let Some(count) = self.cache.get(&current_joltage) {
            return *count;
        }

        let mut count = 0;
        while adapters.len() > 0 && adapters[0] <= current_joltage + 3 {
            count += self.count(adapters[0], &adapters[1..]);
            adapters = &adapters[1..]
        }

        self.cache.insert(current_joltage, count);

        count
    }

    fn count2(&mut self, mut adapters: &[i64]) -> usize {
        if adapters.len() == 3 {
            match adapters.last().unwrap() - adapters.first().unwrap() {
                2 | 3 => return 2,
                4 | 5 | 6 => return 1,
                _ => panic!("invalid sequence {:?}", adapters),
            }
            return 1;
        }

        if adapters.len() == 4 {
            let first = adapters.first().unwrap();
            return match [
                adapters[0] - first,
                adapters[1] - first,
                adapters[2] - first,
                adapters[3] - first,
            ] {
                [0, 1, 2, 3] => 4,
                [0, 1, 2, 5] => 2,
                [0, 1, 4, 5] => 1,
                [0, 3, 4, 5] => 2,
                [0, 3, 4, 7] => 1,
                [0, 3, 6, 7] => 1,
                _ => panic!("invalid sequence {:?}", adapters),
            };
        }

        let pivot = adapters.len() / 2;
        let left = &adapters[..pivot + 1];
        let right = &adapters[pivot..];
        let mid = &adapters[pivot - 1..pivot + 2];

        let count = self.count2(left) * self.count2(right) * self.count2(mid);

        if adapters.len() <= 5 {
            println!("{}  {:?}", count, adapters);
        }

        count
    }
}
