
use std::io::{self, Write};

use crate::style::*;
use crate::commands::*;

static PROMPT: &str = ">> ";

pub fn run() {
    
    loop {

        print!("{}", PROMPT);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("read line failed");
        let cmd: Vec<&str> = input.trim().split_whitespace().collect();

        match cmd.first() {
            Some(&"exit") => {
                break;
            }
            Some(&"clear") => {
                clear_screen();
            }
            Some(&"help") => {
                help();
            }
            Some(_) => {
                println!("{0} {1} {2}", 
                    "idk what", 
                    input.trim(),
                    "is supposed to mean");
            }
            None => {
                print!(">");
            }
        }
    }
}
