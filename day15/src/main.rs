use common::containers::VecMap;

fn main() {
    let input = vec![13, 16, 0, 12, 15, 1];
    println!("Part 1: {}", brute_force(2020, &input));
    println!("Part 2: {}", brute_force(30000000, &input));
}

fn brute_force(n: usize, input: &[usize]) -> usize {
    let mut game = ElvenGame::new(input.iter().copied());

    let mut most_recent = 0;
    for _ in 0..n - input.len() {
        most_recent = game.step();
    }

    most_recent
}

struct ElvenGame {
    turn_counter: usize,
    last_spoken: VecMap<usize>,
    prev_number: usize,
}

impl ElvenGame {
    fn new(input: impl IntoIterator<Item = usize>) -> Self {
        let mut last_spoken = VecMap::new();

        let mut game = Self {
            turn_counter: 0,
            last_spoken,
            prev_number: 0,
        };

        for n in input.into_iter() {
            game.speak(n);
        }

        game
    }

    fn step(&mut self) -> usize {
        let new_number = self.get_spoken(self.prev_number);
        self.speak(new_number)
    }

    fn speak(&mut self, new_number: usize) -> usize {
        self.set_spoken(self.prev_number, self.turn_counter);
        self.turn_counter += 1;
        self.prev_number = new_number;
        new_number
    }

    fn set_spoken(&mut self, n: usize, turn: usize) {
        self.last_spoken.insert(n, turn);
    }

    fn get_spoken(&mut self, n: usize) -> usize {
        let turn = self.last_spoken.get(&n).copied().unwrap_or(0);
        if turn == 0 {
            0
        } else {
            self.turn_counter - turn
        }
    }
}
