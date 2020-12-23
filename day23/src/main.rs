use std::cell::Cell;
use std::collections::VecDeque;

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
    cb.take_cup();

    print!("Part 1: ");
    for _ in 1..=8 {
        print!("{}", cb.take_cup())
    }
    println!()
}

fn part2() {
    let mut input_cups = vec![4, 1, 8, 9, 7, 6, 2, 3, 5];
    for i in 10..=1000000 {
        input_cups.push(i)
    }
    let mut cb = CupBoard::new(input_cups);

    for _ in 0..10000000 {
        crab_move(&mut cb);
    }

    find_cup_one(&mut cb);
    cb.take_cup();

    println!("Part 2: {}", cb.take_cup() * cb.take_cup());
}

fn find_cup_one(cb: &mut CupBoard) {
    while cb.get_current() != 1 {
        cb.rotate_clockwise()
    }
}

fn crab_move(cb: &mut CupBoard) {
    let picked = pick_three(cb);
    let destination_label = find_destination_label(&picked, cb);
    insert_at(destination_label, picked, cb);
    new_current_cup(cb);
}

fn pick_three(cb: &mut CupBoard) -> [Cup; 3] {
    cb.rotate_clockwise();
    let a = cb.take_cup();
    let b = cb.take_cup();
    let c = cb.take_cup();
    cb.rotate_counterclockwise();
    [a, b, c]
}

fn find_destination_label(picked: &[Cup], cb: &mut CupBoard) -> Cup {
    let mut label = wrapping_dec(cb.get_current());
    while picked.contains(&label) {
        label = wrapping_dec(label);
    }
    label
}

fn wrapping_dec(mut x: Cup) -> Cup {
    if x == 1 {
        9
    } else {
        x - 1
    }
}

fn insert_at(label: Cup, picked: [Cup; 3], cb: &mut CupBoard) {
    let mut n_rot = 1;
    while cb.get_current() != label {
        n_rot += 1;
        cb.rotate_clockwise();
    }
    cb.rotate_clockwise();

    cb.put_cup(picked[2]);
    cb.put_cup(picked[1]);
    cb.put_cup(picked[0]);

    for _ in 0..n_rot {
        cb.rotate_counterclockwise();
    }
}

fn new_current_cup(cb: &mut CupBoard) {
    cb.rotate_clockwise();
}

type Cup = u32;

#[derive(Debug)]
struct CupBoard {
    /// current cup is defined to be at index 0
    /// clockwise is index +1
    cups: VecDeque<Cup>,
}

impl CupBoard {
    fn new(input_cups: Vec<Cup>) -> Self {
        CupBoard {
            cups: input_cups.into(),
        }
    }

    /// point the current cup to the next clockwise cup
    fn rotate_clockwise(&mut self) {
        let tmp = self.cups.pop_front().unwrap();
        self.cups.push_back(tmp);
    }

    /// point the current cup to the next counter clockwise cup
    fn rotate_counterclockwise(&mut self) {
        let tmp = self.cups.pop_back().unwrap();
        self.cups.push_front(tmp);
    }

    /// returns current cup
    fn get_current(&self) -> Cup {
        self.cups[0]
    }

    /// removes current cup, making the clockwise cup the new current
    fn take_cup(&mut self) -> Cup {
        self.cups.pop_front().unwrap()
    }

    /// removes current cup, making the clockwise cup the new current
    fn put_cup(&mut self, cup: Cup) {
        self.cups.push_front(cup)
    }
}
