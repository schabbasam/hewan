// the typewriter function

use std::{
    io::{self, Write}, // used for flush()
    thread,
    time::Duration
};

use crate::style::*;

// delay in milliseconds
const NORMAL_DELAY: u64     = 50;
const COMMA_DELAY: u64      = 250;
const SENTENCE_DELAY: u64   = 500;

pub fn typewriter(text: &String, line_width: usize) {

    // this is done to ignore all previous whitespaces and newlines
    let normalized_text = text
        .split_whitespace() // split at whitespaces
        .collect::<Vec<_>>() // create a dynamic Vec with all words
        .join(" "); // join it to a string
    
    // which character in a line we are on
    let mut column = 0;

    // runs through each word
    for word in normalized_text.split_whitespace() {

        // how many characters the word has
        let word_length = word.chars().count();

        // if it is a new line, no space is needed
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
            
            // the print output may write the text in chunks
            // that's why we need to clear the buffer
            // and force the output immediatly
            io::stdout().flush().unwrap();

            column += 1;

            let delay = match character {
                '.' | '!' | '?' => SENTENCE_DELAY,
                ',' | ';' | ':' => COMMA_DELAY,
                _ => NORMAL_DELAY
            };

            thread::sleep(Duration::from_millis(delay));
        }
    }
    
    // reset the styling codes
    println!("\x1b[0m");
}
