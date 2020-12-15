use std::collections::HashMap;

fn main() {
    let input = vec![13, 16, 0, 12, 15, 1];
    println!("Part 1: {}", brute_force(2020, &input));
    println!("Part 2: {}", brute_force(30000000, &input));
}

fn brute_force(n: usize, input: &[u64]) -> u64 {
    let mut game = ElvenGame::new(input.iter().copied());

    let mut most_recent = 0;
    for _ in 0..n - input.len() {
        most_recent = game.step();
    }

    most_recent
}

struct ElvenGame {
    turn_counter: u64,
    last_spoken: HashMap<u64, u64>,
    prev_number: u64,
}

impl ElvenGame {
    fn new(input: impl IntoIterator<Item = u64>) -> Self {
        let mut turn_counter = 0;
        let mut last_spoken = HashMap::new();
        let mut prev_number = 0;

        for n in input.into_iter() {
            turn_counter += 1;
            last_spoken.insert(n, turn_counter);
            prev_number = n;
        }
        Self {
            turn_counter,
            last_spoken,
            prev_number,
        }
    }

    fn step(&mut self) -> u64 {
        let new_number = if let Some(&n) = self.last_spoken.get(&self.prev_number) {
            self.turn_counter - n
        } else {
            0
        };
        self.last_spoken.insert(self.prev_number, self.turn_counter);
        self.turn_counter += 1;
        self.prev_number = new_number;
        new_number
    }
}
