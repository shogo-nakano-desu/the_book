use rand::Rng;
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    restaurant::hosting::add_to_waitlist();
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
