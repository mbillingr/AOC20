use common::input::Input;

fn main() {
    let input = Input::from_file("data/day11-input.txt");

    input.iter_lines()
        .inspect(|x| println!("{:?}", x))
        .collect::<Vec<_>>();

    println!("Hello, world!");
}
