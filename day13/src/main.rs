use common::input::Input;
use common19::lcm;

fn main() {
    let input = Input::from_file("data/day13-input.txt");
    let mut lines = input.iter_lines();
    let t0: i64 = lines.next().unwrap().parse().unwrap();

    let buses: Vec<_> = input
        .iter_lines()
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .map(str::parse::<i64>)
        .enumerate()
        .filter(|(_, r)| r.is_ok())
        .map(|(i, r)| (i as i64, r.unwrap()))
        .collect();

    let part1 = buses
        .iter()
        .map(|(_, bus)| *bus)
        .map(|bus| (bus, bus - t0 % bus))
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(bus, t_wait)| bus * t_wait)
        .unwrap();

    println!("Part 1: {}", part1);

    let mut t = 0;
    let mut dt = 1;

    for (ofs, bus) in buses {
        while (t + ofs) % bus != 0 {
            t += dt;
        }
        dt = lcm(dt, bus);
    }

    println!("Part 2: {}", t);
}

/*
1002578
19,x,x,x,x,x,x,x,x,x,x,x,x,
37,x,x,x,x,x,
751,x,
29,x,x,x,x,x,x,x,x,x,x,
13,x,x,x,x,x,x,x,x,x,
23,x,x,x,x,x,x,x,
431,x,x,x,x,x,x,x,x,x,
41,x,x,x,x,x,x,
17
*/
