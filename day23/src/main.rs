use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part1() {
    // Input: 418976235
    let input_cups = vec![4, 1, 8, 9, 7, 6, 2, 3, 5];
    //let input_cups = vec![3,8,9,1,2,5,4,6,7];
    let mut cb = CupBoard::new(input_cups);

    for _ in 0..100 {
        crab_move(&mut cb);
    }

    find_cup_one(&mut cb);
    let mut cup = 1;

    print!("Part 1: ");
    for _ in 1..=8 {
        cup = cb.get_clockwise(cup);
        print!("{}", cup)
    }
    println!()
}

fn part2() {
    let mut input_cups = vec![4, 1, 8, 9, 7, 6, 2, 3, 5];
    for i in 10..=1000000 {
        input_cups.push(i)
    }
    let mut cb = CupBoard::new(input_cups);

    for _ in (0..10000000).rev() {
        crab_move(&mut cb);
    }

    find_cup_one(&mut cb);
    let a = cb.get_clockwise(1);
    let b = cb.get_clockwise(a);

    println!("Part 2: {}", a as u64 * b as u64);
}

fn find_cup_one(cb: &mut CupBoard) {
    while cb.get_current() != 1 {
        cb.rotate_clockwise()
    }
}

fn crab_move(cb: &mut CupBoard) {
    let picked = pick_three(cb);
    let destination_label = find_destination_label(&picked, cb);
    cb.insert_after(destination_label, &picked);
    new_current_cup(cb);
}

fn pick_three(cb: &mut CupBoard) -> [Cup; 3] {
    let a = cb.take_after_current();
    let b = cb.take_after_current();
    let c = cb.take_after_current();
    [a, b, c]
}

fn find_destination_label(picked: &[Cup], cb: &mut CupBoard) -> Cup {
    let mut label = wrapping_dec(cb.get_current());
    while picked.contains(&label) {
        label = wrapping_dec(label);
    }
    label
}

fn wrapping_dec(x: Cup) -> Cup {
    if x == 1 {
        9
    } else {
        x - 1
    }
}

fn new_current_cup(cb: &mut CupBoard) {
    cb.rotate_clockwise();
}

type Cup = u32;

#[derive(Debug)]
struct CupBoard {
    cups: HashMap<Cup, Cup>,
    current: Cup,
}

impl CupBoard {
    fn new(mut input_cups: Vec<Cup>) -> Self {
        let first_cup = input_cups[0];
        input_cups.push(first_cup);
        let cups = input_cups.windows(2).map(|w| (w[0], w[1])).collect();
        CupBoard {
            cups,
            current: first_cup,
        }
    }

    /// point the current cup to the next clockwise cup
    fn rotate_clockwise(&mut self) {
        self.current = self.cups[&self.current];
    }

    /// returns current cup
    fn get_current(&self) -> Cup {
        self.current
    }

    /// returns current cup
    fn get_clockwise(&self, label: Cup) -> Cup {
        self.cups[&label]
    }

    fn take_after_current(&mut self) -> Cup {
        let cup = self.cups[&self.current];
        let new_after = self.cups[&cup];
        self.cups.insert(self.current, new_after);
        self.cups.remove(&cup);
        cup
    }

    fn put_after(&mut self, label: Cup, cup: Cup) {
        let old_after = self.cups[&label];
        self.cups.insert(cup, old_after);
        self.cups.insert(label, cup);
    }

    fn insert_after(&mut self, label: Cup, picked: &[Cup]) {
        for &cup in picked.iter().rev() {
            self.put_after(label, cup)
        }
    }
}
