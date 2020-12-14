
pub fn get_nth_bit(x: u64, n: u64) -> bool {
    ((x >> n) & 1) == 1
}

pub fn assign_nth_bit(x: u64, n: u64, val: bool) -> u64 {
    if val {
        x | (1 << n)
    } else {
        x & !(1 << n)
    }
}

pub fn set_nth_bit(x: u64, n: u64) -> u64 {
    x | (1 << n)
}

pub fn clear_nth_bit(x: u64, n: u64) -> u64 {
    x & !(1 << n)
}

pub fn toggle_nth_bit(x: u64, n: u64) -> u64 {
    x ^ !(1 << n)
}
