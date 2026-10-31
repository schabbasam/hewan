
use std::io::{self, Write};

use crate::style::*;
use crate::commands::*;

static PROMPT: &str = ">> ";

pub fn run() {
    
    loop {
        set_style(&Style::PROMPT);
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
                help(cmd);
            }
            Some(_) => {
                set_style(&Style::ERROR);
                println!("idk what {} is supposed to mean", input.trim());
            }
            None => {
                print!(">");
            }
        }
    }
}
