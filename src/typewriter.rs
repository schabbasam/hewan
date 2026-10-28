
use std::{
    io::{self, Write},
    thread,
    time::Duration
};

pub fn typewriter(text: &str, line_width: usize) {
    let normalized_text = text.split_whitespace().collect::<Vec<_>>().join(" ");

    let mut column = 0;

    for word in normalized_text.split_whitespace() {
        let word_length = word.chars().count();
        let space_needed = if column == 0 { 0 } else { 1 };

        // start a new line if the word does not fit
        if column + space_needed + word_length > line_width && column > 0 {
            println!();
            column = 0;
        }

        // print a space between words
        if column > 0 {
            print!(" ");
            column += 1;
        }

        for character in word.chars() {
            print!("{character}");

            io::stdout().flush().unwrap();

            column += 1;

            let delay = match character {
                '.' | '!' | '?' => 500,
                ',' | ';' | ':' => 250,
                _ => 30
            };

            thread::sleep(Duration::from_millis(delay));
        }
    }
    
    println!();
}
