use common::input::Input;

fn main() {
    let input = Input::from_file("data/day18-input.txt");

    let _: Vec<_> = input
        .iter_lines()
        .inspect(|x| println!("{:?}", x))
        .collect();

    println!("Hello, world!");
}
