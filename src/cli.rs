
use std::io::{self, Write};
use crate::style::{Style, StyledText};
use std::sync::LazyLock;

static PROMPT: LazyLock<StyledText> = LazyLock::new(|| ">> ".green().bold());

pub fn run() {
    
    loop {

        print!("{}", *PROMPT);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("read line failed");
        let cmd: Vec<&str> = input.trim().split_whitespace().collect();
    }
}
