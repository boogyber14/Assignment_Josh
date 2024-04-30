use rand::Rng;

fn main() {
    // Create a random number generator
    let mut rng = rand::thread_rng();

    // Generate a random integer between 1 and 100
    let random_number: u32 = rng.gen_range(1..=100);
    println!("Random number: {}", random_number);

    // Generate a random floating-point number between 0 and 1
    let random_float: f64 = rng.gen();
    println!("Random float: {}", random_float);

    // Generate a random boolean value
    let random_bool: bool = rng.gen();
    println!("Random boolean: {}", random_bool);
}
