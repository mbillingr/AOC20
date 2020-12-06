use common::input::Input;

fn main() {
    let input = Input::from_file("data/day07-input.txt");

    input.iter_lines()
        .for_each(|x| print!("{:?}", x));

    println!("Hello, world!");
}
