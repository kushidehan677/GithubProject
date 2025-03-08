
use std::time::{Duration, SystemTime};

fn main() {
    let now = SystemTime::now();
    let mut rng = rand::thread_rng();
    let random_number: u64 = rng.gen_range(1..=10);
    println!("The random number is {}", random_number);
}