
use std::io::{self, Write};
use std::sync::LazyLock;

use crate::style::{self, Style, StyledText};
use crate::commands::*;

static PROMPT: LazyLock<StyledText> = LazyLock::new(|| ">> ".green().bold());

pub fn run() {
    
    loop {

        print!("{}", *PROMPT);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("read line failed");
        let cmd: Vec<&str> = input.trim().split_whitespace().collect();

        match cmd.first() {
            Some(&"exit") => {
                break;
            }
            Some(&"clear") => {
                style::clear_screen();
            }
            Some(&"help") => {
                help();
            }
            Some(_) => {
                println!("{0} {1} {2}", 
                    "idk what".green().italic(), 
                    input.trim(),
                    "is supposed to mean");
                style::clear_styling();
            }
            None => {
                print!(">");
            }
        }
    }
}
