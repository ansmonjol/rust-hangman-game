use std::io;

fn main() {
    let word_to_guess = String::from("table");
    let mut is_valid = false;
    let mut typed_word = String::new();
    for _ in 0..word_to_guess.len() { typed_word.push('_') }


    println!("Word to guess contains {} letters", &typed_word.len());

    while !is_valid {
        println!("Please enter a letter:");
        
        // Get user input
        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read user input.");

        // Clean input backline
        let len = input.trim_end_matches(&['\r', '\n'][..]).len();
        input.truncate(len);

        // Check against user input and word to guess
        for (i, w) in word_to_guess.chars().enumerate() {
            // If user guessed one letter
            if w.to_string() == input {
                // Replace the char '_' at the same position with the user input
                typed_word.replace_range(i..i+1, &w.to_string());
            }
        }
        
        // Word check
        if typed_word == word_to_guess {
            println!("Congrats! You found the word {}", word_to_guess);
            is_valid = true;
        } else {
            println!("You found the letter {}, congrats!\n{}\n", input, typed_word)
        }
    }
}
