use std::io;

fn main() {
    let word_to_guess = String::from("table");
    println!("Word to guess: {}", &typed_word);
    // Get user input
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .ok()
    .expect("Failed to read user input.");
}
