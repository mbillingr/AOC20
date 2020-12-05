use common::input::Input;

fn main() {
    let input = Input::from_file("data/day05-input.txt");

    let mut ids: Vec<_> = input
        .iter_lines()
        .map(|line| line.replace('F', "0"))
        .map(|line| line.replace('B', "1"))
        .map(|line| line.replace('L', "0"))
        .map(|line| line.replace('R', "1"))
        .map(|line| usize::from_str_radix(&line, 2).unwrap())
        .collect();

    ids.sort();

    println!("Part 1: {:?}", ids.last().unwrap());

    let my_id = ids
        .windows(3)
        .filter(|win| win[1] - win[0] != 1)
        .map(|win| win[0] + 1)
        .next()
        .unwrap();

    println!("Part 2: {:?}", my_id);
}
