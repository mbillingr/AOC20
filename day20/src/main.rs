use common::input::Input;

fn main() {
    let input = Input::from_file("data/day20-input.txt");

    let _ = input
        .iter_lines()
        .inspect(|x| println!("{:?}", x))
        .collect::<Vec<_>>();

    println!("Hello, world!");
}
