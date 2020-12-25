use common::input::Input;

type Int = i64;

const PUB_CARD: Int = 5290733;
const PUB_DOOR: Int = 15231938;


fn main() {

    let mut value = 1;
    let mut card_loop_size = 0;
    while value != PUB_CARD {
        value *= 7;
        value %= 20201227;
        card_loop_size += 1;
    }

    println!("The card's loop size seems to be {}", card_loop_size);

    let mut value = 1;
    let mut door_loop_size = 0;
    while value != PUB_DOOR {
        value *= 7;
        value %= 20201227;
        door_loop_size += 1;
    }

    println!("The door's loop size seems to be {}", door_loop_size);

    let mut value = 1;
    for _ in 0..card_loop_size {
        value *= PUB_DOOR;
        value %= 20201227;
    }

    println!("Encryption key (card side): {}", value);

    let mut value = 1;
    for _ in 0..door_loop_size {
        value *= PUB_CARD;
        value %= 20201227;
    }

    println!("Encryption key (door side): {}", value);
}
