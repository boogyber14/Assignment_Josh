fn main() {
    match inquire::Text::new("What is your name?").prompt() {
        Ok(name) => {
            println!("Hello, {}!", name);
            match inquire::Text::new("How old are you?").prompt() {
                Ok(input) => {
                    match input.parse::<i32>() {
                        Ok(age) => println!("You are {} years old.", age),
                        Err(_) => println!("Invalid input. Please enter a valid integer."),
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
