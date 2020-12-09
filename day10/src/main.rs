use common::input::Input;

fn main() {
    let input = Input::from_file("data/day10-input.txt");

    input.iter_lines()
        .inspect(|x| println!("{:?}", x))
        .count();

    println!("Hello, world!");
}
