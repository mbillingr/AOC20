use common::bitops::{assign_nth_bit, get_nth_bit};
use common::input::Input;
use std::collections::HashMap;

fn main() {
    let input = Input::from_file("data/day14-input.txt");

    let program: Vec<Op> = input
        .iter_lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let mut memory = HashMap::new();
    let mut mask = Mask::default();

    for op in &program {
        match op {
            Op::SetMask(m) => mask = m.clone(),
            Op::SetMem(addr, val) => {
                memory.insert(addr, mask.apply(*val));
            }
        }
    }

    println!("Part 1: {:?}", memory.values().sum::<u64>());

    let mut memory = HashMap::new();
    let mut mask = Mask::default();

    for op in &program {
        match op {
            Op::SetMask(m) => mask = m.clone(),
            Op::SetMem(addr, val) => {
                for a in mask.all_addrs(*addr) {
                    memory.insert(a, *val);
                }
            }
        }
    }

    println!("Part 2: {:?}", memory.values().sum::<u64>());
}

#[derive(Debug, Clone)]
enum Op {
    SetMask(Mask),
    SetMem(u64, u64),
}

impl std::str::FromStr for Op {
    type Err = ();
    fn from_str(s: &str) -> Result<Op, ()> {
        let mut parts = s.split(" = ");
        let left = parts.next().unwrap();
        let right = parts.next().unwrap();

        if left == "mask" {
            Ok(Op::SetMask(Mask::from_str(right)))
        } else {
            let addr = left
                .split('[')
                .skip(1)
                .next()
                .unwrap()
                .split(']')
                .next()
                .unwrap()
                .parse()
                .unwrap();
            let val = right.parse().unwrap();
            Ok(Op::SetMem(addr, val))
        }
    }
}

#[derive(Clone)]
struct Mask {
    and_mask: u64,
    or_mask: u64,
    x_locs: Vec<u64>,
}

impl Default for Mask {
    fn default() -> Self {
        Mask {
            and_mask: u64::max_value(),
            or_mask: 0,
            x_locs: vec![],
        }
    }
}

impl std::fmt::Debug for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in (0..36).rev() {
            if self.x_locs.contains(&i) {
                write!(f, "X")?;
            } else {
                if (self.and_mask >> i) % 2 == 0 {
                    write!(f, "0")?;
                } else if (self.or_mask >> i) % 2 == 1 {
                    write!(f, "1")?;
                } else {
                    write!(f, "?")?;
                }
            }
        }
        Ok(())
    }
}

impl Mask {
    fn from_str(s: &str) -> Self {
        let mut and_mask = 1;
        let mut or_mask = 0;
        let mut x_locs = Vec::with_capacity(36);

        let mut i = 36;

        for ch in s.chars() {
            and_mask = and_mask * 2 + 1;
            or_mask = or_mask * 2 + 0;
            i -= 1;
            match ch {
                '0' => and_mask -= 1,
                '1' => or_mask += 1,
                'X' => x_locs.push(i),
                _ => panic!("Invalid char"),
            }
        }

        Mask {
            and_mask,
            or_mask,
            x_locs,
        }
    }

    fn apply(&self, x: u64) -> u64 {
        (x & self.and_mask) | self.or_mask
    }

    fn all_addrs(&self, addr0: u64) -> impl Iterator<Item = u64> + '_ {
        let addr0 = addr0 | self.or_mask;
        let n_x = self.x_locs.len();
        (0..2u64.pow(n_x as u32)).map(move |i| {
            let mut addr = addr0;
            for (nth_x, &x_pos) in self.x_locs.iter().enumerate() {
                addr = assign_nth_bit(addr, x_pos, get_nth_bit(i, nth_x as u64));
            }
            addr
        })
    }
}
