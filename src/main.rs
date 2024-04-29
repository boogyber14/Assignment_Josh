extern crate inquire;
extern crate dirs;
extern crate walkdir;

use inquire::Text;

fn main() {
    // Prompt the user to enter their name
    let name: String = Text::new("Enter your name:")
        .prompt()
        .unwrap();

    // Prompt the user to enter their age
    let age: String = Text::new("Enter your age:")
        .prompt()
        .unwrap();

    // Convert age to u32 (if needed)
    let age: u32 = match age.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Age must be a number.");
            return;
        }
    };

    // Print a greeting message
    println!("Hello, {}! You are {} years old.", name, age);
}
