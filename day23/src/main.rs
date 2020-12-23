use std::collections::HashSet;

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
    assert_eq!(input_cups.len(), 1000000);
    assert_eq!(
        input_cups.clone().into_iter().collect::<HashSet<_>>().len(),
        1000000
    );
    let mut cb = CupBoard::new(input_cups);

    for _ in (0..10000000).rev() {
        crab_move(&mut cb);
    }

    let a = cb.get_clockwise(1);
    let b = cb.get_clockwise(a);

    println!("Part 2: {}", a as u64 * b as u64);
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
    let mut label = wrapping_dec(cb.get_current(), cb.highest_cup());
    while picked.contains(&label) {
        label = wrapping_dec(label, cb.highest_cup());
    }
    label
}

fn wrapping_dec(x: Cup, maximum: Cup) -> Cup {
    if x == 1 {
        maximum
    } else {
        x - 1
    }
}

fn new_current_cup(cb: &mut CupBoard) {
    cb.rotate_clockwise();
}

type Cup = usize;

#[derive(Debug)]
struct CupBoard {
    clockwise_cup: Vec<Cup>,
    current: Cup,
}

impl CupBoard {
    fn new(input_cups: Vec<Cup>) -> Self {
        let first_cup = input_cups[0];
        let mut cups = vec![0; input_cups.len() + 1];
        cups[*input_cups.last().unwrap()] = first_cup;
        for w in input_cups.windows(2) {
            cups[w[0]] = w[1];
        }
        CupBoard {
            clockwise_cup: cups,
            current: first_cup,
        }
    }

    fn highest_cup(&self) -> Cup {
        self.clockwise_cup.len() - 1
    }

    /// point the current cup to the next clockwise cup
    fn rotate_clockwise(&mut self) {
        self.current = self.clockwise_cup[self.current];
    }

    /// returns current cup
    fn get_current(&self) -> Cup {
        self.current
    }

    /// returns current cup
    fn get_clockwise(&self, label: Cup) -> Cup {
        self.clockwise_cup[label]
    }

    fn take_after_current(&mut self) -> Cup {
        let cup = self.clockwise_cup[self.current];
        let new_after = self.clockwise_cup[cup];
        self.clockwise_cup[self.current] = new_after;
        self.clockwise_cup[cup] = 0;
        cup
    }

    fn put_after(&mut self, label: Cup, cup: Cup) {
        let old_after = self.clockwise_cup[label];
        self.clockwise_cup[cup] = old_after;
        self.clockwise_cup[label] = cup;
    }

    fn insert_after(&mut self, label: Cup, picked: &[Cup]) {
        for &cup in picked.iter().rev() {
            self.put_after(label, cup)
        }
    }
}
