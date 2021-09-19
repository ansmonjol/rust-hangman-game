use std::io;

fn main() {
    let word_to_guess = String::from("table");
    let mut is_valid = false;
    println!("Word to guess: {}", &typed_word);
    while !is_valid {
        println!("Please enter a letter");
        
        // Get user input
        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read user input.");
    }
}
