use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();

    // Generate a random number between 1 and 100
    let num: i32 = rng.gen_range(1, 101);

    println!("The generated number is {}", num);
}
