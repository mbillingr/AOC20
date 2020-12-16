use common::input::Input;
use std::collections::HashMap;

fn main() {
    let input = Input::from_file("data/day16-input.txt");

    let blocks = input.iter_blocks().collect::<Vec<_>>();

    let loc_str = blocks[0];
    let tickt_str = blocks[1];
    let other_str = blocks[2];

    let loc_ranges: HashMap<&'static str, Range> = loc_str
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let name: &'static str = Box::leak(parts.next().unwrap().to_string().into_boxed_str());
            let ranges = parts.next().unwrap();
            (name, Range::from_str(ranges))
        })
        .collect();

    let ticket = tickt_str.lines().skip(1).map(parse_numbers).next().unwrap();

    let other_tickets = other_str
        .lines()
        .skip(1)
        .map(parse_numbers)
        .collect::<Vec<_>>();

    let mut error_rate = 0;
    for &value in other_tickets.iter().flatten() {
        if !loc_ranges.values().any(|range| range.is_valid(value)) {
            error_rate += value;
        }
    }

    let mut remaining_tickets = other_tickets
        .into_iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|&value| loc_ranges.values().any(|range| range.is_valid(value)))
        })
        .collect::<Vec<_>>();

    remaining_tickets.push(ticket.clone());  // not necessary

    let mut am = ArrangementMatrix::new(loc_ranges.keys().copied().collect(), |field, pos| {
        is_valid(pos, field, &remaining_tickets, &loc_ranges)
    });
    am.print();
    am.sort_rows();
    am.sort_cols();
    am.print();
    let total: usize = am
        .assignments()
        .filter(|(&field, _)| field.starts_with("departure"))
        .map(|(_, idx)| ticket[idx])
        .product();

    println!("Part 1: {}", error_rate);
    println!("Part 2: {}", total);
}

fn parse_numbers(s: &str) -> Vec<usize> {
    s.split(',').map(str::parse).map(Result::unwrap).collect()
}

#[derive(Debug)]
struct Range {
    min_max: Vec<(usize, usize)>,
}

impl Range {
    fn from_str(s: &str) -> Self {
        let mut min_max = vec![];
        for segment in s.split(" or ") {
            let mut parts = segment.split('-');
            let min = parts.next().unwrap().parse().unwrap();
            let max = parts.next().unwrap().parse().unwrap();
            min_max.push((min, max));
        }
        Range { min_max }
    }

    fn is_valid(&self, value: usize) -> bool {
        self.min_max
            .iter()
            .any(|&(min, max)| value >= min && value <= max)
    }
}

fn is_valid(field_position: usize, field_name: &str, tickets: &[Vec<usize>], ranges: &HashMap<&str, Range>) -> bool {
    let rng = &ranges[field_name];

    let all_valid = tickets
        .iter()
        .map(|ticket| ticket[field_position])
        .all(|value| rng.is_valid(value));
    all_valid
}


struct ArrangementMatrix<T> {
    data: Vec<Vec<bool>>,
    items: Vec<T>,
    positions: Vec<usize>,
}

impl<T: Clone> ArrangementMatrix<T> {
    fn new(items: Vec<T>, is_valid: impl Fn(&T, usize) -> bool) -> Self {
        let mut data = vec![];
        for item in &items {
            data.push((0..items.len()).map(|i| is_valid(item, i)).collect());
        }

        let positions = (0..items.len()).collect();

        ArrangementMatrix {
            data,
            items,
            positions,
        }
    }

    fn n(&self) -> usize {
        self.items.len()
    }

    fn sort_rows(&mut self) {
        let mut indices = (0..self.n()).collect::<Vec<_>>();
        indices.sort_by_key(|&i| self.row_iter(i).filter(|&x| x).count());
        indices.reverse();
        self.items = indices.iter().map(|&i| self.items[i].clone()).collect();
        self.data = indices.iter().map(|&i| self.data[i].clone()).collect();
    }

    fn sort_cols(&mut self) {
        let mut indices = (0..self.n()).collect::<Vec<_>>();
        indices.sort_by_key(|&i| self.col_iter(i).filter(|&x| x).count());
        self.positions = indices.iter().map(|&i| self.positions[i].clone()).collect();
        for row in &mut self.data {
            *row = indices.iter().map(|&i| row[i]).collect()
        }
    }

    fn row_iter(&self, row: usize) -> impl Iterator<Item = bool> + '_ {
        self.data[row].iter().copied()
    }

    fn col_iter(&self, col: usize) -> impl Iterator<Item = bool> + '_ {
        self.data.iter().map(move |row| row[col])
    }

    fn assignments(&self) -> impl Iterator<Item = (&T, usize)> {
        (0..self.n()).map(move |i| (&self.items[i], self.positions[i]))
    }
}

impl<T: std::fmt::Display> ArrangementMatrix<T> {
    fn print(&self) {
        for p in &self.positions {
            print!("{:2}|", p)
        }
        println!();
        for (row, name) in self.data.iter().zip(&self.items) {
            for item in row {
                if *item {
                    print!(" #|");
                } else {
                    print!("  |");
                }
            }
            println!(" {}", name);
        }
        for p in &self.positions {
            print!("{:2}|", p)
        }
        println!();
    }
}
