use std::io;

fn main() {
    let word_to_guess = String::from("table");
    let mut is_valid = false;
    let mut typed_word = String::new();
    for _ in 0..word_to_guess.len() { typed_word.push('_') }


    println!("Word to guess: {}", &typed_word);

    while !is_valid {
        println!("Please enter a letter");
        
        // Get user input
        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read user input.");

        // Clean input backline
        let len = input.trim_end_matches(&['\r', '\n'][..]).len();
        input.truncate(len);
    }
}
